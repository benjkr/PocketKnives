//! The glyphs: SVG parsing (straight-line `<path>`s only), the built-in artwork and the
//! simulator's override, and the left earbud's outline the glow runs along.
use super::sort::binary_search;
use crate::math::{abs, fmod, max, min, pow, sqrt};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicPtr, Ordering};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Pt {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Bounds {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl Bounds {
    pub const EMPTY: Bounds = Bounds { left: f64::INFINITY, top: f64::INFINITY, right: f64::NEG_INFINITY, bottom: f64::NEG_INFINITY };

    pub fn union(self, o: Bounds) -> Bounds {
        Bounds {
            left: min(self.left, o.left),
            top: min(self.top, o.top),
            right: max(self.right, o.right),
            bottom: max(self.bottom, o.bottom),
        }
    }
}

/// One `<path>` of a glyph (closed polygons), drawn at its own opacity (SVG `fill-opacity`) times
/// the caller's.
pub struct Layer {
    pub contours: Vec<Vec<Pt>>,
    pub opacity: f64,
}

/// A closed polyline with the distance along it at each point.
pub struct Polyline {
    pub points: Vec<Pt>,
    pub distance: Vec<f64>,
    pub length: f64,
}

/// A glyph's paths, their combined bounds, the optional `data-tracer` orbit (cx cy rx ry), and
/// its left-earbud outline (the glow's path; only the line glyph's is used).
pub struct Glyph {
    pub layers: Vec<Layer>,
    pub bounds: Bounds,
    pub tracer: Option<[f64; 4]>,
    pub outline: Option<Polyline>,
}

const FILL_SVG: &str = include_str!("../../../assets/airpods-pro-fill.svg");
const LINE_SVG: &str = include_str!("../../../assets/airpods-pro-line.svg");

// Each slot holds a leaked, never-freed glyph: a reference handed out stays valid for the
// process. Built-ins are parsed on first use; overrides (simulator only) replace the pointer.
static FILL_BUILTIN: AtomicPtr<Glyph> = AtomicPtr::new(core::ptr::null_mut());
static LINE_BUILTIN: AtomicPtr<Glyph> = AtomicPtr::new(core::ptr::null_mut());
static FILL_OVERRIDE: AtomicPtr<Glyph> = AtomicPtr::new(core::ptr::null_mut());
static LINE_OVERRIDE: AtomicPtr<Glyph> = AtomicPtr::new(core::ptr::null_mut());

fn slot(override_: &AtomicPtr<Glyph>, builtin: &'static AtomicPtr<Glyph>, svg: &str) -> &'static Glyph {
    let o = override_.load(Ordering::Acquire);
    if !o.is_null() {
        // SAFETY: only leaked boxes are ever stored, and none is freed.
        return unsafe { &*o };
    }
    let b = builtin.load(Ordering::Acquire);
    if !b.is_null() {
        // SAFETY: as above.
        return unsafe { &*b };
    }
    let parsed = parse_glyph(svg).expect("built-in glyph");
    let new = Box::into_raw(Box::new(parsed));
    match builtin.compare_exchange(core::ptr::null_mut(), new, Ordering::AcqRel, Ordering::Acquire) {
        // SAFETY: `new` is now stored and never freed.
        Ok(_) => unsafe { &*new },
        Err(existing) => {
            // Another thread parsed it first: ours was never shared.
            // SAFETY: `new` came from `Box::into_raw` above and nobody else saw it.
            drop(unsafe { Box::from_raw(new) });
            // SAFETY: as above.
            unsafe { &*existing }
        }
    }
}

pub fn fill() -> &'static Glyph {
    slot(&FILL_OVERRIDE, &FILL_BUILTIN, FILL_SVG)
}

pub fn line() -> &'static Glyph {
    slot(&LINE_OVERRIDE, &LINE_BUILTIN, LINE_SVG)
}

/// Replaces the override (`None`, or SVG that doesn't parse: the built-in glyph). The previous
/// override is leaked, not freed: a frame being drawn may still hold it.
pub fn set_override(fill_svg: Option<&str>, line_svg: Option<&str>) {
    let leak = |svg: Option<&str>| svg.and_then(parse_glyph).map_or(core::ptr::null_mut(), |g| Box::into_raw(Box::new(g)));
    FILL_OVERRIDE.store(leak(fill_svg), Ordering::Release);
    LINE_OVERRIDE.store(leak(line_svg), Ordering::Release);
}

// ------------------------------------------------------------------ SVG

/// The tags matching `<path\b[^>]*>`.
fn path_tags(svg: &str) -> impl Iterator<Item = &str> {
    let b = svg.as_bytes();
    let mut i = 0;
    core::iter::from_fn(move || {
        while let Some(off) = svg[i..].find("<path") {
            let start = i + off;
            let after = start + 5;
            i = after;
            // `\b`: the next character isn't a word character.
            if after < b.len() && (b[after].is_ascii_alphanumeric() || b[after] == b'_' || b[after] >= 0x80) {
                continue;
            }
            let end = svg[after..].find('>')? + after;
            i = end + 1;
            return Some(&svg[start..=end]);
        }
        None
    })
}

/// The first match of `\s<name>="([^"]+)"` in `text`: the value.
fn attr<'a>(text: &'a str, name: &str) -> Option<&'a str> {
    let b = text.as_bytes();
    for (i, &c) in b.iter().enumerate() {
        if !is_regex_space(c) {
            continue;
        }
        let rest = &text[i + 1..];
        let Some(rest) = rest.strip_prefix(name).and_then(|r| r.strip_prefix("=\"")) else { continue };
        let Some(close) = rest.find('"') else { continue };
        if close > 0 {
            return Some(&rest[..close]);
        }
    }
    None
}

/// .NET's `\s` for the ASCII range (the glyph files are ASCII).
fn is_regex_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\x0B' | b'\x0C' | b'\r')
}

/// `double.Parse(s, CultureInfo.InvariantCulture)` (surrounding white space allowed).
fn parse_number(s: &str) -> Option<f64> {
    crate::num::parse_f64(s.trim_matches(|c: char| c.is_ascii() && is_regex_space(c as u8)))
}

/// A glyph from SVG text; `None` where the C# threw (no `<path>`, a curve, a bad number).
pub fn parse_glyph(svg: &str) -> Option<Glyph> {
    let mut layers = Vec::new();
    let mut bounds = Bounds::EMPTY;
    for tag in path_tags(svg) {
        let contours = parse_path(attr(tag, "d").unwrap_or(""))?;
        let opacity = match attr(tag, "fill-opacity") {
            Some(v) => parse_number(v)?,
            None => 1.0,
        };
        for c in &contours {
            for p in c {
                bounds = bounds.union(Bounds { left: p.x, top: p.y, right: p.x, bottom: p.y });
            }
        }
        layers.push(Layer { contours, opacity });
    }
    if layers.is_empty() {
        return None;
    }
    let tracer = match attr(svg, "data-tracer") {
        Some(v) => {
            let mut vals = Vec::new();
            for part in v.split(' ').filter(|p| !p.is_empty()) {
                vals.push(parse_number(part)?);
            }
            <[f64; 4]>::try_from(vals.as_slice()).ok()
        }
        None => None,
    };
    let outline = outline_path(&layers);
    Some(Glyph { layers, bounds, tracer, outline })
}

/// SVG path data with straight segments only: M/L/H/V/Z, absolute or relative.
fn parse_path(d: &str) -> Option<Vec<Vec<Pt>>> {
    let b = d.as_bytes();
    let mut contours = Vec::new();
    let mut cur: Vec<Pt> = Vec::new();
    let (mut x, mut y, mut start_x, mut start_y) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let mut cmd = 0u8;
    let mut i = 0usize;

    fn close(cur: &mut Vec<Pt>, contours: &mut Vec<Vec<Pt>>) {
        if cur.len() >= 3 {
            contours.push(core::mem::take(cur));
        }
        cur.clear();
    }
    fn next_is_number(b: &[u8], i: &mut usize) -> bool {
        while *i < b.len() && (b[*i].is_ascii_whitespace() || b[*i] == b'\x0B' || b[*i] == b',') {
            *i += 1;
        }
        *i < b.len() && (b[*i].is_ascii_digit() || matches!(b[*i], b'-' | b'+' | b'.'))
    }
    fn number(d: &str, i: &mut usize) -> Option<f64> {
        let b = d.as_bytes();
        next_is_number(b, i);
        let s = *i;
        if *i < b.len() && matches!(b[*i], b'-' | b'+') {
            *i += 1;
        }
        while *i < b.len() && (b[*i].is_ascii_digit() || b[*i] == b'.') {
            *i += 1;
        }
        if *i < b.len() && matches!(b[*i], b'e' | b'E') {
            *i += 1;
            if *i < b.len() && matches!(b[*i], b'-' | b'+') {
                *i += 1;
            }
            while *i < b.len() && b[*i].is_ascii_digit() {
                *i += 1;
            }
        }
        crate::num::parse_f64(&d[s..*i])
    }

    loop {
        if !next_is_number(b, &mut i) {
            if i >= b.len() {
                break;
            }
            cmd = b[i];
            i += 1;
            if cmd == b'Z' || cmd == b'z' {
                close(&mut cur, &mut contours);
                x = start_x;
                y = start_y;
                continue;
            }
        }
        let rel = cmd.is_ascii_lowercase();
        match cmd.to_ascii_uppercase() {
            b'M' => {
                let nx = number(d, &mut i)?;
                let ny = number(d, &mut i)?;
                close(&mut cur, &mut contours);
                x = if rel { x + nx } else { nx };
                y = if rel { y + ny } else { ny };
                start_x = x;
                start_y = y;
                cur.push(Pt { x, y });
                cmd = if rel { b'l' } else { b'L' }; // further pairs are line-tos
            }
            b'L' => {
                let nx = number(d, &mut i)?;
                let ny = number(d, &mut i)?;
                x = if rel { x + nx } else { nx };
                y = if rel { y + ny } else { ny };
                cur.push(Pt { x, y });
            }
            b'H' => {
                let n = number(d, &mut i)?;
                x = if rel { x + n } else { n };
                cur.push(Pt { x, y });
            }
            b'V' => {
                let n = number(d, &mut i)?;
                y = if rel { y + n } else { n };
                cur.push(Pt { x, y });
            }
            // A curve or anything else: the tray glyphs must be straight-line polygons.
            _ => return None,
        }
    }
    close(&mut cur, &mut contours); // fills close open subpaths implicitly
    Some(contours)
}

// ------------------------------------------------------------------ the outline

fn signed_area(c: &[Pt]) -> f64 {
    let mut sum = 0.0;
    for k in 0..c.len() {
        let a = c[k];
        let b = c[(k + 1) % c.len()];
        sum += a.x * b.y - b.x * a.y;
    }
    sum / 2.0
}

/// The left earbud's outline as a closed polyline: the largest contour left of the centre (the
/// body and stem; the ear tip sits on it). Starts at its top, runs clockwise on screen.
fn outline_path(layers: &[Layer]) -> Option<Polyline> {
    let mut best: Option<&Vec<Pt>> = None;
    let mut best_area = 0.0;
    for layer in layers {
        for c in &layer.contours {
            if c.iter().map(|p| p.x).fold(f64::NEG_INFINITY, max) > 12.0 {
                continue;
            }
            let area = abs(signed_area(c));
            if area > best_area {
                best_area = area;
                best = Some(c);
            }
        }
    }
    let best = best?;
    let mut pts = best.clone();
    if signed_area(best) < 0.0 {
        pts.reverse(); // y points down: positive shoelace area = clockwise on screen
    }
    let mut top = 0;
    for k in 1..pts.len() {
        if pts[k].y < pts[top].y || (pts[k].y == pts[top].y && pts[k].x < pts[top].x) {
            top = k;
        }
    }
    let mut ring: Vec<Pt> = pts[top..].iter().chain(pts[..top].iter()).copied().collect();
    ring.push(pts[top]);
    let mut dist = alloc::vec![0.0; ring.len()];
    for k in 1..ring.len() {
        dist[k] = dist[k - 1] + sqrt(pow(ring[k].x - ring[k - 1].x, 2.0) + pow(ring[k].y - ring[k - 1].y, 2.0));
    }
    let length = dist[dist.len() - 1];
    Some(Polyline { points: ring, distance: dist, length })
}

/// The point `s` units along the closed outline (wraps around).
pub fn point_at(path: &Polyline, s: f64) -> Pt {
    let mut s = fmod(s, path.length);
    if s < 0.0 {
        s += path.length;
    }
    let mut i = binary_search(&path.distance, s);
    if i < 0 {
        i = !i - 1;
    }
    let i = i.clamp(0, path.points.len() as i32 - 2) as usize;
    let u = (s - path.distance[i]) / max(1e-9, path.distance[i + 1] - path.distance[i]);
    let a = path.points[i];
    let b = path.points[i + 1];
    Pt { x: a.x + (b.x - a.x) * u, y: a.y + (b.y - a.y) * u }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn built_in_glyphs_parse() {
        let f = parse_glyph(FILL_SVG).unwrap();
        let l = parse_glyph(LINE_SVG).unwrap();
        assert_eq!(f.layers.len(), 2);
        assert_eq!(l.layers.len(), 2);
        assert_eq!(l.layers[0].opacity, 0.85);
        assert_eq!(l.tracer, Some([6.333, 9.252, 4.533, 3.796]));
        assert!(f.tracer.is_none());
        assert!(l.outline.is_some());
        let b = f.bounds.union(l.bounds);
        assert!(b.left > 1.0 && b.left < 2.0 && b.right > 22.0 && b.right < 23.0);
    }

    #[test]
    fn path_commands() {
        let c = parse_path("M1 1 2 1 2 2Z m0 0 h1 v1 z M5,5L6,5").unwrap();
        assert_eq!(c.len(), 2); // the last subpath has only 2 points
        assert_eq!(c[1], alloc::vec![Pt { x: 1.0, y: 1.0 }, Pt { x: 2.0, y: 1.0 }, Pt { x: 2.0, y: 2.0 }]);
        assert!(parse_path("M0 0 C1 1 2 2 3 3").is_none());
        assert!(parse_glyph("<svg><paths d=\"M0 0 1 0 1 1\"/></svg>").is_none());
    }
}
