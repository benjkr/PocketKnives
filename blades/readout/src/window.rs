//! The bar's window: an app bar along the top of the main screen (Windows moves other windows out
//! of its way, like the taskbar), redrawn every second, with a panel under the tile the pointer is
//! on. The panel stays while the pointer is on it, and its keys end programs.
use crate::bar::{self, BAR_H, Item, Layout, ListView, PANEL_GAP, PANEL_MARGIN, PANEL_W, Time};
use crate::icons::{self, Icon};
use crate::metrics::Collector;
use crate::model::Model;
use crate::procs::{self, Kill};
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
use windows_sys::Win32::Graphics::Gdi::{AC_SRC_ALPHA, AC_SRC_OVER, BLENDFUNCTION};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::SystemInformation::{GetLocalTime, GetTickCount64};
use windows_sys::Win32::UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    GetDoubleClickTime, ReleaseCapture, SetCapture, TME_LEAVE, TRACKMOUSEEVENT, TrackMouseEvent,
};
use windows_sys::Win32::UI::Shell::{
    ABE_TOP, ABM_NEW, ABM_QUERYPOS, ABM_REMOVE, ABM_SETPOS, ABN_FULLSCREENAPP, ABN_POSCHANGED, APPBARDATA, SHAppBarMessage,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyMenu, DestroyWindow, DispatchMessageW, GWLP_USERDATA,
    GetCursorPos, GetMessageW, GetWindowLongPtrW, HWND_BOTTOM, HWND_TOPMOST, IDC_ARROW, KillTimer, LoadCursorW, MA_NOACTIVATE,
    MF_CHECKED, MF_POPUP, MF_SEPARATOR, MF_STRING, MSG, PostMessageW, PostQuitMessage, RegisterClassExW, SW_HIDE,
    SW_SHOWNOACTIVATE, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SetForegroundWindow, SetTimer, SetWindowLongPtrW, SetWindowPos,
    ShowWindow, TPM_RETURNCMD, TPM_RIGHTBUTTON, TrackPopupMenu, TranslateMessage, ULW_ALPHA, UpdateLayeredWindow, WM_APP,
    WM_CAPTURECHANGED, WM_DESTROY, WM_DISPLAYCHANGE, WM_DPICHANGED, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEACTIVATE, WM_MOUSEMOVE,
    WM_RBUTTONUP, WM_TIMER, WNDCLASSEXW, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_POPUP,
};
use windows_sys::core::PCWSTR;

const MUTEX_NAME: PCWSTR = w!("Local\\PocketKnives.Readout.v1");
const BAR_CLASS: PCWSTR = w!("PocketKnives.Readout.Bar");
const PANEL_CLASS: PCWSTR = w!("PocketKnives.Readout.Panel");
const RELOAD_MSG: PCWSTR = w!("PocketKnives.Readout.Reload.v1");
const EXIT_MSG: PCWSTR = w!("PocketKnives.Readout.Exit.v1");
const WM_APPBAR: u32 = WM_APP + 1;
const WM_MOUSELEAVE: u32 = 0x02A3;
const TIMER_TICK: usize = 1;
/// While a tile is hovered: where the pointer is (it may cross the gap to the panel, where
/// neither window hears it leave), the list's countdowns, and the glare following the pointer.
const TIMER_POINTER: usize = 2;
const POINTER_MS: u32 = 33;
/// How far the glare moves towards the pointer each tick.
const GLARE_EASE: f32 = 0.35;
/// A selection lasts this long; a refusal shows this long.
const SEL_MS: u64 = 5000;
const DENIED_MS: u64 = 2500;
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
    /// The panel as drawn (at its whole drawing scale), with its glare, and as shown.
    panel_draw: Canvas,
    panel_lit: Canvas,
    /// Where the glare is, and where the pointer would take it (see `bar::panel_glare`).
    glare: (f32, f32),
    glare_to: (f32, f32),
    panel_canvas: Canvas,
    /// The panel's part that never changes, for `panel_item` (`None` when it must be drawn again).
    panel_back: Canvas,
    panel_surface: Surface,
    panel_item: Option<Item>,
    hover: Option<usize>,
    tracking: bool,
    fullscreen: bool,
    /// The process list: what it shows, the item it's for, whether the pointer is on the panel
    /// (the rows hold still then), and when the selection and the refusal began.
    list: ListView,
    list_item: Option<Item>,
    on_panel: bool,
    sel_at: u64,
    denied_at: u64,
    /// Programs' icons by name, read once each (none: no readable icon).
    icons: Vec<(String, Option<Icon>)>,
    reload_msg: u32,
    exit_msg: u32,
}

pub fn run() -> u32 {
    let Some(guard) = NamedMutex::first(MUTEX_NAME) else { return 0 };
    guard.keep();
    let hinst = unsafe { GetModuleHandleW(null()) };
    type Proc = unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT;
    for (class, proc_) in [(BAR_CLASS, bar_proc as Proc), (PANEL_CLASS, panel_proc as Proc)] {
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
    let panel_hwnd = unsafe { CreateWindowExW(ex, PANEL_CLASS, w!("Readout panel"), WS_POPUP, 0, 0, 1, 1, hwnd, null_mut(), hinst, null()) };
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
        panel_draw: Canvas::new(1, 1),
        panel_lit: Canvas::new(1, 1),
        glare: (0.0, 0.0),
        glare_to: (0.0, 0.0),
        panel_canvas: Canvas::new(1, 1),
        panel_back: Canvas::new(1, 1),
        panel_surface: Surface::new(1, 1),
        panel_item: None,
        hover: None,
        tracking: false,
        fullscreen: false,
        list: ListView::default(),
        list_item: None,
        on_panel: false,
        sel_at: 0,
        denied_at: 0,
        icons: Vec::new(),
        reload_msg: register_message(RELOAD_MSG),
        exit_msg: register_message(EXIT_MSG),
    });
    let mut abd = appbar(hwnd);
    abd.uCallbackMessage = WM_APPBAR;
    unsafe { SHAppBarMessage(ABM_NEW, &mut abd) };
    app.sample();
    app.place();
    let ptr = Box::into_raw(app) as isize;
    unsafe {
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, ptr);
        SetWindowLongPtrW(panel_hwnd, GWLP_USERDATA, ptr);
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

fn ms() -> u64 {
    unsafe { GetTickCount64() }
}

/// A point in a window's client area from a mouse message's `lp`, in device pixels.
fn point(lp: LPARAM) -> (i32, i32) {
    ((lp & 0xFFFF) as i16 as i32, ((lp >> 16) & 0xFFFF) as i16 as i32)
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
        // the panels a step smaller where the tallest wouldn't fit under the bar
        let room = r.bottom - abd.rc.bottom - PANEL_GAP * k;
        while self.layout.panel_halves > 2 * k && bar::panel_h(Item::Cpu) * self.layout.panel_halves > 2 * room {
            self.layout.panel_halves -= 1;
        }
        self.origin = (abd.rc.left, abd.rc.top);
        let (w, h) = self.layout.size();
        if self.canvas.w != w || self.canvas.h != h {
            self.canvas = Canvas::new(w, h);
            self.back = Canvas::new(w, h);
            self.surface = Surface::new(w, h);
        }
        self.back_hover = None;
        self.panel_item = None;
        self.set_hover(None);
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

    /// The hovered tile's item, when it has a panel.
    fn panel_item(&self) -> Option<Item> {
        self.hover.map(|i| self.layout.tiles[i].item).filter(|&t| t != Item::Clock)
    }

    /// The panel's body on the screen, in device pixels: left, top, right, bottom.
    fn panel_rect(&self, item: Item) -> (i32, i32, i32, i32) {
        let (k, s) = (self.layout.k, self.layout.panel_scale());
        let left = self.origin.0 + (self.layout.panel_x(self.hover.unwrap_or(0)) * k as f32) as i32;
        let top = self.origin.1 + (BAR_H + PANEL_GAP) * k;
        (left, top, left + (PANEL_W as f32 * s) as i32, top + (bar::panel_h(item) as f32 * s) as i32)
    }

    fn draw_panel(&mut self) {
        let Some(item) = self.panel_item() else {
            unsafe { ShowWindow(self.panel_hwnd, SW_HIDE) };
            return;
        };
        let (k, halve) = self.layout.panel_draw();
        if self.panel_item != Some(item) {
            let (pw, ph) = bar::panel_size(k, item);
            if self.panel_draw.w != pw || self.panel_draw.h != ph {
                self.panel_draw = Canvas::new(pw, ph);
                self.panel_lit = Canvas::new(pw, ph);
                self.panel_back = Canvas::new(pw, ph);
                let (sw, sh) = if halve { (pw.div_ceil(2), ph.div_ceil(2)) } else { (pw, ph) };
                self.panel_canvas = Canvas::new(sw, sh);
                self.panel_surface = Surface::new(sw, sh);
            }
            bar::panel_back(&mut self.panel_back, k, &self.pal, item);
            self.panel_item = Some(item);
        }
        self.update_list(item);
        self.panel_draw.px.copy_from_slice(&self.panel_back.px);
        bar::panel_front(&mut self.panel_draw, k, &self.pal, &self.model, item, &self.list);
        self.show_panel(item);
        unsafe { ShowWindow(self.panel_hwnd, SW_SHOWNOACTIVATE) };
    }

    /// Puts the drawn panel on the screen with its glare where it is now.
    fn show_panel(&mut self, item: Item) {
        let (k, halve) = self.layout.panel_draw();
        self.panel_lit.px.copy_from_slice(&self.panel_draw.px);
        bar::panel_glare(&mut self.panel_lit, k, &self.pal, item, self.glare);
        if halve {
            self.panel_lit.halve_into(&mut self.panel_canvas);
        } else {
            self.panel_canvas.px.copy_from_slice(&self.panel_lit.px);
        }
        let (left, top, _, _) = self.panel_rect(item);
        let margin = (PANEL_MARGIN as f32 * self.layout.panel_scale()) as i32;
        self.panel_surface.show(self.panel_hwnd, &self.panel_canvas, left - margin, top - margin);
    }

    /// Eases the glare towards where the pointer would put it, and shows it when it moved.
    fn move_glare(&mut self, item: Item) {
        let mut pt = POINT { x: 0, y: 0 };
        unsafe { GetCursorPos(&mut pt) };
        let (left, top, _, _) = self.panel_rect(item);
        let s = self.layout.panel_scale();
        let (gx, gy, gw, gh) = bar::list_glass();
        let (cx, cy) = (left as f32 + (gx + gw / 2.0) * s, top as f32 + (gy + gh / 2.0) * s);
        // the reach, in design pixels, that takes the glare to its ends
        self.glare_to = (((pt.x as f32 - cx) / s / 133.0).clamp(-1.0, 1.0), ((pt.y as f32 - cy) / s / 167.0).clamp(-1.0, 1.0));
        let (dx, dy) = (self.glare_to.0 - self.glare.0, self.glare_to.1 - self.glare.1);
        if dx.abs() + dy.abs() < 0.004 {
            return;
        }
        self.glare = (self.glare.0 + dx * GLARE_EASE, self.glare.1 + dy * GLARE_EASE);
        self.show_panel(item);
    }

    /// The list's rows and states for `item` now. The rows hold still while the pointer is on
    /// the panel or anything is selected, held or refused, so a key never moves under it; then
    /// a program that ends leaves its row blank.
    fn update_list(&mut self, item: Item) {
        if self.list_item != Some(item) {
            self.list = ListView::default();
            self.list_item = Some(item);
        }
        let t = ms();
        if self.list.sel.is_some() && t.saturating_sub(self.sel_at) >= SEL_MS {
            self.list.sel = None;
        }
        if self.list.denied.is_some() && t.saturating_sub(self.denied_at) >= DENIED_MS {
            self.list.denied = None;
        }
        let still = self.on_panel || self.list.sel.is_some() || self.list.down.is_some() || self.list.denied.is_some();
        if !still || self.list.rows.is_empty() {
            self.list.rows = bar::ranking(&self.model, item);
            self.list.icons = self.list.rows.clone().iter().map(|n| self.icon(n)).collect();
        }
        if let Some(i) = self.list.sel
            && !self.running(i)
        {
            self.list.sel = None;
        }
    }

    /// Program `name`'s icon, read from its file the first time it's listed.
    fn icon(&mut self, name: &str) -> Option<Icon> {
        if let Some(e) = self.icons.iter().find(|e| e.0 == name) {
            return e.1;
        }
        // a bound on what's kept: the programs that come and go over a long session
        if self.icons.len() >= 64 {
            self.icons.clear();
        }
        let icon = icons::for_rows(&[String::from(name)], &self.model.last.procs)[0];
        self.icons.push((String::from(name), icon));
        icon
    }

    /// Whether row `i`'s program is running (in the latest sample).
    fn running(&self, i: usize) -> bool {
        bar::row_proc(&self.model, &self.list.rows, i).is_some()
    }

    /// Hovers tile `hover` (or none): per-program sampling and the pointer timer run only while a
    /// panel can be open.
    fn set_hover(&mut self, hover: Option<usize>) {
        if hover == self.hover && hover.is_some() {
            return;
        }
        self.hover = hover;
        self.on_panel = false;
        let item = self.panel_item();
        self.collector.set_procs(item.is_some_and(bar::has_list));
        unsafe {
            if hover.is_some() {
                SetTimer(self.hwnd, TIMER_POINTER, POINTER_MS, None);
            } else {
                KillTimer(self.hwnd, TIMER_POINTER);
            }
        }
    }

    /// Follows the pointer (screen device pixels): the tile under it on the bar; or, on the way
    /// to the open panel or on it, the same tile; or nothing.
    fn follow(&mut self, x: i32, y: i32) {
        let k = self.layout.k;
        let (bx, by) = (x - self.origin.0, y - self.origin.1);
        let mut hover = if by >= 0 && by < BAR_H * k { self.layout.hit(bx as f32 / k as f32, by as f32 / k as f32) } else { None };
        let mut on_panel = false;
        if let Some(item) = self.panel_item() {
            let (l, t, r, b) = self.panel_rect(item);
            if x >= l && x < r && y >= self.origin.1 && y < b && (hover.is_none() || hover == self.hover) {
                hover = self.hover;
                on_panel = y >= t;
            }
        }
        let was_on_panel = self.on_panel;
        if hover != self.hover {
            self.set_hover(hover);
            self.on_panel = on_panel;
            self.draw();
        } else if on_panel != was_on_panel {
            self.on_panel = on_panel;
            self.draw_panel();
        }
    }

    fn follow_cursor(&mut self) {
        let mut pt = POINT { x: 0, y: 0 };
        unsafe { GetCursorPos(&mut pt) };
        self.follow(pt.x, pt.y);
    }

    /// The pointer timer: where the pointer went, and the list's countdowns.
    fn pointer_tick(&mut self) {
        if self.list.down.is_none() {
            self.follow_cursor();
        }
        let (sel, denied) = (self.list.sel, self.list.denied);
        if let Some(item) = self.panel_item() {
            self.update_list(item);
            if (self.list.sel, self.list.denied) != (sel, denied) {
                self.draw_panel();
            }
            if bar::has_list(item) {
                self.move_glare(item);
            }
        }
    }

    fn mouse(&mut self, lp: LPARAM) {
        if !self.tracking {
            let mut tme = TRACKMOUSEEVENT { cbSize: core::mem::size_of::<TRACKMOUSEEVENT>() as u32, dwFlags: TME_LEAVE, hwndTrack: self.hwnd, dwHoverTime: 0 };
            unsafe { TrackMouseEvent(&mut tme) };
            self.tracking = true;
        }
        let (x, y) = point(lp);
        self.follow(self.origin.0 + x, self.origin.1 + y);
    }

    /// The key under a point in the panel window (device pixels).
    fn key_at(&self, lp: LPARAM) -> Option<usize> {
        let item = self.panel_item()?;
        let k = self.layout.panel_scale();
        let (x, y) = point(lp);
        bar::key_at(item, x as f32 / k - PANEL_MARGIN as f32, y as f32 / k - PANEL_MARGIN as f32)
    }

    fn key_down(&mut self, lp: LPARAM) {
        if let Some(i) = self.key_at(lp).filter(|&i| self.running(i)) {
            self.list.down = Some(i);
            unsafe { SetCapture(self.panel_hwnd) };
            self.draw_panel();
        }
    }

    fn key_up(&mut self, lp: LPARAM) {
        let Some(i) = self.list.down.take() else { return };
        unsafe { ReleaseCapture() };
        if self.key_at(lp) == Some(i) {
            self.press(i);
        }
        self.draw_panel();
    }

    /// X on row `i`: the first press selects its program, a second press on the selected row
    /// ends it. A press within a double-click's time of the selection is ignored, so a
    /// double-click only selects.
    fn press(&mut self, i: usize) {
        let t = ms();
        self.list.denied = None;
        if self.list.sel != Some(i) {
            self.list.sel = Some(i);
            self.sel_at = t;
            return;
        }
        if t.saturating_sub(self.sel_at) < unsafe { GetDoubleClickTime() } as u64 {
            return;
        }
        self.list.sel = None;
        if procs::kill(&self.list.rows[i]) == Kill::Denied {
            self.list.denied = Some(i);
            self.denied_at = t;
        }
    }

    fn reload(&mut self) {
        self.settings = Settings::load();
        self.pal = load_theme(&self.settings.theme);
        self.place();
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

/// The App both windows share, or `None` before it's set and after the bar is gone.
fn app_of(hwnd: HWND) -> Option<&'static mut App> {
    let ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) } as *mut App;
    if ptr.is_null() { None } else { Some(unsafe { &mut *ptr }) }
}

unsafe extern "system" fn bar_proc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    let Some(app) = app_of(hwnd) else { return unsafe { DefWindowProcW(hwnd, msg, wp, lp) } };
    match msg {
        WM_TIMER if wp == TIMER_TICK => {
            app.sample();
            if !app.fullscreen {
                app.draw();
            }
            0
        }
        WM_TIMER if wp == TIMER_POINTER => {
            app.pointer_tick();
            0
        }
        WM_MOUSEMOVE => {
            app.mouse(lp);
            0
        }
        WM_MOUSELEAVE => {
            app.tracking = false;
            app.follow_cursor();
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
                let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut App;
                SetWindowLongPtrW(app.panel_hwnd, GWLP_USERDATA, 0);
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

/// The panel: it never takes focus; a press on a key holds it down until the button comes up.
unsafe extern "system" fn panel_proc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    let Some(app) = app_of(hwnd) else { return unsafe { DefWindowProcW(hwnd, msg, wp, lp) } };
    match msg {
        WM_MOUSEACTIVATE => MA_NOACTIVATE as LRESULT,
        WM_LBUTTONDOWN => {
            app.key_down(lp);
            0
        }
        WM_LBUTTONUP => {
            app.key_up(lp);
            0
        }
        WM_CAPTURECHANGED => {
            if app.list.down.take().is_some() {
                app.draw_panel();
            }
            0
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wp, lp) },
    }
}
