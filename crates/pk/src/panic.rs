//! What a blade does with a panic before the process ends: usually one line in its log, so a
//! tray icon that vanished leaves a reason behind. The hook gets nothing from the `PanicInfo`:
//! reading its location keeps every panic site's file and line in the exe (14 KB for
//! QuickTooth), and its message all their formatting (28 KB).
use core::sync::atomic::{AtomicBool, AtomicPtr, Ordering};

static HOOK: AtomicPtr<()> = AtomicPtr::new(core::ptr::null_mut());
// Only the panic handler reads these, and `cargo test` builds without it (it links std).
#[cfg_attr(test, allow(dead_code))]
static REPORTED: AtomicBool = AtomicBool::new(false);

/// Called once when the process panics, on the thread that panicked. It must not wait for a lock
/// the panicking code may hold.
pub fn set_hook(f: fn()) {
    HOOK.store(f as *mut (), Ordering::Release);
}

/// Runs the hook (not again if the hook itself panics).
#[cfg_attr(test, allow(dead_code))]
pub(crate) fn report() {
    if REPORTED.swap(true, Ordering::AcqRel) {
        return;
    }
    let p = HOOK.load(Ordering::Acquire);
    if !p.is_null() {
        // Only ever a `fn()` stored by `set_hook`.
        let f = unsafe { core::mem::transmute::<*mut (), fn()>(p) };
        f();
    }
}
