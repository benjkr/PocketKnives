//! `Readout.exe --status [seconds]`: prints one line of live readings a second.
//! `Readout.exe --top [seconds]`: the programs using most CPU, memory, GPU and I/O.
use crate::metrics::{Collector, Sample};
use crate::procs::Proc;
use core::fmt::Write;
use pk::out::Writer;
use windows_sys::Win32::System::Threading::Sleep;

const GB: f64 = (1u64 << 30) as f64;

/// Takes `seconds` readings 1 s apart, one line each. Exit code 0.
pub fn run(out: &mut Writer, seconds: u32) -> u32 {
    let mut c = Collector::new();
    for _ in 0..seconds {
        unsafe { Sleep(1000) };
        line(out, &c.sample());
    }
    0
}

/// Takes `seconds` readings 1 s apart (at least one) and prints the last one's top ten
/// programs by CPU, memory, GPU and I/O, a section each. Exit code 0.
pub fn top(out: &mut Writer, seconds: u32) -> u32 {
    let mut c = Collector::new();
    c.set_procs(true);
    let mut s = Sample::default();
    for _ in 0..seconds.max(1) {
        unsafe { Sleep(1000) };
        s = c.sample();
    }
    let mut p = s.procs;
    section(out, "cpu", &mut p, |a| a.cpu as f64, |out, a| {
        let _ = write!(out, "{:.1}%", a.cpu);
    });
    section(out, "memory", &mut p, |a| a.mem as f64, |out, a| size(out, a.mem as f64, ""));
    section(out, "gpu", &mut p, |a| a.gpu as f64, |out, a| {
        let _ = write!(out, "{:.1}%", a.gpu);
    });
    section(out, "io", &mut p, |a| a.io, |out, a| rate(out, a.io));
    0
}

/// A heading and the ten programs with the most `key`, highest first.
fn section(out: &mut Writer, title: &str, p: &mut [Proc], key: fn(&Proc) -> f64, value: fn(&mut Writer, &Proc)) {
    p.sort_unstable_by(|a, b| key(b).total_cmp(&key(a)).then_with(|| a.name.cmp(&b.name)));
    let _ = write!(out, "{title}\r\n");
    for a in p.iter().take(10) {
        let _ = write!(out, "  {:<32} ", a.name);
        value(out, a);
        out.write_bytes(b"\r\n");
    }
}

fn line(out: &mut Writer, s: &Sample) {
    let _ = write!(out, "cpu {:.1}% ({} cores:", s.cpu, s.cores.len());
    for &c in &s.cores {
        let _ = write!(out, " {}", (c + 0.5) as u32);
    }
    let _ = write!(out, ") ");
    if s.cpu_mhz > 0.0 {
        let _ = write!(out, "{:.2} GHz", s.cpu_mhz / 1000.0);
    } else {
        let _ = write!(out, "-- GHz");
    }
    let _ = write!(out, " | mem {:.1}% {:.1}/{:.1} GB", s.mem, s.mem_used as f64 / GB, s.mem_total as f64 / GB);
    let _ = write!(out, " | gpu {:.1}% vram {:.1}/", s.gpu, s.gpu_mem_used as f64 / GB);
    if s.gpu_mem_total > 0 {
        let _ = write!(out, "{:.1} GB", s.gpu_mem_total as f64 / GB);
    } else {
        let _ = write!(out, "-- GB");
    }
    let _ = write!(out, " | disk {:.1}% r ", s.disk);
    rate(out, s.disk_read);
    let _ = write!(out, " w ");
    rate(out, s.disk_write);
    let _ = write!(out, " | net up ");
    rate(out, s.net_up);
    let _ = write!(out, " down ");
    rate(out, s.net_down);
    out.write_bytes(b"\r\n");
}

/// Bytes a second in 1024-based units: `0 B/s`, `3.1 KB/s`, `40 KB/s`, `1.2 MB/s`.
fn rate(out: &mut Writer, bytes: f64) {
    size(out, bytes, "/s");
}

/// Bytes in 1024-based units, then `suffix`: `0 B`, `3.1 KB`, `40 KB`, `1.2 MB`.
fn size(out: &mut Writer, bytes: f64, suffix: &str) {
    let mut v = bytes.max(0.0);
    let mut unit = 0;
    while v >= 1024.0 && unit < 3 {
        v /= 1024.0;
        unit += 1;
    }
    let name = ["B", "KB", "MB", "GB"][unit];
    let _ = if unit == 0 || v >= 10.0 { write!(out, "{v:.0} {name}{suffix}") } else { write!(out, "{v:.1} {name}{suffix}") };
}
