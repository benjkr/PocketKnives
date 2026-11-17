//! What the tray needs from its platform. The exe implements these with a Win32 window,
//! `Shell_NotifyIcon`, `SetTimer`, `TrackPopupMenu` and the Bluetooth stack; the simulator with
//! its virtual clock, mock taskbar and mock Bluetooth. Every call happens on the host's UI thread.
//!
//! Add a member only for something the platform really provides, and implement it in both hosts
//! in the same change.
use crate::Guid;
use crate::battery::BatteryReading;
use crate::devices::{OneShot, SendResult, Snapshot};
use crate::exec::Op;
use crate::icon::AnimPhase;
use crate::num::fmt_r;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::Cell;

pub trait TrayHost {
    fn exe_path(&self) -> String;

    // ---- time
    /// Monotonic milliseconds (QueryPerformanceCounter on Windows, the virtual clock in the simulator).
    fn now_ms(&self) -> f64;
    /// Wall-clock milliseconds since 1970-01-01 UTC, for the age of battery readings.
    fn now_epoch_ms(&self) -> f64;
    /// Completes after `ms`, on the host's clock.
    fn delay(&self, ms: i32) -> Op<()>;
    /// A repeating timer like Win32 `SetTimer` (setting it again restarts it); the host calls
    /// [`crate::controller::TrayController::on_timer`] on each tick.
    fn set_timer(&self, id: i32, ms: i32);
    fn kill_timer(&self, id: i32);
    fn kill_all_timers(&self);

    // ---- the taskbar
    fn light_taskbar(&self) -> bool;
    /// The small-icon size at the taskbar's DPI (16 px at 100 %).
    fn icon_size(&self) -> i32;
    fn apply_menu_theme(&self, light: bool);
    fn add_icon(&self, icon: &IconImage, tip: &str) -> bool;
    fn modify_icon(&self, icon: &IconImage, tip: &str) -> bool;
    fn remove_icon(&self);
    /// The icon is no longer shown or needed: free its native handle, if the host made one.
    fn release_icon(&self, icon: &IconImage);
    fn cursor_pos(&self) -> Option<(i32, i32)>;
    /// Where a menu opened by command (`--show-menu`) is anchored: the icon's top centre.
    fn icon_anchor(&self) -> Option<(i32, i32)>;
    /// Shows the popup menu; completes with the chosen command ID (0 = dismissed).
    fn track_popup_menu(&self, items: &[MenuItem], x: i32, y: i32, light: bool) -> Op<i32>;
    fn end_menu(&self);
    /// Marks our notification-area entry as promoted; returns the promotion result's name.
    fn promote_self(&self) -> String;

    // ---- system
    fn autostart_enabled(&self) -> bool;
    fn set_autostart(&self, on: bool);
    /// Ends the app (the exe destroys its window, which ends the message loop).
    fn exit(&self);
    /// A structured event for the simulator's timeline ([`Event::to_json`]); ignored by the exe.
    fn emit(&self, event: &Event<'_>);
}

/// The Bluetooth side: Core Audio, KS and BLE on Windows; the mock stack in the simulator.
///
/// Endpoint notifications aren't part of this trait: when Windows reports an audio endpoint
/// change, the host calls `TrayController::on_endpoint_changed` on its UI thread.
pub trait DeviceBackend {
    fn register_notifications(&self) -> Op<()>;
    /// Enumerates Bluetooth audio devices; also reads the battery Windows knows for `mac`.
    fn refresh(&self, mac: Option<&str>) -> Op<(Snapshot, Option<i32>)>;
    /// Sends the one-shot connect or disconnect to every KS control of the device, A2DP first,
    /// and logs each result ([`crate::devices::send_log`]).
    fn send_one_shot(&self, container: Guid, op: OneShot) -> Op<Vec<SendResult>>;
    /// The Bluetooth link's connected flag (`None` when the device isn't known).
    fn bluetooth_connected(&self, mac: &str) -> Op<Option<bool>>;
    fn scan_battery(&self, model: u16, window_ms: i32) -> Op<Option<BatteryReading>>;
}

/// Icon pixels plus what they show. Compared by identity (`Rc::ptr_eq`), as the C# class was.
pub struct IconImage {
    /// Straight-alpha BGRA, top-down rows, `size * size * 4` bytes.
    pub bgra: Vec<u8>,
    pub size: i32,
    pub desc: IconDesc,
    /// The host's native handle for these pixels (the exe's HICON), 0 until it makes one.
    pub host_handle: Cell<isize>,
}

impl IconImage {
    pub fn new(bgra: Vec<u8>, size: i32, desc: IconDesc) -> Self {
        IconImage { bgra, size, desc, host_handle: Cell::new(0) }
    }
}

/// What an icon shows, for the simulator. Only the simulator turns descriptions and events into
/// JSON, so the exe carries no float formatting.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IconDesc {
    Connected,
    Disconnected,
    LowBattery,
    /// An animation frame; `t` and `tracer_t` are already rounded to 4 places.
    Anim {
        phase: AnimPhase,
        t: f64,
        tracer_t: f64,
    },
}

impl IconDesc {
    pub fn is_anim(&self) -> bool {
        matches!(self, IconDesc::Anim { .. })
    }

    /// e.g. `{"kind":"static","state":"connected"}` or
    /// `{"kind":"anim","phase":"Landing","t":0.2,"tracerT":1.6}`.
    pub fn to_json(&self) -> String {
        match *self {
            IconDesc::Connected => "{\"kind\":\"static\",\"state\":\"connected\"}".into(),
            IconDesc::Disconnected => "{\"kind\":\"static\",\"state\":\"disconnected\"}".into(),
            IconDesc::LowBattery => "{\"kind\":\"static\",\"state\":\"connected\",\"lowBattery\":true}".into(),
            IconDesc::Anim { phase, t, tracer_t } => {
                alloc::format!("{{\"kind\":\"anim\",\"phase\":\"{}\",\"t\":{},\"tracerT\":{}}}", phase.name(), fmt_r(t), fmt_r(tracer_t))
            }
        }
    }
}

/// A timeline event for the simulator.
pub enum Event<'a> {
    /// A JSON object with no numbers that need .NET formatting.
    Json(&'a str),
    /// A pending action landed after `ms`.
    Confirmed { op: OneShot, ms: f64 },
}

impl Event<'_> {
    pub fn to_json(&self) -> String {
        match self {
            Event::Json(s) => String::from(*s),
            Event::Confirmed { op, ms } => alloc::format!("{{\"type\":\"confirmed\",\"op\":\"{op}\",\"ms\":{}}}", fmt_r(*ms)),
        }
    }
}

/// One popup menu entry: a separator, a command, or a submenu.
#[derive(Clone, Debug, PartialEq)]
pub struct MenuItem {
    pub id: i32,
    pub text: Option<String>,
    pub grayed: bool,
    pub checked: bool,
    pub separator: bool,
    pub submenu: Option<Vec<MenuItem>>,
}

impl MenuItem {
    pub fn command(id: i32, text: impl Into<String>) -> Self {
        MenuItem { id, text: Some(text.into()), grayed: false, checked: false, separator: false, submenu: None }
    }

    pub fn divider() -> Self {
        MenuItem { id: 0, text: None, grayed: false, checked: false, separator: true, submenu: None }
    }

    pub fn grayed(mut self, grayed: bool) -> Self {
        self.grayed = grayed;
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn submenu(text: impl Into<String>, items: Vec<MenuItem>) -> Self {
        MenuItem { id: 0, text: Some(text.into()), grayed: false, checked: false, separator: false, submenu: Some(items) }
    }
}

/// Commands another QuickTooth.exe sends to the running tray (the `QuickTooth.Command.v1` wParam).
pub mod tray_commands {
    pub const TOGGLE: i32 = 1;
    pub const CONNECT: i32 = 2;
    pub const DISCONNECT: i32 = 3;
    pub const EXIT: i32 = 4;
    pub const SHOW_MENU: i32 = 5;
    pub const CLOSE_MENU: i32 = 6;
}
