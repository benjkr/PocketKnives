//! `QuickTooth.exe --selftest-icons [cycles]`: drives the real controller through connect and
//! disconnect animations with a window-less host whose icons are real HICONs, made and destroyed
//! exactly as the tray does ([`icons::handle_for`] / [`icons::release`]), and reports the
//! process's GDI and USER object counts, which must stay flat. Read-only: a fake device backend,
//! no tray icon, no device operations, no log lines and no settings saved.
use crate::icons;
use alloc::collections::BTreeSet;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};
use core::fmt::Write;
use pk::out::Writer;
use quicktooth_core::Guid;
use quicktooth_core::battery::BatteryReading;
use quicktooth_core::controller::{self as ctl, TrayController};
use quicktooth_core::devices::{BtDevice, BtEndpoint, KS_NOT_ATTEMPTED, OneShot, SendResult, Snapshot, device_state};
use quicktooth_core::exec::{Inline, Op, Schedule};
use quicktooth_core::host::{DeviceBackend, IconImage, MenuItem, TrayHost, tray_commands};
use quicktooth_core::settings::Settings;
use quicktooth_core::tunables::{self, T};
use windows_sys::Win32::System::Threading::{GR_GDIOBJECTS, GR_USEROBJECTS, GetCurrentProcess, GetGuiResources};

const CONTAINER: Guid = Guid::from_u128(0xa1000001_0000_4000_8000_000000000001);
const MAC: &str = "A0B1C2D3E4F5";

pub fn icons(out: &mut Writer, cycles: i32) -> u32 {
    quicktooth_core::log::set_sink(|_| {});
    quicktooth_core::settings::set_save_handler(|_| true);

    let host = Rc::new(FakeHost::default());
    let connected = Rc::new(Cell::new(false));
    let devices = Rc::new(FakeDevices { connected: connected.clone() });
    let settings = Settings {
        target_container_id: Some(CONTAINER.to_b()),
        target_mac: Some(MAC.into()),
        target_name: Some("Test AirPods".into()),
        tray_promoted_once: true,
        low_battery_percent: None,
        icon: None,
    };
    let schedule: Rc<dyn Schedule> = Rc::new(Inline::default());
    let app = TrayController::new(host.clone(), devices, settings, schedule);
    app.start();

    let count = || unsafe { (GetGuiResources(GetCurrentProcess(), GR_GDIOBJECTS), GetGuiResources(GetCurrentProcess(), GR_USEROBJECTS)) };
    let frame_ms = tunables::get(T::AnimFrameMs);
    let tick = |id: i32| {
        host.now.set(host.now.get() + frame_ms);
        app.on_timer(id);
    };
    let cycle = || {
        app.on_command(tray_commands::CONNECT); // Connecting: buds part, tracer runs
        (0..25).for_each(|_| tick(ctl::TIMER_ANIM));
        connected.set(true);
        app.on_timer(ctl::TIMER_FASTPOLL); // confirmed → Landing
        (0..12).for_each(|_| tick(ctl::TIMER_ANIM));
        app.on_command(tray_commands::DISCONNECT); // Disconnecting
        (0..6).for_each(|_| tick(ctl::TIMER_ANIM));
        connected.set(false);
        app.on_timer(ctl::TIMER_FASTPOLL);
        (0..12).for_each(|_| tick(ctl::TIMER_ANIM));
    };

    cycle(); // warm-up: static icons, first frames
    let before = count();
    let frames_before = host.frames.get();
    for _ in 0..cycles {
        cycle();
    }
    let after = count();
    let frames = host.frames.get() - frames_before;
    app.exit();
    let end = count();

    let _ = write!(out, "{cycles} connect/disconnect cycles, {frames} animation frames shown as HICONs\r\n");
    let _ = write!(out, "GDI objects:  {} \u{2192} {} (after exit {})\r\n", before.0, after.0, end.0);
    let _ = write!(out, "USER objects: {} \u{2192} {} (after exit {})\r\n", before.1, after.1, end.1);
    let _ = write!(out, "icons still owned by the controller: {} (the two static icons)\r\n", host.live.borrow().len());
    let ok = after.0 <= before.0 + 2 && after.1 <= before.1 + 2 && frames > 0;
    let _ = write!(out, "{}\r\n", if ok { "PASS: no icon handles leak" } else { "FAIL: icon handles leak" });
    if ok { 0 } else { 1 }
}

#[derive(Default)]
struct FakeHost {
    now: Cell<f64>,
    frames: Cell<u32>,
    /// The icons shown and not yet released, by address (compared by identity, as in C#).
    live: RefCell<BTreeSet<usize>>,
}

impl FakeHost {
    fn show(&self, icon: &IconImage) -> bool {
        icons::handle_for(icon);
        if self.live.borrow_mut().insert(icon as *const IconImage as usize) && icon.desc.is_anim() {
            self.frames.set(self.frames.get() + 1);
        }
        true
    }
}

impl TrayHost for FakeHost {
    fn exe_path(&self) -> String {
        "selftest".into()
    }
    fn now_ms(&self) -> f64 {
        self.now.get()
    }
    fn now_epoch_ms(&self) -> f64 {
        self.now.get()
    }
    fn delay(&self, ms: i32) -> Op<()> {
        self.now.set(self.now.get() + ms as f64);
        Op::ok(())
    }
    fn set_timer(&self, _: i32, _: i32) {}
    fn kill_timer(&self, _: i32) {}
    fn kill_all_timers(&self) {}
    fn light_taskbar(&self) -> bool {
        false
    }
    fn icon_size(&self) -> i32 {
        16
    }
    fn apply_menu_theme(&self, _: bool) {}
    fn add_icon(&self, icon: &IconImage, _: &str) -> bool {
        self.show(icon)
    }
    fn modify_icon(&self, icon: &IconImage, _: &str) -> bool {
        self.show(icon)
    }
    fn remove_icon(&self) {}
    fn release_icon(&self, icon: &IconImage) {
        icons::release(icon);
        self.live.borrow_mut().remove(&(icon as *const IconImage as usize));
    }
    fn cursor_pos(&self) -> Option<(i32, i32)> {
        None
    }
    fn icon_anchor(&self) -> Option<(i32, i32)> {
        None
    }
    fn track_popup_menu(&self, _: &[MenuItem], _: i32, _: i32, _: bool) -> Op<i32> {
        Op::ok(0)
    }
    fn end_menu(&self) {}
    fn promote_self(&self) -> String {
        "AlreadyPromoted".into()
    }
    fn autostart_enabled(&self) -> bool {
        false
    }
    fn set_autostart(&self, _: bool) {}
    fn exit(&self) {}
    fn emit(&self, _: &quicktooth_core::host::Event<'_>) {}
}

struct FakeDevices {
    connected: Rc<Cell<bool>>,
}

impl DeviceBackend for FakeDevices {
    fn register_notifications(&self) -> Op<()> {
        Op::ok(())
    }

    fn refresh(&self, _: Option<&str>) -> Op<(Snapshot, Option<i32>)> {
        let state = if self.connected.get() { device_state::ACTIVE } else { device_state::UNPLUGGED };
        let ep = |id: &str, name: &str, bth: &str, hidden: bool| BtEndpoint {
            endpoint_id: id.into(),
            name: name.into(),
            interface_name: None,
            state,
            bth_device_id: bth.into(),
            container_id: CONTAINER,
            mac: Some(MAC.into()),
            hidden_from_enumeration: hidden,
            ks_activate_hr: KS_NOT_ATTEMPTED,
        };
        let endpoints: Vec<BtEndpoint> = vec![
            ep("ep1", "Headphones (Test AirPods)", "{2}.\\\\?\\bthenum#x", false),
            ep("ep2", "Headset (Test AirPods)", "{2}.\\\\?\\bthhfenum#x", true),
        ];
        let device = BtDevice { container_id: CONTAINER, name: "Test AirPods".into(), mac: Some(MAC.into()), endpoints };
        Op::ok((Snapshot { devices: vec![device], others: Vec::new(), error: None }, None))
    }

    fn send_one_shot(&self, _: Guid, _: OneShot) -> Op<Vec<SendResult>> {
        Op::ok(vec![SendResult { endpoint_name: "Headphones (Test AirPods)".into(), bth_device_id: "x".into(), hr: 0, bytes_returned: 0 }])
    }

    fn bluetooth_connected(&self, _: &str) -> Op<Option<bool>> {
        Op::ok(Some(false))
    }

    fn scan_battery(&self, _: u16, _: i32) -> Op<Option<BatteryReading>> {
        Op::ok(None)
    }
}
