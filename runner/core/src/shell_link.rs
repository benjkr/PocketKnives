//! A shortcut file, through the two methods it needs of `IShellLinkW` and `IPersistFile`.
use crate::Result;
use core::ffi::c_void;
use core::ptr::null_mut;
use pk::wide::to_wide;
use windows_sys::Win32::Foundation::RPC_E_CHANGED_MODE;
use windows_sys::Win32::System::Com::{CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx, CoUninitialize};
use windows_sys::core::{GUID, HRESULT};

const CLSID_SHELL_LINK: GUID = GUID::from_u128(0x00021401_0000_0000_c000_000000000046);
const IID_SHELL_LINK_W: GUID = GUID::from_u128(0x000214f9_0000_0000_c000_000000000046);
const IID_PERSIST_FILE: GUID = GUID::from_u128(0x0000010b_0000_0000_c000_000000000046);

/// Vtable slots: IUnknown's QueryInterface 0 and Release 2, IShellLinkW::SetPath 20,
/// IPersistFile::Save 6.
const QUERY_INTERFACE: usize = 0;
const RELEASE: usize = 2;
const SET_PATH: usize = 20;
const SAVE: usize = 6;

/// Writes a shortcut at `path` that starts `target`.
pub fn save(target: &str, path: &str) -> Result<()> {
    let apartment = unsafe { CoInitializeEx(null_mut(), COINIT_APARTMENTTHREADED as u32) };
    // COM may already be initialised in another apartment model (the Windows 11 app): keep it.
    if apartment != RPC_E_CHANGED_MODE {
        check(apartment)?;
    }
    let mut link: *mut c_void = null_mut();
    let mut file: *mut c_void = null_mut();
    let result = (|| {
        check(unsafe { CoCreateInstance(&CLSID_SHELL_LINK, null_mut(), CLSCTX_INPROC_SERVER, &IID_SHELL_LINK_W, &mut link) })?;
        let target = to_wide(target);
        let set_path: unsafe extern "system" fn(*mut c_void, *const u16) -> HRESULT = unsafe { method(link, SET_PATH) };
        check(unsafe { set_path(link, target.as_ptr()) })?;
        let query: unsafe extern "system" fn(*mut c_void, *const GUID, *mut *mut c_void) -> HRESULT =
            unsafe { method(link, QUERY_INTERFACE) };
        check(unsafe { query(link, &IID_PERSIST_FILE, &mut file) })?;
        let path = to_wide(path);
        let save: unsafe extern "system" fn(*mut c_void, *const u16, i32) -> HRESULT = unsafe { method(file, SAVE) };
        check(unsafe { save(file, path.as_ptr(), 1) })
    })();
    release(file);
    release(link);
    if apartment >= 0 {
        unsafe { CoUninitialize() };
    }
    result
}

/// The function in vtable slot `slot` of a COM object.
///
/// # Safety
/// `object` is a live COM object whose vtable has `slot` with the signature `F`.
unsafe fn method<F: Copy>(object: *mut c_void, slot: usize) -> F {
    unsafe {
        let vtable = *(object as *const *const usize);
        core::mem::transmute_copy(&*vtable.add(slot))
    }
}

fn release(object: *mut c_void) {
    if !object.is_null() {
        let f: unsafe extern "system" fn(*mut c_void) -> u32 = unsafe { method(object, RELEASE) };
        unsafe { f(object) };
    }
}

fn check(hr: HRESULT) -> Result<()> {
    if hr >= 0 { Ok(()) } else { Err(pk::error::message(hr as u32)) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn writes_a_shortcut() {
        let dir = std::env::temp_dir().join(std::format!("runner-core-lnk-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let lnk = dir.join("cmd.lnk");
        super::save("C:\\Windows\\System32\\cmd.exe", lnk.to_str().unwrap()).unwrap();
        let bytes = std::fs::read(&lnk).unwrap();
        // The shell link header: size 0x4C, then CLSID 00021401-0000-0000-C000-000000000046.
        assert_eq!(&bytes[..8], &[0x4C, 0, 0, 0, 0x01, 0x14, 0x02, 0x00]);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
