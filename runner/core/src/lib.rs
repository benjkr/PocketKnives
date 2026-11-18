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

/// A failure, as the sentence the settings window shows.
pub type Result<T> = core::result::Result<T, alloc::string::String>;

/// `dir\name`.
pub fn join(dir: &str, name: &str) -> alloc::string::String {
    alloc::format!("{dir}\\{name}")
}
