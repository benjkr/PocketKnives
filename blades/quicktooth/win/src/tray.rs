//! The Windows side of the tray icon: a hidden top-level window (a message-only window never gets
//! "TaskbarCreated" or `WM_SETTINGCHANGE`), `Shell_NotifyIcon` v4, `SetTimer`, the popup menu and
//! HICONs (docs/specs/tray.md). All behaviour is the shared [`TrayController`]; this module only
//! connects it to Win32.
//!
//! Threads: everything here runs on the UI thread, which owns the window, the controller and
//! every core `Op`/`Done` (they're `Rc`). Device work runs on the [`Worker`]; BLE scans on their
//! own thread. Both queue their result (with the id of its pending `Done`) in [`RESULTS`] and post
//! `WM_APP_INVOKE`; no message carries a pointer, so a stray or forged one does nothing. Endpoint
//! notifications arrive on audio-service threads and post `WM_APP_DEVCHANGE`.
//!
//! Scheduling, as C#'s `SynchronizationContext` did: a woken task is queued and a `WM_APP_INVOKE`
//! posted, and each `WM_APP_INVOKE` runs everything queued, results and tasks in the order they
//! arrived, including what's queued meanwhile. A result completes its `Done` and runs the task it
//! wakes right there (C# queued the continuation itself). Continuations also run inside
//! `TrackPopupMenuEx`'s modal loop. `delay` is a thread-pool timer whose expiry queues its result
//! like any other (C#'s `Task.Delay`), so it takes its place in arrival order.
use crate::audio::Audio;
use crate::worker::Worker;
use crate::{ble, clock, files, icons, ipc, pnp, system};
use alloc::boxed::Box;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::any::Any;
use core::cell::{Cell, OnceCell, RefCell};
use core::ptr::{null, null_mut};
use core::sync::atomic::{AtomicPtr, AtomicU64, Ordering};
use pk::wide::to_wide;
use quicktooth_core::Guid;
use quicktooth_core::battery::BatteryReading;
use quicktooth_core::blade_settings::{self, SetResult};
use quicktooth_core::controller::{ALL_TIMERS, TrayController};
use quicktooth_core::devices::{OneShot, SendResult, Snapshot};
use quicktooth_core::exec::{self, Done, Op, OpResult, Schedule, Task};
use quicktooth_core::host::{DeviceBackend, IconImage, MenuItem, TrayHost};
use quicktooth_core::popup::{Align, Header};
use windows_sys::Win32::Graphics::Gdi::{
    BI_RGB, BITMAPINFO, BITMAPINFOHEADER, CLEARTYPE_QUALITY, CLIP_DEFAULT_PRECIS, CreateCompatibleDC, CreateDIBSection, CreateFontW,
    DEFAULT_CHARSET, DEFAULT_PITCH, DIB_RGB_COLORS, DeleteDC, DeleteObject, GetDC, HBITMAP, OUT_DEFAULT_PRECIS, ReleaseDC, SelectObject,
    SetBkMode, SetTextAlign, SetTextColor, TA_BASELINE, TA_CENTER, TA_LEFT, TA_RIGHT, TRANSPARENT, TextOutW,
};
use windows_sys::Win32::Foundation::{HANDLE, HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows_sys::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx};
use windows_sys::Win32::System::DataExchange::COPYDATASTRUCT;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::Threading::{CreateTimerQueueTimer, DeleteTimerQueueTimer, WT_EXECUTEONLYONCE};
use windows_sys::Win32::UI::HiDpi::{GetDpiForSystem, GetDpiForWindow, GetSystemMetricsForDpi};
use windows_sys::Win32::UI::Shell::{
    NIF_ICON, NIF_MESSAGE, NIF_SHOWTIP, NIF_TIP, NIM_ADD, NIM_DELETE, NIM_MODIFY, NIM_SETVERSION, NIN_SELECT, NINF_KEY,
    NOTIFYICON_VERSION_4, NOTIFYICONDATAW, NOTIFYICONIDENTIFIER, Shell_NotifyIconGetRect, Shell_NotifyIconW,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, ChangeWindowMessageFilterEx, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyMenu, DestroyWindow,
    DispatchMessageW, EndMenu, FindWindowW, GetCursorPos, GetMessageW, HMENU, KillTimer, MF_BITMAP, MF_CHECKED, MF_GRAYED,
    MF_POPUP, MF_SEPARATOR,
    MF_STRING, MSG, MSGFLT_ALLOW, PostMessageW, PostQuitMessage, RegisterClassExW, SM_CXSMICON, SetForegroundWindow, SetTimer,
    TPM_BOTTOMALIGN, TPM_RETURNCMD, TPM_RIGHTBUTTON, TrackPopupMenuEx, TranslateMessage, WM_APP, WM_CONTEXTMENU, WM_COPYDATA, WM_DESTROY,
    WM_DISPLAYCHANGE, WM_DPICHANGED, WM_NULL, WM_SETTINGCHANGE, WM_TIMER, WNDCLASSEXW, WS_EX_TOOLWINDOW, WS_POPUP,
};

const WM_TRAY: u32 = WM_APP + 1;
/// Space or Enter on the focused icon (shellapi.h: `NIN_SELECT | NINF_KEY`; windows-sys has no constant).
const NIN_KEYSELECT: u32 = NIN_SELECT | NINF_KEY;
const WM_APP_INVOKE: u32 = WM_APP + 2;
const WM_APP_DEVCHANGE: u32 = WM_APP + 3;
const WM_APP_ADVERT: u32 = WM_APP + 4;

type Deliver = Box<dyn FnOnce(Box<dyn Any + Send>)>;

/// The UI thread's state. Made once, never freed; only ever touched on the UI thread. No
/// `RefCell` borrow is held across a call into the controller or a modal loop.
struct Ui {
    hwnd: Cell<HWND>,
    clock: clock::Stopwatch,
    controller: OnceCell<TrayController>,
    /// Woken tasks, with their arrival number (shared with [`RESULTS`], so both run in order).
    tasks: RefCell<VecDeque<(u64, Task)>>,
    /// The next task woken runs at once (the one a result being delivered wakes).
    run_woken_now: Cell<bool>,
    pending: RefCell<BTreeMap<usize, Deliver>>,
    /// `delay`'s thread-pool timers by result id, deleted once their result is delivered.
    delay_timers: RefCell<BTreeMap<usize, isize>>,
    next_id: Cell<usize>,
    msg_taskbar_created: Cell<u32>,
    msg_command: Cell<u32>,
    destroying: Cell<bool>,
}

/// The UI state, set before the window exists (null on any other thread's view is never used).
static UI: AtomicPtr<Ui> = AtomicPtr::new(null_mut());

impl Ui {
    fn hwnd(&self) -> HWND {
        self.hwnd.get()
    }

    fn next(&self) -> usize {
        let id = self.next_id.get();
        self.next_id.set(id.wrapping_add(1));
        id
    }

    /// Registers `done` for a result posted later with [`post_result`]; returns its id.
    fn expect<T: 'static>(&self, done: Done<T>) -> usize {
        let id = self.next();
        let deliver: Deliver = Box::new(move |b: Box<dyn Any + Send>| match b.downcast::<OpResult<T>>() {
            Ok(v) => done.complete(*v),
            Err(_) => done.complete(Err("internal: result of the wrong type".into())),
        });
        self.pending.borrow_mut().insert(id, deliver);
        id
    }

    fn controller(&self) -> Option<TrayController> {
        self.controller.get().cloned()
    }
}

/// Results from the worker and scan threads for the UI thread: (arrival number, the id of the
/// pending `Done`, the value).
static RESULTS: pk::sync::Mutex<VecDeque<(u64, usize, Box<dyn Any + Send>)>> = pk::sync::Mutex::new(VecDeque::new());
/// Status adverts from the advert watch (thread-pool threads) for the UI thread. One
/// `WM_APP_ADVERT` is posted while it's non-empty; past `ADVERTS_MAX` the oldest are dropped.
static ADVERTS: pk::sync::Mutex<VecDeque<(Vec<u8>, i32)>> = pk::sync::Mutex::new(VecDeque::new());
const ADVERTS_MAX: usize = 64;
/// Arrival numbers for results and woken tasks.
static ARRIVAL: AtomicU64 = AtomicU64::new(0);

/// Sends a result to the UI thread (from any thread). If the window is gone, it's never read.
fn post_result<T: Send + 'static>(hwnd: isize, id: usize, value: OpResult<T>) {
    let boxed: Box<dyn Any + Send> = Box::new(value);
    {
        let mut q = RESULTS.lock();
        q.push_back((ARRIVAL.fetch_add(1, Ordering::Relaxed), id, boxed));
    }
    unsafe { PostMessageW(hwnd as HWND, WM_APP_INVOKE, 0, 0) };
}

/// A `delay` timer expired (on a thread-pool thread): queue its result for the UI thread.
unsafe extern "system" fn delay_expired(param: *mut core::ffi::c_void, _: bool) {
    let (hwnd, id) = *unsafe { Box::from_raw(param as *mut (isize, usize)) };
    post_result(hwnd, id, OpResult::<()>::Ok(()));
}

static DELAY_FAILED: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

/// Runs everything queued for the UI thread, oldest first (C#'s `UiContext.Drain`). Nested calls
/// (a modal loop inside a task) carry on with the same queues.
fn drain(ui: &Ui) {
    loop {
        let task_at = ui.tasks.borrow().front().map(|t| t.0);
        let result_at = RESULTS.lock().front().map(|r| r.0);
        match (task_at, result_at) {
            (None, None) => return,
            (Some(t), r) if r.is_none_or(|r| t < r) => {
                let next = ui.tasks.borrow_mut().pop_front();
                if let Some((_, task)) = next {
                    task.run();
                }
            }
            _ => {
                let next = RESULTS.lock().pop_front();
                let Some((_, id, value)) = next else { continue };
                let deliver = ui.pending.borrow_mut().remove(&id);
                if let Some(deliver) = deliver {
                    ui.run_woken_now.set(true);
                    deliver(value);
                    ui.run_woken_now.set(false);
                }
                let timer = ui.delay_timers.borrow_mut().remove(&id);
                if let Some(t) = timer {
                    // It has fired; no completion event, so this never waits.
                    unsafe { DeleteTimerQueueTimer(null_mut(), t as HANDLE, null_mut()) };
                }
            }
        }
    }
}

pub fn run() -> u32 {
    let Some(mutex) = pk::instance::NamedMutex::first(ipc::MUTEX) else {
        quicktooth_core::log::info("another tray instance is already running; exiting");
        return 0;
    };
    mutex.keep();
    unsafe { CoInitializeEx(null(), COINIT_APARTMENTTHREADED as u32) };

    let ui: &'static Ui = Box::leak(Box::new(Ui {
        hwnd: Cell::new(null_mut()),
        clock: clock::Stopwatch::start(),
        controller: OnceCell::new(),
        tasks: RefCell::new(VecDeque::new()),
        run_woken_now: Cell::new(false),
        pending: RefCell::new(BTreeMap::new()),
        delay_timers: RefCell::new(BTreeMap::new()),
        next_id: Cell::new(1),
        msg_taskbar_created: Cell::new(0),
        msg_command: Cell::new(0),
        destroying: Cell::new(false),
    }));
    UI.store(ui as *const Ui as *mut Ui, Ordering::Release);

    let instance = unsafe { GetModuleHandleW(null()) };
    let wc = WNDCLASSEXW {
        cbSize: size_of::<WNDCLASSEXW>() as u32,
        lpfnWndProc: Some(wndproc),
        hInstance: instance,
        lpszClassName: ipc::WINDOW_CLASS,
        ..unsafe { core::mem::zeroed() }
    };
    if unsafe { RegisterClassExW(&wc) } == 0 {
        quicktooth_core::log::error("tray", &alloc::format!("RegisterClassEx failed ({})", files::last_error_message()));
        return 1;
    }
    let hwnd = unsafe {
        CreateWindowExW(
            WS_EX_TOOLWINDOW,
            ipc::WINDOW_CLASS,
            pk::w!("QuickTooth"),
            WS_POPUP,
            0,
            0,
            0,
            0,
            null_mut(),
            null_mut(),
            instance,
            null(),
        )
    };
    if hwnd.is_null() {
        quicktooth_core::log::error("tray", &alloc::format!("CreateWindowEx failed ({})", files::last_error_message()));
        return 1;
    }
    ui.hwnd.set(hwnd);
    ui.msg_taskbar_created.set(pk::instance::register_message(pk::w!("TaskbarCreated")));
    ui.msg_command.set(ipc::command_message());
    // If we ever run elevated, let Explorer's broadcast and non-elevated CLI commands through UIPI.
    for msg in [ui.msg_taskbar_created.get(), ui.msg_command.get(), WM_COPYDATA] {
        unsafe { ChangeWindowMessageFilterEx(hwnd, msg, MSGFLT_ALLOW, null_mut()) };
    }

    let schedule: Rc<dyn Schedule> = Rc::new(UiSchedule(ui));
    let host: Rc<dyn TrayHost> = Rc::new(WinHost(ui));
    let devices: Rc<dyn DeviceBackend> = Rc::new(WinDevices { ui, worker: Worker::start(), watch: RefCell::new(None) });
    let controller = TrayController::new(host, devices, files::load_settings(), schedule);
    let _ = ui.controller.set(controller.clone());
    controller.start();

    let mut msg: MSG = unsafe { core::mem::zeroed() };
    while unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) } > 0 {
        unsafe {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    0
}

// ---------------------------------------------------------------------------- window procedure

fn lo_word(v: usize) -> i32 {
    (v & 0xFFFF) as u16 as i16 as i32
}

fn hi_word(v: usize) -> i32 {
    ((v >> 16) & 0xFFFF) as u16 as i16 as i32
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let p = UI.load(Ordering::Acquire);
    if p.is_null() {
        return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
    }
    let ui: &'static Ui = unsafe { &*p };
    let c = ui.controller(); // None while CreateWindowEx is still running
    match msg {
        WM_TRAY => {
            if let Some(c) = c {
                match lo_word(lparam as usize) as u32 {
                    NIN_SELECT => c.on_tray_event("select", 0, 0),       // left click (v4)
                    NIN_KEYSELECT => c.on_tray_event("keyselect", 0, 0), // Space/Enter on the focused icon
                    // the anchor is in wParam (v4)
                    WM_CONTEXTMENU => c.on_tray_event("contextmenu", lo_word(wparam), hi_word(wparam)),
                    _ => {}
                }
            }
            return 0;
        }
        WM_APP_INVOKE => {
            drain(ui);
            return 0;
        }
        WM_APP_ADVERT => {
            let adverts = core::mem::take(&mut *ADVERTS.lock());
            if let Some(c) = c {
                for (data, rssi) in adverts {
                    c.on_advert(&data, rssi);
                }
            }
            return 0;
        }
        WM_APP_DEVCHANGE => {
            if let Some(c) = c {
                c.on_endpoint_changed();
            }
            return 0;
        }
        WM_TIMER => {
            if let Some(c) = c {
                c.on_timer(wparam as i32);
            }
            return 0;
        }
        WM_COPYDATA => {
            return match c {
                Some(c) => on_settings_request(&c, lparam) as LRESULT,
                None => 0,
            };
        }
        WM_SETTINGCHANGE => {
            if let Some(c) = c {
                let area = (lparam != 0).then(|| unsafe { pk::wide::from_ptr(lparam as *const u16) });
                c.on_setting_change(area.as_deref());
            }
        }
        WM_DISPLAYCHANGE | WM_DPICHANGED => {
            if let Some(c) = c {
                c.on_display_change();
            }
        }
        WM_DESTROY => {
            ui.destroying.set(true);
            if let Some(c) = c {
                c.exit();
            }
            unsafe { PostQuitMessage(0) };
            return 0;
        }
        _ => {
            if msg != 0 && msg == ui.msg_taskbar_created.get() {
                if let Some(c) = c {
                    c.on_taskbar_created();
                }
                return 0;
            }
            if msg != 0 && msg == ui.msg_command.get() {
                if let Some(c) = c {
                    c.on_command(wparam as i32);
                }
                return 0;
            }
        }
    }
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

/// `QuickTooth.exe --set`, sent with `SendMessageTimeout(WM_COPYDATA)`: the key=value pairs, one
/// per line, in UTF-16. Returns the [`SetResult`]; 0 (not ours, or unreadable) means not handled.
fn on_settings_request(c: &TrayController, lparam: LPARAM) -> i32 {
    if lparam == 0 {
        return 0;
    }
    let cds = unsafe { &*(lparam as *const COPYDATASTRUCT) };
    if cds.dwData != ipc::SET_SETTINGS_DATA
        || cds.cbData == 0
        || cds.cbData % 2 != 0
        || cds.cbData > ipc::SET_SETTINGS_MAX_BYTES
        || cds.lpData.is_null()
    {
        return 0;
    }
    let units = unsafe { core::slice::from_raw_parts(cds.lpData as *const u16, cds.cbData as usize / 2) };
    let text = String::from_utf16_lossy(units);
    let pairs: Vec<&str> = text.split('\n').collect();
    match blade_settings::parse(&pairs) {
        Ok(change) => c.apply_settings(&change) as i32,
        Err(_) => SetResult::Invalid as i32,
    }
}

// ---------------------------------------------------------------------------- scheduler

struct UiSchedule(&'static Ui);

impl Schedule for UiSchedule {
    fn schedule(&self, task: Task) {
        if self.0.run_woken_now.replace(false) {
            task.run();
            return;
        }
        self.0.tasks.borrow_mut().push_back((ARRIVAL.fetch_add(1, Ordering::Relaxed), task));
        unsafe { PostMessageW(self.0.hwnd(), WM_APP_INVOKE, 0, 0) };
    }
}

// ---------------------------------------------------------------------------- TrayHost

struct WinHost(&'static Ui);

impl WinHost {
    fn nid(&self) -> NOTIFYICONDATAW {
        NOTIFYICONDATAW {
            cbSize: size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: self.0.hwnd(),
            uID: ipc::ICON_ID,
            ..unsafe { core::mem::zeroed() }
        }
    }
}

/// `szTip`: at most 127 characters and a terminating zero.
fn set_tip(nid: &mut NOTIFYICONDATAW, tip: &str) {
    let mut n = 0;
    for u in tip.encode_utf16().take(nid.szTip.len() - 1) {
        nid.szTip[n] = u;
        n += 1;
    }
    nid.szTip[n] = 0;
}

impl TrayHost for WinHost {
    fn exe_path(&self) -> String {
        pk::env::exe_path()
    }

    fn now_ms(&self) -> f64 {
        self.0.clock.elapsed_ms()
    }

    fn now_epoch_ms(&self) -> f64 {
        clock::epoch_ms()
    }

    fn delay(&self, ms: i32) -> Op<()> {
        if ms <= 0 {
            return Op::ok(());
        }
        let (op, done) = exec::op();
        let id = self.0.expect(done);
        let param = Box::into_raw(Box::new((self.0.hwnd() as isize, id)));
        let mut timer: HANDLE = null_mut();
        let made =
            unsafe { CreateTimerQueueTimer(&mut timer, null_mut(), Some(delay_expired), param.cast(), ms as u32, 0, WT_EXECUTEONLYONCE) };
        if made == 0 {
            drop(unsafe { Box::from_raw(param) });
            // C#'s Task.Delay couldn't fail: finish now rather than never (logged once).
            if !DELAY_FAILED.swap(true, Ordering::Relaxed) {
                quicktooth_core::log::error("delay", "CreateTimerQueueTimer failed");
            }
            let deliver = self.0.pending.borrow_mut().remove(&id);
            if let Some(deliver) = deliver {
                deliver(Box::new(OpResult::<()>::Ok(())));
            }
            return op;
        }
        self.0.delay_timers.borrow_mut().insert(id, timer as isize);
        op
    }

    fn set_timer(&self, id: i32, ms: i32) {
        unsafe { SetTimer(self.0.hwnd(), id as usize, ms as u32, None) };
    }

    fn kill_timer(&self, id: i32) {
        unsafe { KillTimer(self.0.hwnd(), id as usize) };
    }

    fn kill_all_timers(&self) {
        for id in ALL_TIMERS {
            self.kill_timer(id);
        }
    }

    fn light_taskbar(&self) -> bool {
        system::taskbar_is_light()
    }

    /// The small-icon size at the taskbar's DPI (16 px at 96 DPI).
    fn icon_size(&self) -> i32 {
        let tray = unsafe { FindWindowW(pk::w!("Shell_TrayWnd"), null()) };
        let mut dpi = if tray.is_null() { 0 } else { unsafe { GetDpiForWindow(tray) } };
        if dpi == 0 {
            dpi = unsafe { GetDpiForSystem() };
        }
        let size = unsafe { GetSystemMetricsForDpi(SM_CXSMICON, dpi) };
        if size > 0 { size } else { 16 }
    }

    fn apply_menu_theme(&self, light: bool) {
        system::apply_menu_theme(self.0.hwnd(), light);
    }

    fn add_icon(&self, icon: &IconImage, tip: &str) -> bool {
        let mut nid = self.nid();
        nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP | NIF_SHOWTIP;
        nid.uCallbackMessage = WM_TRAY;
        nid.hIcon = icons::handle_for(icon);
        set_tip(&mut nid, tip);
        if unsafe { Shell_NotifyIconW(NIM_ADD, &nid) } == 0 {
            // Probably a stale icon from a previous session (or Explorer isn't ready yet at sign-in).
            unsafe { Shell_NotifyIconW(NIM_DELETE, &nid) };
            if unsafe { Shell_NotifyIconW(NIM_ADD, &nid) } == 0 {
                return false;
            }
        }
        nid.Anonymous.uVersion = NOTIFYICON_VERSION_4;
        unsafe { Shell_NotifyIconW(NIM_SETVERSION, &nid) };
        true
    }

    fn modify_icon(&self, icon: &IconImage, tip: &str) -> bool {
        let mut nid = self.nid();
        nid.uFlags = NIF_ICON | NIF_TIP | NIF_SHOWTIP;
        nid.hIcon = icons::handle_for(icon);
        set_tip(&mut nid, tip);
        unsafe { Shell_NotifyIconW(NIM_MODIFY, &nid) != 0 }
    }

    fn remove_icon(&self) {
        let nid = self.nid();
        unsafe { Shell_NotifyIconW(NIM_DELETE, &nid) };
    }

    fn release_icon(&self, icon: &IconImage) {
        icons::release(icon);
    }

    fn cursor_pos(&self) -> Option<(i32, i32)> {
        let mut pt = POINT { x: 0, y: 0 };
        (unsafe { GetCursorPos(&mut pt) } != 0).then_some((pt.x, pt.y))
    }

    fn icon_anchor(&self) -> Option<(i32, i32)> {
        let id = NOTIFYICONIDENTIFIER {
            cbSize: size_of::<NOTIFYICONIDENTIFIER>() as u32,
            hWnd: self.0.hwnd(),
            uID: ipc::ICON_ID,
            guidItem: unsafe { core::mem::zeroed() },
        };
        let mut r = unsafe { core::mem::zeroed() };
        (unsafe { Shell_NotifyIconGetRect(&id, &mut r) } == 0).then_some(((r.left + r.right) / 2, r.top))
    }

    /// Shows the menu modally (`TrackPopupMenuEx`), so the returned op is already complete.
    fn track_popup_menu(&self, items: &[MenuItem], x: i32, y: i32, _light: bool) -> Op<i32> {
        let hwnd = self.0.hwnd();
        let menu = unsafe { CreatePopupMenu() };
        let bitmaps = append(menu, items);
        unsafe { SetForegroundWindow(hwnd) }; // so the menu closes when clicking elsewhere
        let cmd = unsafe { TrackPopupMenuEx(menu, TPM_RETURNCMD | TPM_RIGHTBUTTON | TPM_BOTTOMALIGN, x, y, hwnd, null()) };
        unsafe { PostMessageW(hwnd, WM_NULL, 0, 0) };
        unsafe { DestroyMenu(menu) }; // also destroys the submenus
        for b in bitmaps {
            unsafe { DeleteObject(b) };
        }
        Op::ok(cmd)
    }

    fn end_menu(&self) {
        unsafe { EndMenu() };
    }

    fn promote_self(&self) -> String {
        system::promote_self().name().into()
    }

    fn autostart_enabled(&self) -> bool {
        system::autostart_enabled()
    }

    fn set_autostart(&self, on: bool) {
        system::set_autostart(on);
    }

    fn exit(&self) {
        if !self.0.destroying.get() {
            unsafe { DestroyWindow(self.0.hwnd()) };
        }
    }

    fn emit(&self, _event: &quicktooth_core::host::Event<'_>) {} // the simulator's timeline only
}

/// The menu's picture header as a bitmap: the core's pixels, then its text in Segoe UI. A bitmap
/// item, not an owner-drawn one: one owner-drawn item turns off Windows' theme for the whole menu.
pub(crate) fn header_bitmap(h: &Header) -> HBITMAP {
    let colorref = |c: u32| ((c & 0xFF) << 16) | (c & 0xFF00) | (c >> 16);
    let mut bmi: BITMAPINFO = unsafe { core::mem::zeroed() };
    bmi.bmiHeader.biSize = size_of::<BITMAPINFOHEADER>() as u32;
    bmi.bmiHeader.biWidth = h.width;
    bmi.bmiHeader.biHeight = -h.height; // top-down
    bmi.bmiHeader.biPlanes = 1;
    bmi.bmiHeader.biBitCount = 32;
    bmi.bmiHeader.biCompression = BI_RGB;
    unsafe {
        let screen = GetDC(null_mut());
        let dc = CreateCompatibleDC(screen);
        ReleaseDC(null_mut(), screen);
        let mut bits = null_mut();
        let bmp = CreateDIBSection(dc, &bmi, DIB_RGB_COLORS, &mut bits, null_mut(), 0);
        if bmp.is_null() || bits.is_null() {
            DeleteDC(dc);
            return bmp;
        }
        core::ptr::copy_nonoverlapping(h.bgra.as_ptr(), bits as *mut u8, h.bgra.len().min((h.width * h.height * 4) as usize));
        let old_bmp = SelectObject(dc, bmp);
        SetBkMode(dc, TRANSPARENT as i32);
        for t in &h.texts {
            let font = CreateFontW(
                -t.size, 0, 0, 0, if t.bold { 600 } else { 400 }, 0, 0, 0, DEFAULT_CHARSET as u32, OUT_DEFAULT_PRECIS as u32,
                CLIP_DEFAULT_PRECIS as u32, CLEARTYPE_QUALITY as u32, DEFAULT_PITCH as u32, pk::w!("Segoe UI"),
            );
            let old = SelectObject(dc, font);
            let align = match t.align {
                Align::Left => TA_LEFT,
                Align::Center => TA_CENTER,
                Align::Right => TA_RIGHT,
            };
            SetTextAlign(dc, align | TA_BASELINE);
            let text = to_wide(&t.text);
            SetTextColor(dc, colorref(t.color));
            TextOutW(dc, t.x, t.y, text.as_ptr(), (text.len() - 1) as i32); // without the terminating null
            SelectObject(dc, old);
            DeleteObject(font);
        }
        SelectObject(dc, old_bmp);
        DeleteDC(dc);
        // GDI's text leaves alpha at 0, and the themed menu blends 32-bit bitmaps by alpha: the
        // picture is opaque.
        let px = core::slice::from_raw_parts_mut(bits as *mut u8, (h.width * h.height * 4) as usize);
        for p in px.chunks_exact_mut(4) {
            p[3] = 0xFF;
        }
        bmp
    }
}

/// Fills `menu`; returns the header bitmaps, to delete once the menu is destroyed.
pub(crate) fn append(menu: HMENU, items: &[MenuItem]) -> Vec<HBITMAP> {
    let mut bitmaps = Vec::new();
    for item in items {
        let grayed = if item.grayed { MF_GRAYED } else { 0 };
        let text = item.text.as_deref().map(to_wide);
        let text_ptr = text.as_ref().map_or(null(), |t| t.as_ptr());
        if item.separator {
            unsafe { AppendMenuW(menu, MF_SEPARATOR, 0, null()) };
        } else if let Some(h) = &item.header {
            let bmp = header_bitmap(h);
            // Enabled: Windows dims a disabled bitmap. Picking it does nothing.
            unsafe { AppendMenuW(menu, MF_BITMAP, item.id as usize, bmp as *const u16) };
            bitmaps.push(bmp);
        } else if let Some(children) = &item.submenu {
            let sub = unsafe { CreatePopupMenu() };
            bitmaps.extend(append(sub, children));
            unsafe { AppendMenuW(menu, MF_POPUP | grayed, sub as usize, text_ptr) };
        } else {
            let checked = if item.checked { MF_CHECKED } else { 0 };
            unsafe { AppendMenuW(menu, MF_STRING | grayed | checked, item.id as usize, text_ptr) };
        }
    }
    bitmaps
}

// ---------------------------------------------------------------------------- DeviceBackend

struct WinDevices {
    ui: &'static Ui,
    worker: Worker,
    watch: RefCell<Option<ble::Watch>>,
}

impl WinDevices {
    /// Runs `f` on the device worker; its result comes back to the UI thread as the op's value.
    fn on_worker<T: Send + 'static>(&self, f: impl FnOnce(&mut Audio) -> OpResult<T> + Send + 'static) -> Op<T> {
        let (op, done) = exec::op();
        let id = self.ui.expect(done);
        let hwnd = self.ui.hwnd() as isize;
        self.worker.post(move |a| post_result(hwnd, id, f(a)));
        op
    }
}

impl DeviceBackend for WinDevices {
    fn register_notifications(&self) -> Op<()> {
        let hwnd = self.ui.hwnd() as isize;
        // Called on an audio-service thread: only post a message.
        let on_change: crate::audio::OnEndpointChanged = Arc::new(move |_: &str| unsafe {
            PostMessageW(hwnd as HWND, WM_APP_DEVCHANGE, 0, 0);
        });
        self.on_worker(move |a| a.register_notifications(on_change))
    }

    fn refresh(&self, mac: Option<&str>) -> Op<(Snapshot, Option<i32>)> {
        let mac: Option<String> = mac.map(String::from);
        self.on_worker(move |a| Ok((a.enumerate(false, false), mac.as_deref().and_then(pnp::bluetooth_battery))))
    }

    fn send_one_shot(&self, container: Guid, op: OneShot) -> Op<Vec<SendResult>> {
        self.on_worker(move |a| a.send_one_shot(container, op, false))
    }

    fn bluetooth_connected(&self, mac: &str) -> Op<Option<bool>> {
        let mac = String::from(mac);
        self.on_worker(move |_| Ok(pnp::bluetooth_connected(&mac)))
    }

    fn scan_battery(&self, model: u16, window_ms: i32) -> Op<Option<BatteryReading>> {
        let (op, done) = exec::op();
        let id = self.ui.expect(done);
        let hwnd = self.ui.hwnd() as isize;
        ble::scan_async(model, window_ms.max(0) as u32, move |scan| post_result(hwnd, id, Ok(scan.reading)));
        op
    }

    fn start_advert_watch(&self, model: u16) {
        let hwnd = self.ui.hwnd() as isize;
        let watch = ble::watch(model, move |data, rssi| {
            let first = {
                let mut q = ADVERTS.lock();
                if q.len() >= ADVERTS_MAX {
                    q.pop_front();
                }
                q.push_back((data, rssi));
                q.len() == 1
            };
            if first {
                unsafe { PostMessageW(hwnd as HWND, WM_APP_ADVERT, 0, 0) };
            }
        });
        let old = self.watch.replace(watch);
        drop(old); // stopped outside the borrow
    }

    fn stop_advert_watch(&self) {
        let old = self.watch.take();
        drop(old);
    }
}
