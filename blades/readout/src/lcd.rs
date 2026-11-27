//! What the glass shows: seven-segment readouts, the 3x5 pixel font and the dot-matrix graphs.
//! Geometry is in design pixels (1 = one pixel at 100% scaling); the bar scales it.
use crate::model::{BURST_KBS, HOT, Series};
use crate::raster::{self, Path, Pt};
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

// ---- seven-segment readouts ----

/// A readout's digit geometry: cell width and height, stroke thickness, gap between cells, the %
/// sign's width and ring size, the unit letter's cell width.
pub struct Digits {
    pub w: f32,
    pub h: f32,
    pub t: f32,
    pub gap: f32,
    pub pw: f32,
    pub pr: f32,
    pub uw: f32,
}

/// The small readouts (metric values, panel figures).
pub const SMALL: Digits = Digits { w: 4.8, h: 9.0, t: 1.25, gap: 1.8, pw: 6.4, pr: 3.0, uw: 6.5 };
/// The large readouts (the time, a panel's headline value).
pub const LARGE: Digits = Digits { w: 7.4, h: 14.0, t: 2.0, gap: 2.4, pw: 9.6, pr: 4.6, uw: 6.5 };

/// The digits lean like a calculator's: skewed 9 degrees.
const TAN: f32 = 0.158_384_44;
/// How far a corner tip leans outwards, and the mitre gap between segments, in thicknesses.
const KO: f32 = 0.55;
const MITRE: f32 = 0.16;
/// The % sign: ring stroke and bar width, in digit heights.
const PCT_STROKE: f32 = 0.083;
const PCT_BAR: f32 = 0.1;

const SEGS: [&str; 10] = ["abcdef", "bc", "abged", "abgcd", "fgbc", "afgcd", "afgecd", "abc", "abcdefg", "abcdfg"];

/// A readout drawn: its lit and unlit (ghost) shapes, in its own box at the origin.
pub struct Readout {
    pub lit: Path,
    pub ghost: Path,
    pub w: f32,
}

/// A horizontal segment between nodes `xl` and `xr` on centre line `yc`; `k` is which way its tips lean.
fn hseg(xl: f32, xr: f32, yc: f32, k: f32, h: f32, d: f32) -> Vec<Pt> {
    let a = k.abs();
    let tl = xl - a * h + d;
    let tr = xr + a * h - d;
    let ty = yc + k * h;
    vec![(tl, ty), (tl + h * (1.0 + k), yc - h), (tr - h * (1.0 + k), yc - h), (tr, ty), (tr - h * (1.0 - k), yc + h), (tl + h * (1.0 - k), yc + h)]
}

/// A side segment: `o` is the digit's outside (-1 left, 1 right); `mid` 1 = its top end, 2 = its
/// bottom end meets the middle bar, where the outer half is cut square so a 1 reads as one stroke.
fn vseg(xc: f32, yt: f32, yb: f32, o: f32, mid: u8, h: f32, d: f32) -> Vec<Pt> {
    let tt = yt - KO * h + d;
    let tb = yb + KO * h - d;
    let (top, it) = if mid == 1 { ([(0.0, yt + d), (h, yt + d)], yt + d + h) } else { ([(KO * h, tt), (h, tt + h * (1.0 - KO))], tt + h * (1.0 + KO)) };
    let (bot, ib) = if mid == 2 { ([(h, yb - d), (0.0, yb - d)], yb - d - h) } else { ([(h, tb - h * (1.0 - KO)), (KO * h, tb)], tb - h * (1.0 + KO)) };
    let mut p: Vec<Pt> = top.iter().chain(bot.iter()).copied().collect();
    p.push((-h, ib));
    p.push((-h, it));
    p.into_iter().map(|(q, y)| (xc + o * q, y)).collect()
}

/// Makes a simple polygon clockwise, so shapes that touch unite instead of cancelling.
fn cw(mut p: Vec<Pt>) -> Vec<Pt> {
    let mut a = 0.0;
    for i in 0..p.len() {
        let (x0, y0) = p[i];
        let (x1, y1) = p[(i + 1) % p.len()];
        a += x0 * y1 - x1 * y0;
    }
    if a < 0.0 {
        p.reverse();
    }
    p
}

fn digit(lit: &mut Path, ghost: &mut Path, ch: Option<u8>, x: f32, g: &Digits) {
    let h = g.t / 2.0;
    let d = g.t * MITRE;
    let (l, r, t, m, b) = (x + h, x + g.w - h, h, g.h / 2.0, g.h - h);
    if ch == Some(1) {
        // Every digit has the same cell; a 1 is its two right-hand segments moved to the middle.
        let c = x + g.w / 2.0;
        lit.poly(cw(vseg(c, t, m, 1.0, 2, h, d)));
        lit.poly(cw(vseg(c, m, b, 1.0, 1, h, d)));
        return;
    }
    let on = ch.map_or("", |c| SEGS[c as usize]);
    let segs = [
        ('a', hseg(l, r, t, -KO, h, d)),
        ('g', hseg(l, r, m, 0.0, h, d)),
        ('d', hseg(l, r, b, KO, h, d)),
        ('f', vseg(l, t, m, -1.0, 2, h, d)),
        ('e', vseg(l, m, b, -1.0, 1, h, d)),
        ('b', vseg(r, t, m, 1.0, 2, h, d)),
        ('c', vseg(r, m, b, 1.0, 1, h, d)),
    ];
    for (name, p) in segs {
        if on.contains(name) { lit.poly(cw(p)) } else { ghost.poly(cw(p)) };
    }
}

/// The % sign on the glass: two small square rings and a bar.
fn percent(lit: &mut Path, x: f32, g: &Digits) {
    let (hh, w, r) = (g.h, g.pw, g.pr);
    let s = hh * PCT_STROKE;
    let bw = hh * PCT_BAR;
    let q = s / 2.0;
    raster::stroke_round_rect(lit, x + q, q, r - s, r - s, r * 0.22, s);
    raster::stroke_round_rect(lit, x + w - r + q, hh - r + q, r - s, r - s, r * 0.22, s);
    lit.poly(cw(vec![(x + w - bw, 0.0), (x + w, 0.0), (x + bw, hh), (x, hh)]));
}

/// The unit letters K, M and G, drawn as strokes in a cell `uw` wide on the readout's baseline.
fn unit(lit: &mut Path, x: f32, letter: u8, hh: f32) {
    let sw = 0.95;
    let base = hh - 0.25;
    let top = base - 5.25;
    let (t, b) = (top + sw / 2.0, base - sw / 2.0);
    let p = |pts: &[(f32, f32)]| -> Vec<Pt> { pts.iter().map(|&(px, py)| (x + px, py)).collect() };
    match letter {
        b'K' => {
            raster::stroke_polyline(lit, &p(&[(0.7, top), (0.7, base)]), sw);
            raster::stroke_polyline(lit, &p(&[(4.6, top), (1.1, 6.0 + (top - 3.5))]), sw);
            raster::stroke_polyline(lit, &p(&[(1.9, 5.3 + (top - 3.5)), (4.8, base)]), sw);
        }
        b'M' => raster::stroke_polyline(lit, &p(&[(0.6, base), (0.6, t), (3.0, b - 1.2), (5.4, t), (5.4, base)]), sw),
        _ => raster::stroke_polyline(lit, &p(&[(4.6, t + 0.6), (3.8, t), (1.6, t), (0.6, t + 1.0), (0.6, b - 0.9), (1.6, b), (3.8, b), (4.7, b - 0.8), (4.7, t + 2.9), (2.9, t + 2.9)]), sw),
    }
}

/// Where each template cell starts, and the template's width before the lean.
/// `8` digit, `:` colon, `.` decimal point (in the gap, takes no room), `%`, `^` the up or down
/// triangle, `U` the unit letter.
fn layout(tpl: &str, g: &Digits) -> (Vec<f32>, f32) {
    let mut x = 0.0;
    let mut xs = Vec::new();
    let mut prev = 0u8;
    for c in tpl.bytes() {
        if c != b'.' {
            if prev != 0 {
                x += if prev == b'^' || c == b'U' { 1.0 } else { g.gap };
            }
            prev = c;
        }
        xs.push(x);
        x += match c {
            b'8' => g.w,
            b':' => g.t,
            b'%' => g.pw,
            b'U' => g.uw,
            b'^' => 4.0,
            _ => 0.0,
        };
    }
    (xs, x)
}

/// `text`'s digits right-aligned in the template's digit cells (an empty cell is an unlit 8), its
/// `.` lit if it has one. `up` picks the triangle, `unit` the unit letter.
pub fn readout(text: &str, tpl: &str, g: &Digits, up: bool, unit_letter: u8) -> Readout {
    let (xs, raw) = layout(tpl, g);
    let lean = g.h * TAN;
    let w = libm::ceilf(raw + lean - 0.01);
    let digits: Vec<u8> = text.bytes().filter(u8::is_ascii_digit).map(|c| c - b'0').collect();
    let cells = tpl.bytes().filter(|&c| c == b'8').count();
    let mut k = digits.len() as isize - cells as isize;
    let has_dot = text.contains('.');
    let (mut lit, mut ghost) = (Path::new(), Path::new());
    let m = g.h / 2.0;
    let ds = g.t.max(1.3);
    let dp = g.t.min(g.gap - 0.6);
    let dot = |p: &mut Path, x: f32, yc: f32, s: f32| {
        p.poly(raster::round_rect(x, yc - s / 2.0, s, s, s * 0.22));
    };
    for (i, c) in tpl.bytes().enumerate() {
        let x = xs[i];
        match c {
            b'8' => {
                digit(&mut lit, &mut ghost, if k < 0 { None } else { Some(digits[k as usize]) }, x, g);
                k += 1;
            }
            b'.' => dot(if has_dot { &mut lit } else { &mut ghost }, x + (g.gap - dp) / 2.0, g.h - dp / 2.0, dp),
            b':' => {
                let y1 = libm::roundf(g.h * 0.3 - ds / 2.0) + ds / 2.0;
                dot(&mut lit, x + (g.t - ds) / 2.0, y1, ds);
                dot(&mut lit, x + (g.t - ds) / 2.0, g.h - y1, ds);
            }
            b'%' => percent(&mut lit, x, g),
            b'^' => {
                let tri = if up { vec![(x, m + 1.5), (x + 2.0, m - 1.5), (x + 4.0, m + 1.5)] } else { vec![(x, m - 1.5), (x + 2.0, m + 1.5), (x + 4.0, m - 1.5)] };
                lit.poly(cw(tri));
            }
            _ => unit(&mut lit, x + 0.2, unit_letter, g.h),
        }
    }
    // translate((W - raw - lean) / 2 + lean, 0) skewX(-9deg)
    let tx = (w - raw - lean) / 2.0 + lean;
    let skew = |(x, y): Pt| (x - TAN * y + tx, y);
    Readout { lit: lit.map(skew), ghost: ghost.map(skew), w }
}

/// At most three characters of number and a unit index: 0..999, then 1.0..9.9 and 10..999 of the
/// next unit (K, M, G for a rate in KB/s).
pub fn fmt3(x: f32) -> (String, usize) {
    let mut x = x.max(0.0);
    let mut u = 0;
    while x >= 999.5 && u < 2 {
        x /= 1024.0;
        u += 1;
    }
    if u > 0 && x < 9.95 { (fixed1(x), u) } else { (alloc::format!("{}", libm::roundf(x).min(999.0) as u32), u) }
}

pub fn fixed1(x: f32) -> String {
    let t = libm::roundf(x.max(0.0) * 10.0) as u32;
    alloc::format!("{}.{}", t / 10, t % 10)
}

pub const KMG: [u8; 3] = [b'K', b'M', b'G'];

/// One direction of a rate readout: triangle, three digit cells, unit letter.
pub fn rate(up: bool, kbs: f32) -> Readout {
    let (text, u) = fmt3(kbs);
    readout(&text, "^88.8U", &SMALL, up, KMG[u])
}

// ---- dot matrix ----

/// What a dot-matrix pixel shows.
pub const OFF: u8 = 0;
pub const LIT: u8 = 1;
pub const HOT_PX: u8 = 2;
pub const DIVIDER: u8 = 3;
/// The body of a bar or filled area (its top row is [`LIT`]).
pub const FILL: u8 = 4;
/// An unlit annunciator or cursor: there, in the ghost colour, so nothing shifts when it lights.
pub const GHOST: u8 = 5;

pub struct Bitmap {
    pub w: usize,
    pub h: usize,
    pub a: Vec<u8>,
}

impl Bitmap {
    pub fn new(w: usize, h: usize) -> Bitmap {
        Bitmap { w, h, a: vec![OFF; w * h] }
    }
    /// Sets a pixel; `soft` leaves one that's already set.
    pub fn set(&mut self, x: i32, y: i32, v: u8, soft: bool) {
        if x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < self.h {
            let i = y as usize * self.w + x as usize;
            if !(soft && self.a[i] != OFF) {
                self.a[i] = v;
            }
        }
    }
    pub fn ps(&mut self, x: i32, y: i32, v: u8) {
        self.set(x, y, v, false)
    }
    /// Copies `o` in at column `x0`.
    pub fn blit(&mut self, o: &Bitmap, x0: usize) {
        for y in 0..o.h.min(self.h) {
            for x in 0..o.w {
                if x0 + x < self.w {
                    self.a[y * self.w + x0 + x] = o.a[y * o.w + x];
                }
            }
        }
    }
}

/// 3x5 pixel font: five rows of bits per glyph, in hex. M, N and W are four wide.
fn glyph(c: u8) -> Option<&'static [u8; 5]> {
    Some(match c {
        b'A' => b"25755", b'B' => b"65656", b'C' => b"34443", b'D' => b"65556", b'E' => b"74647", b'F' => b"74644",
        b'G' => b"34553", b'H' => b"55755", b'I' => b"72227", b'J' => b"11152", b'K' => b"55655", b'L' => b"44447",
        b'M' => b"9FF99", b'N' => b"9DB99", b'O' => b"25552", b'P' => b"65644", b'Q' => b"25563", b'R' => b"65655",
        b'S' => b"34216", b'T' => b"72222", b'U' => b"55557", b'V' => b"55552", b'W' => b"99FF9", b'X' => b"55255",
        b'Y' => b"55222", b'Z' => b"71247", b'0' => b"75557", b'1' => b"26227", b'2' => b"61247", b'3' => b"61216",
        b'4' => b"55711", b'5' => b"74616", b'6' => b"34757", b'7' => b"71222", b'8' => b"75757", b'9' => b"75716",
        b'.' => b"00002", b'/' => b"11244", b':' => b"02020", b'-' => b"00700", b'%' => b"51245", b'*' => b"25200",
        // `~` stands for the rest of a name cut short.
        b'~' => b"00005", b'_' => b"00007", b'?' => b"61202", b'(' => b"24442", b')' => b"42224",
        _ => return None,
    })
}

fn wide(c: u8) -> bool {
    matches!(c, b'M' | b'N' | b'W')
}

/// The width of `s` in the pixel font, without the gap after its last glyph.
pub fn text_w(s: &str) -> usize {
    s.bytes().map(|c| if wide(c) { 5 } else { 4 }).sum::<usize>().saturating_sub(1)
}

/// `s` in the pixel font; `fixed`, when not 0, is the bitmap's width (for text that changes).
pub fn text(s: &str, hot: bool, fixed: usize) -> Bitmap {
    let w: usize = s.bytes().map(|c| if wide(c) { 5 } else { 4 }).sum();
    let mut b = Bitmap::new(if fixed > 0 { fixed } else { w }, 5);
    let mut x = 0i32;
    for c in s.bytes() {
        let gw = if wide(c) { 4 } else { 3 };
        if let Some(g) = glyph(c) {
            for (r, &hex) in g.iter().enumerate() {
                let bits = (hex as char).to_digit(16).unwrap_or(0);
                for col in 0..gw {
                    if bits >> (gw - 1 - col) & 1 != 0 {
                        b.ps(x + col, r as i32, if hot { HOT_PX } else { LIT });
                    }
                }
            }
        }
        x += gw + 1;
    }
    b
}

/// Fills a column from row `y0` down to `y1`: a lit top row over the fill, or all hot.
fn col_fill(b: &mut Bitmap, x: i32, y0: i32, y1: i32, hot: bool) {
    for y in y0..=y1 {
        b.ps(x, y, if hot { HOT_PX } else if y == y0 { LIT } else { FILL });
    }
}

fn round(v: f32) -> i32 {
    libm::floorf(v + 0.5) as i32
}

/// Bars `bw` wide every `pitch` columns from `x0`, scaled to `dh` rows; a bar over [`HOT`] is hot.
/// With fewer than `cap` values they sit at the right, the newest last.
pub fn bars(b: &mut Bitmap, vals: &[f32], bw: i32, pitch: i32, x0: i32, dh: i32, cap: usize) {
    let x0 = x0 + pitch * cap.saturating_sub(vals.len()) as i32;
    for (i, &v) in vals.iter().enumerate() {
        let bh = round(v.clamp(0.0, 100.0) / 100.0 * dh as f32).max(1);
        for j in 0..bw {
            col_fill(b, x0 + i as i32 * pitch + j, dh - bh, dh - 1, v > HOT);
        }
    }
}

/// A stepped line with its risers, as a calculator plots it; above the hot level the column is
/// filled down to that level. `cursor` 1 = a small arm at the newest point, 2 = a cross; `dots` =
/// a dotted line at the hot level. With fewer than `cap` values the line ends at the right.
pub fn line(b: &mut Bitmap, vals: &[f32], x0: i32, dh: i32, cursor: u8, dots: bool, cap: usize) {
    let ty = dh - 1 - round(HOT / 100.0 * (dh - 1) as f32);
    if dots {
        let mut x = x0;
        while x < x0 + cap as i32 {
            b.ps(x, ty, LIT);
            x += 3;
        }
    }
    let x0 = x0 + cap.saturating_sub(vals.len()) as i32;
    let mut prev: Option<i32> = None;
    for (i, &v) in vals.iter().enumerate() {
        let x = x0 + i as i32;
        let y = dh - 1 - round(v.clamp(0.0, 100.0) / 100.0 * (dh - 1) as f32);
        if y < ty {
            for yy in y..ty {
                b.ps(x, yy, HOT_PX);
            }
        } else {
            b.ps(x, y, LIT);
        }
        if let Some(p) = prev {
            for yy in p.min(y) + 1..p.max(y) {
                b.set(x, yy, if yy < ty { HOT_PX } else { LIT }, true);
            }
        }
        prev = Some(y);
    }
    let (Some(p), true) = (prev, !vals.is_empty()) else { return };
    let x = x0 + vals.len() as i32 - 1;
    if cursor == 1 {
        b.ps(x + 1, p, LIT);
        b.set(x, p - 1, LIT, true);
        b.set(x, p + 1, LIT, true);
    } else if cursor == 2 {
        for (dx, dy) in [(1, 0), (2, 0), (-1, 0), (-2, 0), (0, 1), (0, 2), (0, -1), (0, -2)] {
            b.ps(x + dx, p + dy, LIT);
        }
    }
}

/// 0..10 MB/s on a log scale, 0..=1.
fn lg(kbs: f32) -> f32 {
    (libm::logf(1.0 + kbs) / libm::logf(1.0 + 10240.0)).clamp(0.0, 1.0)
}

/// The network mirror: upload a line above the axis at row `mid`, download filled below it, on a
/// fixed log scale; past a burst the pixels are hot. Each sample is `step` columns wide, the
/// newest at the right.
pub fn net(b: &mut Bitmap, up: &Series, dn: &Series, n: usize, step: i32, x0: i32, mid: i32, amp: i32) {
    let (us, ds) = (up.last(n), dn.last(n));
    let t = libm::floorf(lg(BURST_KBS) * amp as f32) as i32;
    let axis = if x0 > 2 { x0 - 1 } else { 0 };
    let x0 = x0 + step * n.saturating_sub(us.len().min(ds.len())) as i32;
    let mut prev: Option<i32> = None;
    for i in 0..us.len().min(ds.len()) {
        let u = round(lg(us[i]) * amp as f32);
        let d = round(lg(ds[i]) * amp as f32);
        for s in 0..step {
            let x = x0 + i as i32 * step + s;
            for y in mid - u..mid - t {
                b.ps(x, y, HOT_PX);
            }
            if u > 0 && u <= t {
                b.ps(x, mid - u, LIT);
            }
            if d > 0 {
                col_fill(b, x, mid + 1, mid + d.min(t), false);
            }
            if d > 0 && d <= t {
                b.ps(x, mid + d, LIT);
            }
            for y in t + 1..=d {
                b.ps(x, mid + y, HOT_PX);
            }
        }
        if let Some(p) = prev {
            for y in p.min(u) + 1..p.max(u) {
                b.set(x0 + i as i32 * step, mid - y, if y > t { HOT_PX } else { LIT }, true);
            }
        }
        prev = Some(u);
    }
    for x in axis..b.w as i32 {
        b.ps(x, mid, LIT);
    }
}

/// Bars for `cores`, merged so no more than `max` show (each merged bar is its busiest core).
pub fn core_bars(cores: &[Series], max: usize) -> Vec<f32> {
    let mut size = 1;
    while cores.len().div_ceil(size) > max {
        size *= 2;
    }
    cores.chunks(size).map(|g| g.iter().map(Series::now).fold(0.0, f32::max)).collect()
}
