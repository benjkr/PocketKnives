//! Direct2D + DirectWrite + UpdateLayeredWindow UI for SnipSnap Recording:
//! - Border overlay (`B2` Corner Lock-In: muted red + 14px corner ticks -> 340ms bright `#e81123`)
//! - Straddling bottom-center pill (`Armed D` -> `S2` 340ms calm dissolve -> `V3 + V1 stop`)
//! Both windows set `WDA_EXCLUDEFROMCAPTURE` so nothing SnipSnap draws appears in the video.
use crate::ipc::Quality;
use alloc::format;
use alloc::vec::Vec;
use core::ffi::c_void;
use core::ptr::{null, null_mut};
use pk::wide::to_wide;
use windows::Win32::Foundation::RECT as D2dRect;
use windows::Win32::Graphics::Direct2D::Common::{D2D_RECT_F, D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_COLOR_F, D2D1_PIXEL_FORMAT};
use windows::Win32::Graphics::Direct2D::{
    D2D1_DRAW_TEXT_OPTIONS_NONE, D2D1_ELLIPSE, D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1_RENDER_TARGET_PROPERTIES,
    D2D1_RENDER_TARGET_TYPE_DEFAULT, D2D1_ROUNDED_RECT, D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE, D2D1CreateFactory, ID2D1DCRenderTarget,
    ID2D1Factory,
};
use windows::Win32::Graphics::DirectWrite::{
    DWRITE_FACTORY_TYPE_SHARED, DWRITE_FONT_FEATURE, DWRITE_FONT_FEATURE_TAG_TABULAR_FIGURES, DWRITE_FONT_STRETCH_NORMAL,
    DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_WEIGHT_NORMAL, DWRITE_FONT_WEIGHT_SEMI_BOLD, DWRITE_PARAGRAPH_ALIGNMENT_CENTER,
    DWRITE_TEXT_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_LEADING, DWRITE_TEXT_RANGE, DWRITE_WORD_WRAPPING_NO_WRAP, DWriteCreateFactory,
    IDWriteFactory, IDWriteTextFormat,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::Graphics::Gdi::HDC as WinHdc;
use windows_core::PCWSTR;
use windows_numerics::Vector2;
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, SIZE, WPARAM};
use windows_sys::Win32::Graphics::Gdi::{
    AC_SRC_ALPHA, AC_SRC_OVER, BI_RGB, BITMAPINFO, BITMAPINFOHEADER, BLENDFUNCTION, CreateCompatibleDC, CreateDIBSection, DIB_RGB_COLORS,
    DeleteDC, DeleteObject, GetMonitorInfoW, HBITMAP, HDC, HGDIOBJ, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromPoint, SelectObject,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::HiDpi::GetDpiForWindow;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{TME_LEAVE, TRACKMOUSEEVENT, TrackMouseEvent};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, GWLP_USERDATA, GetWindowLongPtrW, HWND_TOPMOST, IDC_ARROW, IDC_HAND, LoadCursorW,
    MA_NOACTIVATE, RegisterClassW, SW_HIDE, SW_SHOWNOACTIVATE, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SetCursor,
    SetWindowDisplayAffinity, SetWindowLongPtrW, SetWindowPos, ShowWindow, ULW_ALPHA, UpdateLayeredWindow, WDA_EXCLUDEFROMCAPTURE,
    WM_LBUTTONUP, WM_MOUSEACTIVATE, WM_MOUSEMOVE, WM_SETCURSOR, WNDCLASSW, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW,
    WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_POPUP,
};

const WM_MOUSELEAVE: u32 = 0x02A3;

const BORDER_CLASS: *const u16 = pk::w!("SnipSnapRecBorder");
const PILL_CLASS: *const u16 = pk::w!("SnipSnapRecPill");

/// Transition duration (`S2` + `B2`), milliseconds.
pub const TRANSITION_MS: f32 = 340.0;

// Layout constants in logical DIPs (matching the locked D + V3/V1 design).
const PILL_H_ARMED: f32 = 26.0;
const PILL_H_REC: f32 = 25.0;
const PILL_W_ARMED: f32 = 196.0;
const PILL_W_REC: f32 = 132.0;
const PILL_PAD_X: f32 = 12.0;
const PILL_PAD_TOP: f32 = 12.0;
const PILL_PAD_BOT: f32 = 10.0;
const BTN_D: f32 = 20.0;
const BTN_RIGHT_PAD: f32 = 3.0;
const BORDER_W_DIP: f32 = 2.0;
const TICK_DIP: f32 = 14.0;

/// Hit zones on the pill window (in logical DIPs relative to the pill's left edge).
const ITEM_RANGES: [(f32, f32); 4] = [
    (6.0, 38.0),    // 480
    (38.0, 70.0),   // 720
    (70.0, 108.0),  // 1080
    (108.0, 158.0), // Native
];

#[derive(Clone, Copy)]
pub struct UiState {
    pub target: RECT,
    pub quality: Quality,
    /// `false` = Armed, `true` = Recording (or transitioning into Recording).
    pub recording: bool,
    /// `0.0` (Armed) -> `1.0` (Recording), linear elapsed ratio over `TRANSITION_MS`.
    pub trans_t: f32,
    pub elapsed_ms: u64,
    pub bytes: u64,
    pub btn_hover: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PillAction {
    None,
    SelectQuality(Quality),
    ToggleRecord,
}

/// Evaluates CSS `cubic-bezier(0.22, 1.0, 0.36, 1.0)` accurately with 5 Newton steps.
fn ease_out_expo_like(x: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let (x1, y1, x2, y2) = (0.22f32, 1.0f32, 0.36f32, 1.0f32);
    let mut t = x;
    for _ in 0..6 {
        let u = 1.0 - t;
        let bx = 3.0 * u * u * t * x1 + 3.0 * u * t * t * x2 + t * t * t;
        let dbx = 3.0 * u * u * x1 + 6.0 * u * t * (x2 - x1) + 3.0 * t * t * (1.0 - x2);
        if dbx.abs() < 1e-5 {
            break;
        }
        t = (t - (bx - x) / dbx).clamp(0.0, 1.0);
    }
    let u = 1.0 - t;
    (3.0 * u * u * t * y1 + 3.0 * u * t * t * y2 + t * t * t).clamp(0.0, 1.0)
}

fn color(r: u8, g: u8, b: u8, a: f32) -> D2D1_COLOR_F {
    D2D1_COLOR_F { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: a.clamp(0.0, 1.0) }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

struct LayeredSurface {
    dc: HDC,
    bmp: HBITMAP,
    old: HGDIOBJ,
    w: i32,
    h: i32,
}

impl LayeredSurface {
    fn new() -> Self {
        Self { dc: null_mut(), bmp: null_mut(), old: null_mut(), w: 0, h: 0 }
    }

    fn ensure(&mut self, w: i32, h: i32) -> bool {
        let w = w.max(1);
        let h = h.max(1);
        if self.w == w && self.h == h && !self.dc.is_null() {
            return true;
        }
        self.free();
        let mut bi: BITMAPINFO = unsafe { core::mem::zeroed() };
        bi.bmiHeader.biSize = size_of::<BITMAPINFOHEADER>() as u32;
        bi.bmiHeader.biWidth = w;
        bi.bmiHeader.biHeight = -h;
        bi.bmiHeader.biPlanes = 1;
        bi.bmiHeader.biBitCount = 32;
        bi.bmiHeader.biCompression = BI_RGB;
        let mut bits: *mut c_void = null_mut();
        let dc = unsafe { CreateCompatibleDC(null_mut()) };
        let bmp = unsafe { CreateDIBSection(null_mut(), &bi, DIB_RGB_COLORS, &mut bits, null_mut(), 0) };
        if dc.is_null() || bmp.is_null() {
            if !dc.is_null() {
                unsafe { DeleteDC(dc) };
            }
            if !bmp.is_null() {
                unsafe { DeleteObject(bmp) };
            }
            return false;
        }
        let old = unsafe { SelectObject(dc, bmp) };
        self.dc = dc;
        self.bmp = bmp;
        self.old = old;
        self.w = w;
        self.h = h;
        true
    }

    fn present(&self, hwnd: HWND, x: i32, y: i32) {
        let dst = POINT { x, y };
        let sz = SIZE { cx: self.w, cy: self.h };
        let src = POINT { x: 0, y: 0 };
        let blend = BLENDFUNCTION { BlendOp: AC_SRC_OVER as u8, BlendFlags: 0, SourceConstantAlpha: 255, AlphaFormat: AC_SRC_ALPHA as u8 };
        unsafe {
            UpdateLayeredWindow(hwnd, null_mut(), &dst, &sz, self.dc, &src, 0, &blend, ULW_ALPHA);
        }
    }

    fn free(&mut self) {
        unsafe {
            if !self.dc.is_null() {
                if !self.old.is_null() {
                    SelectObject(self.dc, self.old);
                }
                DeleteDC(self.dc);
            }
            if !self.bmp.is_null() {
                DeleteObject(self.bmp);
            }
        }
        self.dc = null_mut();
        self.bmp = null_mut();
        self.old = null_mut();
        self.w = 0;
        self.h = 0;
    }
}

impl Drop for LayeredSurface {
    fn drop(&mut self) {
        self.free();
    }
}

/// Callback slot stored in `GWLP_USERDATA` of the pill window.
struct PillCallback {
    on_action: fn(*mut c_void, PillAction),
    on_hover: fn(*mut c_void, bool),
    ctx: *mut c_void,
    scale: f32,
    win_w_dip: f32,
    recording: bool,
    over_interactive: bool,
    tracking_leave: bool,
}

unsafe extern "system" fn border_proc(h: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    unsafe { DefWindowProcW(h, msg, wp, lp) }
}

unsafe extern "system" fn pill_proc(h: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    let cb_ptr = unsafe { GetWindowLongPtrW(h, GWLP_USERDATA) } as *mut PillCallback;
    match msg {
        WM_MOUSEACTIVATE => return MA_NOACTIVATE as LRESULT,
        WM_SETCURSOR => {
            let hand = !cb_ptr.is_null() && unsafe { (*cb_ptr).over_interactive };
            unsafe { SetCursor(LoadCursorW(null_mut(), if hand { IDC_HAND } else { IDC_ARROW })) };
            return 1;
        }
        WM_MOUSELEAVE => {
            if !cb_ptr.is_null() {
                let cb = unsafe { &mut *cb_ptr };
                cb.tracking_leave = false;
                cb.over_interactive = false;
                (cb.on_hover)(cb.ctx, false);
            }
            return 0;
        }
        WM_MOUSEMOVE | WM_LBUTTONUP => {
            if !cb_ptr.is_null() {
                let cb = unsafe { &mut *cb_ptr };
                if msg == WM_MOUSEMOVE && !cb.tracking_leave {
                    let mut tme =
                        TRACKMOUSEEVENT { cbSize: size_of::<TRACKMOUSEEVENT>() as u32, dwFlags: TME_LEAVE, hwndTrack: h, dwHoverTime: 0 };
                    if unsafe { TrackMouseEvent(&mut tme) } != 0 {
                        cb.tracking_leave = true;
                    }
                }
                let x = (lp as u16 as i16 as f32) / cb.scale;
                let y = ((lp >> 16) as u16 as i16 as f32) / cb.scale;
                let pill_w = if cb.recording { PILL_W_REC } else { PILL_W_ARMED };
                let pill_h = if cb.recording { PILL_H_REC } else { PILL_H_ARMED };
                let pill_left = (cb.win_w_dip - pill_w) * 0.5;
                let pill_top = PILL_PAD_TOP;
                let rx = x - pill_left;
                let ry = y - pill_top;

                let in_y = ry >= -4.0 && ry <= pill_h + 4.0;
                let btn_cx = pill_w - BTN_RIGHT_PAD - BTN_D * 0.5;
                let btn_cy = pill_h * 0.5;
                let dx = rx - btn_cx;
                let dy = ry - btn_cy;
                let on_btn = dx * dx + dy * dy <= (BTN_D * 0.6) * (BTN_D * 0.6);

                let mut action = PillAction::None;
                if on_btn {
                    action = PillAction::ToggleRecord;
                } else if !cb.recording && in_y {
                    for (i, &(l, r)) in ITEM_RANGES.iter().enumerate() {
                        if rx >= l && rx <= r {
                            action = PillAction::SelectQuality(Quality::from_index(i));
                            break;
                        }
                    }
                }
                cb.over_interactive = action != PillAction::None;
                if msg == WM_MOUSEMOVE {
                    (cb.on_hover)(cb.ctx, on_btn);
                } else if action != PillAction::None {
                    (cb.on_action)(cb.ctx, action);
                }
                return 0;
            }
        }
        _ => {}
    }
    unsafe { DefWindowProcW(h, msg, wp, lp) }
}

pub struct OverlayUi {
    _d2d: ID2D1Factory,
    rt: ID2D1DCRenderTarget,
    dwrite: IDWriteFactory,
    fmt_item: IDWriteTextFormat,
    fmt_item_bold: IDWriteTextFormat,
    fmt_badge: IDWriteTextFormat,
    fmt_timer: IDWriteTextFormat,
    fmt_size: IDWriteTextFormat,
    border_hwnd: HWND,
    pill_hwnd: HWND,
    border_surf: LayeredSurface,
    pill_surf: LayeredSurface,
    pill_cb: alloc::boxed::Box<PillCallback>,
    dpi: u32,
    visible: bool,
}

impl OverlayUi {
    pub fn new(on_action: fn(*mut c_void, PillAction), on_hover: fn(*mut c_void, bool), ctx: *mut c_void) -> Option<Self> {
        let d2d: ID2D1Factory = unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None) }.ok()?;
        let rt_props = D2D1_RENDER_TARGET_PROPERTIES {
            r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
            pixelFormat: D2D1_PIXEL_FORMAT { format: DXGI_FORMAT_B8G8R8A8_UNORM, alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED },
            dpiX: 96.0,
            dpiY: 96.0,
            ..Default::default()
        };
        let rt = unsafe { d2d.CreateDCRenderTarget(&rt_props) }.ok()?;
        unsafe { rt.SetTextAntialiasMode(D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE) };
        let dwrite: IDWriteFactory = unsafe { DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED) }.ok()?;

        let font = to_wide("Segoe UI Variable Text");
        let locale = to_wide("en-us");
        let make_fmt = |size: f32, weight, align| -> Option<IDWriteTextFormat> {
            let f = unsafe {
                dwrite.CreateTextFormat(
                    PCWSTR(font.as_ptr()),
                    None,
                    weight,
                    DWRITE_FONT_STYLE_NORMAL,
                    DWRITE_FONT_STRETCH_NORMAL,
                    size,
                    PCWSTR(locale.as_ptr()),
                )
            }
            .ok()?;
            unsafe {
                let _ = f.SetTextAlignment(align);
                let _ = f.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER);
                let _ = f.SetWordWrapping(DWRITE_WORD_WRAPPING_NO_WRAP);
            }
            Some(f)
        };

        let fmt_item = make_fmt(11.5, DWRITE_FONT_WEIGHT_NORMAL, DWRITE_TEXT_ALIGNMENT_CENTER)?;
        let fmt_item_bold = make_fmt(11.5, DWRITE_FONT_WEIGHT_SEMI_BOLD, DWRITE_TEXT_ALIGNMENT_CENTER)?;
        let fmt_badge = make_fmt(8.0, DWRITE_FONT_WEIGHT_NORMAL, DWRITE_TEXT_ALIGNMENT_CENTER)?;
        let fmt_timer = make_fmt(11.5, DWRITE_FONT_WEIGHT_SEMI_BOLD, DWRITE_TEXT_ALIGNMENT_LEADING)?;
        let fmt_size = make_fmt(10.0, DWRITE_FONT_WEIGHT_NORMAL, DWRITE_TEXT_ALIGNMENT_LEADING)?;

        let hinst = unsafe { GetModuleHandleW(null()) };
        let mut wc_b: WNDCLASSW = unsafe { core::mem::zeroed() };
        wc_b.lpfnWndProc = Some(border_proc);
        wc_b.hInstance = hinst;
        wc_b.lpszClassName = BORDER_CLASS;
        unsafe { RegisterClassW(&wc_b) };

        let mut wc_p: WNDCLASSW = unsafe { core::mem::zeroed() };
        wc_p.lpfnWndProc = Some(pill_proc);
        wc_p.hInstance = hinst;
        wc_p.lpszClassName = PILL_CLASS;
        unsafe { RegisterClassW(&wc_p) };

        let border_hwnd = unsafe {
            CreateWindowExW(
                WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
                BORDER_CLASS,
                null(),
                WS_POPUP,
                0,
                0,
                1,
                1,
                null_mut(),
                null_mut(),
                hinst,
                null(),
            )
        };
        let pill_hwnd = unsafe {
            CreateWindowExW(
                WS_EX_LAYERED | WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
                PILL_CLASS,
                null(),
                WS_POPUP,
                0,
                0,
                1,
                1,
                null_mut(),
                null_mut(),
                hinst,
                null(),
            )
        };
        if border_hwnd.is_null() || pill_hwnd.is_null() {
            if !border_hwnd.is_null() {
                unsafe { DestroyWindow(border_hwnd) };
            }
            if !pill_hwnd.is_null() {
                unsafe { DestroyWindow(pill_hwnd) };
            }
            return None;
        }

        // Neither overlay window may ever appear in the recorded video.
        unsafe {
            SetWindowDisplayAffinity(border_hwnd, WDA_EXCLUDEFROMCAPTURE);
            SetWindowDisplayAffinity(pill_hwnd, WDA_EXCLUDEFROMCAPTURE);
        }

        let pill_win_w_dip = PILL_W_ARMED + PILL_PAD_X * 2.0;
        let mut pill_cb = alloc::boxed::Box::new(PillCallback {
            on_action,
            on_hover,
            ctx,
            scale: 1.0,
            win_w_dip: pill_win_w_dip,
            recording: false,
            over_interactive: false,
            tracking_leave: false,
        });
        unsafe { SetWindowLongPtrW(pill_hwnd, GWLP_USERDATA, (&mut *pill_cb as *mut PillCallback) as isize) };

        Some(Self {
            _d2d: d2d,
            rt,
            dwrite,
            fmt_item,
            fmt_item_bold,
            fmt_badge,
            fmt_timer,
            fmt_size,
            border_hwnd,
            pill_hwnd,
            border_surf: LayeredSurface::new(),
            pill_surf: LayeredSurface::new(),
            pill_cb,
            dpi: 96,
            visible: false,
        })
    }

    pub fn hide(&mut self) {
        if self.visible {
            unsafe {
                ShowWindow(self.border_hwnd, SW_HIDE);
                ShowWindow(self.pill_hwnd, SW_HIDE);
            }
            self.visible = false;
        }
    }

    pub fn render(&mut self, st: &UiState) {
        let r = st.target;
        let rw = r.right - r.left;
        let rh = r.bottom - r.top;
        if rw <= 0 || rh <= 0 {
            self.hide();
            return;
        }

        // On first render, move the hidden pill window onto the target monitor so GetDpiForWindow
        // returns that monitor's DPI right from frame 0.
        if !self.visible {
            unsafe {
                SetWindowPos(self.pill_hwnd, null_mut(), r.left, r.top, 0, 0, SWP_NOACTIVATE | SWP_NOSIZE | SWP_NOZORDER);
            }
        }
        let dpi = unsafe { GetDpiForWindow(self.pill_hwnd) }.max(96);
        self.dpi = dpi;
        let scale = dpi as f32 / 96.0;
        self.pill_cb.scale = scale;
        self.pill_cb.recording = st.recording;

        self.render_border(st, scale);
        self.render_pill(st, dpi, scale);

        if !self.visible {
            unsafe {
                ShowWindow(self.border_hwnd, SW_SHOWNOACTIVATE);
                ShowWindow(self.pill_hwnd, SW_SHOWNOACTIVATE);
                SetWindowPos(self.pill_hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOSIZE);
            }
            self.visible = true;
        } else {
            unsafe {
                SetWindowPos(self.pill_hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOSIZE);
            }
        }
    }

    fn render_border(&mut self, st: &UiState, scale: f32) {
        let r = st.target;
        let bw_px = (BORDER_W_DIP * scale + 0.5) as i32;
        let bw_px = bw_px.max(2);
        // Draw the 2px border cleanly *outside* the target rect (clamped to screen for fullscreen).
        let center = POINT { x: (r.left + r.right) / 2, y: (r.top + r.bottom) / 2 };
        let mon = unsafe { MonitorFromPoint(center, MONITOR_DEFAULTTONEAREST) };
        let mut mi: MONITORINFO = unsafe { core::mem::zeroed() };
        mi.cbSize = size_of::<MONITORINFO>() as u32;
        let mon_r = if unsafe { GetMonitorInfoW(mon, &mut mi) } != 0 { mi.rcMonitor } else { r };

        let bx0 = (r.left - bw_px).max(mon_r.left);
        let by0 = (r.top - bw_px).max(mon_r.top);
        let bx1 = (r.right + bw_px).min(mon_r.right);
        let by1 = (r.bottom + bw_px).min(mon_r.bottom);
        let w = (bx1 - bx0).max(4);
        let h = (by1 - by0).max(4);

        if !self.border_surf.ensure(w, h) {
            return;
        }
        let bind_rect = D2dRect { left: 0, top: 0, right: w, bottom: h };
        if unsafe { self.rt.BindDC(WinHdc(self.border_surf.dc), &bind_rect) }.is_err() {
            return;
        }

        let wf = w as f32;
        let hf = h as f32;
        let bw = bw_px as f32;
        let half = bw * 0.5;
        let tick = (TICK_DIP * scale).min(wf * 0.4).min(hf * 0.4);
        let eased = if st.recording { ease_out_expo_like(st.trans_t) } else { 0.0 };

        unsafe {
            self.rt.SetDpi(96.0, 96.0);
            self.rt.BeginDraw();
            self.rt.Clear(Some(&D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }));

            // Base muted border while armed / transitioning; solid bright red once landed.
            let base_col = if eased >= 0.999 { color(232, 17, 35, 1.0) } else { color(232, 17, 35, 0.30) };
            if let Ok(br) = self.rt.CreateSolidColorBrush(&base_col, None) {
                let rect = D2D_RECT_F { left: half, top: half, right: wf - half, bottom: hf - half };
                self.rt.DrawRectangle(&rect, &br, bw, None);
            }

            // B2 Corner Lock-In ticks / extending bright strokes.
            if eased < 0.999 {
                let tick_col = if st.recording { color(232, 17, 35, 1.0) } else { color(232, 17, 35, 0.55) };
                if let Ok(br) = self.rt.CreateSolidColorBrush(&tick_col, None) {
                    let hx = lerp(tick, wf * 0.5, eased);
                    let vy = lerp(tick, hf * 0.5, eased);
                    // Top-left
                    self.rt.DrawLine(Vector2 { X: 0.0, Y: half }, Vector2 { X: hx, Y: half }, &br, bw, None);
                    self.rt.DrawLine(Vector2 { X: half, Y: 0.0 }, Vector2 { X: half, Y: vy }, &br, bw, None);
                    // Top-right
                    self.rt.DrawLine(Vector2 { X: wf, Y: half }, Vector2 { X: wf - hx, Y: half }, &br, bw, None);
                    self.rt.DrawLine(Vector2 { X: wf - half, Y: 0.0 }, Vector2 { X: wf - half, Y: vy }, &br, bw, None);
                    // Bottom-right
                    self.rt.DrawLine(Vector2 { X: wf, Y: hf - half }, Vector2 { X: wf - hx, Y: hf - half }, &br, bw, None);
                    self.rt.DrawLine(Vector2 { X: wf - half, Y: hf }, Vector2 { X: wf - half, Y: hf - vy }, &br, bw, None);
                    // Bottom-left
                    self.rt.DrawLine(Vector2 { X: 0.0, Y: hf - half }, Vector2 { X: hx, Y: hf - half }, &br, bw, None);
                    self.rt.DrawLine(Vector2 { X: half, Y: hf }, Vector2 { X: half, Y: hf - vy }, &br, bw, None);
                }
            }

            let _ = self.rt.EndDraw(None, None);
        }

        self.border_surf.present(self.border_hwnd, bx0, by0);
    }

    fn render_pill(&mut self, st: &UiState, dpi: u32, scale: f32) {
        let win_w_dip = PILL_W_ARMED + PILL_PAD_X * 2.0;
        let win_h_dip = PILL_H_ARMED + PILL_PAD_TOP + PILL_PAD_BOT;
        let win_w = (win_w_dip * scale + 0.5) as i32;
        let win_h = (win_h_dip * scale + 0.5) as i32;

        if !self.pill_surf.ensure(win_w, win_h) {
            return;
        }
        let bind_rect = D2dRect { left: 0, top: 0, right: win_w, bottom: win_h };
        if unsafe { self.rt.BindDC(WinHdc(self.pill_surf.dc), &bind_rect) }.is_err() {
            return;
        }

        let t = if st.recording { st.trans_t.clamp(0.0, 1.0) } else { 0.0 };
        let e = ease_out_expo_like(t);

        // Smooth pill geometry (`S2` calm dissolve: 340ms width morph, soft cross-fade).
        let pill_w = lerp(PILL_W_ARMED, PILL_W_REC, e);
        let pill_h = lerp(PILL_H_ARMED, PILL_H_REC, e);
        let px = (win_w_dip - pill_w) * 0.5;
        let py = PILL_PAD_TOP + (PILL_H_ARMED - pill_h) * 0.5;
        let pr = pill_h * 0.5;

        // S2 opacity curves: armed fades out over 0..160ms, recording fades in over 110..340ms.
        let armed_alpha = if !st.recording { 1.0 } else { (1.0 - (t * TRANSITION_MS) / 160.0).clamp(0.0, 1.0) };
        let rec_alpha = if !st.recording { 0.0 } else { (((t * TRANSITION_MS) - 110.0) / 230.0).clamp(0.0, 1.0) };

        unsafe {
            // Set render target DPI so all DIP coordinates AND DirectWrite fonts scale identically.
            self.rt.SetDpi(dpi as f32, dpi as f32);
            self.rt.BeginDraw();
            self.rt.Clear(Some(&D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }));

            // Soft layered drop shadow (3 subtle concentric passes, no harsh glow).
            for &(dy, spread, a) in &[(3.0f32, 2.5f32, 0.14f32), (2.0, 1.2, 0.18), (1.0, 0.4, 0.16)] {
                if let Ok(sh) = self.rt.CreateSolidColorBrush(&color(0, 0, 0, a), None) {
                    let rr = D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: px - spread,
                            top: py + dy - spread,
                            right: px + pill_w + spread,
                            bottom: py + pill_h + dy + spread,
                        },
                        radiusX: pr + spread,
                        radiusY: pr + spread,
                    };
                    self.rt.FillRoundedRectangle(&rr, &sh);
                }
            }

            // Pill body (#1e1e1e) + 1px hairline border rgba(255,255,255,0.09).
            let body_rr = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F { left: px, top: py, right: px + pill_w, bottom: py + pill_h },
                radiusX: pr,
                radiusY: pr,
            };
            if let Ok(bg) = self.rt.CreateSolidColorBrush(&color(30, 30, 30, 0.96), None) {
                self.rt.FillRoundedRectangle(&body_rr, &bg);
            }
            if let Ok(bd) = self.rt.CreateSolidColorBrush(&color(255, 255, 255, 0.09), None) {
                self.rt.DrawRoundedRectangle(&body_rr, &bd, 1.0, None);
            }

            // --- Armed layer (`D`: 480 / 720 / 1080 / Native + perched 1 2 3 4) ---
            if armed_alpha > 0.01 {
                let armed_left = (win_w_dip - PILL_W_ARMED) * 0.5;
                let badge_dy = lerp(0.0, 3.0, 1.0 - armed_alpha);
                for (i, q) in Quality::ALL.iter().enumerate() {
                    let (l, r) = ITEM_RANGES[i];
                    let active = *q == st.quality;
                    let col = if active { color(255, 255, 255, armed_alpha) } else { color(255, 255, 255, 0.58 * armed_alpha) };
                    if let Ok(br) = self.rt.CreateSolidColorBrush(&col, None) {
                        let wtxt = to_wide(q.label());
                        let fmt = if active { &self.fmt_item_bold } else { &self.fmt_item };
                        self.draw_text(
                            fmt,
                            &wtxt,
                            D2D_RECT_F { left: armed_left + l, top: py, right: armed_left + r, bottom: py + pill_h },
                            &br,
                            false,
                        );
                    }
                    // Perched micro number `1`..`4` tucked slightly into top rim.
                    let cx = armed_left + (l + r) * 0.5;
                    let by = py - 5.5 + badge_dy;
                    let bg_rr = D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F { left: cx - 5.0, top: by, right: cx + 5.0, bottom: by + 9.0 },
                        radiusX: 3.0,
                        radiusY: 3.0,
                    };
                    if let Ok(bg) = self.rt.CreateSolidColorBrush(&color(30, 30, 30, armed_alpha), None) {
                        self.rt.FillRoundedRectangle(&bg_rr, &bg);
                    }
                    if let Ok(br) = self.rt.CreateSolidColorBrush(&color(255, 255, 255, 0.50 * armed_alpha), None) {
                        let num = [(b'1' + i as u8) as u16, 0];
                        let txt_rect = D2D_RECT_F { left: cx - 7.0, top: by, right: cx + 7.0, bottom: by + 9.0 };
                        self.draw_text(&self.fmt_badge, &num, txt_rect, &br, false);
                    }
                }
            }

            // --- Recording layer (`V3`: 5.5px red dot + tabular timer + muted smaller file size) ---
            if rec_alpha > 0.01 {
                let rec_left = px + 8.0;
                let cy = py + pill_h * 0.5;
                if let Ok(dot_br) = self.rt.CreateSolidColorBrush(&color(232, 17, 35, rec_alpha), None) {
                    let dot = D2D1_ELLIPSE { point: Vector2 { X: rec_left + 2.75, Y: cy }, radiusX: 2.75, radiusY: 2.75 };
                    self.rt.FillEllipse(&dot, &dot_br);
                }

                let timer_str = format_timer(st.elapsed_ms);
                let size_str = format_size(st.bytes);
                if let Ok(tbr) = self.rt.CreateSolidColorBrush(&color(255, 255, 255, rec_alpha), None) {
                    let wt = to_wide(&timer_str);
                    self.draw_text(
                        &self.fmt_timer,
                        &wt,
                        D2D_RECT_F { left: rec_left + 11.5, top: py, right: rec_left + 64.0, bottom: py + pill_h },
                        &tbr,
                        true,
                    );
                }
                if let Ok(sbr) = self.rt.CreateSolidColorBrush(&color(255, 255, 255, 0.34 * rec_alpha), None) {
                    let ws = to_wide(&size_str);
                    self.draw_text(
                        &self.fmt_size,
                        &ws,
                        D2D_RECT_F { left: rec_left + 58.0, top: py + 0.5, right: px + pill_w - BTN_D - 5.0, bottom: py + pill_h + 0.5 },
                        &sbr,
                        true,
                    );
                }
            }

            // --- Shared solid red shutter / stop circle (`V1` stop, right side of pill) ---
            let bcx = px + pill_w - BTN_RIGHT_PAD - BTN_D * 0.5;
            let bcy = py + pill_h * 0.5;
            let btn_col = if st.btn_hover { color(209, 52, 56, 1.0) } else { color(196, 43, 28, 1.0) };
            if let Ok(bbr) = self.rt.CreateSolidColorBrush(&btn_col, None) {
                let circ = D2D1_ELLIPSE { point: Vector2 { X: bcx, Y: bcy }, radiusX: BTN_D * 0.5, radiusY: BTN_D * 0.5 };
                self.rt.FillEllipse(&circ, &bbr);
            }
            // Inner white glyph morphs smoothly from 7px circle (r=3.5) -> 6.5px rounded square (r=1.5).
            let g_sz = lerp(7.0, 6.5, e);
            let g_rad = lerp(3.5, 1.5, e);
            if let Ok(wbr) = self.rt.CreateSolidColorBrush(&color(255, 255, 255, 1.0), None) {
                let grr = D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F { left: bcx - g_sz * 0.5, top: bcy - g_sz * 0.5, right: bcx + g_sz * 0.5, bottom: bcy + g_sz * 0.5 },
                    radiusX: g_rad,
                    radiusY: g_rad,
                };
                self.rt.FillRoundedRectangle(&grr, &wbr);
            }
            // Perched `↵` badge above shutter button while Armed.
            if armed_alpha > 0.01 {
                let ky = py - 7.5 + lerp(0.0, 3.0, 1.0 - armed_alpha);
                let krr = D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F { left: bcx - 5.0, top: ky, right: bcx + 5.0, bottom: ky + 9.0 },
                    radiusX: 3.0,
                    radiusY: 3.0,
                };
                if let Ok(bg) = self.rt.CreateSolidColorBrush(&color(30, 30, 30, armed_alpha), None) {
                    self.rt.FillRoundedRectangle(&krr, &bg);
                }
                if let Ok(br) = self.rt.CreateSolidColorBrush(&color(255, 255, 255, 0.52 * armed_alpha), None) {
                    let ent = to_wide("↵");
                    let txt_rect = D2D_RECT_F { left: bcx - 7.0, top: ky, right: bcx + 7.0, bottom: ky + 9.0 };
                    self.draw_text(&self.fmt_badge, &ent, txt_rect, &br, false);
                }
            }

            let _ = self.rt.EndDraw(None, None);
        }

        // Position straddling the bottom-center of the 2px red border; if the target touches
        // the monitor work-area bottom (e.g. full screen or maximized), tuck cleanly inside.
        let r = st.target;
        let center = POINT { x: (r.left + r.right) / 2, y: (r.top + r.bottom) / 2 };
        let mon = unsafe { MonitorFromPoint(center, MONITOR_DEFAULTTONEAREST) };
        let mut mi: MONITORINFO = unsafe { core::mem::zeroed() };
        mi.cbSize = size_of::<MONITORINFO>() as u32;
        let work = if unsafe { GetMonitorInfoW(mon, &mut mi) } != 0 { mi.rcWork } else { r };

        let pill_center_y_in_win = ((PILL_PAD_TOP + PILL_H_ARMED * 0.5) * scale + 0.5) as i32;
        let ideal_y = r.bottom - pill_center_y_in_win;
        let max_y = work.bottom - win_h - ((4.0 * scale) as i32);
        let min_y = work.top;
        let y = ideal_y.clamp(min_y, max_y.max(min_y));

        let ideal_x = (r.left + r.right) / 2 - win_w / 2;
        let max_x = work.right - win_w;
        let x = ideal_x.clamp(work.left, max_x.max(work.left));

        self.pill_surf.present(self.pill_hwnd, x, y);
    }

    unsafe fn draw_text(
        &self,
        fmt: &IDWriteTextFormat,
        wtxt: &[u16],
        rect: D2D_RECT_F,
        brush: &windows::Win32::Graphics::Direct2D::ID2D1SolidColorBrush,
        tabular: bool,
    ) {
        let len = wtxt.iter().position(|&c| c == 0).unwrap_or(wtxt.len());
        if len == 0 {
            return;
        }
        let w = (rect.right - rect.left).max(1.0);
        let h = (rect.bottom - rect.top).max(1.0);
        if let Ok(layout) = unsafe { self.dwrite.CreateTextLayout(&wtxt[..len], fmt, w, h) } {
            if tabular && let Ok(typo) = unsafe { self.dwrite.CreateTypography() } {
                let feat = DWRITE_FONT_FEATURE { nameTag: DWRITE_FONT_FEATURE_TAG_TABULAR_FIGURES, parameter: 1 };
                let _ = unsafe { typo.AddFontFeature(feat) };
                let _ = unsafe { layout.SetTypography(&typo, DWRITE_TEXT_RANGE { startPosition: 0, length: len as u32 }) };
            }
            unsafe {
                self.rt.DrawTextLayout(Vector2 { X: rect.left, Y: rect.top }, &layout, brush, D2D1_DRAW_TEXT_OPTIONS_NONE);
            }
        }
    }
}

impl Drop for OverlayUi {
    fn drop(&mut self) {
        unsafe {
            SetWindowLongPtrW(self.pill_hwnd, GWLP_USERDATA, 0);
            DestroyWindow(self.pill_hwnd);
            DestroyWindow(self.border_hwnd);
        }
    }
}

fn format_timer(ms: u64) -> alloc::string::String {
    let tenths = ms / 100;
    let total_sec = tenths / 10;
    let d = tenths % 10;
    let mins = total_sec / 60;
    let secs = total_sec % 60;
    format!("{:02}:{:02}.{}", mins, secs, d)
}

pub fn format_size(bytes: u64) -> alloc::string::String {
    if bytes < 1_048_576 {
        let kb = (bytes / 1024).max(1);
        if kb < 100 {
            format!("{kb} KB")
        } else {
            let tenths_mb = ((bytes * 10 + 524_288) / 1_048_576).max(1);
            format!("{}.{} MB", tenths_mb / 10, tenths_mb % 10)
        }
    } else {
        let tenths_mb = (bytes * 10 + 524_288) / 1_048_576;
        format!("{}.{} MB", tenths_mb / 10, tenths_mb % 10)
    }
}

/// Keeps `Vec` imported without warning if unused later.
#[allow(dead_code)]
fn _unused(_: Vec<u8>) {}
