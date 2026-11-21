//! The exe's host re-enters the controller from inside host calls, and a `RefCell` borrow alive
//! at any of them aborts the process (panic = abort). This fake host does what Windows does:
//! - `exit()` destroys the window, whose `WM_DESTROY` calls `controller.exit()` again, nested;
//! - `track_popup_menu` runs the menu's modal loop synchronously: timers, commands, tray events
//!   and posted task wake-ups all reach the controller before it returns;
//! - `add_icon` / `modify_icon` / `remove_icon` deliver sent messages mid-call: `WM_COPYDATA`
//!   (`apply_settings`), `WM_SETTINGCHANGE`, display changes.
//!
//! Wake-ups are posted (run later from the "message loop"), as the exe schedules them. In tests
//! a double borrow panics instead of aborting, so any such borrow fails the test.
use quicktooth_core::Guid;
use quicktooth_core::battery::BatteryReading;
use quicktooth_core::blade_settings::SettingsChange;
use quicktooth_core::controller::*;
use quicktooth_core::devices::{BtDevice, BtEndpoint, KS_NOT_ATTEMPTED, OneShot, SendResult, Snapshot, device_state};
use quicktooth_core::exec::{self, Op, Schedule, Task};
use quicktooth_core::host::{DeviceBackend, Event, IconImage, MenuItem, TrayHost, tray_commands};
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

const CONTAINER: Guid = Guid::from_u128(0x5a1b2c3d_0001_0002_a1b2_0123456789ab);
const OTHER: Guid = Guid::from_u128(0x5a1b2c3d_0001_0002_a1b2_0123456789ac);

fn device(container: Guid, name: &str, connected: bool) -> BtDevice {
    let state = if connected { device_state::ACTIVE } else { device_state::UNPLUGGED };
    let ep = |bth: &str| BtEndpoint {
        endpoint_id: bth.into(),
        name: "Headphones".into(),
        interface_name: None,
        state,
        bth_device_id: bth.into(),
        container_id: container,
        mac: Some("307AD26790E8".into()),
        hidden_from_enumeration: false,
        ks_activate_hr: KS_NOT_ATTEMPTED,
    };
    BtDevice {
        container_id: container,
        name: name.into(),
        mac: Some("307AD26790E8".into()),
        endpoints: vec![ep("{0}\\bthenum#a2dp_vid&0001004c_pid&2014"), ep("{0}\\bthhfenum#hfp")],
    }
}

/// What the message loop has queued: task wake-ups and finished device work.
enum Posted {
    Wake(Task),
    Work(Box<dyn FnOnce()>),
}

#[derive(Default)]
struct Win {
    now: Cell<f64>,
    queue: RefCell<VecDeque<Posted>>,
    connected: Cell<bool>,
    light: Cell<bool>,
    size: Cell<i32>,
    /// Inside a re-entering host call (one level, as a nested modal loop would be refused).
    inside: Cell<bool>,
    /// What the modal menu loop ends with.
    menu_answer: Cell<i32>,
    menus: Cell<u32>,
    exits: Cell<u32>,
    reentries: Cell<u32>,
    controller: RefCell<Option<TrayController>>,
}

/// The exe's scheduler: a wake-up is a posted window message.
struct Posting(Rc<Win>);

impl Schedule for Posting {
    fn schedule(&self, task: Task) {
        self.0.queue.borrow_mut().push_back(Posted::Wake(task));
    }
}

impl Win {
    fn c(&self) -> TrayController {
        self.controller.borrow().clone().expect("controller")
    }

    /// The message loop: runs everything posted (bounded, the poll timers re-post).
    fn pump(&self) {
        for _ in 0..1000 {
            let next = self.queue.borrow_mut().pop_front();
            match next {
                Some(Posted::Wake(t)) => t.run(),
                Some(Posted::Work(f)) => f(),
                None => return,
            }
        }
    }

    fn post(&self, f: impl FnOnce() + 'static) {
        self.queue.borrow_mut().push_back(Posted::Work(Box::new(f)));
    }

    /// Sent messages arriving in the middle of a host call.
    fn sent_messages(&self) {
        if self.inside.replace(true) {
            return;
        }
        self.reentries.set(self.reentries.get() + 1);
        let c = self.c();
        let _ = c.apply_settings(&SettingsChange { device: None, low_battery: Some(25), icon: None }); // WM_COPYDATA
        c.on_setting_change(Some("ImmersiveColorSet")); // WM_SETTINGCHANGE (the theme flipped)
        c.on_setting_change(None);
        self.size.set(if self.size.get() == 16 { 20 } else { 16 });
        c.on_display_change(); // WM_DPICHANGED
        let _ = (c.running(), c.status_line(), c.status_text(), c.snapshot_json());
        self.inside.set(false);
    }
}

impl TrayHost for Win {
    fn exe_path(&self) -> String {
        "C:\\QuickTooth.exe".into()
    }
    fn now_ms(&self) -> f64 {
        // Time moves on every reading, so animations run and finish.
        self.now.set(self.now.get() + 37.0);
        self.now.get()
    }
    fn now_epoch_ms(&self) -> f64 {
        self.now.get()
    }
    fn delay(&self, _ms: i32) -> Op<()> {
        let (op, done) = exec::op();
        self.post(move || done.ok(()));
        op
    }
    fn set_timer(&self, _id: i32, _ms: i32) {}
    fn kill_timer(&self, _id: i32) {}
    fn kill_all_timers(&self) {}
    fn light_taskbar(&self) -> bool {
        self.light.set(!self.light.get());
        self.light.get()
    }
    fn icon_size(&self) -> i32 {
        self.size.get()
    }
    fn apply_menu_theme(&self, _light: bool) {}
    fn add_icon(&self, _icon: &IconImage, _tip: &str) -> bool {
        self.sent_messages();
        true
    }
    fn modify_icon(&self, _icon: &IconImage, _tip: &str) -> bool {
        self.sent_messages();
        true
    }
    fn remove_icon(&self) {
        self.sent_messages();
    }
    fn release_icon(&self, _icon: &IconImage) {}
    fn cursor_pos(&self) -> Option<(i32, i32)> {
        Some((10, 20))
    }
    fn icon_anchor(&self) -> Option<(i32, i32)> {
        Some((5, 6))
    }
    /// The modal loop: everything reaches the controller before the menu returns.
    fn track_popup_menu(&self, _items: &[MenuItem], _x: i32, _y: i32, _light: bool) -> Op<i32> {
        self.menus.set(self.menus.get() + 1);
        if !self.inside.replace(true) {
            self.reentries.set(self.reentries.get() + 1);
            let c = self.c();
            for id in ALL_TIMERS {
                c.on_timer(id);
            }
            c.on_tray_event("select", 0, 0);
            c.on_command(tray_commands::TOGGLE);
            c.on_tray_event("contextmenu", 1, 2); // a nested menu (returns dismissed at once)
            c.on_endpoint_changed();
            self.sent_messages_inside_menu(&c);
            self.pump();
            self.inside.set(false);
            return Op::ok(self.menu_answer.get());
        }
        Op::ok(0)
    }
    fn update_menu_header(&self, _header: &quicktooth_core::popup::Header) {}
    fn end_menu(&self) {}
    fn promote_self(&self) -> String {
        "EntryNotFound".into()
    }
    fn autostart_enabled(&self) -> bool {
        false
    }
    fn set_autostart(&self, _on: bool) {}
    fn exit(&self) {
        // DestroyWindow -> WM_DESTROY -> exit() again, nested.
        self.exits.set(self.exits.get() + 1);
        self.c().exit();
    }
    fn emit(&self, _event: &Event<'_>) {}
}

impl Win {
    fn sent_messages_inside_menu(&self, c: &TrayController) {
        let _ = c.apply_settings(&SettingsChange { device: Some(OTHER.to_b()), low_battery: Some(10), icon: None });
        c.on_setting_change(Some("ImmersiveColorSet"));
        c.on_taskbar_created();
        c.rebuild_icons_now();
    }
}

struct Devices(Rc<Win>);

impl DeviceBackend for Devices {
    fn register_notifications(&self) -> Op<()> {
        let (op, done) = exec::op();
        self.0.post(move || done.ok(()));
        op
    }
    fn refresh(&self, _mac: Option<&str>) -> Op<(Snapshot, Option<i32>)> {
        let (op, done) = exec::op();
        let w = self.0.clone();
        self.0.post(move || {
            let devices = vec![device(CONTAINER, "AirPods Pro", w.connected.get()), device(OTHER, "Studio Headset", true)];
            done.ok((Snapshot { devices, ..Snapshot::default() }, Some(30)));
        });
        op
    }
    fn send_one_shot(&self, _container: Guid, op: OneShot) -> Op<Vec<SendResult>> {
        let (o, done) = exec::op();
        let w = self.0.clone();
        self.0.post(move || {
            w.connected.set(op == OneShot::Connect);
            done.ok(vec![SendResult { endpoint_name: "Headphones".into(), bth_device_id: "x".into(), hr: 0, bytes_returned: 0 }]);
        });
        o
    }
    fn bluetooth_connected(&self, _mac: &str) -> Op<Option<bool>> {
        let (op, done) = exec::op();
        self.0.post(move || done.ok(Some(false)));
        op
    }
    fn scan_battery(&self, _model: u16, _window_ms: i32) -> Op<Option<BatteryReading>> {
        let (op, done) = exec::op();
        let w = self.0.clone();
        self.0.post(move || {
            done.ok(Some(BatteryReading {
                left: Some(10),
                right: Some(20),
                case: None,
                buds_charging: false,
                left_charging: false,
                right_charging: false,
                case_charging: false,
                rssi: -60,
                at_epoch_ms: w.now.get(),
                source: "ble".into(),
            }))
        });
        op
    }
    fn start_advert_watch(&self, _model: u16) {
        // An advert arrives later, as the exe posts it to its window.
        let w = self.0.clone();
        self.0.post(move || {
            let c = w.controller.borrow().clone();
            if let Some(c) = c {
                c.on_advert(&[0x07, 0x19, 0x01, 0x14, 0x20, 0x75, 0x88, 0xB5, 0x32, 0x00, 0x04], -50);
            }
        });
    }
    fn stop_advert_watch(&self) {}
}

#[test]
fn host_calls_that_reenter_never_meet_a_live_borrow() {
    quicktooth_core::log::set_sink(sink);
    quicktooth_core::settings::set_save_handler(|_| true);
    let win = Rc::new(Win::default());
    win.size.set(16);
    let schedule: Rc<dyn Schedule> = Rc::new(Posting(win.clone()));
    let c = TrayController::new(win.clone(), Rc::new(Devices(win.clone())), Settings::default(), schedule);
    *win.controller.borrow_mut() = Some(c.clone());

    c.start();
    win.pump();
    assert!(win.reentries.get() > 0);

    // Clicks, timers and battery scans while sent messages keep arriving mid-call.
    for round in 0..6 {
        c.on_tray_event("select", 0, 0);
        win.pump();
        for id in ALL_TIMERS {
            c.on_timer(id);
            win.pump();
        }
        c.on_endpoint_changed();
        c.on_command(if round % 2 == 0 { tray_commands::CONNECT } else { tray_commands::DISCONNECT });
        win.pump();
    }

    // A menu whose modal loop runs everything, ending with a device pick, then with Exit.
    win.menu_answer.set(CMD_DEVICE_BASE + 1);
    c.on_tray_event("contextmenu", 3, 4);
    win.pump();
    win.menu_answer.set(CMD_EXIT);
    c.on_command(tray_commands::SHOW_MENU);
    win.pump();

    assert!(!c.running());
    // The nested exit from WM_DESTROY found running already cleared and did nothing.
    assert_eq!(win.exits.get(), 1);
    let log = LOG.with(|l| l.borrow().clone());
    assert_eq!(log.iter().filter(|l| *l == "tray exit").count(), 1);
    assert!(log.iter().any(|l| l.starts_with("Connect confirmed after")), "{log:#?}");
    assert!(log.iter().any(|l| l.starts_with("target set: Studio Headset")), "{log:#?}");
    assert!(win.menus.get() >= 3);

    // After exit, everything is ignored (late wake-ups and messages land on a stopped app).
    c.on_tray_event("select", 0, 0);
    c.on_timer(TIMER_POLL);
    win.pump();
    assert!(!c.running());
}
