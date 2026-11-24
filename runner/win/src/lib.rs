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
use runner_core::blade_settings::{ALT, CTRL, Kind, SHIFT, Setting, WIN, is_extended, join_hotkey, split_hotkey};
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

// Native (XP and Windows 10) settings window geometry, in 96-DPI pixels.
pub const MIN_WINDOW_WIDTH: i32 = 564;
pub const MIN_WINDOW_HEIGHT: i32 = 262;
pub const PAGE_HOST_X: i32 = 174;
pub const PAGE_HOST_Y: i32 = 68;
pub const PAGE_RIGHT_MARGIN: i32 = 8;
pub const PAGE_LOCAL_X: i32 = 4;
pub const PAGE_LOCAL_Y: i32 = 4;
pub const PAGE_BOTTOM_PAD: i32 = 4;
pub const PAGE_BUTTON_GAP: i32 = 12;
pub const BUTTON_ROW_FROM_BOTTOM: i32 = 32;
pub const ROW_HEIGHT: i32 = 22;
pub const ROW_STEP: i32 = 30;
pub const CONTROL_LOCAL_X: i32 = 124; // window x = PAGE_HOST_X + 124 = 298
pub const PICTURE_WIDTH: i32 = 76;
pub const PICTURE_HEIGHT: i32 = 56;
pub const PICTURE_GAP: i32 = 6;
pub const PICTURE_SIZE: i32 = 24;
pub const FRAME_WIDTH: i32 = 2;
pub const SCROLLBAR_RESERVE: i32 = 24;

/// A page's required content area inside `page_host` (96-DPI pixels).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PageSize {
    pub width: i32,
    pub height: i32,
}

/// Measures a blade page's content width and height inside `page_host` (96-DPI pixels).
/// `has_pictures[i]` is true when setting `i` is a `Picture` whose icon files all exist.
pub fn measure_blade_page(settings: &[Setting], has_pictures: &[bool]) -> PageSize {
    if settings.is_empty() {
        return PageSize { width: PAGE_LOCAL_X + 330, height: 6 + 18 + PAGE_BOTTOM_PAD };
    }
    let mut y = PAGE_LOCAL_Y;
    let mut max_bottom = y;
    let mut max_right = PAGE_LOCAL_X + 330;
    for (i, setting) in settings.iter().enumerate() {
        if setting.kind == Kind::Picture && has_pictures.get(i).copied().unwrap_or(false) {
            let count = setting.choices.len() as i32;
            if count > 0 {
                let right = CONTROL_LOCAL_X + count * PICTURE_WIDTH + (count - 1) * PICTURE_GAP + FRAME_WIDTH;
                max_right = max_right.max(right);
            }
            max_bottom = max_bottom.max(y + PICTURE_HEIGHT + FRAME_WIDTH);
            y += PICTURE_HEIGHT + PICTURE_GAP;
        } else {
            max_bottom = max_bottom.max(y + ROW_HEIGHT);
            y += ROW_STEP;
        }
    }
    PageSize { width: max_right, height: max_bottom + PAGE_BOTTOM_PAD }
}

/// The native window's computed client size and derived coordinates (96-DPI pixels).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NativeLayout {
    pub client_width: i32,
    pub client_height: i32,
    pub page_host_width: i32,
    pub page_host_height: i32,
    pub blade_list_height: i32,
    pub hint_y: i32,
    pub button_y: i32,
    pub page_icon_x: i32,
    pub description_width: i32,
    pub ok_x: i32,
    pub cancel_x: i32,
    pub apply_x: i32,
}

impl NativeLayout {
    /// Computes the window's client layout to fit the tallest and widest page, capped to
    /// `work_client` (the monitor work area minus non-client chrome, in 96-DPI pixels).
    pub fn compute(pages: &[PageSize], work_client: Option<(i32, i32)>) -> Self {
        let tallest = pages.iter().map(|p| p.height).max().unwrap_or(0);
        let widest = pages.iter().map(|p| p.width).max().unwrap_or(0);
        let needed_height = PAGE_HOST_Y + tallest + PAGE_BUTTON_GAP + BUTTON_ROW_FROM_BOTTOM;
        let client_height = runner_core::fit_work_area(needed_height, MIN_WINDOW_HEIGHT, work_client.map(|(_, h)| h));
        let page_host_height = client_height - PAGE_HOST_Y - PAGE_BUTTON_GAP - BUTTON_ROW_FROM_BOTTOM;
        let extra_right = if tallest > page_host_height { SCROLLBAR_RESERVE } else { 8 };
        let needed_width = PAGE_HOST_X + widest + extra_right + PAGE_RIGHT_MARGIN;
        let client_width = runner_core::fit_work_area(needed_width, MIN_WINDOW_WIDTH, work_client.map(|(w, _)| w));
        let page_host_width = client_width - PAGE_HOST_X - PAGE_RIGHT_MARGIN;
        let page_icon_x = client_width - 42;
        Self {
            client_width,
            client_height,
            page_host_width,
            page_host_height,
            blade_list_height: client_height - 134,
            hint_y: client_height - 64,
            button_y: client_height - BUTTON_ROW_FROM_BOTTOM,
            page_icon_x,
            description_width: page_icon_x - 192,
            ok_x: client_width - 242,
            cancel_x: client_width - 162,
            apply_x: client_width - 82,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use runner_core::blade_settings::Choice;

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

    fn setting(kind: Kind, choices: usize) -> Setting {
        Setting {
            key: String::from("k"),
            kind,
            value: String::new(),
            label: String::from("L"),
            choices: (0..choices).map(|_| Choice { value: String::from("v"), label: String::from("V") }).collect(),
        }
    }

    #[test]
    fn native_layout_sizing() {
        // Small pages preserve the classic 564 x 262 proportions exactly.
        let small = [
            PageSize { width: PAGE_LOCAL_X + 200, height: 48 },
            measure_blade_page(&[setting(Kind::Toggle, 0)], &[false]),
        ];
        let base = NativeLayout::compute(&small, Some((1920, 1000)));
        assert_eq!((base.client_width, base.client_height), (564, 262));
        assert_eq!((base.blade_list_height, base.hint_y, base.button_y), (128, 198, 230));
        assert_eq!((base.page_icon_x, base.description_width, base.ok_x, base.cancel_x, base.apply_x), (522, 330, 322, 402, 482));

        // SnipSnap's 7 settings (6 hotkeys + 1 toggle) grow the window cleanly above the buttons.
        let snipsnap_settings = [
            setting(Kind::Hotkey, 0),
            setting(Kind::Hotkey, 0),
            setting(Kind::Hotkey, 0),
            setting(Kind::Hotkey, 0),
            setting(Kind::Hotkey, 0),
            setting(Kind::Hotkey, 0),
            setting(Kind::Toggle, 0),
        ];
        let snipsnap = measure_blade_page(&snipsnap_settings, &[false; 7]);
        assert_eq!(snipsnap.height, 210);
        let expanded = NativeLayout::compute(&[snipsnap], Some((1920, 1000)));
        assert_eq!((expanded.client_width, expanded.client_height), (564, 322));
        assert_eq!(expanded.page_host_height, 210);
        assert!(PAGE_HOST_Y + snipsnap.height <= expanded.button_y - PAGE_BUTTON_GAP);

        // QuickTooth's 2 choices + 2-choice picture fit without widening.
        let qt = measure_blade_page(&[setting(Kind::Choice, 2), setting(Kind::Choice, 5), setting(Kind::Picture, 2)], &[false, false, true]);
        assert_eq!(qt.height, 126);
        assert_eq!(NativeLayout::compute(&[qt], Some((1920, 1000))).client_width, 564);

        // A wide 4-choice picture widens the window so nothing clips horizontally.
        let wide = measure_blade_page(&[setting(Kind::Picture, 4)], &[true]);
        assert!(NativeLayout::compute(&[wide], Some((1920, 1000))).client_width > 564);

        // A huge page caps to 85% of work area so the page host enables scrolling.
        let huge: Vec<Setting> = (0..25).map(|_| setting(Kind::Hotkey, 0)).collect();
        let huge_size = measure_blade_page(&huge, &[false; 25]);
        let capped = NativeLayout::compute(&[huge_size], Some((1920, 500)));
        assert_eq!(capped.client_height, 425);
        assert!(huge_size.height > capped.page_host_height);
    }
}
