//! What the running tray and other QuickTooth.exe copies (and the installer and the runner)
//! agree on. Never change these names: other programs find the tray by them.
use windows_sys::Win32::Foundation::HWND;
use windows_sys::core::PCWSTR;

pub const WINDOW_CLASS: PCWSTR = pk::w!("QuickTooth.TrayWindow");
pub const COMMAND_MESSAGE: PCWSTR = pk::w!("QuickTooth.Command.v1");
pub const MUTEX: PCWSTR = pk::w!("Local\\QuickTooth.Tray.6F0C2B1E");
pub const ICON_ID: u32 = 1;
/// `WM_COPYDATA` `dwData` of a `--set` handed to the tray ("QTS1"), and the most it reads.
pub const SET_SETTINGS_DATA: usize = 0x5154_5331;
pub const SET_SETTINGS_MAX_BYTES: u32 = 16 * 1024;
pub const SET_SETTINGS_TIMEOUT_MS: u32 = 5000;

/// The running tray's window (null when none runs).
pub fn find_tray() -> HWND {
    pk::instance::find_window(WINDOW_CLASS)
}

pub fn command_message() -> u32 {
    pk::instance::register_message(COMMAND_MESSAGE)
}
