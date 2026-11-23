//! Post-recording finish steps (after GPU / WGC / Media Foundation are fully torn down):
//! - SnipSnap's 150 ms white flash over the recorded area
//! - `CF_HDROP` clipboard copy so `Ctrl+V` pastes the `.mp4` immediately
//! - Native Windows 11 notification (`Shell_NotifyIconW` `NIF_INFO`) with click-to-open-folder
use crate::ipc::{WM_APP_SAVE_DONE, WM_APP_TRAY};
use crate::ui::format_size;
use alloc::format;
use alloc::string::String;
use core::ffi::c_void;
use core::ptr::{null, null_mut};
use pk::wide::to_wide;
use windows_sys::Win32::Foundation::{CloseHandle, GlobalFree, HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows_sys::Win32::Graphics::Dwm::{DWMWA_TRANSITIONS_FORCEDISABLED, DwmSetWindowAttribute};
use windows_sys::Win32::Graphics::Gdi::{GetStockObject, UpdateWindow, WHITE_BRUSH};
use windows_sys::Win32::System::DataExchange::{CloseClipboard, EmptyClipboard, OpenClipboard, SetClipboardData};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::Memory::{GMEM_MOVEABLE, GMEM_ZEROINIT, GlobalAlloc, GlobalLock, GlobalUnlock};
use windows_sys::Win32::System::Ole::CF_HDROP;
use windows_sys::Win32::System::SystemInformation::GetTickCount64;
use windows_sys::Win32::System::Threading::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW, Sleep};
use windows_sys::Win32::UI::Shell::{
    DROPFILES, NIF_ICON, NIF_INFO, NIF_MESSAGE, NIF_TIP, NIIF_LARGE_ICON, NIIF_USER, NIM_ADD, NIM_DELETE, NOTIFYICONDATAW,
    Shell_NotifyIconW,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyIcon, DestroyWindow, DispatchMessageW, GetMessageW, IMAGE_ICON, LR_DEFAULTSIZE, LWA_ALPHA,
    LoadImageW, MSG, RegisterClassW, SW_SHOWNOACTIVATE, SetLayeredWindowAttributes, SetTimer, ShowWindow, TranslateMessage,
    USER_TIMER_MINIMUM, WM_DESTROY, WM_TIMER, WNDCLASSW, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST,
    WS_EX_TRANSPARENT, WS_POPUP,
};

const FLASH_CLASS: *const u16 = pk::w!("SnipSnapFlash");
const FLASH_ALPHA: u64 = 150;
const FLASH_MS: u64 = 150;
const NIN_BALLOONUSERCLICK: u32 = 0x0405;
const NIN_BALLOONTIMEOUT: u32 = 0x0404;

static mut FLASH_HWND: HWND = null_mut();
static mut FLASH_T0: u64 = 0;

unsafe extern "system" fn flash_proc(h: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    if msg == WM_TIMER {
        let ms = unsafe { GetTickCount64() } - unsafe { FLASH_T0 };
        if ms >= FLASH_MS {
            unsafe { DestroyWindow(h) };
            return 0;
        }
        unsafe { SetLayeredWindowAttributes(h, 0, (FLASH_ALPHA * (FLASH_MS - ms) / FLASH_MS) as u8, LWA_ALPHA) };
        return 0;
    }
    if msg == WM_DESTROY && h == unsafe { FLASH_HWND } {
        unsafe { FLASH_HWND = null_mut() };
    }
    unsafe { DefWindowProcW(h, msg, wp, lp) }
}

/// Shows SnipSnap's 150 ms white flash over `r` and pumps messages until it fades out.
/// Calls `on_save_done` if `WM_APP_SAVE_DONE` arrives mid-flash so clipboard copy happens as
/// soon as the background MP4 finalize finishes (~72 ms) without waiting for the fade to end.
pub fn show_flash<F: FnMut()>(r: RECT, mut on_save_done: F) {
    let rw = r.right - r.left;
    let rh = r.bottom - r.top;
    if rw <= 0 || rh <= 0 {
        return;
    }
    let hinst = unsafe { GetModuleHandleW(null()) };
    let mut wc: WNDCLASSW = unsafe { core::mem::zeroed() };
    wc.lpfnWndProc = Some(flash_proc);
    wc.hInstance = hinst;
    wc.hbrBackground = unsafe { GetStockObject(WHITE_BRUSH) };
    wc.lpszClassName = FLASH_CLASS;
    unsafe { RegisterClassW(&wc) };

    let h = unsafe {
        CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
            FLASH_CLASS,
            null(),
            WS_POPUP,
            r.left,
            r.top,
            rw,
            rh,
            null_mut(),
            null_mut(),
            hinst,
            null(),
        )
    };
    if h.is_null() {
        return;
    }
    unsafe {
        FLASH_HWND = h;
        let on = 1i32;
        DwmSetWindowAttribute(h, DWMWA_TRANSITIONS_FORCEDISABLED as u32, &on as *const i32 as *const c_void, 4);
        SetLayeredWindowAttributes(h, 0, FLASH_ALPHA as u8, LWA_ALPHA);
        ShowWindow(h, SW_SHOWNOACTIVATE);
        UpdateWindow(h);
        FLASH_T0 = GetTickCount64();
        SetTimer(h, 1, USER_TIMER_MINIMUM, None);
    }
    let mut m: MSG = unsafe { core::mem::zeroed() };
    while !unsafe { FLASH_HWND }.is_null() {
        if unsafe { GetMessageW(&mut m, null_mut(), 0, 0) } <= 0 {
            break;
        }
        if m.message == WM_APP_SAVE_DONE {
            on_save_done();
            continue;
        }
        unsafe {
            TranslateMessage(&m);
            DispatchMessageW(&m);
        }
    }
}

/// Places `path` on the clipboard as `CF_HDROP` so `Ctrl+V` pastes the `.mp4` into Discord,
/// Slack, Teams, browsers and Explorer.
pub fn copy_file_to_clipboard(owner: HWND, path: &str) -> bool {
    let wide = to_wide(path); // includes one trailing 0; CF_HDROP needs double-NUL
    let header_bytes = size_of::<DROPFILES>();
    let path_bytes = (wide.len() + 1) * 2;
    let total = header_bytes + path_bytes;

    let hmem = unsafe { GlobalAlloc(GMEM_MOVEABLE | GMEM_ZEROINIT, total) };
    if hmem.is_null() {
        return false;
    }
    let ptr = unsafe { GlobalLock(hmem) } as *mut u8;
    if ptr.is_null() {
        unsafe { GlobalFree(hmem) };
        return false;
    }
    let df = DROPFILES { pFiles: header_bytes as u32, pt: POINT { x: 0, y: 0 }, fNC: 0, fWide: 1 };
    unsafe {
        (ptr as *mut DROPFILES).write_unaligned(df);
        let dst = core::slice::from_raw_parts_mut(ptr.add(header_bytes) as *mut u16, wide.len() + 1);
        dst[..wide.len()].copy_from_slice(&wide);
        dst[wide.len()] = 0;
        GlobalUnlock(hmem);
    }

    let mut opened = false;
    for _ in 0..20 {
        if unsafe { OpenClipboard(owner) } != 0 {
            opened = true;
            break;
        }
        unsafe { Sleep(10) };
    }
    if !opened {
        unsafe { GlobalFree(hmem) };
        return false;
    }
    let ok = unsafe {
        EmptyClipboard();
        let ok = !SetClipboardData(CF_HDROP as u32, hmem).is_null();
        CloseClipboard();
        ok
    };
    if !ok {
        unsafe { GlobalFree(hmem) };
    }
    ok
}

fn copy_wstr(dst: &mut [u16], src: &str) {
    let w = to_wide(src);
    let n = w.len().min(dst.len());
    if n > 0 {
        dst[..n].copy_from_slice(&w[..n]);
        dst[n - 1] = 0;
    }
}

/// Launches `explorer.exe /select,"<path>"` when the user clicks the saved notification.
pub fn reveal_in_explorer(path: &str) {
    let mut cmd = to_wide(&format!("explorer.exe /select,\"{path}\""));
    let mut si: STARTUPINFOW = unsafe { core::mem::zeroed() };
    si.cb = size_of::<STARTUPINFOW>() as u32;
    let mut pi: PROCESS_INFORMATION = unsafe { core::mem::zeroed() };
    if unsafe { CreateProcessW(null(), cmd.as_mut_ptr(), null(), null(), 0, 0, null(), null(), &si, &mut pi) } != 0 {
        unsafe {
            CloseHandle(pi.hThread);
            CloseHandle(pi.hProcess);
        }
    }
}

/// Shows a native Windows 11 notification (`Shell_NotifyIconW` `NIF_INFO`) and pumps messages
/// briefly (~5.5 s) so clicking the notification reveals the saved `.mp4` in Explorer.
/// Runs with 0 MB GPU memory because `CaptureSession` / `GpuContext` are already dropped.
pub fn show_saved_notification(host: HWND, path: &str, bytes: u64, copied: bool) {
    let file_name = path.rsplit('\\').next().unwrap_or(path);
    let size = format_size(bytes);
    let body = if copied { format!("{file_name} · {size}\nCopied to clipboard") } else { format!("{file_name} · {size}") };

    let hinst = unsafe { GetModuleHandleW(null()) };
    let icon = unsafe { LoadImageW(hinst, 1usize as *const u16, IMAGE_ICON, 0, 0, LR_DEFAULTSIZE) };

    let mut nid: NOTIFYICONDATAW = unsafe { core::mem::zeroed() };
    nid.cbSize = size_of::<NOTIFYICONDATAW>() as u32;
    nid.hWnd = host;
    nid.uID = 1;
    nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP | NIF_INFO;
    nid.uCallbackMessage = WM_APP_TRAY;
    nid.hIcon = icon;
    nid.dwInfoFlags = NIIF_USER | NIIF_LARGE_ICON;
    nid.hBalloonIcon = icon;
    copy_wstr(&mut nid.szTip, "SnipSnap");
    copy_wstr(&mut nid.szInfoTitle, "Recording saved");
    copy_wstr(&mut nid.szInfo, &body);

    if unsafe { Shell_NotifyIconW(NIM_ADD, &nid) } == 0 {
        if !icon.is_null() {
            unsafe { DestroyIcon(icon) };
        }
        return;
    }

    // Wait up to ~5.5 s for a click or balloon dismiss, then clean up and exit.
    let deadline = unsafe { GetTickCount64() } + 5500;
    unsafe { SetTimer(host, 99, 200, None) };
    let mut m: MSG = unsafe { core::mem::zeroed() };
    while unsafe { GetTickCount64() } < deadline {
        if unsafe { GetMessageW(&mut m, null_mut(), 0, 0) } <= 0 {
            break;
        }
        if m.message == WM_APP_TRAY {
            let ev = (m.lParam & 0xFFFF) as u32;
            if ev == NIN_BALLOONUSERCLICK {
                reveal_in_explorer(path);
                break;
            }
            if ev == NIN_BALLOONTIMEOUT {
                break;
            }
        }
        unsafe {
            TranslateMessage(&m);
            DispatchMessageW(&m);
        }
    }

    let _ = unsafe { Shell_NotifyIconW(NIM_DELETE, &nid) };
    if !icon.is_null() {
        unsafe { DestroyIcon(icon) };
    }
}

#[allow(dead_code)]
fn _keep(_: String) {}
