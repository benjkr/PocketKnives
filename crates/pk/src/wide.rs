//! UTF-16 (Windows' strings) to and from Rust strings.
use alloc::string::String;
use alloc::vec::Vec;

/// `s` as UTF-16 with a terminating zero, for passing to a `PCWSTR` parameter.
pub fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(core::iter::once(0)).collect()
}

/// The units of a zero-terminated UTF-16 string, without the zero. Null gives an empty slice.
///
/// # Safety
/// `p` is null or points to a zero-terminated string that stays valid for `'a`.
pub unsafe fn slice_from_ptr<'a>(p: *const u16) -> &'a [u16] {
    if p.is_null() {
        return &[];
    }
    let mut n = 0;
    while unsafe { *p.add(n) } != 0 {
        n += 1;
    }
    unsafe { core::slice::from_raw_parts(p, n) }
}

/// A zero-terminated UTF-16 string as a `String` (unpaired surrogates become U+FFFD).
///
/// # Safety
/// As [`slice_from_ptr`].
pub unsafe fn from_ptr(p: *const u16) -> String {
    String::from_utf16_lossy(unsafe { slice_from_ptr(p) })
}

/// A UTF-16 buffer up to its first zero (or its end) as a `String`.
pub fn from_buf(buf: &[u16]) -> String {
    let n = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..n])
}
