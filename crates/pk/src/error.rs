//! Windows' own text for an error code.
use alloc::format;
use alloc::string::String;
use core::ptr::null;
use windows_sys::Win32::Foundation::GetLastError;
use windows_sys::Win32::System::Diagnostics::Debug::{FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS, FormatMessageW};

/// The system's text for the calling thread's last error.
pub fn last() -> String {
    message(unsafe { GetLastError() })
}

/// The system's text for a Win32 error code or an HRESULT, without the trailing line break.
pub fn message(code: u32) -> String {
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
