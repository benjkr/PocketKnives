//! The lettering on the plastic, like a calculator's printed labels: Arial Bold (on every
//! Windows), its outlines from GDI at Arial's own size, unhinted, filled by our rasterizer.
//! Hinting at a few pixels an em would space the letters unevenly.
use crate::raster::{Path, Pt};
use alloc::vec;
use alloc::vec::Vec;
use core::ptr::null_mut;
use windows_sys::Win32::Graphics::Gdi::{
    ANSI_CHARSET, ANTIALIASED_QUALITY, CLIP_DEFAULT_PRECIS, CreateCompatibleDC, CreateFontW, DeleteDC, DeleteObject,
    FIXED, FW_BOLD, GDI_ERROR, GGO_NATIVE, GGO_UNHINTED, GLYPHMETRICS, GetGlyphOutlineW, HDC, HFONT, HGDIOBJ, MAT2,
    OUT_TT_ONLY_PRECIS, SelectObject, TT_PRIM_LINE, TT_PRIM_QSPLINE,
};

/// The capitals' height in design pixels: Arial's are 0.716 of its em, so an em of 7.
const CAP: f32 = 5.0;
const EM: f32 = 7.0;
/// Arial's own units to the em: outlines read at this size are exact.
const UNITS: i32 = 2048;
/// No transformation: GDI fails with a zero matrix.
const MAT: MAT2 = MAT2 {
    eM11: FIXED { fract: 0, value: 1 },
    eM12: FIXED { fract: 0, value: 0 },
    eM21: FIXED { fract: 0, value: 0 },
    eM22: FIXED { fract: 0, value: 1 },
};

/// Arial Bold, open for as long as it's needed.
pub struct Face {
    dc: HDC,
    font: HFONT,
    old: HGDIOBJ,
}

impl Face {
    pub fn new() -> Face {
        let name: Vec<u16> = "Arial".encode_utf16().chain([0]).collect();
        unsafe {
            let dc = CreateCompatibleDC(null_mut());
            let font = CreateFontW(
                -UNITS,
                0,
                0,
                0,
                FW_BOLD as i32,
                0,
                0,
                0,
                ANSI_CHARSET as u32,
                OUT_TT_ONLY_PRECIS as u32,
                CLIP_DEFAULT_PRECIS as u32,
                ANTIALIASED_QUALITY as u32,
                0,
                name.as_ptr(),
            );
            let old = if dc.is_null() || font.is_null() { null_mut() } else { SelectObject(dc, font) };
            Face { dc, font, old }
        }
    }

    /// `s`'s outline in design pixels, the capitals' top at y 0 and the first glyph's origin at
    /// x 0, and its advance.
    pub fn text(&self, s: &str) -> (Path, f32) {
        let (mut p, mut x) = (Path::new(), 0.0);
        let sc = EM / UNITS as f32;
        for ch in s.chars() {
            let (contours, adv) = self.glyph(ch as u32);
            for c in contours {
                p.poly(c.iter().map(|&(gx, gy)| (x + gx * sc, CAP - gy * sc)).collect());
            }
            x += adv as f32 * sc;
        }
        (p, x)
    }

    /// `s`'s advance in design pixels.
    pub fn width(&self, s: &str) -> f32 {
        self.text(s).1
    }

    /// One glyph's contours (Arial's units, y up from the baseline) and its advance; nothing
    /// when GDI fails. A space has an advance and no contours.
    fn glyph(&self, ch: u32) -> (Vec<Vec<Pt>>, i32) {
        if self.old.is_null() {
            return (Vec::new(), 0);
        }
        let format = GGO_NATIVE | GGO_UNHINTED;
        let mut gm: GLYPHMETRICS = unsafe { core::mem::zeroed() };
        let n = unsafe { GetGlyphOutlineW(self.dc, ch, format, &mut gm, 0, null_mut(), &MAT) };
        if n == GDI_ERROR as u32 {
            return (Vec::new(), 0);
        }
        let mut buf = vec![0u8; n as usize];
        if n > 0 && unsafe { GetGlyphOutlineW(self.dc, ch, format, &mut gm, n, buf.as_mut_ptr().cast(), &MAT) } == GDI_ERROR as u32 {
            return (Vec::new(), 0);
        }
        (outline(&buf), gm.gmCellIncX as i32)
    }
}

impl Drop for Face {
    fn drop(&mut self) {
        unsafe {
            if !self.old.is_null() {
                SelectObject(self.dc, self.old);
            }
            if !self.font.is_null() {
                DeleteObject(self.font);
            }
            if !self.dc.is_null() {
                DeleteDC(self.dc);
            }
        }
    }
}

/// `GGO_NATIVE`'s buffer as polygons: each `TTPOLYGONHEADER` (size, type, start point) then its
/// `TTPOLYCURVE`s (type, count, points), quadratic splines flattened.
fn outline(b: &[u8]) -> Vec<Vec<Pt>> {
    let u16_at = |i: usize| b.get(i..i + 2).map_or(0, |s| u16::from_le_bytes([s[0], s[1]]));
    let u32_at = |i: usize| b.get(i..i + 4).map_or(0, |s| u32::from_le_bytes([s[0], s[1], s[2], s[3]]));
    // FIXED: a 16-bit fraction, then a signed 16-bit whole part
    let fixed = |i: usize| u16_at(i + 2) as i16 as f32 + u16_at(i) as f32 / 65536.0;
    let point = |i: usize| (fixed(i), fixed(i + 4));
    let mut out = Vec::new();
    let mut h = 0;
    while h + 16 <= b.len() {
        let cb = u32_at(h) as usize;
        if cb < 16 || h + cb > b.len() {
            break;
        }
        let mut c = vec![point(h + 8)];
        let mut i = h + 16;
        while i + 4 <= h + cb {
            let (kind, n) = (u16_at(i) as u32, u16_at(i + 2) as usize);
            let pts: Vec<Pt> = (0..n).map(|j| point(i + 4 + j * 8)).collect();
            if kind == TT_PRIM_LINE {
                c.extend_from_slice(&pts);
            } else if kind == TT_PRIM_QSPLINE && n >= 2 {
                // every point but the last is a control point; between two, the curve passes midway
                for j in 0..n - 1 {
                    let p0 = *c.last().unwrap_or(&pts[j]);
                    let ctl = pts[j];
                    let end = if j == n - 2 { pts[n - 1] } else { ((pts[j].0 + pts[j + 1].0) / 2.0, (pts[j].1 + pts[j + 1].1) / 2.0) };
                    for s in 1..=4 {
                        let t = s as f32 / 4.0;
                        let (a, bb, cc) = ((1.0 - t) * (1.0 - t), 2.0 * t * (1.0 - t), t * t);
                        c.push((a * p0.0 + bb * ctl.0 + cc * end.0, a * p0.1 + bb * ctl.1 + cc * end.1));
                    }
                }
            }
            i += 4 + n * 8;
        }
        out.push(c);
        h += cb;
    }
    out
}
