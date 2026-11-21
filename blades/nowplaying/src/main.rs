//! NowPlaying.exe: live media cover, seekable timeline and 5-band audio frequency meter docked
//! in the Windows 11 taskbar. Rust, `no_std` on `pk`.
#![no_std]
#![no_main]

extern crate alloc;

mod audio;
mod cli;
mod fft;
mod ipc;
mod render;
mod settings;
mod smtc;
mod window;

pk::entry!(main);

fn main() -> u32 {
    let args = pk::args::args();
    if args.is_empty() { window::run() } else { cli::run(&args) }
}
