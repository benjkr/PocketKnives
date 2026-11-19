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
    STARTUPINFOW, TerminateProcess, WaitForMultipleObjects, WaitForSingleObject,
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
    Capture::start(exe, args)?.finish(timeout_ms)
}

/// [`run`] for several commands at once (up to 64, `WaitForMultipleObjects`' limit): all start
/// now and run while the caller does other work; [`Started::finish`] collects them.
pub fn start_all<const N: usize>(commands: [(&str, &[&str]); N]) -> Started<N> {
    Started { captures: commands.map(|(exe, args)| Capture::start(exe, args)) }
}

/// Commands from [`start_all`], still running or done.
pub struct Started<const N: usize> {
    captures: [Result<Capture>; N],
}

impl<const N: usize> Started<N> {
    /// Each command's result, in the order given. They get `timeout_ms` more from now.
    pub fn finish(self, timeout_ms: u32) -> [Result<(u32, String)>; N] {
        let mut handles = [null_mut(); N];
        let mut count = 0;
        for capture in self.captures.iter().flatten() {
            handles[count] = capture.child.handle();
            count += 1;
        }
        if count > 0 {
            unsafe { WaitForMultipleObjects(count as u32, handles.as_ptr(), TRUE, timeout_ms) };
        }
        // Whatever is still running now has had its time.
        self.captures.map(|c| c?.finish(0))
    }
}

/// A started process whose standard output another thread is reading.
struct Capture {
    /// The exe's name without its extension, for the error.
    name: String,
    child: Child,
    reader: pk::thread::Thread,
    output: Arc<Mutex<Vec<u8>>>,
}

impl Capture {
    fn start(exe: &str, args: &[&str]) -> Result<Capture> {
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
        // Our copy of the write end must go, or the read never sees the end of the output. Closed
        // before the next start, so another child never inherits it.
        unsafe { CloseHandle(write) };
        let child = match child {
            Ok(c) => c,
            Err(_) => {
                unsafe { CloseHandle(read) };
                return Err(format!("Could not start {}.", file_name(exe)));
            }
        };
        // Read while waiting: a blade that fills the pipe would otherwise never end.
        let pipe = Pipe(read as usize);
        let output = Arc::new(Mutex::new(Vec::new()));
        let sink = output.clone();
        let Some(reader) = pk::thread::spawn(move || *sink.lock() = pipe.read_to_end()) else {
            child.kill();
            return Err(format!("Could not start {}.", file_name(exe)));
        };
        Ok(Capture { name: file_stem(exe).into(), child, reader, output })
    }

    /// Waits up to `ms` for the process to end, ends it after that, and collects its output.
    fn finish(self, ms: u32) -> Result<(u32, String)> {
        let code = self.child.wait(ms);
        if code.is_none() {
            self.child.kill();
        }
        // The pipe ends with the process (the blades start nothing that inherits it).
        self.reader.join();
        let bytes = core::mem::take(&mut *self.output.lock());
        match code {
            Some(code) => Ok((code, decode_utf8(&bytes))),
            None => Err(format!("{} did not answer.", self.name)),
        }
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

    /// Two 2-second commands finish together, not one after the other; a slow one is ended at the
    /// time limit while the others keep their answers, in the order given.
    #[test]
    fn runs_at_the_same_time() {
        let cmd = "C:\\Windows\\System32\\cmd.exe";
        let wait2: &[&str] = &["/c", "ping -n 3 127.0.0.1 >nul& echo done& exit 4"];
        let clock = std::time::Instant::now();
        let both = start_all([(cmd, wait2), (cmd, wait2)]).finish(10000);
        let took = clock.elapsed().as_millis();
        assert_eq!(both, [Ok((4, String::from("done\r\n"))), Ok((4, String::from("done\r\n")))]);
        assert!(took < 3500, "took {took} ms");

        // (cmd's ping keeps the pipe open after cmd is ended, so this waits for the ping.)
        let quick: &[&str] = &["/c", "echo hi& exit 3"];
        let slow: &[&str] = &["/c", "ping -n 3 127.0.0.1 >nul"];
        let mixed = start_all([(cmd, slow), ("C:\\nowhere\\missing.exe", &[]), (cmd, quick)]).finish(500);
        assert_eq!(
            mixed,
            [Err(String::from("cmd did not answer.")), Err(String::from("Could not start missing.exe.")), Ok((3, String::from("hi\r\n")))]
        );
        assert_eq!(start_all::<0>([]).finish(1000), []);
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
