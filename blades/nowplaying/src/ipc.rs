//! Single-instance identity and cross-process messages for NowPlaying.
use pk::instance::{NamedMutex, find_window, register_message};
use pk::w;
use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::UI::WindowsAndMessaging::PostMessageW;
use windows_sys::core::PCWSTR;

pub const MUTEX_NAME: PCWSTR = w!("Local\\PocketKnives.NowPlaying.v1");
pub const WINDOW_CLASS: PCWSTR = w!("PocketKnives.NowPlaying.Strip");
pub const RELOAD_MSG_NAME: PCWSTR = w!("PocketKnives.NowPlaying.Reload.v1");
pub const EXIT_MSG_NAME: PCWSTR = w!("PocketKnives.NowPlaying.Exit.v1");

pub fn acquire_single_instance() -> Option<NamedMutex> {
    NamedMutex::first(MUTEX_NAME)
}

pub fn find_running() -> HWND {
    find_window(WINDOW_CLASS)
}

pub fn reload_message() -> u32 {
    register_message(RELOAD_MSG_NAME)
}

pub fn exit_message() -> u32 {
    register_message(EXIT_MSG_NAME)
}

pub fn notify_reload() {
    let hwnd = find_running();
    if !hwnd.is_null() {
        unsafe { PostMessageW(hwnd, reload_message(), 0, 0) };
    }
}

pub fn request_exit() -> bool {
    let hwnd = find_running();
    if hwnd.is_null() {
        return false;
    }
    unsafe { PostMessageW(hwnd, exit_message(), 0, 0) != 0 }
}
