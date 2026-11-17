//! The Windows side of the tray icon: a hidden top-level window (a message-only window never gets
//! "TaskbarCreated" or `WM_SETTINGCHANGE`), `Shell_NotifyIcon` v4, `SetTimer`, the popup menu and
//! HICONs (docs/specs/tray.md). All behaviour is the shared [`TrayController`]; this module only
//! connects it to Win32.
//!
//! Threads: everything here runs on the UI thread, which owns the window, the controller and
//! every core `Op`/`Done` (they're `Rc`). Device work runs on the [`Worker`]; BLE scans on their
//! own thread. Both send results back with `PostMessage(WM_APP_RESULT, id, boxed result)`; the UI
//! thread looks the id up in its table of pending `Done`s and completes it. Endpoint
//! notifications arrive on audio-service threads and post `WM_APP_DEVCHANGE`.
//!
//! Scheduling: a woken controller task is queued and a `WM_APP_INVOKE` posted; each such message
//! runs one task (C#'s `SynchronizationContext.Post`), so continuations also run inside
//! `TrackPopupMenuEx`'s modal loop. `delay` is a window timer with an id above the controller's.
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
use core::sync::atomic::{AtomicPtr, Ordering};
use pk::wide::to_wide;
use quicktooth_core::Guid;
use quicktooth_core::battery::BatteryReading;
use quicktooth_core::blade_settings::{self, SetResult};
use quicktooth_core::controller::{ALL_TIMERS, TrayController};
use quicktooth_core::devices::{OneShot, SendResult, Snapshot};
use quicktooth_core::exec::{self, Done, Op, OpResult, Schedule, Task};
use quicktooth_core::host::{DeviceBackend, IconImage, MenuItem, TrayHost};
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows_sys::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx};
use windows_sys::Win32::System::DataExchange::COPYDATASTRUCT;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::HiDpi::{GetDpiForSystem, GetDpiForWindow, GetSystemMetricsForDpi};
use windows_sys::Win32::UI::Shell::{
    NIF_ICON, NIF_MESSAGE, NIF_SHOWTIP, NIF_TIP, NIM_ADD, NIM_DELETE, NIM_MODIFY, NIM_SETVERSION, NIN_SELECT, NINF_KEY,
    NOTIFYICON_VERSION_4, NOTIFYICONDATAW, NOTIFYICONIDENTIFIER, Shell_NotifyIconGetRect, Shell_NotifyIconW,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, ChangeWindowMessageFilterEx, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyMenu, DestroyWindow,
    DispatchMessageW, EndMenu, FindWindowW, GetCursorPos, GetMessageW, HMENU, KillTimer, MF_CHECKED, MF_GRAYED, MF_POPUP, MF_SEPARATOR,
    MF_STRING, MSG, MSGFLT_ALLOW, PostMessageW, PostQuitMessage, RegisterClassExW, SM_CXSMICON, SetForegroundWindow, SetTimer,
    TPM_BOTTOMALIGN, TPM_RETURNCMD, TPM_RIGHTBUTTON, TrackPopupMenuEx, TranslateMessage, WM_APP, WM_CONTEXTMENU, WM_COPYDATA, WM_DESTROY,
    WM_DISPLAYCHANGE, WM_DPICHANGED, WM_NULL, WM_SETTINGCHANGE, WM_TIMER, WNDCLASSEXW, WS_EX_TOOLWINDOW, WS_POPUP,
};

const WM_TRAY: u32 = WM_APP + 1;
/// Space or Enter on the focused icon (shellapi.h: `NIN_SELECT | NINF_KEY`; windows-sys has no constant).
const NIN_KEYSELECT: u32 = NIN_SELECT | NINF_KEY;
const WM_APP_INVOKE: u32 = WM_APP + 2;
const WM_APP_DEVCHANGE: u32 = WM_APP + 3;
/// A worker or scan result: wParam = its id in [`Ui::pending`], lParam = `Box<Box<dyn Any + Send>>`.
const WM_APP_RESULT: u32 = WM_APP + 4;
/// `delay` timers use ids from here up; the controller's own timers are 1–10.
const DELAY_TIMER_BASE: usize = 0x1000;

type Deliver = Box<dyn FnOnce(Box<dyn Any + Send>)>;

/// The UI thread's state. Made once, never freed; only ever touched on the UI thread. No
/// `RefCell` borrow is held across a call into the controller or a modal loop.
struct Ui {
    hwnd: Cell<HWND>,
    clock: clock::Stopwatch,
    controller: OnceCell<TrayController>,
    tasks: RefCell<VecDeque<Task>>,
    pending: RefCell<BTreeMap<usize, Deliver>>,
    delays: RefCell<BTreeMap<usize, Done<()>>>,
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

/// Sends a result to the UI thread (from any thread). If the window is gone, it's dropped.
fn post_result<T: Send + 'static>(hwnd: isize, id: usize, value: OpResult<T>) {
    let boxed: Box<dyn Any + Send> = Box::new(value);
    let raw = Box::into_raw(Box::new(boxed));
    if unsafe { PostMessageW(hwnd as HWND, WM_APP_RESULT, id, raw as LPARAM) } == 0 {
        drop(unsafe { Box::from_raw(raw) });
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
        pending: RefCell::new(BTreeMap::new()),
        delays: RefCell::new(BTreeMap::new()),
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
    let devices: Rc<dyn DeviceBackend> = Rc::new(WinDevices { ui, worker: Worker::start() });
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
            let task = ui.tasks.borrow_mut().pop_front();
            if let Some(t) = task {
                t.run();
            }
            return 0;
        }
        WM_APP_RESULT => {
            let value = *unsafe { Box::from_raw(lparam as *mut Box<dyn Any + Send>) };
            let deliver = ui.pending.borrow_mut().remove(&wparam);
            if let Some(deliver) = deliver {
                deliver(value);
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
            if wparam >= DELAY_TIMER_BASE {
                unsafe { KillTimer(hwnd, wparam) };
                let done = ui.delays.borrow_mut().remove(&wparam);
                if let Some(done) = done {
                    done.ok(());
                }
            } else if let Some(c) = c {
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
        self.0.tasks.borrow_mut().push_back(task);
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
        let id = DELAY_TIMER_BASE + (self.0.next() % (usize::MAX - DELAY_TIMER_BASE));
        self.0.delays.borrow_mut().insert(id, done);
        unsafe { SetTimer(self.0.hwnd(), id, ms as u32, None) };
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
        append(menu, items);
        unsafe { SetForegroundWindow(hwnd) }; // so the menu closes when clicking elsewhere
        let cmd = unsafe { TrackPopupMenuEx(menu, TPM_RETURNCMD | TPM_RIGHTBUTTON | TPM_BOTTOMALIGN, x, y, hwnd, null()) };
        unsafe { PostMessageW(hwnd, WM_NULL, 0, 0) };
        unsafe { DestroyMenu(menu) }; // also destroys the submenus
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

    fn emit(&self, _json: &str) {} // the simulator's timeline only
}

fn append(menu: HMENU, items: &[MenuItem]) {
    for item in items {
        let grayed = if item.grayed { MF_GRAYED } else { 0 };
        let text = item.text.as_deref().map(to_wide);
        let text_ptr = text.as_ref().map_or(null(), |t| t.as_ptr());
        if item.separator {
            unsafe { AppendMenuW(menu, MF_SEPARATOR, 0, null()) };
        } else if let Some(children) = &item.submenu {
            let sub = unsafe { CreatePopupMenu() };
            append(sub, children);
            unsafe { AppendMenuW(menu, MF_POPUP | grayed, sub as usize, text_ptr) };
        } else {
            let checked = if item.checked { MF_CHECKED } else { 0 };
            unsafe { AppendMenuW(menu, MF_STRING | grayed | checked, item.id as usize, text_ptr) };
        }
    }
}

// ---------------------------------------------------------------------------- DeviceBackend

struct WinDevices {
    ui: &'static Ui,
    worker: Worker,
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
}
