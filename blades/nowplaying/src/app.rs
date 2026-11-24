//! The playing app, from the shell: its icon, and bringing it to the front. The media session's
//! AppUserModelID names an entry in the Apps folder (`shell:AppsFolder`), as for Chrome
//! (`Chrome`), Edge (`MSEdge`) and Store apps (`…!App`). Desktop apps that report their exe name
//! instead (Spotify: `Spotify.exe`) aren't in that folder, so their icon and window come from the
//! running process's exe.
use pk::wide::{from_buf, to_wide};
use windows::Win32::Foundation::SIZE;
use windows::Win32::Graphics::Gdi::HBITMAP;
use windows::Win32::Storage::EnhancedStorage::PKEY_Link_TargetParsingPath;
use windows::Win32::System::Com::CoTaskMemFree;
use windows::Win32::UI::Shell::{
    FOLDERID_AppsFolder, IShellItem2, IShellItemImageFactory, KF_FLAG_DEFAULT, SHCreateItemFromParsingName,
    SHCreateItemInKnownFolder, SIIGBF_ICONONLY, SIIGBF_RESIZETOFIT,
};
use windows_core::{Interface, PCWSTR};
use windows_sys::Win32::Foundation::{CloseHandle, HWND, INVALID_HANDLE_VALUE, LPARAM};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION, QueryFullProcessImageNameW};
use windows_sys::Win32::UI::Shell::{SEE_MASK_ASYNCOK, SEE_MASK_FLAG_NO_UI, SHELLEXECUTEINFOW, ShellExecuteExW};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    ASFW_ANY, AllowSetForegroundWindow, EnumWindows, GW_OWNER, GetWindow, GetWindowTextW, GetWindowThreadProcessId, IsIconic,
    IsWindowVisible, SW_RESTORE, SW_SHOWNORMAL, SetForegroundWindow, ShowWindow,
};

/// The app's icon as a `px`×`px` bitmap with alpha; the caller deletes it. Needs COM on this thread.
pub fn bitmap(aumid: &str, px: u32) -> Option<HBITMAP> {
    let factory = image_factory(aumid)?;
    let size = SIZE { cx: px as i32, cy: px as i32 };
    unsafe { factory.GetImage(size, SIIGBF_ICONONLY | SIIGBF_RESIZETOFIT) }.ok()
}

/// Brings the app's window to the front, restoring it if minimized: of its top-level windows, the
/// one whose title contains the track's `title` (a browser's playing tab), else the topmost. With
/// no window (Spotify closed to the tray, a Store app) it launches the app, which a running app
/// answers by showing itself. Call it from the click's input handler: that click is what lets
/// Windows hand the foreground to another app. Needs COM on this thread.
pub fn activate(aumid: &str, title: &str) {
    if aumid.is_empty() {
        return;
    }
    let item = apps_folder_item(aumid);
    let exe = item.as_ref().and_then(link_target).or_else(|| running_exe_path(aumid));
    if let Some(hwnd) = exe.as_deref().and_then(|exe| app_window(exe, title)) {
        unsafe {
            if IsIconic(hwnd) != 0 {
                ShowWindow(hwnd, SW_RESTORE);
            }
            SetForegroundWindow(hwnd);
        }
        return;
    }
    let target = match (item, exe) {
        (Some(_), _) => alloc::format!("shell:AppsFolder\\{aumid}"),
        (None, Some(exe)) => exe,
        (None, None) => return,
    };
    let file = to_wide(&target);
    let mut info: SHELLEXECUTEINFOW = unsafe { core::mem::zeroed() };
    info.cbSize = core::mem::size_of::<SHELLEXECUTEINFOW>() as u32;
    info.fMask = SEE_MASK_FLAG_NO_UI | SEE_MASK_ASYNCOK;
    info.lpFile = file.as_ptr();
    info.nShow = SW_SHOWNORMAL;
    unsafe {
        AllowSetForegroundWindow(ASFW_ANY);
        ShellExecuteExW(&mut info);
    }
}

fn apps_folder_item(aumid: &str) -> Option<IShellItem2> {
    let id = to_wide(aumid);
    unsafe { SHCreateItemInKnownFolder(&FOLDERID_AppsFolder, KF_FLAG_DEFAULT, PCWSTR(id.as_ptr())) }.ok()
}

fn image_factory(aumid: &str) -> Option<IShellItemImageFactory> {
    if aumid.is_empty() {
        return None;
    }
    if let Some(item) = apps_folder_item(aumid) {
        return item.cast().ok();
    }
    let path = to_wide(&running_exe_path(aumid)?);
    unsafe { SHCreateItemFromParsingName(PCWSTR(path.as_ptr()), None) }.ok()
}

/// The exe a desktop app's Apps-folder entry starts (Chrome: `…\chrome.exe`); Store apps have none.
fn link_target(item: &IShellItem2) -> Option<alloc::string::String> {
    let p = unsafe { item.GetString(&PKEY_Link_TargetParsingPath) }.ok()?;
    let path = from_buf(unsafe { p.as_wide() });
    unsafe { CoTaskMemFree(Some(p.0 as *const _)) };
    (!path.is_empty()).then_some(path)
}

struct WindowSearch<'a> {
    exe: &'a str,
    title: &'a str,
    topmost: HWND,
    titled: HWND,
}

/// A visible, unowned, titled top-level window of a process running `exe`. Cloaked windows count:
/// a window on another virtual desktop is cloaked, and switching to it beats starting a new one.
fn app_window(exe: &str, title: &str) -> Option<HWND> {
    let mut search = WindowSearch { exe, title, topmost: core::ptr::null_mut(), titled: core::ptr::null_mut() };
    unsafe { EnumWindows(Some(visit_window), &mut search as *mut WindowSearch as LPARAM) };
    let hwnd = if search.titled.is_null() { search.topmost } else { search.titled };
    (!hwnd.is_null()).then_some(hwnd)
}

unsafe extern "system" fn visit_window(hwnd: HWND, lparam: LPARAM) -> windows_sys::core::BOOL {
    let search = unsafe { &mut *(lparam as *mut WindowSearch) };
    if unsafe { IsWindowVisible(hwnd) } == 0 || !unsafe { GetWindow(hwnd, GW_OWNER) }.is_null() {
        return 1;
    }
    let mut buf = [0u16; 512];
    let len = unsafe { GetWindowTextW(hwnd, buf.as_mut_ptr(), buf.len() as i32) };
    if len <= 0 {
        return 1;
    }
    let mut pid = 0u32;
    unsafe { GetWindowThreadProcessId(hwnd, &mut pid) };
    if !exe_path_of(pid).is_some_and(|p| p.eq_ignore_ascii_case(search.exe)) {
        return 1;
    }
    if search.topmost.is_null() {
        search.topmost = hwnd;
    }
    if !search.title.is_empty() && from_buf(&buf[..len as usize]).contains(search.title) {
        search.titled = hwnd;
        return 0;
    }
    1
}

/// Full path of a running process whose exe file name is `exe_name` (case-insensitive).
fn running_exe_path(exe_name: &str) -> Option<alloc::string::String> {
    let snap = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if snap == INVALID_HANDLE_VALUE {
        return None;
    }
    let mut entry: PROCESSENTRY32W = unsafe { core::mem::zeroed() };
    entry.dwSize = core::mem::size_of::<PROCESSENTRY32W>() as u32;
    let mut found = None;
    let mut ok = unsafe { Process32FirstW(snap, &mut entry) } != 0;
    while ok {
        if from_buf(&entry.szExeFile).eq_ignore_ascii_case(exe_name) {
            found = exe_path_of(entry.th32ProcessID);
            if found.is_some() {
                break;
            }
        }
        ok = unsafe { Process32NextW(snap, &mut entry) } != 0;
    }
    unsafe { CloseHandle(snap) };
    found
}

fn exe_path_of(pid: u32) -> Option<alloc::string::String> {
    let h = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid) };
    if h.is_null() {
        return None;
    }
    let mut buf = [0u16; 1024];
    let mut len = buf.len() as u32;
    let ok = unsafe { QueryFullProcessImageNameW(h, PROCESS_NAME_WIN32, buf.as_mut_ptr(), &mut len) } != 0;
    unsafe { CloseHandle(h) };
    ok.then(|| from_buf(&buf[..len as usize]))
}
