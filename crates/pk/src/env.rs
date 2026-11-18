//! The process's environment and its own exe.
use alloc::string::String;
use alloc::vec;
use windows_sys::Win32::System::Environment::GetEnvironmentVariableW;
use windows_sys::Win32::System::LibraryLoader::GetModuleFileNameW;

/// An environment variable's value, or `None` when it isn't set.
pub fn var(name: &str) -> Option<String> {
    let name = crate::wide::to_wide(name);
    let mut buf = vec![0u16; 256];
    loop {
        let n = unsafe { GetEnvironmentVariableW(name.as_ptr(), buf.as_mut_ptr(), buf.len() as u32) } as usize;
        if n == 0 {
            return None;
        }
        if n < buf.len() {
            return Some(String::from_utf16_lossy(&buf[..n]));
        }
        buf.resize(n, 0); // n is the size needed, zero included
    }
}

/// `POCKETKNIVES_SETTINGS_PREVIEW=1`: a blade's settings commands use the settings file next to
/// its exe and never message a running copy (docs/specs/blade-settings.md).
pub fn settings_preview() -> bool {
    var("POCKETKNIVES_SETTINGS_PREVIEW").as_deref() == Some("1")
}

/// The full path of this exe.
pub fn exe_path() -> String {
    let mut buf = vec![0u16; 260];
    loop {
        let n = unsafe { GetModuleFileNameW(core::ptr::null_mut(), buf.as_mut_ptr(), buf.len() as u32) } as usize;
        if n < buf.len() {
            return String::from_utf16_lossy(&buf[..n]);
        }
        let more = buf.len() * 2;
        buf.resize(more, 0);
    }
}

/// The folder this exe is in, without a trailing backslash.
pub fn exe_dir() -> String {
    let mut p = exe_path();
    if let Some(i) = p.rfind('\\') {
        p.truncate(i);
    }
    p
}

/// A known folder's path (`FOLDERID_*`), as .NET's `Environment.GetFolderPath` finds it.
pub fn known_folder(id: &windows_sys::core::GUID) -> Option<String> {
    let mut p = core::ptr::null_mut();
    let hr = unsafe { windows_sys::Win32::UI::Shell::SHGetKnownFolderPath(id, 0, core::ptr::null_mut(), &mut p) };
    let path = (hr == 0).then(|| unsafe { crate::wide::from_ptr(p) });
    unsafe { windows_sys::Win32::System::Com::CoTaskMemFree(p as *const _) };
    path
}

/// The current directory, without a trailing backslash (except at a drive's root).
pub fn current_dir() -> String {
    let mut buf = vec![0u16; 260];
    loop {
        let n = unsafe { windows_sys::Win32::System::Environment::GetCurrentDirectoryW(buf.len() as u32, buf.as_mut_ptr()) } as usize;
        if n < buf.len() {
            return String::from_utf16_lossy(&buf[..n]);
        }
        buf.resize(n, 0);
    }
}
