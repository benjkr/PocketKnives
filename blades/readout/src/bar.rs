//! The bar and its hover panel, drawn from a [`Model`] in a theme's colours. All geometry is in
//! design pixels (the approved mock's CSS pixels); `k` device pixels make one, a whole number, so
//! the dot matrix and the engraving stay sharp.
use crate::lcd::{self, Bitmap, LARGE, Readout, SMALL};
use crate::model::{BURST_KBS, HOT, Model, Series};
use crate::procs::Proc;
use crate::raster::{self, Canvas, Mask, Paint, Path, Pt};
use crate::theme::{Palette, Rgba};
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

/// The bar's height, and the shadow drawn under it (outside the reserved space).
pub const BAR_H: i32 = 37;
pub const SHADOW_H: i32 = 4;
/// Space at the bar's left and right ends.
const PAD: f32 = 3.0;
/// Each tile's box: from row 2, 33 rows tall.
const TILE_Y: f32 = 2.0;
const TILE_H: f32 = 33.0;
/// The graph row: 13 pixels tall, 2 rows under the digit windows.
const GRAPH_H: usize = 13;

/// The hover panel: its size, the gap above it, and the room round it for its shadow.
pub const PANEL_W: i32 = 150;
pub const PANEL_H: i32 = 95;
pub const PANEL_GAP: i32 = 4;
pub const PANEL_MARGIN: i32 = 24;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Item {
    Cpu,
    Mem,
    Gpu,
    Dsk,
    Net,
    Clock,
}

impl Item {
    /// Width of the tile's digit window content.
    fn digits_w(self) -> f32 {
        match self {
            Item::Net => 68.0,
            Item::Clock => 44.0,
            _ => 28.0,
        }
    }
    fn name(self) -> &'static str {
        match self {
            Item::Cpu => "CPU",
            Item::Mem => "MEM",
            Item::Gpu => "GPU",
            Item::Dsk => "DSK",
            Item::Net => "NET",
            Item::Clock => "",
        }
    }
}

/// The local time the clock shows.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Time {
    pub hour: u16,
    pub minute: u16,
    /// 0 = Sunday.
    pub weekday: u16,
    pub day: u16,
    /// 1 = January.
    pub month: u16,
}

/// A tile: what it shows and its box along the bar.
#[derive(Clone, Copy)]
pub struct Tile {
    pub item: Item,
    pub x: f32,
    pub w: f32,
}

pub struct Layout {
    /// Device pixels per design pixel.
    pub k: i32,
    /// The bar's width in design pixels.
    pub width: f32,
    pub tiles: Vec<Tile>,
}

impl Layout {
    /// The metrics in `metrics` side by side from the left on one glass, the clock (when `clock`)
    /// at the right end.
    pub fn new(width: f32, k: i32, metrics: &[Item], clock: bool) -> Layout {
        let mut tiles = Vec::new();
        let mut x = PAD;
        for &item in metrics {
            let w = item.digits_w() + 22.0;
            tiles.push(Tile { item, x, w });
            x += w;
        }
        if clock {
            let w = Item::Clock.digits_w() + 22.0;
            tiles.push(Tile { item: Item::Clock, x: width - PAD - w, w });
        }
        Layout { k, width, tiles }
    }

    /// The tile under a point (design pixels from the bar's top left).
    pub fn hit(&self, x: f32, y: f32) -> Option<usize> {
        if !(TILE_Y..TILE_Y + TILE_H).contains(&y) {
            return None;
        }
        self.tiles.iter().position(|t| x >= t.x && x < t.x + t.w)
    }

    /// The device size of the bar's window: the bar and its shadow.
    pub fn size(&self) -> (usize, usize) {
        ((self.width * self.k as f32) as usize, ((BAR_H + SHADOW_H) * self.k) as usize)
    }

    /// Where tile `i`'s panel goes, its left edge in design pixels from the bar's left.
    pub fn panel_x(&self, i: usize) -> f32 {
        (self.tiles[i].x - 1.0).clamp(4.0, (self.width - PANEL_W as f32 - 4.0).max(4.0))
    }

    fn metrics(&self) -> Vec<Tile> {
        self.tiles.iter().copied().filter(|t| t.item != Item::Clock).collect()
    }
}

/// Draws with design-pixel geometry on a canvas of device pixels.
struct Pen<'a> {
    c: &'a mut Canvas,
    k: f32,
    pal: &'a Palette,
}

impl Pen<'_> {
    fn dev(&self, p: &Path) -> Path {
        let k = self.k;
        p.map(|(x, y)| (x * k, y * k))
    }
    fn fill(&mut self, p: &Path, col: Rgba) {
        if col.a > 0.0 {
            let d = self.dev(p);
            self.c.fill(&d, Paint::Solid(col), None);
        }
    }
    /// A vertical gradient from `y0` to `y1` (design pixels) through evenly placed or given stops.
    fn fill_grad(&mut self, p: &Path, stops: &[(f32, Rgba)]) {
        let k = self.k;
        let dev: Vec<(f32, Rgba)> = stops.iter().map(|&(y, c)| (y * k, c)).collect();
        let d = self.dev(p);
        self.c.fill(&d, Paint::Vertical(&dev), None);
    }
    fn mask(&mut self, p: &Path) -> Mask {
        let d = self.dev(p);
        self.c.mask(&d)
    }

    /// The faceplate: gradient, stipple and edge lines, inside `shape` spanning rows `y0..y1`.
    fn plate(&mut self, shape: &Path, y0: f32, y1: f32) {
        let pal = *self.pal;
        let h = y1 - y0;
        self.fill_grad(shape, &[(y0, pal.plate_top), (y0 + h * 0.22, pal.plate_upper), (y0 + h * 0.55, pal.plate_lower), (y1, pal.plate_bottom)]);
        let m = self.mask(shape);
        let k = self.k as i32;
        // The stipple: a soft dot of radius 0.4..0.6 at the middle and at the corners of every 2x2.
        let (light, dark) = stipple(k);
        let (ky0, ky1) = ((y0 * self.k) as i32, (y1 * self.k) as i32);
        let p = 2 * k;
        for y in ky0..ky1 {
            for x in 0..self.c.w as i32 {
                let cov = m.at(x, y);
                if cov > 0.0 {
                    let i = (y.rem_euclid(p) * p + x.rem_euclid(p)) as usize;
                    self.c.dot(x, y, pal.plate_dot_light, cov * light[i]);
                    self.c.dot(x, y, pal.plate_dot_dark, cov * dark[i]);
                    // inset 0 1px 0 and inset 0 -1px 0: one row along the top and the bottom
                    if m.at(x, y - k) == 0.0 || y < ky0 + k {
                        self.c.dot(x, y, pal.plate_edge_top, cov);
                    } else if m.at(x, y + k) == 0.0 || y >= ky1 - k {
                        self.c.dot(x, y, pal.plate_edge_bottom, cov);
                    }
                }
            }
        }
    }

    /// The glass's own details inside `gls`: the dark line round its inner edge and the shadow
    /// under its top edges.
    fn glass_detail(&mut self, gls: &Path) {
        let pal = *self.pal;
        let m = self.mask(gls);
        let k = self.k as i32;
        let d = self.dev(gls);
        let Some((x0, y0, x1, y1)) = bounds(&d) else { return };
        let edge_alpha = (0.5 * self.k).min(1.0);
        for y in y0..=y1 {
            for x in x0..=x1 {
                let c = m.at(x, y);
                if c <= 0.0 {
                    continue;
                }
                let inner = c.min(m.at(x - 1, y)).min(m.at(x + 1, y)).min(m.at(x, y - 1)).min(m.at(x, y + 1));
                self.c.dot(x, y, pal.glass_edge, (c - inner) * edge_alpha);
                self.c.dot(x, y, pal.glass_shadow, c * (1.0 - m.at(x, y - k)));
            }
        }
    }

    /// A surround with its glass: the lips, the surround's gradient, the glow under the glass,
    /// the glass's gradient and its details.
    fn surround(&mut self, sur: &Path, gls: &Path) {
        let pal = *self.pal;
        let (_, sy0, _, sy1) = path_bounds(sur);
        let (_, gy0, _, gy1) = path_bounds(gls);
        self.fill(&sur.offset(0.0, -0.5), pal.surround_lip_above);
        self.fill(&sur.offset(0.0, 0.5), pal.surround_lip_below);
        self.fill_grad(sur, &[(sy0, pal.surround_top), (sy1, pal.surround_bottom)]);
        self.fill(&gls.offset(0.0, 0.5), pal.glass_glow);
        self.fill_grad(gls, &[(gy0, pal.glass_top), (gy1, pal.glass_bottom)]);
        self.glass_detail(gls);
    }

    /// A readout at (x, y): its shadow, ghost and lit segments.
    fn readout(&mut self, r: &Readout, x: f32, y: f32) {
        let pal = *self.pal;
        self.fill(&r.lit.offset(x + 0.5, y + 0.5), pal.ink_shadow);
        self.fill(&r.ghost.offset(x, y), pal.ink_ghost);
        self.fill(&r.lit.offset(x, y), pal.ink);
    }

    /// A dot matrix's unlit pixels: `w` x `h` of them at (x, y), `sc` design pixels each, tinted,
    /// with a hairline along each one's top and left.
    fn grid(&mut self, x: f32, y: f32, w: usize, h: usize, sc: i32) {
        let pal = *self.pal;
        let k = self.k as i32;
        let (ox, oy, s) = ((x * self.k) as i32, (y * self.k) as i32, sc * k);
        self.c.rect(ox, oy, w as i32 * s, h as i32 * s, pal.grid_tint);
        let a = (0.2 * k as f32).min(1.0);
        for by in 0..h as i32 {
            for bx in 0..w as i32 {
                let (px, py) = (ox + bx * s, oy + by * s);
                for i in 0..s {
                    self.c.dot(px + i, py, pal.grid_line, a);
                    if i > 0 {
                        self.c.dot(px, py + i, pal.grid_line, a);
                    }
                }
            }
        }
    }

    /// A dot-matrix bitmap's lit pixels at (x, y), `sc` design pixels per bitmap pixel.
    fn bitmap(&mut self, b: &Bitmap, x: f32, y: f32, sc: i32) {
        let pal = *self.pal;
        let k = self.k as i32;
        let (ox, oy, s) = ((x * self.k) as i32, (y * self.k) as i32, sc * k);
        // drop-shadow(.5px .5px): every lit run, half a pixel down and right
        let mut sh = Path::new();
        for by in 0..b.h {
            let mut bx = 0;
            while bx < b.w {
                if b.a[by * b.w + bx] == lcd::OFF {
                    bx += 1;
                    continue;
                }
                let start = bx;
                while bx < b.w && b.a[by * b.w + bx] != lcd::OFF {
                    bx += 1;
                }
                let (fx, fy) = (x + (start as i32 * sc) as f32 + 0.5, y + (by as i32 * sc) as f32 + 0.5);
                sh.poly(raster::rect_path(fx, fy, ((bx - start) as i32 * sc) as f32, sc as f32));
            }
        }
        self.fill(&sh, pal.ink_shadow);
        for by in 0..b.h {
            for bx in 0..b.w {
                let col = match b.a[by * b.w + bx] {
                    lcd::LIT => pal.ink,
                    lcd::HOT_PX => pal.hot,
                    lcd::DIVIDER => pal.divider,
                    lcd::FILL => pal.fill,
                    lcd::GHOST => pal.ink_ghost,
                    _ => continue,
                };
                self.c.rect(ox + bx as i32 * s, oy + by as i32 * s, s, s, col);
            }
        }
    }

    /// An engraved icon in its 12x12 box at (x, y): the lips one pixel above and below, then the groove.
    fn icon(&mut self, item: Item, x: f32, y: f32, hot: bool) {
        let pal = *self.pal;
        let g = icon_path(item).offset(x, y);
        self.fill(&g.offset(0.0, -1.0), pal.icon_lip_above);
        self.fill(&g.offset(0.0, 1.0), pal.icon_lip_below);
        self.fill(&g, if hot { pal.icon_hot } else { pal.icon_groove });
    }

    /// Pixel text (or any bitmap's set pixels) engraved like the icons, its top left at (x, y).
    fn engrave(&mut self, b: &Bitmap, x: f32, y: f32) {
        let pal = *self.pal;
        let g = pixels(b, x, y);
        self.fill(&g.offset(0.0, -1.0), pal.icon_lip_above);
        self.fill(&g.offset(0.0, 1.0), pal.icon_lip_below);
        self.fill(&g, pal.icon_groove);
    }

    /// A key with `legend` centred on its cap, at (x, y), `w` x `h`: its shadow on the plate, its
    /// side wall a pixel below the cap, the cap's gradient and its lit top row. Pressed, the cap
    /// drops over its wall; its colours stay.
    fn key(&mut self, x: f32, y: f32, w: f32, h: f32, legend: &Bitmap, down: bool) {
        let pal = *self.pal;
        let y = if down { y + 1.0 } else { y };
        let mut cap = Path::new();
        cap.poly(raster::round_rect(x, y, w, h, 2.0));
        if down {
            self.fill(&cap.offset(0.0, 1.0), pal.key_shadow);
        } else {
            self.fill(&cap.offset(0.0, 2.0), pal.key_shadow);
            self.fill(&cap.offset(0.0, 1.0), pal.key_side);
        }
        self.fill_grad(&cap, &[(y, pal.key_top), (y + h, pal.key_bottom)]);
        // inset 0 1px 0: the cap's top row lit, under the rest of the cap
        self.fill(&cap, pal.key_edge);
        let mut body = Path::new();
        body.poly(raster::round_rect(x, y + 1.0, w, h - 1.0, 2.0));
        self.fill_grad(&body, &[(y, pal.key_top), (y + h, pal.key_bottom)]);
        let (lx, ly) = (x + libm::floorf((w - legend.w as f32) / 2.0), y + libm::floorf((h - legend.h as f32) / 2.0));
        self.fill(&pixels(legend, lx, ly), pal.key_legend);
    }
}

/// A bitmap's set pixels as one path of runs, one design pixel each, its top left at (x, y).
fn pixels(b: &Bitmap, x: f32, y: f32) -> Path {
    let mut p = Path::new();
    for by in 0..b.h {
        let mut bx = 0;
        while bx < b.w {
            if b.a[by * b.w + bx] == lcd::OFF {
                bx += 1;
                continue;
            }
            let start = bx;
            while bx < b.w && b.a[by * b.w + bx] != lcd::OFF {
                bx += 1;
            }
            p.poly(raster::rect_path(x + start as f32, y + by as f32, (bx - start) as f32, 1.0));
        }
    }
    p
}

/// The engraved glyphs on a 12px grid with 1px strokes on pixel centres.
fn icon_path(item: Item) -> Path {
    let mut p = Path::new();
    let line = |p: &mut Path, pts: &[Pt]| raster::stroke_polyline(p, pts, 1.0);
    match item {
        Item::Cpu => {
            raster::stroke_round_rect(&mut p, 3.5, 3.5, 5.0, 5.0, 0.6, 1.0);
            for &(x0, y0, x1, y1) in &[(4.5, 1.0, 4.5, 3.0), (7.5, 1.0, 7.5, 3.0), (4.5, 9.0, 4.5, 11.0), (7.5, 9.0, 7.5, 11.0), (1.0, 4.5, 3.0, 4.5), (1.0, 7.5, 3.0, 7.5), (9.0, 4.5, 11.0, 4.5), (9.0, 7.5, 11.0, 7.5)] {
                line(&mut p, &[(x0, y0), (x1, y1)]);
            }
        }
        Item::Mem => {
            raster::stroke_round_rect(&mut p, 0.5, 2.5, 11.0, 5.0, 0.6, 1.0);
            for x in [2.5, 4.5, 7.5, 9.5] {
                line(&mut p, &[(x, 8.0), (x, 11.0)]);
            }
        }
        Item::Gpu => {
            line(&mut p, &[(0.5, 1.0), (0.5, 11.0)]);
            line(&mut p, &[(1.0, 2.5), (10.5, 2.5), (10.5, 8.5), (1.0, 8.5)]);
            line(&mut p, &[(3.5, 9.0), (3.5, 10.5), (7.0, 10.5)]);
            raster::stroke_circle(&mut p, 6.5, 5.5, 1.5, 1.0);
        }
        Item::Dsk => {
            raster::stroke_round_rect(&mut p, 2.5, 0.5, 7.0, 10.0, 1.0, 1.0);
            raster::stroke_circle(&mut p, 6.0, 4.5, 2.0, 1.0);
            line(&mut p, &[(4.0, 8.5), (6.0, 8.5)]);
        }
        Item::Net => {
            raster::stroke_round_rect(&mut p, 3.5, 0.5, 4.0, 3.0, 0.0, 1.0);
            line(&mut p, &[(5.5, 4.0), (5.5, 6.5)]);
            line(&mut p, &[(2.5, 8.0), (2.5, 6.5), (8.5, 6.5), (8.5, 8.0)]);
            raster::stroke_round_rect(&mut p, 0.5, 8.5, 4.0, 2.0, 0.0, 1.0);
            raster::stroke_round_rect(&mut p, 6.5, 8.5, 4.0, 2.0, 0.0, 1.0);
        }
        Item::Clock => {
            raster::stroke_circle(&mut p, 5.5, 5.5, 5.0, 1.0);
            line(&mut p, &[(5.5, 2.5), (5.5, 5.5), (8.0, 5.5)]);
        }
    }
    p
}

/// The stipple's two dot patterns over one 2x2-design-pixel cell, `2k` device pixels square:
/// the coverage of a light dot at its middle and a dark dot at its corners.
fn stipple(k: i32) -> (Vec<f32>, Vec<f32>) {
    let p = (2 * k) as usize;
    let kf = k as f32;
    let mut light = vec![0.0; p * p];
    let mut dark = vec![0.0; p * p];
    const SS: usize = 6;
    let dot = |dx: f32, dy: f32| -> f32 {
        let r = libm::sqrtf(dx * dx + dy * dy) / kf;
        ((0.6 - r) / 0.2).clamp(0.0, 1.0)
    };
    for y in 0..p {
        for x in 0..p {
            let (mut l, mut d) = (0.0, 0.0);
            for sy in 0..SS {
                for sx in 0..SS {
                    let fx = x as f32 + (sx as f32 + 0.5) / SS as f32;
                    let fy = y as f32 + (sy as f32 + 0.5) / SS as f32;
                    l += dot(fx - kf, fy - kf);
                    let cx = if fx < kf { 0.0 } else { 2.0 * kf };
                    let cy = if fy < kf { 0.0 } else { 2.0 * kf };
                    d += dot(fx - cx, fy - cy);
                }
            }
            light[y * p + x] = l / (SS * SS) as f32;
            dark[y * p + x] = d / (SS * SS) as f32;
        }
    }
    (light, dark)
}

fn path_bounds(p: &Path) -> (f32, f32, f32, f32) {
    let mut b = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for &(x, y) in p.contours.iter().flatten() {
        b = (b.0.min(x), b.1.min(y), b.2.max(x), b.3.max(y));
    }
    b
}

fn bounds(p: &Path) -> Option<(i32, i32, i32, i32)> {
    if p.contours.is_empty() {
        return None;
    }
    let b = path_bounds(p);
    Some((libm::floorf(b.0) as i32 - 1, libm::floorf(b.1) as i32 - 1, libm::ceilf(b.2) as i32 + 1, libm::ceilf(b.3) as i32 + 1))
}

fn poly(pts: &[(f32, f32, f32)], ox: f32, oy: f32) -> Path {
    let mut p = Path::new();
    let moved: Vec<(f32, f32, f32)> = pts.iter().map(|&(x, y, r)| (x + ox, y + oy, r)).collect();
    p.poly(raster::round_poly(&moved));
    p
}

fn series(m: &Model, item: Item) -> &Series {
    match item {
        Item::Mem => &m.mem,
        Item::Gpu => &m.gpu,
        Item::Dsk => &m.dsk,
        _ => &m.cpu,
    }
}

fn value(m: &Model, item: Item) -> f32 {
    series(m, item).now()
}

fn hot(m: &Model, item: Item) -> bool {
    match item {
        Item::Net => m.up.now() >= BURST_KBS || m.dn.now() >= BURST_KBS,
        Item::Clock => false,
        _ => value(m, item) > HOT,
    }
}

fn pct_text(v: f32) -> String {
    format!("{}", libm::floorf(v.clamp(0.0, 100.0) + 0.5) as u32)
}

/// A tile's graph in the shared graph row: 11 rows of data, the axis, a row of ticks.
fn tile_graph(m: &Model, item: Item, gw: usize) -> Bitmap {
    let mut b = Bitmap::new(gw, GRAPH_H);
    let dh = GRAPH_H as i32 - 2;
    match item {
        Item::Net => {
            lcd::net(&mut b, &m.up, &m.dn, gw / 2, 2, 0, GRAPH_H as i32 / 2, GRAPH_H as i32 / 2);
            return b;
        }
        Item::Cpu => {
            let vals = lcd::core_bars(&m.cores, 14);
            if !vals.is_empty() {
                let n = vals.len() as i32;
                let bw = ((gw as i32 + 1) / n - 1).clamp(1, 3);
                let w = n * (bw + 1) - 1;
                lcd::bars(&mut b, &vals, bw, bw + 1, (gw as i32 - w) / 2, dh, vals.len());
            }
        }
        Item::Dsk => lcd::bars(&mut b, m.dsk.last(gw / 3), 2, 3, 1, dh, gw / 3),
        _ => lcd::line(&mut b, series(m, item).last(gw - 2), 0, dh, 1, false, gw - 2),
    }
    for x in 0..gw as i32 {
        b.ps(x, dh, lcd::LIT);
        if x % 4 == 1 {
            b.ps(x, dh + 1, lcd::LIT);
        }
    }
    b
}

const MONTHS: [&str; 12] = ["JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC"];
const DAYS: [&str; 7] = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];

/// Draws the bar (and the shadow under it) on `c`, which is [`Layout::size`]: [`background`]
/// then [`foreground`].
pub fn bar(c: &mut Canvas, l: &Layout, pal: &Palette, m: &Model, t: Time, hover: Option<usize>) {
    background(c, l, pal, hover);
    foreground(c, l, pal, m, t);
}

/// What doesn't change from second to second: the faceplate and its shadow, the hover tint, the
/// surrounds, the glass and the dot matrix's unlit pixels.
pub fn background(c: &mut Canvas, l: &Layout, pal: &Palette, hover: Option<usize>) {
    c.clear();
    let mut pen = Pen { c, k: l.k as f32, pal };
    let bh = BAR_H as f32;
    // the shadow under the bar: 0 1px 3px
    for (i, a) in [0.7f32, 0.42, 0.18, 0.05].iter().enumerate() {
        let mut row = Path::new();
        row.poly(raster::rect_path(0.0, bh + i as f32, l.width, 1.0));
        pen.fill(&row, pal.plate_shadow.with_alpha(*a));
    }
    let mut body = Path::new();
    body.poly(raster::rect_path(0.0, 0.0, l.width, bh));
    pen.plate(&body, 0.0, bh);

    if let Some(i) = hover {
        let t = l.tiles[i];
        let mut p = Path::new();
        p.poly(raster::round_rect(t.x, TILE_Y, t.w, TILE_H, 4.0));
        pen.fill(&p, pal.hover);
    }

    let metrics = l.metrics();
    if !metrics.is_empty() {
        let g = Strip::new(&metrics);
        let (ox, oy) = (g.x0 + 1.0, TILE_Y + 1.0);
        pen.surround(&poly(&g.sur, ox, oy), &poly(&g.gls, ox, oy));
        pen.grid(g.x0 + 4.0, GRAPH_Y, g.graph_w, GRAPH_H, 1);
    }
    if let Some(ct) = l.tiles.iter().find(|t| t.item == Item::Clock) {
        let w = Item::Clock.digits_w() + 20.0;
        let (ox, oy) = (ct.x + 1.0, TILE_Y + 1.0);
        let sur = poly(&[(14.0, 0.0, 2.0), (w, 0.0, 3.5), (w, 31.0, 3.5), (0.0, 31.0, 3.5), (0.0, 11.0, 3.5), (14.0, 11.0, 1.5)], ox, oy);
        let gls = poly(&[(16.0, 2.0, 1.5), (w - 2.0, 2.0, 1.5), (w - 2.0, 29.0, 1.5), (2.0, 29.0, 1.5), (2.0, 13.0, 1.5), (16.0, 13.0, 1.5)], ox, oy);
        pen.surround(&sur, &gls);
    }
}

/// What the readings change, over a [`background`]: the graphs, the readouts, the icons, the clock.
pub fn foreground(c: &mut Canvas, l: &Layout, pal: &Palette, m: &Model, t: Time) {
    let mut pen = Pen { c, k: l.k as f32, pal };
    let metrics = l.metrics();
    if !metrics.is_empty() {
        strip(&mut pen, &metrics, m);
    }
    if let Some(ct) = l.tiles.iter().find(|t| t.item == Item::Clock) {
        clock(&mut pen, ct.x, t);
    }
}

/// The graph row's top.
const GRAPH_Y: f32 = TILE_Y + 15.0;

/// The metric tiles' shared shapes: one surround with a tower per digit window, one glass under
/// all the graphs (corner points relative to the surround's origin).
struct Strip {
    x0: f32,
    xs: Vec<f32>,
    sur: Vec<(f32, f32, f32)>,
    gls: Vec<(f32, f32, f32)>,
    graph_w: usize,
}

impl Strip {
    fn new(tiles: &[Tile]) -> Strip {
        let x0 = tiles[0].x;
        let xs: Vec<f32> = tiles.iter().map(|t| t.x - x0).collect();
        let n = tiles.len();
        let t_end = tiles[n - 1].x + tiles[n - 1].w - x0 - 2.0;
        let (mut sur, mut gls) = (Vec::new(), Vec::new());
        for i in 0..n {
            let x1 = xs[i] + tiles[i].item.digits_w() + 20.0;
            sur.push((xs[i] + 14.0, 0.0, 2.0));
            sur.push((x1, 0.0, if i + 1 < n { 2.0 } else { 3.5 }));
            gls.push((xs[i] + 16.0, 2.0, 1.5));
            gls.push((x1 - 2.0, 2.0, 1.5));
            if i + 1 < n {
                sur.push((x1, 11.0, 1.5));
                sur.push((xs[i + 1] + 14.0, 11.0, 1.5));
                gls.push((x1 - 2.0, 13.0, 1.5));
                gls.push((xs[i + 1] + 16.0, 13.0, 1.5));
            }
        }
        sur.extend([(t_end, 31.0, 3.5), (0.0, 31.0, 3.5), (0.0, 11.0, 3.5), (14.0, 11.0, 1.5)]);
        gls.extend([(t_end - 2.0, 29.0, 1.5), (2.0, 29.0, 1.5), (2.0, 13.0, 1.5), (16.0, 13.0, 1.5)]);
        Strip { x0, xs, sur, gls, graph_w: (t_end - 6.0) as usize }
    }
}

/// The metric tiles' readings: the graph row, the icons and the readouts.
fn strip(pen: &mut Pen, tiles: &[Tile], m: &Model) {
    let g = Strip::new(tiles);
    let (xs, n) = (&g.xs, tiles.len());
    // each tile's graph in its own columns, a dotted divider between them
    let mut row = Bitmap::new(g.graph_w, GRAPH_H);
    for i in 0..n {
        let mut c0 = if i > 0 { xs[i] as usize - 2 } else { 0 };
        let c1 = if i + 1 < n { xs[i + 1] as usize - 5 } else { g.graph_w };
        if tiles[i].item == Item::Net {
            c0 += (c1 - c0) & 1; // two columns a sample
        }
        row.blit(&tile_graph(m, tiles[i].item, c1 - c0), c0);
        if i > 0 {
            for y in (0..GRAPH_H as i32).step_by(2) {
                row.ps(xs[i] as i32 - 4, y, lcd::DIVIDER);
            }
        }
    }
    pen.bitmap(&row, g.x0 + 4.0, GRAPH_Y, 1);

    for (i, t) in tiles.iter().enumerate() {
        // centred in the gap between towers (the first sits in the corner)
        let ix = if i == 0 { t.x + 2.0 } else { t.x + 1.0 };
        pen.icon(t.item, ix, TILE_Y, hot(m, t.item));
        let (dx, dy) = (t.x + 18.0, TILE_Y + 5.0);
        if t.item == Item::Net {
            let up = lcd::rate(true, m.up.now());
            pen.readout(&up, dx, dy);
            pen.readout(&lcd::rate(false, m.dn.now()), dx + up.w + 4.0, dy);
        } else {
            pen.readout(&lcd::readout(&pct_text(value(m, t.item)), "888%", &SMALL, false, 0), dx, dy);
        }
    }
}

/// The clock's readings: the date in the pixel font over the time.
fn clock(pen: &mut Pen, x: f32, t: Time) {
    pen.icon(Item::Clock, x + 2.0, TILE_Y, false);
    let date = format!("{} {}{} {}", DAYS[t.weekday as usize % 7], if t.day < 10 { " " } else { "" }, t.day, MONTHS[(t.month as usize + 11) % 12]);
    pen.bitmap(&lcd::text(&date, false, 44), x + 18.0, TILE_Y + 7.0, 1);
    let time = format!("{:02}{:02}", t.hour, t.minute);
    pen.readout(&lcd::readout(&time, "88:88", &LARGE, false, 0), x + 11.0, TILE_Y + 15.0);
}

// ---- the hover panel ----

/// One thing in the panel's footer row, laid out left to right with 2px gaps.
enum Foot {
    Text(Bitmap),
    Bars(Bitmap),
    Seg(Readout),
    Gap(f32),
    Spacer,
}

impl Foot {
    fn w(&self) -> f32 {
        match self {
            Foot::Text(b) | Foot::Bars(b) => b.w as f32,
            Foot::Seg(r) => r.w,
            Foot::Gap(w) => *w,
            Foot::Spacer => 0.0,
        }
    }
}

const GB: f64 = 1024.0 * 1024.0 * 1024.0;

/// An amount in GB for a `88.8` readout: one decimal below 100, whole above.
fn gb(bytes: u64) -> String {
    let v = (bytes as f64 / GB) as f32;
    if v < 99.95 { lcd::fixed1(v) } else { format!("{}", libm::roundf(v) as u32) }
}

fn size_rate(kbs: f64) -> Readout {
    let (text, u) = lcd::fmt3(kbs as f32);
    lcd::readout(&text, "88.8U", &SMALL, false, lcd::KMG[u])
}

/// The panel's height in design pixels: the screen, and for a metric with a process list, the
/// list's part of the body under it.
pub fn panel_h(item: Item) -> i32 {
    PANEL_H + if has_list(item) { LIST_H } else { 0 }
}

/// The panel canvas's size in device pixels: the panel and the room for its shadow.
pub fn panel_size(k: i32, item: Item) -> (usize, usize) {
    (((PANEL_W + 2 * PANEL_MARGIN) * k) as usize, ((panel_h(item) + 2 * PANEL_MARGIN) * k) as usize)
}

/// Draws `item`'s panel on `c` ([`panel_size`]), with its process list as `list` says; the
/// panel's top left is at (`PANEL_MARGIN`, `PANEL_MARGIN`): [`panel_back`] then [`panel_front`].
pub fn panel(c: &mut Canvas, k: i32, pal: &Palette, m: &Model, item: Item, list: &ListView) {
    panel_back(c, k, pal, item);
    panel_front(c, k, pal, m, item, list);
}

/// The screen's glass and where its content goes: (x, y, width) of the content.
fn screen_box() -> (f32, f32, f32, f32, f32, f32, f32) {
    let (px, py) = (PANEL_MARGIN as f32, PANEL_MARGIN as f32);
    let (gx, gy, gw, gh) = (px + 5.0, py + 5.0, PANEL_W as f32 - 10.0, PANEL_H as f32 - 10.0);
    (gx, gy, gw, gh, gx + 4.0, gy + 4.0, gw - 8.0)
}

/// What of `item`'s panel never changes, drawn over the whole of `c`: its shadow, the plastic and
/// what's engraved in it, the windows' rings and glass, the graph's grid and axis labels.
pub fn panel_back(c: &mut Canvas, k: i32, pal: &Palette, item: Item) {
    c.clear();
    let mut pen = Pen { c, k: k as f32, pal };
    let (px, py) = (PANEL_MARGIN as f32, PANEL_MARGIN as f32);
    let (pw, ph) = (PANEL_W as f32, panel_h(item) as f32);

    // 0 6px 18px shadow
    let mut sh = Path::new();
    sh.poly(raster::round_rect(px, py + 6.0, pw, ph, 5.0));
    let mut sm = pen.mask(&sh);
    sm.blur((3 * k) as usize);
    for y in 0..pen.c.h as i32 {
        for x in 0..pen.c.w as i32 {
            let a = sm.at(x, y);
            if a > 0.003 {
                pen.c.dot(x, y, pal.panel_shadow, a);
            }
        }
    }
    let mut body = Path::new();
    body.poly(raster::round_rect(px, py, pw, ph, 5.0));
    pen.plate(&body, py, py + ph);

    if has_list(item) {
        list_back(&mut pen, px, py, item);
    }

    // the screen, in its ring
    let (gx, gy, gw, gh, cx, cy, cw) = screen_box();
    let rr = |x: f32, y: f32, w: f32, h: f32, r: f32| {
        let mut p = Path::new();
        p.poly(raster::round_rect(x, y, w, h, r));
        p
    };
    pen.fill(&rr(gx, gy + 2.5, gw, gh, 2.0), pal.panel_ring_lip);
    pen.fill(&rr(gx - 2.0, gy - 2.0, gw + 4.0, gh + 4.0, 4.0), pal.panel_ring);
    let gls = rr(gx, gy, gw, gh, 2.0);
    pen.fill_grad(&gls, &[(gy, pal.glass_top), (gy + gh, pal.glass_bottom)]);
    pen.glass_detail(&gls);
    let net = item == Item::Net;
    pen.grid(cx, cy + 17.0, 66, 20, 2);

    // the axis labels
    let (a0, a1, a2) = (lcd::text("60S", false, 0), lcd::text(if net { "1M/S" } else { "70%" }, false, 0), lcd::text("NOW", false, 0));
    let free = cw - (a0.w + a1.w + a2.w) as f32;
    let ay = cy + 59.0;
    pen.bitmap(&a0, cx, ay, 1);
    pen.bitmap(&a1, cx + a0.w as f32 + libm::floorf(free / 2.0), ay, 1);
    pen.bitmap(&a2, cx + cw - a2.w as f32, ay, 1);
}

/// What of `item`'s panel changes, over its [`panel_back`]: the headline, the graph, the footer,
/// the process list and its keys.
pub fn panel_front(c: &mut Canvas, k: i32, pal: &Palette, m: &Model, item: Item, list: &ListView) {
    let mut pen = Pen { c, k: k as f32, pal };
    let (px, py) = (PANEL_MARGIN as f32, PANEL_MARGIN as f32);
    let (_, _, _, _, cx, cy, cw) = screen_box();
    let net = item == Item::Net;
    let val = value(m, item);
    // headline: the name, scaled 2x, and the value
    let name = lcd::text(item.name(), !net && val > HOT, 0);
    pen.bitmap(&name, cx, cy + 4.0, 2);
    if net {
        let b = lcd::text("LOG", false, 0);
        pen.bitmap(&b, cx + cw - b.w as f32, cy + 9.0, 1);
    } else {
        let r = lcd::readout(&pct_text(val), "888%", &LARGE, false, 0);
        pen.readout(&r, cx + cw - r.w, cy);
    }

    // the last minute, 2x, with axes
    let mut b = Bitmap::new(66, 20);
    match item {
        Item::Net => lcd::net(&mut b, &m.up, &m.dn, 60, 1, 3, 9, 9),
        Item::Dsk => lcd::bars(&mut b, m.dsk.last(60), 1, 1, 3, 18, 60),
        _ => lcd::line(&mut b, series(m, item).last(60), 3, 18, 2, true, 60),
    }
    for y in 0..19 {
        b.ps(1, y, lcd::LIT);
        if y % 3 == 0 {
            b.ps(0, y, lcd::LIT);
        }
    }
    if !net {
        for x in 1..66 {
            b.ps(x, 18, lcd::LIT);
            if x % 5 == 2 {
                b.ps(x, 19, lcd::LIT);
            }
        }
    }
    pen.bitmap(&b, cx, cy + 17.0, 2);

    // the footer
    let s = &m.last;
    let foot: Vec<Foot> = match item {
        Item::Cpu => {
            let vals = lcd::core_bars(&m.cores, 24);
            let pitch = if vals.len() <= 8 { 4 } else { 3 };
            let mut cb = Bitmap::new((vals.len() * pitch).saturating_sub(1).max(1), 7);
            lcd::bars(&mut cb, &vals, pitch as i32 - 1, pitch as i32, 0, 7, vals.len());
            let ghz = if s.cpu_mhz > 0.0 { lcd::fixed1(s.cpu_mhz / 1000.0) } else { String::new() };
            vec![Foot::Bars(cb), Foot::Spacer, Foot::Seg(lcd::readout(&ghz, "8.8", &SMALL, false, 0)), Foot::Text(lcd::text("GHZ", false, 0))]
        }
        Item::Mem => vec![
            Foot::Text(lcd::text("USED", false, 0)),
            Foot::Spacer,
            Foot::Seg(lcd::readout(&gb(s.mem_used), "88.8", &SMALL, false, 0)),
            Foot::Text(lcd::text(&format!("/{}GB", libm::round(s.mem_total as f64 / GB) as u32), false, 0)),
        ],
        Item::Gpu => {
            let mut f = vec![Foot::Text(lcd::text("MEM", false, 0))];
            if s.gpu_mem_total > 0 {
                let tpl = if s.gpu_mem_total as f64 >= 9.95 * GB { "88.8" } else { "8.8" };
                f.push(Foot::Seg(lcd::readout(&gb(s.gpu_mem_used), tpl, &SMALL, false, 0)));
                f.push(Foot::Text(lcd::text(&format!("/{}GB", libm::round(s.gpu_mem_total as f64 / GB) as u32), false, 0)));
            } else {
                f.push(Foot::Seg(lcd::readout("", "8.8", &SMALL, false, 0)));
            }
            f.push(Foot::Spacer);
            f
        }
        Item::Dsk => vec![
            Foot::Text(lcd::text("RD", false, 0)),
            Foot::Seg(size_rate(s.disk_read / 1024.0)),
            Foot::Spacer,
            Foot::Text(lcd::text("WR", false, 0)),
            Foot::Seg(size_rate(s.disk_write / 1024.0)),
        ],
        _ => vec![
            Foot::Text(lcd::text("NOW", false, 0)),
            Foot::Spacer,
            Foot::Seg(lcd::rate(true, m.up.now())),
            Foot::Gap(4.0),
            Foot::Seg(lcd::rate(false, m.dn.now())),
        ],
    };
    let fixed: f32 = foot.iter().map(Foot::w).sum::<f32>() + 2.0 * (foot.len() as f32 - 1.0);
    let spacers = foot.iter().filter(|f| matches!(f, Foot::Spacer)).count().max(1) as f32;
    let sp = libm::floorf(((cw - fixed) / spacers).max(0.0));
    let (mut x, fy) = (cx, cy + 68.0);
    for f in &foot {
        match f {
            Foot::Text(b) => pen.bitmap(b, x, fy + 3.0, 1),
            Foot::Bars(b) => pen.bitmap(b, x, fy + 1.0, 1),
            Foot::Seg(r) => pen.readout(r, x, fy),
            _ => {}
        }
        x += if matches!(f, Foot::Spacer) { sp } else { f.w() } + 2.0;
    }
    if has_list(item) {
        list_front(&mut pen, px, py, m, item, list);
    }
}

// ---- the panel's process list ----
// What can change is on glass; what is fixed (the heading, the row numbers, the scale, the
// footer) is engraved in the plastic; a key only travels when pressed. Design "P4", X twice.

/// The rows in a process list.
pub const LIST_ROWS: usize = 10;
/// The list's part of the body under the screen: the parting line, the heading, the list window,
/// its scale and the footer.
const LIST_H: i32 = 147;
/// The list window's glass (from the panel's top left) and its bitmap's width.
const LW_X: f32 = 15.0;
const LW_Y: f32 = 114.0;
const LW_W: f32 = 113.0;
const LW_H: f32 = 100.0;
const LB_W: usize = 109;
/// The first row's text on the bitmap, and the row pitch.
const ROW_Y: i32 = 12;
const ROW_PITCH: i32 = 9;
/// The keys: left edge, first key's top, size.
const KEY_X: f32 = 134.0;
const KEY_Y: f32 = 125.0;
const KEY_W: f32 = 11.0;
const KEY_H: f32 = 7.0;
/// Characters of a name; the bar's left and width; the value's right edge (bitmap columns).
const NAME_CHARS: usize = 11;
const BAR_X: i32 = 50;
const BAR_W: i32 = 35;
const VAL_R: i32 = 108;
/// Pips counting down a selection's seconds.
pub const SEL_PIPS: u8 = 5;

/// Whether `item`'s panel lists processes (the network's can't: Windows doesn't count traffic
/// per process without administrator rights).
pub fn has_list(item: Item) -> bool {
    matches!(item, Item::Cpu | Item::Mem | Item::Gpu | Item::Dsk)
}

/// What the list shows besides the readings.
#[derive(Clone, Default)]
pub struct ListView {
    /// The programs on the rows, top first; a program that has ended leaves its row blank.
    pub rows: Vec<String>,
    /// The selected row, and its seconds left (pips lit).
    pub sel: Option<usize>,
    pub left: u8,
    /// A row whose program couldn't be ended.
    pub denied: Option<usize>,
    /// The key held down.
    pub down: Option<usize>,
}

/// The key at a point in the panel (design pixels from the panel's top left): the row's whole
/// height across the key and the tick before it.
pub fn key_at(item: Item, x: f32, y: f32) -> Option<usize> {
    if !has_list(item) || !(KEY_X - 4.0..KEY_X + KEY_W + 1.0).contains(&x) {
        return None;
    }
    let r = libm::floorf((y - KEY_Y + 1.0) / ROW_PITCH as f32);
    (r >= 0.0 && (r as usize) < LIST_ROWS).then_some(r as usize)
}

/// The list's rows as the readings stand: the top [`LIST_ROWS`] programs by `item`.
pub fn ranking(m: &Model, item: Item) -> Vec<String> {
    let mut v: Vec<(f64, &str)> = m.last.procs.iter().map(|p| (proc_value(p, item), p.name.as_str())).collect();
    v.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(core::cmp::Ordering::Equal).then(a.1.cmp(b.1)));
    v.into_iter().take(LIST_ROWS).map(|(_, n)| String::from(n)).collect()
}

fn proc_value(p: &Proc, item: Item) -> f64 {
    match item {
        Item::Cpu => p.cpu as f64,
        Item::Mem => p.mem as f64,
        Item::Gpu => p.gpu as f64,
        _ => p.io,
    }
}

/// What a full bar is: the whole machine's CPU or GPU, all the memory, all programs' I/O.
fn whole(m: &Model, item: Item) -> f64 {
    match item {
        Item::Mem => m.last.mem_total as f64,
        Item::Dsk => m.last.procs.iter().map(|p| p.io).sum(),
        _ => 100.0,
    }
}

fn list_label(item: Item) -> &'static str {
    match item {
        Item::Cpu => "OF ALL CORES",
        Item::Mem => "PRIVATE MEMORY",
        Item::Gpu => "OF GPU",
        _ => "I/O PER SEC",
    }
}

/// A value as at most three digits and a point, and its unit.
fn proc_text(v: f64, item: Item) -> (String, &'static str) {
    match item {
        Item::Mem => {
            let (t, u) = lcd::fmt3((v / (1024.0 * 1024.0)) as f32);
            (t, ["M", "G", "T"][u])
        }
        Item::Dsk => {
            let (t, u) = lcd::fmt3((v / 1024.0) as f32);
            (t, ["K", "M", "G"][u])
        }
        _ => (if v < 99.95 { lcd::fixed1(v as f32) } else { String::from("100") }, "%"),
    }
}

/// A name in capitals cut to `n` characters (a `~` for the rest), and to the field's width.
fn fit_name(name: &str, n: usize) -> String {
    let up: String = name.chars().map(|c| if c.is_ascii() { c.to_ascii_uppercase() } else { '?' }).collect();
    let mut s = if up.len() > n {
        let mut t = String::from(&up[..n - 1]);
        t.push('~');
        t
    } else {
        up
    };
    while lcd::text_w(&s) > n * 4 + 2 && s.len() > 2 {
        s.truncate(s.len() - 2);
        s.push('~');
    }
    s
}

fn text_at(b: &mut Bitmap, s: &str, x: i32, y: i32, v: u8) {
    let t = lcd::text(s, false, 0);
    for ty in 0..t.h {
        for tx in 0..t.w {
            if t.a[ty * t.w + tx] != lcd::OFF {
                b.ps(x + tx as i32, y + ty as i32, v);
            }
        }
    }
}

fn text_right(b: &mut Bitmap, s: &str, xr: i32, y: i32, v: u8) {
    text_at(b, s, xr - lcd::text_w(s) as i32, y, v);
}

/// Turns columns `x0..x1` of rows `y0..=y1` over: set pixels off, the rest lit.
fn invert(b: &mut Bitmap, x0: i32, x1: i32, y0: i32, y1: i32) {
    for y in y0.max(0)..=y1.min(b.h as i32 - 1) {
        for x in x0.max(0)..x1.min(b.w as i32) {
            let i = y as usize * b.w + x as usize;
            b.a[i] = if b.a[i] == lcd::OFF { lcd::LIT } else { lcd::OFF };
        }
    }
}

/// An annunciator: a word at a fixed place on the glass, a ghost when off, in inverse when on.
/// Returns the next one's x.
fn annunciator(b: &mut Bitmap, x: i32, y: i32, s: &str, on: bool) -> i32 {
    let w = lcd::text_w(s) as i32 + 4;
    text_at(b, s, x + 2, y, if on { lcd::LIT } else { lcd::GHOST });
    if on {
        invert(b, x, x + w, y - 1, y + 5);
    }
    x + w + 3
}

/// The list window's bitmap: the annunciators over a dotted rule, then a row per program.
fn list_bitmap(m: &Model, item: Item, v: &ListView) -> Bitmap {
    let mut b = Bitmap::new(LB_W, LW_H as usize);
    let mut x = annunciator(&mut b, 1, 2, "SEL", v.sel.is_some());
    for i in 0..SEL_PIPS {
        let on = v.sel.is_some() && i < v.left;
        for y in 2..7 {
            for dx in 0..2 {
                b.ps(x + i as i32 * 4 + dx, y, if on { lcd::LIT } else { lcd::GHOST });
            }
        }
    }
    x += SEL_PIPS as i32 * 4 + 1;
    annunciator(&mut b, x, 2, "DENIED", v.denied.is_some());
    for x in (0..LB_W as i32).step_by(2) {
        b.ps(x, 8, lcd::DIVIDER);
    }
    let whole = whole(m, item);
    for (i, name) in v.rows.iter().enumerate().take(LIST_ROWS) {
        let y = ROW_Y + i as i32 * ROW_PITCH;
        let Some(p) = m.last.procs.iter().find(|p| p.name == *name) else { continue };
        let val = proc_value(p, item);
        let share = if whole > 0.0 { (val / whole).clamp(0.0, 1.0) } else { 0.0 };
        // all programs' I/O has no capacity to run hot against
        let hot = item != Item::Dsk && share * 100.0 > HOT as f64;
        text_at(&mut b, &fit_name(name, NAME_CHARS), 1, y, if hot { lcd::HOT_PX } else { lcd::LIT });
        if v.denied == Some(i) {
            text_right(&mut b, "ACCESS DENIED", VAL_R, y, lcd::LIT);
        } else {
            // three rows of bar over a dotted track, its length the share on a square-root scale
            for x in (0..BAR_W).step_by(2) {
                b.ps(BAR_X + x, y + 4, lcd::DIVIDER);
            }
            let len = (libm::round(libm::sqrt(share) * BAR_W as f64) as i32).clamp(1, BAR_W);
            for x in BAR_X..BAR_X + len {
                for r in 1..=3 {
                    b.ps(x, y + r, if hot { lcd::HOT_PX } else if r == 1 { lcd::LIT } else { lcd::FILL });
                }
            }
            let (num, unit) = proc_text(val, item);
            text_at(&mut b, unit, VAL_R - 4, y, lcd::LIT);
            text_right(&mut b, &num, VAL_R - 5, y, lcd::LIT);
        }
        if v.sel == Some(i) {
            invert(&mut b, 0, VAL_R + 1, y - 1, y + 5);
        }
    }
    b
}

/// The scale under the bars, engraved: ticks at 0, 1, 5, 10, 25, 50 and 100% on the bars'
/// square-root scale, three of them labelled.
fn scale_bitmap() -> Bitmap {
    let mut b = Bitmap::new(BAR_W as usize + 16, 9);
    for (t, label) in [(0.0, "0"), (0.01, ""), (0.05, ""), (0.1, "10"), (0.25, ""), (0.5, ""), (1.0, "100%")] {
        let x = (libm::round(libm::sqrt(t) * BAR_W as f64) as i32).min(BAR_W - 1);
        b.ps(x, 0, lcd::LIT);
        if !label.is_empty() {
            b.ps(x, 1, lcd::LIT);
            let lx = if t == 0.0 {
                0
            } else if t == 1.0 {
                x - 5
            } else {
                x - lcd::text_w(label) as i32 / 2
            };
            text_at(&mut b, label, lx, 3, lcd::LIT);
        }
    }
    b
}

/// The list's part of the body that never changes: a parting line under the screen, the
/// engraved heading and row numbers, the list window's ring and glass, the engraved tick before
/// each key, the engraved scale and footer. (px, py) is the panel's top left.
fn list_back(pen: &mut Pen, px: f32, py: f32, item: Item) {
    let pal = *pen.pal;
    let line = |y: f32| {
        let mut p = Path::new();
        p.poly(raster::rect_path(px + 2.0, py + y, PANEL_W as f32 - 4.0, 1.0));
        p
    };
    pen.fill(&line(96.0), pal.plate_edge_bottom);
    pen.fill(&line(97.0), pal.plate_edge_top);
    let right = |s: &str| px + PANEL_W as f32 - 5.0 - lcd::text_w(s) as f32;
    pen.engrave(&lcd::text("TOP 10 PROCESSES", false, 0), px + 5.0, py + 103.0);
    let label = list_label(item);
    pen.engrave(&lcd::text(label, false, 0), right(label), py + 103.0);
    for i in 0..LIST_ROWS {
        let n = format!("{}", i + 1);
        pen.engrave(&lcd::text(&n, false, 0), px + 13.0 - lcd::text_w(&n) as f32, py + 126.0 + (i as i32 * ROW_PITCH) as f32);
    }

    // the window: the highlight under its ring, the ring, the glass
    let (wx, wy) = (px + LW_X, py + LW_Y);
    let rr = |x: f32, y: f32, w: f32, h: f32, r: f32| {
        let mut p = Path::new();
        p.poly(raster::round_rect(x, y, w, h, r));
        p
    };
    pen.fill(&rr(wx, wy + 1.5, LW_W, LW_H, 1.5), pal.panel_ring_lip);
    pen.fill(&rr(wx - 1.0, wy - 1.0, LW_W + 2.0, LW_H + 2.0, 2.5), pal.panel_ring);
    let gls = rr(wx, wy, LW_W, LW_H, 1.5);
    pen.fill_grad(&gls, &[(wy, pal.glass_top), (wy + LW_H, pal.glass_bottom)]);
    pen.glass_detail(&gls);

    // the ticks tying each key to its row
    let mut tick = Bitmap::new(3, 1);
    (0..3).for_each(|x| tick.ps(x, 0, lcd::LIT));
    for i in 0..LIST_ROWS {
        pen.engrave(&tick, px + 130.0, py + KEY_Y + (i as i32 * ROW_PITCH) as f32 + 3.0);
    }

    // the scale under the bars, and the footer
    pen.engrave(&scale_bitmap(), px + LW_X + 2.0 + BAR_X as f32, py + 218.0);
    pen.engrave(&lcd::text("BAR: OF TOTAL", false, 0), px + 5.0, py + 221.0);
    pen.engrave(&lcd::text("X SELECTS", false, 0), px + 5.0, py + 231.0);
    pen.engrave(&lcd::text("X AGAIN KILLS", false, 0), right("X AGAIN KILLS"), py + 231.0);
}

/// The list's moving parts: the list window's content and the keys (one may be held down).
fn list_front(pen: &mut Pen, px: f32, py: f32, m: &Model, item: Item, v: &ListView) {
    pen.bitmap(&list_bitmap(m, item, v), px + LW_X + 2.0, py + LW_Y, 1);
    let mut x_glyph = Bitmap::new(5, 5);
    for i in 0..5 {
        x_glyph.ps(i, i, lcd::LIT);
        x_glyph.ps(4 - i, i, lcd::LIT);
    }
    for i in 0..LIST_ROWS {
        let y = py + KEY_Y + (i as i32 * ROW_PITCH) as f32;
        pen.key(px + KEY_X, y, KEY_W, KEY_H, &x_glyph, v.down == Some(i));
    }
}
