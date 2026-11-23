//! IPC contract, target geometry, quality presets and shared constants between `snipsnap.exe`
//! and `snipsnap-rec.exe`.
use windows_sys::Win32::Foundation::RECT;
use windows_sys::Win32::UI::WindowsAndMessaging::WM_APP;

pub const REC_CLASS: *const u16 = pk::w!("SnipSnapRec");
pub const REC_MUTEX: *const u16 = pk::w!("Local\\SnipSnap.Recorder");
pub const COPYDATA_TARGET: usize = 0x5352_4543; // "SREC"

pub const WM_APP_WARM_DONE: u32 = WM_APP + 1;
pub const WM_APP_STOP: u32 = WM_APP + 2;
pub const WM_APP_REC_ENDED: u32 = WM_APP + 3;
pub const WM_APP_TRAY: u32 = WM_APP + 4;
pub const WM_APP_PREP_DONE: u32 = WM_APP + 5;
pub const WM_APP_SAVE_DONE: u32 = WM_APP + 6;

/// Binary payload received via `WM_COPYDATA` from `snipsnap.exe`.
/// Must match `RecTarget` in `blades/snipsnap/src/capture.rs` field-for-field.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RecTarget {
    /// 0 = rectangle, 1 = window, 2 = full monitor.
    pub kind: u32,
    /// Physical screen coordinates (clamped to target monitor for Rect, frame bounds for Window).
    pub rect: RECT,
    /// Target window handle for Window mode (0 otherwise).
    pub hwnd: usize,
    /// Target monitor handle (`HMONITOR`).
    pub monitor: usize,
    /// Non-zero when SnipSnap's flash setting is enabled.
    pub flash: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RecKind {
    Rect,
    Window,
    Full,
}

impl RecTarget {
    pub fn kind(self) -> RecKind {
        match self.kind {
            1 => RecKind::Window,
            2 => RecKind::Full,
            _ => RecKind::Rect,
        }
    }
}

/// Resolution cap presets selected with keys `1`–`4` (downscale-only, default `Q720`).
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Quality {
    Q480 = 0,
    Q720 = 1,
    Q1080 = 2,
    Native = 3,
}

impl Quality {
    pub fn from_index(i: usize) -> Self {
        match i {
            0 => Quality::Q480,
            2 => Quality::Q1080,
            3 => Quality::Native,
            _ => Quality::Q720,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Quality::Q480 => "480p",
            Quality::Q720 => "720p",
            Quality::Q1080 => "1080p",
            Quality::Native => "Native",
        }
    }

    pub fn index(self) -> usize {
        self as usize
    }

    /// Maximum output height in pixels (`0` = no downscale).
    pub fn max_height(self) -> u32 {
        match self {
            Quality::Q480 => 480,
            Quality::Q720 => 720,
            Quality::Q1080 => 1080,
            Quality::Native => 0,
        }
    }

    /// Lightweight target H.264 bitrate (bits/sec) tailored to crisp UI clips, scaled gently by FPS.
    pub fn bitrate(self, out_w: u32, out_h: u32, fps: Fps) -> u32 {
        let base = match self {
            Quality::Q480 => 1_200_000,
            Quality::Q720 => 2_500_000,
            Quality::Q1080 => 5_000_000,
            Quality::Native => {
                let px = (out_w as u64) * (out_h as u64);
                if px >= 3_600_000 {
                    9_000_000
                } else if px >= 2_000_000 {
                    6_000_000
                } else {
                    3_500_000
                }
            }
        };
        match fps {
            Fps::F24 => (base as u64 * 60 / 100) as u32,
            Fps::F30 => (base as u64 * 70 / 100) as u32,
            Fps::F60 => base,
        }
    }
}

/// Frame-rate cap presets on the adjacent F2 FPS drum (default `F60`).
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Fps {
    F24 = 0,
    F30 = 1,
    F60 = 2,
}

impl Fps {
    pub fn from_index(i: usize) -> Self {
        match i {
            0 => Fps::F24,
            1 => Fps::F30,
            _ => Fps::F60,
        }
    }

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn fps(self) -> u32 {
        match self {
            Fps::F24 => 24,
            Fps::F30 => 30,
            Fps::F60 => 60,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Fps::F24 => "24f",
            Fps::F30 => "30f",
            Fps::F60 => "60f",
        }
    }

    /// Nominal frame duration in 100 ns units.
    pub fn frame_dur_100ns(self) -> i64 {
        match self {
            Fps::F24 => 416_667,
            Fps::F30 => 333_333,
            Fps::F60 => 166_667,
        }
    }

    /// Minimum inter-frame interval in 100 ns (~85% of nominal so compositor jitter isn't dropped).
    pub fn min_interval_100ns(self) -> i64 {
        match self {
            Fps::F24 => 355_000,
            Fps::F30 => 285_000,
            Fps::F60 => 140_000,
        }
    }
}

/// Computes even H.264 output dimensions `(out_w, out_h)` from physical source dimensions.
/// Never upscales regions smaller than the chosen preset cap.
pub fn output_size(src_w: u32, src_h: u32, q: Quality) -> (u32, u32) {
    let sw = src_w.max(2);
    let sh = src_h.max(2);
    let cap = q.max_height();
    let (w, h) = if cap > 0 && sh > cap {
        let scaled_w = ((sw as u64 * cap as u64 + (sh as u64 / 2)) / sh as u64) as u32;
        (scaled_w, cap)
    } else {
        (sw, sh)
    };
    let even = |v: u32| (v & !1).max(2);
    (even(w), even(h))
}
