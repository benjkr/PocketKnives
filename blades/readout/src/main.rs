//! Readout.exe: a calculator-style system monitor bar along the top of the screen.
//! Rust, `no_std` on `pk`.
#![no_std]
#![no_main]

extern crate alloc;

mod bar;
mod cli;
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
