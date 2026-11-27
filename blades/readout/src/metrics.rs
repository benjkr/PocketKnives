//! Live system load: CPU per logical processor and clock, memory, GPU and VRAM, disk and
//! network, from Windows' own counters, all readable without admin rights.
//!
//! - CPU: PDH `\Processor Information(*)\% Processor Utility` (what Task Manager shows since
//!   Windows 8, frequency-scaled and capped at 100 here), one instance per logical processor in
//!   every processor group. Clock: `% Processor Performance` × the processor's base MHz
//!   (`CallNtPowerInformation`), as Task Manager does.
//! - Memory: `GlobalMemoryStatusEx`.
//! - GPU: PDH `\GPU Engine(*)\Utilization Percentage`, summed per adapter and engine type; the
//!   busiest wins. VRAM used: `\GPU Adapter Memory(*)\Dedicated Usage` of the adapter using
//!   most; total: the largest `HardwareInformation.qwMemorySize` of the display adapters.
//! - Disk: PDH `\PhysicalDisk(_Total)`. Network: octet deltas (`GetIfEntry2`) of the connected
//!   hardware interfaces, listed every few seconds (`GetIfTable2Ex` without statistics).
//!
//! Every source is optional: one that fails reads as 0 and the others carry on. Nothing here
//! prints. Buffers are kept between calls and only grow to the largest size seen.
use alloc::vec::Vec;
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};
use windows_sys::Win32::Foundation::{ERROR_SUCCESS, FILETIME};
use windows_sys::Win32::NetworkManagement::IpHelper::{
    FreeMibTable, GetIfEntry2, GetIfTable2Ex, IF_TYPE_SOFTWARE_LOOPBACK, IF_TYPE_TUNNEL, MIB_IF_ROW2, MIB_IF_TABLE2,
    MibIfTableNormalWithoutStatistics,
};
use windows_sys::Win32::NetworkManagement::Ndis::IfOperStatusUp;
use windows_sys::Win32::System::Performance::{
    PDH_CSTATUS_NEW_DATA, PDH_FMT_COUNTERVALUE, PDH_FMT_COUNTERVALUE_ITEM_W, PDH_FMT_DOUBLE, PDH_HCOUNTER, PDH_HQUERY,
    PDH_MORE_DATA, PdhAddEnglishCounterW, PdhCloseQuery, PdhCollectQueryData, PdhGetFormattedCounterArrayW,
    PdhGetFormattedCounterValue, PdhOpenQueryW, QueryPerformanceCounter, QueryPerformanceFrequency,
};
use windows_sys::Win32::System::Power::{CallNtPowerInformation, PROCESSOR_POWER_INFORMATION, ProcessorInformation};
use windows_sys::Win32::System::Registry::{
    HKEY, HKEY_LOCAL_MACHINE, KEY_READ, REG_BINARY, REG_DWORD, REG_QWORD, RRF_RT_ANY, RegCloseKey, RegEnumKeyExW,
    RegGetValueW, RegOpenKeyExW,
};
use windows_sys::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
use windows_sys::Win32::System::Threading::{ALL_PROCESSOR_GROUPS, GetActiveProcessorCount};
use windows_sys::core::PCWSTR;

/// One reading. Loads are percentages 0..=100; a field whose source failed is 0.
#[derive(Clone, Default)]
pub struct Sample {
    pub cores: Vec<f32>,
    pub cpu: f32,
    pub cpu_mhz: f32,
    pub mem: f32,
    pub mem_used: u64,
    pub mem_total: u64,
    pub gpu: f32,
    pub gpu_mem_used: u64,
    pub gpu_mem_total: u64,
    pub disk: f32,
    pub disk_read: f64,
    pub disk_write: f64,
    pub net_up: f64,
    pub net_down: f64,
}

/// `PDH_FMT_NOCAP100` (pdh.h), not in windows-sys: percentages above 100 (Processor Utility
/// with turbo) come back as they are and are clamped here.
const PDH_FMT_NOCAP100: u32 = 0x8000;
const FMT: u32 = PDH_FMT_DOUBLE | PDH_FMT_NOCAP100;

/// Calls between re-listing the network interfaces: a new connection is counted within this
/// many seconds. One that drops is noticed at once.
const NET_RELIST: u32 = 10;

/// Display adapters' device class (`GUID_DEVCLASS_DISPLAY`).
const DISPLAY_CLASS: PCWSTR = pk::w!(r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}");

pub struct Collector {
    /// CPU, clock and disk.
    query: PDH_HQUERY,
    /// GPU engines and adapter memory: its own query, the costly one (an instance per process
    /// per engine), so a failure there leaves the others alone.
    gpu_query: PDH_HQUERY,
    cpu: PDH_HCOUNTER,
    perf: PDH_HCOUNTER,
    disk_idle: PDH_HCOUNTER,
    disk_read: PDH_HCOUNTER,
    disk_write: PDH_HCOUNTER,
    gpu_engine: PDH_HCOUNTER,
    gpu_mem: PDH_HCOUNTER,
    base_mhz: f32,
    gpu_mem_total: u64,
    /// `PdhGetFormattedCounterArrayW` output (u64 for the items' alignment).
    array: Vec<u64>,
    /// (processor sort key, load), for ordering the cores.
    cores: Vec<(u32, f32)>,
    /// (adapter hash, engine type hash, summed utilisation).
    engines: Vec<(u64, u64, f64)>,
    /// LUIDs of the interfaces counted, re-listed when `net_relist` (calls) reaches 0.
    net_ifs: Vec<u64>,
    net_relist: u32,
    /// (interface LUID, in octets, out octets) at the previous call, and this call's.
    net_prev: Vec<(u64, u64, u64)>,
    net_cur: Vec<(u64, u64, u64)>,
    net_time: i64,
    qpc_freq: i64,
}

impl Collector {
    pub fn new() -> Collector {
        let query = open_query();
        let gpu_query = open_query();
        // Processor Utility is missing before Windows 8; Processor Time is the next best.
        let mut cpu = add(query, pk::w!(r"\Processor Information(*)\% Processor Utility"));
        if cpu.is_null() {
            cpu = add(query, pk::w!(r"\Processor Information(*)\% Processor Time"));
        }
        let mut c = Collector {
            query,
            gpu_query,
            cpu,
            perf: add(query, pk::w!(r"\Processor Information(_Total)\% Processor Performance")),
            disk_idle: add(query, pk::w!(r"\PhysicalDisk(_Total)\% Idle Time")),
            disk_read: add(query, pk::w!(r"\PhysicalDisk(_Total)\Disk Read Bytes/sec")),
            disk_write: add(query, pk::w!(r"\PhysicalDisk(_Total)\Disk Write Bytes/sec")),
            gpu_engine: add(gpu_query, pk::w!(r"\GPU Engine(*)\Utilization Percentage")),
            gpu_mem: add(gpu_query, pk::w!(r"\GPU Adapter Memory(*)\Dedicated Usage")),
            base_mhz: base_mhz(),
            gpu_mem_total: vram_total(),
            array: Vec::new(),
            cores: Vec::new(),
            engines: Vec::new(),
            net_ifs: Vec::new(),
            net_relist: 0,
            net_prev: Vec::new(),
            net_cur: Vec::new(),
            net_time: 0,
            qpc_freq: 0,
        };
        unsafe { QueryPerformanceFrequency(&mut c.qpc_freq) };
        // Rates need two readings: take the first now, so the first `sample` already has them.
        c.collect();
        c.net(&mut Sample::default());
        c
    }

    /// Called about once a second. Rates and loads are measured since the previous call (or
    /// since `new` for the first).
    pub fn sample(&mut self) -> Sample {
        let mut s = Sample::default();
        self.collect();
        self.cpu(&mut s);
        self.gpu(&mut s);
        if let Some(idle) = value(self.disk_idle) {
            s.disk = clamp_pct(100.0 - idle as f32);
        }
        s.disk_read = value(self.disk_read).unwrap_or(0.0).max(0.0);
        s.disk_write = value(self.disk_write).unwrap_or(0.0).max(0.0);
        memory(&mut s);
        self.net(&mut s);
        s
    }

    fn collect(&mut self) {
        for q in [self.query, self.gpu_query] {
            if !q.is_null() {
                unsafe { PdhCollectQueryData(q) };
            }
        }
    }

    fn cpu(&mut self, s: &mut Sample) {
        self.cores.clear();
        let mut total = None;
        for item in read_array(self.cpu, &mut self.array) {
            let Some(v) = item_value(item) else { continue };
            let name = unsafe { wstr(item.szName) };
            match processor_key(name) {
                Some(key) => self.cores.push((key, clamp_pct(v as f32))),
                None if eq_ascii(name, b"_Total") => total = Some(clamp_pct(v as f32)),
                None => {} // "0,_Total": one group's total.
            }
        }
        self.cores.sort_unstable_by_key(|c| c.0);
        s.cores = self.cores.iter().map(|c| c.1).collect();
        s.cpu = total.unwrap_or_else(|| {
            if s.cores.is_empty() { 0.0 } else { s.cores.iter().sum::<f32>() / s.cores.len() as f32 }
        });
        if let Some(perf) = value(self.perf) {
            s.cpu_mhz = self.base_mhz * perf.max(0.0) as f32 / 100.0;
        }
    }

    fn gpu(&mut self, s: &mut Sample) {
        // "pid_1234_luid_0x00000000_0x0000D1E6_phys_0_eng_3_engtype_VideoDecode"
        self.engines.clear();
        for item in read_array(self.gpu_engine, &mut self.array) {
            let Some(v) = item_value(item) else { continue };
            let name = unsafe { wstr(item.szName) };
            let (Some(luid), Some(eng), Some(ty)) = (find(name, b"luid_"), find(name, b"_eng_"), find(name, b"engtype_")) else {
                continue;
            };
            if eng < luid {
                continue;
            }
            // Adapter: "luid_…_phys_0"; type: what follows "engtype_".
            let key = (hash(&name[luid..eng]), hash(&name[ty + b"engtype_".len()..]));
            match self.engines.iter_mut().find(|e| (e.0, e.1) == key) {
                Some(e) => e.2 += v,
                None => self.engines.push((key.0, key.1, v)),
            }
        }
        let busiest = self.engines.iter().fold(0.0f64, |m, e| m.max(e.2));
        s.gpu = clamp_pct(busiest as f32);
        let mut used = 0.0f64;
        for item in read_array(self.gpu_mem, &mut self.array) {
            if let Some(v) = item_value(item) {
                used = used.max(v);
            }
        }
        s.gpu_mem_used = used as u64;
        s.gpu_mem_total = self.gpu_mem_total;
    }

    fn net(&mut self, s: &mut Sample) {
        if self.net_relist == 0 {
            self.list_interfaces();
            self.net_relist = NET_RELIST;
        }
        self.net_relist -= 1;
        let mut now = 0i64;
        unsafe { QueryPerformanceCounter(&mut now) };
        self.net_cur.clear();
        for &luid in &self.net_ifs {
            let mut row: MIB_IF_ROW2 = unsafe { zeroed() };
            row.InterfaceLuid.Value = luid;
            if unsafe { GetIfEntry2(&mut row) } == ERROR_SUCCESS && counted(&row) {
                self.net_cur.push((luid, row.InOctets, row.OutOctets));
            } else {
                self.net_relist = 0; // Gone or disconnected: list them again next time.
            }
        }
        let secs = if self.net_time != 0 && self.qpc_freq > 0 { (now - self.net_time) as f64 / self.qpc_freq as f64 } else { 0.0 };
        if secs > 0.0 {
            let (mut down, mut up) = (0u64, 0u64);
            // An interface that just appeared has no baseline and adds nothing this time.
            for &(luid, rx, tx) in &self.net_cur {
                if let Some(&(_, prx, ptx)) = self.net_prev.iter().find(|p| p.0 == luid) {
                    down += rx.saturating_sub(prx);
                    up += tx.saturating_sub(ptx);
                }
            }
            s.net_down = down as f64 / secs;
            s.net_up = up as f64 / secs;
        }
        core::mem::swap(&mut self.net_prev, &mut self.net_cur);
        self.net_time = now;
    }

    /// The interfaces to count, listed without their statistics: a full `GetIfTable2` reads
    /// the statistics of every filter and virtual interface too, which costs far more than
    /// `GetIfEntry2` on the few that count.
    fn list_interfaces(&mut self) {
        self.net_ifs.clear();
        let mut table: *mut MIB_IF_TABLE2 = null_mut();
        if unsafe { GetIfTable2Ex(MibIfTableNormalWithoutStatistics, &mut table) } == ERROR_SUCCESS && !table.is_null() {
            let rows = unsafe {
                let n = (*table).NumEntries as usize;
                core::slice::from_raw_parts((&raw const (*table).Table).cast::<MIB_IF_ROW2>(), n)
            };
            self.net_ifs.extend(rows.iter().filter(|r| counted(r)).map(|r| unsafe { r.InterfaceLuid.Value }));
        }
        if !table.is_null() {
            unsafe { FreeMibTable(table.cast()) };
        }
    }
}

impl Drop for Collector {
    fn drop(&mut self) {
        for q in [self.query, self.gpu_query] {
            if !q.is_null() {
                unsafe { PdhCloseQuery(q) };
            }
        }
    }
}

/// A connected hardware interface: not loopback, a tunnel, a virtual adapter or a filter
/// driver's view of another interface (which would count the same bytes twice).
fn counted(r: &MIB_IF_ROW2) -> bool {
    let flags = r.InterfaceAndOperStatusFlags._bitfield;
    let hardware = flags & 1 != 0;
    let filter = flags & 2 != 0;
    hardware && !filter && r.OperStatus == IfOperStatusUp && r.Type != IF_TYPE_SOFTWARE_LOOPBACK && r.Type != IF_TYPE_TUNNEL
}

fn memory(s: &mut Sample) {
    let mut m: MEMORYSTATUSEX = unsafe { zeroed() };
    m.dwLength = size_of::<MEMORYSTATUSEX>() as u32;
    if unsafe { GlobalMemoryStatusEx(&mut m) } != 0 && m.ullTotalPhys > 0 {
        s.mem_total = m.ullTotalPhys;
        s.mem_used = m.ullTotalPhys.saturating_sub(m.ullAvailPhys);
        s.mem = clamp_pct((s.mem_used as f64 * 100.0 / s.mem_total as f64) as f32);
    }
}

fn open_query() -> PDH_HQUERY {
    let mut q = null_mut();
    if unsafe { PdhOpenQueryW(null(), 0, &mut q) } == ERROR_SUCCESS { q } else { null_mut() }
}

/// A counter on `query`, or null when the query or the counter isn't there.
fn add(query: PDH_HQUERY, path: PCWSTR) -> PDH_HCOUNTER {
    let mut c = null_mut();
    if query.is_null() || unsafe { PdhAddEnglishCounterW(query, path, 0, &mut c) } != ERROR_SUCCESS {
        return null_mut();
    }
    c
}

/// A single-instance counter's value at the last collection, if it has one.
fn value(c: PDH_HCOUNTER) -> Option<f64> {
    if c.is_null() {
        return None;
    }
    let mut v: PDH_FMT_COUNTERVALUE = unsafe { zeroed() };
    if unsafe { PdhGetFormattedCounterValue(c, FMT, null_mut(), &mut v) } != ERROR_SUCCESS || v.CStatus > PDH_CSTATUS_NEW_DATA {
        return None;
    }
    let d = unsafe { v.Anonymous.doubleValue };
    if d.is_finite() { Some(d) } else { None }
}

fn item_value(item: &PDH_FMT_COUNTERVALUE_ITEM_W) -> Option<f64> {
    if item.FmtValue.CStatus > PDH_CSTATUS_NEW_DATA || item.szName.is_null() {
        return None;
    }
    let d = unsafe { item.FmtValue.Anonymous.doubleValue };
    if d.is_finite() { Some(d) } else { None }
}

/// A wildcard counter's instances at the last collection, read into `buf` (grown when the
/// instance list outgrows it; the names point into it). Empty when anything fails.
fn read_array(c: PDH_HCOUNTER, buf: &mut Vec<u64>) -> &[PDH_FMT_COUNTERVALUE_ITEM_W] {
    if c.is_null() {
        return &[];
    }
    // The instance list can grow between the size query and the read: retry a few times.
    for _ in 0..4 {
        let mut size = (buf.len() * 8) as u32;
        let mut count = 0u32;
        let ptr = if buf.is_empty() { null_mut() } else { buf.as_mut_ptr().cast() };
        let r = unsafe { PdhGetFormattedCounterArrayW(c, FMT, &mut size, &mut count, ptr) };
        if r == ERROR_SUCCESS && !buf.is_empty() {
            return unsafe { core::slice::from_raw_parts(buf.as_ptr().cast(), count as usize) };
        }
        if r != PDH_MORE_DATA {
            return &[];
        }
        // A quarter more than asked for, so a few new processes don't mean a new buffer.
        let words = (size as usize + size as usize / 4).div_ceil(8);
        buf.resize(words.max(buf.len() + 1), 0);
    }
    &[]
}

/// A NUL-terminated UTF-16 string.
///
/// # Safety
/// `p` is null or points at a NUL-terminated string that outlives the slice.
unsafe fn wstr<'a>(p: *const u16) -> &'a [u16] {
    if p.is_null() {
        return &[];
    }
    let mut n = 0;
    while unsafe { *p.add(n) } != 0 {
        n += 1;
    }
    unsafe { core::slice::from_raw_parts(p, n) }
}

/// `"group,number"` (Processor Information) or `"number"` (Processor) as a sort key; `None`
/// for the totals.
fn processor_key(name: &[u16]) -> Option<u32> {
    let (group, number) = match name.iter().position(|&c| c == b',' as u16) {
        Some(i) => (number(&name[..i])?, number(&name[i + 1..])?),
        None => (0, number(name)?),
    };
    Some(group << 16 | number)
}

fn number(s: &[u16]) -> Option<u32> {
    if s.is_empty() || s.len() > 5 {
        return None;
    }
    s.iter().try_fold(0u32, |n, &c| if (b'0' as u16..=b'9' as u16).contains(&c) { Some(n * 10 + (c - b'0' as u16) as u32) } else { None })
}

/// Where `needle` (ASCII) starts in `hay`.
fn find(hay: &[u16], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| eq_ascii(w, needle))
}

fn eq_ascii(s: &[u16], ascii: &[u8]) -> bool {
    s.len() == ascii.len() && s.iter().zip(ascii).all(|(&a, &b)| a == b as u16)
}

/// FNV-1a: groups GPU engines by adapter and type without keeping their names.
fn hash(s: &[u16]) -> u64 {
    s.iter().fold(0xcbf29ce484222325u64, |h, &c| (h ^ c as u64).wrapping_mul(0x100000001b3))
}

fn clamp_pct(v: f32) -> f32 {
    if v.is_nan() { 0.0 } else { v.clamp(0.0, 100.0) }
}

/// The base clock in MHz: the first processor's `MaxMhz`, which Task Manager shows as its
/// "Base speed". 0 when unknown.
fn base_mhz() -> f32 {
    let n = unsafe { GetActiveProcessorCount(ALL_PROCESSOR_GROUPS) }.max(1) as usize;
    let mut info: Vec<PROCESSOR_POWER_INFORMATION> = Vec::with_capacity(n);
    let bytes = (n * size_of::<PROCESSOR_POWER_INFORMATION>()) as u32;
    let status = unsafe { CallNtPowerInformation(ProcessorInformation, null(), 0, info.as_mut_ptr().cast(), bytes) };
    if status != 0 {
        return 0.0;
    }
    unsafe { info.set_len(n) };
    info[0].MaxMhz as f32
}

/// The largest dedicated video memory of the display adapters, from the driver's registry key
/// (`HardwareInformation.qwMemorySize`, or the 32-bit `MemorySize` older drivers write). 0 when
/// none says.
fn vram_total() -> u64 {
    let mut class: HKEY = null_mut();
    if unsafe { RegOpenKeyExW(HKEY_LOCAL_MACHINE, DISPLAY_CLASS, 0, KEY_READ, &mut class) } != ERROR_SUCCESS {
        return 0;
    }
    let mut best = 0u64;
    let mut name = [0u16; 64];
    for i in 0.. {
        let mut len = name.len() as u32;
        let r = unsafe {
            RegEnumKeyExW(class, i, name.as_mut_ptr(), &mut len, null(), null_mut(), null_mut(), null_mut::<FILETIME>())
        };
        if r != ERROR_SUCCESS {
            break;
        }
        let sub = name.as_ptr();
        let size = reg_u64(class, sub, pk::w!("HardwareInformation.qwMemorySize"))
            .or_else(|| reg_u64(class, sub, pk::w!("HardwareInformation.MemorySize")))
            .unwrap_or(0);
        best = best.max(size);
    }
    unsafe { RegCloseKey(class) };
    best
}

/// A 4- or 8-byte integer value (DWORD, QWORD or the binary form drivers also use).
fn reg_u64(key: HKEY, sub: PCWSTR, value: PCWSTR) -> Option<u64> {
    let mut data = 0u64;
    let mut size = size_of::<u64>() as u32;
    let mut ty = 0u32;
    let r = unsafe { RegGetValueW(key, sub, value, RRF_RT_ANY, &mut ty, (&raw mut data).cast(), &mut size) };
    match (r, ty, size) {
        (ERROR_SUCCESS, REG_QWORD | REG_BINARY, 8) => Some(data),
        (ERROR_SUCCESS, REG_DWORD | REG_BINARY, 4) => Some(data & 0xffff_ffff),
        _ => None,
    }
}
