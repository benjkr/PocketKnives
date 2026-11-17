//! The tray icon's behaviour, shared by the exe and the simulator: the connect/disconnect state
//! machine (link-teardown wait, resend, timeout), the pending animation, tooltip text, battery
//! readings and the popup menu's contents (docs/specs/tray.md). The platform comes in through
//! [`TrayHost`] and [`DeviceBackend`]. Single-threaded: every call and task wake-up runs on the
//! host's UI thread.
//!
//! Re-entrancy: a host call can call back into the controller (`exit()` from a menu, a modal menu
//! loop on Windows), and the log sink and settings saver are host code too. So no `RefCell`
//! borrow of the state is ever held across a host call, a log line, a save or an `.await`: each
//! step reads or writes the state in a short borrow of its own.
//!
//! Each C# `async` method is a future started with [`exec::spawn`] (runs synchronously up to its
//! first pending wait, as `_ = M()` did); an exception the C# caught is an `Err` from an [`Op`].
use crate::Guid;
use crate::battery::{self, BatteryReading};
use crate::blade_settings::{self, SetResult, SettingsChange};
use crate::devices::{BtDevice, OneShot};
use crate::exec::{self, Schedule};
use crate::host::{DeviceBackend, IconImage, MenuItem, TrayHost, tray_commands};
use crate::icon::{self, AnimPhase, IconState};
use crate::json;
use crate::log;
use crate::num::{fmt_r, round_away, trunc_i64, write_r};
use crate::settings::Settings;
use crate::tunables::{self, T};
use alloc::format;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::{Ref, RefCell, RefMut};

pub const TIMER_ANIM: i32 = 1;
pub const TIMER_TIMEOUT: i32 = 2;
pub const TIMER_POLL: i32 = 3;
pub const TIMER_DEBOUNCE: i32 = 4;
pub const TIMER_FASTPOLL: i32 = 5;
pub const TIMER_ADD_RETRY: i32 = 6;
pub const TIMER_PROMOTE: i32 = 7;
pub const TIMER_RESEND: i32 = 8;
pub const TIMER_BATTERY: i32 = 9;
pub const TIMER_LOWBATTERY: i32 = 10;
pub const ALL_TIMERS: [i32; 10] = [
    TIMER_ANIM,
    TIMER_TIMEOUT,
    TIMER_POLL,
    TIMER_DEBOUNCE,
    TIMER_FASTPOLL,
    TIMER_ADD_RETRY,
    TIMER_PROMOTE,
    TIMER_RESEND,
    TIMER_BATTERY,
    TIMER_LOWBATTERY,
];

pub const CMD_CONNECT: i32 = 1;
pub const CMD_DISCONNECT: i32 = 2;
pub const CMD_AUTOSTART: i32 = 3;
pub const CMD_EXIT: i32 = 4;
pub const CMD_STATUS: i32 = 9;
pub const CMD_DEVICE_BASE: i32 = 100;

const MS_PER_MINUTE: f64 = 60_000.0;

/// A handle to one tray app (cheap to clone; all clones are the same app).
#[derive(Clone)]
pub struct TrayController {
    inner: Rc<Inner>,
}

struct Inner {
    host: Rc<dyn TrayHost>,
    devices: Rc<dyn DeviceBackend>,
    schedule: Rc<dyn Schedule>,
    settings: RefCell<Settings>,
    st: RefCell<State>,
}

#[derive(Default)]
struct State {
    running: bool,
    device_list: Vec<BtDevice>,
    target: Option<BtDevice>,
    have_snapshot: bool,
    last_state_log: Option<String>,

    pending: Option<OneShot>,
    pending_seq: i32,
    pending_container: Guid,
    pending_start: f64,
    /// The pending animation phase (see [`icon`]), when it started (host ms), and the tracer
    /// time carried into it.
    anim: Option<AnimPhase>,
    anim_start: f64,
    tracer_offset: f64,
    /// The last live-rendered frame (released when replaced).
    last_anim_icon: Option<Rc<IconImage>>,
    anim_frames: i32,

    refreshing: bool,
    refresh_again: bool,

    battery: Option<BatteryReading>,
    battery_container: Guid,
    battery_scanning: bool,
    was_connected: bool,

    icon_added: bool,
    add_attempts: i32,
    promote_attempts: i32,
    icon_size: i32,
    light_taskbar: bool,
    icon_on: Option<Rc<IconImage>>,
    icon_off: Option<Rc<IconImage>>,
    icon_low: Option<Rc<IconImage>>,
    shown_icon: Option<Rc<IconImage>>,
    /// The static icon last chosen was the low-battery one.
    low_shown: bool,
    shown_tip: String,

    menu_devices: Vec<BtDevice>,
}

fn int(t: T) -> i32 {
    tunables::int(t)
}

fn light_name(light: bool) -> &'static str {
    if light { "light" } else { "dark" }
}

/// A C# `$"{x}"` of a nullable enum: empty when null.
fn opt_op(op: Option<OneShot>) -> String {
    op.map(|o| format!("{o}")).unwrap_or_default()
}

fn same_icon(a: &Option<Rc<IconImage>>, b: &Option<Rc<IconImage>>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => Rc::ptr_eq(a, b),
        (None, None) => true,
        _ => false,
    }
}

/// `s[..max]` in UTF-16 units, as the C# cut the tooltip. A surrogate pair the cut would split is
/// dropped whole (a Rust string can't hold half of one).
fn cut_utf16(s: &mut String, max: usize) {
    let mut units = 0;
    for (i, c) in s.char_indices() {
        units += c.len_utf16();
        if units > max {
            s.truncate(i);
            return;
        }
    }
}

impl TrayController {
    /// `schedule` runs the controller's woken tasks: the exe posts them to its window, the
    /// simulator runs them inline ([`crate::exec::Inline`]).
    pub fn new(host: Rc<dyn TrayHost>, devices: Rc<dyn DeviceBackend>, settings: Settings, schedule: Rc<dyn Schedule>) -> Self {
        TrayController {
            inner: Rc::new(Inner { host, devices, schedule, settings: RefCell::new(settings), st: RefCell::new(State::default()) }),
        }
    }

    fn st(&self) -> Ref<'_, State> {
        self.inner.st.borrow()
    }

    fn st_mut(&self) -> RefMut<'_, State> {
        self.inner.st.borrow_mut()
    }

    fn host(&self) -> &dyn TrayHost {
        &*self.inner.host
    }

    /// Changes the settings on a copy, so no borrow is held while they log or save (host code).
    fn with_settings<R>(&self, f: impl FnOnce(&mut Settings) -> R) -> R {
        let mut s = self.inner.settings.borrow().clone();
        let r = f(&mut s);
        *self.inner.settings.borrow_mut() = s;
        r
    }

    fn settings(&self) -> Ref<'_, Settings> {
        self.inner.settings.borrow()
    }

    fn spawn(&self, future: impl core::future::Future<Output = ()> + 'static) {
        exec::spawn(&self.inner.schedule, future);
    }

    fn emit(&self, json: &str) {
        self.host().emit(json);
    }

    fn pending_ms(&self) -> f64 {
        let start = self.st().pending_start;
        self.host().now_ms() - start
    }

    fn anim_seconds(&self) -> f64 {
        let start = self.st().anim_start;
        (self.host().now_ms() - start) / 1000.0
    }

    pub fn running(&self) -> bool {
        self.st().running
    }

    fn target_connected(&self) -> bool {
        self.st().target.as_ref().is_some_and(BtDevice::connected)
    }

    // ------------------------------------------------------------------ lifetime

    pub fn start(&self) {
        self.st_mut().running = true;
        let exe = self.host().exe_path();
        log::info(&format!("tray start: {exe}"));
        let light = self.host().light_taskbar();
        self.st_mut().light_taskbar = light;
        self.host().apply_menu_theme(light);
        self.rebuild_icons();
        self.add_icon();

        let me = self.clone();
        self.spawn(async move {
            let op = me.inner.devices.register_notifications();
            if let Err(e) = op.await {
                log::error("register notifications", &e);
            }
        });
        self.request_refresh();
        self.host().set_timer(TIMER_POLL, int(T::IdlePollMs));
        self.host().set_timer(TIMER_BATTERY, int(T::BatteryCheckMs));
        let promoted = self.settings().tray_promoted_once;
        if !promoted {
            self.host().set_timer(TIMER_PROMOTE, int(T::PromoteRetryMs));
        }
    }

    /// Exit from the menu or `QuickTooth.exe --exit`; also the window being destroyed.
    pub fn exit(&self) {
        if !self.running() {
            return;
        }
        self.remove_icon();
        self.host().end_menu();
        self.host().kill_all_timers();
        self.st_mut().running = false;
        log::info("tray exit");
        self.emit("{\"type\":\"exit\"}");
        self.host().exit();
    }

    // ------------------------------------------------------------------ host events

    /// A notification-area event: `"select"` (left click), `"keyselect"`, or `"contextmenu"` at x, y.
    pub fn on_tray_event(&self, ev: &str, x: i32, y: i32) {
        if !self.running() {
            return;
        }
        match ev {
            "select" => self.toggle("click"),
            "keyselect" => self.toggle("key"),
            "contextmenu" => self.show_menu(x, y),
            _ => {}
        }
    }

    /// A command from another QuickTooth.exe ([`crate::host::tray_commands`]).
    pub fn on_command(&self, cmd: i32) {
        if !self.running() {
            return;
        }
        match cmd {
            tray_commands::TOGGLE => self.toggle("cli"),
            tray_commands::CONNECT => self.start_action(OneShot::Connect, "cli"),
            tray_commands::DISCONNECT => self.start_action(OneShot::Disconnect, "cli"),
            tray_commands::EXIT => self.exit(),
            tray_commands::SHOW_MENU => {
                if let Some((x, y)) = self.host().icon_anchor() {
                    self.show_menu(x, y);
                }
            }
            tray_commands::CLOSE_MENU => self.host().end_menu(),
            _ => {}
        }
    }

    pub fn on_timer(&self, id: i32) {
        if !self.running() {
            return;
        }
        match id {
            TIMER_ANIM => self.advance_animation(),
            TIMER_TIMEOUT => {
                let pending = self.st().pending;
                let ms = trunc_i64(self.pending_ms());
                log::info(&format!("{} timed out after {ms} ms; showing the real state", opt_op(pending)));
                self.end_pending(false);
                self.request_refresh();
            }
            TIMER_FASTPOLL | TIMER_POLL => {
                self.request_refresh();
                if id == TIMER_POLL {
                    self.check_icon_metrics();
                }
            }
            TIMER_DEBOUNCE => {
                self.host().kill_timer(TIMER_DEBOUNCE);
                self.request_refresh();
            }
            TIMER_ADD_RETRY => {
                self.host().kill_timer(TIMER_ADD_RETRY);
                self.add_icon();
            }
            TIMER_PROMOTE => {
                self.host().kill_timer(TIMER_PROMOTE);
                self.try_promote_once();
            }
            TIMER_RESEND => {
                self.host().kill_timer(TIMER_RESEND);
                self.resend_connect();
            }
            TIMER_LOWBATTERY => {
                // The next ripple, while the dot shows and nothing else animates.
                let replay = {
                    let st = self.st();
                    st.anim.is_none() && st.pending.is_none() && st.low_shown
                };
                if replay {
                    self.start_animation(AnimPhase::LowBattery, 0.0);
                }
            }
            TIMER_BATTERY => {
                self.host().set_timer(TIMER_BATTERY, int(T::BatteryCheckMs)); // back to the periodic check
                self.maybe_scan_battery(int(T::BatteryMaxAgeMin) as f64 * MS_PER_MINUTE);
            }
            _ => {}
        }
    }

    /// An audio endpoint changed (the exe calls it on its UI thread after the notification).
    pub fn on_endpoint_changed(&self) {
        if self.running() {
            self.host().set_timer(TIMER_DEBOUNCE, int(T::DebounceMs));
        }
    }

    /// `WM_SETTINGCHANGE`: `"ImmersiveColorSet"` means the taskbar theme may have flipped.
    pub fn on_setting_change(&self, area: Option<&str>) {
        if !self.running() {
            return;
        }
        if area == Some("ImmersiveColorSet") {
            let light = self.host().light_taskbar();
            if light != self.st().light_taskbar {
                log::info(&format!("taskbar theme -> {}", light_name(light)));
                self.st_mut().light_taskbar = light;
                self.host().apply_menu_theme(light);
                self.rebuild_icons();
            }
        } else {
            self.check_icon_metrics();
        }
    }

    /// `WM_DISPLAYCHANGE` / `WM_DPICHANGED`.
    pub fn on_display_change(&self) {
        if self.running() {
            self.check_icon_metrics();
        }
    }

    /// `"TaskbarCreated"`: Explorer restarted, so the icon is added again.
    pub fn on_taskbar_created(&self) {
        if !self.running() {
            return;
        }
        log::info("TaskbarCreated: re-adding icon");
        {
            let mut st = self.st_mut();
            st.icon_added = false;
            st.add_attempts = 0;
        }
        self.rebuild_icons();
        self.add_icon();
    }

    // ------------------------------------------------------------------ state

    fn request_refresh(&self) {
        {
            let mut st = self.st_mut();
            if st.refreshing {
                st.refresh_again = true;
                return;
            }
        }
        let me = self.clone();
        self.spawn(async move { me.refresh_async().await });
    }

    async fn refresh_async(&self) {
        self.st_mut().refreshing = true;
        loop {
            let mac = {
                let mut st = self.st_mut();
                st.refresh_again = false;
                st.target.as_ref().and_then(|t| t.mac.clone())
            };
            let op = self.inner.devices.refresh(mac.as_deref());
            match op.await {
                Err(e) => {
                    log::error("refresh", &e);
                    break;
                }
                Ok((snap, windows_battery)) => {
                    if !self.running() {
                        break;
                    }
                    self.apply(snap.devices, windows_battery);
                }
            }
            if !self.st().refresh_again {
                break;
            }
        }
        self.st_mut().refreshing = false;
    }

    fn apply(&self, devices: Vec<BtDevice>, windows_battery: Option<i32>) {
        {
            let mut st = self.st_mut();
            st.device_list = devices.clone();
            st.have_snapshot = true;
        }
        let target = self.with_settings(|s| s.resolve_or_default(&devices, true));
        self.st_mut().target = target.clone();

        let connected = target.as_ref().is_some_and(BtDevice::connected);
        if !connected {
            self.st_mut().battery = None;
        } else {
            if let Some(level) = windows_battery {
                // Headsets that report battery to Windows (not AirPods).
                let changed = self.st().battery.as_ref().is_none_or(|b| b.source != "windows" || b.left != Some(level));
                if changed {
                    log::info(&format!("battery: {level}% (Windows)"));
                }
                let now = self.host().now_epoch_ms();
                let mut st = self.st_mut();
                st.battery = Some(BatteryReading {
                    left: Some(level),
                    right: None,
                    case: None,
                    buds_charging: false,
                    case_charging: false,
                    rssi: 0,
                    at_epoch_ms: now,
                    source: "windows".into(),
                });
                st.battery_container = target.as_ref().map(|t| t.container_id).unwrap_or_default();
            }
            let was_connected = self.st().was_connected;
            if !was_connected {
                self.host().set_timer(TIMER_BATTERY, int(T::FirstBatteryMs)); // first reading soon after connecting
            }
        }
        self.st_mut().was_connected = connected;

        let state_log = match &target {
            Some(t) => format!("{}: {} ({})", t.name, if t.connected() { "connected" } else { "not connected" }, t.state_signature()),
            None => {
                let s = self.settings();
                if s.has_target() { format!("{}: not available", s.target_name.as_deref().unwrap_or("")) } else { "no target".into() }
            }
        };
        let changed = self.st().last_state_log.as_deref() != Some(state_log.as_str());
        if changed {
            log::info(&format!("state {state_log}"));
            self.st_mut().last_state_log = Some(state_log);
        }

        let pending = self.st().pending;
        if let (Some(op), Some(t)) = (pending, &target)
            && t.connected() == (op == OneShot::Connect)
        {
            let ms = self.pending_ms();
            log::info(&format!("{op} confirmed after {} ms", trunc_i64(ms)));
            self.emit(&format!("{{\"type\":\"confirmed\",\"op\":\"{op}\",\"ms\":{}}}", fmt_r(ms)));
            self.end_pending(true);
        }
        self.update_icon(false);
    }

    fn toggle(&self, source: &str) {
        let (pending, target) = {
            let st = self.st();
            (st.pending, st.target.as_ref().map(BtDevice::connected))
        };
        if let Some(p) = pending {
            log::info(&format!("{source}: ignored, {p} still pending"));
            return;
        }
        let Some(connected) = target else {
            log::info(&format!("{source}: no target device"));
            if source != "cli"
                && let Some((x, y)) = self.host().cursor_pos()
            {
                self.show_menu(x, y);
            }
            return;
        };
        self.start_action(if connected { OneShot::Disconnect } else { OneShot::Connect }, source);
    }

    fn start_action(&self, op: OneShot, source: &str) {
        let me = self.clone();
        let source = String::from(source);
        self.spawn(async move {
            if let Err(e) = me.start_action_async(op, &source).await {
                log::error(&format!("{op}"), &e);
                me.end_pending(false);
                me.request_refresh();
            }
        });
    }

    async fn start_action_async(&self, op: OneShot, source: &str) -> Result<(), String> {
        let (pending, target) = {
            let st = self.st();
            (st.pending, st.target.clone())
        };
        if let Some(p) = pending {
            log::info(&format!("{source}: {op} ignored, {p} still pending"));
            return Ok(());
        }
        let Some(target) = target else {
            log::info(&format!("{source}: {op} ignored, no target"));
            return Ok(());
        };
        if target.connected() == (op == OneShot::Connect) {
            log::info(&format!("{source}: already {}", if target.connected() { "connected" } else { "disconnected" }));
            return Ok(());
        }

        let now = self.host().now_ms();
        let seq = {
            let mut st = self.st_mut();
            st.pending = Some(op);
            st.pending_seq += 1;
            st.pending_container = target.container_id;
            st.pending_start = now;
            st.pending_seq
        };
        self.start_animation(if op == OneShot::Connect { AnimPhase::Connecting } else { AnimPhase::Disconnecting }, 0.0);
        log::info(&format!("{source}: {op} '{}' {} (was {})", target.name, target.container_id, target.state_signature()));
        self.emit(&format!("{{\"type\":\"start\",\"op\":\"{op}\",\"source\":\"{source}\"}}"));
        self.update_icon(false);

        if let (OneShot::Connect, Some(mac)) = (op, target.mac.as_deref()) {
            // After a disconnect the Bluetooth link lingers ~3 s; a reconnect sent into that
            // teardown can stall for 15 s+. Wait (still animating) until the link has closed.
            let hold_start = self.host().now_ms();
            while self.host().now_ms() - hold_start < int(T::LinkTeardownWaitMs) as f64 {
                let connected = self.inner.devices.bluetooth_connected(mac);
                if connected.await? != Some(true) {
                    break;
                }
                let delay = self.host().delay(int(T::TeardownPollMs));
                delay.await?;
            }
            let still = {
                let st = self.st();
                seq == st.pending_seq && st.pending == Some(op)
            };
            if !still {
                return Ok(());
            }
            let held = self.host().now_ms() - hold_start;
            if held > int(T::TeardownLogAboveMs) as f64 {
                log::info(&format!("waited {} ms for the previous link to close", trunc_i64(held)));
            }
        }

        self.host().set_timer(TIMER_TIMEOUT, int(T::PendingTimeoutMs));
        self.host().set_timer(TIMER_FASTPOLL, int(T::PendingPollMs)); // notifications may not fire for hidden endpoints
        if op == OneShot::Connect {
            self.host().set_timer(TIMER_RESEND, int(T::ResendAfterMs));
        }

        // A2DP *and* Hands-Free controls: kicking only A2DP leaves Windows waiting for the
        // headset to bring up Hands-Free, which measured 15–40 s+ instead of ~1–3 s.
        let send = self.inner.devices.send_one_shot(target.container_id, op);
        let results = send.await?;
        // Judge success by the resulting state; only give up early if nothing accepted the request.
        if results.iter().all(|r| r.hr < 0) {
            log::info(&format!("{op}: no control accepted the request"));
            self.end_pending(false);
        }
        self.request_refresh();
        Ok(())
    }

    /// `succeeded`: the state landed. A connect plays its close-and-fill ending, a disconnect plays
    /// its motion to the end. Otherwise a connect glides back (cancel). Clicks are accepted again
    /// immediately.
    fn end_pending(&self, succeeded: bool) {
        let (op, anim, tracer_offset) = {
            let mut st = self.st_mut();
            let op = st.pending.take();
            st.pending_seq += 1;
            (op, st.anim, st.tracer_offset)
        };
        let t = self.anim_seconds();
        if op == Some(OneShot::Connect) && anim == Some(AnimPhase::Connecting) {
            self.start_animation(if succeeded { AnimPhase::Landing } else { AnimPhase::Cancelling }, tracer_offset + t);
        } else if op == Some(OneShot::Disconnect) && anim == Some(AnimPhase::Disconnecting) && succeeded {
            // Keeps playing; advance_animation stops it at its end pose.
        } else {
            self.stop_animation();
        }
        self.host().kill_timer(TIMER_TIMEOUT);
        self.host().kill_timer(TIMER_FASTPOLL);
        self.host().kill_timer(TIMER_RESEND);
        let op_json = op.map(|o| format!("\"{o}\"")).unwrap_or_else(|| "null".into());
        self.emit(&format!("{{\"type\":\"end\",\"op\":{op_json},\"succeeded\":{succeeded}}}"));
        self.update_icon(false);
    }

    /// One safety-net resend if a reconnect hasn't landed after `ResendAfterMs`.
    fn resend_connect(&self) {
        let me = self.clone();
        self.spawn(async move {
            let (pending, container) = {
                let st = me.st();
                (st.pending, st.pending_container)
            };
            if pending != Some(OneShot::Connect) {
                return;
            }
            log::info(&format!("Connect not confirmed after {} ms; resending once", trunc_i64(me.pending_ms())));
            me.emit("{\"type\":\"resend\"}");
            let send = me.inner.devices.send_one_shot(container, OneShot::Connect);
            if let Err(e) = send.await {
                log::error("resend", &e);
            }
            me.request_refresh();
        });
    }

    // ------------------------------------------------------------------ text

    pub fn status_text(&self) -> String {
        let st = self.st();
        String::from(match st.pending {
            Some(OneShot::Connect) => "Connecting…",
            Some(OneShot::Disconnect) => "Disconnecting…",
            None if !st.have_snapshot => "Checking…",
            None => match &st.target {
                None if self.settings().has_target() => "Not available",
                None => "No device selected",
                Some(t) if t.connected() => "Connected",
                Some(_) => "Not connected",
            },
        })
    }

    fn device_label(&self) -> String {
        if let Some(t) = &self.st().target {
            return t.name.clone();
        }
        self.settings().target_name.clone().unwrap_or_else(|| "QuickTooth".into())
    }

    /// The reading to show (`"30% · case 80%"`) and whether it's the low-battery state, while
    /// connected and the reading is recent.
    fn battery_shown(&self) -> Option<BatteryReading> {
        let (reading, matches) = {
            let st = self.st();
            let target = st.target.as_ref().filter(|t| t.connected())?;
            if st.pending.is_some() {
                return None;
            }
            let b = st.battery.clone()?;
            (b, st.battery_container == target.container_id)
        };
        if !matches || self.host().now_epoch_ms() - reading.at_epoch_ms > int(T::BatteryShowAgeMin) as f64 * MS_PER_MINUTE {
            return None;
        }
        Some(reading)
    }

    /// `"30% · case 80%"` while connected and the reading is recent; otherwise `None`.
    fn battery_text(&self) -> Option<String> {
        self.battery_shown()?.text()
    }

    /// The tooltip shows a battery reading, the buds are at or below the low mark and not charging.
    fn low_battery(&self) -> bool {
        let Some(b) = self.battery_shown() else { return false };
        if b.text().is_none() || b.buds_charging {
            return false;
        }
        let warning = self.settings().low_battery_warning();
        b.buds().is_some_and(|level| level <= warning)
    }

    /// Tooltip and menu header, e.g. "AirPods Pro — Connected · 30%".
    pub fn status_line(&self) -> String {
        let battery = self.battery_text();
        let (label, status) = (self.device_label(), self.status_text());
        match battery {
            Some(b) => format!("{label} — {status} · {b}"),
            None => format!("{label} — {status}"),
        }
    }

    /// Reads AirPods battery from their BLE status advert (Windows doesn't expose it for AirPods).
    fn maybe_scan_battery(&self, max_age_ms: f64) {
        let me = self.clone();
        self.spawn(async move { me.maybe_scan_battery_async(max_age_ms).await });
    }

    async fn maybe_scan_battery_async(&self, max_age_ms: f64) {
        let (target, battery, battery_container) = {
            let st = self.st();
            if st.battery_scanning || st.pending.is_some() {
                return;
            }
            let Some(t) = st.target.clone().filter(BtDevice::connected) else { return };
            (t, st.battery.clone(), st.battery_container)
        };
        if let Some(b) = &battery
            && battery_container == target.container_id
            && (b.source == "windows" || self.host().now_epoch_ms() - b.at_epoch_ms < max_age_ms)
        {
            return;
        }
        let Some(model) = battery::model_of(&target) else { return };
        self.st_mut().battery_scanning = true;
        let scan = self.inner.devices.scan_battery(model, int(T::BleScanSeconds) * 1000);
        match scan.await {
            Err(e) => log::error("battery scan", &e),
            Ok(reading) => {
                let same = self.st().target.as_ref().is_some_and(|t| t.container_id == target.container_id && t.connected());
                if same {
                    match reading {
                        None => log::info("battery: no status advert heard"),
                        Some(reading) => {
                            let old_text = self.st().battery.as_ref().and_then(BatteryReading::text);
                            if old_text != reading.text() {
                                log::info(&format!("battery: {}", reading.describe()));
                            }
                            {
                                let mut st = self.st_mut();
                                st.battery = Some(reading);
                                st.battery_container = target.container_id;
                            }
                            self.update_icon(false);
                        }
                    }
                }
            }
        }
        self.st_mut().battery_scanning = false;
    }

    // ------------------------------------------------------------------ pending animation

    fn start_animation(&self, phase: AnimPhase, tracer_offset: f64) {
        let now = self.host().now_ms();
        {
            let mut st = self.st_mut();
            st.anim = Some(phase);
            st.tracer_offset = tracer_offset;
            st.anim_start = now;
        }
        self.host().set_timer(TIMER_ANIM, int(T::AnimFrameMs));
        self.emit(&format!("{{\"type\":\"anim\",\"phase\":\"{}\"}}", phase.name()));
        self.update_icon(false);
    }

    fn stop_animation(&self) {
        self.st_mut().anim = None;
        self.host().kill_timer(TIMER_ANIM);
        self.emit("{\"type\":\"anim\",\"phase\":null}");
        self.update_icon(false);
    }

    fn advance_animation(&self) {
        let (anim, pending) = {
            let st = self.st();
            (st.anim, st.pending)
        };
        let Some(phase) = anim else {
            self.stop_animation();
            return;
        };
        if icon::phase_done(phase, self.anim_seconds(), pending == Some(OneShot::Disconnect)) {
            self.stop_animation();
        } else {
            self.update_icon(false);
        }
    }

    // ------------------------------------------------------------------ notify icon

    fn update_icon(&self, force: bool) {
        let (anim, icon_size, light) = {
            let st = self.st();
            (st.anim, st.icon_size, st.light_taskbar)
        };
        let icon: Option<Rc<IconImage>>;
        let mut anim_icon: Option<Rc<IconImage>> = None;
        match anim {
            Some(phase) if icon_size > 0 => {
                let t = self.anim_seconds();
                let tracer_t = self.st().tracer_offset + t;
                let desc = format!(
                    "{{\"kind\":\"anim\",\"phase\":\"{}\",\"t\":{},\"tracerT\":{}}}",
                    phase.name(),
                    fmt_r(round_away(t, 4)),
                    fmt_r(round_away(tracer_t, 4))
                );
                let img = Rc::new(IconImage::new(icon::render_anim(phase, t, tracer_t, light, icon_size), icon_size, desc));
                self.st_mut().anim_frames += 1;
                icon = Some(img.clone());
                anim_icon = Some(img);
            }
            _ => {
                let connected = self.target_connected();
                let low = connected && self.low_battery();
                if low != self.st().low_shown {
                    self.st_mut().low_shown = low;
                    if low {
                        self.host().set_timer(TIMER_LOWBATTERY, int(T::LowBatteryReplayMs));
                    } else {
                        self.host().kill_timer(TIMER_LOWBATTERY);
                    }
                    if low && icon_size > 0 {
                        self.start_animation(AnimPhase::LowBattery, 0.0); // the first ripple
                        return;
                    }
                }
                let st = self.st();
                icon = if !connected {
                    st.icon_off.clone()
                } else if low {
                    st.icon_low.clone()
                } else {
                    st.icon_on.clone()
                };
            }
        }
        let Some(icon) = icon else { return };
        let mut tip = self.status_line();
        cut_utf16(&mut tip, int(T::TooltipMaxChars).max(0) as usize);
        let (unchanged, tip_changed, pending) = {
            let st = self.st();
            (same_icon(&Some(icon.clone()), &st.shown_icon) && tip == st.shown_tip, tip != st.shown_tip, st.pending)
        };
        if !force && anim_icon.is_none() && unchanged {
            return;
        }
        if tip_changed && pending.is_none() {
            log::info(&format!("tooltip: {tip}"));
        }
        let (previous_frame, added) = {
            let mut st = self.st_mut();
            let previous = core::mem::replace(&mut st.last_anim_icon, anim_icon);
            st.shown_icon = Some(icon.clone());
            st.shown_tip = tip.clone();
            (previous, st.icon_added)
        };
        if added && !self.host().modify_icon(&icon, &tip) {
            log::info("NIM_MODIFY failed");
        }
        // Animation frames are drawn per tick; release the previous one once the new one is in place.
        if let Some(previous) = previous_frame {
            self.host().release_icon(&previous);
        }
    }

    fn add_icon(&self) {
        if self.st().icon_added {
            return;
        }
        self.update_icon(false); // computes shown_icon / shown_tip
        let (shown, tip) = {
            let st = self.st();
            (st.shown_icon.clone(), st.shown_tip.clone())
        };
        let added = match &shown {
            Some(icon) => self.host().add_icon(icon, &tip),
            None => false,
        };
        if !added {
            let attempts = {
                let mut st = self.st_mut();
                st.add_attempts += 1;
                st.add_attempts
            };
            log::info(&format!("NIM_ADD failed (attempt {attempts}); retrying"));
            if attempts < int(T::AddMaxAttempts) {
                self.host().set_timer(TIMER_ADD_RETRY, int(T::AddRetryMs));
            }
            return;
        }
        let (size, light) = {
            let mut st = self.st_mut();
            st.icon_added = true;
            (st.icon_size, st.light_taskbar)
        };
        log::info(&format!("tray icon added ({size} px, {} taskbar)", light_name(light)));
    }

    fn remove_icon(&self) {
        if !self.st().icon_added {
            return;
        }
        self.host().remove_icon();
        self.st_mut().icon_added = false;
    }

    fn check_icon_metrics(&self) {
        if self.host().icon_size() != self.st().icon_size {
            self.rebuild_icons();
        }
    }

    /// Redraws the static icons (the simulator after a `--tune` change).
    pub fn rebuild_icons_now(&self) {
        if self.running() {
            self.rebuild_icons();
        }
    }

    fn rebuild_icons(&self) {
        let size = self.host().icon_size();
        let light = self.st().light_taskbar;
        let make = |state, desc: &str| Rc::new(IconImage::new(icon::render_bgra(state, light, size), size, desc.into()));
        let on = make(IconState::Connected, "{\"kind\":\"static\",\"state\":\"connected\"}");
        let off = make(IconState::Disconnected, "{\"kind\":\"static\",\"state\":\"disconnected\"}");
        let low = make(IconState::LowBattery, "{\"kind\":\"static\",\"state\":\"connected\",\"lowBattery\":true}");
        let old = {
            let mut st = self.st_mut();
            let old = [st.icon_on.replace(on), st.icon_off.replace(off), st.icon_low.replace(low)];
            st.icon_size = size;
            old
        };
        if old[0].is_some() {
            log::info(&format!("icons rebuilt: {size} px, {} taskbar", light_name(light)));
        }
        self.update_icon(true); // the shell has the new icon before the old one goes
        for img in old.into_iter().flatten() {
            self.host().release_icon(&img);
        }
    }

    // ------------------------------------------------------------------ menu

    /// Builds the popup menu, shows it (modal on Windows), then runs the chosen command.
    fn show_menu(&self, x: i32, y: i32) {
        let me = self.clone();
        self.spawn(async move { me.show_menu_async(x, y).await });
    }

    async fn show_menu_async(&self, x: i32, y: i32) {
        // Refreshes the tooltip if the reading is a bit old.
        self.maybe_scan_battery(int(T::MenuBatteryMaxAgeS) as f64 * 1000.0);
        let mut items = alloc::vec![MenuItem::command(CMD_STATUS, self.status_line()).grayed(true), MenuItem::divider()];

        let (pending, target) = {
            let st = self.st();
            (st.pending, st.target.clone())
        };
        if pending.is_some() {
            items.push(MenuItem::command(0, self.status_text()).grayed(true));
        } else {
            items.push(match &target {
                None => MenuItem::command(0, "Connect").grayed(true),
                Some(t) if t.connected() => MenuItem::command(CMD_DISCONNECT, "Disconnect"),
                Some(_) => MenuItem::command(CMD_CONNECT, "Connect"),
            });
        }

        let menu_devices = {
            let mut st = self.st_mut();
            st.menu_devices = st.device_list.clone();
            st.menu_devices.clone()
        };
        let mut sub = Vec::new();
        for (i, d) in menu_devices.iter().enumerate() {
            let is_target = target.as_ref().is_some_and(|t| t.container_id == d.container_id);
            let text = format!("{}{}", d.name, if d.connected() { "  (connected)" } else { "" });
            sub.push(MenuItem::command(CMD_DEVICE_BASE + i as i32, text).checked(is_target));
        }
        let (has_target, target_name) = {
            let s = self.settings();
            (s.has_target(), s.target_name.clone())
        };
        let missing = target.is_none() && has_target;
        if missing {
            let name = target_name.unwrap_or_else(|| "Saved device".into());
            sub.push(MenuItem::command(0, format!("{name} (not available)")).grayed(true).checked(true));
        }
        if menu_devices.is_empty() && !missing {
            sub.push(MenuItem::command(0, "No Bluetooth audio devices").grayed(true));
        }
        items.push(MenuItem::submenu("Device", sub).grayed(pending.is_some()));

        items.push(MenuItem::divider());
        let autostart = self.host().autostart_enabled();
        items.push(MenuItem::command(CMD_AUTOSTART, "Start with Windows").checked(autostart));
        items.push(MenuItem::command(CMD_EXIT, "Exit"));

        self.emit("{\"type\":\"menu\",\"open\":true}");
        let light = self.st().light_taskbar;
        let menu = self.host().track_popup_menu(&items, x, y, light);
        // A failed menu ends here, as the C# task's unobserved exception did.
        let Ok(cmd) = menu.await else { return };
        self.emit(&format!("{{\"type\":\"menu\",\"open\":false,\"cmd\":{cmd}}}"));
        self.on_menu_command(cmd);
    }

    fn on_menu_command(&self, cmd: i32) {
        match cmd {
            0 => return,
            CMD_CONNECT => return self.start_action(OneShot::Connect, "menu"),
            CMD_DISCONNECT => return self.start_action(OneShot::Disconnect, "menu"),
            CMD_AUTOSTART => {
                let on = self.host().autostart_enabled();
                return self.host().set_autostart(!on);
            }
            CMD_EXIT => return self.exit(),
            _ => {}
        }
        let index = cmd - CMD_DEVICE_BASE;
        let device = {
            let st = self.st();
            if index >= 0 && (index as usize) < st.menu_devices.len() && st.pending.is_none() {
                Some(st.menu_devices[index as usize].clone())
            } else {
                None
            }
        };
        if let Some(d) = device {
            self.with_settings(|s| {
                s.choose_target(&d);
                s.save();
            });
            self.target_chosen(d);
        }
    }

    fn target_chosen(&self, d: BtDevice) {
        self.st_mut().target = Some(d);
        self.update_icon(false);
        self.request_refresh();
    }

    // ------------------------------------------------------------------ the PocketKnives Settings window

    /// `QuickTooth.exe --set`, handed to the running tray: refused while a connect or disconnect
    /// is pending, a device checked against the list the Device menu offers, then applied like a
    /// menu pick and saved.
    pub fn apply_settings(&self, change: &SettingsChange) -> SetResult {
        let (running, pending, devices) = {
            let st = self.st();
            (st.running, st.pending, st.device_list.clone())
        };
        if !running {
            return SetResult::Busy;
        }
        if let Some(p) = pending {
            log::info(&format!("settings: ignored, {p} still pending"));
            return SetResult::Busy;
        }
        let (result, device) = blade_settings::check(change, &self.settings(), &devices);
        if result != SetResult::Applied {
            return result;
        }
        let result = self.with_settings(|s| blade_settings::apply(change, s, device.as_ref()));
        match device {
            Some(d) => self.target_chosen(d),
            None => self.update_icon(false), // a new low-battery warning shows or hides the dot at once
        }
        result
    }

    // ------------------------------------------------------------------ overflow promotion

    /// New tray icons land in the overflow on Windows 11. Once (first run only) mark our own
    /// NotifyIconSettings entry as promoted; after that the user's choice is left alone.
    fn try_promote_once(&self) {
        let attempts = {
            let mut st = self.st_mut();
            st.promote_attempts += 1;
            st.promote_attempts
        };
        let result = self.host().promote_self();
        log::info(&format!("tray promotion attempt {attempts}: {result}"));
        if result == "EntryNotFound" && attempts < int(T::PromoteMaxAttempts) {
            self.host().set_timer(TIMER_PROMOTE, int(T::PromoteRetryMs));
            return;
        }
        self.with_settings(|s| {
            s.tray_promoted_once = true;
            s.save();
        });
    }

    // ------------------------------------------------------------------ introspection (simulator)

    /// The controller's state as JSON (the simulator adds its host-side parts: timers, autostart).
    pub fn snapshot_json(&self) -> String {
        let status = self.status_text();
        let anim = self.st().anim;
        let anim_t = anim.map(|_| round_away(self.anim_seconds(), 3));
        let st = self.st();
        let settings = self.settings();
        let mut w = String::from("{\"running\":");
        w.push_str(if st.running { "true" } else { "false" });
        w.push_str(",\"pending\":");
        match st.pending {
            Some(p) => json::write_str(&mut w, &format!("{p}")),
            None => w.push_str("null"),
        }
        w.push_str(",\"anim\":");
        match (st.anim, anim_t) {
            (Some(a), Some(t)) => {
                json::write_str(&mut w, a.name());
                w.push_str(",\"animT\":");
                write_r(&mut w, t);
            }
            _ => w.push_str("null,\"animT\":null"),
        }
        w.push_str(",\"target\":");
        match &st.target {
            Some(t) => {
                w.push_str("{\"name\":");
                json::write_str(&mut w, &t.name);
                w.push_str(",\"connected\":");
                w.push_str(if t.connected() { "true" } else { "false" });
                w.push_str(",\"stateSignature\":");
                json::write_str(&mut w, &t.state_signature());
                w.push('}');
            }
            None => w.push_str("null"),
        }
        w.push_str(",\"haveSnapshot\":");
        w.push_str(if st.have_snapshot { "true" } else { "false" });
        w.push_str(",\"statusText\":");
        json::write_str(&mut w, &status);
        w.push_str(",\"tooltip\":");
        json::write_str(&mut w, &st.shown_tip);
        w.push_str(",\"icon\":");
        match &st.shown_icon {
            Some(i) => w.push_str(&i.desc_json),
            None => w.push_str("null"),
        }
        w.push_str(&format!(",\"iconSize\":{}", st.icon_size));
        w.push_str(",\"battery\":");
        match &st.battery {
            Some(b) => {
                w.push_str("{\"text\":");
                match b.text() {
                    Some(t) => json::write_str(&mut w, &t),
                    None => w.push_str("null"),
                }
                w.push_str(",\"source\":");
                json::write_str(&mut w, &b.source);
                w.push('}');
            }
            None => w.push_str("null"),
        }
        w.push_str(",\"settings\":{\"targetName\":");
        match &settings.target_name {
            Some(n) => json::write_str(&mut w, n),
            None => w.push_str("null"),
        }
        w.push_str(",\"trayPromotedOnce\":");
        w.push_str(if settings.tray_promoted_once { "true" } else { "false" });
        w.push_str(&format!(",\"lowBatteryWarning\":{}}}", settings.low_battery_warning()));
        w.push_str(&format!(",\"animFrames\":{}}}", st.anim_frames));
        w
    }
}
