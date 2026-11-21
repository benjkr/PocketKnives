//! QuickTooth's shared code: everything that decides what the tray looks like and does. No
//! Windows APIs: the exe (`../win`) and the simulator's WebAssembly build (`../web`) only adapt the
//! platform through [`host::TrayHost`] and [`host::DeviceBackend`]
//! (docs/working/architecture.md).
//!
//! Rules that hold for every file here:
//! - Time comes only from the host (`now_ms`, `now_epoch_ms`, `delay`, `set_timer`).
//! - Numbers are in [`tunables`], by name.
//! - No per-app state in statics: the simulator runs several apps in one page. Per-process hooks
//!   (the log sink, the settings saver) are set once by each host.
//! - The controller is single-threaded: host calls and task wake-ups happen on the UI thread.
#![no_std]

extern crate alloc;

pub mod battery;
pub mod blade_settings;
pub mod controller;
pub mod devices;
pub mod exec;
pub mod guid;
pub mod host;
pub mod icon;
pub mod json;
pub mod log;
pub mod math;
pub mod num;
pub mod popup;
pub mod settings;
pub mod tunables;

pub use guid::Guid;
