//! The device worker: one MTA thread that owns every Core Audio and KS COM object
//! ([`crate::audio::Audio`]) and runs device jobs one at a time, so the UI thread never blocks on
//! a device call and an enumeration never interleaves with a send.
use crate::audio::Audio;
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use core::ptr::{null, null_mut};
use pk::sync::Mutex;
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
use windows_sys::Win32::System::Com::{COINIT_MULTITHREADED, CoInitializeEx};
use windows_sys::Win32::System::Threading::{CreateEventW, CreateSemaphoreW, INFINITE, ReleaseSemaphore, SetEvent, WaitForSingleObject};

type Job = Box<dyn FnOnce(&mut Audio) + Send>;

/// A kernel handle shared between threads.
pub struct Handle(pub HANDLE);
unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}

impl Handle {
    /// A manual-reset event, not set.
    pub fn event() -> Handle {
        Handle(unsafe { CreateEventW(null(), 1, 0, null()) })
    }

    pub fn set(&self) {
        unsafe { SetEvent(self.0) };
    }

    /// Waits until set or `ms` passed (`INFINITE` for no limit).
    pub fn wait(&self, ms: u32) {
        unsafe { WaitForSingleObject(self.0, ms) };
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.0) };
    }
}

struct Shared {
    jobs: Mutex<VecDeque<Job>>,
    /// Counts queued jobs.
    ready: Handle,
}

#[derive(Clone)]
pub struct Worker(Arc<Shared>);

impl Worker {
    pub fn start() -> Worker {
        let shared =
            Arc::new(Shared { jobs: Mutex::new(VecDeque::new()), ready: Handle(unsafe { CreateSemaphoreW(null(), 0, i32::MAX, null()) }) });
        let s = shared.clone();
        // Detached: it lives as long as the process, like C#'s background thread. If Windows
        // can't start it the process can't do anything useful (C#'s Thread.Start threw).
        let started = pk::thread::spawn(move || {
            unsafe { CoInitializeEx(null(), COINIT_MULTITHREADED as u32) };
            let mut audio = Audio::new();
            loop {
                s.ready.wait(INFINITE);
                let job = s.jobs.lock().pop_front();
                if let Some(job) = job {
                    job(&mut audio);
                }
            }
        });
        if started.is_none() {
            quicktooth_core::log::error("device worker", "the thread could not be started");
            pk::exit(1);
        }
        Worker(shared)
    }

    /// Queues `job`; it runs on the worker thread.
    pub fn post(&self, job: impl FnOnce(&mut Audio) + Send + 'static) {
        self.0.jobs.lock().push_back(Box::new(job));
        unsafe { ReleaseSemaphore(self.0.ready.0, 1, null_mut()) };
    }

    /// Runs `f` on the worker thread and waits for its result (the command line).
    pub fn call<T: Send + 'static>(&self, f: impl FnOnce(&mut Audio) -> T + Send + 'static) -> T {
        let slot = Arc::new((Mutex::new(None), Handle::event()));
        let s = slot.clone();
        self.post(move |a| {
            *s.0.lock() = Some(f(a));
            s.1.set();
        });
        slot.1.wait(INFINITE);
        slot.0.lock().take().expect("worker result")
    }
}
