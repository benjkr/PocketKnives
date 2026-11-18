//! A blade's settings as it prints them, and changing them through it
//! (docs/specs/blade-settings.md). Both settings windows use this.
use crate::settings::{BLADES, Blade, Suite};
use crate::{Result, files, process};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetKeyNameTextW, MAPVK_VK_TO_VSC, MapVirtualKeyW};

/// The time a blade gets to print or change its settings.
const TIMEOUT_MS: u32 = 5000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Hotkey,
    Toggle,
    Choice,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Choice {
    pub value: String,
    pub label: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Setting {
    pub key: String,
    pub kind: Kind,
    pub value: String,
    pub label: String,
    pub choices: Vec<Choice>,
}

/// Each blade's settings, in [`BLADES`] order: none when its exe is missing or its settings
/// command fails; the error is a blade that couldn't be started or didn't answer. The blades are
/// asked at the same time, so opening Settings waits for the slowest one, not for each in turn.
pub fn read_all(suite: &Suite) -> [Result<Vec<Setting>>; BLADES.len()] {
    let exes = BLADES.each_ref().map(|blade| suite.blade_exe(blade));
    let commands: [_; BLADES.len()] = core::array::from_fn(|i| (exes[i].as_str(), core::slice::from_ref(&BLADES[i].settings_command)));
    let mut answers = process::run_all(commands, TIMEOUT_MS).into_iter();
    exes.each_ref().map(|exe| match answers.next() {
        Some(Ok((0, output))) => Ok(parse(&output)),
        // A missing exe can't start: that blade has no settings, not an error.
        Some(Err(e)) if files::exists(exe) => Err(e),
        _ => Ok(Vec::new()),
    })
}

/// The settings a blade printed: one per line, tab-separated `key, kind, value, label` and then
/// `value=label` choices. Lines it can't use are skipped.
pub fn parse(output: &str) -> Vec<Setting> {
    let mut settings = Vec::new();
    for line in output.split('\n') {
        let fields: Vec<&str> = line.trim_end_matches('\r').split('\t').collect();
        if fields.len() < 4 || fields[0].is_empty() {
            continue;
        }
        let kind = match fields[1] {
            "hotkey" => Kind::Hotkey,
            "toggle" => Kind::Toggle,
            "choice" => Kind::Choice,
            _ => continue,
        };
        let choices = fields[4..]
            .iter()
            .filter_map(|c| match c.find('=') {
                Some(i) if i > 0 => Some(Choice { value: c[..i].into(), label: c[i + 1..].into() }),
                _ => None,
            })
            .collect();
        settings.push(Setting { key: fields[0].into(), kind, value: fields[2].into(), label: fields[3].into(), choices });
    }
    settings
}

/// Changes the blade's settings (`key=value` each). The error is the blade's own reason when it
/// refuses (its first line of output).
pub fn apply(suite: &Suite, blade: &Blade, changes: &[(String, String)]) -> Result<()> {
    if changes.is_empty() {
        return Ok(());
    }
    let pairs: Vec<String> = changes.iter().map(|(k, v)| format!("{k}={v}")).collect();
    let mut args: Vec<&str> = Vec::with_capacity(pairs.len() + 1);
    args.push(blade.set_command);
    args.extend(pairs.iter().map(String::as_str));
    let exe = suite.blade_exe(blade);
    if !files::exists(&exe) {
        return Err(format!("{} is missing from the PocketKnives folder.", blade.name));
    }
    let (code, output) = process::run(&exe, &args, TIMEOUT_MS)?;
    if code != 0 {
        let reason = output.split('\n').map(str::trim).find(|l| !l.is_empty());
        return Err(reason.map_or_else(|| format!("{} could not change its settings.", blade.name), String::from));
    }
    Ok(())
}

// A hotkey value is RegisterHotKey's modifiers << 8 | the virtual key, in decimal.
pub const ALT: u32 = 1;
pub const CTRL: u32 = 2;
pub const SHIFT: u32 = 4;
pub const WIN: u32 = 8;

/// (modifiers, key); (0, 0) for no hotkey or a value that isn't one.
pub fn split_hotkey(value: &str) -> (u32, u32) {
    match parse_int(value) {
        Some(n) if n > 0 => ((n >> 8 & 0xf) as u32, (n & 0xff) as u32),
        _ => (0, 0),
    }
}

pub fn join_hotkey(modifiers: u32, key: u32) -> String {
    (modifiers << 8 | key).to_string()
}

/// e.g. `Ctrl+Shift+S`, `PrtSc`, or `None`.
pub fn hotkey_name(value: &str) -> String {
    let (modifiers, key) = split_hotkey(value);
    if key == 0 {
        return String::from("None");
    }
    let mut name = String::new();
    for (bit, text) in [(CTRL, "Ctrl+"), (ALT, "Alt+"), (SHIFT, "Shift+"), (WIN, "Win+")] {
        if modifiers & bit != 0 {
            name.push_str(text);
        }
    }
    name.push_str(&key_name(key));
    name
}

/// The keyboard layout's name for a key.
fn key_name(key: u32) -> String {
    if key == 0x2c {
        return String::from("PrtSc"); // Windows names it "Prnt Scrn"; the blades' own text says PrtSc.
    }
    let scan = unsafe { MapVirtualKeyW(key, MAPVK_VK_TO_VSC) };
    let mut text = [0u16; 64];
    let lparam = (scan << 16 | if is_extended(key) { 1 << 24 } else { 0 }) as i32;
    let n = unsafe { GetKeyNameTextW(lparam, text.as_mut_ptr(), text.len() as i32) };
    if n > 0 { String::from_utf16_lossy(&text[..n as usize]) } else { format!("Key {key}") }
}

/// Navigation keys share scan codes with the number pad; the extended bit tells them apart.
pub fn is_extended(key: u32) -> bool {
    matches!(key, 0x21..=0x2e | 0x5b..=0x5d | 0x6f | 0x90)
}

/// An integer as .NET's `int.TryParse` reads one: optional surrounding whitespace and sign.
fn parse_int(s: &str) -> Option<i32> {
    let s = s.trim();
    let (negative, digits) = match s.as_bytes().first() {
        Some(b'-') => (true, &s[1..]),
        Some(b'+') => (false, &s[1..]),
        _ => (false, s),
    };
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let mut n: i64 = 0;
    for b in digits.bytes() {
        n = n * 10 + (b - b'0') as i64;
        if n > i32::MAX as i64 + 1 {
            return None;
        }
    }
    let n = if negative { -n } else { n };
    i32::try_from(n).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn parses_what_blades_print() {
        let out = "rect\thotkey\t1092\tRectangle\r\nflash\ttoggle\t1\tFlash the captured area\r\n\
                   device\tchoice\tAA:BB\tHeadset\tAA:BB=AirPods Pro\t=skipped\tnoequals\tCC:DD=Other=x\n\
                   \tchoice\tx\tno key\nbad\tslider\t1\tunknown kind\nshort\ttoggle\t1\n\n";
        let s = parse(out);
        assert_eq!(s.len(), 3);
        assert_eq!(
            s[0],
            Setting { key: "rect".into(), kind: Kind::Hotkey, value: "1092".into(), label: "Rectangle".into(), choices: vec![] }
        );
        assert_eq!(s[1].kind, Kind::Toggle);
        assert_eq!(
            s[2].choices,
            vec![Choice { value: "AA:BB".into(), label: "AirPods Pro".into() }, Choice { value: "CC:DD".into(), label: "Other=x".into() }]
        );
    }

    #[test]
    fn hotkeys() {
        assert_eq!(split_hotkey("1092"), (4, 0x44));
        assert_eq!(split_hotkey(" +1092 "), (4, 0x44));
        assert_eq!(split_hotkey("0"), (0, 0));
        assert_eq!(split_hotkey("-5"), (0, 0));
        assert_eq!(split_hotkey("x"), (0, 0));
        assert_eq!(split_hotkey("99999999999"), (0, 0));
        assert_eq!(split_hotkey("2147483647"), (0xf, 0xff));
        assert_eq!(join_hotkey(CTRL | SHIFT, 0x53), "1619");
        assert_eq!(hotkey_name(""), "None");
        assert_eq!(hotkey_name(&join_hotkey(0, 0x2c)), "PrtSc");
        assert_eq!(hotkey_name(&join_hotkey(CTRL | ALT | SHIFT | WIN, 0x2c)), "Ctrl+Alt+Shift+Win+PrtSc");
        assert!(is_extended(0x2e) && is_extended(0x90) && !is_extended(0x41));
    }
}
