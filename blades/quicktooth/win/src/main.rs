//! QuickTooth.exe: the Windows host. With no argument it runs the tray; with one, a command
//! (docs/specs/settings-and-cli.md).
//!
//! GROUNDWORK SKELETON: only `--render-icons` is wired, for the renderer's parity checks. The
//! tray, the CLI and the device layer replace this file's body.
#![no_std]
#![no_main]

extern crate alloc;

mod icons;

use core::fmt::Write;

pk::entry!(main);

fn main() -> u32 {
    let args = pk::args::args();
    let mut out = match pk::out::stdout_kind() {
        pk::out::StdoutKind::FileOrPipe => pk::out::Writer::new(pk::out::stdout_handle()),
        _ if pk::out::attach_parent_console() => pk::out::console_output().unwrap_or(pk::out::Writer::null()),
        _ => pk::out::Writer::null(),
    };
    match args.first().map(|a| a.to_ascii_lowercase()).as_deref() {
        Some("--render-icons") => {
            if let Some(i) = args.iter().position(|a| a.eq_ignore_ascii_case("--zoom")) {
                if let Some(z) = args.get(i + 1).and_then(|z| z.parse::<f64>().ok()) {
                    quicktooth_core::tunables::set_value(quicktooth_core::tunables::T::Zoom, z);
                }
            }
            let dir = match args.get(1) {
                Some(d) if !d.starts_with("--") => d.clone(),
                _ => alloc::format!("{}\\icon-check", pk::env::current_dir()),
            };
            for file in icons::write_check_images(&dir) {
                let _ = writeln!(out, "{file}");
            }
            0
        }
        _ => {
            let _ = writeln!(out, "QuickTooth (Rust groundwork build): only --render-icons works yet.");
            2
        }
    }
}
