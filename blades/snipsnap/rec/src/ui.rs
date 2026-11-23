//! Direct2D + DirectWrite + UpdateLayeredWindow UI for SnipSnap Recording:
//! - Border overlay (`B2` Corner Lock-In: muted red + 14px corner ticks -> 340ms bright `#e81123`)
//! - Straddling bottom-center housing (`H5` monoblock milled titanium housing with `E3` embedded
//!   brushed-titanium rolling tumbler wheel -> `S2` 340ms calm dissolve -> `H5` 25px recording block)
//! Both windows set `WDA_EXCLUDEFROMCAPTURE` so nothing SnipSnap draws appears in the video.
use crate::ipc::Quality;
use alloc::format;
use core::ffi::c_void;
use core::ptr::{null, null_mut};
use libm::{cosf, sinf};
use pk::wide::to_wide;
use windows::Win32::Foundation::RECT as D2dRect;
use windows::Win32::Graphics::Direct2D::Common::{
    D2D_RECT_F, D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_COLOR_F, D2D1_GRADIENT_STOP, D2D1_PIXEL_FORMAT,
};
use windows::Win32::Graphics::Direct2D::{
    D2D1_DRAW_TEXT_OPTIONS_NONE, D2D1_ELLIPSE, D2D1_EXTEND_MODE_CLAMP, D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1_GAMMA_2_2,
    D2D1_LAYER_OPTIONS_NONE, D2D1_LAYER_PARAMETERS, D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, D2D1_RENDER_TARGET_PROPERTIES,
    D2D1_RENDER_TARGET_TYPE_DEFAULT, D2D1_ROUNDED_RECT, D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE, D2D1CreateFactory, ID2D1DCRenderTarget,
    ID2D1Factory, ID2D1LinearGradientBrush,
};
use windows::Win32::Graphics::DirectWrite::{
    DWRITE_FACTORY_TYPE_SHARED, DWRITE_FONT_FEATURE, DWRITE_FONT_FEATURE_TAG_TABULAR_FIGURES, DWRITE_FONT_STRETCH_NORMAL,
    DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_WEIGHT_MEDIUM, DWRITE_FONT_WEIGHT_NORMAL, DWRITE_FONT_WEIGHT_SEMI_BOLD,
    DWRITE_PARAGRAPH_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_LEADING, DWRITE_TEXT_RANGE,
    DWRITE_WORD_WRAPPING_NO_WRAP, DWriteCreateFactory, IDWriteFactory, IDWriteTextFormat,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::Graphics::Gdi::HDC as WinHdc;
use windows_core::PCWSTR;
use windows_numerics::{Matrix3x2, Vector2};
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, SIZE, WPARAM};
use windows_sys::Win32::Graphics::Gdi::{
    AC_SRC_ALPHA, AC_SRC_OVER, BI_RGB, BITMAPINFO, BITMAPINFOHEADER, BLENDFUNCTION, CreateCompatibleDC, CreateDIBSection, DIB_RGB_COLORS,
    DeleteDC, DeleteObject, GetMonitorInfoW, HBITMAP, HDC, HGDIOBJ, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromPoint, SelectObject,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::SystemInformation::GetTickCount64;
use windows_sys::Win32::UI::HiDpi::GetDpiForWindow;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{TME_LEAVE, TRACKMOUSEEVENT, TrackMouseEvent};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, GWLP_USERDATA, GetWindowLongPtrW, HWND_TOPMOST, IDC_ARROW, IDC_HAND, IDC_SIZENS,
    LoadCursorW, MA_NOACTIVATE, RegisterClassW, SW_HIDE, SW_SHOWNOACTIVATE, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER,
    SetCursor, SetWindowDisplayAffinity, SetWindowLongPtrW, SetWindowPos, ShowWindow, ULW_ALPHA, UpdateLayeredWindow,
    WDA_EXCLUDEFROMCAPTURE, WM_LBUTTONUP, WM_MOUSEACTIVATE, WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_SETCURSOR, WNDCLASSW, WS_EX_LAYERED,
    WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_POPUP,
};

const BORDER_CLASS: *const u16 = pk::w!("SnipSnapRecBorder");
const PILL_CLASS: *const u16 = pk::w!("SnipSnapRecPill");
const WM_MOUSELEAVE: u32 = 0x02A3;

/// Transition duration (`S2` + `B2`) and drum roll duration, milliseconds.
pub const TRANSITION_MS: f32 = 340.0;
pub const DRUM_ROLL_MS: f32 = 280.0;

// Locked `H5` Monoblock Milled Titanium + `E3` wheel geometry in logical DIPs.
const BLOCK_W_ARMED: f32 = 105.0;
const BLOCK_H_ARMED: f32 = 30.0;
const BLOCK_W_REC: f32 = 132.0;
const BLOCK_H_REC: f32 = 25.0;
const BLOCK_R: f32 = 9.0;
const WIN_PAD_X: f32 = 14.0;
const WIN_PAD_TOP: f32 = 10.0;
const WIN_PAD_BOT: f32 = 14.0;

// Embedded `E3` milled pocket & cylinder constants.
const WELL_LEFT: f32 = 5.0;
const WELL_W: f32 = 54.0;
const WELL_H: f32 = 24.0;
const WELL_R: f32 = 5.0;
const DRUM_R: f32 = 18.0;
const DRUM_STEP_RAD: f32 = 36.0 * core::f32::consts::PI / 180.0;
const KNURL_W: f32 = 4.0;

// `H5` seated matte-red enamel shutter / stop circle inside milled dark ring.
const SHUT_D_ARMED: f32 = 20.0;
const SHUT_D_REC: f32 = 17.0;
const SHUT_R_ARMED: f32 = 8.0;
const SHUT_R_REC: f32 = 6.0;

const BORDER_W_DIP: f32 = 2.0;
const TICK_DIP: f32 = 14.0;

#[derive(Clone, Copy)]
pub struct UiState {
    pub target: RECT,
    pub quality: Quality,
    /// Smoothly interpolated continuous drum index (`0.0` = 480p .. `3.0` = Native).
    pub drum_pos: f32,
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
    StepQuality(i32),
    ToggleRecord,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum HoverZone {
    None,
    Wheel,
    Shutter,
}

/// Evaluates CSS `cubic-bezier(0.22, 1.0, 0.36, 1.0)`.
pub fn ease_out_expo_like(x: f32) -> f32 {
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

fn make_vgrad(rt: &ID2D1DCRenderTarget, y0: f32, y1: f32, stops: &[(f32, D2D1_COLOR_F)]) -> Option<ID2D1LinearGradientBrush> {
    let mut gstops = [D2D1_GRADIENT_STOP { position: 0.0, color: D2D1_COLOR_F::default() }; 6];
    let n = stops.len().min(gstops.len());
    for i in 0..n {
        gstops[i] = D2D1_GRADIENT_STOP { position: stops[i].0, color: stops[i].1 };
    }
    let coll = unsafe { rt.CreateGradientStopCollection(&gstops[..n], D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP) }.ok()?;
    let props = D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES { startPoint: Vector2 { X: 0.0, Y: y0 }, endPoint: Vector2 { X: 0.0, Y: y1 } };
    unsafe { rt.CreateLinearGradientBrush(&props, None, &coll) }.ok()
}

fn make_hgrad(rt: &ID2D1DCRenderTarget, x0: f32, x1: f32, stops: &[(f32, D2D1_COLOR_F)]) -> Option<ID2D1LinearGradientBrush> {
    let mut gstops = [D2D1_GRADIENT_STOP { position: 0.0, color: D2D1_COLOR_F::default() }; 4];
    let n = stops.len().min(gstops.len());
    for i in 0..n {
        gstops[i] = D2D1_GRADIENT_STOP { position: stops[i].0, color: stops[i].1 };
    }
    let coll = unsafe { rt.CreateGradientStopCollection(&gstops[..n], D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP) }.ok()?;
    let props = D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES { startPoint: Vector2 { X: x0, Y: 0.0 }, endPoint: Vector2 { X: x1, Y: 0.0 } };
    unsafe { rt.CreateLinearGradientBrush(&props, None, &coll) }.ok()
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

struct PillCallback {
    on_action: fn(*mut c_void, PillAction),
    on_hover: fn(*mut c_void, bool),
    ctx: *mut c_void,
    scale: f32,
    win_w_dip: f32,
    recording: bool,
    hover_zone: HoverZone,
    tracking_leave: bool,
    last_wheel_ms: u64,
}

unsafe extern "system" fn border_proc(h: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    unsafe { DefWindowProcW(h, msg, wp, lp) }
}

unsafe extern "system" fn pill_proc(h: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    let cb_ptr = unsafe { GetWindowLongPtrW(h, GWLP_USERDATA) } as *mut PillCallback;
    match msg {
        WM_MOUSEACTIVATE => return MA_NOACTIVATE as LRESULT,
        WM_SETCURSOR => {
            let zone = if cb_ptr.is_null() { HoverZone::None } else { unsafe { (*cb_ptr).hover_zone } };
            let cur = match zone {
                HoverZone::Shutter => IDC_HAND,
                HoverZone::Wheel => IDC_SIZENS,
                HoverZone::None => IDC_ARROW,
            };
            unsafe { SetCursor(LoadCursorW(null_mut(), cur)) };
            return 1;
        }
        WM_MOUSELEAVE => {
            if !cb_ptr.is_null() {
                let cb = unsafe { &mut *cb_ptr };
                cb.tracking_leave = false;
                cb.hover_zone = HoverZone::None;
                (cb.on_hover)(cb.ctx, false);
            }
            return 0;
        }
        WM_MOUSEWHEEL => {
            if !cb_ptr.is_null() {
                let cb = unsafe { &mut *cb_ptr };
                if !cb.recording {
                    let delta = ((wp >> 16) as u16) as i16;
                    let now = unsafe { GetTickCount64() };
                    if delta != 0 && now.saturating_sub(cb.last_wheel_ms) >= 110 {
                        cb.last_wheel_ms = now;
                        // Scroll down -> next (higher res), scroll up -> previous (lower res).
                        let dir = if delta < 0 { 1 } else { -1 };
                        (cb.on_action)(cb.ctx, PillAction::StepQuality(dir));
                    }
                }
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
                let bw = if cb.recording { BLOCK_W_REC } else { BLOCK_W_ARMED };
                let bh = if cb.recording { BLOCK_H_REC } else { BLOCK_H_ARMED };
                let b_left = (cb.win_w_dip - bw) * 0.5;
                let b_top = WIN_PAD_TOP + (BLOCK_H_ARMED - bh) * 0.5;
                let rx = x - b_left;
                let ry = y - b_top;

                let shut_d = if cb.recording { SHUT_D_REC } else { SHUT_D_ARMED };
                let shut_r = if cb.recording { SHUT_R_REC } else { SHUT_R_ARMED };
                let bcx = bw - shut_r - shut_d * 0.5;
                let bcy = bh * 0.5;
                let dx = rx - bcx;
                let dy = ry - bcy;
                let on_shut = dx * dx + dy * dy <= (shut_d * 0.65) * (shut_d * 0.65);

                let well_top = (bh - WELL_H) * 0.5;
                let on_wheel = !cb.recording && rx >= WELL_LEFT && rx <= WELL_LEFT + WELL_W && ry >= well_top && ry <= well_top + WELL_H;

                cb.hover_zone = if on_shut {
                    HoverZone::Shutter
                } else if on_wheel {
                    HoverZone::Wheel
                } else {
                    HoverZone::None
                };

                if msg == WM_MOUSEMOVE {
                    (cb.on_hover)(cb.ctx, on_shut);
                } else if on_shut {
                    (cb.on_action)(cb.ctx, PillAction::ToggleRecord);
                } else if on_wheel {
                    let frac = (ry - well_top) / WELL_H;
                    if frac < 0.42 {
                        (cb.on_action)(cb.ctx, PillAction::StepQuality(-1));
                    } else if frac > 0.58 {
                        (cb.on_action)(cb.ctx, PillAction::StepQuality(1));
                    }
                }
                return 0;
            }
        }
        _ => {}
    }
    unsafe { DefWindowProcW(h, msg, wp, lp) }
}

pub struct OverlayUi {
    d2d: ID2D1Factory,
    rt: ID2D1DCRenderTarget,
    dwrite: IDWriteFactory,
    fmt_wheel: IDWriteTextFormat,
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

        let fmt_wheel = make_fmt(10.8, DWRITE_FONT_WEIGHT_MEDIUM, DWRITE_TEXT_ALIGNMENT_CENTER)?;
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

        let win_w_dip = BLOCK_W_REC.max(BLOCK_W_ARMED) + WIN_PAD_X * 2.0;
        let mut pill_cb = alloc::boxed::Box::new(PillCallback {
            on_action,
            on_hover,
            ctx,
            scale: 1.0,
            win_w_dip,
            recording: false,
            hover_zone: HoverZone::None,
            tracking_leave: false,
            last_wheel_ms: 0,
        });
        unsafe { SetWindowLongPtrW(pill_hwnd, GWLP_USERDATA, (&mut *pill_cb as *mut PillCallback) as isize) };

        Some(Self {
            d2d,
            rt,
            dwrite,
            fmt_wheel,
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
        let bw_px = ((BORDER_W_DIP * scale + 0.5) as i32).max(2);
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

            let base_col = if eased >= 0.999 { color(232, 17, 35, 1.0) } else { color(232, 17, 35, 0.30) };
            if let Ok(br) = self.rt.CreateSolidColorBrush(&base_col, None) {
                let rect = D2D_RECT_F { left: half, top: half, right: wf - half, bottom: hf - half };
                self.rt.DrawRectangle(&rect, &br, bw, None);
            }

            if eased < 0.999 {
                let tick_col = if st.recording { color(232, 17, 35, 1.0) } else { color(232, 17, 35, 0.55) };
                if let Ok(br) = self.rt.CreateSolidColorBrush(&tick_col, None) {
                    let hx = lerp(tick, wf * 0.5, eased);
                    let vy = lerp(tick, hf * 0.5, eased);
                    self.rt.DrawLine(Vector2 { X: 0.0, Y: half }, Vector2 { X: hx, Y: half }, &br, bw, None);
                    self.rt.DrawLine(Vector2 { X: half, Y: 0.0 }, Vector2 { X: half, Y: vy }, &br, bw, None);
                    self.rt.DrawLine(Vector2 { X: wf, Y: half }, Vector2 { X: wf - hx, Y: half }, &br, bw, None);
                    self.rt.DrawLine(Vector2 { X: wf - half, Y: 0.0 }, Vector2 { X: wf - half, Y: vy }, &br, bw, None);
                    self.rt.DrawLine(Vector2 { X: wf, Y: hf - half }, Vector2 { X: wf - hx, Y: hf - half }, &br, bw, None);
                    self.rt.DrawLine(Vector2 { X: wf - half, Y: hf }, Vector2 { X: wf - half, Y: hf - vy }, &br, bw, None);
                    self.rt.DrawLine(Vector2 { X: 0.0, Y: hf - half }, Vector2 { X: hx, Y: hf - half }, &br, bw, None);
                    self.rt.DrawLine(Vector2 { X: half, Y: hf }, Vector2 { X: half, Y: hf - vy }, &br, bw, None);
                }
            }

            let _ = self.rt.EndDraw(None, None);
        }

        self.border_surf.present(self.border_hwnd, bx0, by0);
    }

    fn render_pill(&mut self, st: &UiState, dpi: u32, scale: f32) {
        let win_w_dip = BLOCK_W_REC.max(BLOCK_W_ARMED) + WIN_PAD_X * 2.0;
        let win_h_dip = BLOCK_H_ARMED + WIN_PAD_TOP + WIN_PAD_BOT;
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

        // Smooth H1 CNC aluminium block morph: 104×30 -> 132×25, keeping 7px machined block corners.
        let bw = lerp(BLOCK_W_ARMED, BLOCK_W_REC, e);
        let bh = lerp(BLOCK_H_ARMED, BLOCK_H_REC, e);
        let px = (win_w_dip - bw) * 0.5;
        let py = WIN_PAD_TOP + (BLOCK_H_ARMED - bh) * 0.5;

        // S2 calm dissolve: wheel fades out over 0..160ms, recording readout fades in over 110..340ms.
        let armed_alpha = if !st.recording { 1.0 } else { (1.0 - (t * TRANSITION_MS) / 160.0).clamp(0.0, 1.0) };
        let rec_alpha = if !st.recording { 0.0 } else { (((t * TRANSITION_MS) - 110.0) / 230.0).clamp(0.0, 1.0) };

        unsafe {
            self.rt.SetDpi(dpi as f32, dpi as f32);
            self.rt.BeginDraw();
            self.rt.Clear(Some(&D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }));

            // Soft hardware drop shadow under the CNC aluminium block.
            for &(dy, spread, a) in &[(4.0f32, 3.0f32, 0.14f32), (2.5, 1.5, 0.18), (1.0, 0.5, 0.18)] {
                if let Ok(sh) = self.rt.CreateSolidColorBrush(&color(0, 0, 0, a), None) {
                    let rr = D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: px - spread,
                            top: py + dy - spread,
                            right: px + bw + spread,
                            bottom: py + bh + dy + spread,
                        },
                        radiusX: BLOCK_R + spread * 0.5,
                        radiusY: BLOCK_R + spread * 0.5,
                    };
                    self.rt.FillRoundedRectangle(&rr, &sh);
                }
            }

            // H5 Monoblock milled titanium body: #353b45 -> #23272f (55%) -> #1a1d23, 9px radius.
            let body_rr = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F { left: px, top: py, right: px + bw, bottom: py + bh },
                radiusX: BLOCK_R,
                radiusY: BLOCK_R,
            };
            if let Some(bg) = make_vgrad(
                &self.rt,
                py,
                py + bh,
                &[(0.0, color(53, 59, 69, 0.99)), (0.55, color(35, 39, 47, 0.99)), (1.0, color(26, 29, 35, 0.99))],
            ) {
                self.rt.FillRoundedRectangle(&body_rr, &bg);
            }
            // Subtle vertical brushed-titanium grain lines clipped inside the monoblock.
            if let Ok(grain) = self.rt.CreateSolidColorBrush(&color(255, 255, 255, 0.028), None) {
                let mut gx = px + 6.0;
                while gx < px + bw - 6.0 {
                    self.rt.DrawLine(Vector2 { X: gx, Y: py + 1.5 }, Vector2 { X: gx, Y: py + bh - 1.5 }, &grain, 0.8, None);
                    gx += 3.0;
                }
            }
            // Top milled catchlight (rgba(255,255,255,0.17)) & bottom bevel (rgba(0,0,0,0.60)).
            if let Ok(top_hi) = self.rt.CreateSolidColorBrush(&color(255, 255, 255, 0.17), None) {
                self.rt.DrawLine(
                    Vector2 { X: px + BLOCK_R * 0.7, Y: py + 1.0 },
                    Vector2 { X: px + bw - BLOCK_R * 0.7, Y: py + 1.0 },
                    &top_hi,
                    1.0,
                    None,
                );
            }
            if let Ok(bot_sh) = self.rt.CreateSolidColorBrush(&color(0, 0, 0, 0.60), None) {
                self.rt.DrawLine(
                    Vector2 { X: px + BLOCK_R * 0.7, Y: py + bh - 1.0 },
                    Vector2 { X: px + bw - BLOCK_R * 0.7, Y: py + bh - 1.0 },
                    &bot_sh,
                    1.0,
                    None,
                );
            }
            if let Ok(bd) = self.rt.CreateSolidColorBrush(&color(11, 13, 16, 1.0), None) {
                self.rt.DrawRoundedRectangle(&body_rr, &bd, 1.0, None);
            }

            // --- Armed layer: E3 brushed-titanium rolling drum inside H5 deep pocket + milled notch ---
            if armed_alpha > 0.01 {
                let well_h = lerp(WELL_H, 12.0, e);
                let wx = px + WELL_LEFT;
                let wy = py + (bh - well_h) * 0.5;
                self.draw_titanium_wheel(wx, wy, WELL_W, well_h, st.drum_pos, armed_alpha);

                // H5 milled dark notch pointer (6x3, #050607) with subtle bright lower lip right off the pocket.
                let nx = wx + WELL_W + 2.0;
                let ny = py + bh * 0.5 - 1.5;
                if let Ok(sh) = self.rt.CreateSolidColorBrush(&color(255, 255, 255, 0.24 * armed_alpha), None) {
                    self.rt.DrawLine(Vector2 { X: nx, Y: ny + 3.5 }, Vector2 { X: nx + 5.5, Y: ny + 3.5 }, &sh, 1.0, None);
                }
                if let Ok(mk) = self.rt.CreateSolidColorBrush(&color(5, 6, 7, armed_alpha), None) {
                    let nrr = D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F { left: nx, top: ny, right: nx + 6.0, bottom: ny + 3.0 },
                        radiusX: 1.0,
                        radiusY: 1.0,
                    };
                    self.rt.FillRoundedRectangle(&nrr, &mk);
                }
            }

            // --- Recording layer: pulsing red dot + tabular timer + quieter muted live file size ---
            if rec_alpha > 0.01 {
                let rec_left = px + 9.0;
                let cy = py + bh * 0.5;
                let pulse = 0.68 + 0.32 * cosf((st.elapsed_ms as f32) * (core::f32::consts::TAU / 1400.0));
                if let Ok(dot_br) = self.rt.CreateSolidColorBrush(&color(232, 17, 35, rec_alpha * pulse), None) {
                    let dot = D2D1_ELLIPSE { point: Vector2 { X: rec_left + 2.75, Y: cy }, radiusX: 2.75, radiusY: 2.75 };
                    self.rt.FillEllipse(&dot, &dot_br);
                }

                let timer_str = format_timer(st.elapsed_ms);
                let size_str = format_size(st.bytes);
                if let Ok(tbr) = self.rt.CreateSolidColorBrush(&color(255, 255, 255, 0.90 * rec_alpha), None) {
                    let wt = to_wide(&timer_str);
                    self.draw_text(
                        &self.fmt_timer,
                        &wt,
                        D2D_RECT_F { left: rec_left + 11.5, top: py, right: rec_left + 64.0, bottom: py + bh },
                        &tbr,
                        true,
                    );
                }
                if let Ok(sbr) = self.rt.CreateSolidColorBrush(&color(255, 255, 255, 0.36 * rec_alpha), None) {
                    let ws = to_wide(&size_str);
                    self.draw_text(
                        &self.fmt_size,
                        &ws,
                        D2D_RECT_F { left: rec_left + 58.0, top: py + 0.5, right: px + bw - 26.0, bottom: py + bh + 0.5 },
                        &sbr,
                        true,
                    );
                }
            }

            // --- H5 seated matte-red enamel shutter / stop circle inside dark milled ring ---
            let shut_d = lerp(SHUT_D_ARMED, SHUT_D_REC, e);
            let shut_pad = lerp(SHUT_R_ARMED, SHUT_R_REC, e);
            let collar = lerp(2.5, 2.0, e);
            let bcx = px + bw - shut_pad - shut_d * 0.5;
            let bcy = py + bh * 0.5;

            // Subtle lower rim highlight around milled socket (#ffffff 11%).
            if let Ok(rim) = self.rt.CreateSolidColorBrush(&color(255, 255, 255, 0.11), None) {
                let c = D2D1_ELLIPSE {
                    point: Vector2 { X: bcx, Y: bcy + 1.0 },
                    radiusX: shut_d * 0.5 + collar + 0.5,
                    radiusY: shut_d * 0.5 + collar + 0.5,
                };
                self.rt.FillEllipse(&c, &rim);
            }
            // Dark milled ring (#0b0d10).
            if let Ok(col_br) = self.rt.CreateSolidColorBrush(&color(11, 13, 16, 1.0), None) {
                let c = D2D1_ELLIPSE { point: Vector2 { X: bcx, Y: bcy }, radiusX: shut_d * 0.5 + collar, radiusY: shut_d * 0.5 + collar };
                self.rt.FillEllipse(&c, &col_br);
            }
            // Seated matte-red enamel (#b3261a; recessed top bevel shadow instead of bright outer dome).
            let (r_top, r_bot) = if st.btn_hover {
                (color(192, 42, 30, 1.0), color(214, 54, 40, 1.0))
            } else {
                (color(150, 30, 20, 1.0), color(184, 40, 28, 1.0))
            };
            if let Some(pbr) = make_vgrad(&self.rt, bcy - shut_d * 0.5, bcy + shut_d * 0.5, &[(0.0, r_top), (1.0, r_bot)]) {
                let c = D2D1_ELLIPSE { point: Vector2 { X: bcx, Y: bcy }, radiusX: shut_d * 0.5, radiusY: shut_d * 0.5 };
                self.rt.FillEllipse(&c, &pbr);
            }
            // Inner seated enamel rim shadow.
            if let Ok(in_sh) = self.rt.CreateSolidColorBrush(&color(0, 0, 0, 0.42), None) {
                let c = D2D1_ELLIPSE { point: Vector2 { X: bcx, Y: bcy }, radiusX: shut_d * 0.5 - 0.5, radiusY: shut_d * 0.5 - 0.5 };
                self.rt.DrawEllipse(&c, &in_sh, 1.0, None);
            }
            // White glyph morphing smoothly from 7px dot -> 6px rounded square.
            let g_sz = lerp(7.0, 6.0, e);
            let g_rad = lerp(3.5, 1.5, e);
            if let Ok(wbr) = self.rt.CreateSolidColorBrush(&color(255, 255, 255, 1.0), None) {
                let grr = D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F { left: bcx - g_sz * 0.5, top: bcy - g_sz * 0.5, right: bcx + g_sz * 0.5, bottom: bcy + g_sz * 0.5 },
                    radiusX: g_rad,
                    radiusY: g_rad,
                };
                self.rt.FillRoundedRectangle(&grr, &wbr);
            }

            let _ = self.rt.EndDraw(None, None);
        }

        // Straddle bottom-center of the 2px border; clamp inside monitor work area when near edges.
        let r = st.target;
        let center = POINT { x: (r.left + r.right) / 2, y: (r.top + r.bottom) / 2 };
        let mon = unsafe { MonitorFromPoint(center, MONITOR_DEFAULTTONEAREST) };
        let mut mi: MONITORINFO = unsafe { core::mem::zeroed() };
        mi.cbSize = size_of::<MONITORINFO>() as u32;
        let work = if unsafe { GetMonitorInfoW(mon, &mut mi) } != 0 { mi.rcWork } else { r };

        let center_y_in_win = ((WIN_PAD_TOP + BLOCK_H_ARMED * 0.5) * scale + 0.5) as i32;
        let ideal_y = r.bottom - center_y_in_win;
        let max_y = work.bottom - win_h - ((4.0 * scale) as i32);
        let min_y = work.top;
        let y = ideal_y.clamp(min_y, max_y.max(min_y));

        let ideal_x = (r.left + r.right) / 2 - win_w / 2;
        let max_x = work.right - win_w;
        let x = ideal_x.clamp(work.left, max_x.max(work.left));

        self.pill_surf.present(self.pill_hwnd, x, y);
    }

    /// Renders `E3`'s satin-titanium rolling cylinder inside the milled pocket `(wx, wy, ww, wh)`.
    unsafe fn draw_titanium_wheel(&self, wx: f32, wy: f32, ww: f32, wh: f32, drum_pos: f32, alpha: f32) {
        let well_rect = D2D_RECT_F { left: wx, top: wy, right: wx + ww, bottom: wy + wh };
        let well_rr = D2D1_ROUNDED_RECT { rect: well_rect, radiusX: WELL_R, radiusY: WELL_R };

        // Outer pocket lip: dark top edge, subtle bright catchlight along bottom rim.
        if let Ok(lip_hi) = unsafe { self.rt.CreateSolidColorBrush(&color(255, 255, 255, 0.11 * alpha), None) } {
            unsafe {
                self.rt.DrawLine(
                    Vector2 { X: wx + 2.0, Y: wy + wh + 0.8 },
                    Vector2 { X: wx + ww - 2.0, Y: wy + wh + 0.8 },
                    &lip_hi,
                    1.0,
                    None,
                );
            }
        }
        if let Ok(lip_lo) = unsafe { self.rt.CreateSolidColorBrush(&color(0, 0, 0, 0.75 * alpha), None) } {
            unsafe {
                self.rt.DrawLine(Vector2 { X: wx + 2.0, Y: wy - 0.8 }, Vector2 { X: wx + ww - 2.0, Y: wy - 0.8 }, &lip_lo, 1.0, None);
            }
        }

        // Clip all drum contents cleanly to the rounded milled pocket (`r = 4`).
        let Ok(clip_geo) = (unsafe { self.d2d.CreateRoundedRectangleGeometry(&well_rr) }) else {
            return;
        };
        let Ok(layer) = (unsafe { self.rt.CreateLayer(None) }) else {
            return;
        };
        let layer_params = D2D1_LAYER_PARAMETERS {
            contentBounds: well_rect,
            geometricMask: core::mem::ManuallyDrop::new(Some(clip_geo.into())),
            maskAntialiasMode: windows::Win32::Graphics::Direct2D::D2D1_ANTIALIAS_MODE_PER_PRIMITIVE,
            maskTransform: Matrix3x2::identity(),
            opacity: alpha,
            opacityBrush: core::mem::ManuallyDrop::new(None),
            layerOptions: D2D1_LAYER_OPTIONS_NONE,
        };
        unsafe { self.rt.PushLayer(&layer_params, &layer) };

        // H5 brighter brushed-titanium drum (#1b1f25 -> #434a56 -> #5e6776 -> #373d48 -> #15181d).
        if let Some(metal) = make_vgrad(
            &self.rt,
            wy,
            wy + wh,
            &[
                (0.0, color(27, 31, 37, 1.0)),
                (0.24, color(67, 74, 86, 1.0)),
                (0.50, color(94, 103, 118, 1.0)),
                (0.76, color(55, 61, 72, 1.0)),
                (1.0, color(21, 24, 29, 1.0)),
            ],
        ) {
            unsafe { self.rt.FillRectangle(&well_rect, &metal) };
        }

        // Turning cylindrical faces (-2..=5: 0..=3 carry labels, bare machined metal past both ends).
        let mid_y = wy + wh * 0.5;
        let face_h = 11.7f32;
        for i in -2i32..=5i32 {
            let theta = (i as f32 - drum_pos) * DRUM_STEP_RAD;
            if theta.abs() >= 1.45 {
                continue;
            }
            let cos_t = cosf(theta).max(0.05);
            let cy = mid_y + DRUM_R * sinf(theta);
            let fh_proj = face_h * cos_t;

            // Left-edge machined grip ticks scrolling with the cylinder.
            for k in 0..3 {
                let local_y = (k as f32 - 1.0) * 3.8 * cos_t;
                let gy = cy + local_y;
                if let Ok(dk) = unsafe { self.rt.CreateSolidColorBrush(&color(0, 0, 0, 0.70 * cos_t), None) } {
                    unsafe {
                        self.rt.DrawLine(Vector2 { X: wx, Y: gy }, Vector2 { X: wx + KNURL_W, Y: gy }, &dk, 1.0, None);
                    }
                }
                if let Ok(lt) = unsafe { self.rt.CreateSolidColorBrush(&color(255, 255, 255, 0.14 * cos_t), None) } {
                    unsafe {
                        self.rt.DrawLine(
                            Vector2 { X: wx, Y: gy + 1.0 * cos_t },
                            Vector2 { X: wx + KNURL_W, Y: gy + 1.0 * cos_t },
                            &lt,
                            0.8,
                            None,
                        );
                    }
                }
            }

            // Horizontal face divider groove between stops.
            let div_y = cy + fh_proj * 0.5;
            if let Ok(grv) = unsafe { self.rt.CreateSolidColorBrush(&color(0, 0, 0, 0.30 * cos_t), None) } {
                unsafe { self.rt.DrawLine(Vector2 { X: wx, Y: div_y }, Vector2 { X: wx + ww, Y: div_y }, &grv, 0.9, None) };
            }

            // Preset text with real cylindrical vertical foreshortening via Direct2D transform.
            if (0..=3).contains(&i) {
                let q = Quality::from_index(i as usize);
                let dist = (i as f32 - drum_pos).abs().min(1.0);
                let txt_col = if dist < 0.35 {
                    let k = dist / 0.35;
                    color(lerp(255.0, 206.0, k) as u8, lerp(255.0, 214.0, k) as u8, lerp(255.0, 226.0, k) as u8, lerp(1.0, 0.62, k))
                } else {
                    color(206, 214, 226, 0.50 * cos_t)
                };
                if let Ok(tbr) = unsafe { self.rt.CreateSolidColorBrush(&txt_col, None) } {
                    let wtxt = to_wide(q.label());
                    // Scale vertically around (center_x, cy) for cylindrical tilt.
                    let cx = wx + KNURL_W + (ww - KNURL_W) * 0.5;
                    let m = Matrix3x2::translation(-cx, -cy) * Matrix3x2::scale(1.0, cos_t) * Matrix3x2::translation(cx, cy);
                    unsafe {
                        self.rt.SetTransform(&m);
                        self.draw_text(
                            &self.fmt_wheel,
                            &wtxt,
                            D2D_RECT_F { left: wx + KNURL_W, top: cy - face_h * 0.65, right: wx + ww, bottom: cy + face_h * 0.65 },
                            &tbr,
                            true,
                        );
                        self.rt.SetTransform(&Matrix3x2::identity());
                    }
                }
            }
        }

        // Specular center sheen band over the turning metal (H5: 0.09).
        if let Some(sheen) = make_vgrad(
            &self.rt,
            wy,
            wy + wh,
            &[(0.38, color(255, 255, 255, 0.0)), (0.50, color(255, 255, 255, 0.09)), (0.62, color(255, 255, 255, 0.0))],
        ) {
            unsafe { self.rt.FillRectangle(&well_rect, &sheen) };
        }

        // Deep inset top & bottom cylindrical shadow gradients where the drum enters the housing.
        if let Some(vsh) = make_vgrad(
            &self.rt,
            wy,
            wy + wh,
            &[
                (0.0, color(0, 0, 0, 0.88)),
                (0.28, color(0, 0, 0, 0.14)),
                (0.50, color(0, 0, 0, 0.0)),
                (0.72, color(0, 0, 0, 0.14)),
                (1.0, color(0, 0, 0, 0.88)),
            ],
        ) {
            unsafe { self.rt.FillRectangle(&well_rect, &vsh) };
        }
        // Subtle left & right pocket wall shadows.
        if let Some(hsh) = make_hgrad(
            &self.rt,
            wx,
            wx + ww,
            &[(0.0, color(0, 0, 0, 0.68)), (0.10, color(0, 0, 0, 0.0)), (0.90, color(0, 0, 0, 0.0)), (1.0, color(0, 0, 0, 0.68))],
        ) {
            unsafe { self.rt.FillRectangle(&well_rect, &hsh) };
        }

        unsafe {
            self.rt.PopLayer();
            let _ = core::mem::ManuallyDrop::into_inner(layer_params.geometricMask);
            let _ = core::mem::ManuallyDrop::into_inner(layer_params.opacityBrush);
        }

        // Crisp 1px inner pocket rim.
        if let Ok(rim) = unsafe { self.rt.CreateSolidColorBrush(&color(0, 0, 0, 0.65 * alpha), None) } {
            unsafe { self.rt.DrawRoundedRectangle(&well_rr, &rim, 1.0, None) };
        }
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
