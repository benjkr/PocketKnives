//! Text output for a GUI-subsystem exe: to its stdout when that's a file or a pipe (a launcher
//! reading it), else to the console of the terminal that started it. Each blade decides which,
//! with these pieces.
use core::fmt;
use core::ptr::null_mut;
use windows_sys::Win32::Foundation::{GENERIC_READ, GENERIC_WRITE, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Storage::FileSystem::{
    CreateFileW, FILE_SHARE_READ, FILE_SHARE_WRITE, FILE_TYPE_DISK, FILE_TYPE_PIPE, FILE_TYPE_UNKNOWN, GetFileType, OPEN_EXISTING,
    WriteFile,
};
use windows_sys::Win32::System::Console::{
    ATTACH_PARENT_PROCESS, AttachConsole, GetConsoleMode, GetStdHandle, STD_OUTPUT_HANDLE, WriteConsoleW,
};

/// What the process's stdout handle is.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StdoutKind {
    /// No handle, or one Windows can't type: write nowhere unless a console is attached.
    None,
    /// A file or a pipe: a launcher or a script is reading.
    FileOrPipe,
    /// A character device (a console, `NUL`).
    Char,
}

pub fn stdout_handle() -> HANDLE {
    unsafe { GetStdHandle(STD_OUTPUT_HANDLE) }
}

pub fn stdout_kind() -> StdoutKind {
    let h = stdout_handle();
    if h.is_null() || h == INVALID_HANDLE_VALUE {
        return StdoutKind::None;
    }
    match unsafe { GetFileType(h) } {
        FILE_TYPE_DISK | FILE_TYPE_PIPE => StdoutKind::FileOrPipe,
        FILE_TYPE_UNKNOWN => StdoutKind::None,
        _ => StdoutKind::Char,
    }
}

/// Attaches to the console of the process that started this one (a terminal), if it has one.
pub fn attach_parent_console() -> bool {
    unsafe { AttachConsole(ATTACH_PARENT_PROCESS) != 0 }
}

/// The attached console's output (`CONOUT$`), after [`attach_parent_console`].
pub fn console_output() -> Option<Writer> {
    let h = unsafe {
        CreateFileW(
            crate::w!("CONOUT$"),
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            core::ptr::null(),
            OPEN_EXISTING,
            0,
            null_mut(),
        )
    };
    if h == INVALID_HANDLE_VALUE { None } else { Some(Writer::new(h)) }
}

/// Writes text to a handle: UTF-16 through `WriteConsoleW` to a console (so any character shows,
/// whatever the code page), UTF-8 bytes through `WriteFile` to anything else.
pub struct Writer {
    handle: HANDLE,
    console: bool,
}

impl Writer {
    pub fn new(handle: HANDLE) -> Self {
        let mut mode = 0;
        let console = unsafe { GetConsoleMode(handle, &mut mode) } != 0;
        Writer { handle, console }
    }

    /// A writer that drops everything (no stdout and no console).
    pub const fn null() -> Self {
        Writer { handle: null_mut(), console: false }
    }

    pub fn is_null(&self) -> bool {
        self.handle.is_null()
    }

    pub fn handle(&self) -> HANDLE {
        self.handle
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        if self.handle.is_null() {
            return;
        }
        if self.console {
            let mut buf = [0u16; 256];
            let text = core::str::from_utf8(bytes).unwrap_or("");
            let mut n = 0;
            for unit in text.encode_utf16() {
                buf[n] = unit;
                n += 1;
                if n == buf.len() {
                    self.write_console(&buf[..n]);
                    n = 0;
                }
            }
            self.write_console(&buf[..n]);
        } else {
            let mut done = 0;
            while done < bytes.len() {
                let mut written = 0u32;
                let chunk = (bytes.len() - done).min(u32::MAX as usize) as u32;
                if unsafe { WriteFile(self.handle, bytes[done..].as_ptr(), chunk, &mut written, null_mut()) } == 0 || written == 0 {
                    return;
                }
                done += written as usize;
            }
        }
    }

    fn write_console(&mut self, units: &[u16]) {
        if !units.is_empty() {
            let mut written = 0u32;
            unsafe { WriteConsoleW(self.handle, units.as_ptr().cast(), units.len() as u32, &mut written, null_mut()) };
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_bytes(s.as_bytes());
        Ok(())
    }
}
