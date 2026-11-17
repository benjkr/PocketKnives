//! The tray icon's artwork and animation: a small scanline rasterizer drawing the two AirPods
//! glyphs (`assets/airpods-pro-*.svg`), the low-battery dot and every animation phase, plus the
//! PNG writer for check images. The same code draws in the exe and in the simulator
//! (docs/specs/icon-and-animation.md).
//!
//! GROUNDWORK STUB: the API is final; the bodies are placeholders until the renderer is ported.
use alloc::vec;
use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IconState {
    Connected,
    Disconnected,
    LowBattery,
}

impl IconState {
    /// `Enum.Parse<IconState>(name, ignoreCase: true)`.
    pub fn parse(name: &str) -> Option<IconState> {
        [IconState::Connected, IconState::Disconnected, IconState::LowBattery].into_iter().find(|s| s.name().eq_ignore_ascii_case(name))
    }

    pub fn name(self) -> &'static str {
        match self {
            IconState::Connected => "Connected",
            IconState::Disconnected => "Disconnected",
            IconState::LowBattery => "LowBattery",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AnimPhase {
    Connecting,
    Landing,
    Disconnecting,
    Cancelling,
    LowBattery,
}

impl AnimPhase {
    /// The C# enum's name, as logs, timelines and the simulator write it.
    pub fn name(self) -> &'static str {
        match self {
            AnimPhase::Connecting => "Connecting",
            AnimPhase::Landing => "Landing",
            AnimPhase::Disconnecting => "Disconnecting",
            AnimPhase::Cancelling => "Cancelling",
            AnimPhase::LowBattery => "LowBattery",
        }
    }

    /// `Enum.Parse<AnimPhase>(name)` (exact case).
    pub fn parse(name: &str) -> Option<AnimPhase> {
        [AnimPhase::Connecting, AnimPhase::Landing, AnimPhase::Disconnecting, AnimPhase::Cancelling, AnimPhase::LowBattery]
            .into_iter()
            .find(|p| p.name() == name)
    }
}

/// A static icon, straight-alpha BGRA, `size * size * 4` bytes.
pub fn render_bgra(_state: IconState, _light_taskbar: bool, size: i32) -> Vec<u8> {
    vec![0; (size * size * 4) as usize]
}

/// One animation frame. `t`: seconds since the phase started; `tracer_t`: seconds the tracer has
/// been running (continuous across phases).
pub fn render_anim(_phase: AnimPhase, _t: f64, _tracer_t: f64, _light_taskbar: bool, size: i32) -> Vec<u8> {
    vec![0; (size * size * 4) as usize]
}

/// Has the phase played out at `t` seconds?
pub fn phase_done(_phase: AnimPhase, _t: f64, _still_pending_disconnect: bool) -> bool {
    true
}

/// A large render of a whole glyph (black on white, 10 px per glyph unit) for checking the artwork.
pub fn render_large_glyph(_fill: bool, size: i32) -> Vec<u8> {
    vec![0; (size * size * 4) as usize]
}

/// Draws with other artwork (SVG text) until reset; `None` restores the built-in glyph.
/// Simulator only.
pub fn set_glyphs(_fill_svg: Option<&str>, _line_svg: Option<&str>) {}

pub mod png {
    use alloc::vec::Vec;

    /// A PNG of straight-alpha BGRA pixels (RGBA, stored deflate blocks), byte for byte what the
    /// C# writer produced.
    pub fn encode(_bgra: &[u8], _width: usize, _height: usize) -> Vec<u8> {
        Vec::new()
    }
}
