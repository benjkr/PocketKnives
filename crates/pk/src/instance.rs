//! One running copy per user session, and finding it from another copy.
use windows_sys::Win32::Foundation::{CloseHandle, ERROR_ACCESS_DENIED, ERROR_ALREADY_EXISTS, GetLastError, HANDLE, HWND};
use windows_sys::Win32::Storage::FileSystem::SYNCHRONIZE;
use windows_sys::Win32::System::Threading::{CreateMutexW, OpenMutexW};
use windows_sys::Win32::UI::WindowsAndMessaging::{FindWindowW, RegisterWindowMessageW};
use windows_sys::core::PCWSTR;

/// A named mutex this process created. Closed (and the name released) when dropped.
pub struct NamedMutex(HANDLE);

impl NamedMutex {
    /// Creates the mutex `name` (for example `Local\QuickTooth.Tray.6F0C2B1E`). `None` when another
    /// process already has it: a copy is running in this session.
    pub fn first(name: PCWSTR) -> Option<NamedMutex> {
        let h = unsafe { CreateMutexW(core::ptr::null(), 0, name) };
        if h.is_null() {
            return None;
        }
        if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
            unsafe { CloseHandle(h) };
            return None;
        }
        Some(NamedMutex(h))
    }

    /// True when a process in this session currently holds the named mutex `name`.
    pub fn exists(name: PCWSTR) -> bool {
        let h = unsafe { OpenMutexW(SYNCHRONIZE, 0, name) };
        if !h.is_null() {
            unsafe { CloseHandle(h) };
            return true;
        }
        unsafe { GetLastError() == ERROR_ACCESS_DENIED }
    }

    /// Keeps the mutex until the process ends.
    pub fn keep(self) {
        core::mem::forget(self);
    }
}

impl Drop for NamedMutex {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.0) };
    }
}

/// The running copy's window, by its class name (null when none runs).
pub fn find_window(class: PCWSTR) -> HWND {
    unsafe { FindWindowW(class, core::ptr::null()) }
}

/// The session-wide message number for `name` (the same in every process).
pub fn register_message(name: PCWSTR) -> u32 {
    unsafe { RegisterWindowMessageW(name) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wide::to_wide;

    #[test]
    fn exists_tracks_live_mutex() {
        let name = to_wide("Local\\PocketKnives.Test.InstanceExists");
        assert!(!NamedMutex::exists(name.as_ptr()));
        let guard = NamedMutex::first(name.as_ptr()).expect("create test mutex");
        assert!(NamedMutex::exists(name.as_ptr()));
        assert!(NamedMutex::first(name.as_ptr()).is_none());
        drop(guard);
        assert!(!NamedMutex::exists(name.as_ptr()));
    }
}
