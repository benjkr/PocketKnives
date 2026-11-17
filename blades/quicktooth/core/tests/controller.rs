//! The tray state machine against a scripted host: log lines, timers, the C# async ordering and
//! re-entrant host calls. (The full behaviour is checked by the simulator's scenarios, diffed
//! against the C# build; these pin the parts a refactor is most likely to break.)
use quicktooth_core::Guid;
use quicktooth_core::battery::BatteryReading;
use quicktooth_core::blade_settings::{SetResult, SettingsChange};
use quicktooth_core::controller::*;
use quicktooth_core::devices::{BtDevice, BtEndpoint, KS_NOT_ATTEMPTED, OneShot, SendResult, Snapshot, device_state};
use quicktooth_core::exec::{self, Done, Inline, Op, Schedule};
use quicktooth_core::host::{DeviceBackend, Event, IconImage, MenuItem, TrayHost};
use quicktooth_core::settings::Settings;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;

thread_local! {
    static LOG: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

fn sink(line: &str) {
    LOG.with(|l| l.borrow_mut().push(line.into()));
}

fn take_log() -> Vec<String> {
    LOG.with(|l| std::mem::take(&mut *l.borrow_mut()))
}

const CONTAINER: Guid = Guid::from_u128(0x5a1b2c3d_0001_0002_a1b2_0123456789ab);

fn airpods(connected: bool, name: &str) -> BtDevice {
    let state = if connected { device_state::ACTIVE } else { device_state::UNPLUGGED };
    let ep = |bth: &str| BtEndpoint {
        endpoint_id: bth.into(),
        name: "Headphones".into(),
        interface_name: None,
        state,
        bth_device_id: bth.into(),
        container_id: CONTAINER,
        mac: Some("307AD26790E8".into()),
        hidden_from_enumeration: false,
        ks_activate_hr: KS_NOT_ATTEMPTED,
    };
    BtDevice {
        container_id: CONTAINER,
        name: name.into(),
        mac: Some("307AD26790E8".into()),
        endpoints: vec![ep("{0}\\bthenum#a2dp_vid&0001004c_pid&2014"), ep("{0}\\bthhfenum#hfp")],
    }
}

/// The platform, scripted: records what the controller does, and holds every async op open
/// until the test completes it.
#[derive(Default)]
struct Mock {
    now: Cell<f64>,
    calls: RefCell<Vec<String>>,
    timers: RefCell<Vec<i32>>,
    tip: RefCell<String>,
    delays: RefCell<VecDeque<Done<()>>>,
    menus: RefCell<VecDeque<(Vec<MenuItem>, Done<i32>)>>,
    refreshes: RefCell<VecDeque<Done<(Snapshot, Option<i32>)>>>,
    sends: RefCell<VecDeque<(OneShot, Done<Vec<SendResult>>)>>,
    links: RefCell<VecDeque<Done<Option<bool>>>>,
    /// Called back from inside `modify_icon` (a re-entrant host call).
    on_modify: RefCell<Option<Box<dyn Fn()>>>,
    controller: RefCell<Option<TrayController>>,
}

impl Mock {
    fn call(&self, s: impl Into<String>) {
        self.calls.borrow_mut().push(s.into());
    }

    fn took(&self) -> Vec<String> {
        std::mem::take(&mut *self.calls.borrow_mut())
    }
}

impl TrayHost for Mock {
    fn exe_path(&self) -> String {
        "C:\\QuickTooth.exe".into()
    }
    fn now_ms(&self) -> f64 {
        self.now.get()
    }
    fn now_epoch_ms(&self) -> f64 {
        self.now.get()
    }
    fn delay(&self, ms: i32) -> Op<()> {
        self.call(format!("delay {ms}"));
        let (op, done) = exec::op();
        self.delays.borrow_mut().push_back(done);
        op
    }
    fn set_timer(&self, id: i32, _ms: i32) {
        let mut t = self.timers.borrow_mut();
        if !t.contains(&id) {
            t.push(id);
        }
    }
    fn kill_timer(&self, id: i32) {
        self.timers.borrow_mut().retain(|t| *t != id);
    }
    fn kill_all_timers(&self) {
        self.timers.borrow_mut().clear();
    }
    fn light_taskbar(&self) -> bool {
        false
    }
    fn icon_size(&self) -> i32 {
        16
    }
    fn apply_menu_theme(&self, _light: bool) {}
    fn add_icon(&self, _icon: &IconImage, tip: &str) -> bool {
        *self.tip.borrow_mut() = tip.into();
        self.call("add_icon");
        true
    }
    fn modify_icon(&self, _icon: &IconImage, tip: &str) -> bool {
        *self.tip.borrow_mut() = tip.into();
        if let Some(f) = &*self.on_modify.borrow() {
            f();
        }
        true
    }
    fn remove_icon(&self) {
        self.call("remove_icon");
    }
    fn release_icon(&self, _icon: &IconImage) {}
    fn cursor_pos(&self) -> Option<(i32, i32)> {
        Some((10, 20))
    }
    fn icon_anchor(&self) -> Option<(i32, i32)> {
        Some((5, 6))
    }
    fn track_popup_menu(&self, items: &[MenuItem], x: i32, y: i32, _light: bool) -> Op<i32> {
        self.call(format!("menu at {x},{y}"));
        let (op, done) = exec::op();
        self.menus.borrow_mut().push_back((items.to_vec(), done));
        op
    }
    fn end_menu(&self) {
        self.call("end_menu");
    }
    fn promote_self(&self) -> String {
        "Promoted".into()
    }
    fn autostart_enabled(&self) -> bool {
        false
    }
    fn set_autostart(&self, on: bool) {
        self.call(format!("autostart {on}"));
    }
    fn exit(&self) {
        // The exe destroys its window here, which calls exit() again (WM_DESTROY).
        let c = self.controller.borrow().clone();
        if let Some(c) = c {
            c.exit();
        }
        self.call("exit");
    }
    fn emit(&self, event: &Event<'_>) {
        self.call(format!("emit {}", event.to_json()));
    }
}

/// The device side delegates to the same mock.
struct Devices(Rc<Mock>);

impl DeviceBackend for Devices {
    fn register_notifications(&self) -> Op<()> {
        Op::ok(())
    }
    fn refresh(&self, _mac: Option<&str>) -> Op<(Snapshot, Option<i32>)> {
        let (op, done) = exec::op();
        self.0.refreshes.borrow_mut().push_back(done);
        op
    }
    fn send_one_shot(&self, _container: Guid, op: OneShot) -> Op<Vec<SendResult>> {
        let (o, done) = exec::op();
        self.0.sends.borrow_mut().push_back((op, done));
        o
    }
    fn bluetooth_connected(&self, _mac: &str) -> Op<Option<bool>> {
        let (op, done) = exec::op();
        self.0.links.borrow_mut().push_back(done);
        op
    }
    fn scan_battery(&self, _model: u16, _window_ms: i32) -> Op<Option<BatteryReading>> {
        Op::ok(None)
    }
}

fn setup(settings: Settings) -> (Rc<Mock>, TrayController) {
    quicktooth_core::log::set_sink(sink);
    quicktooth_core::settings::set_save_handler(|_| true);
    take_log();
    let mock = Rc::new(Mock::default());
    let schedule: Rc<dyn Schedule> = Rc::new(Inline::default());
    let c = TrayController::new(mock.clone(), Rc::new(Devices(mock.clone())), settings, schedule);
    *mock.controller.borrow_mut() = Some(c.clone());
    (mock, c)
}

fn refresh(mock: &Mock, devices: Vec<BtDevice>) {
    let done = mock.refreshes.borrow_mut().pop_front().expect("a refresh in flight");
    done.ok((Snapshot { devices, ..Snapshot::default() }, None));
}

fn ok(hr: i32) -> Vec<SendResult> {
    vec![SendResult { endpoint_name: "Headphones".into(), bth_device_id: "x".into(), hr, bytes_returned: 0 }]
}

#[test]
fn start_logs_adds_the_icon_and_finds_the_default_target() {
    let (mock, c) = setup(Settings::default());
    c.start();
    assert!(c.running());
    assert_eq!(take_log(), ["tray start: C:\\QuickTooth.exe", "tooltip: QuickTooth — Checking…", "tray icon added (16 px, dark taskbar)"]);
    assert_eq!(*mock.timers.borrow(), [TIMER_POLL, TIMER_BATTERY, TIMER_PROMOTE]);
    refresh(&mock, vec![airpods(false, "AirPods Pro")]);
    let log = take_log();
    assert!(log[0].starts_with("default target chosen: AirPods Pro {5a1b2c3d-0001-0002-a1b2-0123456789ab}"), "{log:?}");
    assert_eq!(log[1..], ["state AirPods Pro: not connected (A2DP=UNPLUGGED, HFP=UNPLUGGED)", "tooltip: AirPods Pro — Not connected"]);
    assert!(c.snapshot_json().contains("\"statusText\":\"Not connected\""));
}

#[test]
fn connect_waits_for_the_old_link_then_sends_and_confirms() {
    let (mock, c) = setup(Settings::default());
    c.start();
    refresh(&mock, vec![airpods(false, "AirPods Pro")]);
    take_log();
    mock.took();
    mock.now.set(1000.0);
    c.on_tray_event("select", 0, 0);
    // Synchronously up to the first wait: the link check.
    assert_eq!(c.status_text(), "Connecting…");
    assert_eq!(mock.links.borrow().len(), 1);
    let log = take_log();
    assert_eq!(log, ["click: Connect 'AirPods Pro' {5a1b2c3d-0001-0002-a1b2-0123456789ab} (was A2DP=UNPLUGGED, HFP=UNPLUGGED)"]);
    // A second click while pending is ignored.
    c.on_tray_event("select", 0, 0);
    assert_eq!(take_log(), ["click: ignored, Connect still pending"]);

    // The old link is still up: poll again after a delay; then it's down.
    mock.links.borrow_mut().pop_front().unwrap().ok(Some(true));
    assert_eq!(mock.delays.borrow().len(), 1);
    mock.now.set(1400.0);
    mock.delays.borrow_mut().pop_front().unwrap().ok(());
    mock.links.borrow_mut().pop_front().unwrap().ok(Some(false));
    assert_eq!(take_log(), ["waited 400 ms for the previous link to close"]);
    assert!(mock.timers.borrow().contains(&TIMER_RESEND));
    let (op, done) = mock.sends.borrow_mut().pop_front().unwrap();
    assert_eq!(op, OneShot::Connect);
    done.ok(ok(0));
    // The send is followed by a refresh; the device is connected 1.2345 s after the click.
    mock.now.set(2234.56);
    refresh(&mock, vec![airpods(true, "AirPods Pro")]);
    let log = take_log();
    assert_eq!(log[0], "state AirPods Pro: connected (A2DP=ACTIVE, HFP=ACTIVE)");
    assert_eq!(log[1], "Connect confirmed after 1234 ms");
    let calls = mock.took();
    assert!(calls.contains(&"emit {\"type\":\"confirmed\",\"op\":\"Connect\",\"ms\":1234.56}".to_string()), "{calls:?}");
    assert!(calls.contains(&"emit {\"type\":\"end\",\"op\":\"Connect\",\"succeeded\":true}".to_string()));
    assert!(!mock.timers.borrow().contains(&TIMER_RESEND));
    assert_eq!(c.status_text(), "Connected");
}

#[test]
fn a_send_nothing_accepts_ends_the_pending_state() {
    let (mock, c) = setup(Settings::default());
    c.start();
    refresh(&mock, vec![airpods(true, "AirPods Pro")]);
    take_log();
    c.on_command(quicktooth_core::host::tray_commands::DISCONNECT);
    let (op, done) = mock.sends.borrow_mut().pop_front().unwrap();
    assert_eq!(op, OneShot::Disconnect);
    done.ok(ok(0x80070490u32 as i32));
    let log = take_log();
    assert_eq!(log[log.len() - 2..], ["Disconnect: no control accepted the request", "tooltip: AirPods Pro — Connected"]);
    assert_eq!(c.status_text(), "Connected");
    // A failed send (the C# exception) is logged with its op as the context.
    c.on_command(quicktooth_core::host::tray_commands::DISCONNECT);
    mock.sends.borrow_mut().pop_front().unwrap().1.complete(Err("boom".into()));
    assert!(take_log().contains(&"ERROR Disconnect: boom".to_string()));
}

#[test]
fn exit_from_the_menu_reenters_without_a_borrow_panic() {
    let (mock, c) = setup(Settings::default());
    c.start();
    refresh(&mock, vec![airpods(false, "AirPods Pro")]);
    // A host that calls back into the controller while it's updating the icon.
    let c2 = c.clone();
    *mock.on_modify.borrow_mut() = Some(Box::new(move || {
        let _ = (c2.running(), c2.status_line(), c2.snapshot_json());
    }));
    mock.now.set(5000.0);
    c.on_timer(TIMER_POLL);
    refresh(&mock, vec![airpods(true, "AirPods Pro")]);
    mock.took();
    c.on_tray_event("contextmenu", 7, 8);
    let (items, done) = mock.menus.borrow_mut().pop_front().unwrap();
    assert_eq!(items[0].text.as_deref(), Some("AirPods Pro — Connected"));
    assert_eq!(items[2], MenuItem::command(CMD_DISCONNECT, "Disconnect"));
    let device = &items[3].submenu.as_ref().unwrap()[0];
    assert_eq!((device.id, device.text.as_deref(), device.checked), (CMD_DEVICE_BASE, Some("AirPods Pro  (connected)"), true));
    done.ok(CMD_EXIT);
    assert!(!c.running());
    let calls = mock.took();
    assert_eq!(calls[..2], ["emit {\"type\":\"menu\",\"open\":true}", "menu at 7,8"]);
    assert_eq!(&calls[calls.len() - 4..], ["remove_icon", "end_menu", "emit {\"type\":\"exit\"}", "exit"]);
    assert!(mock.timers.borrow().is_empty());
}

#[test]
fn the_tooltip_is_cut_in_utf16_units() {
    // 125 units, then a character outside the BMP (two units) that the cut at 127 splits.
    let name = format!("{}😀x", "a".repeat(125 - " — Not connected".encode_utf16().count()));
    let (mock, c) = setup(Settings::default());
    c.start();
    refresh(&mock, vec![airpods(false, &format!("AirPods {name}"))]);
    let tip = mock.tip.borrow().clone();
    assert!(tip.encode_utf16().count() <= 127);
    let full = c.status_line();
    assert!(full.starts_with(&tip) && full.len() > tip.len());
    // A cut that falls between characters keeps exactly 127 units.
    let (mock, c) = setup(Settings::default());
    c.start();
    refresh(&mock, vec![airpods(false, &format!("AirPods {}", "—".repeat(200)))]);
    assert_eq!(mock.tip.borrow().encode_utf16().count(), 127);
    assert!(c.status_line().starts_with(&*mock.tip.borrow()));
}

#[test]
fn settings_are_refused_while_pending() {
    let (mock, c) = setup(Settings::default());
    assert_eq!(c.apply_settings(&SettingsChange::default()), SetResult::Busy);
    c.start();
    refresh(&mock, vec![airpods(false, "AirPods Pro")]);
    c.on_command(quicktooth_core::host::tray_commands::TOGGLE);
    take_log();
    let change = SettingsChange { device: None, low_battery: Some(15) };
    assert_eq!(c.apply_settings(&change), SetResult::Busy);
    assert_eq!(take_log(), ["settings: ignored, Connect still pending"]);
}
