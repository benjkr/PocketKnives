//! The captures: reading the screen, the frozen overlay with window picking, the clipboard and
//! the flash.
use crate::{
    DAEMON, FLASH_ALPHA, FLASH_CLASS, FLASH_MS, FRAME_COLOR, FRAME_W, G, HK_ESC, OVERLAY_CLASS, RC_CANCEL, RC_FAIL, RC_OK, S_FLASH, SET,
    instance, message_loop, pixels,
};
use core::ffi::c_void;
use core::ptr::{null, null_mut};
use pk::w;
use windows_sys::Win32::Foundation::{GlobalFree, HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows_sys::Win32::Graphics::Dwm::{
    DWMWA_CLOAK, DWMWA_CLOAKED, DWMWA_EXTENDED_FRAME_BOUNDS, DWMWA_TRANSITIONS_FORCEDISABLED, DwmFlush, DwmGetWindowAttribute,
    DwmSetWindowAttribute,
};
use windows_sys::Win32::Graphics::Gdi::{
    BI_RGB, BITMAPINFO, BITMAPINFOHEADER, BeginPaint, BitBlt, CAPTUREBLT, CreateCompatibleDC, CreateDIBSection, CreateSolidBrush,
    DIB_RGB_COLORS, DeleteDC, DeleteObject, EndPaint, EqualRect, GdiFlush, GetDC, GetMonitorInfoW, GetStockObject, HBITMAP, HBRUSH, HDC,
    HGDIOBJ, IntersectRect, InvalidateRect, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromPoint, OffsetRect, PAINTSTRUCT, PtInRect,
    ReleaseDC, SRCCOPY, SelectObject, UnionRect, UpdateWindow, WHITE_BRUSH,
};
use windows_sys::Win32::System::DataExchange::{CloseClipboard, EmptyClipboard, OpenClipboard, SetClipboardData};
use windows_sys::Win32::System::Memory::{GMEM_MOVEABLE, GlobalAlloc, GlobalLock, GlobalUnlock};
use windows_sys::Win32::System::Ole::CF_DIB;
use windows_sys::Win32::System::SystemInformation::GetTickCount64;
use windows_sys::Win32::System::Threading::Sleep;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    MOD_NOREPEAT, RegisterHotKey, ReleaseCapture, SetCapture, UnregisterHotKey, VK_ESCAPE,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, EnumWindows, FindWindowExW, GWL_EXSTYLE, GetClassNameW, GetCursorPos,
    GetLayeredWindowAttributes, GetSystemMetrics, GetWindowLongW, GetWindowRect, HWND_MESSAGE, IDC_ARROW, IDC_CROSS, IsIconic,
    IsWindowVisible, LWA_ALPHA, LoadCursorW, PostQuitMessage, RegisterClassW, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN,
    SM_YVIRTUALSCREEN, SW_HIDE, SW_SHOWNOACTIVATE, SetCursor, SetForegroundWindow, SetLayeredWindowAttributes, SetTimer, ShowWindow,
    USER_TIMER_MINIMUM, WA_INACTIVE, WM_ACTIVATE, WM_CAPTURECHANGED, WM_CLOSE, WM_DESTROY, WM_ERASEBKGND, WM_HOTKEY, WM_KEYDOWN,
    WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE, WM_PAINT, WM_RBUTTONDOWN, WM_SETCURSOR, WM_TIMER, WNDCLASSW, WS_EX_LAYERED,
    WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_POPUP,
};

const NO_RECT: RECT = RECT { left: 0, top: 0, right: 0, bottom: 0 };

fn dwm_set(h: HWND, attr: i32, on: bool) {
    let on = on as i32;
    unsafe { DwmSetWindowAttribute(h, attr as u32, &on as *const i32 as *const c_void, 4) };
}

fn register_class(proc_: unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT, class: *const u16, white: bool) {
    let mut wc: WNDCLASSW = unsafe { core::mem::zeroed() };
    wc.lpfnWndProc = Some(proc_);
    wc.hInstance = instance();
    if white {
        wc.hbrBackground = unsafe { GetStockObject(WHITE_BRUSH) };
    }
    wc.lpszClassName = class;
    unsafe { RegisterClassW(&wc) };
}

// -------------------------------------------------------------------------------------- flash
//
// A white, click-through layer over the captured area that fades out in ~150 ms. It appears only
// after the pixels are captured, so it never ends up in an image.

static FLASH: G<HWND> = G::new(null_mut());
static FLASH_T0: G<u64> = G::new(0);
static FLASH_CLASS_DONE: G<bool> = G::new(false);

unsafe extern "system" fn flash_proc(h: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    if msg == WM_TIMER {
        // Fade by elapsed time: timer ticks are coarse (~16-31 ms).
        let ms = unsafe { GetTickCount64() } - FLASH_T0.get();
        if ms >= FLASH_MS {
            unsafe { DestroyWindow(h) };
            return 0;
        }
        unsafe { SetLayeredWindowAttributes(h, 0, (FLASH_ALPHA * (FLASH_MS - ms) / FLASH_MS) as u8, LWA_ALPHA) };
        return 0;
    }
    if msg == WM_DESTROY && h == FLASH.get() {
        FLASH.set(null_mut());
    }
    unsafe { DefWindowProcW(h, msg, wp, lp) }
}

/// Removes any flash still fading - ours, or one from another SnipSnap process - and waits for
/// the screen to show that, so it can't end up in the next capture.
fn flash_clear() {
    let mut any = false;
    let f = FLASH.get();
    if !f.is_null() {
        unsafe { DestroyWindow(f) };
        any = true;
    }
    let mut w = null_mut();
    loop {
        w = unsafe { FindWindowExW(null_mut(), w, FLASH_CLASS, null()) };
        if w.is_null() {
            break;
        }
        if unsafe { IsWindowVisible(w) } != 0 {
            unsafe { ShowWindow(w, SW_HIDE) };
            any = true;
        }
    }
    if any {
        unsafe { DwmFlush() };
    }
}

fn flash(r: RECT) {
    if SET.get()[S_FLASH] == 0 {
        return;
    }
    if !FLASH_CLASS_DONE.get() {
        register_class(flash_proc, FLASH_CLASS, true);
        FLASH_CLASS_DONE.set(true);
    }
    flash_clear();
    let h = unsafe {
        CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
            FLASH_CLASS,
            null(),
            WS_POPUP,
            r.left,
            r.top,
            r.right - r.left,
            r.bottom - r.top,
            null_mut(),
            null_mut(),
            instance(),
            null(),
        )
    };
    if h.is_null() {
        return;
    }
    FLASH.set(h);
    dwm_set(h, DWMWA_TRANSITIONS_FORCEDISABLED, true);
    unsafe {
        SetLayeredWindowAttributes(h, 0, FLASH_ALPHA as u8, LWA_ALPHA);
        ShowWindow(h, SW_SHOWNOACTIVATE);
        UpdateWindow(h);
    }
    FLASH_T0.set(unsafe { GetTickCount64() });
    unsafe { SetTimer(h, 1, USER_TIMER_MINIMUM, None) };
}

/// One-shot runs: keeps the process alive until the flash has faded.
fn flash_wait() {
    message_loop(|| !FLASH.get().is_null());
}

// ------------------------------------------------------------------------------------- pixels

/// A screen image: a 32 bpp top-down DIB section (BGRX) selected into a memory DC, and where it
/// was on the screen, in physical pixels.
#[derive(Clone, Copy)]
struct Shot {
    dc: HDC,
    bmp: HBITMAP,
    old: HGDIOBJ,
    px: *mut u32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

const NO_SHOT: Shot = Shot { dc: null_mut(), bmp: null_mut(), old: null_mut(), px: null_mut(), x: 0, y: 0, w: 0, h: 0 };

impl Shot {
    fn len(&self) -> usize {
        self.w as usize * self.h as usize
    }
    /// The pixels (`shot_alloc` succeeded).
    fn px(&self) -> &[u32] {
        unsafe { core::slice::from_raw_parts(self.px, self.len()) }
    }
    fn px_mut(&mut self) -> &mut [u32] {
        unsafe { core::slice::from_raw_parts_mut(self.px, self.len()) }
    }
}

fn vscreen() -> RECT {
    unsafe {
        let left = GetSystemMetrics(SM_XVIRTUALSCREEN);
        let top = GetSystemMetrics(SM_YVIRTUALSCREEN);
        RECT { left, top, right: left + GetSystemMetrics(SM_CXVIRTUALSCREEN), bottom: top + GetSystemMetrics(SM_CYVIRTUALSCREEN) }
    }
}

/// The monitor containing a point.
fn monitor_at(p: POINT) -> RECT {
    let mut mi: MONITORINFO = unsafe { core::mem::zeroed() };
    mi.cbSize = size_of::<MONITORINFO>() as u32;
    if unsafe { GetMonitorInfoW(MonitorFromPoint(p, MONITOR_DEFAULTTONEAREST), &mut mi) } == 0 {
        return vscreen();
    }
    mi.rcMonitor
}

/// The monitor under the mouse cursor.
pub fn cursor_monitor() -> RECT {
    let mut p = POINT { x: 0, y: 0 };
    unsafe { GetCursorPos(&mut p) };
    monitor_at(p)
}

// ----------------------------------------------------------------------------- window picking
//
// The window list is taken when the screen is frozen, top of the z-order first, so hovering
// always matches the frozen image. Each entry is the window's visible frame (DWM extended frame
// bounds: no invisible resize border or shadow).

const MAX_WINS: usize = 512;
#[derive(Clone, Copy)]
struct Win {
    r: RECT,
    desktop: bool,
}
static WINS: G<[Win; MAX_WINS]> = G::new([Win { r: NO_RECT, desktop: false }; MAX_WINS]);
static NWINS: G<usize> = G::new(0);

unsafe extern "system" fn collect_window(w: HWND, _: LPARAM) -> i32 {
    let n = NWINS.get();
    if n >= MAX_WINS || unsafe { IsWindowVisible(w) == 0 || IsIconic(w) != 0 } {
        return 1;
    }
    let ex = unsafe { GetWindowLongW(w, GWL_EXSTYLE) } as u32;
    if ex & WS_EX_TRANSPARENT != 0 {
        return 1; // click-through overlays
    }
    let mut cloaked = 0u32;
    if unsafe { DwmGetWindowAttribute(w, DWMWA_CLOAKED as u32, &mut cloaked as *mut u32 as *mut c_void, 4) } >= 0 && cloaked != 0 {
        return 1; // other desktops, suspended apps
    }
    let (mut a, mut fl) = (0u8, 0u32);
    if ex & WS_EX_LAYERED != 0
        && unsafe { GetLayeredWindowAttributes(w, null_mut(), &mut a, &mut fl) } != 0
        && fl & LWA_ALPHA != 0
        && a == 0
    {
        return 1; // fully transparent
    }
    let mut r = NO_RECT;
    if unsafe { DwmGetWindowAttribute(w, DWMWA_EXTENDED_FRAME_BOUNDS as u32, &mut r as *mut RECT as *mut c_void, 16) } < 0
        && unsafe { GetWindowRect(w, &mut r) } == 0
    {
        return 1;
    }
    if r.right <= r.left || r.bottom <= r.top {
        return 1;
    }
    // The class name with its zero: both desktop classes have 7 letters.
    const PROGMAN: [u16; 8] = crate::wide(b"Progman\0");
    const WORKERW: [u16; 8] = crate::wide(b"WorkerW\0");
    let mut cls = [0u16; 16];
    unsafe { GetClassNameW(w, cls.as_mut_ptr(), 16) };
    let desktop = cls[..8] == PROGMAN || cls[..8] == WORKERW;
    // EnumWindows calls back on this thread, one window at a time.
    unsafe { (*WINS.ptr())[n] = Win { r, desktop } };
    NWINS.set(n + 1);
    1
}

fn collect_windows() {
    NWINS.set(0);
    unsafe { EnumWindows(Some(collect_window), 0) };
}

/// What to capture at a screen point: the topmost window there, or the whole monitor over the
/// bare desktop. Clipped to the screen.
fn pick_at(p: POINT) -> RECT {
    let mut r = monitor_at(p);
    let v = vscreen();
    // Read only while no window list is being taken.
    let wins = unsafe { &*WINS.ptr() };
    for win in &wins[..NWINS.get()] {
        if unsafe { PtInRect(&win.r, p) } != 0 {
            if !win.desktop {
                r = win.r;
            }
            break;
        }
    }
    let mut out = NO_RECT;
    if unsafe { IntersectRect(&mut out, &r, &v) } == 0 {
        return monitor_at(p);
    }
    out
}

fn shot_alloc(s: &mut Shot, x: i32, y: i32, w: i32, h: i32) -> bool {
    let mut bi: BITMAPINFO = unsafe { core::mem::zeroed() };
    bi.bmiHeader.biSize = size_of::<BITMAPINFOHEADER>() as u32;
    bi.bmiHeader.biWidth = w;
    bi.bmiHeader.biHeight = -h;
    bi.bmiHeader.biPlanes = 1;
    bi.bmiHeader.biBitCount = 32;
    bi.bmiHeader.biCompression = BI_RGB;
    (s.x, s.y, s.w, s.h) = (x, y, w, h);
    let mut bits: *mut c_void = null_mut();
    s.dc = unsafe { CreateCompatibleDC(null_mut()) };
    s.bmp = unsafe { CreateDIBSection(null_mut(), &bi, DIB_RGB_COLORS, &mut bits, null_mut(), 0) };
    s.px = bits as *mut u32;
    if s.dc.is_null() || s.bmp.is_null() {
        return false;
    }
    s.old = unsafe { SelectObject(s.dc, s.bmp) };
    true
}

fn shot_free(s: &mut Shot) {
    unsafe {
        if !s.dc.is_null() {
            if !s.old.is_null() {
                SelectObject(s.dc, s.old);
            }
            DeleteDC(s.dc);
        }
        if !s.bmp.is_null() {
            DeleteObject(s.bmp);
        }
    }
    *s = NO_SHOT;
}

/// Grabs a screen rectangle (physical pixels).
fn grab(s: &mut Shot, v: RECT) -> bool {
    if v.right <= v.left || v.bottom <= v.top {
        return false;
    }
    flash_clear();
    let scr = unsafe { GetDC(null_mut()) };
    let ok = !scr.is_null()
        && shot_alloc(s, v.left, v.top, v.right - v.left, v.bottom - v.top)
        && unsafe { BitBlt(s.dc, 0, 0, s.w, s.h, scr, v.left, v.top, SRCCOPY | CAPTUREBLT) } != 0;
    if !scr.is_null() {
        unsafe { ReleaseDC(null_mut(), scr) };
    }
    if !ok {
        shot_free(s);
        return false;
    }
    unsafe { GdiFlush() };
    true
}

fn open_clipboard(owner: HWND) -> bool {
    for _ in 0..20 {
        if unsafe { OpenClipboard(owner) } != 0 {
            return true;
        }
        unsafe { Sleep(10) };
    }
    false
}

/// Copies a sub-rectangle of a shot to the clipboard as a 24 bpp bottom-up DIB.
fn to_clipboard(owner: HWND, s: &Shot, rx: i32, ry: i32, w: i32, h: i32) -> bool {
    const HEADER: usize = size_of::<BITMAPINFOHEADER>();
    let stride = pixels::dib24_stride(w);
    let bytes = stride * h as usize;
    let g = unsafe { GlobalAlloc(GMEM_MOVEABLE, HEADER + bytes) };
    if g.is_null() {
        return false;
    }
    let bh = unsafe { GlobalLock(g) } as *mut BITMAPINFOHEADER;
    if bh.is_null() {
        unsafe { GlobalFree(g) };
        return false;
    }
    let mut hdr: BITMAPINFOHEADER = unsafe { core::mem::zeroed() };
    hdr.biSize = HEADER as u32;
    hdr.biWidth = w;
    hdr.biHeight = h; // positive = bottom-up
    hdr.biPlanes = 1;
    hdr.biBitCount = 24;
    hdr.biCompression = BI_RGB;
    hdr.biSizeImage = bytes as u32;
    unsafe { bh.write_unaligned(hdr) };
    // The block holds the header and then `bytes` of pixel rows.
    let dst = unsafe { core::slice::from_raw_parts_mut((bh as *mut u8).add(HEADER), bytes) };
    pixels::dib24(s.px(), s.w, rx, ry, w, h, dst);
    unsafe { GlobalUnlock(g) };

    let mut ok = false;
    if open_clipboard(owner) {
        unsafe {
            EmptyClipboard();
            ok = !SetClipboardData(CF_DIB as u32, g).is_null();
            CloseClipboard();
        }
    }
    if !ok {
        unsafe { GlobalFree(g) };
    }
    ok
}

// -------------------------------------------------------------------------------- full screen

pub fn snip_area(owner: HWND, r: RECT) -> i32 {
    let mut s = NO_SHOT;
    if !grab(&mut s, r) {
        return RC_FAIL;
    }
    flash(r);
    let ok = to_clipboard(owner, &s, 0, 0, s.w, s.h);
    if DAEMON.get() {
        shot_free(&mut s); // a one-shot run just exits
    }
    if ok { RC_OK } else { RC_FAIL }
}

// ----------------------------------------------------------------- rectangle and window pick
//
// Both freeze all monitors and show the frozen image dimmed. Rectangle: drag to select. Window:
// the window under the cursor shows at full brightness with a border; click to take it. Either
// way the result is cropped from the frozen image, so copying after mouse-up takes only a few ms.

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Rect,
    Window,
}

static SHOT: G<Shot> = G::new(NO_SHOT);
static DIM: G<Shot> = G::new(NO_SHOT);
static OWNER: G<HWND> = G::new(null_mut());
static FRAME: G<HBRUSH> = G::new(null_mut());
static DRAG: G<bool> = G::new(false);
static PRESSED: G<bool> = G::new(false);
static HAS_HOVER: G<bool> = G::new(false);
/// The hovered window, overlay coordinates.
static HOVER: G<RECT> = G::new(NO_RECT);
static MODE: G<Mode> = G::new(Mode::Rect);
/// The drag's anchor and its current corner, overlay coordinates.
static ANCHOR: G<POINT> = G::new(POINT { x: 0, y: 0 });
static CORNER: G<POINT> = G::new(POINT { x: 0, y: 0 });
/// The session's exit code; negative while it runs.
static RESULT: G<i32> = G::new(RC_CANCEL);
static OVERLAY_CLASS_DONE: G<bool> = G::new(false);

fn cloak(h: HWND, on: bool) {
    dwm_set(h, DWMWA_CLOAK, on);
}

fn sel_rect() -> RECT {
    let (a, c) = (ANCHOR.get(), CORNER.get());
    RECT {
        left: a.x.min(c.x),
        top: a.y.min(c.y),
        right: a.x.max(c.x) + 1, // inclusive of both corners
        bottom: a.y.max(c.y) + 1,
    }
}

/// The mouse position in a mouse message's `lp` (signed: left of or above the overlay is < 0).
fn mouse(lp: LPARAM) -> (i32, i32) {
    (lp as u16 as i16 as i32, (lp >> 16) as u16 as i16 as i32)
}

fn track(lp: LPARAM) {
    let (x, y) = mouse(lp);
    let s = SHOT.get();
    let clamp = |v: i32, n: i32| {
        if v < 0 {
            0
        } else if v >= n {
            n - 1
        } else {
            v
        }
    };
    CORNER.set(POINT { x: clamp(x, s.w), y: clamp(y, s.h) });
}

/// Points the window pick at the window under an overlay point.
fn hover_at(h: HWND, x: i32, y: i32) {
    let s = SHOT.get();
    let mut r = pick_at(POINT { x: x + s.x, y: y + s.y });
    unsafe { OffsetRect(&mut r, -s.x, -s.y) }; // screen -> overlay coordinates
    let old = HOVER.get();
    if HAS_HOVER.get() && unsafe { EqualRect(&r, &old) } != 0 {
        return;
    }
    if !h.is_null() && HAS_HOVER.get() {
        unsafe { InvalidateRect(h, &old, 0) };
    }
    HOVER.set(r);
    HAS_HOVER.set(true);
    if !h.is_null() {
        unsafe { InvalidateRect(h, &r, 0) };
    }
}

/// Copies an overlay-coordinate area of the frozen image, with the flash.
fn take(h: HWND, r: RECT) -> i32 {
    let s = SHOT.get();
    cloak(h, true); // vanish first, copy after
    let mut sr = r;
    unsafe { OffsetRect(&mut sr, s.x, s.y) }; // overlay -> screen coordinates
    flash(sr);
    if to_clipboard(OWNER.get(), &s, r.left, r.top, r.right - r.left, r.bottom - r.top) { RC_OK } else { RC_FAIL }
}

/// Ends the session; cloaking makes the overlay vanish at once (~0.2 ms vs ~7 ms for a real
/// hide), the slower cleanup happens after.
fn finish(h: HWND, result: i32) {
    if RESULT.get() >= 0 {
        return;
    }
    cloak(h, true);
    RESULT.set(result);
}

fn cursor(mode: Mode) {
    unsafe { SetCursor(LoadCursorW(null_mut(), if mode == Mode::Window { IDC_ARROW } else { IDC_CROSS })) };
}

unsafe extern "system" fn overlay_proc(h: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    let window = MODE.get() == Mode::Window;
    match msg {
        WM_ERASEBKGND => return 1,
        WM_SETCURSOR => {
            cursor(MODE.get());
            return 1;
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = unsafe { core::mem::zeroed() };
            let dc = unsafe { BeginPaint(h, &mut ps) };
            let (dim, shot, sel, hover) = (DIM.get(), SHOT.get(), sel_rect(), HOVER.get());
            if !dim.dc.is_null() {
                let lit = if !window {
                    if DRAG.get() { Some(&sel) } else { None }
                } else if HAS_HOVER.get() {
                    Some(&hover)
                } else {
                    None
                };
                pixels::paint_backdrop(dc, ps.rcPaint, lit, dim.dc, shot.dc, FRAME.get(), FRAME_W);
            }
            unsafe { EndPaint(h, &ps) };
        }
        WM_LBUTTONDOWN => {
            if RESULT.get() >= 0 {
                return 0;
            }
            if window {
                let (x, y) = mouse(lp);
                hover_at(h, x, y);
                PRESSED.set(true);
            } else {
                track(lp);
                ANCHOR.set(CORNER.get());
                DRAG.set(true);
            }
            unsafe { SetCapture(h) };
        }
        WM_MOUSEMOVE => {
            if window {
                if !PRESSED.get() {
                    let (x, y) = mouse(lp);
                    hover_at(h, x, y);
                }
            } else if DRAG.get() {
                let a = sel_rect();
                track(lp);
                let b = sel_rect();
                let mut u = NO_RECT;
                unsafe {
                    UnionRect(&mut u, &a, &b);
                    InvalidateRect(h, &u, 0);
                }
            }
        }
        WM_LBUTTONUP => {
            if window && PRESSED.get() {
                // Take the window on release, so the release doesn't land in the app underneath
                // once the overlay is gone.
                PRESSED.set(false);
                unsafe { ReleaseCapture() };
                if HAS_HOVER.get() {
                    finish(h, take(h, HOVER.get()));
                }
            } else if DRAG.get() {
                track(lp);
                DRAG.set(false);
                unsafe { ReleaseCapture() };
                let (a, c) = (ANCHOR.get(), CORNER.get());
                if a.x == c.x && a.y == c.y {
                    // A plain click: keep waiting.
                    unsafe { InvalidateRect(h, null(), 0) };
                    return 0;
                }
                finish(h, take(h, sel_rect()));
            }
        }
        WM_RBUTTONDOWN => finish(h, RC_CANCEL),
        WM_KEYDOWN => {
            if wp == VK_ESCAPE as usize {
                finish(h, RC_CANCEL);
            }
        }
        WM_HOTKEY => {
            if wp == HK_ESC as usize {
                finish(h, RC_CANCEL);
            }
        }
        WM_CAPTURECHANGED => {
            if lp as HWND != h && (DRAG.get() || PRESSED.get()) {
                DRAG.set(false);
                PRESSED.set(false);
                unsafe { InvalidateRect(h, null(), 0) };
            }
        }
        WM_ACTIVATE => {
            if wp as u16 as u32 == WA_INACTIVE {
                finish(h, RC_CANCEL);
            }
        }
        WM_CLOSE => finish(h, RC_CANCEL),
        _ => return unsafe { DefWindowProcW(h, msg, wp, lp) },
    }
    0
}

fn overlay_create(s: &Shot) -> HWND {
    if !OVERLAY_CLASS_DONE.get() {
        register_class(overlay_proc, OVERLAY_CLASS, false);
        FRAME.set(unsafe { CreateSolidBrush(FRAME_COLOR) });
        OVERLAY_CLASS_DONE.set(true);
    }
    let h = unsafe {
        CreateWindowExW(
            WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
            OVERLAY_CLASS,
            w!("SnipSnap"),
            WS_POPUP,
            s.x,
            s.y,
            s.w,
            s.h,
            null_mut(),
            null_mut(),
            instance(),
            null(),
        )
    };
    if !h.is_null() {
        dwm_set(h, DWMWA_TRANSITIONS_FORCEDISABLED, true);
    }
    h
}

pub fn snip_select(owner: HWND, mode: Mode) -> i32 {
    let mut shot = NO_SHOT;
    let mut dim = NO_SHOT;
    if !grab(&mut shot, vscreen()) {
        return RC_FAIL; // freeze all monitors
    }
    if mode == Mode::Window {
        collect_windows(); // before our overlay exists
    }

    // The dimmed copy of the frozen screen for the backdrop.
    if !shot_alloc(&mut dim, shot.x, shot.y, shot.w, shot.h) {
        shot_free(&mut dim);
        shot_free(&mut shot);
        return RC_FAIL;
    }
    pixels::dim(shot.px(), dim.px_mut());

    let h = overlay_create(&shot);
    if h.is_null() {
        shot_free(&mut dim);
        shot_free(&mut shot);
        return RC_FAIL;
    }
    // For the overlay's messages, which paint and pick from these until it's destroyed. (It gets
    // none that need them while it's created.)
    SHOT.set(shot);
    DIM.set(dim);

    OWNER.set(owner);
    MODE.set(mode);
    DRAG.set(false);
    PRESSED.set(false);
    HAS_HOVER.set(false);
    RESULT.set(-1);
    if mode == Mode::Window {
        // Light up the window under the cursor now.
        let mut cur = POINT { x: 0, y: 0 };
        unsafe { GetCursorPos(&mut cur) };
        hover_at(null_mut(), cur.x - shot.x, cur.y - shot.y);
    }
    // Paint while cloaked so the first visible frame is complete. Showing without activation
    // first, then taking focus, makes it visible ~20 ms sooner than a plain SW_SHOW.
    cloak(h, true);
    unsafe {
        ShowWindow(h, SW_SHOWNOACTIVATE);
        InvalidateRect(h, null(), 0);
        UpdateWindow(h);
    }
    cloak(h, false);
    unsafe { SetForegroundWindow(h) };
    cursor(mode);
    // Esc works even if Windows refused to give the overlay keyboard focus.
    unsafe { RegisterHotKey(h, HK_ESC, MOD_NOREPEAT, VK_ESCAPE as u32) };

    let quit = message_loop(|| RESULT.get() < 0);
    if quit.is_some() {
        finish(h, RC_CANCEL);
    }
    unsafe { UnregisterHotKey(h, HK_ESC) };
    if !DAEMON.get() {
        return RESULT.get(); // a one-shot run just exits
    }

    unsafe { DestroyWindow(h) }; // the copy is done; nobody waits for this
    shot_free(&mut dim);
    shot_free(&mut shot);
    if let Some(code) = quit {
        unsafe { PostQuitMessage(code) };
    }
    RESULT.get()
}

/// `rect`, `window` or `full` once (by its first letter), then exit.
pub fn one_shot(first: u8) -> i32 {
    // A message-only window to own the clipboard data.
    let owner = unsafe { CreateWindowExW(0, w!("STATIC"), null(), 0, 0, 0, 0, 0, HWND_MESSAGE, null_mut(), null_mut(), null()) };
    let rc = match first {
        b'r' => snip_select(owner, Mode::Rect),
        b'w' => snip_select(owner, Mode::Window),
        _ => snip_area(owner, cursor_monitor()),
    };
    flash_wait(); // the image is already on the clipboard
    rc
}
