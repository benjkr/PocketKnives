//! A small anti-aliased rasterizer into a premultiplied BGRA buffer (what `UpdateLayeredWindow`
//! takes). Shapes are polygons; coverage is exact signed area per pixel, so overlapping contours
//! of the same direction unite and a reversed contour cuts a hole. Dot-matrix pixels and pixel-font
//! glyphs are whole device pixels, drawn with [`Canvas::rect`].
use crate::theme::Rgba;
use alloc::vec;
use alloc::vec::Vec;

pub type Pt = (f32, f32);

/// Closed contours. A clockwise contour (on screen, y down) adds; an anticlockwise one subtracts.
#[derive(Default, Clone)]
pub struct Path {
    pub contours: Vec<Vec<Pt>>,
}

impl Path {
    pub fn new() -> Path {
        Path::default()
    }
    pub fn poly(&mut self, pts: Vec<Pt>) -> &mut Path {
        if pts.len() >= 3 {
            self.contours.push(pts);
        }
        self
    }
    /// Every point through `f`.
    pub fn map(&self, f: impl Fn(Pt) -> Pt) -> Path {
        Path { contours: self.contours.iter().map(|c| c.iter().map(|&p| f(p)).collect()).collect() }
    }
    pub fn offset(&self, dx: f32, dy: f32) -> Path {
        self.map(|(x, y)| (x + dx, y + dy))
    }
    fn bounds(&self) -> Option<(f32, f32, f32, f32)> {
        let mut b: Option<(f32, f32, f32, f32)> = None;
        for &(x, y) in self.contours.iter().flatten() {
            b = Some(match b {
                None => (x, y, x, y),
                Some((a, c, d, e)) => (a.min(x), c.min(y), d.max(x), e.max(y)),
            });
        }
        b
    }
}

/// What a shape is filled with: one colour, or a vertical gradient through stops `(y, colour)`
/// in device pixels, interpolated in straight alpha like SVG.
#[derive(Clone, Copy)]
pub enum Paint<'a> {
    Solid(Rgba),
    Vertical(&'a [(f32, Rgba)]),
}

impl Paint<'_> {
    fn at(&self, y: f32) -> Rgba {
        match self {
            Paint::Solid(c) => *c,
            Paint::Vertical(stops) => {
                let n = stops.len();
                if y <= stops[0].0 {
                    return stops[0].1;
                }
                for i in 1..n {
                    if y <= stops[i].0 {
                        let (y0, c0) = stops[i - 1];
                        let (y1, c1) = stops[i];
                        return c0.lerp(c1, if y1 > y0 { (y - y0) / (y1 - y0) } else { 1.0 });
                    }
                }
                stops[n - 1].1
            }
        }
    }
}

/// A coverage mask the size of the canvas, 0..=1 per pixel: a clip.
pub struct Mask {
    a: Vec<f32>,
    w: usize,
    h: usize,
}

impl Mask {
    /// Coverage at a device pixel; 0 outside the canvas.
    pub fn at(&self, x: i32, y: i32) -> f32 {
        if x < 0 || y < 0 || x as usize >= self.w || y as usize >= self.h { 0.0 } else { self.a[y as usize * self.w + x as usize] }
    }

    /// Blurs the mask: three box blurs of radius `r` each way, close to a Gaussian.
    pub fn blur(&mut self, r: usize) {
        if r == 0 {
            return;
        }
        let (w, h) = (self.w, self.h);
        let mut tmp = vec![0.0f32; w.max(h)];
        for _ in 0..3 {
            for y in 0..h {
                box1(&mut self.a[y * w..(y + 1) * w], 1, w, r, &mut tmp);
            }
            for x in 0..w {
                box1(&mut self.a[x..], w, h, r, &mut tmp);
            }
        }
    }
}

/// One box-blur pass over `n` values `stride` apart.
fn box1(a: &mut [f32], stride: usize, n: usize, r: usize, tmp: &mut [f32]) {
    let norm = 1.0 / (2 * r + 1) as f32;
    let mut s = 0.0;
    for i in 0..n {
        tmp[i] = a[i * stride];
    }
    for i in 0..=r.min(n - 1) {
        s += tmp[i];
    }
    for i in 0..n {
        a[i * stride] = s * norm;
        if i + r + 1 < n {
            s += tmp[i + r + 1];
        }
        if i >= r {
            s -= tmp[i - r];
        }
    }
}

pub struct Canvas {
    pub w: usize,
    pub h: usize,
    /// Premultiplied BGRA, one `u32` per pixel (`0xAARRGGBB`).
    pub px: Vec<u32>,
    acc: Vec<f32>,
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Canvas {
        Canvas { w, h, px: vec![0; w * h], acc: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.px.fill(0);
    }

    /// The path's coverage as a clip mask.
    pub fn mask(&mut self, path: &Path) -> Mask {
        let mut m = Mask { a: vec![0.0; self.w * self.h], w: self.w, h: self.h };
        let (w, h) = (self.w, self.h);
        self.cover(path, |x, y, c| m.a[y * w + x] = c, w, h);
        m
    }

    /// Fills `path` (device pixels) with `paint`, inside `clip` when given.
    pub fn fill(&mut self, path: &Path, paint: Paint, clip: Option<&Mask>) {
        let (w, h) = (self.w, self.h);
        let mut px = core::mem::take(&mut self.px);
        let mut row_y = usize::MAX;
        let mut col = Rgba::CLEAR;
        self.cover(
            path,
            |x, y, c| {
                if y != row_y {
                    row_y = y;
                    col = paint.at(y as f32 + 0.5);
                }
                let c = match clip {
                    Some(m) => c * m.a[y * w + x],
                    None => c,
                };
                blend(&mut px[y * w + x], col, c);
            },
            w,
            h,
        );
        self.px = px;
    }

    /// Fills whole device pixels `[x, x+w) x [y, y+h)`.
    pub fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, c: Rgba) {
        let x0 = x.max(0) as usize;
        let y0 = y.max(0) as usize;
        let x1 = ((x + w).max(0) as usize).min(self.w);
        let y1 = ((y + h).max(0) as usize).min(self.h);
        for yy in y0..y1 {
            for xx in x0..x1 {
                blend(&mut self.px[yy * self.w + xx], c, 1.0);
            }
        }
    }

    /// Blends one device pixel with coverage `cov`.
    pub fn dot(&mut self, x: i32, y: i32, c: Rgba, cov: f32) {
        if x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < self.h {
            blend(&mut self.px[y as usize * self.w + x as usize], c, cov);
        }
    }

    /// Calls `put(x, y, coverage)` for every pixel the path touches, inside a `w` x `h` canvas.
    fn cover(&mut self, path: &Path, mut put: impl FnMut(usize, usize, f32), w: usize, h: usize) {
        let Some((x0, y0, x1, y1)) = path.bounds() else { return };
        let bx = (libm::floorf(x0).max(0.0) as usize).min(w);
        let by = (libm::floorf(y0).max(0.0) as usize).min(h);
        let ex = (libm::ceilf(x1).max(0.0) as usize + 1).min(w);
        let ey = (libm::ceilf(y1).max(0.0) as usize).min(h);
        if bx >= ex || by >= ey {
            return;
        }
        let rw = ex - bx + 2;
        let rh = ey - by;
        self.acc.clear();
        self.acc.resize(rw * rh, 0.0);
        let (ox, oy) = (bx as f32, by as f32);
        for c in &path.contours {
            for i in 0..c.len() {
                let a = c[i];
                let b = c[(i + 1) % c.len()];
                line(&mut self.acc, rw, rh, ((a.0 - ox).clamp(0.0, (rw - 2) as f32), a.1 - oy), ((b.0 - ox).clamp(0.0, (rw - 2) as f32), b.1 - oy));
            }
        }
        for y in 0..rh {
            let mut s = 0.0f32;
            for x in 0..rw {
                s += self.acc[y * rw + x];
                let c = libm::fabsf(s).min(1.0);
                if c > 0.002 && bx + x < w {
                    put(bx + x, by + y, c);
                }
            }
        }
    }

    /// Un-premultiplied RGBA rows, for a PNG.
    pub fn rgba(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.w * self.h * 4);
        for &p in &self.px {
            let a = (p >> 24) as f32;
            let un = |v: u32| if a > 0.0 { ((v & 255) as f32 * 255.0 / a + 0.5).min(255.0) as u8 } else { 0 };
            out.extend_from_slice(&[un(p >> 16), un(p >> 8), un(p), (p >> 24) as u8]);
        }
        out
    }
}

/// Source-over of a straight colour at coverage `c` onto a premultiplied pixel.
fn blend(d: &mut u32, col: Rgba, c: f32) {
    let a = col.a * c;
    if a <= 0.0 {
        return;
    }
    let inv = 1.0 - a;
    let ch = |shift: u32, v: f32| -> u32 {
        let dv = ((*d >> shift) & 255) as f32;
        ((v * a * 255.0 + dv * inv) + 0.5).clamp(0.0, 255.0) as u32
    };
    let r = ch(16, col.r);
    let g = ch(8, col.g);
    let b = ch(0, col.b);
    let da = ((*d >> 24) & 255) as f32;
    let na = ((a * 255.0 + da * inv) + 0.5).clamp(0.0, 255.0) as u32;
    *d = (na << 24) | (r << 16) | (g << 8) | b;
}

/// Adds one edge's signed area to the accumulation buffer (font-rs's method): after a running sum
/// along each row, `|sum|` is the pixel's coverage.
fn line(acc: &mut [f32], w: usize, h: usize, p0: Pt, p1: Pt) {
    if (p0.1 - p1.1).abs() <= f32::EPSILON {
        return;
    }
    let (dir, p0, p1) = if p0.1 < p1.1 { (1.0, p0, p1) } else { (-1.0, p1, p0) };
    let dxdy = (p1.0 - p0.0) / (p1.1 - p0.1);
    let mut x = p0.0;
    if p0.1 < 0.0 {
        x -= p0.1 * dxdy;
    }
    let ys = p0.1.max(0.0) as usize;
    let ye = (libm::ceilf(p1.1).max(0.0) as usize).min(h);
    for y in ys..ye {
        let ls = y * w;
        let dy = ((y + 1) as f32).min(p1.1) - (y as f32).max(p0.1);
        let xn = x + dxdy * dy;
        let d = dy * dir;
        let (xa, xb) = if x < xn { (x, xn) } else { (xn, x) };
        let xaf = libm::floorf(xa);
        let xai = xaf as usize;
        let xbc = libm::ceilf(xb);
        let xbi = xbc as usize;
        if xbi <= xai + 1 {
            let xmf = 0.5 * (x + xn) - xaf;
            acc[ls + xai] += d - d * xmf;
            acc[ls + xai + 1] += d * xmf;
        } else {
            let s = 1.0 / (xb - xa);
            let x0f = xa - xaf;
            let a0 = 0.5 * s * (1.0 - x0f) * (1.0 - x0f);
            let x1f = xb - xbc + 1.0;
            let am = 0.5 * s * x1f * x1f;
            acc[ls + xai] += d * a0;
            if xbi == xai + 2 {
                acc[ls + xai + 1] += d * (1.0 - a0 - am);
            } else {
                let a1 = s * (1.5 - x0f);
                acc[ls + xai + 1] += d * (a1 - a0);
                for xi in xai + 2..xbi - 1 {
                    acc[ls + xi] += d * s;
                }
                let a2 = a1 + (xbi - xai - 3) as f32 * s;
                acc[ls + xbi - 1] += d * (1.0 - a2 - am);
            }
            acc[ls + xbi] += d * am;
        }
        x = xn;
    }
}

// ---- shapes, in any units (callers scale) ----

/// A rectangle, clockwise.
pub fn rect_path(x: f32, y: f32, w: f32, h: f32) -> Vec<Pt> {
    vec![(x, y), (x + w, y), (x + w, y + h), (x, y + h)]
}

/// A closed outline through corner points `(x, y, radius)`, each corner rounded with a quadratic
/// curve like the mock's `rpoly` (sides are upright or level).
pub fn round_poly(p: &[(f32, f32, f32)]) -> Vec<Pt> {
    let n = p.len();
    let mut out = Vec::new();
    for i in 0..n {
        let a = p[(i + n - 1) % n];
        let b = p[i];
        let c = p[(i + 1) % n];
        let r = b.2;
        let la = (a.0 - b.0).abs() + (a.1 - b.1).abs();
        let lc = (c.0 - b.0).abs() + (c.1 - b.1).abs();
        if r <= 0.0 || la == 0.0 || lc == 0.0 {
            out.push((b.0, b.1));
            continue;
        }
        let s = (b.0 + (a.0 - b.0) * r / la, b.1 + (a.1 - b.1) * r / la);
        let e = (b.0 + (c.0 - b.0) * r / lc, b.1 + (c.1 - b.1) * r / lc);
        const STEPS: usize = 6;
        for k in 0..=STEPS {
            let t = k as f32 / STEPS as f32;
            let u = 1.0 - t;
            out.push((u * u * s.0 + 2.0 * u * t * b.0 + t * t * e.0, u * u * s.1 + 2.0 * u * t * b.1 + t * t * e.1));
        }
    }
    out
}

/// A rounded rectangle, clockwise.
pub fn round_rect(x: f32, y: f32, w: f32, h: f32, r: f32) -> Vec<Pt> {
    let r = r.min(w / 2.0).min(h / 2.0).max(0.0);
    let mut out = Vec::new();
    let arc = |out: &mut Vec<Pt>, cx: f32, cy: f32, a0: f32| {
        for k in 0..=6 {
            let a = a0 + core::f32::consts::FRAC_PI_2 * k as f32 / 6.0;
            out.push((cx + r * libm::cosf(a), cy + r * libm::sinf(a)));
        }
    };
    use core::f32::consts::PI;
    arc(&mut out, x + w - r, y + r, -PI / 2.0);
    arc(&mut out, x + w - r, y + h - r, 0.0);
    arc(&mut out, x + r, y + h - r, PI / 2.0);
    arc(&mut out, x + r, y + r, PI);
    out
}

/// A circle, clockwise.
pub fn circle(cx: f32, cy: f32, r: f32) -> Vec<Pt> {
    (0..32).map(|k| {
        let a = core::f32::consts::TAU * k as f32 / 32.0;
        (cx + r * libm::cosf(a), cy + r * libm::sinf(a))
    }).collect()
}

/// Adds a stroke of width `sw` along an open polyline: butt caps, round joins.
pub fn stroke_polyline(path: &mut Path, pts: &[Pt], sw: f32) {
    let h = sw / 2.0;
    for i in 1..pts.len() {
        let (a, b) = (pts[i - 1], pts[i]);
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let len = libm::sqrtf(dx * dx + dy * dy);
        if len == 0.0 {
            continue;
        }
        let (nx, ny) = (-dy / len * h, dx / len * h);
        path.poly(vec![(a.0 - nx, a.1 - ny), (b.0 - nx, b.1 - ny), (b.0 + nx, b.1 + ny), (a.0 + nx, a.1 + ny)]);
        if i + 1 < pts.len() {
            path.poly(circle(b.0, b.1, h));
        }
    }
}

/// Adds the stroke of a rounded rectangle (centred on its outline) as a ring.
pub fn stroke_round_rect(path: &mut Path, x: f32, y: f32, w: f32, h: f32, r: f32, sw: f32) {
    let o = sw / 2.0;
    path.poly(round_rect(x - o, y - o, w + sw, h + sw, r + o));
    let mut inner = round_rect(x + o, y + o, w - sw, h - sw, (r - o).max(0.0));
    inner.reverse();
    path.poly(inner);
}

/// Adds the stroke of a circle as a ring.
pub fn stroke_circle(path: &mut Path, cx: f32, cy: f32, r: f32, sw: f32) {
    path.poly(circle(cx, cy, r + sw / 2.0));
    let mut inner = circle(cx, cy, (r - sw / 2.0).max(0.0));
    inner.reverse();
    path.poly(inner);
}

// ---- PNG, for previews: stored (uncompressed) deflate blocks ----

pub fn png(w: usize, h: usize, rgba: &[u8]) -> Vec<u8> {
    let mut raw = Vec::with_capacity((w * 4 + 1) * h);
    for row in rgba.chunks(w * 4) {
        raw.push(0);
        raw.extend_from_slice(row);
    }
    let mut z = vec![0x78, 0x01];
    let mut chunks = raw.chunks(65535).peekable();
    while let Some(c) = chunks.next() {
        z.push(u8::from(chunks.peek().is_none()));
        let n = c.len() as u16;
        z.extend_from_slice(&n.to_le_bytes());
        z.extend_from_slice(&(!n).to_le_bytes());
        z.extend_from_slice(c);
    }
    let (mut a, mut b) = (1u32, 0u32);
    for &v in &raw {
        a = (a + v as u32) % 65521;
        b = (b + a) % 65521;
    }
    z.extend_from_slice(&((b << 16) | a).to_be_bytes());
    let mut out = vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&(w as u32).to_be_bytes());
    ihdr.extend_from_slice(&(h as u32).to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
    chunk(&mut out, b"IHDR", &ihdr);
    chunk(&mut out, b"IDAT", &z);
    chunk(&mut out, b"IEND", &[]);
    out
}

fn chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    let start = out.len();
    out.extend_from_slice(kind);
    out.extend_from_slice(data);
    let mut crc = 0xFFFF_FFFFu32;
    for &byte in &out[start..] {
        crc ^= byte as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 { (crc >> 1) ^ 0xEDB8_8320 } else { crc >> 1 };
        }
    }
    out.extend_from_slice(&(!crc).to_be_bytes());
}
