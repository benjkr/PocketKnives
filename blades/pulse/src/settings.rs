//! Settings (`%APPDATA%\Pulse\settings.ini`, or next to the exe in preview mode) and the user's
//! themes folder (`%APPDATA%\Pulse\themes`).
use crate::bar::Item;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::ptr::{null, null_mut};
use pk::wide::to_wide;
use windows_sys::Win32::Foundation::{CloseHandle, ERROR_FILE_NOT_FOUND, GENERIC_READ, GENERIC_WRITE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Registry::{HKEY_CURRENT_USER, REG_SZ, RegDeleteKeyValueW, RegSetKeyValueW};
use windows_sys::Win32::Storage::FileSystem::{
    CREATE_ALWAYS, CreateDirectoryW, CreateFileW, FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, FindClose,
    FindFirstFileW, FindNextFileW, GetFileSize, OPEN_EXISTING, ReadFile, WIN32_FIND_DATAW, WriteFile,
};
use windows_sys::Win32::UI::Shell::FOLDERID_RoamingAppData;

const RUN_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
const RUN_VALUE: &str = "Pulse";

/// The theme a new install starts with.
const DEFAULT_THEME: &str = "ivory";

/// The metrics in the order the bar shows them, with their setting keys and labels.
pub const METRICS: [(Item, &str, &str); 5] = [
    (Item::Cpu, "cpu", "Show CPU"),
    (Item::Mem, "memory", "Show memory"),
    (Item::Gpu, "gpu", "Show GPU"),
    (Item::Dsk, "disk", "Show disk"),
    (Item::Net, "network", "Show network"),
];

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Settings {
    pub theme: String,
    /// Per entry of [`METRICS`].
    pub show: [bool; 5],
}

impl Default for Settings {
    fn default() -> Self {
        Settings { theme: String::from(DEFAULT_THEME), show: [true; 5] }
    }
}

impl Settings {
    pub fn load() -> Settings {
        let mut s = Settings::default();
        let Some(bytes) = read_file(&settings_path()) else { return s };
        let Ok(text) = core::str::from_utf8(&bytes) else { return s };
        for line in text.lines() {
            if let Some((k, v)) = line.trim().split_once('=') {
                let _ = s.set(k.trim(), v.trim());
            }
        }
        s
    }

    pub fn save(&self) -> Result<(), String> {
        let dir = settings_dir();
        let wdir = to_wide(&dir);
        unsafe { CreateDirectoryW(wdir.as_ptr(), null()) };
        let mut text = format!("theme={}\r\n", self.theme);
        for (i, m) in METRICS.iter().enumerate() {
            text.push_str(&format!("{}={}\r\n", m.1, u8::from(self.show[i])));
        }
        write_file(&settings_path(), text.as_bytes())
    }

    /// The metrics shown, in bar order.
    pub fn items(&self) -> Vec<Item> {
        METRICS.iter().zip(self.show).filter(|(_, on)| *on).map(|(m, _)| m.0).collect()
    }

    /// Lines for PocketKnives Settings (`docs/specs/blade-settings.md`).
    pub fn format_blade_settings(&self) -> String {
        let mut out = format!("theme\tchoice\t{}\tTheme", self.theme);
        for name in crate::theme::names() {
            out.push_str(&format!("\t{name}={}", title(&name)));
        }
        out.push_str("\r\n");
        for (i, m) in METRICS.iter().enumerate() {
            out.push_str(&format!("{}\ttoggle\t{}\t{}\r\n", m.1, u8::from(self.show[i]), m.2));
        }
        out
    }

    /// Checks every `key=value` before changing anything.
    pub fn apply_pairs(&self, pairs: &[String]) -> Result<Settings, String> {
        let mut next = self.clone();
        for pair in pairs {
            let Some((k, v)) = pair.split_once('=') else { return Err(format!("Not a setting: {pair}")) };
            next.set(k, v)?;
        }
        Ok(next)
    }

    fn set(&mut self, k: &str, v: &str) -> Result<(), String> {
        if k == "theme" {
            crate::theme::load(v)?;
            self.theme = String::from(v);
            return Ok(());
        }
        let on = match v {
            "1" => true,
            "0" => false,
            _ => return Err(format!("{k} is 1 or 0, not {v}")),
        };
        if let Some(i) = METRICS.iter().position(|m| m.1 == k) {
            self.show[i] = on;
        } else {
            return Err(format!("Unknown setting: {k}"));
        }
        Ok(())
    }
}

/// A theme's file name as a label: the first letter upper case.
fn title(name: &str) -> String {
    let mut c = name.chars();
    c.next().map(|f| f.to_uppercase().chain(c).collect()).unwrap_or_default()
}

fn settings_dir() -> String {
    if pk::env::settings_preview() {
        return pk::env::exe_dir();
    }
    let appdata = pk::env::known_folder(&FOLDERID_RoamingAppData).unwrap_or_else(pk::env::exe_dir);
    format!("{appdata}\\Pulse")
}

fn settings_path() -> String {
    if pk::env::settings_preview() {
        return format!("{}\\pulse-preview.ini", pk::env::exe_dir());
    }
    format!("{}\\settings.ini", settings_dir())
}

/// Whether the bar starts when the user signs in (`HKCU\...\Run\Pulse`).
pub fn autostart_enabled() -> bool {
    pk::reg::string(HKEY_CURRENT_USER, Some(RUN_KEY), RUN_VALUE).is_some_and(|v| !v.trim().is_empty())
}

pub fn set_autostart(on: bool) -> Result<(), String> {
    let (key, name) = (to_wide(RUN_KEY), to_wide(RUN_VALUE));
    if on {
        let val = to_wide(&format!("\"{}\"", pk::env::exe_path()));
        let err = unsafe { RegSetKeyValueW(HKEY_CURRENT_USER, key.as_ptr(), name.as_ptr(), REG_SZ, val.as_ptr().cast(), (val.len() * 2) as u32) };
        if err != 0 {
            return Err(format!("Could not turn on starting at sign-in (error {err})."));
        }
    } else {
        let err = unsafe { RegDeleteKeyValueW(HKEY_CURRENT_USER, key.as_ptr(), name.as_ptr()) };
        if err != 0 && err != ERROR_FILE_NOT_FOUND {
            return Err(format!("Could not turn off starting at sign-in (error {err})."));
        }
    }
    Ok(())
}

/// The folder the user's own theme files go in.
pub fn themes_dir() -> Option<String> {
    Some(format!("{}\\themes", settings_dir()))
}

/// The names (without `ext`) of the files in `dir` ending in `ext`.
pub fn list_files(dir: &str, ext: &str) -> Vec<String> {
    let mut out = Vec::new();
    let pattern = to_wide(&format!("{dir}\\*{ext}"));
    let mut fd: WIN32_FIND_DATAW = unsafe { core::mem::zeroed() };
    let h = unsafe { FindFirstFileW(pattern.as_ptr(), &mut fd) };
    if h == INVALID_HANDLE_VALUE {
        return out;
    }
    loop {
        if fd.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY == 0 {
            let name = pk::wide::from_buf(&fd.cFileName);
            if let Some(stem) = name.strip_suffix(ext)
                && !stem.is_empty()
            {
                out.push(String::from(stem));
            }
        }
        if unsafe { FindNextFileW(h, &mut fd) } == 0 {
            break;
        }
    }
    unsafe { FindClose(h) };
    out
}

pub fn read_file(path: &str) -> Option<Vec<u8>> {
    let w = to_wide(path);
    let h = unsafe { CreateFileW(w.as_ptr(), GENERIC_READ, FILE_SHARE_READ, null(), OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, null_mut()) };
    if h == INVALID_HANDLE_VALUE {
        return None;
    }
    let size = unsafe { GetFileSize(h, null_mut()) };
    if size == u32::MAX || size > 256 * 1024 {
        unsafe { CloseHandle(h) };
        return None;
    }
    let mut buf = vec![0u8; size as usize];
    let mut read = 0u32;
    let ok = unsafe { ReadFile(h, buf.as_mut_ptr(), size, &mut read, null_mut()) } != 0;
    unsafe { CloseHandle(h) };
    ok.then(|| {
        buf.truncate(read as usize);
        buf
    })
}

pub fn write_file(path: &str, bytes: &[u8]) -> Result<(), String> {
    let w = to_wide(path);
    let h = unsafe { CreateFileW(w.as_ptr(), GENERIC_WRITE, 0, null(), CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, null_mut()) };
    if h == INVALID_HANDLE_VALUE {
        return Err(format!("Could not write {path}."));
    }
    let mut written = 0u32;
    let ok = unsafe { WriteFile(h, bytes.as_ptr(), bytes.len() as u32, &mut written, null_mut()) } != 0;
    unsafe { CloseHandle(h) };
    if ok && written as usize == bytes.len() { Ok(()) } else { Err(format!("Could not write {path}.")) }
}
