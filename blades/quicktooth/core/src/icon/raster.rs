//! The scanline rasterizer: nonzero fill, 16 sub-scanlines per pixel with exact horizontal
//! coverage, clips intersected geometrically with each span. One colour per canvas, so it only
//! keeps alpha. Every expression keeps the C# order of operations: the pixels match it bit for bit.
use super::glyph::{Glyph, Pt};
use super::sort::{compare_f64, introsort};
use crate::math::{abs, ceil, clamp, floor, max, min, round, sqrt};
use alloc::vec;
use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    /// `#RRGGBB`, at compile time (the colours are written once, in `tunables`).
    pub const fn hex(s: &str) -> Rgb {
        const fn nib(c: u8) -> u8 {
            match c {
                b'0'..=b'9' => c - b'0',
                b'a'..=b'f' => c - b'a' + 10,
                b'A'..=b'F' => c - b'A' + 10,
                _ => panic!("not a hex digit"),
            }
        }
        let b = s.as_bytes();
        assert!(b.len() == 7 && b[0] == b'#');
        Rgb { r: nib(b[1]) * 16 + nib(b[2]), g: nib(b[3]) * 16 + nib(b[4]), b: nib(b[5]) * 16 + nib(b[6]) }
    }
}

/// Glyph units to pixels: x' = S·(x + Dx) + Tx, y' = S·y + Ty (scale and translate only).
#[derive(Clone, Copy, Debug)]
pub struct Xf {
    pub s: f64,
    pub tx: f64,
    pub ty: f64,
    pub dx: f64,
}

impl Xf {
    pub fn new(s: f64, tx: f64, ty: f64) -> Xf {
        Xf { s, tx, ty, dx: 0.0 }
    }
    fn x(&self, x: f64) -> f64 {
        self.s * (x + self.dx) + self.tx
    }
    fn y(&self, y: f64) -> f64 {
        self.s * y + self.ty
    }
    pub fn offset(self, dx: f64) -> Xf {
        Xf { dx: self.dx + dx, ..self }
    }
}

/// A clip in glyph units: a rectangle, optionally intersected with the union of equal discs (one
/// disc for a round spot; a dense row of them for a band along a path).
#[derive(Clone, Copy, Debug)]
pub struct Clip<'a> {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub dots: Option<&'a [Pt]>,
    pub dot_r: f64,
}

impl<'a> Clip<'a> {
    pub const NONE: Clip<'static> =
        Clip { x0: f64::NEG_INFINITY, y0: f64::NEG_INFINITY, x1: f64::INFINITY, y1: f64::INFINITY, dots: None, dot_r: 0.0 };

    pub fn intersect(self, x0: f64, y0: f64, x1: f64, y1: f64) -> Clip<'a> {
        Clip { x0: max(self.x0, x0), y0: max(self.y0, y0), x1: min(self.x1, x1), y1: min(self.y1, y1), ..self }
    }

    pub fn with_dots<'b>(self, dots: &'b [Pt], r: f64) -> Clip<'b> {
        Clip { x0: self.x0, y0: self.y0, x1: self.x1, y1: self.y1, dots: Some(dots), dot_r: r }
    }
}

const SUB: i32 = 16; // sub-scanlines per pixel row

pub struct Canvas {
    /// Width and height in pixels.
    size: i32,
    height: i32,
    pub alpha: Vec<f64>,
    cov: Vec<f64>,
    xs: Vec<(f64, i32)>,
    clip: Vec<(f64, f64)>,
}

/// Brush alpha as an 8-bit value, as a 32-bit ARGB brush would hold it.
fn quantize(opacity: f64) -> f64 {
    round(255.0 * clamp(opacity, 0.0, 1.0)) / 255.0
}

impl Canvas {
    pub fn new(size: i32) -> Canvas {
        Canvas::rect(size, size)
    }

    /// A `width` × `height` canvas.
    pub fn rect(width: i32, height: i32) -> Canvas {
        let n = (width * height) as usize;
        Canvas { size: width, height, alpha: vec![0.0; n], cov: vec![0.0; n], xs: Vec::new(), clip: Vec::new() }
    }

    pub fn draw(&mut self, glyph: &Glyph, opacity: f64, xf: Xf, clip: Clip<'_>) {
        for layer in &glyph.layers {
            self.fill_polygons(&layer.contours, quantize(opacity * layer.opacity), xf, clip);
        }
    }

    /// Fills closed polygons (nonzero) at `opacity`.
    pub fn fill<C: AsRef<[Pt]>>(&mut self, contours: &[C], opacity: f64, xf: Xf) {
        self.fill_polygons(contours, quantize(opacity), xf, Clip::NONE);
    }

    /// Clears the canvas for reuse.
    pub fn clear(&mut self) {
        self.alpha.fill(0.0);
    }

    /// Fills closed polygons (nonzero) at `opacity`, inside `clip`.
    pub fn fill_clipped<C: AsRef<[Pt]>>(&mut self, contours: &[C], opacity: f64, xf: Xf, clip: Clip<'_>) {
        self.fill_polygons(contours, quantize(opacity), xf, clip);
    }

    /// Clears this canvas by `amount` where a glyph covers it, inside `clip` (a knock-out).
    pub fn erase_glyph(&mut self, glyph: &Glyph, amount: f64, xf: Xf, clip: Clip<'_>) {
        let mut mask = Canvas::rect(self.size, self.height);
        for layer in &glyph.layers {
            mask.fill_polygons(&layer.contours, 1.0, xf, clip);
        }
        let amount = quantize(amount);
        for (a, m) in self.alpha.iter_mut().zip(&mask.alpha) {
            *a *= 1.0 - min(*m, 1.0) * amount;
        }
    }

    /// Clears this canvas inside a polygon (a knock-out), anti-aliased at its edge.
    pub fn erase(&mut self, contour: &[Pt], xf: Xf) {
        let mut mask = Canvas::rect(self.size, self.height);
        mask.fill(&[contour], 1.0, xf);
        for (a, m) in self.alpha.iter_mut().zip(&mask.alpha) {
            *a *= 1.0 - min(*m, 1.0);
        }
    }

    fn fill_polygons<C: AsRef<[Pt]>>(&mut self, contours: &[C], alpha: f64, xf: Xf, clip: Clip<'_>) {
        if alpha <= 0.0 {
            return;
        }
        self.cov.fill(0.0);
        let size = self.size;
        // The clip in pixel space (the transform has no rotation, so rectangles and circles stay so).
        let (cx0, cx1, cy0, cy1) = (xf.x(clip.x0), xf.x(clip.x1), xf.y(clip.y0), xf.y(clip.y1));
        let dots: Option<Vec<(f64, f64)>> = clip.dots.map(|d| d.iter().map(|p| (xf.x(p.x), xf.y(p.y))).collect());
        let dr = clip.dot_r * xf.s;

        let mut edges: Vec<(f64, f64, f64, f64)> = Vec::new();
        let (mut min_y, mut max_y) = (f64::INFINITY, f64::NEG_INFINITY);
        for c in contours {
            let c = c.as_ref();
            for k in 0..c.len() {
                let a = c[k];
                let b = c[(k + 1) % c.len()];
                let (ax, ay, bx, by) = (xf.x(a.x), xf.y(a.y), xf.x(b.x), xf.y(b.y));
                if ay == by {
                    continue; // horizontal edges never cross a scanline
                }
                edges.push((ax, ay, bx, by));
                min_y = min(min_y, min(ay, by));
                max_y = max(max_y, max(ay, by));
            }
        }

        let row_from = 0.max(floor(max(min_y, cy0)) as i32);
        let row_to = (self.height - 1).min(ceil(min(max_y, cy1)) as i32);
        let mut py = row_from;
        while py <= row_to {
            for s in 0..SUB {
                let y = py as f64 + (s as f64 + 0.5) / SUB as f64;
                if y < cy0 || y >= cy1 {
                    continue;
                }
                let lo = max(0.0, cx0);
                let hi = min(size as f64, cx1);
                if lo >= hi {
                    continue;
                }
                // Where this sub-scanline is inside the clip: [lo, hi), or its overlap with the discs.
                self.clip.clear();
                match &dots {
                    None => self.clip.push((lo, hi)),
                    Some(dots) => {
                        for &(dx, dy0) in dots {
                            let dy = y - dy0;
                            if abs(dy) >= dr {
                                continue;
                            }
                            let hw = sqrt(dr * dr - dy * dy);
                            let a = max(lo, dx - hw);
                            let b = min(hi, dx + hw);
                            if a < b {
                                self.clip.push((a, b));
                            }
                        }
                        if self.clip.is_empty() {
                            continue;
                        }
                        introsort(&mut self.clip, &|p: &(f64, f64), q: &(f64, f64)| compare_f64(p.0, q.0));
                        let mut m = 0;
                        for k in 1..self.clip.len() {
                            if self.clip[k].0 <= self.clip[m].1 {
                                self.clip[m] = (self.clip[m].0, max(self.clip[m].1, self.clip[k].1));
                            } else {
                                m += 1;
                                self.clip[m] = self.clip[k];
                            }
                        }
                        self.clip.truncate(m + 1);
                    }
                }

                self.xs.clear();
                for &(x0, y0, x1, y1) in &edges {
                    let down = y1 > y0;
                    if if down { y < y0 || y >= y1 } else { y < y1 || y >= y0 } {
                        continue;
                    }
                    self.xs.push((x0 + (y - y0) * (x1 - x0) / (y1 - y0), if down { 1 } else { -1 }));
                }
                if self.xs.len() < 2 {
                    continue;
                }
                introsort(&mut self.xs, &|p: &(f64, i32), q: &(f64, i32)| compare_f64(p.0, q.0));

                let mut winding = 0;
                let mut span_start = 0.0;
                for k in 0..self.xs.len() {
                    let (x, dir) = self.xs[k];
                    let before = winding;
                    winding += dir;
                    if before == 0 && winding != 0 {
                        span_start = x;
                    } else if before != 0 && winding == 0 {
                        for c in 0..self.clip.len() {
                            let (cl, ch) = self.clip[c];
                            self.add_span(py, max(span_start, cl), min(x, ch));
                        }
                    }
                }
            }
            py += 1;
        }

        for (a, &cov) in self.alpha.iter_mut().zip(&self.cov) {
            let src = min(cov, 1.0) * alpha;
            if src > 0.0 {
                *a = src + *a * (1.0 - src);
            }
        }
    }

    /// Adds the exact horizontal coverage of [a, b) on one sub-scanline of row `py`.
    fn add_span(&mut self, py: i32, a: f64, b: f64) {
        if b <= a {
            return;
        }
        let row = py * self.size;
        let first = floor(a) as i32;
        let last = (self.size - 1).min(ceil(b) as i32 - 1);
        let mut px = 0.max(first);
        while px <= last {
            self.cov[(row + px) as usize] += (min(b, (px + 1) as f64) - max(a, px as f64)) / SUB as f64;
            px += 1;
        }
    }

    /// Straight-alpha BGRA, top-down rows.
    pub fn to_bgra(&self, color: Rgb) -> Vec<u8> {
        let mut px = vec![0u8; self.alpha.len() * 4];
        for (i, &al) in self.alpha.iter().enumerate() {
            let a = round(255.0 * clamp(al, 0.0, 1.0)) as u8;
            if a == 0 {
                continue;
            }
            px[i * 4] = color.b;
            px[i * 4 + 1] = color.g;
            px[i * 4 + 2] = color.r;
            px[i * 4 + 3] = a;
        }
        px
    }
}

/// Straight-alpha "source over" of `top` onto `bottom` (in place).
pub fn over(mut bottom: Vec<u8>, top: &[u8]) -> Vec<u8> {
    let mut i = 0;
    while i < bottom.len() {
        let sa = top[i + 3] as f64 / 255.0;
        if sa != 0.0 {
            let da = bottom[i + 3] as f64 / 255.0;
            let oa = sa + da * (1.0 - sa);
            for c in 0..3 {
                bottom[i + c] = round((top[i + c] as f64 * sa + bottom[i + c] as f64 * da * (1.0 - sa)) / oa) as u8;
            }
            bottom[i + 3] = round(255.0 * oa) as u8;
        }
        i += 4;
    }
    bottom
}
