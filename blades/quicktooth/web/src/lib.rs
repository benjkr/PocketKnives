//! The simulator's side of the core: WebAssembly exports the page calls and the `qt-host`
//! imports it supplies (`sim/web/core.js`). Built only for wasm32-unknown-unknown, with std
//! (size doesn't matter in the browser).
//!
//! The ABI, all plain numbers:
//! - Strings are UTF-8 in linear memory, passed as pointer + length. A string the page passes in
//!   is in a block it got from [`qt_alloc`]; Rust copies it and frees the block. A null string is
//!   pointer 0 (a real block never is).
//! - A string or byte array coming back is in a result buffer: the export returns its pointer
//!   (0 for an error the page turns into an exception) and [`qt_result_len`] gives its length. The
//!   page copies it out at once.
//! - A sync import that returns a string writes the pointer and length of a block it allocated
//!   with [`qt_alloc`] to the two `u32`s at `out`.
//! - An async import takes an op ID. The page awaits its promise, then calls [`qt_complete`] with
//!   the value (a number or a string) or the error text, which completes the controller's [`Op`].
//! - Host objects (one `TrayAppModel` per app) are integer handles the page keeps.
use quicktooth_core::Guid;
use quicktooth_core::battery::BatteryReading;
use quicktooth_core::blade_settings::{self, SetResult};
use quicktooth_core::controller::{CMD_AUTOSTART, CMD_DEVICE_BASE, TrayController};
use quicktooth_core::devices::{self, BtDevice, BtEndpoint, KS_NOT_ATTEMPTED, OneShot, SendResult, Snapshot};
use quicktooth_core::exec::{self, Inline, Op, OpResult, Schedule};
use quicktooth_core::host::{DeviceBackend, Event, IconImage, MenuItem, TrayHost};
use quicktooth_core::icon::{self, AnimPhase, IconState};
use quicktooth_core::json::{self, Value};
use quicktooth_core::settings::{self, Settings};
use quicktooth_core::{log, tunables};
use std::alloc::{Layout, alloc, dealloc};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

// ---------------------------------------------------------------------------- imports

#[link(wasm_import_module = "qt-host")]
unsafe extern "C" {
    #[link_name = "exePath"]
    fn js_exe_path(h: i32, out: *mut u32);
    #[link_name = "nowMs"]
    fn js_now_ms(h: i32) -> f64;
    #[link_name = "nowEpochMs"]
    fn js_now_epoch_ms(h: i32) -> f64;
    #[link_name = "delay"]
    fn js_delay(h: i32, op: u32, ms: i32);
    #[link_name = "setTimer"]
    fn js_set_timer(h: i32, id: i32, ms: i32);
    #[link_name = "killTimer"]
    fn js_kill_timer(h: i32, id: i32);
    #[link_name = "killAllTimers"]
    fn js_kill_all_timers(h: i32);
    #[link_name = "lightTaskbar"]
    fn js_light_taskbar(h: i32) -> i32;
    #[link_name = "iconSize"]
    fn js_icon_size(h: i32) -> i32;
    #[link_name = "addIcon"]
    fn js_add_icon(
        h: i32,
        rgba: *const u8,
        rgba_len: usize,
        size: i32,
        tip: *const u8,
        tip_len: usize,
        desc: *const u8,
        desc_len: usize,
    ) -> i32;
    #[link_name = "modifyIcon"]
    fn js_modify_icon(
        h: i32,
        rgba: *const u8,
        rgba_len: usize,
        size: i32,
        tip: *const u8,
        tip_len: usize,
        desc: *const u8,
        desc_len: usize,
    ) -> i32;
    #[link_name = "removeIcon"]
    fn js_remove_icon(h: i32);
    #[link_name = "cursorPos"]
    fn js_cursor_pos(h: i32, out: *mut u32);
    #[link_name = "iconAnchor"]
    fn js_icon_anchor(h: i32, out: *mut u32);
    #[link_name = "trackPopupMenu"]
    fn js_track_popup_menu(h: i32, op: u32, items: *const u8, items_len: usize, x: i32, y: i32, light: i32);
    #[link_name = "endMenu"]
    fn js_end_menu(h: i32);
    #[link_name = "promoteSelf"]
    fn js_promote_self(h: i32, out: *mut u32);
    #[link_name = "autostartEnabled"]
    fn js_autostart_enabled(h: i32) -> i32;
    #[link_name = "setAutostart"]
    fn js_set_autostart(h: i32, on: i32);
    #[link_name = "exitApp"]
    fn js_exit_app(h: i32);
    #[link_name = "emit"]
    fn js_emit(h: i32, json: *const u8, len: usize);
    #[link_name = "log"]
    fn js_log(line: *const u8, len: usize);
    #[link_name = "saveSettings"]
    fn js_save_settings(json: *const u8, len: usize);
    #[link_name = "btRegister"]
    fn js_bt_register(h: i32, op: u32);
    #[link_name = "btRefresh"]
    fn js_bt_refresh(h: i32, op: u32, mac: *const u8, len: usize);
    #[link_name = "btSend"]
    fn js_bt_send(h: i32, op: u32, container: *const u8, container_len: usize, one_shot: *const u8, one_shot_len: usize);
    #[link_name = "btConnected"]
    fn js_bt_connected(h: i32, op: u32, mac: *const u8, len: usize);
    #[link_name = "btScan"]
    fn js_bt_scan(h: i32, op: u32, model: i32, window_ms: i32);
    #[link_name = "updateMenuHeader"]
    fn js_update_menu_header(h: i32, json: *const u8, len: usize);
    #[link_name = "btWatchStart"]
    fn js_bt_watch_start(h: i32, model: i32);
    #[link_name = "btWatchStop"]
    fn js_bt_watch_stop(h: i32);
    /// A Rust panic's message, for the console (the module traps right after).
    #[link_name = "panic"]
    fn js_panic(message: *const u8, len: usize);
}

// ---------------------------------------------------------------------------- memory and strings

/// A block for the page to write a string into (Rust frees it when it takes the string).
#[unsafe(no_mangle)]
pub extern "C" fn qt_alloc(len: usize) -> *mut u8 {
    if len == 0 {
        return 1 as *mut u8;
    }
    // Byte blocks never fail to lay out; a failed allocation traps in `alloc`'s error handler.
    unsafe { alloc(Layout::from_size_align(len, 1).unwrap()) }
}

#[unsafe(no_mangle)]
pub extern "C" fn qt_free(ptr: *mut u8, len: usize) {
    if len != 0 && !ptr.is_null() {
        unsafe { dealloc(ptr, Layout::from_size_align(len, 1).unwrap()) };
    }
}

/// Takes a string the page wrote into a [`qt_alloc`] block (and frees the block). `None` for
/// pointer 0.
fn take_str(ptr: *mut u8, len: usize) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    let s = if len == 0 { String::new() } else { String::from_utf8_lossy(unsafe { std::slice::from_raw_parts(ptr, len) }).into_owned() };
    qt_free(ptr, len);
    Some(s)
}

fn take(ptr: *mut u8, len: usize) -> String {
    take_str(ptr, len).unwrap_or_default()
}

/// A string a sync import hands back through `out` (pointer, length).
fn out_str(f: impl FnOnce(*mut u32)) -> String {
    let mut out = [0u32; 2];
    f(out.as_mut_ptr());
    take(out[0] as usize as *mut u8, out[1] as usize)
}

thread_local! {
    static RESULT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

/// Puts `bytes` in the result buffer; returns its pointer (never 0).
fn ret(bytes: Vec<u8>) -> *const u8 {
    RESULT.with(|r| {
        *r.borrow_mut() = bytes;
        let p = r.borrow().as_ptr();
        if p.is_null() { 1 as *const u8 } else { p }
    })
}

fn ret_str(s: String) -> *const u8 {
    ret(s.into_bytes())
}

/// The length of what the last export put in the result buffer.
#[unsafe(no_mangle)]
pub extern "C" fn qt_result_len() -> usize {
    RESULT.with(|r| r.borrow().len())
}

// ---------------------------------------------------------------------------- async ops

/// What the page settled an async import with: a number or a string.
struct Reply {
    num: f64,
    text: String,
}

type Completion = Box<dyn FnOnce(Result<Reply, String>)>;

thread_local! {
    static OPS: RefCell<HashMap<u32, Completion>> = RefCell::new(HashMap::new());
    static NEXT_OP: Cell<u32> = const { Cell::new(0) };
    /// One scheduler per page: woken tasks run at once, in order, never nested (what the C#
    /// browser build's `InlineContext` did).
    static SCHEDULE: Rc<dyn Schedule> = Rc::new(Inline::default());
}

/// A new async op: its ID for the import, and the [`Op`] the controller awaits. `convert` turns
/// the page's reply into the value (it runs when the reply arrives, before the task resumes).
fn pending<T: 'static>(convert: impl FnOnce(Reply) -> OpResult<T> + 'static) -> (u32, Op<T>) {
    let (op, done) = exec::op::<T>();
    let id = NEXT_OP.with(|n| {
        let id = n.get().wrapping_add(1);
        n.set(id);
        id
    });
    let completion: Completion = Box::new(move |r| done.complete(r.and_then(convert)));
    OPS.with(|ops| ops.borrow_mut().insert(id, completion));
    (id, op)
}

/// The page's promise for async op `op` settled: `ok` with `num` or the string at `ptr`/`len`,
/// or not `ok` with the error text there.
#[unsafe(no_mangle)]
pub extern "C" fn qt_complete(op: u32, ok: i32, num: f64, ptr: *mut u8, len: usize) {
    let text = take(ptr, len);
    // Out of the map before it runs: the task it wakes starts new ops.
    let Some(completion) = OPS.with(|ops| ops.borrow_mut().remove(&op)) else { return };
    completion(if ok != 0 { Ok(Reply { num, text }) } else { Err(text) });
}

// ---------------------------------------------------------------------------- hooks

fn sink(line: &str) {
    unsafe { js_log(line.as_ptr(), line.len()) };
}

fn save(s: &Settings) -> bool {
    // The page keeps settings.json; System.Text.Json wrote "\n" newlines in the browser.
    let text = s.to_json("\n");
    unsafe { js_save_settings(text.as_ptr(), text.len()) };
    true
}

/// Wires the core's log and settings saves to the page; called once after loading.
#[unsafe(no_mangle)]
pub extern "C" fn qt_init() {
    log::set_sink(sink);
    settings::set_save_handler(save);
    std::panic::set_hook(Box::new(|info| {
        let m = info.to_string();
        unsafe { js_panic(m.as_ptr(), m.len()) };
    }));
}

// ---------------------------------------------------------------------------- tunables, glyphs, renderer

#[unsafe(no_mangle)]
pub extern "C" fn qt_tunables() -> *const u8 {
    ret_str(tunables::to_json())
}

#[unsafe(no_mangle)]
pub extern "C" fn qt_set_tunable(name: *mut u8, name_len: usize, value: f64) -> i32 {
    tunables::set(&take(name, name_len), value) as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn qt_reset_tunables() {
    tunables::reset_all();
}

#[unsafe(no_mangle)]
pub extern "C" fn qt_set_glyphs(fill: *mut u8, fill_len: usize, line: *mut u8, line_len: usize) {
    let (fill, line) = (take_str(fill, fill_len), take_str(line, line_len));
    icon::set_glyphs(fill.as_deref(), line.as_deref());
}

/// Straight-alpha BGRA to RGBA (for `ImageData`).
fn to_rgba(bgra: &[u8]) -> Vec<u8> {
    let mut rgba = bgra.to_vec();
    for px in rgba.chunks_exact_mut(4) {
        px.swap(0, 2);
    }
    rgba
}

/// `state`: "connected", "disconnected" or "lowBattery" (any case). 0 for another name.
#[unsafe(no_mangle)]
pub extern "C" fn qt_render_static(state: *mut u8, state_len: usize, light: i32, size: i32) -> *const u8 {
    match IconState::parse(&take(state, state_len)) {
        Some(s) => ret(to_rgba(&icon::render_bgra(s, light != 0, size))),
        None => std::ptr::null(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn qt_render_anim(phase: *mut u8, phase_len: usize, t: f64, tracer_t: f64, light: i32, size: i32) -> *const u8 {
    match AnimPhase::parse(&take(phase, phase_len)) {
        Some(p) => ret(to_rgba(&icon::render_anim(p, t, tracer_t, light != 0, size))),
        None => std::ptr::null(),
    }
}

/// 1 or 0; -1 for an unknown phase.
#[unsafe(no_mangle)]
pub extern "C" fn qt_phase_done(phase: *mut u8, phase_len: usize, t: f64, still_pending_disconnect: i32) -> i32 {
    match AnimPhase::parse(&take(phase, phase_len)) {
        Some(p) => icon::phase_done(p, t, still_pending_disconnect != 0) as i32,
        None => -1,
    }
}

// ---------------------------------------------------------------------------- tray app instances

thread_local! {
    static APPS: RefCell<HashMap<i32, TrayController>> = RefCell::new(HashMap::new());
    static NEXT_APP: Cell<i32> = const { Cell::new(0) };
}

/// The app (cloned out of the map: the page calls `AppRunning` from inside host callbacks).
fn app(id: i32) -> TrayController {
    APPS.with(|a| a.borrow().get(&id).cloned()).unwrap_or_else(|| panic!("no app {id}"))
}

/// Creates an app driven by the page's host object `host`; returns its ID (0 if the settings
/// JSON can't be read).
#[unsafe(no_mangle)]
pub extern "C" fn qt_app_create(host: i32, settings: *mut u8, settings_len: usize) -> i32 {
    let Ok(settings) = Settings::from_json(&take(settings, settings_len)) else { return 0 };
    let schedule = SCHEDULE.with(Rc::clone);
    let controller = TrayController::new(Rc::new(WebHost(host)), Rc::new(WebDevices(host)), settings, schedule);
    let id = NEXT_APP.with(|n| {
        n.set(n.get() + 1);
        n.get()
    });
    APPS.with(|a| a.borrow_mut().insert(id, controller));
    id
}

#[unsafe(no_mangle)]
pub extern "C" fn qt_app_start(id: i32) {
    app(id).start();
}

#[unsafe(no_mangle)]
pub extern "C" fn qt_app_exit(id: i32) {
    app(id).exit();
}

#[unsafe(no_mangle)]
pub extern "C" fn qt_app_running(id: i32) -> i32 {
    APPS.with(|a| a.borrow().get(&id).cloned()).is_some_and(|c| c.running()) as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn qt_app_tray_event(id: i32, ev: *mut u8, ev_len: usize, x: i32, y: i32) {
    let ev = take(ev, ev_len);
    app(id).on_tray_event(&ev, x, y);
}

#[unsafe(no_mangle)]
pub extern "C" fn qt_app_command(id: i32, cmd: i32) {
    app(id).on_command(cmd);
}

#[unsafe(no_mangle)]
pub extern "C" fn qt_app_timer(id: i32, timer: i32) {
    app(id).on_timer(timer);
}

#[unsafe(no_mangle)]
pub extern "C" fn qt_app_setting_change(id: i32, area: *mut u8, area_len: usize) {
    let area = take_str(area, area_len);
    app(id).on_setting_change(area.as_deref());
}

#[unsafe(no_mangle)]
pub extern "C" fn qt_app_display_change(id: i32) {
    app(id).on_display_change();
}

#[unsafe(no_mangle)]
pub extern "C" fn qt_app_taskbar_created(id: i32) {
    app(id).on_taskbar_created();
}

#[unsafe(no_mangle)]
pub extern "C" fn qt_app_endpoint_changed(id: i32) {
    app(id).on_endpoint_changed();
}

/// A status advert from the mock watch (its bytes in a [`qt_alloc`] block, freed here).
#[unsafe(no_mangle)]
pub extern "C" fn qt_app_advert(id: i32, data: *mut u8, len: usize, rssi: i32) {
    let bytes = unsafe { core::slice::from_raw_parts(data, len) }.to_vec();
    qt_free(data, len);
    app(id).on_advert(&bytes, rssi);
}

#[unsafe(no_mangle)]
pub extern "C" fn qt_app_rebuild_icons(id: i32) {
    app(id).rebuild_icons_now();
}

#[unsafe(no_mangle)]
pub extern "C" fn qt_app_snapshot(id: i32) -> *const u8 {
    ret_str(app(id).snapshot_json())
}

#[unsafe(no_mangle)]
pub extern "C" fn qt_app_status_line(id: i32) -> *const u8 {
    ret_str(app(id).status_line())
}

/// `QuickTooth.exe --set` handed to the running app (key=value lines): "" when applied, else the
/// line the CLI prints.
#[unsafe(no_mangle)]
pub extern "C" fn qt_app_set(id: i32, pairs: *mut u8, pairs_len: usize) -> *const u8 {
    let pairs = take(pairs, pairs_len);
    let lines: Vec<&str> = pairs.split('\n').collect();
    let message = match blade_settings::parse(&lines) {
        Err(e) => e,
        Ok(change) => match app(id).apply_settings(&change) {
            SetResult::Applied => String::new(),
            r => blade_settings::message(r).into(),
        },
    };
    ret_str(message)
}

// ---------------------------------------------------------------------------- the host

/// [`TrayHost`] on the simulator page; `.0` is the page's handle for the app's host object.
struct WebHost(i32);

fn point(xy: &str) -> Option<(i32, i32)> {
    let mut parts = xy.split(',');
    let (x, y) = (parts.next()?, parts.next()?);
    if parts.next().is_some() {
        return None;
    }
    Some((x.trim().parse().ok()?, y.trim().parse().ok()?))
}

/// The menu as the mock shell takes it: `[{id,text,grayed,checked} | {separator} | {text,grayed,submenu}]`.
fn menu_json(items: &[MenuItem]) -> String {
    let mut w = String::new();
    write_menu(&mut w, items);
    w
}

fn write_menu(w: &mut String, items: &[MenuItem]) {
    w.push('[');
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            w.push(',');
        }
        w.push('{');
        if item.separator {
            w.push_str("\"separator\":true");
        } else {
            let mut sep = "";
            if item.submenu.is_none() {
                w.push_str(&format!("\"id\":{}", item.id));
                sep = ",";
            }
            w.push_str(sep);
            w.push_str("\"text\":");
            match &item.text {
                Some(t) => json::write_str(w, t),
                None => w.push_str("null"),
            }
            if item.grayed || item.submenu.is_some() {
                w.push_str(if item.grayed { ",\"grayed\":true" } else { ",\"grayed\":false" });
            }
            // Checkable items (devices, Start with Windows) always say whether they're checked.
            let checkable = item.id == CMD_AUTOSTART || item.id >= CMD_DEVICE_BASE;
            if item.checked || checkable {
                w.push_str(if item.checked { ",\"checked\":true" } else { ",\"checked\":false" });
            }
            if let Some(sub) = &item.submenu {
                w.push_str(",\"submenu\":");
                write_menu(w, sub);
            }
            if let Some(h) = &item.header {
                write_header(w, h);
            }
        }
        w.push('}');
    }
    w.push(']');
}

/// `,"header":{"w","h","background","rgba":<base64>,"texts":[{x,y,size,text,bold,align,color}]}`: the
/// owner-drawn picture item, as the exe draws it.
fn write_header(w: &mut String, h: &quicktooth_core::popup::Header) {
    w.push_str(&format!(",\"header\":{{\"w\":{},\"h\":{},\"background\":\"#{:06X}\",\"rgba\":\"", h.width, h.height, h.background));
    base64(w, &to_rgba(&h.bgra));
    w.push_str("\",\"texts\":[");
    for (i, t) in h.texts.iter().enumerate() {
        if i > 0 {
            w.push(',');
        }
        w.push_str(&format!("{{\"x\":{},\"y\":{},\"size\":{},\"text\":", t.x, t.y, t.size));
        json::write_str(w, &t.text);
        let align = match t.align {
            quicktooth_core::popup::Align::Left => "left",
            quicktooth_core::popup::Align::Center => "center",
            quicktooth_core::popup::Align::Right => "right",
        };
        w.push_str(&format!(",\"bold\":{},\"align\":\"{align}\",\"color\":\"#{:06X}\"}}", t.bold, t.color));
    }
    w.push_str("]}");
}

fn base64(w: &mut String, bytes: &[u8]) {
    const A: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    for c in bytes.chunks(3) {
        let n = (c[0] as u32) << 16 | (*c.get(1).unwrap_or(&0) as u32) << 8 | *c.get(2).unwrap_or(&0) as u32;
        for k in 0..4 {
            if k <= c.len() {
                w.push(A[(n >> (18 - 6 * k) & 63) as usize] as char);
            } else {
                w.push('=');
            }
        }
    }
}

impl WebHost {
    fn icon_call(
        &self,
        icon: &IconImage,
        tip: &str,
        f: unsafe extern "C" fn(i32, *const u8, usize, i32, *const u8, usize, *const u8, usize) -> i32,
    ) -> bool {
        let rgba = to_rgba(&icon.bgra);
        let desc = icon.desc.to_json();
        unsafe { f(self.0, rgba.as_ptr(), rgba.len(), icon.size, tip.as_ptr(), tip.len(), desc.as_ptr(), desc.len()) != 0 }
    }
}

impl TrayHost for WebHost {
    fn exe_path(&self) -> String {
        out_str(|out| unsafe { js_exe_path(self.0, out) })
    }

    fn now_ms(&self) -> f64 {
        unsafe { js_now_ms(self.0) }
    }

    fn now_epoch_ms(&self) -> f64 {
        unsafe { js_now_epoch_ms(self.0) }
    }

    fn delay(&self, ms: i32) -> Op<()> {
        let (id, op) = pending(|_| Ok(()));
        unsafe { js_delay(self.0, id, ms) };
        op
    }

    fn set_timer(&self, id: i32, ms: i32) {
        unsafe { js_set_timer(self.0, id, ms) };
    }

    fn kill_timer(&self, id: i32) {
        unsafe { js_kill_timer(self.0, id) };
    }

    fn kill_all_timers(&self) {
        unsafe { js_kill_all_timers(self.0) };
    }

    fn light_taskbar(&self) -> bool {
        unsafe { js_light_taskbar(self.0) != 0 }
    }

    fn icon_size(&self) -> i32 {
        unsafe { js_icon_size(self.0) }
    }

    /// The mock menu reads the theme when it opens.
    fn apply_menu_theme(&self, _light: bool) {}

    fn add_icon(&self, icon: &IconImage, tip: &str) -> bool {
        self.icon_call(icon, tip, js_add_icon)
    }

    fn modify_icon(&self, icon: &IconImage, tip: &str) -> bool {
        self.icon_call(icon, tip, js_modify_icon)
    }

    fn remove_icon(&self) {
        unsafe { js_remove_icon(self.0) };
    }

    fn release_icon(&self, _icon: &IconImage) {}

    fn cursor_pos(&self) -> Option<(i32, i32)> {
        point(&out_str(|out| unsafe { js_cursor_pos(self.0, out) }))
    }

    fn icon_anchor(&self) -> Option<(i32, i32)> {
        point(&out_str(|out| unsafe { js_icon_anchor(self.0, out) }))
    }

    fn track_popup_menu(&self, items: &[MenuItem], x: i32, y: i32, light: bool) -> Op<i32> {
        let json = menu_json(items);
        let (id, op) = pending(|r| Ok(r.num as i32));
        unsafe { js_track_popup_menu(self.0, id, json.as_ptr(), json.len(), x, y, light as i32) };
        op
    }

    fn update_menu_header(&self, header: &quicktooth_core::popup::Header) {
        // `{"header":…}`: the same object the menu's items carry.
        let mut json = String::from("{\"id\":0");
        write_header(&mut json, header);
        json.push('}');
        unsafe { js_update_menu_header(self.0, json.as_ptr(), json.len()) };
    }

    fn end_menu(&self) {
        unsafe { js_end_menu(self.0) };
    }

    fn promote_self(&self) -> String {
        out_str(|out| unsafe { js_promote_self(self.0, out) })
    }

    fn autostart_enabled(&self) -> bool {
        unsafe { js_autostart_enabled(self.0) != 0 }
    }

    fn set_autostart(&self, on: bool) {
        unsafe { js_set_autostart(self.0, on as i32) };
    }

    fn exit(&self) {
        unsafe { js_exit_app(self.0) };
    }

    fn emit(&self, event: &Event<'_>) {
        let json = event.to_json();
        unsafe { js_emit(self.0, json.as_ptr(), json.len()) };
    }
}

// ---------------------------------------------------------------------------- the mock Bluetooth stack

/// [`DeviceBackend`] on the simulator's mock Bluetooth stack (`sim/web/bluetooth.js`, behind the
/// page's device worker).
struct WebDevices(i32);

fn parse(text: &str) -> OpResult<Value> {
    json::parse(text)
}

fn missing(name: &str) -> String {
    format!("The given key '{name}' was not present.")
}

fn prop<'a>(v: &'a Value, name: &str) -> OpResult<&'a Value> {
    v.get(name).ok_or_else(|| missing(name))
}

fn str_prop(v: &Value, name: &str) -> OpResult<String> {
    prop(v, name)?.as_str().map(String::from).ok_or_else(|| format!("'{name}' is not a string."))
}

fn int_prop(v: &Value, name: &str) -> OpResult<i32> {
    prop(v, name)?.as_i32().ok_or_else(|| format!("'{name}' is not an Int32."))
}

fn array<'a>(v: &'a Value, what: &str) -> OpResult<&'a [Value]> {
    v.as_array().ok_or_else(|| format!("'{what}' is not an array."))
}

/// A number member as an `int`, `None` when it's missing or not a number.
fn level(v: &Value, name: &str) -> OpResult<Option<i32>> {
    match v.get(name) {
        Some(n @ Value::Num(_)) => n.as_i32().map(Some).ok_or_else(|| format!("'{name}' is not an Int32.")),
        _ => Ok(None),
    }
}

fn snapshot(text: &str) -> OpResult<(Snapshot, Option<i32>)> {
    let root = parse(text)?;
    let mut devices = Vec::new();
    for d in array(prop(&root, "devices")?, "devices")? {
        let container = Guid::parse(&str_prop(d, "containerId")?).ok_or("Unrecognized Guid format.")?;
        let mac = d.get("mac").and_then(Value::as_str).map(String::from);
        let mut endpoints = Vec::new();
        for e in array(prop(d, "endpoints")?, "endpoints")? {
            let bth = str_prop(e, "bthDeviceId")?;
            let state = prop(e, "state")?.as_f64().filter(|s| s.fract() == 0.0 && *s >= 0.0).ok_or("'state' is not a UInt32.")? as u32;
            endpoints.push(BtEndpoint {
                endpoint_id: bth.clone(),
                name: str_prop(e, "name")?,
                interface_name: None,
                state,
                bth_device_id: bth,
                container_id: container,
                mac: mac.clone(),
                hidden_from_enumeration: e.get("hidden").and_then(Value::as_bool).unwrap_or(false),
                ks_activate_hr: KS_NOT_ATTEMPTED,
            });
        }
        devices.push(BtDevice { container_id: container, name: str_prop(d, "name")?, mac, endpoints });
    }
    let windows_battery = level(&root, "windowsBattery")?;
    Ok((Snapshot { devices, ..Snapshot::default() }, windows_battery))
}

fn send_results(text: &str) -> OpResult<Vec<SendResult>> {
    let root = parse(text)?;
    array(&root, "results")?
        .iter()
        .map(|r| {
            Ok(SendResult {
                endpoint_name: str_prop(r, "endpointName")?,
                bth_device_id: str_prop(r, "bthDeviceId")?,
                hr: int_prop(r, "hr")?,
                bytes_returned: 0,
            })
        })
        .collect()
}

fn battery_reading(text: &str, now_epoch_ms: f64) -> OpResult<Option<BatteryReading>> {
    let r = parse(text)?;
    if !matches!(r, Value::Obj(_)) {
        return Ok(None);
    }
    let flag = |name: &str| r.get(name).and_then(Value::as_bool) == Some(true);
    let source = match prop(&r, "source")? {
        Value::Null => "ble".into(),
        v => v.as_str().map(String::from).ok_or("'source' is not a string.")?,
    };
    Ok(Some(BatteryReading {
        left: level(&r, "left")?,
        right: level(&r, "right")?,
        case: level(&r, "case")?,
        buds_charging: flag("budsCharging"),
        left_charging: flag("budsCharging"),
        right_charging: flag("budsCharging"),
        case_charging: flag("caseCharging"),
        rssi: level(&r, "rssi")?.unwrap_or(0),
        at_epoch_ms: now_epoch_ms,
        source,
    }))
}

impl DeviceBackend for WebDevices {
    fn register_notifications(&self) -> Op<()> {
        let (id, op) = pending(|_| Ok(()));
        unsafe { js_bt_register(self.0, id) };
        op
    }

    fn refresh(&self, mac: Option<&str>) -> Op<(Snapshot, Option<i32>)> {
        let mac = mac.unwrap_or(""); // "" = no target MAC
        let (id, op) = pending(|r| snapshot(&r.text));
        unsafe { js_bt_refresh(self.0, id, mac.as_ptr(), mac.len()) };
        op
    }

    fn send_one_shot(&self, container: Guid, one_shot: OneShot) -> Op<Vec<SendResult>> {
        let (cid, name) = (container.to_b(), format!("{one_shot}"));
        let (id, op) = pending(move |r| {
            let results = send_results(&r.text)?;
            devices::send_log(container, one_shot, &results);
            Ok(results)
        });
        unsafe { js_bt_send(self.0, id, cid.as_ptr(), cid.len(), name.as_ptr(), name.len()) };
        op
    }

    fn bluetooth_connected(&self, mac: &str) -> Op<Option<bool>> {
        // -1 unknown, 0, 1.
        let (id, op) = pending(|r| {
            Ok(match r.num as i32 {
                1 => Some(true),
                0 => Some(false),
                _ => None,
            })
        });
        unsafe { js_bt_connected(self.0, id, mac.as_ptr(), mac.len()) };
        op
    }

    fn scan_battery(&self, model: u16, window_ms: i32) -> Op<Option<BatteryReading>> {
        let h = self.0;
        // JSON or "null"; the reading is stamped when it arrives.
        let (id, op) = pending(move |r| battery_reading(&r.text, unsafe { js_now_epoch_ms(h) }));
        unsafe { js_bt_scan(self.0, id, model as i32, window_ms) };
        op
    }

    fn start_advert_watch(&self, model: u16) {
        unsafe { js_bt_watch_start(self.0, model as i32) };
    }

    fn stop_advert_watch(&self) {
        unsafe { js_bt_watch_stop(self.0) };
    }
}
