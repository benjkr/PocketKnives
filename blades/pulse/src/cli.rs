//! Command-line options for `Pulse.exe`.
use crate::bar::{self, Item, Layout};
use crate::model::Model;
use crate::raster::{self, Canvas};
use crate::settings::{self, Settings};
use crate::theme;
use alloc::format;
use alloc::string::String;
use core::fmt::Write;
use pk::out::{StdoutKind, Writer, attach_parent_console, console_output, stdout_handle, stdout_kind};

macro_rules! wl {
    ($out:expr, $($t:tt)*) => {{ let _ = write!($out, $($t)*); $out.write_bytes(b"\r\n"); }};
}

const HELP: &str = "Pulse \u{2014} a calculator-style system monitor strip in the taskbar.\r\n\
\r\n\
  Pulse.exe                          start the bar (single instance)\r\n\
  Pulse.exe --exit                   close the running bar\r\n\
  Pulse.exe --autostart on|off|status\r\n\
  Pulse.exe --status [seconds]       print live readings, one line a second (default 3)\r\n\
  Pulse.exe --settings               print settings for PocketKnives Settings\r\n\
  Pulse.exe --top [seconds]          print the top 10 programs by CPU, memory, GPU and I/O (default 2)\r\n\
  Pulse.exe --set key=value ...      change settings\r\n\
  Pulse.exe --preview-render PATH [--theme NAME|FILE.json] [--scale 1|2|3|4] [--panel cpu|mem|gpu|dsk|net]\r\n                       \
[--live SECONDS] [--select ROW] [--deny ROW] [--press ROW]\r\n                                     \
draw the bar with the design's sample values to a PNG\r\n\
  Pulse.exe --version                print the version";

pub fn run(args: &[String]) -> u32 {
    let mut out = open_console();
    match args[0].to_ascii_lowercase().as_str() {
        "--version" => {
            wl!(out, "Pulse {}", env!("CARGO_PKG_VERSION"));
            0
        }
        "--help" | "-h" | "/?" => {
            wl!(out, "{HELP}");
            0
        }
        "--settings" => {
            out.write_bytes(Settings::load().format_blade_settings().as_bytes());
            0
        }
        "--set" => match Settings::load().apply_pairs(&args[1..]).and_then(|s| s.save()) {
            Ok(()) => {
                crate::window::notify_settings_changed();
                0
            }
            Err(e) => {
                wl!(out, "{e}");
                2
            }
        },
        "--status" => crate::status::run(&mut out, args.get(1).and_then(|s| s.parse().ok()).unwrap_or(3)),
        "--top" => crate::status::top(&mut out, args.get(1).and_then(|s| s.parse().ok()).unwrap_or(2)),
        "--exit" => {
            crate::window::request_exit();
            0
        }
        "--autostart" => match args.get(1).map(|a| a.to_ascii_lowercase()).as_deref().unwrap_or("status") {
            "status" => {
                wl!(out, "Autostart: {}", if settings::autostart_enabled() { "on" } else { "off" });
                0
            }
            v @ ("on" | "off") => match settings::set_autostart(v == "on") {
                Ok(()) => {
                    wl!(out, "Autostart: {v}");
                    0
                }
                Err(e) => {
                    wl!(out, "Error: {e}");
                    2
                }
            },
            _ => {
                wl!(out, "Usage: Pulse.exe --autostart on|off|status");
                2
            }
        },
        "--preview-render" => preview(&mut out, &args[1..]),
        other => {
            wl!(out, "Unknown option {other}. Try --help.");
            2
        }
    }
}

fn open_console() -> Writer {
    if stdout_kind() != StdoutKind::None {
        return Writer::new(stdout_handle());
    }
    if attach_parent_console()
        && let Some(w) = console_output()
    {
        return w;
    }
    Writer::null()
}

fn preview(out: &mut Writer, args: &[String]) -> u32 {
    let Some(path) = args.first() else {
        wl!(out, "Usage: Pulse.exe --preview-render <path.png> [options]");
        return 2;
    };
    let (mut theme_arg, mut k, mut panel, mut live) = (String::from("ivory"), 1i32, None, 0u32);
    // The list's states, by row from 1: selected, refused, a key held down.
    let (mut select, mut deny, mut press) = (0usize, 0usize, 0usize);
    let mut i = 1;
    while i < args.len() {
        let v = args.get(i + 1).map(String::as_str).unwrap_or("");
        match args[i].as_str() {
            "--theme" => theme_arg = String::from(v),
            "--scale" => k = v.parse().unwrap_or(0),
            "--live" => live = v.parse().unwrap_or(0).min(64),
            "--select" => select = v.parse().unwrap_or(0),
            "--deny" => deny = v.parse().unwrap_or(0),
            "--press" => press = v.parse().unwrap_or(0),
            "--panel" => {
                panel = match v {
                    "cpu" => Some(Item::Cpu),
                    "mem" => Some(Item::Mem),
                    "gpu" => Some(Item::Gpu),
                    "dsk" => Some(Item::Dsk),
                    "net" => Some(Item::Net),
                    _ => {
                        wl!(out, "--panel is cpu, mem, gpu, dsk or net");
                        return 2;
                    }
                }
            }
            o => {
                wl!(out, "Unknown option {o}");
                return 2;
            }
        }
        i += 2;
    }
    if !(1..=4).contains(&k) || [select, deny, press].iter().any(|&r| r > bar::LIST_ROWS) {
        wl!(out, "--scale is 1 to 4, and --select, --deny and --press a row from 1 to {}", bar::LIST_ROWS);
        return 2;
    }
    let pal = if theme_arg.ends_with(".json") {
        match settings::read_file(&theme_arg) {
            Some(b) => theme::parse(core::str::from_utf8(&b).unwrap_or("")).map_err(|e| format!("{theme_arg}: {e}")),
            None => Err(format!("Can't read {theme_arg}")),
        }
    } else {
        theme::load(&theme_arg)
    };
    let pal = match pal {
        Ok(p) => p,
        Err(e) => {
            wl!(out, "{e}");
            return 2;
        }
    };
    let m = if live > 0 {
        // this machine's readings, a second apart
        let mut m = Model::new();
        let mut c = crate::metrics::Collector::new();
        c.set_procs(true);
        for _ in 0..live {
            unsafe { windows_sys::Win32::System::Threading::Sleep(1000) };
            m.push(c.sample());
        }
        m
    } else {
        Model::mock()
    };
    let items = Settings::default().items();
    let l = Layout::new(k, &items);
    let (bw, bh) = l.size();
    let mut c = Canvas::new(bw, bh);
    let hover = panel.and_then(|p| l.tiles.iter().position(|t| t.item == p));
    bar::bar(&mut c, &l, &pal, &m, hover);
    // A panel is drawn above the bar where the window would put it.
    let img = if let (Some(item), Some(i)) = (panel, hover) {
        let (pk, halve) = l.panel_draw();
        let (pw, ph) = bar::panel_size(pk, item);
        let mut pc = Canvas::new(pw, ph);
        let row = |r: usize| r.checked_sub(1);
        let rows = bar::ranking(&m, item);
        // the icons of this machine's programs with those names (the sample's have no processes)
        let icons = if live > 0 { crate::icons::for_rows(&rows, &m.last.procs) } else { crate::icons::for_rows(&rows, &crate::procs::Sampler::new().sample()) };
        let view = bar::ListView { rows, sel: row(select), denied: row(deny), down: row(press), icons };
        bar::panel(&mut pc, pk, &pal, &m, item, &view);
        bar::panel_glare(&mut pc, pk, &pal, item, (0.0, 0.0));
        if halve {
            let mut half = Canvas::new(pw.div_ceil(2), ph.div_ceil(2));
            pc.halve_into(&mut half);
            pc = half;
        }
        let margin = (bar::PANEL_MARGIN as f32 * l.panel_scale()) as i32;
        let px = (l.panel_x(i) * k as f32) as i32 - margin;
        let body = (bar::panel_h(item) as f32 * l.panel_scale()) as i32;
        let py = -bar::PANEL_GAP * k - body - margin;
        let (ox, oy) = ((-px).max(0), -py);
        let w = (px + ox + pc.w as i32).max(bw as i32 + ox) as usize;
        let h = (py + oy + pc.h as i32).max(bh as i32 + oy) as usize;
        let mut all = Canvas::new(w, h);
        compose(&mut all, &c, ox, oy);
        compose(&mut all, &pc, px + ox, py + oy);
        all
    } else {
        c
    };
    let png = raster::png(img.w, img.h, &img.rgba());
    match settings::write_file(path, &png) {
        Ok(()) => {
            wl!(out, "Wrote {path} ({}x{})", img.w, img.h);
            0
        }
        Err(e) => {
            wl!(out, "{e}");
            1
        }
    }
}

/// Draws `src` over `dst` at (x, y), both premultiplied.
fn compose(dst: &mut Canvas, src: &Canvas, x: i32, y: i32) {
    for sy in 0..src.h as i32 {
        for sx in 0..src.w as i32 {
            let (dx, dy) = (sx + x, sy + y);
            if dx < 0 || dy < 0 || dx as usize >= dst.w || dy as usize >= dst.h {
                continue;
            }
            let s = src.px[sy as usize * src.w + sx as usize];
            let d = &mut dst.px[dy as usize * dst.w + dx as usize];
            let sa = (s >> 24) as u32;
            let inv = 255 - sa;
            let ch = |sh: u32| ((s >> sh) & 255) + (((*d >> sh) & 255) * inv + 127) / 255;
            *d = (ch(24).min(255) << 24) | (ch(16).min(255) << 16) | (ch(8).min(255) << 8) | ch(0).min(255);
        }
    }
}
