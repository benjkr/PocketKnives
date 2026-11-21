//! Settings persistence (`%APPDATA%\NowPlaying\settings.ini`, or next to the exe in preview mode),
//! autostart (`HKCU\...\Run\NowPlaying`), taskbar theme detection and dark/light menu support.
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::ptr::{null, null_mut};
use pk::reg::{dword as reg_dword, string as reg_string};
use pk::wide::to_wide;
use windows_sys::Win32::Foundation::{CloseHandle, ERROR_FILE_NOT_FOUND, GENERIC_READ, GENERIC_WRITE, HWND, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Storage::FileSystem::{
    CREATE_ALWAYS, CreateDirectoryW, CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, GetFileSize, OPEN_EXISTING, ReadFile, WriteFile,
};
use windows_sys::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
use windows_sys::Win32::System::Registry::{HKEY_CURRENT_USER, REG_SZ, RegDeleteKeyValueW, RegSetKeyValueW};
use windows_sys::Win32::UI::Shell::FOLDERID_RoamingAppData;

const RUN_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
const RUN_VALUE: &str = "NowPlaying";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DockPosition {
    Left,
    BeforeTray,
}

impl DockPosition {
    pub fn as_str(self) -> &'static str {
        match self {
            DockPosition::Left => "left",
            DockPosition::BeforeTray => "tray",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.trim() {
            "left" => Some(DockPosition::Left),
            "tray" => Some(DockPosition::BeforeTray),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Settings {
    pub position: DockPosition,
    pub visualizer: bool,
    pub hide_when_idle: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { position: DockPosition::Left, visualizer: true, hide_when_idle: true }
    }
}

impl Settings {
    pub fn load() -> Self {
        let path = settings_path();
        let Some(bytes) = read_file(&path) else { return Self::default() };
        let Ok(text) = core::str::from_utf8(&bytes) else { return Self::default() };
        let mut s = Self::default();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                let (k, v) = (k.trim(), v.trim());
                match k {
                    "position" => {
                        if let Some(p) = DockPosition::parse(v) {
                            s.position = p;
                        }
                    }
                    "visualizer" => {
                        if let Some(b) = parse_bool(v) {
                            s.visualizer = b;
                        }
                    }
                    "hideWhenIdle" => {
                        if let Some(b) = parse_bool(v) {
                            s.hide_when_idle = b;
                        }
                    }
                    _ => {}
                }
            }
        }
        s
    }

    pub fn save(&self) -> Result<(), String> {
        let dir = settings_dir();
        let wdir = to_wide(&dir);
        unsafe { CreateDirectoryW(wdir.as_ptr(), null()) };

        let content = format!(
            "position={}\r\nvisualizer={}\r\nhideWhenIdle={}\r\n",
            self.position.as_str(),
            u8::from(self.visualizer),
            u8::from(self.hide_when_idle),
        );
        write_file(&settings_path(), content.as_bytes())
    }

    /// Prints lines matching `docs/specs/blade-settings.md`.
    pub fn format_blade_settings(&self) -> String {
        format!(
            "position\tchoice\t{}\tTaskbar position\tleft=Bottom-left\ttray=Left of system tray\r\n\
             visualizer\ttoggle\t{}\tShow live audio frequency meter\r\n\
             hideWhenIdle\ttoggle\t{}\tHide when nothing is playing\r\n",
            self.position.as_str(),
            u8::from(self.visualizer),
            u8::from(self.hide_when_idle),
        )
    }

    /// Validates all `key=value` pairs before mutating anything.
    pub fn apply_pairs(&self, pairs: &[String]) -> Result<Self, String> {
        let mut next = *self;
        for pair in pairs {
            let Some((k, v)) = pair.split_once('=') else {
                return Err(format!("Invalid setting: {pair}"));
            };
            match k {
                "position" => {
                    next.position = DockPosition::parse(v).ok_or_else(|| format!("Invalid taskbar position: {v}"))?;
                }
                "visualizer" => {
                    next.visualizer = parse_bool(v).ok_or_else(|| format!("Invalid value for visualizer: {v}"))?;
                }
                "hideWhenIdle" => {
                    next.hide_when_idle = parse_bool(v).ok_or_else(|| format!("Invalid value for hideWhenIdle: {v}"))?;
                }
                _ => return Err(format!("Unknown setting: {k}")),
            }
        }
        Ok(next)
    }
}

fn parse_bool(v: &str) -> Option<bool> {
    match v {
        "1" | "true" | "on" => Some(true),
        "0" | "false" | "off" => Some(false),
        _ => None,
    }
}

fn settings_dir() -> String {
    if pk::env::settings_preview() {
        return pk::env::exe_dir();
    }
    let appdata = pk::env::known_folder(&FOLDERID_RoamingAppData).unwrap_or_else(pk::env::exe_dir);
    format!("{appdata}\\NowPlaying")
}

fn settings_path() -> String {
    if pk::env::settings_preview() {
        return format!("{}\\nowplaying-preview.ini", pk::env::exe_dir());
    }
    format!("{}\\settings.ini", settings_dir())
}

fn read_file(path: &str) -> Option<Vec<u8>> {
    let w = to_wide(path);
    let h = unsafe { CreateFileW(w.as_ptr(), GENERIC_READ, FILE_SHARE_READ, null(), OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, null_mut()) };
    if h == INVALID_HANDLE_VALUE {
        return None;
    }
    let size = unsafe { GetFileSize(h, null_mut()) };
    if size == u32::MAX || size > 64 * 1024 {
        unsafe { CloseHandle(h) };
        return None;
    }
    let mut buf = vec![0u8; size as usize];
    let mut read = 0u32;
    let ok = unsafe { ReadFile(h, buf.as_mut_ptr(), size, &mut read, null_mut()) } != 0;
    unsafe { CloseHandle(h) };
    if ok {
        buf.truncate(read as usize);
        Some(buf)
    } else {
        None
    }
}

fn write_file(path: &str, bytes: &[u8]) -> Result<(), String> {
    let w = to_wide(path);
    let h = unsafe { CreateFileW(w.as_ptr(), GENERIC_WRITE, 0, null(), CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, null_mut()) };
    if h == INVALID_HANDLE_VALUE {
        return Err(String::from("Could not open settings file for writing."));
    }
    let mut written = 0u32;
    let ok = unsafe { WriteFile(h, bytes.as_ptr(), bytes.len() as u32, &mut written, null_mut()) } != 0;
    unsafe { CloseHandle(h) };
    if ok && written as usize == bytes.len() {
        Ok(())
    } else {
        Err(String::from("Could not write settings file."))
    }
}

// ---- Autostart & Theme ----

pub fn autostart_enabled() -> bool {
    reg_string(HKEY_CURRENT_USER, Some(RUN_KEY), RUN_VALUE).is_some_and(|v| !v.trim().is_empty())
}

pub fn set_autostart(on: bool) -> Result<(), String> {
    let (key, name) = (to_wide(RUN_KEY), to_wide(RUN_VALUE));
    if on {
        let exe = pk::env::exe_path();
        let val = to_wide(&format!("\"{exe}\""));
        let err = unsafe {
            RegSetKeyValueW(HKEY_CURRENT_USER, key.as_ptr(), name.as_ptr(), REG_SZ, val.as_ptr().cast(), (val.len() * 2) as u32)
        };
        if err != 0 {
            return Err(format!("Could not enable autostart (error {err})."));
        }
    } else {
        let err = unsafe { RegDeleteKeyValueW(HKEY_CURRENT_USER, key.as_ptr(), name.as_ptr()) };
        if err != 0 && err != ERROR_FILE_NOT_FOUND {
            return Err(format!("Could not disable autostart (error {err})."));
        }
    }
    Ok(())
}

pub fn taskbar_is_light() -> bool {
    reg_dword(HKEY_CURRENT_USER, Some("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize"), "SystemUsesLightTheme")
        .is_some_and(|v| v != 0)
}

/// Applies Windows 10/11 dark or light popup menu styling (uxtheme ordinals #135, #133, #136).
pub fn apply_menu_theme(hwnd: HWND, light: bool) {
    const FORCE_DARK: i32 = 2;
    const FORCE_LIGHT: i32 = 3;
    let lib = unsafe { LoadLibraryW(pk::w!("uxtheme.dll")) };
    if lib.is_null() {
        return;
    }
    unsafe {
        if let Some(f) = GetProcAddress(lib, 135 as *const u8) {
            core::mem::transmute::<_, unsafe extern "system" fn(i32) -> i32>(f)(if light { FORCE_LIGHT } else { FORCE_DARK });
        }
        if let Some(f) = GetProcAddress(lib, 133 as *const u8) {
            core::mem::transmute::<_, unsafe extern "system" fn(HWND, i32) -> i32>(f)(hwnd, (!light) as i32);
        }
        if let Some(f) = GetProcAddress(lib, 136 as *const u8) {
            core::mem::transmute::<_, unsafe extern "system" fn()>(f)();
        }
    }
}
