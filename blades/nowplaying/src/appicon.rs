//! The playing app's own icon, from the shell: the media session's AppUserModelID names an entry
//! in the Apps folder (`shell:AppsFolder`), as for Chrome (`Chrome`), Edge (`MSEdge`) and Store
//! apps (`…!App`). Desktop apps that report their exe name instead (Spotify: `Spotify.exe`) aren't
//! in that folder, so their icon comes from the running process's exe.
use pk::wide::{from_buf, to_wide};
use windows::Win32::Foundation::SIZE;
use windows::Win32::Graphics::Gdi::HBITMAP;
use windows::Win32::UI::Shell::{
    FOLDERID_AppsFolder, IShellItemImageFactory, KF_FLAG_DEFAULT, SHCreateItemFromParsingName, SHCreateItemInKnownFolder,
    SIIGBF_ICONONLY, SIIGBF_RESIZETOFIT,
};
use windows_core::PCWSTR;
use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION, QueryFullProcessImageNameW};

/// The app's icon as a `px`×`px` bitmap with alpha; the caller deletes it. Needs COM on this thread.
pub fn bitmap(aumid: &str, px: u32) -> Option<HBITMAP> {
    let factory = image_factory(aumid)?;
    let size = SIZE { cx: px as i32, cy: px as i32 };
    unsafe { factory.GetImage(size, SIIGBF_ICONONLY | SIIGBF_RESIZETOFIT) }.ok()
}

fn image_factory(aumid: &str) -> Option<IShellItemImageFactory> {
    if aumid.is_empty() {
        return None;
    }
    let id = to_wide(aumid);
    if let Ok(f) = unsafe { SHCreateItemInKnownFolder(&FOLDERID_AppsFolder, KF_FLAG_DEFAULT, PCWSTR(id.as_ptr())) } {
        return Some(f);
    }
    let path = to_wide(&running_exe_path(aumid)?);
    unsafe { SHCreateItemFromParsingName(PCWSTR(path.as_ptr()), None) }.ok()
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
