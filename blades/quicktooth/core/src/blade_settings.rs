//! QuickTooth's side of the PocketKnives Settings contract (docs/specs/blade-settings.md at the
//! repo root): the lines `--settings` prints, the checks `--set` makes and the words for a
//! refusal. Used by the CLI, by the running tray a `--set` is handed to, and by the simulator.
use crate::devices::BtDevice;
use crate::settings::Settings;
use crate::Guid;
use alloc::format;
use alloc::string::String;

/// What became of a `--set`. The tray returns it to the CLI as the `WM_COPYDATA` result, so 0
/// stays "not handled".
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum SetResult {
    Applied = 1,
    Busy = 2,
    DeviceNotAvailable = 3,
    SaveFailed = 4,
    Invalid = 5,
}

impl SetResult {
    pub fn from_i32(v: i32) -> Option<SetResult> {
        Some(match v {
            1 => SetResult::Applied,
            2 => SetResult::Busy,
            3 => SetResult::DeviceNotAvailable,
            4 => SetResult::SaveFailed,
            5 => SetResult::Invalid,
            _ => return None,
        })
    }
}

/// A `--set` whose form is checked. `device`: a container ID in `B` form, `""` for none, `None`
/// for no change.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SettingsChange {
    pub device: Option<String>,
    pub low_battery: Option<i32>,
}

pub const DEVICE_KEY: &str = "device";
pub const LOW_BATTERY_KEY: &str = "lowBattery";
pub const LOW_BATTERY_CHOICES: [i32; 5] = [10, 15, 20, 25, 30];

pub fn is_low_battery_choice(percent: i32) -> bool {
    LOW_BATTERY_CHOICES.contains(&percent)
}

/// The `--settings` lines: the headset (the devices the Device menu offers, plus the stored one
/// marked "(not available)" when it's missing, as in the menu) and the low-battery warning.
pub fn describe(settings: &Settings, devices: &[BtDevice]) -> String {
    let mut s = String::new();
    let target = if settings.has_target() { settings.resolve(devices) } else { None };
    s.push_str(DEVICE_KEY);
    s.push_str("\tchoice\t");
    s.push_str(&current_device(settings, devices));
    s.push_str("\tHeadset");
    for d in devices {
        choice(&mut s, d.container_id, &d.name);
    }
    if let (None, Some(missing)) = (target, settings.target_guid()) {
        choice(&mut s, missing, &format!("{} (not available)", settings.target_name.as_deref().unwrap_or("Saved device")));
    }
    s.push('\n');

    s.push_str(LOW_BATTERY_KEY);
    s.push_str(&format!("\tchoice\t{}\tLow battery warning", settings.low_battery_warning()));
    for p in LOW_BATTERY_CHOICES {
        s.push_str(&format!("\t{p}={p}%"));
    }
    s.push('\n');
    s
}

fn choice(s: &mut String, id: Guid, label: &str) {
    s.push('\t');
    s.push_str(&id.to_b());
    s.push('=');
    s.extend(label.chars().map(|c| if matches!(c, '\t' | '\r' | '\n') { ' ' } else { c }));
}

/// The headset `--settings` shows as chosen: the stored target (as resolved), `""` if none.
fn current_device(settings: &Settings, devices: &[BtDevice]) -> String {
    if !settings.has_target() {
        return String::new();
    }
    settings.resolve(devices).map(|d| d.container_id).or(settings.target_guid()).map(|g| g.to_b()).unwrap_or_default()
}

/// Checks the pairs' form, keys and values, as far as that's possible without the device list.
/// `Err` holds the line to show.
pub fn parse<S: AsRef<str>>(pairs: &[S]) -> Result<SettingsChange, String> {
    if pairs.is_empty() {
        return Err("Nothing to set.".into());
    }
    let mut change = SettingsChange::default();
    for pair in pairs {
        let pair = pair.as_ref();
        let eq = match pair.find('=') {
            Some(i) if i > 0 => i,
            _ => return Err(format!("Expected key=value, got \"{pair}\".")),
        };
        let (key, value) = (&pair[..eq], &pair[eq + 1..]);
        match key {
            DEVICE_KEY => {
                change.device = Some(if value.is_empty() {
                    String::new()
                } else {
                    match Guid::parse(value) {
                        Some(g) => g.to_b(),
                        None => return Err(message(SetResult::DeviceNotAvailable).into()),
                    }
                });
            }
            LOW_BATTERY_KEY => {
                change.low_battery = LOW_BATTERY_CHOICES.iter().copied().find(|c| value == format!("{c}"));
                if change.low_battery.is_none() {
                    return Err(format!("Low battery warning can be {}.", choice_list()));
                }
            }
            _ => return Err(format!("QuickTooth has no setting called \"{key}\".")),
        }
    }
    Ok(change)
}

/// Checks a headset change against `devices` (what the Device menu offers). The headset already
/// chosen is no change, even while it's missing; any other must be there. Gives the device to
/// choose, or `None` for no change.
pub fn check(change: &SettingsChange, settings: &Settings, devices: &[BtDevice]) -> (SetResult, Option<BtDevice>) {
    let Some(wanted) = change.device.as_deref() else { return (SetResult::Applied, None) };
    if wanted == current_device(settings, devices) {
        return (SetResult::Applied, None);
    }
    match devices.iter().find(|d| d.container_id.to_b() == wanted) {
        Some(d) => (SetResult::Applied, Some(d.clone())),
        None => (SetResult::DeviceNotAvailable, None),
    }
}

/// Applies a checked change (`device` from [`check`]) and saves it, if it changes anything.
pub fn apply(change: &SettingsChange, settings: &mut Settings, device: Option<&BtDevice>) -> SetResult {
    let mut changed = device.is_some();
    if let Some(p) = change.low_battery {
        if p != settings.low_battery_warning() {
            changed = true;
            settings.low_battery_percent = Some(p);
            crate::log!("low battery warning set: {p}%");
        }
    }
    if let Some(d) = device {
        settings.choose_target(d);
    }
    if !changed || settings.save() { SetResult::Applied } else { SetResult::SaveFailed }
}

/// "10%, 15%, 20%, 25% or 30%".
fn choice_list() -> String {
    let mut s = String::new();
    let n = LOW_BATTERY_CHOICES.len();
    for (i, c) in LOW_BATTERY_CHOICES.iter().enumerate() {
        s.push_str(if i == 0 { "" } else if i == n - 1 { " or " } else { ", " });
        s.push_str(&format!("{c}%"));
    }
    s
}

/// The line a refusal shows in Settings.
pub fn message(result: SetResult) -> &'static str {
    match result {
        SetResult::Applied => "",
        SetResult::Busy => "QuickTooth is connecting or disconnecting. Try again in a moment.",
        SetResult::DeviceNotAvailable => "That headset isn't available right now.",
        SetResult::SaveFailed => "QuickTooth couldn't save its settings.",
        SetResult::Invalid => "QuickTooth didn't understand that change.",
    }
}
