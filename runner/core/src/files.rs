//! The few file operations the settings need, with .NET's `File` semantics.
use crate::Result;
use alloc::string::String;
use alloc::vec::Vec;
use core::ptr::{null, null_mut};
use pk::wide::to_wide;
use windows_sys::Win32::Foundation::{
    CloseHandle, ERROR_FILE_NOT_FOUND, ERROR_PATH_NOT_FOUND, GENERIC_READ, GENERIC_WRITE, GetLastError, INVALID_HANDLE_VALUE,
};
use windows_sys::Win32::Storage::FileSystem::{
    CREATE_ALWAYS, CreateFileW, DeleteFileW, FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, GetFileAttributesW,
    INVALID_FILE_ATTRIBUTES, OPEN_EXISTING, ReadFile, WriteFile,
};

/// `File.Exists`: a file (not a folder) is there.
pub fn exists(path: &str) -> bool {
    let a = unsafe { GetFileAttributesW(to_wide(path).as_ptr()) };
    a != INVALID_FILE_ATTRIBUTES && a & FILE_ATTRIBUTE_DIRECTORY == 0
}

/// `File.Delete`: a missing file is fine.
pub fn delete(path: &str) -> Result<()> {
    if unsafe { DeleteFileW(to_wide(path).as_ptr()) } != 0 {
        return Ok(());
    }
    match unsafe { GetLastError() } {
        ERROR_FILE_NOT_FOUND => Ok(()),
        e => Err(pk::error::message(e)),
    }
}

/// `File.ReadAllLines`: `None` when the file isn't there.
pub fn read_lines(path: &str) -> Result<Option<Vec<String>>> {
    let h = unsafe {
        CreateFileW(to_wide(path).as_ptr(), GENERIC_READ, FILE_SHARE_READ, null(), OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, null_mut())
    };
    if h == INVALID_HANDLE_VALUE {
        return match unsafe { GetLastError() } {
            ERROR_FILE_NOT_FOUND | ERROR_PATH_NOT_FOUND => Ok(None),
            e => Err(pk::error::message(e)),
        };
    }
    let mut bytes = Vec::new();
    let mut buf = [0u8; 4096];
    let result = loop {
        let mut n = 0u32;
        if unsafe { ReadFile(h, buf.as_mut_ptr(), buf.len() as u32, &mut n, null_mut()) } == 0 {
            break Err(pk::error::last());
        }
        if n == 0 {
            break Ok(());
        }
        bytes.extend_from_slice(&buf[..n as usize]);
    };
    unsafe { CloseHandle(h) };
    result?;
    Ok(Some(split_lines(&decode(&bytes))))
}

/// `File.WriteAllLines`: UTF-8 without a BOM, each line ending in CRLF; replaces the file.
pub fn write_lines(path: &str, lines: &[String]) -> Result<()> {
    let mut text = String::new();
    for l in lines {
        text.push_str(l);
        text.push_str("\r\n");
    }
    let h = unsafe { CreateFileW(to_wide(path).as_ptr(), GENERIC_WRITE, 0, null(), CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, null_mut()) };
    if h == INVALID_HANDLE_VALUE {
        return Err(pk::error::last());
    }
    let mut done = 0;
    let bytes = text.as_bytes();
    let mut result = Ok(());
    while done < bytes.len() {
        let mut n = 0u32;
        if unsafe { WriteFile(h, bytes[done..].as_ptr(), (bytes.len() - done) as u32, &mut n, null_mut()) } == 0 {
            result = Err(pk::error::last());
            break;
        }
        done += n as usize;
    }
    unsafe { CloseHandle(h) };
    result
}

/// Text from a file: UTF-8, or UTF-16 when it starts with that byte-order mark (what .NET's
/// reader detects); a UTF-8 BOM is dropped.
fn decode(b: &[u8]) -> String {
    if let Some(rest) = b.strip_prefix(&[0xEF, 0xBB, 0xBF]) {
        return String::from_utf8_lossy(rest).into();
    }
    let utf16 = |rest: &[u8], be: bool| {
        let units: Vec<u16> =
            rest.chunks_exact(2).map(|c| if be { u16::from_be_bytes([c[0], c[1]]) } else { u16::from_le_bytes([c[0], c[1]]) }).collect();
        String::from_utf16_lossy(&units)
    };
    if let Some(rest) = b.strip_prefix(&[0xFF, 0xFE]) {
        return utf16(rest, false);
    }
    if let Some(rest) = b.strip_prefix(&[0xFE, 0xFF]) {
        return utf16(rest, true);
    }
    String::from_utf8_lossy(b).into()
}

/// Lines as .NET's `ReadLine` gives them: ended by CRLF, LF or CR; a final line ending adds no
/// empty line.
fn split_lines(text: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut cur = String::new();
    let mut chars = text.chars().peekable();
    let mut pending = false;
    while let Some(c) = chars.next() {
        match c {
            '\r' | '\n' => {
                if c == '\r' && chars.peek() == Some(&'\n') {
                    chars.next();
                }
                lines.push(core::mem::take(&mut cur));
                pending = false;
            }
            _ => {
                cur.push(c);
                pending = true;
            }
        }
    }
    if pending {
        lines.push(cur);
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn lines() {
        let s = |v: &[&str]| v.iter().map(|x| String::from(*x)).collect::<Vec<_>>();
        assert_eq!(split_lines(""), s(&[]));
        assert_eq!(split_lines("a"), s(&["a"]));
        assert_eq!(split_lines("a\r\n"), s(&["a"]));
        assert_eq!(split_lines("a\r\nb\nc\rd"), s(&["a", "b", "c", "d"]));
        assert_eq!(split_lines("\r\n\r\n"), s(&["", ""]));
        assert_eq!(split_lines("a\n\nb\n"), s(&["a", "", "b"]));
        assert_eq!(decode(&[0xEF, 0xBB, 0xBF, b'x']), "x");
        assert_eq!(decode(&[0xFF, 0xFE, b'x', 0]), "x");
    }

    #[test]
    fn round_trip() {
        let dir = std::env::temp_dir().join(std::format!("runner-core-files-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("lines.txt");
        let p = path.to_str().unwrap();
        assert_eq!(read_lines(p).unwrap(), None);
        assert!(!exists(p));
        write_lines(p, &vec![String::from("win11"), String::from("on"), String::from("off")]).unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), b"win11\r\non\r\noff\r\n");
        assert!(exists(p));
        assert!(!exists(dir.to_str().unwrap()));
        assert_eq!(read_lines(p).unwrap().unwrap(), vec!["win11", "on", "off"]);
        delete(p).unwrap();
        delete(p).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
