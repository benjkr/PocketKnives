//! QuickTooth.exe: the Windows host. With no argument it runs the tray; with one, a command
//! (docs/specs/settings-and-cli.md).
//!
//! - [`tray`]: the hidden window, `Shell_NotifyIcon`, timers, menus; the controller's host.
//! - [`cli`]: every command-line option.
//! - [`worker`]: the device thread; [`audio`] (Core Audio, KS), [`pnp`] (cfgmgr32), [`ble`]
//!   (the battery scan) are the device layer.
//! - [`files`] (settings.json, the log), [`system`] (autostart, theme, promotion, dark menus),
//!   [`ipc`] (names other programs find the tray by), [`clock`], [`selftest`].
//! - [`icons`]: icon pixels to HICONs, and the `--render-icons` sheets.
#![no_std]
#![no_main]

extern crate alloc;

mod audio;
mod ble;
mod cli;
mod clock;
mod files;
mod icons;
mod ipc;
mod pnp;
mod selftest;
mod system;
mod tray;
mod worker;

pk::entry!(main);

fn main() -> u32 {
    quicktooth_core::log::set_sink(files::log_sink);
    quicktooth_core::settings::set_save_handler(files::save_settings);
    let args = pk::args::args();
    if args.is_empty() { tray::run() } else { cli::run(&args) }
}
