//! The registry: open keys, read string and DWORD values, write strings.
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::ptr::{null, null_mut};
use windows_sys::Win32::Foundation::{ERROR_MORE_DATA, ERROR_SUCCESS};
use windows_sys::Win32::System::Registry::{
    HKEY, KEY_SET_VALUE, REG_OPTION_NON_VOLATILE, REG_SZ, RRF_RT_REG_DWORD, RRF_RT_REG_SZ, RegCloseKey, RegCreateKeyExW, RegGetValueW,
    RegOpenKeyExW, RegSetValueExW,
};

use crate::wide::to_wide;

/// An open key, closed when dropped.
pub struct Key(pub HKEY);

impl Key {
    /// Opens an existing key; the error is the Win32 code (`ERROR_FILE_NOT_FOUND` when missing).
    pub fn open(parent: HKEY, subkey: &str, access: u32) -> Result<Key, u32> {
        let w = to_wide(subkey);
        let mut k: HKEY = null_mut();
        let r = unsafe { RegOpenKeyExW(parent, w.as_ptr(), 0, access, &mut k) };
        if r == ERROR_SUCCESS { Ok(Key(k)) } else { Err(r) }
    }

    /// Opens a key for writing values, creating it (and its parents) when missing.
    pub fn create(parent: HKEY, subkey: &str) -> Result<Key, u32> {
        let w = to_wide(subkey);
        let mut k: HKEY = null_mut();
        let r =
            unsafe { RegCreateKeyExW(parent, w.as_ptr(), 0, null(), REG_OPTION_NON_VOLATILE, KEY_SET_VALUE, null(), &mut k, null_mut()) };
        if r == ERROR_SUCCESS { Ok(Key(k)) } else { Err(r) }
    }

    /// Sets a REG_SZ value.
    pub fn set_string(&self, name: &str, value: &str) -> Result<(), u32> {
        let name = to_wide(name);
        let value = to_wide(value);
        let r = unsafe { RegSetValueExW(self.0, name.as_ptr(), 0, REG_SZ, value.as_ptr().cast(), (value.len() * 2) as u32) };
        if r == ERROR_SUCCESS { Ok(()) } else { Err(r) }
    }
}

impl Drop for Key {
    fn drop(&mut self) {
        unsafe { RegCloseKey(self.0) };
    }
}

/// A string value (REG_SZ, or REG_EXPAND_SZ expanded, as .NET's `GetValue` gives), `None` if the
/// key or value is absent or another type.
pub fn string(key: HKEY, subkey: Option<&str>, name: &str) -> Option<String> {
    let sub = subkey.map(to_wide);
    let name = to_wide(name);
    let sub_ptr = sub.as_ref().map_or(null(), |s| s.as_ptr());
    let mut buf: Vec<u16> = vec![0; 260];
    loop {
        let mut size = (buf.len() * 2) as u32;
        let r = unsafe { RegGetValueW(key, sub_ptr, name.as_ptr(), RRF_RT_REG_SZ, null_mut(), buf.as_mut_ptr().cast(), &mut size) };
        if r == ERROR_MORE_DATA {
            buf.resize(size as usize / 2 + 1, 0);
            continue;
        }
        return (r == ERROR_SUCCESS).then(|| crate::wide::from_buf(&buf[..size as usize / 2]));
    }
}

/// A REG_DWORD value, `None` if the key or value is absent or another type.
pub fn dword(key: HKEY, subkey: Option<&str>, name: &str) -> Option<u32> {
    let sub = subkey.map(to_wide);
    let name = to_wide(name);
    let mut v = 0u32;
    let mut size = 4u32;
    let r = unsafe {
        RegGetValueW(
            key,
            sub.as_ref().map_or(null(), |s| s.as_ptr()),
            name.as_ptr(),
            RRF_RT_REG_DWORD,
            null_mut(),
            (&mut v as *mut u32).cast(),
            &mut size,
        )
    };
    (r == ERROR_SUCCESS).then_some(v)
}
