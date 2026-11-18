//! Icon pixels to HICONs, and the `--render-icons` check sheets drawn through the real
//! HICON + `DrawIconEx` path (16/20/24/32 px on both taskbars, every animation frame, 240 px glyphs).
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use quicktooth_core::host::IconImage;
use quicktooth_core::icon::{self, AnimPhase, IconState};
use quicktooth_core::tunables::{self, T};
use windows_sys::Win32::Foundation::{CloseHandle, GENERIC_WRITE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Graphics::Gdi::{
    BI_RGB, BITMAPINFO, BITMAPINFOHEADER, CreateCompatibleDC, CreateDIBSection, DIB_RGB_COLORS, DeleteDC, DeleteObject, SelectObject,
};
use windows_sys::Win32::Storage::FileSystem::{CREATE_ALWAYS, CreateDirectoryW, CreateFileW, FILE_ATTRIBUTE_NORMAL, WriteFile};
use windows_sys::Win32::UI::WindowsAndMessaging::{CreateIconFromResourceEx, DI_NORMAL, DestroyIcon, DrawIconEx, HICON};

/// The HICON for `icon`, made on first use and kept in `icon.host_handle`.
#[allow(dead_code)]
pub fn handle_for(icon: &IconImage) -> HICON {
    let h = icon.host_handle.get();
    if h != 0 {
        return h as HICON;
    }
    let h = from_bgra(&icon.bgra, icon.size);
    if h.is_null() {
        quicktooth_core::log::error("icon", "CreateIconFromResourceEx failed");
    }
    icon.host_handle.set(h as isize);
    h
}

/// Destroys `icon`'s HICON, if one was made.
#[allow(dead_code)]
pub fn release(icon: &IconImage) {
    let h = icon.host_handle.replace(0);
    if h != 0 {
        unsafe { DestroyIcon(h as HICON) };
    }
}

#[allow(dead_code)]
pub fn create_icon(state: IconState, light_taskbar: bool, size: i32) -> HICON {
    from_bgra(&icon::render_bgra(state, light_taskbar, size), size)
}

#[allow(dead_code)]
pub fn create_anim_frame(phase: AnimPhase, t: f64, tracer_t: f64, light_taskbar: bool, size: i32) -> HICON {
    from_bgra(&icon::render_anim(phase, t, tracer_t, light_taskbar, size), size)
}

/// A 32-bit HICON from straight-alpha BGRA pixels; null if Windows refuses. The caller owns it
/// (`DestroyIcon`).
pub fn from_bgra(bgra: &[u8], size: i32) -> HICON {
    // An icon resource image: BITMAPINFOHEADER (height doubled) + bottom-up XOR pixels + AND mask.
    let n = size as usize;
    let xor_bytes = n * n * 4;
    let mask_stride = n.div_ceil(32) * 4;
    let mask_bytes = mask_stride * n;
    let mut res = vec![0u8; 40 + xor_bytes + mask_bytes];
    res[0..4].copy_from_slice(&40i32.to_le_bytes());
    res[4..8].copy_from_slice(&size.to_le_bytes());
    res[8..12].copy_from_slice(&(size * 2).to_le_bytes());
    res[12..14].copy_from_slice(&1i16.to_le_bytes());
    res[14..16].copy_from_slice(&32i16.to_le_bytes());
    res[20..24].copy_from_slice(&((xor_bytes + mask_bytes) as i32).to_le_bytes());
    for y in 0..n {
        let src = (n - 1 - y) * n * 4;
        res[40 + y * n * 4..40 + (y + 1) * n * 4].copy_from_slice(&bgra[src..src + n * 4]);
    }
    // The AND mask stays all zero: the alpha channel decides transparency.
    unsafe { CreateIconFromResourceEx(res.as_ptr(), res.len() as u32, 1, 0x0003_0000, size, size, 0) }
}

// ------------------------------------------------------------------ verification images

#[derive(Clone, Copy)]
struct Bg {
    r: u8,
    g: u8,
    b: u8,
}

/// Draws an HICON the way the shell does (`DrawIconEx`) onto an opaque background colour.
fn draw_icon_on_background(h_icon: HICON, size: i32, bg: Bg) -> Vec<u8> {
    let n = (size * size) as usize;
    let mut result = vec![0u8; n * 4];
    unsafe {
        let mut bmi: BITMAPINFO = core::mem::zeroed();
        bmi.bmiHeader = BITMAPINFOHEADER {
            biSize: 40,
            biWidth: size,
            biHeight: -size,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB,
            ..core::mem::zeroed()
        };
        let dc = CreateCompatibleDC(core::ptr::null_mut());
        let mut bits: *mut core::ffi::c_void = core::ptr::null_mut();
        let dib = CreateDIBSection(dc, &bmi, DIB_RGB_COLORS, &mut bits, core::ptr::null_mut(), 0);
        if dib.is_null() || bits.is_null() {
            DeleteDC(dc);
            return result;
        }
        let old = SelectObject(dc, dib);
        let p = core::slice::from_raw_parts_mut(bits as *mut u8, n * 4);
        for px in p.chunks_exact_mut(4) {
            px.copy_from_slice(&[bg.b, bg.g, bg.r, 255]);
        }
        DrawIconEx(dc, 0, 0, h_icon, size, size, 0, core::ptr::null_mut(), DI_NORMAL);
        result.copy_from_slice(p);
        SelectObject(dc, old);
        DeleteObject(dib);
        DeleteDC(dc);
    }
    for px in result.chunks_exact_mut(4) {
        px[3] = 255;
    }
    result
}

/// `Path.Combine(dir, name)`.
fn combine(dir: &str, name: &str) -> String {
    if dir.is_empty() {
        return String::from(name);
    }
    let mut s = String::from(dir);
    if !dir.ends_with(['\\', '/', ':']) {
        s.push('\\');
    }
    s.push_str(name);
    s
}

/// `Directory.CreateDirectory`: `dir` and any missing parents.
fn create_directory(dir: &str) {
    for (i, c) in dir.char_indices() {
        if (c == '\\' || c == '/') && i > 0 {
            unsafe { CreateDirectoryW(pk::wide::to_wide(&dir[..i]).as_ptr(), core::ptr::null()) };
        }
    }
    unsafe { CreateDirectoryW(pk::wide::to_wide(dir).as_ptr(), core::ptr::null()) };
}

/// Writes `bytes` to `path` (replacing it); false if it couldn't.
fn write_file(path: &str, bytes: &[u8]) -> bool {
    let h = unsafe {
        CreateFileW(
            pk::wide::to_wide(path).as_ptr(),
            GENERIC_WRITE,
            0,
            core::ptr::null(),
            CREATE_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            core::ptr::null_mut(),
        )
    };
    if h == INVALID_HANDLE_VALUE {
        return false;
    }
    let mut done = 0;
    let mut ok = true;
    while done < bytes.len() {
        let mut written = 0u32;
        let chunk = (bytes.len() - done).min(1 << 30) as u32;
        if unsafe { WriteFile(h, bytes[done..].as_ptr(), chunk, &mut written, core::ptr::null_mut()) } == 0 || written == 0 {
            ok = false;
            break;
        }
        done += written as usize;
    }
    unsafe { CloseHandle(h) };
    ok
}

fn save_png(files: &mut Vec<String>, dir: &str, name: &str, bgra: &[u8], w: usize, h: usize) {
    let path = combine(dir, name);
    if write_file(&path, &icon::png::encode(bgra, w, h)) {
        files.push(path);
    }
}

fn fill(px: &mut [u8], bg: Bg) {
    px.copy_from_slice(&[bg.b, bg.g, bg.r, 255]);
}

/// Writes the check sheets into `dir` (created if needed); returns the files written.
///
/// Both states and low battery on dark and light taskbar colours at 16/20/24/32 px, drawn through
/// the real HICON path, at 1x and 8x (nearest neighbour); each at 16 px on its own; every frame of
/// each animation; and 240 px renders of each glyph (black on white) to compare with a browser's
/// SVG rendering.
pub fn write_check_images(dir: &str) -> Vec<String> {
    create_directory(dir);
    let mut files = Vec::new();
    const SIZES: [i32; 4] = [16, 20, 24, 32];
    let themes = [("dark", false, Bg { r: 0x1C, g: 0x1C, b: 0x1C }), ("light", true, Bg { r: 0xEE, g: 0xEE, b: 0xEE })];
    const PAD: i32 = 8;
    let max_size = 32;
    let cell_w = SIZES.iter().sum::<i32>() + PAD * (SIZES.len() as i32 + 1);
    let row_h = max_size + PAD * 2;
    let width = cell_w * themes.len() as i32;
    let height = row_h * 3;
    let mut sheet = vec![0u8; (width * height * 4) as usize];

    let states = [IconState::Connected, IconState::Disconnected, IconState::LowBattery];
    for (t, &(_, light, bg)) in themes.iter().enumerate() {
        for i in 0..(width * height) as usize {
            let x = i as i32 % width;
            if x / cell_w == t as i32 {
                fill(&mut sheet[i * 4..i * 4 + 4], bg);
            }
        }
        for (s, &state) in states.iter().enumerate() {
            // Rows: connected, disconnected, low battery.
            let mut x0 = t as i32 * cell_w + PAD;
            for size in SIZES {
                let h_icon = create_icon(state, light, size);
                let px = draw_icon_on_background(h_icon, size, bg);
                unsafe { DestroyIcon(h_icon) };
                let y0 = s as i32 * row_h + PAD + (max_size - size) / 2;
                for y in 0..size {
                    let d = (((y0 + y) * width + x0) * 4) as usize;
                    let src = (y * size * 4) as usize;
                    sheet[d..d + (size * 4) as usize].copy_from_slice(&px[src..src + (size * 4) as usize]);
                }
                x0 += size + PAD;
            }
        }
    }
    let (w, h) = (width as usize, height as usize);
    save_png(&mut files, dir, "tray-check-1x.png", &sheet, w, h);
    save_png(&mut files, dir, "tray-check-8x.png", &scale(&sheet, w, h, 8), w * 8, h * 8);

    // Each theme/state at 16 px as a standalone file.
    for &(name, light, bg) in &themes {
        for state in states {
            let h_icon = create_icon(state, light, 16);
            let px = draw_icon_on_background(h_icon, 16, bg);
            unsafe { DestroyIcon(h_icon) };
            let file = alloc::format!("icon16-{name}-{}.png", state.name().to_ascii_lowercase());
            save_png(&mut files, dir, &file, &scale(&px, 16, 16, 6), 96, 96);
        }
    }

    // Pending animation, frame by frame at 12 fps, on both taskbars, through the HICON path.
    // Per theme: connecting (1.4 s), landing after 1.4 s of connecting, disconnecting (+0.4 s
    // still pending), the low-battery ripple. Cancelling (a failed connect, after 1.4 s of
    // connecting) goes in a sheet of its own.
    {
        let ft = tunables::int(T::AnimFrameMs) as f64 / 1000.0;
        let frames = |seconds: f64| ceil(seconds / ft) as i32;
        let mut rows: Vec<AnimRow> = Vec::new();
        let mut cancel_rows: Vec<AnimRow> = Vec::new();
        for &(_, light, bg) in &themes {
            rows.push(AnimRow { light, bg, phase: AnimPhase::Connecting, frames: 17, tracer_offset: 0.0 });
            rows.push(AnimRow {
                light,
                bg,
                phase: AnimPhase::Landing,
                frames: frames(tunables::get(T::LandSeconds)) + 1,
                tracer_offset: 1.4,
            });
            rows.push(AnimRow {
                light,
                bg,
                phase: AnimPhase::Disconnecting,
                frames: frames(tunables::get(T::DisconnectSeconds) + 0.4),
                tracer_offset: 0.0,
            });
            rows.push(AnimRow {
                light,
                bg,
                phase: AnimPhase::LowBattery,
                frames: frames(tunables::get(T::LowBatteryAnimSeconds)) + 1,
                tracer_offset: 0.0,
            });
            cancel_rows.push(AnimRow {
                light,
                bg,
                phase: AnimPhase::Cancelling,
                frames: frames(tunables::get(T::CancelSeconds)) + 1,
                tracer_offset: 1.4,
            });
        }
        write_anim_sheet(dir, "anim-frames", &rows, themes[0].2, themes[1].2, &mut files);
        write_anim_sheet(dir, "anim-cancel", &cancel_rows, themes[0].2, themes[1].2, &mut files);
    }

    // Large renders for parse verification against the SVG.
    for (file, fill_glyph) in [("large-fill.png", true), ("large-line.png", false)] {
        let px = icon::render_large_glyph(fill_glyph, 240);
        let mut flat = vec![0u8; px.len()];
        for (d, s) in flat.chunks_exact_mut(4).zip(px.chunks_exact(4)) {
            let v = 255 - s[3];
            d.copy_from_slice(&[v, v, v, 255]);
        }
        save_png(&mut files, dir, file, &flat, 240, 240);
    }
    files
}

fn ceil(v: f64) -> f64 {
    quicktooth_core::math::ceil(v)
}

struct AnimRow {
    light: bool,
    bg: Bg,
    phase: AnimPhase,
    frames: i32,
    tracer_offset: f64,
}

fn write_anim_sheet(dir: &str, name: &str, rows: &[AnimRow], dark_bg: Bg, light_bg: Bg, files: &mut Vec<String>) {
    const FS: i32 = 16;
    const FPAD: i32 = 4;
    const COLS: i32 = 17;
    let ft = tunables::int(T::AnimFrameMs) as f64 / 1000.0;
    let fw = COLS * (FS + FPAD) + FPAD;
    let fh = rows.len() as i32 * (FS + FPAD) + FPAD;
    let mut strip = vec![0u8; (fw * fh * 4) as usize];
    for i in 0..(fw * fh) as usize {
        let bg = if (i as i32) / fw < fh / 2 { dark_bg } else { light_bg }; // top half dark, bottom half light
        fill(&mut strip[i * 4..i * 4 + 4], bg);
    }
    for (r, row) in rows.iter().enumerate() {
        for f in 0..row.frames.min(COLS) {
            let t = f as f64 * ft;
            let h = create_anim_frame(row.phase, t, row.tracer_offset + t, row.light, FS);
            let px = draw_icon_on_background(h, FS, row.bg);
            unsafe { DestroyIcon(h) };
            let x0 = FPAD + f * (FS + FPAD);
            let y0 = FPAD + r as i32 * (FS + FPAD);
            for y in 0..FS {
                let d = (((y0 + y) * fw + x0) * 4) as usize;
                let src = (y * FS * 4) as usize;
                strip[d..d + (FS * 4) as usize].copy_from_slice(&px[src..src + (FS * 4) as usize]);
            }
        }
    }
    let (w, h) = (fw as usize, fh as usize);
    save_png(files, dir, &alloc::format!("{name}-1x.png"), &strip, w, h);
    save_png(files, dir, &alloc::format!("{name}-6x.png"), &scale(&strip, w, h, 6), w * 6, h * 6);
}

/// Nearest-neighbour upscale by `k`.
fn scale(src: &[u8], w: usize, h: usize, k: usize) -> Vec<u8> {
    let mut dst = vec![0u8; w * k * h * k * 4];
    for y in 0..h * k {
        for x in 0..w * k {
            let s = ((y / k) * w + x / k) * 4;
            let d = (y * w * k + x) * 4;
            dst[d..d + 4].copy_from_slice(&src[s..s + 4]);
        }
    }
    dst
}
