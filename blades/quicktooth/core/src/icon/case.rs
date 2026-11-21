//! Where the buds are, in the tray icon (docs/specs/icon-and-animation.md): an outlined charging
//! case with a hinged lid around the buds. Closed: the case alone. Lid open: the lid swings up,
//! the buds sit small inside. One bud out: the case flows down out of the icon while the buds stay
//! where they are at full size, the one still in the case dimmed. Both out: the plain glyph.
//! Every pose change eases over `CaseAnimSeconds`.
//!
//! Laid out on the 16-unit grid (whole units = whole pixels at 16 px), scaled to the icon size.
use super::glyph::Pt;
use super::raster::{Canvas, Clip, Xf};
use super::{color, ease_in_out, fill, glyph_transform, line, tune};
use crate::battery::Pods;
use crate::math::{PI, cos, max, min, sin};
use crate::tunables::T;
use alloc::vec::Vec;

/// A pose of the case and buds; the animation mixes two of them.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Pose {
    /// Left and right bud opacity.
    pub left: f64,
    pub right: f64,
    /// Bud size against the plain glyph (smaller inside the case).
    pub scale: f64,
    /// Case opacity.
    pub case: f64,
    /// How far the lid is open (0 closed, 1 open).
    pub open: f64,
    /// The case body's top (the hinge line) and bottom, and the lid's height above the hinge.
    pub body_top: f64,
    pub body_bottom: f64,
    pub lid_height: f64,
    /// The status light's and the body's fill opacity.
    pub led: f64,
    pub body_fill: f64,
}

const CLOSED: Pose =
    Pose { left: 0.0, right: 0.0, scale: 0.72, case: 1.0, open: 0.0, body_top: 6.8, body_bottom: 13.0, lid_height: 3.8, led: 0.6, body_fill: 0.0 };
const OPEN: Pose =
    Pose { left: 1.0, right: 1.0, scale: 0.72, case: 1.0, open: 1.0, body_top: 8.8, body_bottom: 14.6, lid_height: 6.6, led: 0.0, body_fill: 0.18 };
/// Both out: the case is below the icon, the buds are the plain glyph.
const OUT: Pose =
    Pose { left: 1.0, right: 1.0, scale: 1.0, case: 0.0, open: 1.0, body_top: 18.0, body_bottom: 23.8, lid_height: 6.6, led: 0.0, body_fill: 0.18 };
/// The bud still in the case, when the other one is out.
const IN_CASE_DIM: f64 = 0.32;

/// The pose that shows `pods`; `None` (not known) shows the plain glyph.
pub fn pose_of(pods: Option<Pods>) -> Pose {
    match pods {
        Some(Pods::Closed) => CLOSED,
        Some(Pods::Open) => OPEN,
        Some(Pods::OneOut { left: true }) => Pose { right: IN_CASE_DIM, ..OUT },
        Some(Pods::OneOut { left: false }) => Pose { left: IN_CASE_DIM, ..OUT },
        Some(Pods::Out) | None => OUT,
    }
}

/// Does `pods` show as the plain glyph (so the static icons, with the low-battery dot, apply)?
pub fn is_plain(pods: Option<Pods>) -> bool {
    pose_of(pods) == OUT
}

/// The pose `u` (0–1) of the way from `a` to `b`, eased. The case fades out late when it leaves
/// and in early when it comes back; buds appear only once the lid has started to open.
pub fn mix(a: &Pose, b: &Pose, u: f64) -> Pose {
    if u >= 1.0 {
        return *b;
    }
    let t = ease_in_out(clamp01(u));
    let m = |x: f64, y: f64| x + (y - x) * t;
    let mut p = Pose {
        left: m(a.left, b.left),
        right: m(a.right, b.right),
        scale: m(a.scale, b.scale),
        case: m(a.case, b.case),
        open: m(a.open, b.open),
        body_top: m(a.body_top, b.body_top),
        body_bottom: m(a.body_bottom, b.body_bottom),
        lid_height: m(a.lid_height, b.lid_height),
        led: m(a.led, b.led),
        body_fill: m(a.body_fill, b.body_fill),
    };
    if a.case == 1.0 && b.case == 0.0 {
        p.case = if t < 0.45 { 1.0 } else { max(0.0, 1.0 - (t - 0.45) / 0.55) };
    }
    if a.case == 0.0 && b.case == 1.0 {
        p.case = min(1.0, t / 0.5);
    }
    if a.open == 0.0 && b.open == 1.0 {
        let late = max(0.0, (t - 0.22) / 0.78);
        p.left = a.left + (b.left - a.left) * late;
        p.right = a.right + (b.right - a.right) * late;
    }
    p
}

fn clamp01(v: f64) -> f64 {
    max(0.0, min(1.0, v))
}

// The case's sides and corner radius on the 16-unit grid.
const X0: f64 = 1.5;
const X1: f64 = 14.5;
const RADIUS: f64 = 3.2;
/// How far the open ends of the lid and body shapes reach past the hinge line (clipped there).
const PAST: f64 = 6.0;

/// A rounded rectangle, clockwise; `r` is capped to fit.
pub(crate) fn rounded(x0: f64, y0: f64, x1: f64, y1: f64, r: f64) -> Vec<Pt> {
    const N: i32 = 8; // segments per corner
    let r = max(0.0, min(r, min((x1 - x0) / 2.0, (y1 - y0) / 2.0)));
    let mut pts = Vec::new();
    for (cx, cy, start) in [(x1 - r, y0 + r, -0.5 * PI), (x1 - r, y1 - r, 0.0), (x0 + r, y1 - r, 0.5 * PI), (x0 + r, y0 + r, PI)] {
        for k in 0..=N {
            let a = start + 0.5 * PI * k as f64 / N as f64;
            pts.push(Pt { x: cx + r * cos(a), y: cy + r * sin(a) });
        }
    }
    pts
}

/// The band `w` wide centred on a rounded rectangle's edge (outer clockwise, inner reversed).
pub(crate) fn ring(x0: f64, y0: f64, x1: f64, y1: f64, r: f64, w: f64) -> [Vec<Pt>; 2] {
    let h = w / 2.0;
    let mut inner = rounded(x0 + h, y0 + h, x1 - h, y1 - h, max(0.0, r - h));
    inner.reverse();
    [rounded(x0 - h, y0 - h, x1 + h, y1 + h, r + h), inner]
}

fn disc(cx: f64, cy: f64, r: f64) -> Vec<Pt> {
    const N: i32 = 24;
    (0..N).map(|k| Pt { x: cx + r * cos(2.0 * PI * k as f64 / N as f64), y: cy + r * sin(2.0 * PI * k as f64 / N as f64) }).collect()
}

/// One frame: `p` drawn in the taskbar's ink, with the connected (solid) or hollow buds.
pub fn render(p: &Pose, connected: bool, light_taskbar: bool, size: i32) -> Vec<u8> {
    let mut canvas = Canvas::new(size);
    let grid = Xf::new(size as f64 / 16.0, 0.0, 0.0);
    let all = Clip::NONE;
    let a = p.case;
    let lid_top = p.body_top - p.lid_height;
    let lid_r = min(RADIUS, max(0.6, p.lid_height * 0.72));

    // 1. The lid, hinged on the body's top corners: an arch over the hinge line.
    if a > 0.004 && lid_top < 16.5 {
        let above = all.intersect(-PAST, -PAST, 16.0 + PAST, p.body_top);
        if p.open > 0.01 {
            canvas.fill_clipped(&[rounded(X0, lid_top, X1, p.body_top + PAST, lid_r)], a * p.open * 0.09, grid, above);
        }
        let w = 1.2 - 0.25 * p.open;
        canvas.fill_clipped(&ring(X0, lid_top, X1, p.body_top + PAST, lid_r, w), a * (1.0 - 0.35 * p.open), grid, above);
    }

    // 2. The buds, where the plain glyph has them (smaller inside the case), between the lid and
    //    the hinge line while the case is there. Connected, they knock the lid's lines out first.
    let clip_bottom = if a > 0.01 { p.body_top } else { 20.0 };
    let clip_top = if p.open < 0.99 { max(-1.0, lid_top + 0.6) } else { -1.0 };
    if clip_bottom > clip_top && (p.left > 0.004 || p.right > 0.004) {
        let xf = glyph_transform(size, tune(T::Zoom) * p.scale);
        // The grid band [clip_top, clip_bottom + 0.4] in glyph units (the transform only scales and moves).
        let to_glyph = |y: f64| (y * size as f64 / 16.0 - xf.ty) / xf.s;
        let band = all.intersect(f64::NEG_INFINITY, to_glyph(clip_top), f64::INFINITY, to_glyph(clip_bottom + 0.4));
        let halves = [(p.left, band.intersect(f64::NEG_INFINITY, f64::NEG_INFINITY, 12.0, f64::INFINITY)), (p.right, band.intersect(12.0, f64::NEG_INFINITY, f64::INFINITY, f64::INFINITY))];
        if connected && a > 0.01 {
            for &(alpha, clip) in &halves {
                if alpha > 0.01 {
                    canvas.erase_glyph(fill(), alpha, xf, clip);
                }
            }
        }
        let glyph = if connected { fill() } else { line() };
        for &(alpha, clip) in &halves {
            if alpha > 0.004 {
                canvas.draw(glyph, alpha, xf, clip);
            }
        }
    }

    // 3. The body: a U below the hinge line, the rim along it, the status light.
    if a > 0.004 && p.body_top < 16.5 {
        let below = all.intersect(-PAST, p.body_top, 16.0 + PAST, 16.0 + PAST);
        if p.body_fill > 0.004 {
            canvas.fill_clipped(&[rounded(X0, p.body_top - PAST, X1, p.body_bottom, RADIUS)], a * p.body_fill, grid, below);
        }
        canvas.fill_clipped(&ring(X0, p.body_top - PAST, X1, p.body_bottom, RADIUS, 1.2), a, grid, below);
        let rim_w = 0.7 + 0.25 * p.open;
        let rim_y = p.body_top + rim_w * 0.4;
        let rim = [Pt { x: X0 - 0.5, y: rim_y - rim_w / 2.0 }, Pt { x: X1 + 0.5, y: rim_y - rim_w / 2.0 }, Pt { x: X1 + 0.5, y: rim_y + rim_w / 2.0 }, Pt { x: X0 - 0.5, y: rim_y + rim_w / 2.0 }];
        canvas.fill_clipped(&[rim], a * 0.9, grid, all);
        if p.led > 0.004 {
            canvas.fill_clipped(&[disc(8.0, p.body_top + 2.4, 0.55)], a * p.led, grid, all);
        }
    }
    canvas.to_bgra(color(light_taskbar))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::icon::{IconState, render_bgra};

    #[test]
    fn out_is_the_plain_icon_and_the_others_differ() {
        for light in [false, true] {
            for size in [16, 20, 24, 32] {
                let out = pose_of(Some(Pods::Out));
                assert_eq!(render(&out, true, light, size), render_bgra(IconState::Connected, light, size));
                assert_eq!(render(&out, false, light, size), render_bgra(IconState::Disconnected, light, size));
                for pods in [Pods::Closed, Pods::Open, Pods::OneOut { left: true }, Pods::OneOut { left: false }] {
                    assert_ne!(render(&pose_of(Some(pods)), true, light, size), render_bgra(IconState::Connected, light, size), "{pods:?}");
                }
            }
        }
        assert!(is_plain(None) && is_plain(Some(Pods::Out)) && !is_plain(Some(Pods::Open)));
    }

    #[test]
    fn a_change_ends_on_its_pose() {
        let (a, b) = (pose_of(Some(Pods::Closed)), pose_of(Some(Pods::OneOut { left: true })));
        assert_eq!(mix(&a, &b, 1.0), b);
        assert_eq!(mix(&a, &b, 0.0), a);
        assert_eq!(mix(&b, &a, 1.0), a);
    }
}
