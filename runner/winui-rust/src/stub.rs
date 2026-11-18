//! Stand-ins for the runner's settings code (runner/Settings.cs, BladeSettings.cs), which is
//! being ported to Rust separately. The texts are the C# app's; the values are fixed, and a
//! change is only reported on the page's event line. Nothing here touches the registry, the
//! Startup folder or a blade.
use alloc::string::String;
use alloc::vec::Vec;

pub const TITLE: &str = "PocketKnives Settings";
pub const GENERAL_NAME: &str = "General";
pub const GENERAL_DESCRIPTION: &str = "Settings for the whole suite.";
pub const BLADES_LABEL: &str = "Blades";
pub const LOOK_LABEL: &str = "Look";
pub const LOOK_NAMES: [&str; 3] = ["XP", "Windows 10", "Windows 11"];
pub const LOOK_WINDOWS11: i32 = 2;

pub struct Blade {
    pub name: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
}

pub const BLADES: [Blade; 2] = [
    Blade { name: "SnipSnap", description: "Copies a rectangle, a window or the screen to the clipboard.", icon: "snipsnap.ico" },
    Blade { name: "QuickTooth", description: "Connects or disconnects your AirPods with one click.", icon: "quicktooth.ico" },
];

pub enum Kind {
    Toggle,
    Choice(&'static [&'static str]),
    Hotkey,
}

pub struct Setting {
    pub label: &'static str,
    pub kind: Kind,
    pub value: &'static str,
}

/// Each blade's settings, as `--settings` would print them.
pub fn settings(blade: usize) -> &'static [Setting] {
    const SNIPSNAP: &[Setting] = &[
        Setting { label: "Copy a rectangle", kind: Kind::Hotkey, value: "6+83" },
        Setting { label: "Copy a window", kind: Kind::Hotkey, value: "6+87" },
        Setting { label: "Flash the screen", kind: Kind::Toggle, value: "1" },
    ];
    // The third headset is refused, to show the error bar.
    const QUICKTOOTH: &[Setting] = &[
        Setting { label: "Headset", kind: Kind::Choice(&["AirPods Pro", "AirPods Max", "Refused (spike)"]), value: "0" },
        Setting { label: "Low battery warning", kind: Kind::Choice(&["Off", "10%", "20%"]), value: "1" },
    ];
    if blade == 0 { SNIPSNAP } else { QUICKTOOTH }
}

pub const CTRL: u32 = 2;
pub const ALT: u32 = 1;
pub const SHIFT: u32 = 4;
pub const WIN: u32 = 8;

/// "6+83" (modifiers + virtual key) as "Ctrl+Shift+S".
pub fn hotkey_name(value: &str) -> String {
    let (mods, key) = value.split_once('+').unwrap_or(("0", value));
    let mods: u32 = mods.parse().unwrap_or(0);
    let key: u32 = key.parse().unwrap_or(0);
    let mut parts: Vec<String> = Vec::new();
    for (bit, name) in [(CTRL, "Ctrl"), (ALT, "Alt"), (SHIFT, "Shift"), (WIN, "Win")] {
        if mods & bit != 0 {
            parts.push(name.into());
        }
    }
    parts.push(match key {
        0x30..=0x39 | 0x41..=0x5a => char::from_u32(key).map(String::from).unwrap_or_default(),
        0x70..=0x87 => alloc::format!("F{}", key - 0x6f),
        0x2c => "PrtSc".into(),
        _ => alloc::format!("Key {key}"),
    });
    parts.join("+")
}

pub fn join_hotkey(modifiers: u32, key: i32) -> String {
    alloc::format!("{modifiers}+{key}")
}
