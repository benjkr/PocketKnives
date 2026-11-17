//! Starting a thread that runs a closure.
use alloc::boxed::Box;
use core::ffi::c_void;
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
use windows_sys::Win32::System::Threading::{CreateThread, INFINITE, WaitForSingleObject};

/// A started thread. Dropping it detaches the thread (it keeps running).
pub struct Thread(HANDLE);

impl Thread {
    /// Waits for the thread to end.
    pub fn join(self) {
        unsafe { WaitForSingleObject(self.0, INFINITE) };
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.0) };
    }
}

/// Runs `f` on a new thread. `None` if Windows couldn't start one.
pub fn spawn<F: FnOnce() + Send + 'static>(f: F) -> Option<Thread> {
    unsafe extern "system" fn start<F: FnOnce()>(p: *mut c_void) -> u32 {
        let f = unsafe { Box::from_raw(p as *mut F) };
        f();
        0
    }
    let p = Box::into_raw(Box::new(f)) as *mut c_void;
    let h = unsafe { CreateThread(core::ptr::null(), 0, Some(start::<F>), p, 0, core::ptr::null_mut()) };
    if h.is_null() {
        drop(unsafe { Box::from_raw(p as *mut F) });
        return None;
    }
    Some(Thread(h))
}
