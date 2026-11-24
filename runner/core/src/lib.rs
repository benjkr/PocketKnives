//! What the PocketKnives runner's two settings windows share: `pk.exe` (its XP and Windows 10
//! looks) and `winui\PocketKnives.Settings.exe` (the Windows 11 look). See docs/specs/runner.md.
//!
//! - [`settings`]: the suite's own settings (the look, which blades start at sign-in), the blade
//!   table, the preview switch, stopping the blades.
//! - [`blade_settings`]: each blade's settings, read from and changed through the blade
//!   (docs/specs/blade-settings.md), and hotkey values.
//! - [`link`]: the messages the runner's window answers.
//! - [`process`]: starting a blade, with its output and a time limit.
//!
//! An error is the text the window shows the user.
#![no_std]

extern crate alloc;
#[cfg(test)]
extern crate std;

pub mod blade_settings;
mod files;
pub mod link;
pub mod process;
pub mod settings;
mod shell_link;

/// `File.Exists`: a file (not a folder) is there.
pub use files::exists;

/// A failure, as the sentence the settings window shows.
pub type Result<T> = core::result::Result<T, alloc::string::String>;

use windows_sys::Win32::Foundation::RECT;
use windows_sys::Win32::UI::WindowsAndMessaging::{SPI_GETWORKAREA, SystemParametersInfoW};

/// `dir\name`.
pub fn join(dir: &str, name: &str) -> alloc::string::String {
    alloc::format!("{dir}\\{name}")
}

/// The primary monitor's work area `(width, height)` in 96-DPI logical pixels, or `None` if
/// Windows doesn't report one.
pub fn work_area(dpi: u32) -> Option<(i32, i32)> {
    let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
    if unsafe { SystemParametersInfoW(SPI_GETWORKAREA, 0, (&mut rect as *mut RECT).cast(), 0) } == 0 {
        return None;
    }
    let dpi = dpi.max(96) as i64;
    let width = ((rect.right - rect.left) as i64 * 96 / dpi) as i32;
    let height = ((rect.bottom - rect.top) as i64 * 96 / dpi) as i32;
    (width > 0 && height > 0).then_some((width, height))
}

/// Fits a window extent (in 96-DPI logical pixels) to at least `min` and at most 85% of the
/// monitor's work area (or `work - 32` when `min` already exceeds 85%).
pub fn fit_work_area(needed: i32, min: i32, work: Option<i32>) -> i32 {
    let wanted = needed.max(min);
    let Some(work) = work else {
        return wanted;
    };
    let cap = (work * 85 / 100).max(min.min((work - 32).max(160)));
    wanted.min(cap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fits_work_area() {
        assert_eq!(fit_work_area(200, 262, Some(1040)), 262);
        assert_eq!(fit_work_area(322, 262, Some(1040)), 322);
        assert_eq!(fit_work_area(1200, 262, Some(1040)), 884);
        assert_eq!(fit_work_area(700, 500, Some(520)), 488);
        assert_eq!(fit_work_area(400, 262, None), 400);
    }
}
