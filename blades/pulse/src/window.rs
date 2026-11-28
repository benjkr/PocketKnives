//! The bar's window: a strip in the main taskbar beside NowPlaying, redrawn every second, with a
//! panel beside the tile the pointer is on. The panel stays while the pointer is on it, and its
//! keys end programs.
use crate::bar::{self, BAR_H, Item, Layout, ListView, PANEL_GAP, PANEL_MARGIN, PANEL_W};
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
    HBITMAP, HDC, HGDIOBJ, MONITOR_DEFAULTTOPRIMARY, MONITORINFO, MonitorFromWindow, SelectObject,
};
use windows_sys::Win32::Graphics::Gdi::{AC_SRC_ALPHA, AC_SRC_OVER, BLENDFUNCTION};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::SystemInformation::GetTickCount64;
use windows_sys::Win32::UI::HiDpi::GetDpiForWindow;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    GetDoubleClickTime, ReleaseCapture, SetCapture, TME_LEAVE, TRACKMOUSEEVENT, TrackMouseEvent,
};
use windows_sys::Win32::UI::Shell::{ABM_NEW, ABM_REMOVE, ABN_FULLSCREENAPP, ABN_POSCHANGED, APPBARDATA, SHAppBarMessage};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyMenu, DestroyWindow, DispatchMessageW, GWLP_USERDATA,
    GW_HWNDPREV, GWLP_HWNDPARENT, GetCursorPos, GetMessageW, GetWindow, GetWindowLongPtrW, GetWindowRect, HWND_TOPMOST, IDC_ARROW, KillTimer, LoadCursorW, MA_NOACTIVATE,
    MF_CHECKED, MF_POPUP, MF_SEPARATOR, MF_STRING, MSG, PostMessageW, PostQuitMessage, RegisterClassExW, SW_HIDE,
    SW_SHOWNOACTIVATE, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SetForegroundWindow, SetTimer, SetWindowLongPtrW, SetWindowPos,
    ShowWindow, TPM_RETURNCMD, TPM_RIGHTBUTTON, TrackPopupMenu, TranslateMessage, ULW_ALPHA, UpdateLayeredWindow, WM_APP,
    WM_CAPTURECHANGED, WM_DESTROY, WM_DISPLAYCHANGE, WM_DPICHANGED, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEACTIVATE, WM_MOUSEMOVE,
    WM_RBUTTONUP, WM_TIMER, WNDCLASSEXW, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_POPUP,
};
use windows_sys::core::PCWSTR;

const MUTEX_NAME: PCWSTR = w!("Local\\PocketKnives.Pulse.v1");
const BAR_CLASS: PCWSTR = w!("PocketKnives.Pulse.Bar");
const PANEL_CLASS: PCWSTR = w!("PocketKnives.Pulse.Panel");
const RELOAD_MSG: PCWSTR = w!("PocketKnives.Pulse.Reload.v1");
const EXIT_MSG: PCWSTR = w!("PocketKnives.Pulse.Exit.v1");
/// In the taskbar, the bar goes after NowPlaying's strip (`blades/nowplaying/src/ipc.rs`).
const NOWPLAYING_CLASS: PCWSTR = w!("PocketKnives.NowPlaying.Strip");
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
/// Programs are read from when the pointer comes to the bar until it has been away this long.
const PROCS_LINGER_MS: u64 = 10_000;
/// A list that opens this soon after the reading began is read again this long after it began,
/// when there's time enough to tell the programs' loads apart.
const PROCS_SETTLE_MS: u64 = 300;
const TIMER_PROCS: usize = 3;
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
    /// The taskbar that owns the bar, and whether the panels open above it.
    tray: HWND,
    up: bool,
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
    /// While programs are read: since when, and until when without the pointer coming back.
    procs_from: u64,
    procs_until: u64,
    denied_at: u64,
    /// Programs' icons by name, read once each (none: no readable icon).
    icons: Vec<(String, Option<Icon>)>,
    reload_msg: u32,
    exit_msg: u32,
    taskbar_msg: u32,
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
    let hwnd = unsafe { CreateWindowExW(ex, BAR_CLASS, w!("Pulse"), WS_POPUP, 0, 0, 1, 1, null_mut(), null_mut(), hinst, null()) };
    let panel_hwnd = unsafe { CreateWindowExW(ex, PANEL_CLASS, w!("Pulse panel"), WS_POPUP, 0, 0, 1, 1, hwnd, null_mut(), hinst, null()) };
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
        layout: Layout::new(1, &settings.items()),
        settings,
        origin: (0, 0),
        tray: null_mut(),
        up: true,
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
        procs_from: 0,
        procs_until: 0,
        denied_at: 0,
        icons: Vec::new(),
        reload_msg: register_message(RELOAD_MSG),
        exit_msg: register_message(EXIT_MSG),
        taskbar_msg: register_message(w!("TaskbarCreated")),
    });
    // Registered as an app bar without claiming space, to hear of taskbar moves and full-screen apps.
    let mut abd = appbar(hwnd);
    abd.uCallbackMessage = WM_APPBAR;
    unsafe { SHAppBarMessage(ABM_NEW, &mut abd) };
    app.sample();
    app.place();
    let ptr = Box::into_raw(app) as isize;
    unsafe {
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, ptr);
        SetWindowLongPtrW(panel_hwnd, GWLP_USERDATA, ptr);
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

    /// Lays the bar out in the main taskbar after NowPlaying (or hides it while there is no
    /// taskbar, e.g. as Explorer restarts).
    fn place(&mut self) {
        let tray = find_window(w!("Shell_TrayWnd"));
        if tray != self.tray {
            // owned by the taskbar, the bar stays above it
            unsafe { SetWindowLongPtrW(self.hwnd, GWLP_HWNDPARENT, tray as isize) };
            self.tray = tray;
        }
        let Some((k, origin)) = self.dock() else {
            self.set_hover(None);
            unsafe {
                ShowWindow(self.panel_hwnd, SW_HIDE);
                ShowWindow(self.hwnd, SW_HIDE);
            }
            return;
        };
        let items = self.settings.items();
        let mut mi = MONITORINFO { cbSize: core::mem::size_of::<MONITORINFO>() as u32, ..unsafe { core::mem::zeroed() } };
        unsafe { GetMonitorInfoW(MonitorFromWindow(tray, MONITOR_DEFAULTTOPRIMARY), &mut mi) };
        let r = mi.rcMonitor;
        self.origin = origin;
        self.up = origin.1 > (r.top + r.bottom) / 2;
        self.layout = Layout::new(k, &items);
        let room = if self.up { origin.1 - r.top } else { r.bottom - origin.1 - BAR_H * k } - PANEL_GAP * k;
        // the panels a step smaller where the tallest wouldn't fit beside the bar
        while self.layout.panel_halves > 2 * k && bar::panel_h(Item::Cpu) * self.layout.panel_halves > 2 * room {
            self.layout.panel_halves -= 1;
        }
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
        if !self.fullscreen {
            unsafe { ShowWindow(self.hwnd, SW_SHOWNOACTIVATE) };
        }
    }

    /// The scale and the bar's top left in the taskbar, after NowPlaying's strip (or at the
    /// taskbar's left end without it) and centred top to bottom.
    fn dock(&self) -> Option<(i32, (i32, i32))> {
        let mut tb: RECT = unsafe { core::mem::zeroed() };
        if self.tray.is_null() || unsafe { GetWindowRect(self.tray, &mut tb) } == 0 {
            return None;
        }
        let k = scale(unsafe { GetDpiForWindow(self.tray) });
        let (ink, rows) = bar::ink_rows();
        let mut left = tb.left;
        let np = find_window(NOWPLAYING_CLASS);
        let mut r: RECT = unsafe { core::mem::zeroed() };
        if !np.is_null()
            && unsafe { GetWindowRect(np, &mut r) } != 0
            && r.top < tb.bottom
            && r.bottom > tb.top
            && (tb.left..tb.right).contains(&r.right)
        {
            left = r.right;
        }
        // what's drawn centred top to bottom
        Some((k, (left, tb.top + (tb.bottom - tb.top - rows * k).max(0) / 2 - ink * k)))
    }

    /// The second's tick: follows the taskbar and NowPlaying, and comes back above the taskbar
    /// when it rose over the bar.
    fn tick(&mut self) {
        if self.procs_until != 0 && self.procs_until <= ms() {
            self.collector.set_procs(false);
            self.procs_until = 0;
        }
        self.sample();
        if self.fullscreen {
            return;
        }
        if self.tray.is_null() {
            if !find_window(w!("Shell_TrayWnd")).is_null() {
                self.place();
            }
            return;
        }
        if self.dock() != Some((self.layout.k, self.origin)) {
            self.place();
            return;
        }
        if is_below(self.hwnd, self.tray) {
            unsafe { SetWindowPos(self.hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE) };
        }
        self.draw();
    }

    fn draw(&mut self) {
        if self.back_hover != Some(self.hover) {
            bar::background(&mut self.back, &self.layout, &self.pal, self.hover);
            self.back_hover = Some(self.hover);
        }
        self.canvas.px.copy_from_slice(&self.back.px);
        bar::foreground(&mut self.canvas, &self.layout, &self.pal, &self.model);
        self.surface.show(self.hwnd, &self.canvas, self.origin.0, self.origin.1);
        self.draw_panel();
    }

    /// The hovered tile's item.
    fn panel_item(&self) -> Option<Item> {
        self.hover.map(|i| self.layout.tiles[i].item)
    }

    /// The panel's body on the screen, in device pixels: left, top, right, bottom.
    fn panel_rect(&self, item: Item) -> (i32, i32, i32, i32) {
        let (k, s) = (self.layout.k, self.layout.panel_scale());
        let left = self.origin.0 + (self.layout.panel_x(self.hover.unwrap_or(0)) * k as f32) as i32;
        let h = (bar::panel_h(item) as f32 * s) as i32;
        let top = if self.up { self.origin.1 - PANEL_GAP * k - h } else { self.origin.1 + (BAR_H + PANEL_GAP) * k };
        (left, top, left + (PANEL_W as f32 * s) as i32, top + h)
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
        if bar::has_list(item) && self.model.last.procs.is_empty() {
            self.fill_procs();
        }
        let still = self.on_panel || self.list.sel.is_some() || self.list.down.is_some() || self.list.denied.is_some();
        if !still || self.list.rows.iter().all(String::is_empty) {
            self.list.rows = bar::ranking(&self.model, item);
            self.list.icons = self.list.rows.clone().iter().map(|n| self.icon(n)).collect();
        }
        if let Some(i) = self.list.sel
            && !self.running(i)
        {
            self.list.sel = None;
        }
    }

    /// The pointer is at the bar or its panel: programs are read from now (a first reading
    /// taken, so the next has their loads) until it has been away a while.
    fn wake_procs(&mut self) {
        let t = ms();
        if self.procs_until <= t {
            self.collector.set_procs(true);
            self.procs_from = t;
        }
        self.procs_until = t + PROCS_LINGER_MS;
    }

    /// A list opened before a second's sample had the programs: they're read now, and again
    /// once enough time has passed since the first reading to tell their loads apart.
    fn fill_procs(&mut self) {
        self.wake_procs();
        self.model.last.procs = self.collector.sample_procs();
        let age = ms().saturating_sub(self.procs_from);
        if age < PROCS_SETTLE_MS {
            unsafe { SetTimer(self.hwnd, TIMER_PROCS, (PROCS_SETTLE_MS - age) as u32, None) };
        }
    }

    /// The second reading [`fill_procs`] asked for.
    fn settle_procs(&mut self) {
        unsafe { KillTimer(self.hwnd, TIMER_PROCS) };
        if self.procs_until > ms() {
            self.model.last.procs = self.collector.sample_procs();
            self.draw_panel();
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

    /// Hovers tile `hover` (or none): the pointer timer runs only while a panel can be open.
    fn set_hover(&mut self, hover: Option<usize>) {
        if hover == self.hover && hover.is_some() {
            return;
        }
        self.hover = hover;
        self.on_panel = false;
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
            // the panel, the bar and the gap between them
            let (top, bottom) = (t.min(self.origin.1), b.max(self.origin.1 + BAR_H * k));
            if x >= l && x < r && y >= top && y < bottom && (hover.is_none() || hover == self.hover) {
                hover = self.hover;
                on_panel = y >= t && y < b;
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
        self.wake_procs();
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
        self.wake_procs();
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

/// Whether `hwnd` is under `other` in the z-order.
fn is_below(hwnd: HWND, other: HWND) -> bool {
    let mut w = unsafe { GetWindow(hwnd, GW_HWNDPREV) };
    while !w.is_null() {
        if w == other {
            return true;
        }
        w = unsafe { GetWindow(w, GW_HWNDPREV) };
    }
    false
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
            app.tick();
            0
        }
        WM_TIMER if wp == TIMER_PROCS => {
            app.settle_procs();
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
                    // Hide while a full-screen app is up, as the taskbar does.
                    app.fullscreen = lp != 0;
                    if app.fullscreen {
                        app.set_hover(None);
                        unsafe {
                            ShowWindow(app.panel_hwnd, SW_HIDE);
                            ShowWindow(hwnd, SW_HIDE);
                        }
                    } else if !app.tray.is_null() {
                        unsafe { ShowWindow(hwnd, SW_SHOWNOACTIVATE) };
                    }
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
        m if m == app.taskbar_msg => {
            // Explorer started again: a new taskbar to sit in
            app.place();
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
