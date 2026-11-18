//! The app's log. Lines go to a sink each host sets once at start, which adds the timestamp and
//! process ID: the exe appends to `%APPDATA%\QuickTooth\quicktooth.log`, the simulator to its
//! virtual-time log. The wording of every line is behaviour (AGENTS.md): it's compared across
//! builds and with the simulator.
use core::sync::atomic::{AtomicPtr, Ordering};

/// Defines a per-process function-pointer hook, set by the host (no allocation, any thread).
macro_rules! fn_hook {
    ($(#[$m:meta])* $name:ident, $ty:ty) => {
        $(#[$m])*
        pub struct $name(AtomicPtr<()>);
        impl $name {
            pub const fn new() -> Self {
                $name(AtomicPtr::new(core::ptr::null_mut()))
            }
            pub fn set(&self, f: $ty) {
                self.0.store(f as *mut (), Ordering::Release);
            }
            pub fn get(&self) -> Option<$ty> {
                let p = self.0.load(Ordering::Acquire);
                // Only ever a value stored by `set`: a fn pointer of exactly this type.
                (!p.is_null()).then(|| unsafe { core::mem::transmute::<*mut (), $ty>(p) })
            }
        }
    };
}

fn_hook!(
    /// The log sink.
    SinkHook,
    fn(&str)
);
fn_hook!(
    /// The settings saver.
    SaveHook,
    fn(&crate::settings::Settings) -> bool
);

static SINK: SinkHook = SinkHook::new();

/// Where log lines go (each host calls this once, before anything logs).
pub fn set_sink(f: fn(&str)) {
    SINK.set(f);
}

pub fn info(message: &str) {
    if let Some(f) = SINK.get() {
        f(message);
    }
}

/// `ERROR <context>: <error>` (the C# version wrote the exception there).
pub fn error(context: &str, error: &str) {
    info(&alloc::format!("ERROR {context}: {error}"));
}

/// `log!("...", args)`: formats and logs one line.
#[macro_export]
macro_rules! log {
    ($($t:tt)*) => { $crate::log::info(&alloc::format!($($t)*)) };
}
