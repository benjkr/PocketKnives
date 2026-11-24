//! The parts of `pk.exe` that are tested on their own: the command line, the names a running
//! runner is found by (Setup and the installed runner use them; Windows keys the tray icon's
//! placement on its GUID), and the hotkey box's values. The exe itself is `main.rs`.
#![no_std]

extern crate alloc;
#[cfg(test)]
extern crate std;

use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::ptr::null;
use runner_core::blade_settings::{ALT, CTRL, SHIFT, WIN, is_extended, join_hotkey, split_hotkey};
use windows_sys::Win32::Globalization::{LCMAP_UPPERCASE, LCMapStringEx, LOCALE_NAME_INVARIANT};
use windows_sys::Win32::Security::Cryptography::{BCRYPT_SHA256_ALG_HANDLE, BCryptHash};
use windows_sys::core::GUID;

/// The suite icon's resource ID (`IDI_APPLICATION`, the one .NET gave it); `build.rs` puts it there.
pub const SUITE_ICON: u16 = 32512;
/// The installed runner's window class and mutex name.
pub const CLASS_NAME: &str = "PocketKnives.Runner";
/// The installed runner's tray icon.
pub const TRAY_IDENTITY: GUID = GUID::from_u128(0xc86c4d67_f167_4393_a49a_935298892f12);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Command {
    /// No command: run the tray (if not already running) and ensure enabled blades are running.
    Run,
    /// `settings`: ensure enabled blades are running and open Settings in the running tray,
    /// starting it first if needed.
    Settings,
    /// `--exit`: close the running tray and Settings.
    Exit,
}

/// The command line: whether it starts with `--preview`, and its command; `None` (exit code 2)
/// for anything else.
pub fn parse(args: &[String]) -> (bool, Option<Command>) {
    let preview = args.first().is_some_and(|a| a == "--preview");
    let args = if preview { &args[1..] } else { args };
    let command = match args {
        [] => Some(Command::Run),
        [c] if c == "settings" => Some(Command::Settings),
        [c] if c == "--exit" => Some(Command::Exit),
        _ => None,
    };
    (preview, command)
}

/// The class and mutex name: [`CLASS_NAME`], or for a preview `PocketKnives.Runner.Preview.`
/// and its identity in .NET's `N` format.
pub fn instance_name(preview: bool, identity: &GUID) -> String {
    if preview { format!("{CLASS_NAME}.Preview.{}", guid_n(identity)) } else { String::from(CLASS_NAME) }
}

/// A preview's tray identity, one per folder: the first 16 bytes of SHA-256 over the UTF-8 of
/// the folder upper-cased (.NET's `ToUpperInvariant`), read as a GUID (as .NET's `new
/// Guid(bytes)`: the first three fields little-endian).
pub fn preview_identity(folder: &str) -> GUID {
    let text = upper_invariant(folder);
    let mut hash = [0u8; 32];
    unsafe { BCryptHash(BCRYPT_SHA256_ALG_HANDLE, core::ptr::null(), 0, text.as_ptr(), text.len() as u32, hash.as_mut_ptr(), 32) };
    let b = hash;
    GUID {
        data1: u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        data2: u16::from_le_bytes([b[4], b[5]]),
        data3: u16::from_le_bytes([b[6], b[7]]),
        data4: [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]],
    }
}

/// .NET's `Guid.ToString("N")`: 32 lower-case hex digits, the fields in order.
pub fn guid_n(g: &GUID) -> String {
    let d = g.data4;
    format!(
        "{:08x}{:04x}{:04x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        g.data1, g.data2, g.data3, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]
    )
}

/// .NET's `ToUpperInvariant`: each character's simple upper case (`ß`, which has none, stays),
/// as Windows' invariant locale maps it.
fn upper_invariant(s: &str) -> String {
    let text: Vec<u16> = s.encode_utf16().collect();
    let mut upper = vec![0u16; text.len()];
    let n = unsafe {
        LCMapStringEx(
            LOCALE_NAME_INVARIANT,
            LCMAP_UPPERCASE,
            text.as_ptr(),
            text.len() as i32,
            upper.as_mut_ptr(),
            upper.len() as i32,
            null(),
            null(),
            0,
        )
    };
    String::from_utf16_lossy(&upper[..n.max(0) as usize])
}

// The hotkey box's value is the key | HOTKEYF flags << 8; a blade's is RegisterHotKey's.
const HOTKEYF_SHIFT: u32 = 1;
const HOTKEYF_CONTROL: u32 = 2;
const HOTKEYF_ALT: u32 = 4;
const HOTKEYF_EXT: u32 = 8;

/// A blade's hotkey value as the hotkey box takes it (`HKM_SETHOTKEY`). The box can't show Win.
pub fn hotkey_box(value: &str) -> usize {
    let (modifiers, key) = split_hotkey(value);
    let mut flags = 0;
    for (bit, flag) in [(SHIFT, HOTKEYF_SHIFT), (CTRL, HOTKEYF_CONTROL), (ALT, HOTKEYF_ALT)] {
        if modifiers & bit != 0 {
            flags |= flag;
        }
    }
    if is_extended(key) {
        flags |= HOTKEYF_EXT;
    }
    (key | flags << 8) as usize
}

/// The blade's value for what the hotkey box holds (`HKM_GETHOTKEY`), `saved` being the value
/// the blade has. An emptied box keeps the saved key, and so does an unchanged one (which keeps
/// a Win the box couldn't show).
pub fn hotkey_value(boxed: u32, saved: &str) -> String {
    let key = boxed & 0xff;
    let flags = boxed >> 8 & 0xff;
    if key == 0 {
        return String::from(saved);
    }
    let mut modifiers = 0;
    for (flag, bit) in [(HOTKEYF_SHIFT, SHIFT), (HOTKEYF_CONTROL, CTRL), (HOTKEYF_ALT, ALT)] {
        if flags & flag != 0 {
            modifiers |= bit;
        }
    }
    let (saved_modifiers, saved_key) = split_hotkey(saved);
    if saved_key == key && saved_modifiers & !WIN == modifiers {
        return String::from(saved);
    }
    join_hotkey(modifiers, key)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| String::from(*s)).collect()
    }

    #[test]
    fn command_line() {
        assert_eq!(parse(&args(&[])), (false, Some(Command::Run)));
        assert_eq!(parse(&args(&["settings"])), (false, Some(Command::Settings)));
        assert_eq!(parse(&args(&["--exit"])), (false, Some(Command::Exit)));
        assert_eq!(parse(&args(&["--preview"])), (true, Some(Command::Run)));
        assert_eq!(parse(&args(&["--preview", "settings"])), (true, Some(Command::Settings)));
        assert_eq!(parse(&args(&["--preview", "--exit"])), (true, Some(Command::Exit)));
        assert_eq!(parse(&args(&["--preview", "--preview"])), (true, None));
        assert_eq!(parse(&args(&["--preview", "settings", "x"])), (true, None));
        assert_eq!(parse(&args(&["settings", "--preview"])), (false, None));
        assert_eq!(parse(&args(&["Settings"])), (false, None));
        assert_eq!(parse(&args(&[""])), (false, None));
    }

    /// The expected names are .NET's, from PowerShell 7:
    /// `[guid]::new([Security.Cryptography.SHA256]::HashData([Text.Encoding]::UTF8.GetBytes($p.ToUpperInvariant()))[0..15]).ToString('N')`.
    #[test]
    fn names() {
        assert_eq!(guid_n(&preview_identity("C:\\Users\\Administrator\\Scratch\\pk-rust")), "a5c9f45de2945d5e3d05a42f8ba0a64a");
        assert_eq!(guid_n(&preview_identity("c:\\Temp\\Ünïcødé ß folder\\x")), "0c1deaee6771824edaa319dc5444f689");
        assert_eq!(upper_invariant("c:\\Temp\\Ünïcødé ß folder\\x"), "C:\\TEMP\\ÜNÏCØDÉ ß FOLDER\\X");
        assert_eq!(guid_n(&TRAY_IDENTITY), "c86c4d67f1674393a49a935298892f12");
        assert_eq!(instance_name(false, &TRAY_IDENTITY), "PocketKnives.Runner");
        assert_eq!(
            instance_name(true, &preview_identity("C:\\Users\\Administrator\\Scratch\\pk-rust")),
            "PocketKnives.Runner.Preview.a5c9f45de2945d5e3d05a42f8ba0a64a"
        );
    }

    #[test]
    fn hotkey_box_values() {
        // Shift (blade 4) + D: HOTKEYF_SHIFT 1. Ctrl+Alt (2|1) + Delete (extended): 2|4|8.
        assert_eq!(hotkey_box("1092"), 0x144);
        assert_eq!(hotkey_box(&join_hotkey(CTRL | ALT, 0x2e)), 0xe2e);
        // Win alone can't be shown: the key without it.
        assert_eq!(hotkey_box(&join_hotkey(WIN | SHIFT, 0x53)), 0x153);
        assert_eq!(hotkey_box(""), 0);

        let saved = join_hotkey(WIN | SHIFT, 0x53);
        assert_eq!(hotkey_value(0, &saved), saved); // emptied
        assert_eq!(hotkey_value(0x153, &saved), saved); // unchanged: keeps Win
        assert_eq!(hotkey_value(0x353, &saved), join_hotkey(CTRL | SHIFT, 0x53));
        assert_eq!(hotkey_value(0x641, "0"), join_hotkey(CTRL | ALT, 0x41));
        // The extended flag isn't a modifier.
        assert_eq!(hotkey_value(0xe2e, "0"), join_hotkey(CTRL | ALT, 0x2e));
    }
}
