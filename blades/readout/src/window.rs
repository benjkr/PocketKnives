//! The bar's window: an app bar along the top of the main screen (Windows moves other windows out
//! of its way, like the taskbar), redrawn every second, with a panel under the tile the pointer is on.
use crate::bar::{self, BAR_H, Item, Layout, PANEL_GAP, PANEL_MARGIN, Time};
use crate::metrics::Collector;
use crate::model::Model;
use crate::raster::Canvas;
use crate::settings::Settings;
use crate::theme::{self, Palette};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::ptr::{null, null_mut};
use pk::instance::{NamedMutex, find_window, register_message};
use pk::w;
use pk::wide::to_wide;
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, SIZE, WPARAM};
use windows_sys::Win32::Graphics::Gdi::{
    BI_RGB, BITMAPINFO, BITMAPINFOHEADER, CreateCompatibleDC, CreateDIBSection, DIB_RGB_COLORS, DeleteDC, DeleteObject, GetMonitorInfoW,
    HBITMAP, HDC, HGDIOBJ, MONITOR_DEFAULTTOPRIMARY, MONITORINFO, MonitorFromPoint, SelectObject,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::SystemInformation::GetLocalTime;
use windows_sys::Win32::UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{TME_LEAVE, TRACKMOUSEEVENT, TrackMouseEvent};
use windows_sys::Win32::UI::Shell::{
    ABE_TOP, ABM_NEW, ABM_QUERYPOS, ABM_REMOVE, ABM_SETPOS, ABN_FULLSCREENAPP, ABN_POSCHANGED, APPBARDATA, SHAppBarMessage,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyMenu, DestroyWindow, DispatchMessageW, GWLP_USERDATA,
    GetCursorPos, GetMessageW, GetWindowLongPtrW, HWND_BOTTOM, HWND_TOPMOST, IDC_ARROW, LoadCursorW, MA_NOACTIVATE, MF_CHECKED,
    MF_POPUP, MF_SEPARATOR, MF_STRING, MSG, PostMessageW, PostQuitMessage, RegisterClassExW, SW_HIDE, SW_SHOWNOACTIVATE,
    SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SetForegroundWindow, SetTimer, SetWindowLongPtrW, SetWindowPos, ShowWindow,
    TPM_RETURNCMD, TPM_RIGHTBUTTON, TrackPopupMenu, TranslateMessage, ULW_ALPHA, UpdateLayeredWindow, WM_APP, WM_DESTROY,
    WM_DISPLAYCHANGE, WM_DPICHANGED, WM_MOUSEACTIVATE, WM_MOUSEMOVE, WM_RBUTTONUP, WM_TIMER, WNDCLASSEXW,
    WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_POPUP,
};
use windows_sys::Win32::Graphics::Gdi::{AC_SRC_ALPHA, AC_SRC_OVER, BLENDFUNCTION};
use windows_sys::core::PCWSTR;

const MUTEX_NAME: PCWSTR = w!("Local\\PocketKnives.Readout.v1");
const BAR_CLASS: PCWSTR = w!("PocketKnives.Readout.Bar");
const PANEL_CLASS: PCWSTR = w!("PocketKnives.Readout.Panel");
const RELOAD_MSG: PCWSTR = w!("PocketKnives.Readout.Reload.v1");
const EXIT_MSG: PCWSTR = w!("PocketKnives.Readout.Exit.v1");
const WM_APPBAR: u32 = WM_APP + 1;
const WM_MOUSELEAVE: u32 = 0x02A3;
const TIMER_TICK: usize = 1;
const MENU_EXIT: usize = 1;
const MENU_THEME: usize = 100;

/// A layered window's pixels: a DIB section the canvas is copied into.
struct Surface {
    dc: HDC,
    bmp: HBITMAP,
    old: HGDIOBJ,
    bits: *mut u8,
    w: usize,
    h: usize,
}

impl Surface {
    fn new(w: usize, h: usize) -> Surface {
        let bi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: core::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: w as i32,
                biHeight: -(h as i32),
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB,
                ..unsafe { core::mem::zeroed() }
            },
            ..unsafe { core::mem::zeroed() }
        };
        let dc = unsafe { CreateCompatibleDC(null_mut()) };
        let mut bits = null_mut();
        let bmp = unsafe { CreateDIBSection(dc, &bi, DIB_RGB_COLORS, &mut bits, null_mut(), 0) };
        let old = unsafe { SelectObject(dc, bmp as HGDIOBJ) };
        Surface { dc, bmp, old, bits: bits as *mut u8, w, h }
    }

    /// Shows `c` (the surface's size) in `hwnd` at (x, y) on the screen.
    fn show(&self, hwnd: HWND, c: &Canvas, x: i32, y: i32) {
        if self.bits.is_null() || c.w != self.w || c.h != self.h {
            return;
        }
        unsafe { core::ptr::copy_nonoverlapping(c.px.as_ptr() as *const u8, self.bits, c.px.len() * 4) };
        let pos = POINT { x, y };
        let size = SIZE { cx: self.w as i32, cy: self.h as i32 };
        let src = POINT { x: 0, y: 0 };
        let blend = BLENDFUNCTION { BlendOp: AC_SRC_OVER as u8, BlendFlags: 0, SourceConstantAlpha: 255, AlphaFormat: AC_SRC_ALPHA as u8 };
        unsafe { UpdateLayeredWindow(hwnd, null_mut(), &pos, &size, self.dc, &src, 0, &blend, ULW_ALPHA) };
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            SelectObject(self.dc, self.old);
            DeleteObject(self.bmp as HGDIOBJ);
            DeleteDC(self.dc);
        }
    }
}

struct App {
    hwnd: HWND,
    panel_hwnd: HWND,
    settings: Settings,
    pal: Palette,
    model: Model,
    collector: Collector,
    layout: Layout,
    /// The bar's top left on the screen, in device pixels.
    origin: (i32, i32),
    canvas: Canvas,
    /// The bar's background for the current hover (`None` when it must be drawn again).
    back: Canvas,
    back_hover: Option<Option<usize>>,
    surface: Surface,
    panel_canvas: Canvas,
    panel_surface: Surface,
    hover: Option<usize>,
    tracking: bool,
    fullscreen: bool,
    reload_msg: u32,
    exit_msg: u32,
}

pub fn run() -> u32 {
    let Some(guard) = NamedMutex::first(MUTEX_NAME) else { return 0 };
    guard.keep();
    let hinst = unsafe { GetModuleHandleW(null()) };
    for (class, proc_) in [(BAR_CLASS, bar_proc as unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT), (PANEL_CLASS, DefWindowProcW)] {
        let wc = WNDCLASSEXW {
            cbSize: core::mem::size_of::<WNDCLASSEXW>() as u32,
            lpfnWndProc: Some(proc_),
            hInstance: hinst,
            hCursor: unsafe { LoadCursorW(null_mut(), IDC_ARROW) },
            lpszClassName: class,
            ..unsafe { core::mem::zeroed() }
        };
        if unsafe { RegisterClassExW(&wc) } == 0 {
            return 1;
        }
    }
    let ex = WS_EX_LAYERED | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_TOPMOST;
    let hwnd = unsafe { CreateWindowExW(ex, BAR_CLASS, w!("Readout"), WS_POPUP, 0, 0, 1, 1, null_mut(), null_mut(), hinst, null()) };
    let panel_hwnd = unsafe { CreateWindowExW(ex | WS_EX_TRANSPARENT, PANEL_CLASS, w!("Readout panel"), WS_POPUP, 0, 0, 1, 1, hwnd, null_mut(), hinst, null()) };
    if hwnd.is_null() || panel_hwnd.is_null() {
        return 1;
    }
    let settings = Settings::load();
    let pal = load_theme(&settings.theme);
    let mut app = Box::new(App {
        hwnd,
        panel_hwnd,
        pal,
        model: Model::new(),
        collector: Collector::new(),
        layout: Layout::new(400.0, 1, &settings.items(), settings.clock),
        settings,
        origin: (0, 0),
        canvas: Canvas::new(1, 1),
        back: Canvas::new(1, 1),
        back_hover: None,
        surface: Surface::new(1, 1),
        panel_canvas: Canvas::new(1, 1),
        panel_surface: Surface::new(1, 1),
        hover: None,
        tracking: false,
        fullscreen: false,
        reload_msg: register_message(RELOAD_MSG),
        exit_msg: register_message(EXIT_MSG),
    });
    let mut abd = appbar(hwnd);
    abd.uCallbackMessage = WM_APPBAR;
    unsafe { SHAppBarMessage(ABM_NEW, &mut abd) };
    app.sample();
    app.place();
    unsafe {
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(app) as isize);
        ShowWindow(hwnd, SW_SHOWNOACTIVATE);
        SetTimer(hwnd, TIMER_TICK, 1000, None);
    }
    let mut msg: MSG = unsafe { core::mem::zeroed() };
    while unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) } > 0 {
        unsafe {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    0
}

/// Tells a running bar to read its settings again.
pub fn notify_settings_changed() {
    let h = find_window(BAR_CLASS);
    if !h.is_null() {
        unsafe { PostMessageW(h, register_message(RELOAD_MSG), 0, 0) };
    }
}

/// Asks a running bar to close.
pub fn request_exit() {
    let h = find_window(BAR_CLASS);
    if !h.is_null() {
        unsafe { PostMessageW(h, register_message(EXIT_MSG), 0, 0) };
    }
}

/// The theme `name`, or the first built-in when it can't be loaded (a file deleted or broken).
fn load_theme(name: &str) -> Palette {
    theme::load(name).unwrap_or_else(|_| theme::parse(theme::BUILT_IN[0].1).expect("built-in theme"))
}

fn appbar(hwnd: HWND) -> APPBARDATA {
    APPBARDATA { cbSize: core::mem::size_of::<APPBARDATA>() as u32, hWnd: hwnd, ..unsafe { core::mem::zeroed() } }
}

/// Device pixels per design pixel at `dpi`: the nearest whole number, at least 1.
fn scale(dpi: u32) -> i32 {
    ((dpi as i32 * 2 + 96) / 192).max(1)
}

pub fn now() -> Time {
    let mut st = unsafe { core::mem::zeroed() };
    unsafe { GetLocalTime(&mut st) };
    Time { hour: st.wHour, minute: st.wMinute, weekday: st.wDayOfWeek, day: st.wDay, month: st.wMonth }
}

impl App {
    fn sample(&mut self) {
        let s = self.collector.sample();
        self.model.push(s);
    }

    /// Claims the strip along the top of the main screen and lays the bar out for its width and scaling.
    fn place(&mut self) {
        let mon = unsafe { MonitorFromPoint(POINT { x: 0, y: 0 }, MONITOR_DEFAULTTOPRIMARY) };
        let mut mi = MONITORINFO { cbSize: core::mem::size_of::<MONITORINFO>() as u32, ..unsafe { core::mem::zeroed() } };
        unsafe { GetMonitorInfoW(mon, &mut mi) };
        let (mut dx, mut dy) = (96u32, 96u32);
        unsafe { GetDpiForMonitor(mon, MDT_EFFECTIVE_DPI, &mut dx, &mut dy) };
        let k = scale(dx);
        let r = mi.rcMonitor;
        let mut abd = appbar(self.hwnd);
        abd.uEdge = ABE_TOP;
        abd.rc = RECT { left: r.left, top: r.top, right: r.right, bottom: r.top + BAR_H * k };
        unsafe { SHAppBarMessage(ABM_QUERYPOS, &mut abd) };
        abd.rc.bottom = abd.rc.top + BAR_H * k;
        unsafe { SHAppBarMessage(ABM_SETPOS, &mut abd) };
        let width = (abd.rc.right - abd.rc.left) as f32 / k as f32;
        self.layout = Layout::new(width, k, &self.settings.items(), self.settings.clock);
        self.origin = (abd.rc.left, abd.rc.top);
        let (w, h) = self.layout.size();
        if self.canvas.w != w || self.canvas.h != h {
            self.canvas = Canvas::new(w, h);
            self.back = Canvas::new(w, h);
            self.surface = Surface::new(w, h);
        }
        self.back_hover = None;
        let (pw, ph) = bar::panel_size(k);
        if self.panel_canvas.w != pw || self.panel_canvas.h != ph {
            self.panel_canvas = Canvas::new(pw, ph);
            self.panel_surface = Surface::new(pw, ph);
        }
        self.hover = None;
        self.draw();
    }

    fn draw(&mut self) {
        if self.back_hover != Some(self.hover) {
            bar::background(&mut self.back, &self.layout, &self.pal, self.hover);
            self.back_hover = Some(self.hover);
        }
        self.canvas.px.copy_from_slice(&self.back.px);
        bar::foreground(&mut self.canvas, &self.layout, &self.pal, &self.model, now());
        self.surface.show(self.hwnd, &self.canvas, self.origin.0, self.origin.1);
        self.draw_panel();
    }

    fn draw_panel(&mut self) {
        let item = self.hover.map(|i| self.layout.tiles[i]).filter(|t| t.item != Item::Clock);
        let Some(tile) = item else {
            unsafe { ShowWindow(self.panel_hwnd, SW_HIDE) };
            return;
        };
        let i = self.hover.unwrap_or(0);
        let k = self.layout.k;
        bar::panel(&mut self.panel_canvas, k, &self.pal, &self.model, tile.item);
        let x = self.origin.0 + ((self.layout.panel_x(i) - PANEL_MARGIN as f32) * k as f32) as i32;
        let y = self.origin.1 + (BAR_H + PANEL_GAP - PANEL_MARGIN) * k;
        self.panel_surface.show(self.panel_hwnd, &self.panel_canvas, x, y);
        unsafe { ShowWindow(self.panel_hwnd, SW_SHOWNOACTIVATE) };
    }

    fn reload(&mut self) {
        self.settings = Settings::load();
        self.pal = load_theme(&self.settings.theme);
        self.place();
    }

    fn mouse(&mut self, lp: LPARAM) {
        if !self.tracking {
            let mut tme = TRACKMOUSEEVENT { cbSize: core::mem::size_of::<TRACKMOUSEEVENT>() as u32, dwFlags: TME_LEAVE, hwndTrack: self.hwnd, dwHoverTime: 0 };
            unsafe { TrackMouseEvent(&mut tme) };
            self.tracking = true;
        }
        let k = self.layout.k as f32;
        let (x, y) = ((lp & 0xFFFF) as i16 as f32 / k, ((lp >> 16) & 0xFFFF) as i16 as f32 / k);
        let hit = self.layout.hit(x, y);
        if hit != self.hover {
            self.hover = hit;
            self.draw();
        }
    }

    fn menu(&mut self) {
        let names: Vec<String> = theme::names();
        let themes = unsafe { CreatePopupMenu() };
        for (i, n) in names.iter().enumerate() {
            let label = to_wide(n);
            let flags = MF_STRING | if *n == self.settings.theme { MF_CHECKED } else { 0 };
            unsafe { AppendMenuW(themes, flags, MENU_THEME + i, label.as_ptr()) };
        }
        let menu = unsafe { CreatePopupMenu() };
        unsafe {
            AppendMenuW(menu, MF_POPUP, themes as usize, w!("Theme"));
            AppendMenuW(menu, MF_SEPARATOR, 0, null());
            AppendMenuW(menu, MF_STRING, MENU_EXIT, w!("Exit"));
        }
        let mut pt = POINT { x: 0, y: 0 };
        unsafe {
            GetCursorPos(&mut pt);
            SetForegroundWindow(self.hwnd);
        }
        let cmd = unsafe { TrackPopupMenu(menu, TPM_RETURNCMD | TPM_RIGHTBUTTON, pt.x, pt.y, 0, self.hwnd, null()) } as usize;
        unsafe { DestroyMenu(menu) };
        if cmd == MENU_EXIT {
            unsafe { DestroyWindow(self.hwnd) };
        } else if cmd >= MENU_THEME && cmd < MENU_THEME + names.len() {
            let mut next = self.settings.clone();
            next.theme = names[cmd - MENU_THEME].clone();
            if next.save().is_ok() {
                self.reload();
            }
        }
    }
}

unsafe extern "system" fn bar_proc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    let ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) } as *mut App;
    if ptr.is_null() {
        return unsafe { DefWindowProcW(hwnd, msg, wp, lp) };
    }
    let app = unsafe { &mut *ptr };
    match msg {
        WM_TIMER if wp == TIMER_TICK => {
            app.sample();
            if !app.fullscreen {
                app.draw();
            }
            0
        }
        WM_MOUSEMOVE => {
            app.mouse(lp);
            0
        }
        WM_MOUSELEAVE => {
            app.tracking = false;
            if app.hover.take().is_some() {
                app.draw();
            }
            0
        }
        WM_MOUSEACTIVATE => MA_NOACTIVATE as LRESULT,
        WM_RBUTTONUP => {
            app.menu();
            0
        }
        WM_APPBAR => {
            match wp as u32 {
                ABN_POSCHANGED => app.place(),
                ABN_FULLSCREENAPP => {
                    // Step aside for a full-screen app, and come back on top after it.
                    app.fullscreen = lp != 0;
                    let after = if app.fullscreen { HWND_BOTTOM } else { HWND_TOPMOST };
                    unsafe { SetWindowPos(hwnd, after, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE) };
                }
                _ => {}
            }
            0
        }
        WM_DISPLAYCHANGE | WM_DPICHANGED => {
            app.place();
            unsafe { DefWindowProcW(hwnd, msg, wp, lp) }
        }
        WM_DESTROY => {
            let mut abd = appbar(hwnd);
            unsafe {
                SHAppBarMessage(ABM_REMOVE, &mut abd);
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
                drop(Box::from_raw(ptr));
                PostQuitMessage(0);
            }
            0
        }
        m if m == app.reload_msg => {
            app.reload();
            0
        }
        m if m == app.exit_msg => {
            unsafe { DestroyWindow(hwnd) };
            0
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wp, lp) },
    }
}
