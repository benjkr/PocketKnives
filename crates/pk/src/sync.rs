//! A mutex (an SRW lock, what std's `Mutex` uses on Windows) for state shared between threads.
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use windows_sys::Win32::System::Threading::{AcquireSRWLockExclusive, ReleaseSRWLockExclusive, SRWLOCK, SRWLOCK_INIT};

pub struct Mutex<T> {
    lock: UnsafeCell<SRWLOCK>,
    value: UnsafeCell<T>,
}

// The lock serialises every access to `value`.
unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Mutex { lock: UnsafeCell::new(SRWLOCK_INIT), value: UnsafeCell::new(value) }
    }

    pub fn lock(&self) -> Guard<'_, T> {
        unsafe { AcquireSRWLockExclusive(self.lock.get()) };
        Guard { m: self }
    }
}

pub struct Guard<'a, T> {
    m: &'a Mutex<T>,
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.m.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.m.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        unsafe { ReleaseSRWLockExclusive(self.m.lock.get()) };
    }
}
