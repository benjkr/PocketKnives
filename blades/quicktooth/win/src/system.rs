//! Windows settings QuickTooth reads or writes: Start with Windows (the Run value), the taskbar's
//! light/dark mode, the notification-area promotion and the dark popup menus
//! (docs/specs/tray.md).
use crate::files;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::ptr::{null, null_mut};
use pk::reg::{Key, dword as reg_dword, string as reg_string};
use pk::wide::to_wide;
use quicktooth_core::Guid;
use windows_sys::Win32::Foundation::{ERROR_FILE_NOT_FOUND, ERROR_SUCCESS, HWND};
use windows_sys::Win32::Storage::FileSystem::GetFullPathNameW;
use windows_sys::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
use windows_sys::Win32::System::Registry::{
    HKEY_CURRENT_USER, KEY_QUERY_VALUE, KEY_READ, KEY_SET_VALUE, REG_DWORD, REG_SZ, RegDeleteKeyValueW, RegEnumKeyExW, RegSetKeyValueW,
    RegSetValueExW,
};

const RUN_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
const RUN_VALUE: &str = "QuickTooth";

// ---------------------------------------------------------------------------- Start with Windows

/// The Run value, `None` without one.
pub fn autostart_value() -> Option<String> {
    reg_string(HKEY_CURRENT_USER, Some(RUN_KEY), RUN_VALUE)
}

pub fn autostart_enabled() -> bool {
    autostart_value().is_some_and(|v| !v.trim().is_empty())
}

pub fn set_autostart(on: bool) {
    let (key, name) = (to_wide(RUN_KEY), to_wide(RUN_VALUE));
    if on {
        let exe = pk::env::exe_path();
        let value = to_wide(&format!("\"{exe}\""));
        let err = unsafe {
            RegSetKeyValueW(HKEY_CURRENT_USER, key.as_ptr(), name.as_ptr(), REG_SZ, value.as_ptr().cast(), (value.len() * 2) as u32)
        };
        if err != 0 {
            return quicktooth_core::log::error("autostart", &format!("RegSetKeyValue error {err}"));
        }
        quicktooth_core::log!("autostart on: \"{exe}\"");
    } else {
        let err = unsafe { RegDeleteKeyValueW(HKEY_CURRENT_USER, key.as_ptr(), name.as_ptr()) };
        // A value that isn't there is already off (C#'s DeleteValue(name, throwOnMissingValue: false)).
        if err != 0 && err != windows_sys::Win32::Foundation::ERROR_FILE_NOT_FOUND {
            return quicktooth_core::log::error("autostart", &format!("RegDeleteKeyValue error {err}"));
        }
        quicktooth_core::log!("autostart off");
    }
}

// ---------------------------------------------------------------------------- theme

/// The taskbar follows `SystemUsesLightTheme` (Windows mode), not `AppsUseLightTheme`. Missing
/// means dark, the Windows 10/11 taskbar default.
pub fn taskbar_is_light() -> bool {
    reg_dword(HKEY_CURRENT_USER, Some("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize"), "SystemUsesLightTheme")
        .is_some_and(|v| v != 0)
}

/// Dark or light popup menus, through uxtheme's undocumented exports (Windows 10 1903+), by
/// ordinal: SetPreferredAppMode #135, AllowDarkModeForWindow #133, FlushMenuThemes #136.
pub fn apply_menu_theme(hwnd: HWND, light: bool) {
    const FORCE_DARK: i32 = 2;
    const FORCE_LIGHT: i32 = 3;
    let lib = unsafe { LoadLibraryW(pk::w!("uxtheme.dll")) };
    let export = |ordinal: usize| -> Result<unsafe extern "system" fn() -> isize, String> {
        if lib.is_null() {
            return Err(format!("Unable to load DLL 'uxtheme.dll' or one of its dependencies: {}", files::last_error_message()));
        }
        unsafe { GetProcAddress(lib, ordinal as *const u8) }
            .ok_or_else(|| format!("Unable to find an entry point named '#{ordinal}' in DLL 'uxtheme.dll'."))
    };
    let run = || -> Result<(), String> {
        let f = export(135)?;
        unsafe { core::mem::transmute::<_, unsafe extern "system" fn(i32) -> i32>(f)(if light { FORCE_LIGHT } else { FORCE_DARK }) };
        let f = export(133)?;
        unsafe { core::mem::transmute::<_, unsafe extern "system" fn(HWND, i32) -> i32>(f)(hwnd, (!light) as i32) };
        let f = export(136)?;
        unsafe { core::mem::transmute::<_, unsafe extern "system" fn()>(f)() };
        Ok(())
    };
    if let Err(e) = run() {
        quicktooth_core::log!("dark menu support unavailable: {e}");
    }
}

// ---------------------------------------------------------------------------- tray promotion

/// The outcome of [`promote_self`], named as the core's log and the C# enum name them.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Promotion {
    Promoted,
    AlreadyPromoted,
    EntryNotFound,
    Failed,
}

impl Promotion {
    pub fn name(self) -> &'static str {
        match self {
            Promotion::Promoted => "Promoted",
            Promotion::AlreadyPromoted => "AlreadyPromoted",
            Promotion::EntryNotFound => "EntryNotFound",
            Promotion::Failed => "Failed",
        }
    }
}

const NOTIFY_ICON_SETTINGS: &str = "Control Panel\\NotifyIconSettings";

/// Windows 11 keeps per-icon notification-area settings under
/// `HKCU\Control Panel\NotifyIconSettings\<id>` (`ExecutablePath`, `IsPromoted`, …).
/// `IsPromoted=1` shows the icon on the taskbar instead of in the overflow.
pub fn promote_self() -> Promotion {
    match try_promote() {
        Ok(p) => p,
        Err(e) => {
            quicktooth_core::log::error("promote tray icon", &e);
            Promotion::Failed
        }
    }
}

fn try_promote() -> Result<Promotion, String> {
    let exe = full_path(&pk::env::exe_path());
    let root = match Key::open(HKEY_CURRENT_USER, NOTIFY_ICON_SETTINGS, KEY_READ) {
        Ok(k) => k,
        Err(ERROR_FILE_NOT_FOUND) => return Ok(Promotion::EntryNotFound),
        Err(e) => return Err(files::error_message(e)),
    };
    let mut result = Promotion::EntryNotFound;
    for name in subkey_names(&root)? {
        let key = match Key::open(root.0, &name, KEY_QUERY_VALUE | KEY_SET_VALUE) {
            Ok(k) => k,
            Err(ERROR_FILE_NOT_FOUND) => continue,
            Err(e) => return Err(files::error_message(e)),
        };
        let Some(path) = reg_string(key.0, None, "ExecutablePath") else { continue };
        if !eq_ignore_case(&resolve_known_folder(&path), &exe) {
            continue;
        }
        if reg_dword(key.0, None, "IsPromoted") == Some(1) {
            if result == Promotion::EntryNotFound {
                result = Promotion::AlreadyPromoted;
            }
            continue;
        }
        let one = 1u32;
        let value = to_wide("IsPromoted");
        let r = unsafe { RegSetValueExW(key.0, value.as_ptr(), 0, REG_DWORD, (&one as *const u32).cast(), 4) };
        if r != ERROR_SUCCESS {
            return Err(files::error_message(r));
        }
        quicktooth_core::log!("NotifyIconSettings\\{name}: IsPromoted=1 ({path})");
        result = Promotion::Promoted;
    }
    Ok(result)
}

fn subkey_names(key: &Key) -> Result<Vec<String>, String> {
    let mut names = Vec::new();
    let mut buf = [0u16; 256]; // registry key names are at most 255 characters
    for i in 0.. {
        let mut len = buf.len() as u32;
        let r = unsafe { RegEnumKeyExW(key.0, i, buf.as_mut_ptr(), &mut len, null(), null_mut(), null_mut(), null_mut()) };
        if r != ERROR_SUCCESS {
            break;
        }
        names.push(String::from_utf16_lossy(&buf[..len as usize]));
    }
    Ok(names)
}

/// `ExecutablePath` may be stored as `{KNOWNFOLDERID}\relative\path.exe`.
fn resolve_known_folder(path: &str) -> String {
    let b = path.as_bytes();
    if path.len() > 38 && b[0] == b'{' && b[37] == b'}' && path.is_char_boundary(38) {
        if let Some(id) = Guid::parse(&path[..38]) {
            let g = windows_sys::core::GUID { data1: id.data1, data2: id.data2, data3: id.data3, data4: id.data4 };
            let resolved = if let Some(folder) = pk::env::known_folder(&g) {
                let rest = path[38..].trim_start_matches('\\');
                Some(full_path(&if folder.ends_with('\\') || rest.is_empty() {
                    format!("{folder}{rest}")
                } else {
                    format!("{folder}\\{rest}")
                }))
            } else {
                None
            };
            return resolved.unwrap_or_else(|| path.into());
        }
    }
    full_path(path)
}

/// `Path.GetFullPath`, or `path` itself when Windows can't make one.
fn full_path(path: &str) -> String {
    let w = to_wide(path);
    let mut buf: Vec<u16> = vec![0; 260];
    loop {
        let n = unsafe { GetFullPathNameW(w.as_ptr(), buf.len() as u32, buf.as_mut_ptr(), null_mut()) } as usize;
        if n == 0 {
            return path.into();
        }
        if n < buf.len() {
            return String::from_utf16_lossy(&buf[..n]);
        }
        buf.resize(n, 0);
    }
}

/// .NET's `OrdinalIgnoreCase`: each UTF-16 unit's simple upper-case mapping.
fn eq_ignore_case(a: &str, b: &str) -> bool {
    fn upper(c: char) -> char {
        let mut u = c.to_uppercase();
        match (u.next(), u.next()) {
            (Some(x), None) => x,
            _ => c,
        }
    }
    a.chars().map(upper).eq(b.chars().map(upper))
}
