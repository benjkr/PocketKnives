//! `pk`: what a PocketKnives blade exe needs in place of the Rust standard library.
//!
//! The blades are `no_std` so each exe stays a few kilobytes plus its own code (std alone costs
//! ~100 KB). Everything here calls kernel32 or user32 only, so a blade that uses it still loads
//! nothing else, and whole-program LTO drops whatever a blade doesn't call.
//!
//! - [`entry!`]: the process entry point; the runtime pieces (panic, `mem*`) are in `rt`.
//! - [`heap`]: the global allocator on the process heap (`alloc::{vec, string, boxed}` work).
//! - [`args`]: the command line, split exactly like the Microsoft C runtime and Rust's std.
//! - [`out`]: writing to stdout, a pipe or the parent's console.
//! - [`wide`]: UTF-16 helpers. [`env`]: environment, exe path, the Settings preview switch.
//! - [`instance`]: a named mutex (one copy per session), finding a running copy's window.
//! - [`sync`], [`thread`]: a lock and threads, for blades with a worker thread.
#![no_std]

extern crate alloc;

pub mod args;
pub mod env;
pub mod heap;
pub mod instance;
pub mod out;
pub mod panic;
pub mod sync;
pub mod thread;
pub mod wide;

#[cfg(not(test))]
mod rt;

pub use windows_sys::w;

/// Ends the process now with `code`. Nothing is torn down first: Windows cleans up.
pub fn exit(code: u32) -> ! {
    unsafe { windows_sys::Win32::System::Threading::ExitProcess(code) }
}

/// Defines the exe's entry point, `WinMainCRTStartup`, calling `$main() -> u32` and exiting with
/// its result. The exe links with `/ENTRY:WinMainCRTStartup /SUBSYSTEM:WINDOWS /NODEFAULTLIB`
/// (its build.rs).
#[macro_export]
macro_rules! entry {
    ($main:path) => {
        #[unsafe(no_mangle)]
        pub extern "system" fn WinMainCRTStartup() -> ! {
            let code: u32 = $main();
            $crate::exit(code)
        }
    };
}
