//! Per-program load for the panels' top-ten lists, and ending a program.
//!
//! Every process comes from one `NtQuerySystemInformation(SystemProcessInformation)` snapshot:
//! no handle per process and no admin rights. A program is every process with one image name
//! (without ".exe", case-insensitive), summed. GPU comes from the GPU engine counter `metrics`
//! already reads, per process.
use alloc::string::String;
use alloc::vec::Vec;
use core::mem::size_of;
use windows_sys::Wdk::System::SystemInformation::{NtQuerySystemInformation, SystemProcessInformation};
use windows_sys::Win32::Foundation::{CloseHandle, ERROR_INVALID_PARAMETER, GetLastError, STATUS_INFO_LENGTH_MISMATCH, UNICODE_STRING};
use windows_sys::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};
use windows_sys::Win32::System::Threading::{
    ALL_PROCESSOR_GROUPS, GetActiveProcessorCount, GetCurrentProcessId, OpenProcess, PROCESS_TERMINATE, TerminateProcess,
};

/// One program: every running process with this image name, summed.
#[derive(Clone, Default)]
pub struct Proc {
    /// The image name without ".exe", as Windows reports it.
    pub name: String,
    /// One of its processes, for its icon.
    pub pid: u32,
    /// Its processes run in session 0, where services and the system run, not in a user's.
    pub service: bool,
    /// CPU, % of the whole machine (every logical processor), 0–100.
    pub cpu: f32,
    /// Private working set (Task Manager's "Memory" column), bytes.
    pub mem: u64,
    /// GPU, % as Task Manager's per-process GPU column (the busiest engine type), 0–100.
    pub gpu: f32,
    /// I/O of every kind (file, network, device), read + write + other, bytes a second.
    pub io: f64,
}

/// What ending a program came to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kill {
    /// Every process with the name ended.
    Ended,
    /// At least one couldn't be ended (protected, another user's, elevated, or Pulse itself).
    Denied,
    /// None was running any more.
    Gone,
}

/// Ends every process with this image name (without ".exe", case-insensitive): never the
/// idle process, System (pid 4) or Pulse itself, which count as denied.
pub fn kill(name: &str) -> Kill {
    let want: Vec<u16> = name.encode_utf16().collect();
    if want.is_empty() {
        return Kill::Gone;
    }
    let mut buf = Vec::new();
    let len = snapshot(&mut buf);
    if len == 0 {
        return Kill::Denied;
    }
    let me = unsafe { GetCurrentProcessId() };
    let (mut found, mut denied) = (false, false);
    for p in Entries::new(&buf, len) {
        let pid = p.pid as u32;
        if pid == 0 || !same(image(p), &want) {
            continue;
        }
        found = true;
        if pid == 4 || pid == me {
            denied = true;
            continue;
        }
        let h = unsafe { OpenProcess(PROCESS_TERMINATE, 0, pid) };
        if h.is_null() {
            // Invalid parameter: it ended since the snapshot.
            denied |= unsafe { GetLastError() } != ERROR_INVALID_PARAMETER;
            continue;
        }
        denied |= unsafe { TerminateProcess(h, 1) } == 0;
        unsafe { CloseHandle(h) };
    }
    match (found, denied) {
        (false, _) => Kill::Gone,
        (true, true) => Kill::Denied,
        (true, false) => Kill::Ended,
    }
}

/// Measures every program while the panels want them; `metrics::Collector` holds one only
/// while they're on, so they cost nothing otherwise.
pub struct Sampler {
    /// The last snapshot (u64 for the records' alignment).
    buf: Vec<u64>,
    /// Every process at the previous sample, by pid, and this sample's.
    prev: Vec<Seen>,
    cur: Vec<Seen>,
    /// `QueryPerformanceCounter` at the previous snapshot; 0 before the first.
    time: i64,
    qpc_freq: f64,
    cpus: f64,
    /// (pid, engine type hash, utilisation) of this sample's GPU engines, filled by `metrics`.
    pub engines: Vec<(u32, u64, f64)>,
    /// (pid, its busiest engine type's utilisation), by pid.
    gpu: Vec<(u32, f32)>,
    /// Name keys of the programs being summed, one per `Proc`.
    keys: Vec<u64>,
}

/// A process at one sample.
#[derive(Clone, Copy)]
struct Seen {
    pid: u32,
    /// Tells a new process apart from an earlier one with the same pid.
    created: i64,
    /// Kernel + user time, 100 ns units.
    cpu: u64,
    /// Read + write + other transfer counts, bytes.
    io: u64,
}

impl Sampler {
    /// Takes a first snapshot, so the next `sample` already has rates.
    pub fn new() -> Sampler {
        let mut freq = 0i64;
        unsafe { QueryPerformanceFrequency(&mut freq) };
        let mut s = Sampler {
            buf: Vec::new(),
            prev: Vec::new(),
            cur: Vec::new(),
            time: 0,
            qpc_freq: freq as f64,
            cpus: unsafe { GetActiveProcessorCount(ALL_PROCESSOR_GROUPS) }.max(1) as f64,
            engines: Vec::new(),
            gpu: Vec::new(),
            keys: Vec::new(),
        };
        s.sample();
        s
    }

    /// Every program, CPU and I/O since the previous call (0 for a process new since then).
    pub fn sample(&mut self) -> Vec<Proc> {
        let len = snapshot(&mut self.buf);
        let mut now = 0i64;
        unsafe { QueryPerformanceCounter(&mut now) };
        let secs = if self.time != 0 && self.qpc_freq > 0.0 { (now - self.time) as f64 / self.qpc_freq } else { 0.0 };
        self.time = now;
        self.reduce_gpu();
        // 100 ns units the whole machine had since the previous call.
        let capacity = secs * 1e7 * self.cpus;
        let mut out: Vec<Proc> = Vec::new();
        self.keys.clear();
        self.cur.clear();
        for p in Entries::new(&self.buf, len) {
            let pid = p.pid as u32;
            if pid == 0 {
                continue; // The idle process.
            }
            let seen = Seen {
                pid,
                created: p.create_time,
                cpu: (p.kernel_time.max(0) as u64).saturating_add(p.user_time.max(0) as u64),
                io: [p.read_bytes, p.write_bytes, p.other_bytes].iter().fold(0u64, |n, &b| n.saturating_add(b.max(0) as u64)),
            };
            self.cur.push(seen);
            let (mut cpu, mut io) = (0.0f32, 0.0f64);
            if let Ok(i) = self.prev.binary_search_by_key(&pid, |s| s.pid)
                && self.prev[i].created == seen.created
                && secs > 0.0
            {
                cpu = (seen.cpu.saturating_sub(self.prev[i].cpu) as f64 * 100.0 / capacity) as f32;
                io = seen.io.saturating_sub(self.prev[i].io) as f64 / secs;
            }
            let gpu = self.gpu.binary_search_by_key(&pid, |g| g.0).map_or(0.0, |i| self.gpu[i].1);
            let name = image(p);
            // the same name in a user's session and in session 0 is two programs
            let service = p.session == 0;
            let key = crate::metrics::hash(name.iter().map(|&c| fold(c))).wrapping_mul(2) | service as u64;
            let i = match self.keys.iter().position(|&k| k == key) {
                Some(i) => i,
                None => {
                    self.keys.push(key);
                    out.push(Proc { name: String::from_utf16_lossy(name), pid, service, ..Proc::default() });
                    out.len() - 1
                }
            };
            let o = &mut out[i];
            o.cpu = (o.cpu + cpu).min(100.0);
            o.mem += p.private_ws.max(0) as u64;
            o.gpu = (o.gpu + gpu).min(100.0);
            o.io += io;
        }
        self.cur.sort_unstable_by_key(|s| s.pid);
        core::mem::swap(&mut self.prev, &mut self.cur);
        out
    }

    /// `engines` summed per (pid, engine type), each pid's busiest type into `gpu`.
    fn reduce_gpu(&mut self) {
        self.engines.sort_unstable_by_key(|e| (e.0, e.1));
        self.gpu.clear();
        let mut i = 0;
        while i < self.engines.len() {
            let (pid, ty) = (self.engines[i].0, self.engines[i].1);
            let mut sum = 0.0f64;
            while i < self.engines.len() && (self.engines[i].0, self.engines[i].1) == (pid, ty) {
                sum += self.engines[i].2;
                i += 1;
            }
            let sum = if sum.is_finite() { sum.clamp(0.0, 100.0) as f32 } else { 0.0 };
            match self.gpu.last_mut() {
                Some(g) if g.0 == pid => g.1 = g.1.max(sum),
                _ => self.gpu.push((pid, sum)),
            }
        }
        self.engines.clear();
    }
}

/// `SYSTEM_PROCESS_INFORMATION`'s fixed part as ntdll fills it since Vista (`ntexapi.h` in the
/// phnt headers). windows-sys has only the documented subset, which hides the private working
/// set, the times (`Reserved1`) and the I/O counts (`Reserved7`). The process's thread records
/// follow it; the next process starts `next` bytes on.
#[repr(C)]
struct SysProc {
    next: u32,
    _threads: u32,
    /// `WorkingSetPrivateSize`.
    private_ws: i64,
    _hard_faults: u32,
    _threads_peak: u32,
    _cycle_time: u64,
    create_time: i64,
    user_time: i64,
    kernel_time: i64,
    image: UNICODE_STRING,
    _base_priority: i32,
    pid: usize,
    _parent_pid: usize,
    _handles: u32,
    session: u32,
    _key: usize,
    /// Peak and current virtual size.
    _virtual: [usize; 2],
    _page_faults: u32,
    /// Working sets, pool quotas, pagefile and private pages.
    _memory: [usize; 9],
    _read_ops: i64,
    _write_ops: i64,
    _other_ops: i64,
    read_bytes: i64,
    write_bytes: i64,
    other_bytes: i64,
}

#[cfg(target_pointer_width = "64")]
const _: () = {
    assert!(size_of::<SysProc>() == 256);
    assert!(core::mem::offset_of!(SysProc, image) == 56);
    assert!(core::mem::offset_of!(SysProc, pid) == 80);
    assert!(core::mem::offset_of!(SysProc, session) == 100);
    assert!(core::mem::offset_of!(SysProc, read_bytes) == 232);
};

/// Reads every process into `buf` (grown when the list outgrows it); the bytes filled, 0 when
/// it fails.
fn snapshot(buf: &mut Vec<u64>) -> usize {
    // Processes can start between the size query and the read: retry a few times.
    for _ in 0..4 {
        let bytes = (buf.len() * 8) as u32;
        let mut len = 0u32;
        let r = unsafe { NtQuerySystemInformation(SystemProcessInformation, buf.as_mut_ptr().cast(), bytes, &mut len) };
        if r >= 0 {
            return len.min(bytes) as usize;
        }
        if r != STATUS_INFO_LENGTH_MISMATCH {
            return 0;
        }
        // A quarter more than asked for, so a few new processes don't mean a new buffer.
        let want = if len > bytes { len as usize + len as usize / 4 } else { bytes as usize * 2 + 64 * 1024 };
        buf.resize(want.div_ceil(8), 0);
    }
    0
}

/// The processes in a snapshot's first `len` bytes.
struct Entries<'a> {
    buf: &'a [u64],
    len: usize,
    /// The next record's offset, `None` after the last.
    at: Option<usize>,
}

impl<'a> Entries<'a> {
    fn new(buf: &'a [u64], len: usize) -> Entries<'a> {
        Entries { buf, len: len.min(buf.len() * 8), at: Some(0) }
    }
}

impl<'a> Iterator for Entries<'a> {
    type Item = &'a SysProc;
    fn next(&mut self) -> Option<&'a SysProc> {
        let at = self.at?;
        if at % 8 != 0 || at + size_of::<SysProc>() > self.len {
            self.at = None;
            return None;
        }
        // In bounds and 8-aligned (the records are), so a whole record.
        let p = unsafe { &*self.buf.as_ptr().cast::<u8>().add(at).cast::<SysProc>() };
        self.at = if p.next == 0 { None } else { Some(at + p.next as usize) };
        Some(p)
    }
}

/// The image name without ".exe"; empty for the idle process. Points into the snapshot.
fn image(p: &SysProc) -> &[u16] {
    if p.image.Buffer.is_null() {
        return &[];
    }
    // Length is in bytes.
    let s = unsafe { core::slice::from_raw_parts(p.image.Buffer, p.image.Length as usize / 2) };
    match s.len().checked_sub(4) {
        Some(n) if n > 0 && same(&s[n..], &[b'.' as u16, b'e' as u16, b'x' as u16, b'e' as u16]) => &s[..n],
        _ => s,
    }
}

/// Equal, ignoring ASCII case.
fn same(a: &[u16], b: &[u16]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(&x, &y)| fold(x) == fold(y))
}

fn fold(c: u16) -> u16 {
    if (b'A' as u16..=b'Z' as u16).contains(&c) { c + 32 } else { c }
}
