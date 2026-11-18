//! `pk`: what a PocketKnives blade exe needs in place of the Rust standard library.
//!
//! The blades are `no_std` so each exe stays a few kilobytes plus its own code (std alone costs
//! ~100 KB). Most of it calls kernel32 or user32 only; [`reg`] and [`env::known_folder`] add
//! advapi32 and shell32, imported only by an exe that calls them: whole-program LTO drops whatever
//! an exe doesn't call.
//!
//! - [`entry!`]: the process entry point, plus the runtime pieces (panic handler, allocator,
//!   `mem*`; [`rt`]) defined in the exe, so `pk` itself is an ordinary, testable library.
//! - [`heap`]: the global allocator on the process heap (`alloc::{vec, string, boxed}` work).
//! - [`args`]: the command line, split exactly like the Microsoft C runtime and Rust's std.
//! - [`out`]: writing to stdout, a pipe or the parent's console.
//! - [`wide`]: UTF-16 helpers. [`env`]: environment, exe path, known folders, the Settings
//!   preview switch. [`error`]: Windows' text for an error code. [`reg`]: the registry.
//! - [`instance`]: a named mutex (one copy per session), finding a running copy's window.
//! - [`sync`], [`thread`]: a lock and threads, for blades with a worker thread.
#![no_std]

extern crate alloc;

pub mod args;
pub mod env;
pub mod error;
pub mod heap;
pub mod instance;
pub mod out;
pub mod panic;
pub mod reg;
pub mod sync;
pub mod thread;
pub mod wide;

pub mod rt;

pub use windows_sys::w;

/// Ends the process now with `code`. Nothing is torn down first: Windows cleans up.
pub fn exit(code: u32) -> ! {
    unsafe { windows_sys::Win32::System::Threading::ExitProcess(code) }
}

/// Defines the exe's entry point, `WinMainCRTStartup`, calling `$main() -> u32` and exiting with
/// its result, and the runtime a `/NODEFAULTLIB` exe needs ([`rt`]: panic handler, allocator,
/// `mem*`). The exe links with `/ENTRY:WinMainCRTStartup /SUBSYSTEM:WINDOWS /NODEFAULTLIB` (its
/// build.rs). Use it once, at the exe's crate root.
#[macro_export]
macro_rules! entry {
    ($main:path) => {
        #[unsafe(no_mangle)]
        pub extern "system" fn WinMainCRTStartup() -> ! {
            let code: u32 = $main();
            $crate::exit(code)
        }
        $crate::runtime!();
    };
}
