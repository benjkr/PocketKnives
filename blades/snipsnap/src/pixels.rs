//! The pixel work on the hot paths: the dimmed copy (before the overlay shows), the overlay's
//! repaint and the 24-bit clipboard rows (after the click). No state of its own, so the loop
//! benchmark builds this file as it is.
use core::arch::x86_64::{__m128i, _mm_and_si128, _mm_loadu_si128, _mm_set1_epi32, _mm_sfence, _mm_srli_epi32, _mm_stream_si128};
use windows_sys::Win32::Foundation::RECT;
use windows_sys::Win32::Graphics::Gdi::{
    BitBlt, ExcludeClipRect, FrameRect, HBRUSH, HDC, InflateRect, IntersectRect, RestoreDC, SRCCOPY, SaveDC,
};

/// `dst` = `src` with each channel halved (the X byte cleared), four pixels at a time with SSE2,
/// which every x64 CPU has.
pub fn dim(src: &[u32], dst: &mut [u32]) {
    // SSE2 is part of x86-64, so the intrinsics' only requirement holds.
    let mask = unsafe { _mm_set1_epi32(0x7F7F7F) };
    let len = src.len().min(dst.len());
    // A DIB section's pixels are page-aligned: then all but the last few go four at a time,
    // stored straight to memory past the cache, which the image is far bigger than (a third
    // faster here). The rest, or everything if `dst` isn't aligned, one at a time.
    let n = if dst.as_ptr() as usize % 16 == 0 { len & !3 } else { 0 };
    let (s, d) = (src.as_ptr(), dst.as_mut_ptr());
    let mut i = 0;
    while i < n {
        // i + 4 <= n <= len: in bounds; d + i is 16-byte aligned.
        unsafe {
            let v = _mm_loadu_si128(s.add(i) as *const __m128i);
            _mm_stream_si128(d.add(i) as *mut __m128i, _mm_and_si128(_mm_srli_epi32(v, 1), mask));
        }
        i += 4;
    }
    unsafe { _mm_sfence() };
    while i < len {
        unsafe { *d.add(i) = (*s.add(i) >> 1) & 0x7F7F7F };
        i += 1;
    }
}

/// The length of a 24 bpp DIB row `w` pixels wide: padded to 4 bytes.
pub fn dib24_stride(w: i32) -> usize {
    (w as usize).wrapping_mul(3).wrapping_add(3) & !3
}

/// The `w`×`h` area at (`rx`, `ry`) of a top-down 32 bpp image `src_w` pixels wide, as the rows
/// of a bottom-up 24 bpp DIB, the padding zeroed, into `dst`. Does nothing if the area isn't
/// inside both.
pub fn dib24(src: &[u32], src_w: i32, rx: i32, ry: i32, w: i32, h: i32, dst: &mut [u8]) {
    // None negative: then each is below 2^31, and no sum or product below can wrap.
    if src_w | rx | ry | w | h < 0 {
        return;
    }
    let (src_w, rx, ry, w, h) = (src_w as usize, rx as usize, ry as usize, w as usize, h as usize);
    let stride = dib24_stride(w as i32);
    if rx + w > src_w || (ry + h) * src_w > src.len() || stride * h > dst.len() {
        return;
    }
    for y in 0..h {
        // In bounds: checked above, row by row.
        unsafe {
            let mut s = src.as_ptr().add((ry + h - 1 - y) * src_w + rx);
            let mut d = dst.as_mut_ptr().add(y * stride);
            let end = d.add(stride);
            let mut n = w;
            // Four pixels make 12 bytes: one 8-byte and one 4-byte store.
            while n >= 4 {
                let (a, b, c, e) = (*s, *s.add(1), *s.add(2), *s.add(3));
                let lo = (a & 0xFF_FFFF) as u64 | ((b & 0xFF_FFFF) as u64) << 24 | (c as u64) << 48;
                (d as *mut u64).write_unaligned(lo);
                (d.add(8) as *mut u32).write_unaligned((c >> 16 & 0xFF) | e << 8);
                s = s.add(4);
                d = d.add(12);
                n -= 4;
            }
            while n > 0 {
                let c = *s;
                *d = c as u8;
                *d.add(1) = (c >> 8) as u8;
                *d.add(2) = (c >> 16) as u8;
                s = s.add(1);
                d = d.add(3);
                n -= 1;
            }
            while d < end {
                *d = 0;
                d = d.add(1);
            }
        }
    }
}

/// Paints the update area `clip` of the overlay: the dimmed image, and when `lit` is set an
/// undimmed area with a white border `frame_w` wide (the selection or the hovered window). Every
/// pixel is written exactly once (dim outside, bright inside, border on its ring), so the
/// compositor never shows a half-painted, flickering state.
pub fn paint_backdrop(dc: HDC, clip: RECT, lit: Option<&RECT>, dim: HDC, shot: HDC, frame: HBRUSH, frame_w: i32) {
    let (w, h) = (clip.right - clip.left, clip.bottom - clip.top);
    let Some(lit) = lit else {
        unsafe { BitBlt(dc, clip.left, clip.top, w, h, dim, clip.left, clip.top, SRCCOPY) };
        return;
    };
    unsafe {
        let saved = SaveDC(dc);
        ExcludeClipRect(dc, lit.left, lit.top, lit.right, lit.bottom);
        BitBlt(dc, clip.left, clip.top, w, h, dim, clip.left, clip.top, SRCCOPY);
        RestoreDC(dc, saved);
        let mut inner = *lit;
        let mut i: RECT = core::mem::zeroed();
        InflateRect(&mut inner, -frame_w, -frame_w);
        if IntersectRect(&mut i, &inner, &clip) != 0 {
            BitBlt(dc, i.left, i.top, i.right - i.left, i.bottom - i.top, shot, i.left, i.top, SRCCOPY);
        }
        let mut f = *lit;
        let mut k = 0;
        while k < frame_w && f.right > f.left && f.bottom > f.top {
            FrameRect(dc, &f, frame);
            InflateRect(&mut f, -1, -1);
            k += 1;
        }
    }
}
