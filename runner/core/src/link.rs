//! The registered messages the runner's hidden window answers: from a second `pk.exe` (open
//! Settings, exit) and from the Windows 11 app (reopen Settings in another look).
use pk::wide::to_wide;
use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::UI::WindowsAndMessaging::{PostMessageW, RegisterWindowMessageW};

pub const OPEN_MESSAGE: &str = "PocketKnives.OpenSettings.v1";
pub const REOPEN_MESSAGE: &str = "PocketKnives.ReopenSettings.v1";
pub const EXIT_MESSAGE: &str = "PocketKnives.Exit.v1";

/// The message's ID in this session.
pub fn register(name: &str) -> u32 {
    unsafe { RegisterWindowMessageW(to_wide(name).as_ptr()) }
}

/// Posts the named message to `window`; false when there's no window or the post failed.
pub fn post(window: HWND, name: &str) -> bool {
    !window.is_null() && unsafe { PostMessageW(window, register(name), 0, 0) } != 0
}
