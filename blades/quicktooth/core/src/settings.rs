//! `%APPDATA%\QuickTooth\settings.json`, read and written exactly as the C# version's
//! System.Text.Json context did (camelCase names, nulls left out, indented, relaxed escaping), so
//! an existing file round-trips byte for byte. Files are the host's business: the core only turns
//! text into settings and back, and saves through the hook each host sets.
use crate::Guid;
use crate::blade_settings;
use crate::devices::{BtDevice, contains_ignore_case};
use crate::icon::Artwork;
use crate::json::{self, Value};
use crate::log::SaveHook;
use crate::tunables::{self, T};
use alloc::string::String;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Settings {
    /// The target device, by container ID (primary) and Bluetooth MAC (fallback).
    pub target_container_id: Option<String>,
    pub target_mac: Option<String>,
    /// Display only; never used to pick a device.
    pub target_name: Option<String>,
    /// The tray icon was promoted out of the overflow once; never again (the user's choice stands).
    pub tray_promoted_once: bool,
    /// The low-battery warning picked in Settings; `None` means the default in [`tunables`].
    pub low_battery_percent: Option<i32>,
    /// The tray icon's artwork picked in Settings ([`Artwork`] key); `None` means the default.
    pub icon: Option<String>,
}

static SAVE: SaveHook = SaveHook::new();

/// Where [`Settings::save`] goes: the exe writes the file, the simulator keeps it in the page.
pub fn set_save_handler(f: fn(&Settings) -> bool) {
    SAVE.set(f);
}

impl Settings {
    /// The connected icon gets the orange dot at or below this level: the picked one if it's one
    /// of the choices, else the default.
    pub fn low_battery_warning(&self) -> i32 {
        match self.low_battery_percent {
            Some(p) if blade_settings::is_low_battery_choice(p) => p,
            _ => tunables::int(T::LowBatteryPercent),
        }
    }

    /// The artwork the tray draws: the picked one if it's known, else the default.
    pub fn artwork(&self) -> Artwork {
        self.icon.as_deref().and_then(Artwork::from_key).unwrap_or_default()
    }

    pub fn target_guid(&self) -> Option<Guid> {
        self.target_container_id.as_deref().and_then(Guid::parse).filter(|g| !g.is_empty())
    }

    pub fn has_target(&self) -> bool {
        self.target_guid().is_some() || self.target_mac.as_deref().is_some_and(|m| !m.is_empty())
    }

    /// Reads settings.json's text (a UTF-8 BOM is skipped, as `File.ReadAllText` did). `null`
    /// gives the defaults; anything System.Text.Json would refuse is an error.
    pub fn from_json(text: &str) -> Result<Settings, String> {
        let text = text.strip_prefix('\u{feff}').unwrap_or(text);
        let root = json::parse(text)?;
        let mut s = Settings::default();
        let members = match root {
            Value::Null => return Ok(s),
            Value::Obj(m) => m,
            _ => return Err("The JSON value could not be converted to Settings.".into()),
        };
        // In file order, so a later duplicate wins and any bad occurrence fails, as in .NET.
        for (key, v) in members {
            let bad = || alloc::format!("The JSON value could not be converted for '{key}'.");
            match key.as_str() {
                "targetContainerId" | "targetMac" | "targetName" | "icon" => {
                    let value = match v {
                        Value::Null => None,
                        Value::Str(s) => Some(s),
                        _ => return Err(bad()),
                    };
                    match key.as_str() {
                        "targetContainerId" => s.target_container_id = value,
                        "targetMac" => s.target_mac = value,
                        "icon" => s.icon = value,
                        _ => s.target_name = value,
                    }
                }
                "trayPromotedOnce" => s.tray_promoted_once = v.as_bool().ok_or_else(bad)?,
                "lowBatteryPercent" => {
                    s.low_battery_percent = if v.is_null() { None } else { Some(v.as_i32().ok_or_else(bad)?) };
                }
                _ => {}
            }
        }
        Ok(s)
    }

    /// The file's text, indented by two spaces. `newline` is the platform's: `"\r\n"` in the exe,
    /// `"\n"` in the simulator (System.Text.Json used `Environment.NewLine`).
    pub fn to_json(&self, newline: &str) -> String {
        let mut out = String::from("{");
        let mut first = true;
        let mut member = |out: &mut String, name: &str| {
            if !first {
                out.push(',');
            }
            first = false;
            out.push_str(newline);
            out.push_str("  \"");
            out.push_str(name);
            out.push_str("\": ");
        };
        for (name, value) in
            [("targetContainerId", &self.target_container_id), ("targetMac", &self.target_mac), ("targetName", &self.target_name)]
        {
            if let Some(v) = value {
                member(&mut out, name);
                json::write_str_relaxed(&mut out, v);
            }
        }
        member(&mut out, "trayPromotedOnce");
        out.push_str(if self.tray_promoted_once { "true" } else { "false" });
        if let Some(p) = self.low_battery_percent {
            member(&mut out, "lowBatteryPercent");
            out.push_str(&alloc::format!("{p}"));
        }
        if let Some(v) = &self.icon {
            member(&mut out, "icon");
            json::write_str_relaxed(&mut out, v);
        }
        out.push_str(newline);
        out.push('}');
        out
    }

    /// Saves through the host's handler; false if it couldn't be written (or none is set).
    pub fn save(&self) -> bool {
        SAVE.get().is_some_and(|f| f(self))
    }

    pub fn set_target(&mut self, device: &BtDevice) {
        self.target_container_id = Some(device.container_id.to_b());
        self.target_mac = device.mac.clone();
        self.target_name = Some(device.name.clone());
    }

    /// The user picked `device` (the Device menu, or `--set device=…`); not saved yet.
    pub fn choose_target(&mut self, device: &BtDevice) {
        self.set_target(device);
        crate::log!("target set: {} {} MAC {}", device.name, device.container_id, device.mac.as_deref().unwrap_or(""));
    }

    /// The stored target: by container ID, then by MAC. A stored target that's absent gives
    /// `None`: it's never swapped for another device.
    pub fn resolve<'a>(&self, devices: &'a [BtDevice]) -> Option<&'a BtDevice> {
        if let Some(g) = self.target_guid() {
            if let Some(d) = devices.iter().find(|d| d.container_id == g) {
                return Some(d);
            }
        }
        match self.target_mac.as_deref() {
            Some(mac) if !mac.is_empty() => devices.iter().find(|d| d.mac.as_deref().is_some_and(|m| m.eq_ignore_ascii_case(mac))),
            _ => None,
        }
    }

    /// The stored target if one is set (never substituted); otherwise the first-run default,
    /// saved if `persist`.
    pub fn resolve_or_default(&mut self, devices: &[BtDevice], persist: bool) -> Option<BtDevice> {
        if self.has_target() {
            return self.resolve(devices).cloned();
        }
        let pick = default_target(devices)?.clone();
        if persist {
            self.set_target(&pick);
            self.save();
            crate::log!("default target chosen: {} {} MAC {}", pick.name, pick.container_id, pick.mac.as_deref().unwrap_or(""));
        }
        Some(pick)
    }
}

/// The first-run default: a device whose name contains "AirPods", a connected one first. Only
/// used when no target has ever been stored.
pub fn default_target(devices: &[BtDevice]) -> Option<&BtDevice> {
    let airpods = || devices.iter().filter(|d| contains_ignore_case(&d.name, "AirPods"));
    airpods().find(|d| d.connected()).or_else(|| airpods().next())
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn round_trips_and_leaves_out_nulls() {
        let text = "{\r\n  \"targetContainerId\": \"{5a1b2c3d-0001-0002-a1b2-0123456789ab}\",\r\n  \"targetMac\": \"307AD26790E8\",\r\n  \"targetName\": \"Ben’s AirPods Pro\",\r\n  \"trayPromotedOnce\": true\r\n}";
        let s = Settings::from_json(text).unwrap();
        assert_eq!(s.to_json("\r\n"), text);
        assert_eq!(s.low_battery_percent, None);
        let with_low = Settings { low_battery_percent: Some(15), ..s };
        assert!(with_low.to_json("\n").ends_with("\"trayPromotedOnce\": true,\n  \"lowBatteryPercent\": 15\n}"));
        assert_eq!(with_low.artwork(), Artwork::AirPodsPro);
    }

    #[test]
    fn icon_round_trips_and_unknown_means_the_default() {
        let text = "{\n  \"trayPromotedOnce\": false,\n  \"icon\": \"airpods\"\n}";
        let s = Settings::from_json(text).unwrap();
        assert_eq!((s.to_json("\n").as_str(), s.artwork()), (text, Artwork::AirPods));
        let unknown = Settings::from_json("{\"icon\": \"beats\"}").unwrap();
        assert_eq!((unknown.icon.as_deref(), unknown.artwork()), (Some("beats"), Artwork::AirPodsPro));
        assert!(Settings::from_json("{\"icon\": 1}").is_err());
    }

    #[test]
    fn refuses_what_dotnet_refuses() {
        assert!(Settings::from_json("").is_err());
        assert!(Settings::from_json("{\"trayPromotedOnce\": null}").is_err());
        assert!(Settings::from_json("{\"lowBatteryPercent\": 15.0}").is_err());
        assert!(Settings::from_json("[]").is_err());
        assert_eq!(Settings::from_json("null").unwrap(), Settings::default());
        let s = Settings::from_json("{\"TargetMac\": \"x\", \"other\": {\"a\": [1]}, \"lowBatteryPercent\": null}").unwrap();
        assert_eq!(s, Settings::default());
    }
}
