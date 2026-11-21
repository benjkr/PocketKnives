//! The menu's header while the buds' place is known (docs/specs/tray.md): Windows XP pixel art
//! (the approved "B1" design) of the case and buds as they are, and a Battery group with Left,
//! Right and Case bars. The core draws the picture; the host draws the text items on it in Tahoma
//! (it owns the fonts), at the positions given here.
//!
//! Laid out in 96-DPI pixels; one art pixel is two of them. At other scalings everything grows by
//! `art / 2`, `art` being the whole number of screen pixels per art pixel nearest to 2 × scaling.
use crate::battery::Pods;
use crate::icon::{self, IconState};
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

mod sprites;

/// A pixel-art sprite: runs of `(x, y, length, palette entry)`, entry 0 never stored.
pub struct Sprite {
    pub w: i32,
    pub h: i32,
    /// The pixel that sits on a bud's position, and the columns its shadow covers.
    pub ax: i32,
    pub ay: i32,
    pub shadow: [i32; 2],
    pub runs: &'static [u8],
}

/// What the header shows.
#[derive(Clone, Debug, PartialEq)]
pub struct HeaderInfo {
    pub name: String,
    /// e.g. "Connected · left in ear".
    pub status: String,
    pub pods: Pods,
    pub connected: bool,
    /// Levels in % (None = not known) and whether each charges.
    pub left: Option<(i32, bool)>,
    pub right: Option<(i32, bool)>,
    pub case: Option<(i32, bool)>,
    /// At or below this a bar is orange.
    pub low_warning: i32,
}

/// A text item for the host to draw: Tahoma at `size` px, top-left at (x, y), or right-aligned to
/// x when `right`.
#[derive(Clone, Debug, PartialEq)]
pub struct Text {
    pub x: i32,
    pub y: i32,
    pub size: i32,
    pub text: String,
    pub bold: bool,
    pub right: bool,
    /// Grayed (an unknown level): `#ACA899` with a white shadow one pixel down and right.
    pub gray: bool,
}

/// The header: opaque BGRA pixels, top-down, and the text to draw on them.
#[derive(Clone, Debug, PartialEq)]
pub struct Header {
    pub width: i32,
    pub height: i32,
    pub bgra: Vec<u8>,
    pub texts: Vec<Text>,
}

/// How a bud reads in the status line: "lid open", "left in ear", …
pub fn pods_phrase(pods: Pods, in_ear: bool) -> &'static str {
    match pods {
        Pods::Closed => "in the case",
        Pods::Open => "lid open",
        Pods::OneOut { left: true } => if in_ear { "left in ear" } else { "left out" },
        Pods::OneOut { left: false } => if in_ear { "right in ear" } else { "right out" },
        Pods::Out => if in_ear { "in ear" } else { "out of the case" },
    }
}

// ---------------------------------------------------------------- layout (96-DPI pixels)

const WIDTH: i32 = 216;
const HEIGHT: i32 = 258;
const FACE: u32 = 0xECE9D8;
const SHADOW: u32 = 0xACA899;
const DARK: u32 = 0x716F64;
const LIGHT: u32 = 0xF1EFE2;
const WHITE: u32 = 0xFFFFFF;
const WELL_BG: u32 = 0x6C8FDD;
/// The scene: 102 × 69 art pixels at (6, 40).
const SCENE: (i32, i32, i32, i32) = (6, 40, 102, 69);
const GROUP_TOP: i32 = 191;
const ROW_TOP: i32 = 201;
const ROW_STEP: i32 = 16;
const TEXT_SIZE: i32 = 11;
/// The bars: 17 blocks of 4 px with 2 px gaps.
const BLOCKS: i32 = 17;

/// The header at `scale` (1 = 96 DPI).
pub fn header(info: &HeaderInfo, scale: f64) -> Header {
    let art = max1(round_half_up(2.0 * scale));
    let px = |v: i32| v * art / 2;
    let (w, h) = (px(WIDTH), px(HEIGHT));
    let mut c = Rgb { w, h, px: vec![FACE; (w * h) as usize] };
    let rect = |c: &mut Rgb, x: i32, y: i32, rw: i32, rh: i32, color: u32| c.fill(px(x), px(y), px(x + rw) - px(x), px(y + rh) - px(y), color);
    let mut texts = Vec::new();
    let text = |texts: &mut Vec<Text>, x: i32, y: i32, s: &str, bold: bool, right: bool, gray: bool| {
        texts.push(Text { x: px(x), y: px(y), size: px(TEXT_SIZE), text: s.into(), bold, right, gray })
    };

    // The head: the tray icon, the name and the status.
    let icon_px = px(16);
    let glyph = icon::render_bgra(if info.connected { IconState::Connected } else { IconState::Disconnected }, true, icon_px);
    c.blend_bgra(px(7), px(11), icon_px, &glyph);
    text(&mut texts, 30, 6, &info.name, true, false, false);
    text(&mut texts, 30, 19, &info.status, false, false, false);

    // The well: a sunken frame around the scene.
    rect(&mut c, 4, 38, 208, 1, SHADOW);
    rect(&mut c, 4, 38, 1, 142, SHADOW);
    rect(&mut c, 4, 179, 208, 1, WHITE);
    rect(&mut c, 211, 38, 1, 142, WHITE);
    rect(&mut c, 5, 39, 206, 140, WELL_BG);
    rect(&mut c, 5, 39, 206, 1, DARK);
    rect(&mut c, 5, 39, 1, 140, DARK);
    rect(&mut c, 5, 178, 206, 1, LIGHT);
    rect(&mut c, 210, 39, 1, 140, LIGHT);
    let scene = scene(info);
    for y in 0..SCENE.3 {
        for x in 0..SCENE.2 {
            let color = scene[(y * SCENE.2 + x) as usize];
            c.fill(px(SCENE.0) + x * art, px(SCENE.1) + y * art, art, art, color);
        }
    }

    // The Battery group: an etched frame with its label over the top edge.
    let (gx, gw, gh) = (6, WIDTH - 12, 62);
    rect(&mut c, gx, GROUP_TOP, gw, 1, SHADOW);
    rect(&mut c, gx, GROUP_TOP, 1, gh, SHADOW);
    rect(&mut c, gx, GROUP_TOP + gh - 1, gw, 1, SHADOW);
    rect(&mut c, gx + gw - 1, GROUP_TOP, 1, gh, SHADOW);
    rect(&mut c, gx + 1, GROUP_TOP + 1, gw - 2, 1, WHITE);
    rect(&mut c, gx + 1, GROUP_TOP + 1, 1, gh - 2, WHITE);
    rect(&mut c, gx, GROUP_TOP + gh, gw + 1, 1, WHITE);
    rect(&mut c, gx + gw, GROUP_TOP, 1, gh + 1, WHITE);
    rect(&mut c, gx + 4, GROUP_TOP - 1, 46, 3, FACE);
    text(&mut texts, gx + 6, GROUP_TOP - 7, "Battery", false, false, false);

    for (i, (label, level)) in [("Left:", info.left), ("Right:", info.right), ("Case:", info.case)].into_iter().enumerate() {
        let y = ROW_TOP + i as i32 * ROW_STEP;
        let gray = level.is_none();
        text(&mut texts, 13, y, label, false, false, gray);
        // The bar: a sunken white box with whole blocks.
        rect(&mut c, 47, y, 107, 1, SHADOW);
        rect(&mut c, 47, y, 1, 13, SHADOW);
        rect(&mut c, 47, y + 12, 107, 1, WHITE);
        rect(&mut c, 153, y, 1, 13, WHITE);
        rect(&mut c, 48, y + 1, 105, 11, WHITE);
        let pct = match level {
            Some((p, charging)) => {
                let n = (round_half_up(p as f64 / 100.0 * BLOCKS as f64)).clamp(1, BLOCKS);
                let ramp = if p <= info.low_warning { &ORANGE } else { &GREEN };
                for b in 0..n {
                    for (r, &color) in ramp.iter().enumerate() {
                        rect(&mut c, 49 + b * 6, y + 2 + r as i32, 4, 1, color);
                    }
                }
                if charging {
                    c.blit_rows(px(190), px(y + 2), art / 2, &BOLT);
                }
                alloc::format!("{p}%")
            }
            None => "\u{2014}".into(),
        };
        text(&mut texts, 186, y, &pct, false, true, gray);
    }
    Header { width: w, height: h, bgra: c.to_bgra(), texts }
}

/// The bars' rows, top to bottom (the mockup's gradient, sampled per row).
const GREEN: [u32; 9] = [0xC9F2C3, 0x7BE077, 0x5AD858, 0x3ED13E, 0x37C937, 0x2DBE2D, 0x3EC83E, 0x4FD14F, 0x8DE58A];
const ORANGE: [u32; 9] = [0xFFE2B0, 0xFFB54A, 0xFFA725, 0xFF9900, 0xF69000, 0xEE8800, 0xF79613, 0xFFA41E, 0xFFC670];
/// The charging bolt, 5 × 8.
const BOLT: [&[u8]; 8] = [b"...kk", b"..kyk", b".kyk.", b"kyykk", b"kkyyk", b".kyk.", b"kyk..", b"kk..."];

// ---------------------------------------------------------------- the scene (art pixels)

/// Backdrop: top and bottom colours dithered down the height, and the shadow colour.
const STAGE_ON: [u32; 3] = [0x7B9FE7, 0x6C90DF, 0x4868BE];
const STAGE_OFF: [u32; 3] = [0xB5B9C1, 0xA8ACB5, 0x8C9099];
const BAYER4: [[i32; 4]; 4] = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];
/// The status light: green (open), amber (charging in the closed case, or a low bud).
const LED_GREEN: [u32; 2] = [0x4DE38A, 0x00C853];
const LED_AMBER: [u32; 2] = [0xFFB13B, 0xFF9900];

/// The scene's pixels, 102 × 69, as the mockup's resting poses lay them out (units of 1.5 art px).
fn scene(info: &HeaderInfo) -> Vec<u32> {
    let (w, h) = (SCENE.2, SCENE.3);
    let stage = if info.connected { STAGE_ON } else { STAGE_OFF };
    let mut px: Vec<u32> = (0..w * h)
        .map(|i| {
            let (x, y) = (i % w, i / w);
            let t = (BAYER4[(y % 4) as usize][(x % 4) as usize] as f64 + 0.5) / 16.0;
            if y as f64 / (h - 1) as f64 > t { stage[1] } else { stage[0] }
        })
        .collect();
    let mut palette: Vec<u32> = if info.connected { sprites::ON.to_vec() } else { sprites::OFF.to_vec() };
    let low = [info.left, info.right].iter().flatten().any(|&(p, _)| p <= info.low_warning);
    let led = if !info.connected {
        None
    } else {
        match info.pods {
            Pods::Out => None,
            Pods::Closed => Some(LED_AMBER),
            _ if low => Some(LED_AMBER),
            _ => Some(LED_GREEN),
        }
    };
    let (m, s) = (key(&palette, b'm'), key(&palette, b's'));
    let lit = led.unwrap_or([m, s]);
    set_key(&mut palette, b'a', lit[0]);
    set_key(&mut palette, b'b', lit[1]);

    let p = |v: f64| round_half_up(v * 1.5);
    let mut canvas = Art { w, h, px: &mut px, palette: &palette };
    let (ky, shadow_y) = (p(9.0), p(44.0));
    let seat_y = p(16.0) - 2;
    // Where the case sits, and which buds are out (and where their middles rest, in units).
    let (kx, out_left, out_right): (f64, Option<(f64, f64)>, Option<(f64, f64)>) = match info.pods {
        Pods::Closed | Pods::Open => (17.0, None, None),
        Pods::OneOut { left: true } => (29.0, Some((12.0, 21.0)), None),
        Pods::OneOut { left: false } => (5.0, None, Some((56.0, 21.0))),
        Pods::Out => (17.0, Some((22.0, 28.0)), Some((46.0, 28.0))),
    };
    let kx = p(kx);
    let place = |b: &Sprite, mx: i32, my: i32, left: bool| (mx - if left { b.w - b.ax } else { b.ax }, my - b.ay);
    let mut top = Vec::new();
    for (left, out) in [(true, out_left), (false, out_right)] {
        if let Some((ux, uy)) = out {
            let b = if left { &sprites::OUT_LEFT } else { &sprites::OUT_RIGHT };
            let (x, y) = place(b, round_half_up(ux * 1.5), round_half_up(uy * 1.5), left);
            let cols = if left { [b.w - 1 - b.shadow[1], b.w - 1 - b.shadow[0]] } else { b.shadow };
            canvas.shadow(x + cols[0], x + cols[1], shadow_y, 2, 1.0, stage[2]);
            top.push((b, x, y));
        }
    }
    if info.pods == Pods::Out {
        canvas.blit(&sprites::CASE_MINI, p(24.0), p(3.0));
    } else {
        let front = if info.pods == Pods::Closed { &sprites::CASE_CLOSED } else { &sprites::CASE_OPEN_FRONT };
        canvas.shadow(kx + p(2.0), kx + front.w - 1 - p(2.0), ky + front.h, p(2.0), 6.0 / (p(2.0) - 1) as f64, stage[2]);
        if info.pods != Pods::Closed {
            canvas.blit(&sprites::CASE_OPEN_BACK, kx, ky);
            // The buds still in the case, seated between the case's back and front.
            let seats = [(true, out_left.is_none()), (false, out_right.is_none())];
            for (left, seated) in seats {
                if seated {
                    let b = if left { &sprites::SEAT_LEFT } else { &sprites::SEAT_RIGHT };
                    let mx = kx + if left { p(9.0) } else { front.w - p(9.0) };
                    let (x, y) = place(b, mx, ky + seat_y, left);
                    canvas.blit(b, x, y);
                }
            }
        }
        canvas.blit(front, kx, ky);
    }
    for (b, x, y) in top {
        canvas.blit(b, x, y);
    }
    px
}

fn key(palette: &[u32], k: u8) -> u32 {
    sprites::KEYS.iter().position(|&c| c == k).map_or(0, |i| palette[i])
}

fn set_key(palette: &mut [u32], k: u8, color: u32) {
    if let Some(i) = sprites::KEYS.iter().position(|&c| c == k) {
        palette[i] = color;
    }
}

struct Art<'a> {
    w: i32,
    h: i32,
    px: &'a mut Vec<u32>,
    palette: &'a [u32],
}

impl Art<'_> {
    fn put(&mut self, x: i32, y: i32, color: u32) {
        if x >= 0 && y >= 0 && x < self.w && y < self.h {
            self.px[(y * self.w + x) as usize] = color;
        }
    }

    fn blit(&mut self, s: &Sprite, x0: i32, y0: i32) {
        for r in s.runs.chunks_exact(4) {
            let color = self.palette[r[3] as usize - 1];
            for i in 0..r[2] as i32 {
                self.put(x0 + r[0] as i32 + i, y0 + r[1] as i32, color);
            }
        }
    }

    /// A checkered shadow, each row shorter at both ends by `taper`.
    fn shadow(&mut self, x0: i32, x1: i32, y0: i32, rows: i32, taper: f64, color: u32) {
        for r in 0..rows {
            let ins = round_half_up(r as f64 * taper);
            for x in x0 + ins..=x1 - ins {
                if (x + y0 + r) & 1 == 1 {
                    self.put(x, y0 + r, color);
                }
            }
        }
    }
}

// ---------------------------------------------------------------- the picture

struct Rgb {
    w: i32,
    h: i32,
    px: Vec<u32>,
}

impl Rgb {
    fn fill(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        for yy in y.max(0)..(y + h).min(self.h) {
            for xx in x.max(0)..(x + w).min(self.w) {
                self.px[(yy * self.w + xx) as usize] = color;
            }
        }
    }

    /// Straight-alpha BGRA `size` × `size` over this picture.
    fn blend_bgra(&mut self, x0: i32, y0: i32, size: i32, bgra: &[u8]) {
        for y in 0..size {
            for x in 0..size {
                let i = ((y * size + x) * 4) as usize;
                let a = bgra[i + 3] as u32;
                if a == 0 || x0 + x >= self.w || y0 + y >= self.h {
                    continue;
                }
                let o = ((y0 + y) * self.w + x0 + x) as usize;
                let d = self.px[o];
                let mix = |s: u8, d: u32| (s as u32 * a + d * (255 - a) + 127) / 255;
                let (r, g, b) = (mix(bgra[i + 2], d >> 16), mix(bgra[i + 1], (d >> 8) & 0xFF), mix(bgra[i], d & 0xFF));
                self.px[o] = (r << 16) | (g << 8) | b;
            }
        }
    }

    /// A small pixel glyph (`k` near-black, `y` yellow), `cell` screen pixels per glyph pixel.
    fn blit_rows(&mut self, x0: i32, y0: i32, cell: i32, rows: &[&[u8]]) {
        let cell = cell.max(1);
        for (y, row) in rows.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                let color = match c {
                    b'k' => 0x202020,
                    b'y' => 0xFFD21E,
                    _ => continue,
                };
                self.fill(x0 + x as i32 * cell, y0 + y as i32 * cell, cell, cell, color);
            }
        }
    }

    fn to_bgra(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.px.len() * 4);
        for &p in &self.px {
            out.extend_from_slice(&[(p & 0xFF) as u8, ((p >> 8) & 0xFF) as u8, (p >> 16) as u8, 0xFF]);
        }
        out
    }
}

/// JavaScript's `Math.round` (halves up), which the mockup's layout used.
fn round_half_up(v: f64) -> i32 {
    crate::math::floor(v + 0.5) as i32
}

fn max1(v: i32) -> i32 {
    v.max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn info(pods: Pods, connected: bool) -> HeaderInfo {
        HeaderInfo {
            name: "AirPods Pro".into(),
            status: "Connected · left in ear".into(),
            pods,
            connected,
            left: Some((80, false)),
            right: Some((20, true)),
            case: None,
            low_warning: 20,
        }
    }

    #[test]
    fn sizes_and_text() {
        for (scale, w) in [(1.0, 216), (1.25, 324), (1.5, 324), (2.0, 432)] {
            let hd = header(&info(Pods::OneOut { left: true }, true), scale);
            assert_eq!((hd.width, hd.bgra.len() as i32), (w, w * hd.height * 4), "{scale}");
        }
        let hd = header(&info(Pods::Open, true), 1.0);
        let t: Vec<&str> = hd.texts.iter().map(|t| t.text.as_str()).collect();
        assert_eq!(t, ["AirPods Pro", "Connected · left in ear", "Battery", "Left:", "80%", "Right:", "20%", "Case:", "\u{2014}"]);
        assert!(hd.texts[8].gray && hd.texts[8].right);
    }

    #[test]
    fn every_pose_draws_differently() {
        let all = [Pods::Closed, Pods::Open, Pods::OneOut { left: true }, Pods::OneOut { left: false }, Pods::Out];
        let scenes: Vec<Vec<u32>> = all.iter().map(|&p| scene(&info(p, true))).collect();
        for i in 0..scenes.len() {
            for j in i + 1..scenes.len() {
                assert_ne!(scenes[i], scenes[j], "{:?} {:?}", all[i], all[j]);
            }
        }
        assert_ne!(scene(&info(Pods::Open, true)), scene(&info(Pods::Open, false)));
    }
}
