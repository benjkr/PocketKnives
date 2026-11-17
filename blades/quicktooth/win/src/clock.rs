//! Time for the host: a stopwatch on QueryPerformanceCounter, and the wall clock.
use windows_sys::Win32::Foundation::FILETIME;
use windows_sys::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};
use windows_sys::Win32::System::SystemInformation::GetSystemTimePreciseAsFileTime;

/// Milliseconds since it was started (C#'s `Stopwatch`).
pub struct Stopwatch {
    start: i64,
    freq: f64,
}

impl Stopwatch {
    pub fn start() -> Stopwatch {
        let mut f = 0i64;
        unsafe { QueryPerformanceFrequency(&mut f) };
        Stopwatch { start: ticks(), freq: f as f64 }
    }

    pub fn restart(&mut self) {
        self.start = ticks();
    }

    pub fn elapsed_ms(&self) -> f64 {
        (ticks() - self.start) as f64 * 1000.0 / self.freq
    }

    /// Whole milliseconds, as `Stopwatch.ElapsedMilliseconds`.
    pub fn elapsed_whole_ms(&self) -> i64 {
        (ticks() - self.start) * 1000 / self.freq as i64
    }
}

fn ticks() -> i64 {
    let mut t = 0i64;
    unsafe { QueryPerformanceCounter(&mut t) };
    t
}

/// Wall-clock milliseconds since 1970-01-01 UTC.
pub fn epoch_ms() -> f64 {
    let mut ft = FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 };
    unsafe { GetSystemTimePreciseAsFileTime(&mut ft) };
    let t = ((ft.dwHighDateTime as u64) << 32) | ft.dwLowDateTime as u64;
    // FILETIME counts 100 ns since 1601-01-01.
    (t as f64 - 116_444_736_000_000_000.0) / 10_000.0
}
