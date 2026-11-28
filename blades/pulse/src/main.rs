//! Pulse.exe: a calculator-style system monitor strip in the taskbar.
//! Rust, `no_std` on `pk`.
#![no_std]
#![no_main]

extern crate alloc;

mod bar;
mod cli;
mod icons;
mod lcd;
mod letters;
mod metrics;
mod model;
mod procs;
mod raster;
mod settings;
mod status;
mod theme;
mod window;

pk::entry!(main);

fn main() -> u32 {
    let args = pk::args::args();
    if args.is_empty() { window::run() } else { cli::run(&args) }
}
