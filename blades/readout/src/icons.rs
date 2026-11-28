//! Programs' own icons for the list's colour screen: the icon in the program's file, cut down to
//! [`SIZE`] x [`SIZE`] opaque or clear pixels, one design pixel each, for a slightly pixelated look.
use crate::procs::Proc;
use alloc::vec;
use alloc::vec::Vec;
use core::mem::{size_of, zeroed};
use core::ptr::null_mut;
use windows_sys::Win32::Foundation::CloseHandle;
use windows_sys::Win32::Graphics::Gdi::{
    BI_RGB, BITMAP, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, DeleteObject, GetDC, GetDIBits, GetObjectW, HBITMAP,
    ReleaseDC,
};
use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION, QueryFullProcessImageNameW};
use windows_sys::Win32::UI::Shell::SHDefExtractIconW;
use windows_sys::Win32::UI::WindowsAndMessaging::{DestroyIcon, GetIconInfo, HICON, ICONINFO};

/// An icon's pixels a side.
pub const SIZE: usize = 8;
/// Row by row, `0xAARRGGBB` with alpha 0 or 255.
pub type Icon = [u32; SIZE * SIZE];

/// The size the icon is read at before it's cut down.
const READ: u32 = 32;
/// A pixel is opaque when at least this much of its area is covered.
const OPAQUE: u32 = 102;

/// The icons of `rows`' programs, found among `procs` by name; none for a program that isn't
/// there, whose file can't be read (protected and system processes) or that has no icon.
pub fn for_rows(rows: &[alloc::string::String], procs: &[Proc]) -> Vec<Option<Icon>> {
    rows.iter().map(|n| procs.iter().find(|p| p.name.eq_ignore_ascii_case(n)).and_then(|p| of_pid(p.pid))).collect()
}

/// The icon of process `pid`'s program file.
pub fn of_pid(pid: u32) -> Option<Icon> {
    if pid == 0 {
        return None;
    }
    let h = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid) };
    if h.is_null() {
        return None;
    }
    let mut path = vec![0u16; 1024];
    let mut n = path.len() as u32 - 1;
    let ok = unsafe { QueryFullProcessImageNameW(h, PROCESS_NAME_WIN32, path.as_mut_ptr(), &mut n) };
    unsafe { CloseHandle(h) };
    if ok == 0 {
        return None;
    }
    path.truncate(n as usize);
    path.push(0);
    let mut icon: HICON = null_mut();
    let hr = unsafe { SHDefExtractIconW(path.as_ptr(), 0, 0, &mut icon, null_mut(), READ) };
    if hr != 0 || icon.is_null() {
        return None;
    }
    let out = pixels(icon).map(|(w, h, px)| shrink(w, h, &px));
    unsafe { DestroyIcon(icon) };
    out
}

/// An icon's colour pixels, `0xAARRGGBB` straight, top row first, with its width and height.
/// An icon without alpha takes it from its mask.
fn pixels(icon: HICON) -> Option<(usize, usize, Vec<u32>)> {
    let mut ii: ICONINFO = unsafe { zeroed() };
    if unsafe { GetIconInfo(icon, &mut ii) } == 0 {
        return None;
    }
    let out = (|| {
        if ii.hbmColor.is_null() {
            return None;
        }
        let (w, h) = size(ii.hbmColor)?;
        let mut px = bits(ii.hbmColor, w, h)?;
        if px.iter().all(|p| p >> 24 == 0) {
            // the mask is white where the icon is clear
            let mask = bits(ii.hbmMask, w, h)?;
            for (p, m) in px.iter_mut().zip(mask) {
                *p = (*p & 0xff_ffff) | if m & 0xff_ffff == 0 { 0xff00_0000 } else { 0 };
            }
        }
        Some((w, h, px))
    })();
    unsafe {
        DeleteObject(ii.hbmColor);
        DeleteObject(ii.hbmMask);
    }
    out
}

fn size(bm: HBITMAP) -> Option<(usize, usize)> {
    let mut b: BITMAP = unsafe { zeroed() };
    let n = unsafe { GetObjectW(bm, size_of::<BITMAP>() as i32, (&mut b as *mut BITMAP).cast()) };
    (n != 0 && b.bmWidth > 0 && b.bmHeight > 0).then_some((b.bmWidth as usize, b.bmHeight as usize))
}

/// The first `h` rows of `bm` as 32-bit pixels, top row first.
fn bits(bm: HBITMAP, w: usize, h: usize) -> Option<Vec<u32>> {
    if bm.is_null() {
        return None;
    }
    let mut bi: BITMAPINFO = unsafe { zeroed() };
    bi.bmiHeader = BITMAPINFOHEADER {
        biSize: size_of::<BITMAPINFOHEADER>() as u32,
        biWidth: w as i32,
        biHeight: -(h as i32),
        biPlanes: 1,
        biBitCount: 32,
        biCompression: BI_RGB,
        ..unsafe { zeroed() }
    };
    let mut px = vec![0u32; w * h];
    let dc = unsafe { GetDC(null_mut()) };
    let n = unsafe { GetDIBits(dc, bm, 0, h as u32, px.as_mut_ptr().cast(), &mut bi, DIB_RGB_COLORS) };
    unsafe { ReleaseDC(null_mut(), dc) };
    (n == h as i32).then_some(px)
}

/// `w` x `h` pixels down to [`SIZE`] x [`SIZE`]: each the average colour of its area, weighted
/// by alpha, and opaque when enough of the area is covered.
fn shrink(w: usize, h: usize, px: &[u32]) -> Icon {
    let mut sum = [[0u32; 5]; SIZE * SIZE];
    for y in 0..h {
        for x in 0..w {
            let p = px[y * w + x];
            let a = p >> 24;
            let s = &mut sum[(y * SIZE / h) * SIZE + x * SIZE / w];
            s[0] += a;
            s[1] += ((p >> 16) & 255) * a;
            s[2] += ((p >> 8) & 255) * a;
            s[3] += (p & 255) * a;
            s[4] += 1;
        }
    }
    let mut out = [0u32; SIZE * SIZE];
    for (o, s) in out.iter_mut().zip(sum) {
        if s[4] > 0 && s[0] >= OPAQUE * s[4] {
            *o = 0xff00_0000 | (s[1] / s[0]) << 16 | (s[2] / s[0]) << 8 | s[3] / s[0];
        }
    }
    out
}
