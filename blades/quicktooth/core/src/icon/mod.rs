//! The tray icon's artwork and animation: a small scanline rasterizer drawing the two AirPods
//! glyphs (`assets/airpods-pro-*.svg`), the low-battery dot and every animation phase, plus the
//! PNG writer for check images. The same code draws in the exe and in the simulator
//! (docs/specs/icon-and-animation.md).
//!
//! Connected = solid, with the ear tips dimmed; disconnected = the reverse: hollow, dimmed body
//! and stem with solid ear tips. Each glyph carries its own per-path opacities.
use crate::math::{PI, ceil, clamp, cos, max, min, pow, sin, sqrt};
use crate::tunables::{self, T};
use alloc::vec::Vec;
use glyph::{Polyline, Pt, fill, line, point_at};
use raster::{Canvas, Clip, Rgb, Xf, over};

mod glyph;
pub mod png;
mod raster;
mod sort;

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

const ON_DARK: Rgb = Rgb::hex(tunables::ON_DARK_TASKBAR);
const ON_LIGHT: Rgb = Rgb::hex(tunables::ON_LIGHT_TASKBAR);
const LOW_ON_DARK: Rgb = Rgb::hex(tunables::LOW_BATTERY_ON_DARK);
const LOW_ON_LIGHT: Rgb = Rgb::hex(tunables::LOW_BATTERY_ON_LIGHT);
const BLACK: Rgb = Rgb { r: 0, g: 0, b: 0 };

fn color(light: bool) -> Rgb {
    if light { ON_LIGHT } else { ON_DARK }
}

fn low_color(light: bool) -> Rgb {
    if light { LOW_ON_LIGHT } else { LOW_ON_DARK }
}

fn tune(t: T) -> f64 {
    tunables::get(t)
}

/// Maps glyph units (24-unit box, centre 12,12) onto an icon of `size` px at `zoom`.
fn glyph_transform(size: i32, zoom: f64) -> Xf {
    let s = size as f64 * zoom / 24.0;
    Xf::new(s, size as f64 / 2.0 - 12.0 * s, size as f64 / 2.0 - 12.0 * s)
}

/// A static icon, straight-alpha BGRA, `size * size * 4` bytes.
pub fn render_bgra(state: IconState, light_taskbar: bool, size: i32) -> Vec<u8> {
    let mut canvas = Canvas::new(size);
    canvas.draw(if state == IconState::Disconnected { line() } else { fill() }, 1.0, glyph_transform(size, tune(T::Zoom)), Clip::NONE);
    if state != IconState::LowBattery {
        return canvas.to_bgra(color(light_taskbar));
    }

    // Low battery: an orange dot in the bottom-right corner, with a ring cut out of the artwork
    // around it (the right stem's tip) so it stands apart. Laid out on a 16-unit grid (whole units
    // = whole pixels at 16 px), scaled to the icon size.
    let grid = Xf::new(size as f64 / 16.0, 0.0, 0.0);
    let r = tune(T::LowBatteryDotRadius);
    canvas.erase(&disc(LOW_DOT_X, LOW_DOT_Y, r + tune(T::LowBatteryDotGap)), grid);
    let mut dot = Canvas::new(size);
    dot.fill(&[disc(LOW_DOT_X, LOW_DOT_Y, r)], 1.0, grid);
    over(canvas.to_bgra(color(light_taskbar)), &dot.to_bgra(low_color(light_taskbar)))
}

// The dot's centre on the 16-unit grid: the free bottom-right corner below the right head.
const LOW_DOT_X: f64 = 13.5;
const LOW_DOT_Y: f64 = 13.0;

fn disc(cx: f64, cy: f64, r: f64) -> Vec<Pt> {
    const N: i32 = 48;
    (0..N).map(|k| Pt { x: cx + r * cos(2.0 * PI * k as f64 / N as f64), y: cy + r * sin(2.0 * PI * k as f64 / N as f64) }).collect()
}

// ------------------------------------------------------------------ low-battery ripple
//
// While the dot shows: one thin orange ring spreads out from it and fades, LowBatteryAnimSeconds
// long, when the dot appears and then every LowBatteryReplayMs. Each ripple ends exactly on the
// static low-battery icon.

fn render_low_battery_anim(u: f64, light_taskbar: bool, size: i32) -> Vec<u8> {
    let still = render_bgra(IconState::LowBattery, light_taskbar, size);
    if u >= 1.0 {
        return still;
    }
    let grid = Xf::new(size as f64 / 16.0, 0.0, 0.0);
    let (r, gap) = (tune(T::LowBatteryDotRadius), tune(T::LowBatteryDotGap));
    // Starts just outside the cut gap, so it never merges with the dot.
    let e = ease_out(u);
    let rr = r + gap + 0.4 + 3.0 * e;
    let w = 0.8 - 0.3 * e;
    let mut inner = disc(LOW_DOT_X, LOW_DOT_Y, rr - w / 2.0);
    inner.reverse();
    let mut ring = Canvas::new(size);
    ring.fill(&[disc(LOW_DOT_X, LOW_DOT_Y, rr + w / 2.0), inner], 0.85 * (1.0 - u), grid);
    over(still, &ring.to_bgra(low_color(light_taskbar)))
}

// ------------------------------------------------------------------ pending animation ("tracer")
//
// Connecting:    the buds spring apart, then a short glow runs around each earbud's outline (head
//                and stem; steady brightness, never loops back to an earlier look) until the connection lands.
// Landing:       the buds close together while the solid fills up from the bottom.
// Disconnecting: the solid drains top-down while the buds part, then the outline settles back;
//                if the disconnect is still pending after that, the tracer runs on the outline.
// Cancelling:    a connect that failed: the buds glide back and the tracer fades out.
// Every phase ends exactly on the static connected / not-connected icon.

// Derived from the artwork, so a regenerated glyph needs no retuning here.
fn art_bounds() -> glyph::Bounds {
    fill().bounds.union(line().bounds)
}

/// Zoom of the fully-apart pose: the largest at which the outermost edge, moved out by `Travel`,
/// still stays inside the icon (x ≥ 0 at any size).
fn apart_zoom() -> f64 {
    min(tune(T::Zoom), 12.0 / (12.0 - art_bounds().left + tune(T::Travel)))
}

/// The tracer spot's orbit around the left head (glyph units): the line glyph's `data-tracer`;
/// without one, a circle inscribed in the left half of the artwork's head row.
fn orbit() -> (f64, f64, f64, f64) {
    if let Some(t) = line().tracer {
        return (t[0], t[1], t[2], t[3]);
    }
    let b = art_bounds();
    let r = (12.0 - b.left) / 2.0;
    (b.left + r, b.top + r, r, r)
}

/// Where the left earbud's spot is after `tracer_t` seconds (glyph units): on an ellipse around
/// the head, one lap per `TracerPeriod`.
fn tracer_spot(tracer_t: f64) -> Pt {
    let (cx, cy, rx, ry) = orbit();
    let angle = 2.0 * PI * tracer_t / tune(T::TracerPeriod);
    Pt { x: cx + rx * sin(angle), y: cy - ry * cos(angle) }
}

/// `TracerOutline` = 1: the lit stretch of the left earbud's outer outline after `tracer_t`
/// seconds, as a dense row of points on the outline. It runs clockwise from the top of the head
/// at the head orbit's speed (so a lap takes as much longer as the outline is), `TracerLength`
/// long and reaching `TracerWidth` either side of the edge: narrower than the stem, so the far
/// side of the stem stays dark.
fn tracer_band(path: &Polyline, tracer_t: f64) -> Vec<Pt> {
    let (_, _, rx, ry) = orbit();
    let h = pow(rx - ry, 2.0) / pow(rx + ry, 2.0);
    let orbit_length = PI * (rx + ry) * (1.0 + 3.0 * h / (10.0 + sqrt(4.0 - 3.0 * h))); // Ramanujan
    let centre = tracer_t * orbit_length / tune(T::TracerPeriod);
    let length = min(tune(T::TracerLength), path.length);
    let n = 2.max(ceil(length / 0.2) as i32 + 1);
    (0..n).map(|k| point_at(path, centre - length / 2.0 + length * k as f64 / (n - 1) as f64)).collect()
}

/// Has the phase played out at `t` seconds?
pub fn phase_done(phase: AnimPhase, t: f64, still_pending_disconnect: bool) -> bool {
    match phase {
        AnimPhase::Landing => t >= tune(T::LandSeconds),
        AnimPhase::Cancelling => t >= tune(T::CancelSeconds),
        // Plays to its end pose; if still pending after that, the tracer runs until it lands.
        AnimPhase::Disconnecting => t >= tune(T::DisconnectSeconds) && !still_pending_disconnect,
        AnimPhase::Connecting => false,
        AnimPhase::LowBattery => t >= tune(T::LowBatteryAnimSeconds),
    }
}

fn clamp01(v: f64) -> f64 {
    clamp(v, 0.0, 1.0)
}

fn ease_out(t: f64) -> f64 {
    1.0 - pow(1.0 - t, 3.0)
}

fn ease_in_out(t: f64) -> f64 {
    if t < 0.5 { 4.0 * t * t * t } else { 1.0 - pow(-2.0 * t + 2.0, 3.0) / 2.0 }
}

/// One animation frame. `t`: seconds since the phase started; `tracer_t`: seconds the tracer has
/// been running (continuous across phases).
pub fn render_anim(phase: AnimPhase, t: f64, tracer_t: f64, light_taskbar: bool, size: i32) -> Vec<u8> {
    if phase == AnimPhase::LowBattery {
        return render_low_battery_anim(clamp01(t / tune(T::LowBatteryAnimSeconds)), light_taskbar, size);
    }
    let travel = tune(T::Travel);
    let mut tracer_t = tracer_t;
    // Bud offset, solid level (0 = none, 1 = full), tracer opacity.
    let (dx, level, tracer) = match phase {
        AnimPhase::Connecting => (travel * ease_out(clamp01(t / tune(T::SpringSeconds))), 0.0, clamp01(t / 0.15)),
        AnimPhase::Landing => {
            let u = clamp01(t / tune(T::LandSeconds));
            (
                travel * (1.0 - ease_out(clamp01(u / 0.5))), // close the gap in the first half
                ease_in_out(clamp01((u - 0.2) / 0.8)),       // fill up from the bottom
                1.0 - clamp01(u / 0.3),
            )
        }
        AnimPhase::Disconnecting => {
            let ds = tune(T::DisconnectSeconds);
            let u = clamp01(t / ds);
            let a = clamp01(u / 0.6);
            tracer_t = max(0.0, t - ds);
            (
                travel * ease_out(a) * (1.0 - ease_in_out(clamp01((u - 0.6) / 0.4))), // part, then settle back
                1.0 - ease_in_out(a),                                                 // drain top-down
                if t > ds { clamp01((t - ds) / 0.15) } else { 0.0 },
            )
        }
        _ => {
            // Cancelling
            let u = clamp01(t / tune(T::CancelSeconds));
            (travel * (1.0 - ease_out(u)), 0.0, 1.0 - u)
        }
    };

    // At rest the glyph is drawn at Zoom; apart, the buds shrink to the apart zoom, the size at
    // which the fully-apart pose still fits inside the icon without clipping.
    let zoom = tune(T::Zoom);
    let pose_zoom = zoom + (apart_zoom() - zoom) * (dx / travel);
    let art = art_bounds();
    let y_level = art.bottom - level * (art.bottom - art.top);
    let (line, fill) = (line(), fill());
    // Without a left-earbud outline (only an override can lack one) the glow is the spot.
    let band_left = match &line.outline {
        Some(path) if tune(T::TracerOutline) >= 0.5 => Some(tracer_band(path, tracer_t)),
        _ => None,
    };

    let mut canvas = Canvas::new(size);
    let base_xf = glyph_transform(size, pose_zoom);
    for (left, offset) in [(true, -dx), (false, dx)] {
        let xf = base_xf.offset(offset);
        let half = Clip::NONE.intersect(if left { 0.0 } else { 12.0 }, 0.0, if left { 12.0 } else { 24.0 }, 24.0); // one earbud per half

        // Above the fill line: the dim outline, plus the tracer.
        let above = half.intersect(0.0, 0.0, 24.0, y_level);
        canvas.draw(line, 1.0, xf, above);
        if tracer > 0.001 {
            // Left runs clockwise, right counter-clockwise (mirrored): along each outline, or around each head.
            if let Some(band_left) = &band_left {
                let band: Vec<Pt> = if left { band_left.clone() } else { band_left.iter().map(|p| Pt { x: 24.0 - p.x, y: p.y }).collect() };
                canvas.draw(line, tracer, xf, above.with_dots(&band, tune(T::TracerWidth)));
            } else {
                let spot_left = tracer_spot(tracer_t);
                let spot = [if left { spot_left } else { Pt { x: 24.0 - spot_left.x, y: spot_left.y } }];
                canvas.draw(line, tracer, xf, above.with_dots(&spot, tune(T::SpotRadius)));
            }
        }

        // Below the fill line: solid.
        if level > 0.001 {
            canvas.draw(fill, 1.0, xf, half.intersect(0.0, y_level, 24.0, 24.0));
        }
    }
    canvas.to_bgra(color(light_taskbar))
}

/// A large render of a whole glyph (black on white, 10 px per glyph unit) for checking the artwork.
pub fn render_large_glyph(fill_glyph: bool, size: i32) -> Vec<u8> {
    let mut canvas = Canvas::new(size);
    canvas.draw(if fill_glyph { fill() } else { line() }, 1.0, Xf::new(size as f64 / 24.0, 0.0, 0.0), Clip::NONE);
    canvas.to_bgra(BLACK)
}

/// Draws with other artwork (SVG text) until reset; `None` restores the built-in glyph.
/// Simulator only.
pub fn set_glyphs(fill_svg: Option<&str>, line_svg: Option<&str>) {
    glyph::set_override(fill_svg, line_svg);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests run in parallel and the tunables are process-wide, so these use the defaults only.

    #[test]
    fn phase_timing() {
        assert!(!phase_done(AnimPhase::Connecting, 1e9, false));
        assert!(!phase_done(AnimPhase::Landing, 0.49, false));
        assert!(phase_done(AnimPhase::Landing, 0.5, false));
        assert!(!phase_done(AnimPhase::Cancelling, 0.29, false));
        assert!(phase_done(AnimPhase::Cancelling, 0.3, false));
        assert!(phase_done(AnimPhase::Disconnecting, 0.8, false));
        assert!(!phase_done(AnimPhase::Disconnecting, 5.0, true));
        assert!(!phase_done(AnimPhase::Disconnecting, 0.79, false));
        assert!(phase_done(AnimPhase::LowBattery, 0.7, false));
        assert!(!phase_done(AnimPhase::LowBattery, 0.69, false));
    }

    #[test]
    fn every_phase_ends_on_the_static_icon() {
        for light in [false, true] {
            for size in [16, 20, 24, 32] {
                let connected = render_bgra(IconState::Connected, light, size);
                let disconnected = render_bgra(IconState::Disconnected, light, size);
                let low = render_bgra(IconState::LowBattery, light, size);
                assert_eq!(render_anim(AnimPhase::Landing, 0.5, 2.0, light, size), connected);
                assert_eq!(render_anim(AnimPhase::Cancelling, 0.3, 2.0, light, size), disconnected);
                assert_eq!(render_anim(AnimPhase::Disconnecting, 0.8, 0.0, light, size), disconnected);
                assert_eq!(render_anim(AnimPhase::LowBattery, 0.7, 0.0, light, size), low);
                assert_ne!(render_anim(AnimPhase::Connecting, 0.5, 0.5, light, size), disconnected);
            }
        }
    }

    #[test]
    fn sizes_and_colours() {
        let px = render_bgra(IconState::Connected, false, 16);
        assert_eq!(px.len(), 16 * 16 * 4);
        let opaque: Vec<_> = px.chunks(4).filter(|p| p[3] == 255).collect();
        assert!(!opaque.is_empty());
        assert!(opaque.iter().all(|p| p[..3] == [0xFF, 0xFF, 0xFF]));
        let low = render_bgra(IconState::LowBattery, true, 16);
        assert!(low.chunks(4).any(|p| p == [0x10, 0x50, 0xCA, 0xFF]));
        assert_eq!(render_large_glyph(true, 240).len(), 240 * 240 * 4);
    }

    #[test]
    fn override_falls_back_to_built_in() {
        let before = render_large_glyph(false, 48);
        set_glyphs(None, Some("<svg>no paths</svg>"));
        assert_eq!(render_large_glyph(false, 48), before);
        set_glyphs(None, None);
    }
}
