//! Generated Windows COM and WinRT bindings for PocketKnives.
//!
//! Regenerate with `cargo run -p pk-windows-gen` whenever a blade calls a new WinRT or COM API.
#![no_std]

extern crate alloc;

#[allow(
    clippy::all,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unsafe_op_in_unsafe_fn,
    unused_must_use,
    unused_variables
)]
#[rustfmt::skip]
mod bindings;

pub use bindings::Windows::*;
