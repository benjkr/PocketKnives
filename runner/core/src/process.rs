//! Starting a blade's exe, as .NET's `Process.Start` with `UseShellExecute = false` and
//! `CreateNoWindow = true` did: the command line is `"<exe>" <args>`, each argument quoted so the
//! blade's parser (the C runtime's rules, `pk::args`) gets it back unchanged.
use crate::Result;
use alloc::format;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::ffi::c_void;
use core::mem::zeroed;
use core::ptr::{null, null_mut};
use pk::sync::Mutex;
use pk::wide::to_wide;
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, HANDLE_FLAG_INHERIT, SetHandleInformation, TRUE, WAIT_OBJECT_0};
use windows_sys::Win32::Security::SECURITY_ATTRIBUTES;
use windows_sys::Win32::Storage::FileSystem::ReadFile;
use windows_sys::Win32::System::Console::{GetStdHandle, STD_ERROR_HANDLE, STD_INPUT_HANDLE};
use windows_sys::Win32::System::Pipes::CreatePipe;
use windows_sys::Win32::System::Threading::{
    CREATE_NO_WINDOW, CREATE_UNICODE_ENVIRONMENT, CreateProcessW, GetExitCodeProcess, PROCESS_INFORMATION, STARTF_USESTDHANDLES,
    STARTUPINFOW, TerminateProcess, WaitForSingleObject,
};

/// A started process, its handle closed when dropped (the process keeps running).
pub struct Child {
    process: HANDLE,
    pub id: u32,
}

impl Child {
    /// Waits up to `ms` for it to end: its exit code, or `None` if it's still running.
    pub fn wait(&self, ms: u32) -> Option<u32> {
        if unsafe { WaitForSingleObject(self.process, ms) } != WAIT_OBJECT_0 {
            return None;
        }
        let mut code = 0u32;
        (unsafe { GetExitCodeProcess(self.process, &mut code) } != 0).then_some(code)
    }

    /// Ends it now (`Process.Kill`).
    pub fn kill(&self) {
        unsafe { TerminateProcess(self.process, u32::MAX) };
    }

    /// The process handle, e.g. to wait on with other handles. Valid while `self` lives.
    pub fn handle(&self) -> HANDLE {
        self.process
    }
}

impl Drop for Child {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.process) };
    }
}

/// Starts `exe` with `args`, no window, sharing this process's standard handles. The error is
/// Windows' reason.
pub fn start(exe: &str, args: &[&str]) -> core::result::Result<Child, String> {
    spawn(exe, args, None, None)
}

/// As [`start`], in the working directory `dir` (the Windows 11 settings app, which runs from its
/// own folder).
pub fn start_in(exe: &str, args: &[&str], dir: &str) -> core::result::Result<Child, String> {
    spawn(exe, args, None, Some(dir))
}

/// Runs `exe` with `args` and collects its standard output (UTF-8). Waits up to `timeout_ms`;
/// after that the process is ended and the error says it `did not answer`.
pub fn run(exe: &str, args: &[&str], timeout_ms: u32) -> Result<(u32, String)> {
    let mut sa: SECURITY_ATTRIBUTES = unsafe { zeroed() };
    sa.nLength = size_of::<SECURITY_ATTRIBUTES>() as u32;
    sa.bInheritHandle = TRUE;
    let (mut read, mut write): (HANDLE, HANDLE) = (null_mut(), null_mut());
    if unsafe { CreatePipe(&mut read, &mut write, &sa, 0) } == 0 {
        return Err(format!("Could not start {}.", file_name(exe)));
    }
    // Only the child's end is inherited.
    unsafe { SetHandleInformation(read, HANDLE_FLAG_INHERIT, 0) };
    let child = spawn(exe, args, Some(write), None);
    // Our copy of the write end must go, or the read never sees the end of the output.
    unsafe { CloseHandle(write) };
    let child = match child {
        Ok(c) => c,
        Err(_) => {
            unsafe { CloseHandle(read) };
            return Err(format!("Could not start {}.", file_name(exe)));
        }
    };
    // Read while waiting: a blade that fills the pipe would otherwise never end.
    let reader = Pipe(read as usize);
    let output = Arc::new(Mutex::new(Vec::new()));
    let sink = output.clone();
    let Some(thread) = pk::thread::spawn(move || *sink.lock() = reader.read_to_end()) else {
        child.kill();
        return Err(format!("Could not start {}.", file_name(exe)));
    };
    let code = child.wait(timeout_ms);
    if code.is_none() {
        child.kill();
    }
    // The pipe ends with the process (the blades start nothing that inherits it).
    thread.join();
    let bytes = core::mem::take(&mut *output.lock());
    match code {
        Some(code) => Ok((code, decode_utf8(&bytes))),
        None => Err(format!("{} did not answer.", file_stem(exe))),
    }
}

/// The read end of the output pipe, closed when dropped.
struct Pipe(usize);

impl Pipe {
    fn read_to_end(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let mut buf = [0u8; 4096];
        loop {
            let mut n = 0u32;
            if unsafe { ReadFile(self.0 as HANDLE, buf.as_mut_ptr(), buf.len() as u32, &mut n, null_mut()) } == 0 || n == 0 {
                break;
            }
            out.extend_from_slice(&buf[..n as usize]);
        }
        out
    }
}

impl Drop for Pipe {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.0 as HANDLE) };
    }
}

fn spawn(exe: &str, args: &[&str], stdout: Option<HANDLE>, dir: Option<&str>) -> core::result::Result<Child, String> {
    let mut line = command_line(exe, args);
    let dir = dir.map(to_wide);
    let mut si: STARTUPINFOW = unsafe { zeroed() };
    si.cb = size_of::<STARTUPINFOW>() as u32;
    if let Some(out) = stdout {
        si.dwFlags = STARTF_USESTDHANDLES;
        si.hStdInput = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
        si.hStdOutput = out;
        si.hStdError = unsafe { GetStdHandle(STD_ERROR_HANDLE) };
    }
    let mut pi: PROCESS_INFORMATION = unsafe { zeroed() };
    let ok = unsafe {
        CreateProcessW(
            null(),
            line.as_mut_ptr(),
            null(),
            null(),
            TRUE,
            CREATE_NO_WINDOW | CREATE_UNICODE_ENVIRONMENT,
            null::<c_void>(),
            dir.as_ref().map_or(null(), |d| d.as_ptr()),
            &si,
            &mut pi,
        )
    };
    if ok == 0 {
        return Err(pk::error::last());
    }
    unsafe { CloseHandle(pi.hThread) };
    Ok(Child { process: pi.hProcess, id: pi.dwProcessId })
}

/// `"<exe>" arg arg…`, zero-terminated.
fn command_line(exe: &str, args: &[&str]) -> Vec<u16> {
    let mut s = String::new();
    s.push('"');
    s.push_str(exe);
    s.push('"');
    for a in args {
        s.push(' ');
        append_arg(&mut s, a);
    }
    to_wide(&s)
}

/// One argument as .NET's `PasteArguments.AppendArgument` writes it: as-is unless it's empty or
/// has whitespace or quotes; then quoted, with backslashes doubled before a quote (or the end) and
/// quotes escaped.
pub fn append_arg(s: &mut String, arg: &str) {
    if !arg.is_empty() && !arg.chars().any(|c| c.is_whitespace() || c == '"') {
        s.push_str(arg);
        return;
    }
    s.push('"');
    let chars: Vec<char> = arg.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        i += 1;
        if c == '\\' {
            let mut n = 1;
            while i < chars.len() && chars[i] == '\\' {
                i += 1;
                n += 1;
            }
            if i == chars.len() {
                s.extend(core::iter::repeat_n('\\', n * 2));
            } else if chars[i] == '"' {
                s.extend(core::iter::repeat_n('\\', n * 2 + 1));
                s.push('"');
                i += 1;
            } else {
                s.extend(core::iter::repeat_n('\\', n));
            }
        } else if c == '"' {
            s.push('\\');
            s.push('"');
        } else {
            s.push(c);
        }
    }
    s.push('"');
}

/// The output as .NET's reader decoded it: UTF-8, a byte-order mark dropped.
fn decode_utf8(b: &[u8]) -> String {
    String::from_utf8_lossy(b.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(b)).into()
}

/// `Path.GetFileName`.
pub fn file_name(path: &str) -> &str {
    path.rsplit(['\\', '/']).next().unwrap_or(path)
}

/// `Path.GetFileNameWithoutExtension`.
pub fn file_stem(path: &str) -> &str {
    let name = file_name(path);
    match name.rfind('.') {
        Some(i) => &name[..i],
        None => name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_sys::Win32::Foundation::LocalFree;
    use windows_sys::Win32::UI::Shell::CommandLineToArgvW;

    fn quoted(a: &str) -> String {
        let mut s = String::new();
        append_arg(&mut s, a);
        s
    }

    /// Windows' own splitter, for the round trip.
    fn split(line: &[u16]) -> Vec<String> {
        let mut n = 0;
        let argv = unsafe { CommandLineToArgvW(line.as_ptr(), &mut n) };
        let out = (0..n as usize).map(|i| unsafe { pk::wide::from_ptr(*argv.add(i)) }).collect();
        unsafe { LocalFree(argv as _) };
        out
    }

    #[test]
    fn quoting() {
        assert_eq!(quoted("set"), "set");
        assert_eq!(quoted("rect=1234"), "rect=1234");
        assert_eq!(quoted(""), "\"\"");
        assert_eq!(quoted("device=AirPods Pro"), "\"device=AirPods Pro\"");
        assert_eq!(quoted("a\"b"), "\"a\\\"b\"");
        assert_eq!(quoted("C:\\a b\\"), "\"C:\\a b\\\\\"");
        assert_eq!(quoted("x\\\\\"y"), "\"x\\\\\\\\\\\"y\"");
        let args = ["set", "device=AirPods Pro", "", "a\"b", "C:\\dir with space\\", "tab\there", "\\\\\"", "q\\", "ünï ço∂é"];
        let line = command_line("C:\\Program Files\\pk\\snipsnap.exe", &args);
        let got = split(&line);
        assert_eq!(got[0], "C:\\Program Files\\pk\\snipsnap.exe");
        assert_eq!(&got[1..], &args.iter().map(|a| String::from(*a)).collect::<Vec<_>>()[..]);
    }

    #[test]
    fn names() {
        assert_eq!(file_name("C:\\x\\QuickTooth.exe"), "QuickTooth.exe");
        assert_eq!(file_stem("C:\\x\\QuickTooth.exe"), "QuickTooth");
        assert_eq!(file_stem("C:\\x.y\\noext"), "noext");
    }

    #[test]
    fn runs_and_times_out() {
        let cmd = "C:\\Windows\\System32\\cmd.exe";
        let (code, out) = run(cmd, &["/c", "echo hi there& exit 3"], 5000).unwrap();
        assert_eq!((code, out.as_str()), (3, "hi there\r\n"));
        let big = run(cmd, &["/c", "for /l %i in (1,1,3000) do @echo 0123456789012345678901234567890123456789"], 20000).unwrap();
        assert_eq!(big.1.len(), 3000 * 42);
        let slow = run(cmd, &["/c", "ping -n 4 127.0.0.1 >nul"], 300);
        assert_eq!(slow, Err(String::from("cmd did not answer.")));
        assert_eq!(run("C:\\nowhere\\missing.exe", &[], 1000), Err(String::from("Could not start missing.exe.")));
    }

    #[test]
    fn starts_in_a_folder() {
        let cmd = "C:\\Windows\\System32\\cmd.exe";
        let check = ["/c", "if exist win.ini (exit 7) else (exit 3)"];
        assert_eq!(start_in(cmd, &check, "C:\\Windows").unwrap().wait(5000), Some(7));
        assert_eq!(start_in(cmd, &check, "C:\\Windows\\System32").unwrap().wait(5000), Some(3));
        assert!(start_in(cmd, &check, "C:\\nowhere\\at all").is_err());
    }
}
