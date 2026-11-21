//! The menu's header while the buds' place is known (docs/specs/tray.md): the device name,
//! centred; a line-art picture of the case with the buds where they are (nested in the
//! open case, or standing beside it once out); and a battery gauge with its level under each bud
//! and the case. In the menu's own colours (Windows 11, dark or light). The core draws the
//! picture; the host draws the text items on it in Segoe UI (it owns the fonts).
//!
//! Laid out in 96-DPI pixels and drawn at `scale`.
use crate::battery::Pods;
use crate::icon::case::{ring, rounded};
use crate::icon::glyph::{Glyph, Pt, parse_glyph};
use crate::icon::raster::{Canvas, Clip, Rgb, Xf, over};
use crate::math::{max, min};
use alloc::string::String;
use alloc::vec;
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
/// The picture's top.
const Y0: f64 = 23.0;
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

    // The case, centred: open while a bud is in it, closed and faint once both are out.
    let solid = info.connected;
    let (in_left, in_right) = match info.pods {
        Pods::Closed | Pods::Open => (true, true),
        Pods::OneOut { left } => (!left, left),
        Pods::Out => (false, false),
    };
    let open = matches!(info.pods, Pods::Open | Pods::OneOut { .. });
    let (x0, x1) = (MID - 30.0, MID + 30.0);
    if open {
        let (nest_h, top, hinge) = (30.0, Y0 + 6.0, Y0 + 26.0);
        let mut nested = Vec::new();
        if in_left {
            nested.push((MID - 2.0 - bud_width(true, nest_h), true));
        }
        if in_right {
            nested.push((MID + 2.0, false));
        }
        draw_case(&mut l.ink, xf, x0, x1, hinge, Y0 + 49.0, Y0 + 1.5, 1.0, 1.0, 0.08, |canvas| {
            for &(x, left) in &nested {
                draw_bud(canvas, scale, x, top, nest_h, left, solid, hinge + 0.6);
            }
        });
    } else {
        let faint = if info.pods == Pods::Out { 0.4 } else { 1.0 };
        draw_case(&mut l.ink, xf, x0, x1, Y0 + 24.0, Y0 + 49.0, Y0 + 10.0, 0.0, faint, 0.0, |_| {});
    }
    // Buds out of the case stand beside it.
    let out_h = 45.0;
    if !in_left {
        draw_bud(&mut l.ink, scale, SIDE_X - bud_width(true, out_h) / 2.0, Y0 + 2.0, out_h, true, solid, f64::INFINITY);
    }
    if !in_right {
        draw_bud(&mut l.ink, scale, WIDTH - SIDE_X - bud_width(false, out_h) / 2.0, Y0 + 2.0, out_h, false, solid, f64::INFINITY);
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

    let mut bgra: Vec<u8> = [(c.bg & 0xFF) as u8, ((c.bg >> 8) & 0xFF) as u8, (c.bg >> 16) as u8, 0xFF].repeat((w * h) as usize);
    for (layer, color) in [(&l.ink, c.ink), (&l.ok, c.ok), (&l.warn, c.warn), (&l.top, c.ink)] {
        bgra = over(bgra, &layer.to_bgra(rgb(color)));
    }
    Header { width: w, height: h, background: c.bg, bgra, texts }
}

fn rgb(c: u32) -> Rgb {
    Rgb { r: (c >> 16) as u8, g: ((c >> 8) & 0xFF) as u8, b: (c & 0xFF) as u8 }
}

fn round(v: f64) -> i32 {
    crate::math::round(v) as i32
}

/// The case as the tray's case icon draws it (`icon::case`), larger: a lid arch over the hinge
/// line, `inside` (the nested buds) drawn after the lid, then the body's U, the rim and, closed,
/// the status light.
#[allow(clippy::too_many_arguments)]
fn draw_case(c: &mut Canvas, xf: Xf, x0: f64, x1: f64, hinge: f64, bottom: f64, lid_top: f64, open: f64, alpha: f64, body_fill: f64, inside: impl FnOnce(&mut Canvas)) {
    const R: f64 = 12.0;
    const SW: f64 = 1.0; // the line drawing's weight at this size
    let lid_r = min(R, max(R * 0.19, (hinge - lid_top) * 0.72));
    let above = Clip::NONE.intersect(-1e9, -1e9, 1e9, hinge);
    let below = Clip::NONE.intersect(-1e9, hinge, 1e9, 1e9);
    if open > 0.0 {
        c.fill_clipped(&[rounded(x0, lid_top, x1, hinge + 2.0 * R, lid_r)], 0.09 * open * alpha, xf, above);
    }
    c.fill_clipped(&ring(x0, lid_top, x1, hinge + 2.0 * R, lid_r, SW * (1.0 - 0.2 * open)), alpha * (1.0 - 0.35 * open), xf, above);
    inside(c);
    if body_fill > 0.0 {
        c.fill_clipped(&[rounded(x0, hinge - 2.0 * R, x1, bottom, R)], body_fill * alpha, xf, below);
    }
    c.fill_clipped(&ring(x0, hinge - 2.0 * R, x1, bottom, R, SW), alpha, xf, below);
    c.fill_clipped(&[rect(x0 - SW / 2.0, hinge, x1 - x0 + SW, SW * 0.75)], 0.9 * alpha, xf, Clip::NONE);
    if open == 0.0 {
        c.fill_clipped(&[disc((x0 + x1) / 2.0, hinge + (bottom - hinge) * 0.4, SW * 0.5)], 0.6 * alpha, xf, Clip::NONE);
    }
}

fn rect(x: f64, y: f64, w: f64, h: f64) -> Vec<Pt> {
    vec![Pt { x, y }, Pt { x: x + w, y }, Pt { x: x + w, y: y + h }, Pt { x, y: y + h }]
}

fn disc(cx: f64, cy: f64, r: f64) -> Vec<Pt> {
    use crate::math::{PI, cos, sin};
    (0..24).map(|k| Pt { x: cx + r * cos(2.0 * PI * k as f64 / 24.0), y: cy + r * sin(2.0 * PI * k as f64 / 24.0) }).collect()
}

// ---------------------------------------------------------------- the bud

/// The buds as the original line drawing has them ("Airpods Pro" by Alex Martynov, SVG Repo
/// #324521, CC Attribution), flattened by `tools/airpods_art.py`: the left bud, ear tip to the
/// right, and the right bud, ear tip to the left.
const LEFT_ART: &str = include_str!("../../../assets/airpods-pro-art-left.svg");
const RIGHT_ART: &str = include_str!("../../../assets/airpods-pro-art-right.svg");

fn bud_art(left: bool) -> Option<Glyph> {
    parse_glyph(if left { LEFT_ART } else { RIGHT_ART })
}

/// A bud's width when it's `h` px tall.
fn bud_width(left: bool, h: f64) -> f64 {
    bud_art(left).map_or(0.0, |g| h * (g.bounds.right - g.bounds.left) / (g.bounds.bottom - g.bounds.top))
}

/// A bud `h` px tall with its top-left at (x, y), cut off below `bottom` (header px). Full ink when
/// connected, dim when not.
#[allow(clippy::too_many_arguments)]
fn draw_bud(c: &mut Canvas, scale: f64, x: f64, y: f64, h: f64, left: bool, solid: bool, bottom: f64) {
    let Some(g) = bud_art(left) else { return };
    let b = g.bounds;
    let s = h / (b.bottom - b.top);
    // Drawing units to pixels: scale·(x + s·(u − left)).
    let xf = Xf::new(scale * s, scale * (x - s * b.left), scale * (y - s * b.top));
    let clip = Clip::NONE.intersect(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::INFINITY, b.top + (bottom - y) / s);
    c.draw(&g, if solid { 1.0 } else { 0.45 }, xf, clip);
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
    fn the_buds_are_read() {
        for left in [true, false] {
            let g = bud_art(left).unwrap();
            assert!(g.layers.iter().map(|l| l.contours.len()).sum::<usize>() > 5);
            assert!((bud_width(left, 75.0) - 53.5).abs() < 1.0, "{}", bud_width(left, 75.0));
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
