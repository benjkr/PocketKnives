//! The menu's header while the buds' place is known (docs/specs/tray.md): the device name,
//! centred; a line-art picture of the case with the buds where they are (nested in the
//! open case, or standing beside it once out); and a battery gauge with its level under each bud
//! and the case. In the menu's own colours (Windows 11, dark or light). The core draws the
//! picture; the host draws the text items on it in Segoe UI (it owns the fonts).
//!
//! Laid out in 96-DPI pixels and drawn at `scale`.
use crate::battery::Pods;
use crate::icon::case::{ring, rounded};
use crate::icon::glyph::{Pt, parse_tones};
use crate::icon::raster::{Canvas, Clip, Rgb, Xf, over};
use crate::math::max;
use alloc::string::String;
use alloc::vec::Vec;

/// What the header shows.
#[derive(Clone, Debug, PartialEq)]
pub struct HeaderInfo {
    pub name: String,
    pub pods: Pods,
    pub connected: bool,
    /// Levels in % (None = not known) and whether each charges.
    pub left: Option<(i32, bool)>,
    pub right: Option<(i32, bool)>,
    pub case: Option<(i32, bool)>,
    /// At or below this a level is orange.
    pub low_warning: i32,
    /// The menu's theme (it follows the taskbar's).
    pub light: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Align {
    Left,
    Center,
    Right,
}

/// A text item for the host to draw: Segoe UI at `size` px, its baseline at `y` (both hosts agree
/// on a baseline; GDI's text top includes the font's internal leading), aligned on `x`.
#[derive(Clone, Debug, PartialEq)]
pub struct Text {
    pub x: i32,
    pub y: i32,
    pub size: i32,
    pub text: String,
    pub bold: bool,
    pub align: Align,
    /// `0xRRGGBB`.
    pub color: u32,
}

/// The header: opaque BGRA pixels, top-down, on the menu's background, and the text to draw on them.
#[derive(Clone, Debug, PartialEq)]
pub struct Header {
    /// The scaling it was drawn at (1 = 96 DPI).
    pub scale: f64,
    pub width: i32,
    pub height: i32,
    /// The menu's background, `0xRRGGBB` (the host fills any room around the picture with it).
    pub background: u32,
    pub bgra: Vec<u8>,
    pub texts: Vec<Text>,
}

/// The menu's colours (background measured on Windows 11; the rest Fluent's).
struct Palette {
    bg: u32,
    text: u32,
    disabled: u32,
    ink: u32,
    ok: u32,
    warn: u32,
}

const DARK: Palette = Palette { bg: 0x2C2C2C, text: 0xFFFFFF, disabled: 0x6D6D6D, ink: 0xF2F2F2, ok: 0x6CCB5F, warn: 0xFFA348 };
const LIGHT: Palette = Palette { bg: 0xF9F9F9, text: 0x000000, disabled: 0x9F9F9F, ink: 0x282828, ok: 0x0F7B0F, warn: 0xC2500A };

const WIDTH: f64 = 236.0;
const HEIGHT: f64 = 102.0;
const MID: f64 = WIDTH / 2.0;
/// The gauges' row, and each gauge's size.
const GAUGE_Y: f64 = 83.0;
const GAUGE_W: f64 = 16.0;
const GAUGE_H: f64 = 10.0;
const TEXT_SIZE: f64 = 12.0;
/// Where the buds out of the case stand (their middles), and the levels under them.
const SIDE_X: f64 = 40.0;

/// The layers, in drawing order: the line art, the green and orange levels, then the bolts.
struct Layers {
    ink: Canvas,
    ok: Canvas,
    warn: Canvas,
    top: Canvas,
}

/// The header at `scale` (1 = 96 DPI).
pub fn header(info: &HeaderInfo, scale: f64) -> Header {
    let c = if info.light { &LIGHT } else { &DARK };
    let (w, h) = (round(WIDTH * scale), round(HEIGHT * scale));
    let xf = Xf::new(scale, 0.0, 0.0);
    let mut l = Layers { ink: Canvas::rect(w, h), ok: Canvas::rect(w, h), warn: Canvas::rect(w, h), top: Canvas::rect(w, h) };
    let mut texts = Vec::new();
    let mut text = |x: f64, baseline: f64, s: &str, bold: bool, align: Align, color: u32| {
        texts.push(Text { x: round(x * scale), y: round(baseline * scale), size: round(TEXT_SIZE * scale), text: s.into(), bold, align, color })
    };
    text(MID, 16.0, &info.name, true, Align::Center, c.text);

    // The picture: the case, centred (open while a bud is in it, closed and faint once both are
    // out); a bud that's out stands beside it.
    let mut bgra: Vec<u8> = [(c.bg & 0xFF) as u8, ((c.bg >> 8) & 0xFF) as u8, (c.bg >> 16) as u8, 0xFF].repeat((w * h) as usize);
    let mut paint = Painter { bgra: &mut bgra, canvas: Canvas::rect(w, h), connected: info.connected, light: info.light };
    let (in_left, in_right) = match info.pods {
        Pods::Closed | Pods::Open => (true, true),
        Pods::OneOut { left } => (!left, left),
        Pods::Out => (false, false),
    };
    let case = Place::art(CASE_BOUNDS, CASE_W, MID - CASE_W / 2.0, PIC_Y + 4.0, scale);
    if matches!(info.pods, Pods::Open | Pods::OneOut { .. }) {
        paint.art(CASE_OPEN_BACK, case, Clip::NONE, 1.0);
        for (left, seated) in [(true, in_left), (false, in_right)] {
            if seated {
                let (art, b) = bud(left);
                // Seated: the bud drawing at SEAT_SCALE, its top-left at the seat (case units), cut
                // off at the wells' front edge.
                let seat = Place { k: case.k * SEAT_SCALE, ox: case.at_x(if left { SEAT_X.0 } else { SEAT_X.1 }), oy: case.at_y(SEAT_Y), u0: b.0, v0: b.1, scale };
                paint.art(art, seat, Clip::NONE.intersect(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::INFINITY, b.1 + (SEAT_CUT - SEAT_Y) / SEAT_SCALE), 1.0);
            }
        }
        paint.art(CASE_OPEN_FRONT, case, Clip::NONE, 1.0);
    } else {
        paint.art(CASE_CLOSED, case, Clip::NONE, if info.pods == Pods::Out { 0.45 } else { 1.0 });
    }
    for (left, out) in [(true, !in_left), (false, !in_right)] {
        if out {
            let (art, b) = bud(left);
            let bw = OUT_H * (b.2 - b.0) / (b.3 - b.1);
            let x = if left { SIDE_X - bw / 2.0 } else { WIDTH - SIDE_X - bw / 2.0 };
            paint.art(art, Place::art(b, bw, x, PIC_Y + 4.0, scale), Clip::NONE, 1.0);
        }
    }

    // A gauge and its level under each: left, the case, right.
    for (cx, level) in [(SIDE_X, info.left), (MID, info.case), (WIDTH - SIDE_X, info.right)] {
        let gx = cx - 24.0;
        gauge(&mut l, xf, gx, GAUGE_Y, level, info.low_warning);
        let color = match level {
            None => c.disabled,
            Some((p, _)) if p <= info.low_warning => c.warn,
            _ => c.text,
        };
        let pct = level.map_or_else(|| String::from("\u{2014}"), |(p, _)| alloc::format!("{p}%"));
        // Digits centred on the gauge: the baseline a pixel above its bottom.
        text(gx + GAUGE_W + 7.0, GAUGE_Y + GAUGE_H - 1.0, &pct, false, Align::Left, color);
    }

    for (layer, color) in [(&l.ink, c.ink), (&l.ok, c.ok), (&l.warn, c.warn), (&l.top, c.ink)] {
        bgra = over(bgra, &layer.to_bgra(rgb(color)));
    }
    Header { scale, width: w, height: h, background: c.bg, bgra, texts }
}

fn rgb(c: u32) -> Rgb {
    Rgb { r: (c >> 16) as u8, g: ((c >> 8) & 0xFF) as u8, b: (c & 0xFF) as u8 }
}

fn round(v: f64) -> i32 {
    crate::math::round(v) as i32
}

// ---------------------------------------------------------------- the pictures

/// The pictures, generated by `tools/header_art.py`: layered SVGs, one flat tone per path, in
/// painting order. The buds are "Airpods Pro" by Alex Martynov (SVG Repo #324521, CC Attribution);
/// the case was drawn for QuickTooth (`tools/airpods_case.py`). All in the drawing's 128-unit box.
const BUD_LEFT: &str = include_str!("../../../assets/header-bud-left.svg");
const BUD_RIGHT: &str = include_str!("../../../assets/header-bud-right.svg");
const CASE_CLOSED: &str = include_str!("../../../assets/header-case-closed.svg");
const CASE_OPEN_BACK: &str = include_str!("../../../assets/header-case-open-back.svg");
const CASE_OPEN_FRONT: &str = include_str!("../../../assets/header-case-open-front.svg");

/// Ink bounds (left, top, right, bottom), drawing units: each bud, and the case (closed and open
/// share its place, so they swap without moving).
const BUD_LEFT_BOUNDS: (f64, f64, f64, f64) = (7.5, 16.5, 61.03, 91.84);
const BUD_RIGHT_BOUNDS: (f64, f64, f64, f64) = (66.97, 36.32, 120.5, 111.66);
const CASE_BOUNDS: (f64, f64, f64, f64) = (9.45, 22.31, 118.55, 117.55);
/// A seated bud: the bud drawing at this scale, its top-left at (x, y) in the case's units, cut
/// off below the wells' front edge.
const SEAT_SCALE: f64 = 0.72;
const SEAT_X: (f64, f64) = (19.73, 69.73);
const SEAT_Y: f64 = 45.92;
const SEAT_CUT: f64 = 72.92;
/// The picture's top, the case's width and a standing bud's height, header px.
const PIC_Y: f64 = 20.0;
const CASE_W: f64 = 60.0;
const OUT_H: f64 = 45.0;

fn bud(left: bool) -> (&'static str, (f64, f64, f64, f64)) {
    if left { (BUD_LEFT, BUD_LEFT_BOUNDS) } else { (BUD_RIGHT, BUD_RIGHT_BOUNDS) }
}

/// Drawing units to pixels: header px = o + k·(u − u0), pixels = scale · header px.
#[derive(Clone, Copy)]
struct Place {
    k: f64,
    ox: f64,
    oy: f64,
    u0: f64,
    v0: f64,
    scale: f64,
}

impl Place {
    /// `b` drawn `width` px wide with its top-left at (x, y).
    fn art(b: (f64, f64, f64, f64), width: f64, x: f64, y: f64, scale: f64) -> Place {
        Place { k: width / (b.2 - b.0), ox: x, oy: y, u0: b.0, v0: b.1, scale }
    }

    fn at_x(&self, u: f64) -> f64 {
        self.ox + self.k * (u - self.u0)
    }

    fn at_y(&self, v: f64) -> f64 {
        self.oy + self.k * (v - self.v0)
    }

    fn xf(&self) -> Xf {
        Xf::new(self.scale * self.k, self.scale * (self.ox - self.k * self.u0), self.scale * (self.oy - self.k * self.v0))
    }
}

/// Paints flat tones onto the header one after another, so later ones cover earlier ones.
struct Painter<'a> {
    bgra: &'a mut Vec<u8>,
    canvas: Canvas,
    connected: bool,
    light: bool,
}

impl Painter<'_> {
    fn art(&mut self, svg: &str, place: Place, clip: Clip<'_>, opacity: f64) {
        for (name, contours) in parse_tones(svg).unwrap_or_default() {
            let (color, alpha) = tone(name, self.connected, self.light);
            self.canvas.clear();
            self.canvas.fill_clipped(&contours, alpha * opacity, place.xf(), clip);
            let layer = self.canvas.to_bgra(rgb(color));
            *self.bgra = over(core::mem::take(self.bgra), &layer);
        }
    }
}

/// A tone's colour and opacity: the approved pixel-art design's materials, lit from the top left;
/// all grey when not connected (as that design greyed them).
fn tone(name: &str, connected: bool, light: bool) -> (u32, f64) {
    let color = match name {
        "shell" => 0xF2F4F8,
        "shell-hi" => 0xFFFFFF,
        "shell-lo" => 0xCDD5E0,
        "shell-lo2" => 0xB6C0CF,
        "tip" => 0xD5DEEA,
        "tip-hi" => 0xEEF3F9,
        "tip-lo" => 0xA7B4C7,
        "tip-lo2" => 0x8793A7,
        "mesh" | "vent-in" => 0x39414F,
        "vent" | "well" => 0x1B1F27,
        "cap" => 0xE2DACA,
        "cap-lo" => 0xB5AA93,
        "lid" => 0xE2E7EF,
        "lid-hi" => 0xF2F4F8,
        "lid-lo" => 0xCDD5E0,
        "hinge" => 0x9AA4B2,
        "mouth" => 0x343B48,
        "led" => 0x00C853,
        "line" => 0x586378,
        "line-under" => 0x1C2028,
        "shadow" => return (0x000000, if light { 0.12 } else { 0.35 }),
        _ => 0xFF00FF,
    };
    (if connected { color } else { grey(color) }, 1.0)
}

/// The pixel-art design's grey for a colour when not connected.
fn grey(c: u32) -> u32 {
    let y = 0.299 * (c >> 16) as f64 + 0.587 * ((c >> 8) & 0xFF) as f64 + 0.114 * (c & 0xFF) as f64;
    let g = crate::math::round(94.0 + (y - 32.0) * 0.641).clamp(0.0, 255.0) as u32;
    (g << 16) | (g << 8) | g
}

// ---------------------------------------------------------------- the gauge

/// A Fluent battery glyph: an outline, a nub, a bar to the level (green while charging, orange at
/// or below the warning), a bolt while charging.
fn gauge(l: &mut Layers, xf: Xf, x: f64, y: f64, level: Option<(i32, bool)>, low: i32) {
    let a = if level.is_some() { 0.9 } else { 0.35 };
    l.ink.fill_clipped(&ring(x + 0.5, y + 0.5, x + GAUGE_W - 0.5, y + GAUGE_H - 0.5, 2.5, 1.0), a, xf, Clip::NONE);
    l.ink.fill_clipped(&[rounded(x + GAUGE_W + 0.5, y + 3.0, x + GAUGE_W + 2.0, y + GAUGE_H - 3.0, 0.75)], a, xf, Clip::NONE);
    let Some((p, charging)) = level else { return };
    let bar = [rounded(x + 2.0, y + 2.0, x + 2.0 + max(1.5, (GAUGE_W - 4.0) * p as f64 / 100.0), y + GAUGE_H - 2.0, 1.0)];
    let layer = if p <= low {
        &mut l.warn
    } else if charging {
        &mut l.ok
    } else {
        &mut l.ink
    };
    layer.fill_clipped(&bar, 1.0, xf, Clip::NONE);
    if charging {
        let (bx, by, bh) = (x + GAUGE_W / 2.0 - 2.7, y + 0.5, GAUGE_H - 1.0);
        let bolt: Vec<Pt> = BOLT.iter().map(|&(px, py)| Pt { x: bx + px * bh / 10.0, y: by + py * bh / 10.0 }).collect();
        // A halo of background around the bolt: the bolt grown about its middle, knocked out below.
        let (cx, cy) = (bx + 3.0 * bh / 10.0, by + bh / 2.0);
        let halo: Vec<Pt> = bolt.iter().map(|p| Pt { x: cx + (p.x - cx) * 1.6, y: cy + (p.y - cy) * 1.3 }).collect();
        for layer in [&mut l.ink, &mut l.ok, &mut l.warn] {
            layer.erase(&halo, xf);
        }
        l.top.fill_clipped(&[bolt], 1.0, xf, Clip::NONE);
    }
}

const BOLT: [(f64, f64); 6] = [(3.6, 0.0), (0.4, 5.7), (2.8, 5.7), (2.2, 10.0), (5.6, 4.2), (3.2, 4.2)];

#[cfg(test)]
mod tests {
    use super::*;

    fn info(pods: Pods, connected: bool, light: bool) -> HeaderInfo {
        HeaderInfo {
            name: "AirPods Pro".into(),
            pods,
            connected,
            left: Some((80, false)),
            right: Some((20, true)),
            case: None,
            low_warning: 20,
            light,
        }
    }

    #[test]
    fn sizes_text_and_background() {
        for (scale, w, h) in [(1.0, 236, 102), (1.25, 295, 128), (1.5, 354, 153), (2.0, 472, 204)] {
            let hd = header(&info(Pods::OneOut { left: true }, true, false), scale);
            assert_eq!((hd.width, hd.height, hd.bgra.len() as i32), (w, h, w * h * 4), "{scale}");
        }
        let hd = header(&info(Pods::Open, true, false), 1.0);
        let t: Vec<&str> = hd.texts.iter().map(|t| t.text.as_str()).collect();
        assert_eq!(t, ["AirPods Pro", "80%", "\u{2014}", "20%"]);
        assert_eq!((hd.texts[2].color, hd.texts[3].color), (DARK.disabled, DARK.warn));
        assert_eq!(&hd.bgra[..4], &[0x2C, 0x2C, 0x2C, 0xFF]);
        assert_eq!(header(&info(Pods::Open, true, true), 1.0).background, 0xF9F9F9);
    }

    #[test]
    fn the_pictures_are_read_and_every_tone_is_known() {
        for svg in [BUD_LEFT, BUD_RIGHT, CASE_CLOSED, CASE_OPEN_BACK, CASE_OPEN_FRONT] {
            let tones = parse_tones(svg).unwrap();
            assert!(tones.len() > 5);
            for (name, contours) in tones {
                assert_ne!(tone(name, true, false).0, 0xFF00FF, "{name}");
                assert!(!contours.is_empty());
            }
        }
    }

    #[test]
    fn every_pose_draws_differently() {
        let all = [Pods::Closed, Pods::Open, Pods::OneOut { left: true }, Pods::OneOut { left: false }, Pods::Out];
        let pics: Vec<Vec<u8>> = all.iter().map(|&p| header(&info(p, true, false), 1.0).bgra).collect();
        for i in 0..pics.len() {
            for j in i + 1..pics.len() {
                assert_ne!(pics[i], pics[j], "{:?} {:?}", all[i], all[j]);
            }
        }
        assert_ne!(header(&info(Pods::Open, true, false), 1.0).bgra, header(&info(Pods::Open, false, false), 1.0).bgra);
    }
}
