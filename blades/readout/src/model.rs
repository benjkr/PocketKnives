//! What the bar shows: the latest values and a short history of each, filled from live samples
//! or from the deterministic mock the previews use.
use crate::metrics::Sample;
use crate::procs::Proc;
use alloc::vec;
use alloc::vec::Vec;

/// Samples kept per series: the widest graph (the panel's 60) plus a little.
pub const HISTORY: usize = 64;
/// A meter above this % is hot.
pub const HOT: f32 = 70.0;
/// Network traffic at or above this many KB/s, either way, is a burst (hot).
pub const BURST_KBS: f32 = 1024.0;

/// A fixed-length series, oldest first.
#[derive(Clone)]
pub struct Series(pub Vec<f32>);

impl Series {
    fn new() -> Series {
        Series(Vec::with_capacity(HISTORY))
    }
    fn push(&mut self, v: f32) {
        if self.0.len() == HISTORY {
            self.0.remove(0);
        }
        self.0.push(v);
    }
    /// The last `n` values (fewer while the history fills up).
    pub fn last(&self, n: usize) -> &[f32] {
        &self.0[self.0.len().saturating_sub(n)..]
    }
    pub fn now(&self) -> f32 {
        self.0.last().copied().unwrap_or(0.0)
    }
}

pub struct Model {
    /// Per core, busy %.
    pub cores: Vec<Series>,
    /// CPU total, memory, GPU and disk busy %; network up and down in KB/s.
    pub cpu: Series,
    pub mem: Series,
    pub gpu: Series,
    pub dsk: Series,
    pub up: Series,
    pub dn: Series,
    /// The latest sample's details, for the panels.
    pub last: Sample,
}

impl Model {
    pub fn new() -> Model {
        Model {
            cores: Vec::new(),
            cpu: Series::new(),
            mem: Series::new(),
            gpu: Series::new(),
            dsk: Series::new(),
            up: Series::new(),
            dn: Series::new(),
            last: Sample::default(),
        }
    }

    pub fn push(&mut self, s: Sample) {
        if self.cores.len() != s.cores.len() {
            self.cores = vec![Series::new(); s.cores.len()];
        }
        for (c, &v) in self.cores.iter_mut().zip(&s.cores) {
            c.push(v);
        }
        self.cpu.push(s.cpu);
        self.mem.push(s.mem);
        self.gpu.push(s.gpu);
        self.dsk.push(s.disk);
        self.up.push((s.net_up / 1024.0) as f32);
        self.dn.push((s.net_down / 1024.0) as f32);
        self.last = s;
    }

    /// The mock's frame, for previews: the same generator and seed as the approved design mock,
    /// warmed up to a frame with a download running, two hot cores and a hot GPU.
    pub fn mock() -> Model {
        let mut m = Model::new();
        let mut g = Mock::new();
        for _ in 0..174 {
            m.push(g.tick());
        }
        let cpu = m.cpu.now();
        m.last.procs = MOCK_PROCS
            .iter()
            .map(|&(name, mb, share, gpu, kbs)| Proc {
                name: name.into(),
                pid: 0,
                cpu: share * cpu,
                mem: (mb as f64 * 1024.0 * 1024.0) as u64,
                gpu,
                io: kbs as f64 * 1024.0,
            })
            .collect();
        m
    }
}

/// The design mock's sample programs: name, private MB, share of the CPU in use, GPU %, I/O KB/s.
const MOCK_PROCS: [(&str, f32, f32, f32, f32); 14] = [
    ("chrome", 3420.0, 0.26, 14.0, 820.0),
    ("Code", 2180.0, 0.15, 1.2, 140.0),
    ("steamwebhelper", 1640.0, 0.17, 3.1, 64.0),
    ("Discord", 1260.0, 0.06, 2.4, 35.0),
    ("MsMpEng", 842.0, 0.12, 0.0, 2600.0),
    ("ms-teams", 815.0, 0.01, 1.8, 22.0),
    ("explorer", 412.0, 0.03, 0.6, 12.0),
    ("Spotify", 384.0, 0.02, 0.9, 180.0),
    ("dwm", 296.0, 0.07, 9.5, 0.0),
    ("SearchIndexer", 188.0, 0.012, 0.0, 960.0),
    ("OneDrive", 164.0, 0.01, 0.0, 310.0),
    ("svchost", 142.0, 0.015, 0.2, 75.0),
    ("System", 0.2, 0.04, 0.0, 1400.0),
    ("RuntimeBroker", 96.0, 0.006, 0.0, 3.0),
];

/// The design mock's random walk (`tick()` in bar-mocks-16.html), 8 cores.
struct Mock {
    seed: u32,
    load: f32,
    mem: f32,
    gpu: f32,
    dsk: f32,
    burst: i32,
    wt: [f32; 24],
}

const BASE: [f32; 8] = [1.0, 1.45, 0.8, 1.25, 0.65, 1.1, 0.9, 0.85];

impl Mock {
    fn new() -> Mock {
        let mut wt = [0.0; 24];
        for (i, w) in wt.iter_mut().enumerate() {
            *w = BASE[i % 8];
        }
        Mock { seed: 908, load: 30.0, mem: 58.0, gpu: 22.0, dsk: 4.0, burst: 0, wt }
    }
    fn rnd(&mut self) -> f32 {
        self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
        (self.seed as f64 / 4294967296.0) as f32
    }
    fn tick(&mut self) -> Sample {
        let r = self.rnd() - 0.5;
        let spike = if self.rnd() < 0.1 { 30.0 } else { 0.0 };
        self.load = (self.load + r * 24.0 + spike - (self.load - 32.0) * 0.1).clamp(3.0, 98.0);
        let r = self.rnd() - 0.5;
        self.mem = (self.mem + r * 2.4).clamp(48.0, 74.0);
        let r = self.rnd() - 0.5;
        let spike = if self.rnd() < 0.07 { 32.0 } else { 0.0 };
        self.gpu = (self.gpu + r * 18.0 + spike - (self.gpu - 28.0) * 0.08).clamp(2.0, 96.0);
        self.dsk = if self.rnd() < 0.28 { 25.0 + self.rnd() * 70.0 } else { (self.dsk * 0.45 + self.rnd() * 6.0).clamp(1.0, 100.0) };
        if self.burst <= 0 && self.rnd() < 0.12 {
            self.burst = 3 + (self.rnd() * 6.0) as i32;
        }
        let (up, dn);
        if self.burst > 0 {
            self.burst -= 1;
            dn = 300.0 + self.rnd() * 3400.0;
            up = 30.0 + self.rnd() * 600.0;
        } else {
            dn = 1.0 + self.rnd() * 40.0;
            up = 1.0 + self.rnd() * 18.0;
        }
        for c in 0..24 {
            let r = self.rnd() - 0.5;
            self.wt[c] = (self.wt[c] + r * 0.4 - (self.wt[c] - BASE[c % 8]) * 0.25).clamp(0.1, 2.4);
        }
        let mut cores = vec![0.0f32; 8];
        let s: f32 = self.wt[..8].iter().map(|w| w / 8.0).sum();
        for (c, v) in cores.iter_mut().enumerate() {
            *v = (self.load * self.wt[c] / s).clamp(1.0, 100.0);
        }
        let cpu = cores.iter().sum::<f32>() / 8.0;
        Sample {
            cpu,
            cpu_mhz: 1200.0 + cpu * 36.0,
            mem: self.mem,
            mem_used: (self.mem as f64 / 100.0 * 32.0 * GB) as u64,
            mem_total: (32.0 * GB) as u64,
            gpu: self.gpu,
            gpu_mem_used: ((1.2 + self.gpu as f64 * 0.05) * GB) as u64,
            gpu_mem_total: (8.0 * GB) as u64,
            disk: self.dsk,
            disk_read: self.dsk as f64 * 900.0 * 1024.0,
            disk_write: self.dsk as f64 * 240.0 * 1024.0,
            net_up: up as f64 * 1024.0,
            net_down: dn as f64 * 1024.0,
            cores,
            procs: Vec::new(),
        }
    }
}

const GB: f64 = 1024.0 * 1024.0 * 1024.0;
