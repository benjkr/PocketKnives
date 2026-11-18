//! QuickTooth's files: the folder they live in, `quicktooth.log` (the core's log sink) and
//! `settings.json` (the core's settings saver), plus the small file helpers they need
//! (docs/specs/settings-and-cli.md).
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::ptr::{null, null_mut};
use pk::sync::Mutex;
use pk::wide::to_wide;
use quicktooth_core::settings::Settings;
use windows_sys::Win32::Foundation::{CloseHandle, GENERIC_READ, GENERIC_WRITE, GetLastError, INVALID_HANDLE_VALUE, SYSTEMTIME};
use windows_sys::Win32::Storage::FileSystem::{
    CREATE_ALWAYS, CreateDirectoryW, CreateFileW, FILE_APPEND_DATA, FILE_ATTRIBUTE_DIRECTORY, FILE_SHARE_DELETE, FILE_SHARE_READ,
    FILE_SHARE_WRITE, GetFileAttributesExW, GetFileExInfoStandard, INVALID_FILE_ATTRIBUTES, MOVEFILE_COPY_ALLOWED,
    MOVEFILE_REPLACE_EXISTING, MoveFileExW, OPEN_ALWAYS, OPEN_EXISTING, ReadFile, WIN32_FILE_ATTRIBUTE_DATA, WriteFile,
};
use windows_sys::Win32::System::Com::CoTaskMemFree;
use windows_sys::Win32::System::Diagnostics::Debug::{FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS, FormatMessageW};
use windows_sys::Win32::System::SystemInformation::GetLocalTime;
use windows_sys::Win32::System::Threading::GetCurrentProcessId;
use windows_sys::Win32::UI::Shell::{FOLDERID_RoamingAppData, SHGetKnownFolderPath};

/// The log is moved to `.old` once it's above this size.
const LOG_ROLL_BYTES: u64 = 512 * 1024;

/// The folder of `settings.json` and the log; `None` until first asked (or set for a preview).
static DIR: Mutex<Option<String>> = Mutex::new(None);
/// Serialises log writes from the UI, device and BLE threads.
static LOG_GATE: Mutex<()> = Mutex::new(());

/// `%APPDATA%\QuickTooth`, found as .NET's `Environment.GetFolderPath(ApplicationData)` did
/// (the Roaming AppData known folder, which follows the user profile's environment).
fn default_dir() -> String {
    let mut p = null_mut();
    let hr = unsafe { SHGetKnownFolderPath(&FOLDERID_RoamingAppData, 0, null_mut(), &mut p) };
    let base = if hr == 0 { unsafe { pk::wide::from_ptr(p) } } else { String::new() };
    unsafe { CoTaskMemFree(p as *const _) };
    join(&base, "QuickTooth")
}

pub fn dir() -> String {
    let mut d = DIR.lock();
    d.get_or_insert_with(default_dir).clone()
}

/// A PocketKnives preview (`POCKETKNIVES_SETTINGS_PREVIEW=1`): settings and log next to the exe.
pub fn use_exe_dir() {
    *DIR.lock() = Some(pk::env::exe_dir());
}

pub fn settings_path() -> String {
    join(&dir(), "settings.json")
}

fn log_path() -> String {
    join(&dir(), "quicktooth.log")
}

/// `Path.Combine(a, b)` for a relative `b`.
pub fn join(a: &str, b: &str) -> String {
    if a.is_empty() {
        return b.into();
    }
    if a.ends_with('\\') || a.ends_with('/') { format!("{a}{b}") } else { format!("{a}\\{b}") }
}

// ---------------------------------------------------------------------------- the log

/// The core's log sink: `yyyy-MM-dd HH:mm:ss.fff [pid] message`, local time, CRLF, appended to
/// `quicktooth.log` (moved to `.old` above 512 KB). Logging never fails the app.
pub fn log_sink(message: &str) {
    let t = local_time();
    let line = format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03} [{}] {message}\r\n",
        t.wYear,
        t.wMonth,
        t.wDay,
        t.wHour,
        t.wMinute,
        t.wSecond,
        t.wMilliseconds,
        unsafe { GetCurrentProcessId() }
    );
    let _gate = LOG_GATE.lock();
    append_log(&line);
}

/// The panic hook: one `ERROR panic` line before the process ends (where it happened would cost
/// 14 KB of exe; see `pk::panic`). It doesn't wait for the log's lock: the panic may have happened
/// while this thread held it.
pub fn log_panic() {
    let t = local_time();
    let line = format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03} [{}] ERROR panic: QuickTooth stopped on a bug\r\n",
        t.wYear,
        t.wMonth,
        t.wDay,
        t.wHour,
        t.wMinute,
        t.wSecond,
        t.wMilliseconds,
        unsafe { GetCurrentProcessId() }
    );
    let _gate = LOG_GATE.try_lock();
    append_log(&line);
}

fn append_log(line: &str) {
    let dir = dir();
    create_dir_all(&dir);
    let path = log_path();
    if file_size(&path).is_some_and(|n| n > LOG_ROLL_BYTES) {
        let old = format!("{path}.old");
        let (w, o) = (to_wide(&path), to_wide(&old));
        unsafe { MoveFileExW(w.as_ptr(), o.as_ptr(), MOVEFILE_REPLACE_EXISTING | MOVEFILE_COPY_ALLOWED) };
    }
    let w = to_wide(&path);
    let h = unsafe { CreateFileW(w.as_ptr(), FILE_APPEND_DATA, FILE_SHARE_READ | FILE_SHARE_WRITE, null(), OPEN_ALWAYS, 0, null_mut()) };
    if h != INVALID_HANDLE_VALUE {
        write_all(h, line.as_bytes());
        unsafe { CloseHandle(h) };
    }
}

pub fn local_time() -> SYSTEMTIME {
    let mut t: SYSTEMTIME = unsafe { core::mem::zeroed() };
    unsafe { GetLocalTime(&mut t) };
    t
}

// ---------------------------------------------------------------------------- settings.json

/// Reads settings from `path` (the real file, or a copy for `--settings-roundtrip`). A missing
/// file gives the defaults; one that can't be read or parsed logs `ERROR load settings: …` and
/// gives the defaults.
pub fn load_settings_from(path: &str) -> Settings {
    let text = match read_file(path) {
        Ok(None) => return Settings::default(),
        Ok(Some(bytes)) => String::from_utf8_lossy(&bytes).into_owned(),
        Err(e) => {
            quicktooth_core::log::error("load settings", &e);
            return Settings::default();
        }
    };
    Settings::from_json(&text).unwrap_or_else(|e| {
        quicktooth_core::log::error("load settings", &e);
        Settings::default()
    })
}

pub fn load_settings() -> Settings {
    load_settings_from(&settings_path())
}

/// The core's settings saver: through a temp file and a move. False if it couldn't be written.
pub fn save_settings(s: &Settings) -> bool {
    let dir = dir();
    create_dir_all(&dir);
    let path = settings_path();
    let tmp = format!("{path}.tmp");
    let result = write_file(&tmp, s.to_json("\r\n").as_bytes()).and_then(|()| {
        let (t, p) = (to_wide(&tmp), to_wide(&path));
        if unsafe { MoveFileExW(t.as_ptr(), p.as_ptr(), MOVEFILE_REPLACE_EXISTING | MOVEFILE_COPY_ALLOWED) } == 0 {
            Err(last_error_message())
        } else {
            Ok(())
        }
    });
    match result {
        Ok(()) => true,
        Err(e) => {
            quicktooth_core::log::error("save settings", &e);
            false
        }
    }
}

// ---------------------------------------------------------------------------- helpers

/// Creates `dir` and any missing parents (`Directory.CreateDirectory`); errors are ignored.
pub fn create_dir_all(dir: &str) {
    for (i, c) in dir.char_indices() {
        if (c == '\\' || c == '/') && i > 0 {
            let w = to_wide(&dir[..i]);
            unsafe { CreateDirectoryW(w.as_ptr(), null()) };
        }
    }
    let w = to_wide(dir);
    unsafe { CreateDirectoryW(w.as_ptr(), null()) };
}

/// The size of the file at `path`, `None` if there's no such file.
fn file_size(path: &str) -> Option<u64> {
    let w = to_wide(path);
    let mut data: WIN32_FILE_ATTRIBUTE_DATA = unsafe { core::mem::zeroed() };
    if unsafe { GetFileAttributesExW(w.as_ptr(), GetFileExInfoStandard, &mut data as *mut _ as *mut _) } == 0
        || data.dwFileAttributes == INVALID_FILE_ATTRIBUTES
        || data.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY != 0
    {
        return None;
    }
    Some(((data.nFileSizeHigh as u64) << 32) | data.nFileSizeLow as u64)
}

/// The whole file, `Ok(None)` if there's no file at `path` (`File.Exists` false).
pub fn read_file(path: &str) -> Result<Option<Vec<u8>>, String> {
    let Some(size) = file_size(path) else { return Ok(None) };
    let w = to_wide(path);
    let h = unsafe {
        CreateFileW(w.as_ptr(), GENERIC_READ, FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE, null(), OPEN_EXISTING, 0, null_mut())
    };
    if h == INVALID_HANDLE_VALUE {
        return Err(last_error_message());
    }
    let mut buf = vec![0u8; size as usize];
    let mut done = 0usize;
    let mut result = Ok(());
    while done < buf.len() {
        let mut n = 0u32;
        let chunk = (buf.len() - done).min(1 << 30) as u32;
        if unsafe { ReadFile(h, buf[done..].as_mut_ptr(), chunk, &mut n, null_mut()) } == 0 {
            result = Err(last_error_message());
            break;
        }
        if n == 0 {
            break;
        }
        done += n as usize;
    }
    unsafe { CloseHandle(h) };
    buf.truncate(done);
    result.map(|()| Some(buf))
}

/// Writes `bytes` as the whole file (`File.WriteAllText`, UTF-8 without a BOM).
pub fn write_file(path: &str, bytes: &[u8]) -> Result<(), String> {
    let w = to_wide(path);
    let h = unsafe { CreateFileW(w.as_ptr(), GENERIC_WRITE, FILE_SHARE_READ, null(), CREATE_ALWAYS, 0, null_mut()) };
    if h == INVALID_HANDLE_VALUE {
        return Err(last_error_message());
    }
    let ok = write_all(h, bytes);
    let err = if ok { None } else { Some(last_error_message()) };
    unsafe { CloseHandle(h) };
    err.map_or(Ok(()), Err)
}

fn write_all(h: windows_sys::Win32::Foundation::HANDLE, bytes: &[u8]) -> bool {
    let mut done = 0;
    while done < bytes.len() {
        let mut n = 0u32;
        let chunk = (bytes.len() - done).min(1 << 30) as u32;
        if unsafe { WriteFile(h, bytes[done..].as_ptr(), chunk, &mut n, null_mut()) } == 0 || n == 0 {
            return false;
        }
        done += n as usize;
    }
    true
}

/// The system's text for the calling thread's last error.
pub fn last_error_message() -> String {
    error_message(unsafe { GetLastError() })
}

/// The system's text for a Win32 error code or an HRESULT, without the trailing line break.
pub fn error_message(code: u32) -> String {
    let mut buf = [0u16; 512];
    let n = unsafe {
        FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            null(),
            code,
            0,
            buf.as_mut_ptr(),
            buf.len() as u32,
            null(),
        )
    } as usize;
    let text = String::from_utf16_lossy(&buf[..n]);
    let text = text.trim_end();
    if text.is_empty() { format!("Error 0x{code:08X}") } else { text.into() }
}
