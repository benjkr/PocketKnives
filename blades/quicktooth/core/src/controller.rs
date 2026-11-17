//! The tray icon's behaviour, shared by the exe and the simulator: the connect/disconnect state
//! machine (link-teardown wait, resend, timeout), the pending animation, tooltip text, battery
//! readings and the popup menu's contents (docs/specs/tray.md). The platform comes in through
//! [`TrayHost`] and [`DeviceBackend`]. Single-threaded: every call and task wake-up runs on the
//! host's UI thread.
//!
//! GROUNDWORK STUB: the API is final; the bodies are placeholders until the controller is ported.
use crate::blade_settings::{SetResult, SettingsChange};
use crate::exec::Schedule;
use crate::host::{DeviceBackend, TrayHost};
use crate::settings::Settings;
use alloc::rc::Rc;
use alloc::string::String;

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

/// A handle to one tray app (cheap to clone; all clones are the same app).
#[derive(Clone)]
pub struct TrayController {
    _inner: Rc<()>,
}

impl TrayController {
    /// `schedule` runs the controller's woken tasks: the exe posts them to its window, the
    /// simulator runs them inline ([`crate::exec::Inline`]).
    pub fn new(_host: Rc<dyn TrayHost>, _devices: Rc<dyn DeviceBackend>, _settings: Settings, _schedule: Rc<dyn Schedule>) -> Self {
        TrayController { _inner: Rc::new(()) }
    }

    pub fn running(&self) -> bool {
        false
    }

    pub fn start(&self) {}

    /// Exit from the menu or `QuickTooth.exe --exit`; also the window being destroyed.
    pub fn exit(&self) {}

    /// A notification-area event: `"select"` (left click), `"keyselect"`, or `"contextmenu"` at x, y.
    pub fn on_tray_event(&self, _ev: &str, _x: i32, _y: i32) {}

    /// A command from another QuickTooth.exe ([`crate::host::tray_commands`]).
    pub fn on_command(&self, _cmd: i32) {}

    pub fn on_timer(&self, _id: i32) {}

    /// An audio endpoint changed (the exe calls it on its UI thread after the notification).
    pub fn on_endpoint_changed(&self) {}

    /// `WM_SETTINGCHANGE`: `"ImmersiveColorSet"` means the taskbar theme may have flipped.
    pub fn on_setting_change(&self, _area: Option<&str>) {}

    /// `WM_DISPLAYCHANGE` / `WM_DPICHANGED`.
    pub fn on_display_change(&self) {}

    /// `"TaskbarCreated"`: Explorer restarted, so the icon is added again.
    pub fn on_taskbar_created(&self) {}

    /// Redraws the static icons (the simulator after a `--tune` change).
    pub fn rebuild_icons_now(&self) {}

    /// `QuickTooth.exe --set`, handed to the running tray.
    pub fn apply_settings(&self, _change: &SettingsChange) -> SetResult {
        SetResult::Busy
    }

    pub fn status_text(&self) -> String {
        String::new()
    }

    /// Tooltip and menu header, e.g. "AirPods Pro — Connected · 30%".
    pub fn status_line(&self) -> String {
        String::new()
    }

    /// The controller's state as JSON (the simulator adds its host-side parts).
    pub fn snapshot_json(&self) -> String {
        String::from("{}")
    }
}
