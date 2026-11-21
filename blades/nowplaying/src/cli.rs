//! Command-line options for `NowPlaying.exe`.
use crate::render::{FrameState, Renderer};
use crate::settings::{self, Settings};
use crate::smtc::{self, MediaSnapshot};
use crate::ipc;
use alloc::string::String;
use core::fmt::Write;
use pk::out::{StdoutKind, Writer, attach_parent_console, console_output, stdout_handle, stdout_kind};
use windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx};

macro_rules! wl {
    ($out:expr) => { $out.write_bytes(b"\r\n") };
    ($out:expr, $($t:tt)*) => {{ let _ = write!($out, $($t)*); $out.write_bytes(b"\r\n"); }};
}

const HELP: &str = "NowPlaying \u{2014} live media cover, seekable timeline and frequency meter in the taskbar.\r\n\
\r\n\
  NowPlaying.exe                        start the taskbar strip (single instance)\r\n\
  NowPlaying.exe --status               print the current SMTC media session (read-only)\r\n\
  NowPlaying.exe --autostart on|off|status\r\n\
  NowPlaying.exe --exit                 close the running strip\r\n\
  NowPlaying.exe --settings             print settings for PocketKnives Settings\r\n\
  NowPlaying.exe --set key=value ...    update settings\r\n\
  NowPlaying.exe --preview-render PATH  render a 2x preview PNG of the strip\r\n\
  NowPlaying.exe --version              print version";

pub fn run(args: &[String]) -> u32 {
    let mut out = open_console();
    let cmd = args[0].to_ascii_lowercase();
    match cmd.as_str() {
        "--version" => {
            wl!(out, "NowPlaying {}  (Rust, no_std)", env!("CARGO_PKG_VERSION"));
            0
        }
        "--help" | "-h" | "/?" => {
            wl!(out, "{HELP}");
            0
        }
        "--exit" => {
            let _ = ipc::request_exit();
            0
        }
        "--status" => status_cmd(&mut out),
        "--settings" => {
            let s = Settings::load();
            out.write_bytes(s.format_blade_settings().as_bytes());
            0
        }
        "--set" => set_cmd(&mut out, &args[1..]),
        "--autostart" => autostart_cmd(&mut out, args.get(1).map_or("status", String::as_str)),
        "--preview-render" => match args.get(1) {
            Some(path) => preview_render_cmd(&mut out, path),
            None => {
                wl!(out, "Usage: NowPlaying.exe --preview-render <path.png>");
                2
            }
        },
        _ => {
            wl!(out, "Unknown option: {}", args[0]);
            wl!(out, "{HELP}");
            2
        }
    }
}

fn open_console() -> Writer {
    if stdout_kind() == StdoutKind::FileOrPipe {
        return Writer::new(stdout_handle());
    }
    if attach_parent_console() {
        if let Some(mut con) = console_output() {
            wl!(con);
            return con;
        }
    }
    Writer::null()
}

fn status_cmd(out: &mut Writer) -> u32 {
    match smtc::read_status_once() {
        Ok(m) if m.active => {
            let pos = m.live_position_sec() as u64;
            let dur = m.duration_sec() as u64;
            wl!(out, "State:    {}", if m.playing { "Playing" } else { "Paused" });
            wl!(out, "App:      {}", m.aumid);
            wl!(out, "Title:    {}", m.title);
            wl!(out, "Artist:   {}", m.artist);
            wl!(out, "Timeline: {}:{:02} / {}:{:02} (seekable={})", pos / 60, pos % 60, dur / 60, dur % 60, m.can_seek);
            wl!(out, "Artwork:  {} bytes", m.thumbnail.len());
            0
        }
        Ok(_) => {
            wl!(out, "No active media session.");
            1
        }
        Err(e) => {
            wl!(out, "Error: {e}");
            2
        }
    }
}

fn set_cmd(out: &mut Writer, pairs: &[String]) -> u32 {
    let cur = Settings::load();
    match cur.apply_pairs(pairs) {
        Ok(next) => {
            if let Err(e) = next.save() {
                wl!(out, "{e}");
                return 2;
            }
            if !pk::env::settings_preview() {
                ipc::notify_reload();
            }
            0
        }
        Err(e) => {
            wl!(out, "{e}");
            2
        }
    }
}

fn autostart_cmd(out: &mut Writer, arg: &str) -> u32 {
    match arg.to_ascii_lowercase().as_str() {
        "on" => match settings::set_autostart(true) {
            Ok(()) => {
                wl!(out, "Autostart: on");
                0
            }
            Err(e) => {
                wl!(out, "Error: {e}");
                2
            }
        },
        "off" => match settings::set_autostart(false) {
            Ok(()) => {
                wl!(out, "Autostart: off");
                0
            }
            Err(e) => {
                wl!(out, "Error: {e}");
                2
            }
        },
        "status" => {
            wl!(out, "Autostart: {}", if settings::autostart_enabled() { "on" } else { "off" });
            0
        }
        _ => {
            wl!(out, "Usage: NowPlaying.exe --autostart on|off|status");
            2
        }
    }
}

fn preview_render_cmd(out: &mut Writer, path: &str) -> u32 {
    let _ = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
    let Some(mut renderer) = Renderer::new(192) else {
        wl!(out, "Error: could not initialize Direct2D/WIC renderer.");
        return 2;
    };
    let sample = MediaSnapshot {
        active: true,
        playing: true,
        title: String::from("Midnight City"),
        artist: String::from("M83"),
        aumid: String::from("Spotify.exe"),
        can_seek: true,
        start_100ns: 0,
        end_100ns: 243 * 10_000_000,
        position_100ns: 84 * 10_000_000,
        last_updated_ft: 0,
        art_version: 1,
        thumbnail: alloc::vec::Vec::new(),
    };
    let state = FrameState {
        media: &sample,
        light_taskbar: false,
        show_visualizer: true,
        bands: [0.82, 0.94, 0.42, 0.78, 0.54],
        strip_hover_alpha: 0.0,
        btn_hover_alpha: [0.0; 3],
        btn_pressed: None,
        scrub_active_alpha: 0.0,
        scrub_hover: false,
        scrub_dragging: false,
        scrub_hover_ratio: 0.0,
        drag_pos_sec: 84.0,
    };
    match renderer.save_preview_png(path, &state) {
        Ok(()) => {
            wl!(out, "Wrote {path}");
            0
        }
        Err(e) => {
            wl!(out, "Error: {e}");
            2
        }
    }
}
