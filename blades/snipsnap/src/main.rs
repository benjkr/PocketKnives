//! SnipSnap - a tiny, fast screen snippet and recording tool for Windows.
//!
//! Three screenshot captures (rectangle, window, monitor under cursor -> clipboard 24-bit CF_DIB)
//! and three video recordings (same three modes, delegated on demand to `snipsnap-rec.exe` ->
//! H.264 MP4 in `Videos\SnipSnap\` + clipboard CF_HDROP).
//!
//! ```text
//! snipsnap            run in the background and listen for hotkeys
//! snipsnap rect       select a rectangle once, then exit
//! snipsnap window     pick a window once, then exit
//! snipsnap full       capture the monitor under the cursor once, then exit
//! snipsnap rec-rect   record a rectangle (spawns snipsnap-rec.exe)
//! snipsnap rec-window record a window (spawns snipsnap-rec.exe)
//! snipsnap rec-full   record the monitor under the cursor (spawns snipsnap-rec.exe)
//! snipsnap quit       stop the background instance (and any active recorder)
//! snipsnap settings   print the settings (for PocketKnives Settings)
//! snipsnap set k=v..  change settings and apply them to the background instance
//! ```
//!
//! Exit codes: 0 = copied / ok, 1 = cancelled, 2 = failed / bad usage.
//!
//! This file: the defaults, the command line, output, settings, hotkeys and the background
//! instance. `capture`: the captures, the overlay, the clipboard and the flash. `pixels`: the
//! per-pixel loops and the overlay's paint.
#![no_std]
#![no_main]

mod capture;
mod pixels;

use core::cell::UnsafeCell;
use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::ptr::{null, null_mut};
use pk::w;
use windows_sys::Win32::Foundation::{CloseHandle, GENERIC_WRITE, HANDLE, HWND, INVALID_HANDLE_VALUE, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::Globalization::{CP_UTF8, WideCharToMultiByte};
use windows_sys::Win32::Storage::FileSystem::{
    CreateDirectoryW, CreateFileW, FILE_SHARE_WRITE, FILE_TYPE_UNKNOWN, GetFileType, OPEN_EXISTING, WriteFile,
};
use windows_sys::Win32::System::Console::{
    ATTACH_PARENT_PROCESS, AttachConsole, FreeConsole, GetStdHandle, STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE, SetStdHandle,
};
use windows_sys::Win32::System::Environment::{GetCommandLineW, GetEnvironmentVariableW};
use windows_sys::Win32::System::LibraryLoader::{GetModuleFileNameW, GetModuleHandleW};
use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_SYNCHRONIZE, WaitForSingleObject};
use windows_sys::Win32::System::WindowsProgramming::{GetPrivateProfileIntW, WritePrivateProfileStringW};
use windows_sys::Win32::UI::HiDpi::{DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, SetProcessDpiAwarenessContext};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyNameTextW, MAPVK_VK_TO_VSC_EX, MOD_ALT, MOD_CONTROL, MOD_NOREPEAT, MOD_SHIFT, MOD_WIN, MapVirtualKeyW, RegisterHotKey,
    UnregisterHotKey, VK_DELETE, VK_F1, VK_NUMLOCK, VK_PAUSE, VK_PRIOR, VK_SNAPSHOT,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, FindWindowExW, GetMessageW, GetWindowThreadProcessId, HWND_MESSAGE,
    MB_ICONERROR, MB_ICONWARNING, MSG, MessageBoxA, PostMessageW, PostQuitMessage, RegisterClassW, RegisterWindowMessageW,
    SMTO_ABORTIFHUNG, SendMessageTimeoutW, SetProcessDPIAware, TranslateMessage, WM_CLOSE, WM_DESTROY, WM_HOTKEY, WNDCLASSW, wsprintfA,
};

pk::entry!(run);

// ------------------------------------------------------------------------------- the defaults

const RC_OK: i32 = 0;
const RC_CANCEL: i32 = 1;
const RC_FAIL: i32 = 2;
/// Hotkey ids: the six captures (`HK_RECT + i` for setting `i`), and Esc on the overlay.
const HK_RECT: i32 = 1;
const HK_WINDOW: i32 = 2;
const HK_FULL: i32 = 3;
const HK_REC_RECT: i32 = 4;
const HK_REC_WINDOW: i32 = 5;
const HK_REC_FULL: i32 = 6;
const HK_ESC: i32 = 7;

const DAEMON_CLASS: *const u16 = w!("SnipSnapDaemon");
const OVERLAY_CLASS: *const u16 = w!("SnipSnapOverlay");
const FLASH_CLASS: *const u16 = w!("SnipSnapFlash");
const REC_CLASS: *const u16 = w!("SnipSnapRec");
const MUTEX_NAME: *const u16 = w!("Local\\SnipSnap.Daemon");
/// Selection / hovered-window border: its colour (0x00BBGGRR) and thickness in px.
const FRAME_COLOR: u32 = 0x00FF_FFFF;
/// Recording selection border: #e81123 in 0x00BBGGRR.
const REC_FRAME_COLOR: u32 = 0x0023_11E8;
const FRAME_W: i32 = 2;
/// The white flash: its starting opacity and how long it takes to fade out, ms.
const FLASH_ALPHA: u64 = 150;
const FLASH_MS: u64 = 150;

/// A hotkey as one number, modifiers * 256 + virtual key: the settings format.
const fn hk(m: u32, vk: u32) -> u32 {
    m << 8 | vk
}
const HK_MAX: u32 = hk(MOD_ALT | MOD_CONTROL | MOD_SHIFT | MOD_WIN, 0xFF);
const CA: u32 = MOD_CONTROL | MOD_ALT;
const CAS: u32 = MOD_CONTROL | MOD_ALT | MOD_SHIFT;

/// A hotkey and its name for people. Every name is followed by a zero in memory (`wsprintfA`'s
/// `%s` reads it).
#[derive(Clone, Copy)]
struct Hotkey {
    v: u32,
    name: &'static [u8],
}
const fn key(v: u32, name: &'static [u8]) -> Hotkey {
    // `name` is a literal ending in "\0"; the slice leaves the zero out.
    Hotkey { v, name: name.split_last().unwrap().1 }
}

/// Hotkey candidates for a capture with no saved hotkey, tried in order; the first one that
/// registers wins. A capture's list ends where the next one's starts (`SETTINGS` below).
static FALLBACKS: [Hotkey; 8] = [
    // Rectangle
    key(hk(CA, b'R' as u32), b"Ctrl+Alt+R\0"),
    key(hk(MOD_CONTROL, VK_SNAPSHOT as u32), b"Ctrl+PrtSc\0"),
    // Window
    key(hk(CA, b'W' as u32), b"Ctrl+Alt+W\0"),
    // Full screen
    key(hk(CA, b'F' as u32), b"Ctrl+Alt+F\0"),
    key(hk(MOD_CONTROL | MOD_SHIFT, VK_SNAPSHOT as u32), b"Ctrl+Shift+PrtSc\0"),
    // Record rectangle
    key(hk(CAS, b'R' as u32), b"Ctrl+Alt+Shift+R\0"),
    // Record window
    key(hk(CAS, b'W' as u32), b"Ctrl+Alt+Shift+W\0"),
    // Record full screen
    key(hk(CAS, b'F' as u32), b"Ctrl+Alt+Shift+F\0"),
];

/// A setting: its key (UTF-16, zero-terminated), its label (zero-terminated) and where its
/// hotkey list starts in `FALLBACKS`.
struct Setting {
    key: &'static [u16],
    label: &'static [u8],
    keys: usize,
}

/// ASCII `s` (ending in "\0") as UTF-16.
const fn wide<const N: usize>(s: &[u8; N]) -> [u16; N] {
    let mut out = [0u16; N];
    let mut i = 0;
    while i < N {
        out[i] = s[i] as u16;
        i += 1;
    }
    out
}

/// The settings, in the order PocketKnives Settings shows them. The first six are the hotkeys
/// of `HK_RECT`..=`HK_REC_FULL`; the seventh's list start is the list's end.
const S_HOTKEYS: usize = 6;
const S_FLASH: usize = 6;
const S_COUNT: usize = 7;
const HK_MASK: i32 = (1 << S_HOTKEYS) - 1;
const BAD_IO: i32 = 1 << S_HOTKEYS;
static SETTINGS: [Setting; S_COUNT] = [
    Setting { key: &wide(b"rect\0"), label: b"Rectangle\0", keys: 0 },
    Setting { key: &wide(b"window\0"), label: b"Window\0", keys: 2 },
    Setting { key: &wide(b"full\0"), label: b"Full screen\0", keys: 3 },
    Setting { key: &wide(b"recRect\0"), label: b"Record rectangle\0", keys: 5 },
    Setting { key: &wide(b"recWindow\0"), label: b"Record window\0", keys: 6 },
    Setting { key: &wide(b"recFull\0"), label: b"Record full screen\0", keys: 7 },
    Setting { key: &wide(b"flash\0"), label: b"Flash the captured area\0", keys: 8 },
];
const INI_SECTION: *const u16 = w!("SnipSnap");
const RELOAD_MSG: *const u16 = w!("PocketKnives.Reload.v1");
/// Reload reply: handled; the low bits are the captures left without a key.
const RELOAD_DONE: usize = 0x100;

// ------------------------------------------------------------------------------------ globals

/// A global of this single-threaded program (its window procedures run on the one thread).
/// Values are copied in and out, so no reference into one outlives a call that could re-enter.
struct G<T>(UnsafeCell<T>);
unsafe impl<T> Sync for G<T> {}
impl<T: Copy> G<T> {
    const fn new(v: T) -> Self {
        G(UnsafeCell::new(v))
    }
    fn get(&self) -> T {
        unsafe { *self.0.get() }
    }
    fn set(&self, v: T) {
        unsafe { *self.0.get() = v }
    }
    /// For a buffer: used only for the duration of one call that doesn't re-enter.
    fn ptr(&self) -> *mut T {
        self.0.get()
    }
}

/// Running as the background instance.
static DAEMON: G<bool> = G::new(false);

/// Whether UTF-16 `a` is the ASCII text `s`.
fn is(a: &[u16], s: &[u8]) -> bool {
    a.len() == s.len() && a.iter().zip(s).all(|(&a, &s)| a == s as u16)
}

/// A text buffer for `sprintf!`, left uninitialised: `wsprintfA` writes it.
struct Buf<const N: usize>([MaybeUninit<u8>; N]);
impl<const N: usize> Buf<N> {
    fn new() -> Self {
        Buf([MaybeUninit::uninit(); N])
    }
    fn ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr().cast()
    }
    /// The first `n` bytes, which `wsprintfA` (or the caller) wrote.
    fn text(&self, n: usize) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.0.as_ptr().cast(), n.min(N)) }
    }
}

/// `wsprintfA` into a `Buf`; the text written, without its zero. (`wsprintfA` writes at most
/// 1024 bytes; every format here fits its buffer.)
macro_rules! sprintf {
    ($buf:expr, $fmt:expr $(, $arg:expr)*) => {{
        let buf = &mut $buf;
        let n = unsafe { wsprintfA(buf.ptr(), $fmt.as_ptr().cast() $(, $arg)*) };
        buf.text(n as usize)
    }};
}

// ------------------------------------------------------------------------------------- output

/// Null: not looked for yet; `INVALID_HANDLE_VALUE`: nowhere to write.
static OUT: G<HANDLE> = G::new(null_mut());

fn say(s: &[u8]) {
    let mut out = OUT.get();
    if out.is_null() {
        let h = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
        if !h.is_null() && h != INVALID_HANDLE_VALUE && unsafe { GetFileType(h) } != FILE_TYPE_UNKNOWN {
            out = h;
        } else if unsafe { AttachConsole(ATTACH_PARENT_PROCESS) } != 0 {
            out = unsafe { CreateFileW(w!("CONOUT$"), GENERIC_WRITE, FILE_SHARE_WRITE, null(), OPEN_EXISTING, 0, null_mut()) };
        }
        if out.is_null() {
            out = INVALID_HANDLE_VALUE;
        }
        OUT.set(out);
    }
    if out == INVALID_HANDLE_VALUE {
        return;
    }
    let mut n = 0u32;
    unsafe { WriteFile(out, s.as_ptr(), s.len() as u32, &mut n, null_mut()) };
}

/// Lets go of any inherited console / pipe, so a background instance never keeps a terminal or a
/// capturing parent waiting.
fn detach_output() {
    let out = OUT.get();
    if !out.is_null() && out != INVALID_HANDLE_VALUE && out != unsafe { GetStdHandle(STD_OUTPUT_HANDLE) } {
        unsafe { CloseHandle(out) };
    }
    OUT.set(INVALID_HANDLE_VALUE);
    for id in STD_ERROR_HANDLE..=STD_INPUT_HANDLE {
        let h = unsafe { GetStdHandle(id) };
        if !h.is_null() && h != INVALID_HANDLE_VALUE {
            unsafe { CloseHandle(h) };
        }
        unsafe { SetStdHandle(id, null_mut()) };
    }
    unsafe { FreeConsole() };
}

// ----------------------------------------------------------------------------------- settings
//
// An INI file read and written through kernel32: %APPDATA%\SnipSnap\snipsnap.ini, or
// snipsnap.ini next to the exe in a PocketKnives preview. A hotkey is saved as hk(mod, vk); a
// capture without one uses its fallback list.

const MAX_PATH: usize = 260;
/// The settings file's path, zero-terminated; empty: no settings file.
static INI: G<[u16; MAX_PATH + 32]> = G::new([0; MAX_PATH + 32]);
/// Where its folder's name ends; 0 = don't create it.
static INI_DIR: G<usize> = G::new(0);
/// `POCKETKNIVES_SETTINGS_PREVIEW=1`.
static PREVIEW: G<bool> = G::new(false);
/// Hotkeys: the saved value or 0; flash: 1 or 0.
static SET: G<[u32; S_COUNT]> = G::new([0; S_COUNT]);

fn hotkey_ok(v: u32) -> bool {
    v <= HK_MAX && v & 0xFF != 0
}

/// A capture's hotkey: the saved one, or the first of its fallbacks.
fn hotkey_of(i: usize) -> u32 {
    let v = SET.get()[i];
    if v != 0 { v } else { FALLBACKS[SETTINGS[i].keys].v }
}

fn ini() -> *const u16 {
    INI.ptr() as *const u16
}

fn have_ini() -> bool {
    INI.get()[0] != 0
}

/// The saved value, or `u32::MAX` when there's none.
#[inline(never)]
fn ini_get(i: usize) -> u32 {
    if !have_ini() {
        return u32::MAX;
    }
    unsafe { GetPrivateProfileIntW(INI_SECTION, SETTINGS[i].key.as_ptr(), -1, ini()) as u32 }
}

/// Saves a value; `u32::MAX` removes it.
fn ini_put(i: usize, v: u32) -> bool {
    let mut num = [0u16; 12];
    let mut d = 11;
    let mut k = v;
    loop {
        d -= 1;
        num[d] = b'0' as u16 + (k % 10) as u16;
        k /= 10;
        if k == 0 {
            break;
        }
    }
    let text = if v == u32::MAX { null() } else { num[d..].as_ptr() };
    have_ini() && unsafe { WritePrivateProfileStringW(INI_SECTION, SETTINGS[i].key.as_ptr(), text, ini()) } != 0
}

fn settings_load() {
    const TAIL: [u16; 23] = wide(b"\\SnipSnap\\snipsnap.ini\0"); // a preview's: its end
    let mut e = [0u16; 2];
    let preview = unsafe { GetEnvironmentVariableW(w!("POCKETKNIVES_SETTINGS_PREVIEW"), e.as_mut_ptr(), 2) } == 1 && e[0] == b'1' as u16;
    PREVIEW.set(preview);
    let path = unsafe { &mut *INI.ptr() };
    let (mut n, tail) = if preview {
        let mut n = unsafe { GetModuleFileNameW(null_mut(), path.as_mut_ptr(), MAX_PATH as u32) } as usize;
        while n > 0 {
            n -= 1;
            if path[n] == b'\\' as u16 {
                break; // the exe's folder
            }
        }
        (n, &TAIL[9..])
    } else {
        let n = unsafe { GetEnvironmentVariableW(w!("APPDATA"), path.as_mut_ptr(), MAX_PATH as u32) } as usize;
        (if n >= MAX_PATH { 0 } else { n }, &TAIL[..])
    };
    INI_DIR.set(if preview { 0 } else { n + 9 });
    path[n..n + tail.len()].copy_from_slice(tail);
    if n == 0 {
        path[0] = 0;
    }
    n = 0;
    let mut set = [0u32; S_COUNT];
    while n < S_COUNT {
        let v = ini_get(n);
        set[n] = if n < S_HOTKEYS { if hotkey_ok(v) { v } else { 0 } } else { (v != 0) as u32 };
        n += 1;
    }
    SET.set(set);
}

/// A hotkey's name for people, such as "Ctrl+Alt+A", in UTF-8, into `out`; the name without its
/// zero (which follows it).
fn key_name(out: &mut [u8; 96], v: u32) -> &[u8] {
    // Each modifier's bit, then its name.
    const MODS: &[u8] = b"\x02Ctrl+\0\x01Alt+\0\x04Shift+\0\x08Win+\0";
    let vk = v & 0xFF;
    let mut sc = unsafe { MapVirtualKeyW(vk, MAPVK_VK_TO_VSC_EX) };
    let mut o = 0;
    let mut bit = 0;
    for &c in MODS {
        if bit == 0 {
            bit = c as u32;
        } else if c == 0 {
            bit = 0;
        } else if v >> 8 & bit != 0 {
            out[o] = c;
            o += 1;
        }
    }
    // Windows quirk: MapVirtualKey doesn't mark the navigation keys and Num Lock as extended (so
    // they'd be named as keypad keys), and gives Pause an E1 scan code (named "Right Ctrl").
    if vk == VK_PAUSE as u32 {
        sc = 0x45;
    } else if vk.wrapping_sub(VK_PRIOR as u32) <= (VK_DELETE - VK_PRIOR) as u32 || vk == VK_NUMLOCK as u32 {
        sc |= 0xE000;
    }
    // Scan code in bits 16-23, extended-key flag in bit 24. F13-F24 have no name; PrtSc is named
    // as in FALLBACKS (Windows: "Prnt Scrn").
    let mut w = [0u16; 24];
    let rest = out[o..].as_mut_ptr();
    let left = (96 - o) as i32;
    let n = unsafe {
        if vk == VK_SNAPSHOT as u32 {
            wsprintfA(rest, c"PrtSc".as_ptr().cast())
        } else if GetKeyNameTextW(((sc & 0xFF) << 16 | ((sc > 0xFF) as u32) << 24) as i32, w.as_mut_ptr(), 24) != 0 {
            WideCharToMultiByte(CP_UTF8, 0, w.as_ptr(), -1, rest, left, null(), null_mut()) - 1
        } else if vk.wrapping_sub(VK_F1 as u32) < 24 {
            wsprintfA(rest, c"F%u".as_ptr().cast(), vk - VK_F1 as u32 + 1)
        } else {
            wsprintfA(rest, c"Key %u".as_ptr().cast(), vk)
        }
    };
    &out[..o + n.clamp(0, left) as usize]
}

fn try_key(h: HWND, id: i32, v: u32) -> bool {
    unsafe { RegisterHotKey(h, id, v >> 8 | MOD_NOREPEAT, v & 0xFF) != 0 }
}

// ------------------------------------------------------------------------------------- daemon

static BUSY: G<bool> = G::new(false);
/// `RELOAD_MSG`.
static RELOAD: G<u32> = G::new(0);
/// Each capture's key name; `None`: no key.
static KEY: G<[Option<&'static [u8]>; S_HOTKEYS]> = G::new([None; S_HOTKEYS]);
static KEY_BUF: G<[[u8; 96]; S_HOTKEYS]> = G::new([[0; 96]; S_HOTKEYS]);

/// (Re)registers the capture hotkeys: a saved one exactly, otherwise the first of the fallbacks
/// that registers. Returns bit i set for capture i without a key, bit i + 4 for one on a fallback.
fn register_keys(h: HWND) -> i32 {
    let mut r = 0;
    let set = SET.get();
    let mut keys = [None; S_HOTKEYS];
    for i in 0..S_HOTKEYS {
        unsafe { UnregisterHotKey(h, HK_RECT + i as i32) };
    }
    for i in 0..S_HOTKEYS {
        let first = SETTINGS[i].keys;
        let mut k = first;
        if set[i] != 0 {
            if try_key(h, HK_RECT + i as i32, set[i]) {
                // A static buffer per capture, written only here.
                let buf: &'static mut [u8; 96] = unsafe { &mut (*KEY_BUF.ptr())[i] };
                keys[i] = Some(key_name(buf, set[i]));
            }
        } else {
            while k < SETTINGS[i + 1].keys {
                if try_key(h, HK_RECT + i as i32, FALLBACKS[k].v) {
                    keys[i] = Some(FALLBACKS[k].name);
                    break;
                }
                k += 1;
            }
        }
        if keys[i].is_none() {
            r |= 1 << i;
        } else if k != first {
            r |= 0x100 << i;
        }
    }
    KEY.set(keys);
    r
}

/// Re-reads the settings and re-registers the hotkeys: the reply to `RELOAD_MSG`, `RELOAD_DONE`
/// plus a bit for each capture left without a key.
fn reload_keys(h: HWND) -> LRESULT {
    settings_load();
    (RELOAD_DONE | (register_keys(h) & HK_MASK) as usize) as LRESULT
}

unsafe extern "system" fn daemon_proc(h: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    if msg == RELOAD.get() {
        return reload_keys(h); // `snipsnap set` saved new settings
    }
    match msg {
        WM_HOTKEY => {
            if !BUSY.get() {
                BUSY.set(true);
                match wp as i32 {
                    HK_RECT => _ = capture::snip_select(h, capture::Mode::Rect),
                    HK_WINDOW => _ = capture::snip_select(h, capture::Mode::Window),
                    HK_FULL => _ = capture::snip_area(h, capture::cursor_monitor()),
                    HK_REC_RECT => _ = capture::rec_trigger(h, capture::RecKind::Rect),
                    HK_REC_WINDOW => _ = capture::rec_trigger(h, capture::RecKind::Window),
                    HK_REC_FULL => _ = capture::rec_trigger(h, capture::RecKind::Full),
                    _ => {}
                }
                BUSY.set(false);
            }
            0
        }
        WM_CLOSE => {
            unsafe { DestroyWindow(h) };
            0
        }
        WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
            0
        }
        _ => unsafe { DefWindowProcW(h, msg, wp, lp) },
    }
}

/// The name of capture `i`'s key, or `none` (both followed by a zero).
fn key_or(i: usize, none: &'static [u8]) -> *const u8 {
    KEY.get()[i].unwrap_or(none).as_ptr()
}

fn run_daemon() -> i32 {
    let Some(mutex) = pk::instance::NamedMutex::first(MUTEX_NAME) else {
        say(b"SnipSnap is already running. Use 'snipsnap quit' to stop it.\n");
        return RC_OK;
    };
    DAEMON.set(true);
    RELOAD.set(unsafe { RegisterWindowMessageW(RELOAD_MSG) });

    let mut wc: WNDCLASSW = unsafe { core::mem::zeroed() };
    wc.lpfnWndProc = Some(daemon_proc);
    wc.hInstance = instance();
    wc.lpszClassName = DAEMON_CLASS;
    unsafe { RegisterClassW(&wc) };
    let h = unsafe { CreateWindowExW(0, DAEMON_CLASS, w!("SnipSnap"), 0, 0, 0, 0, 0, HWND_MESSAGE, null_mut(), wc.hInstance, null()) };
    if h.is_null() {
        say(b"SnipSnap: could not create window.\n");
        return RC_FAIL;
    }

    let r = register_keys(h);
    if r & HK_MASK == HK_MASK {
        const MSG: &[u8] = b"SnipSnap: none of the hotkeys could be registered (another program owns them).\n\
                             Use 'snipsnap rect' / 'window' / 'full' from a launcher instead.\n\0";
        say(&MSG[..MSG.len() - 1]);
        if OUT.get() == INVALID_HANDLE_VALUE {
            unsafe { MessageBoxA(null_mut(), MSG.as_ptr(), c"SnipSnap".as_ptr().cast(), MB_ICONERROR) };
        }
        return RC_FAIL;
    }

    say(b"SnipSnap running in the background.\n");
    let keys = KEY.get();
    for i in 0..S_HOTKEYS {
        // "  Record full screen:  Ctrl+Alt+Shift+F"
        let mut line = Buf::<40>::new();
        let mut n = sprintf!(line, c"  %s:", SETTINGS[i].label.as_ptr()).len();
        while n < 23 {
            line.0[n].write(b' ');
            n += 1;
        }
        say(line.text(n));
        say(keys[i].unwrap_or(b"(unavailable - hotkeys taken)"));
        say(b"\n");
    }
    say(b"  Stop with:           snipsnap quit\n");
    // No console to print to: speak up only if a capture didn't get its first choice (its saved
    // hotkey, or its first fallback). A saved key's name is UTF-8, so a non-ASCII one shows
    // garbled here (as in the C build, where converting it cost a 512-byte section step).
    if OUT.get() == INVALID_HANDLE_VALUE && r != 0 {
        let mut msg = Buf::<600>::new();
        let none = b"(unavailable)\0".as_slice();
        _ = sprintf!(
            msg,
            c"Some default hotkeys are taken by another program.\n\nRectangle:\t\t%s\nWindow:\t\t\t%s\nFull screen:\t\t%s\nRecord rectangle:\t%s\nRecord window:\t\t%s\nRecord full screen:\t%s",
            key_or(0, none),
            key_or(1, none),
            key_or(2, none),
            key_or(3, none),
            key_or(4, none),
            key_or(5, none)
        );
        unsafe { MessageBoxA(null_mut(), msg.ptr(), c"SnipSnap".as_ptr().cast(), MB_ICONWARNING) };
    }
    detach_output();

    message_loop(|| true);
    drop(mutex);
    RC_OK
}

/// Dispatches messages while `go()` holds and the queue isn't quitting. Returns the `WM_QUIT`'s
/// code if that came (`GetMessageW` failing counts as a quit too).
#[inline(never)]
fn message_loop(go: fn() -> bool) -> Option<i32> {
    let mut m: MSG = unsafe { core::mem::zeroed() };
    while go() {
        if unsafe { GetMessageW(&mut m, null_mut(), 0, 0) } <= 0 {
            return Some(m.wParam as i32);
        }
        unsafe {
            TranslateMessage(&m);
            DispatchMessageW(&m);
        }
    }
    None
}

fn find_daemon() -> HWND {
    unsafe { FindWindowExW(HWND_MESSAGE, null_mut(), DAEMON_CLASS, null()) }
}

fn quit_daemon() -> i32 {
    let rec = unsafe { FindWindowExW(HWND_MESSAGE, null_mut(), REC_CLASS, null()) };
    if !rec.is_null() {
        unsafe { PostMessageW(rec, WM_CLOSE, 0, 0) };
    }
    let h = find_daemon();
    if h.is_null() {
        say(b"SnipSnap is not running.\n");
        return RC_OK;
    }
    let mut pid = 0u32;
    unsafe {
        GetWindowThreadProcessId(h, &mut pid);
        let proc = OpenProcess(PROCESS_SYNCHRONIZE, 0, pid);
        PostMessageW(h, WM_CLOSE, 0, 0);
        if !proc.is_null() {
            WaitForSingleObject(proc, 3000);
            CloseHandle(proc);
        }
    }
    say(b"SnipSnap stopped.\n");
    RC_OK
}

// ----------------------------------------------------------------- settings and set commands
//
// The PocketKnives blade settings contract (docs/specs/blade-settings.md at the repo root):
// `settings` prints one tab-separated line per setting; `set` checks every key=value, saves,
// and has the background instance apply it.

fn print_settings() -> i32 {
    let set = SET.get();
    for (i, s) in SETTINGS.iter().enumerate() {
        let mut line = Buf::<96>::new();
        let (kind, v) = if i < S_HOTKEYS { (c"hotkey", hotkey_of(i)) } else { (c"toggle", set[i]) };
        say(sprintf!(line, c"%ls\t%s\t%u\t%s\n", s.key.as_ptr(), kind.as_ptr(), v, s.label.as_ptr()));
    }
    RC_OK
}

/// Refuses with one line naming a hotkey; exit code 2.
#[inline(never)]
fn refuse(fmt: &core::ffi::CStr, v: u32) -> i32 {
    // A static buffer (no zeroing code); this is the process's last output.
    static NAME: G<[u8; 96]> = G::new([0; 96]);
    let mut msg = Buf::<160>::new();
    let name = key_name(unsafe { &mut *NAME.ptr() }, v).as_ptr();
    say(sprintf!(msg, fmt, name));
    RC_FAIL
}

/// Asks the running copy to reload: its reply, or 0 if it didn't answer.
fn reload(d: HWND) -> usize {
    let mut r = 0usize;
    let msg = unsafe { RegisterWindowMessageW(RELOAD_MSG) };
    if unsafe { SendMessageTimeoutW(d, msg, 0, 0, SMTO_ABORTIFHUNG, 5000, &mut r) } == 0 {
        r = 0;
    }
    r
}

fn set_settings(mut p: Line) -> i32 {
    let mut v = [0u32; S_COUNT];
    let mut old = [0u32; S_COUNT];
    let mut mask = 0i32;
    let mut bad = 0i32;
    let set = SET.get();
    for i in 0..S_COUNT {
        v[i] = if i < S_HOTKEYS { hotkey_of(i) } else { set[i] };
    }
    loop {
        // Check every key=value first.
        let mut tok = [0u16; 32];
        let n;
        (n, p) = next_arg(p, &mut tok);
        if n == 0 {
            break;
        }
        // The setting whose key is followed by '='; `q` is where its value starts. (`tok` ends in
        // a zero, which ends each scan.)
        let mut i = 0;
        let mut q = 0;
        while i < S_COUNT {
            let k = SETTINGS[i].key;
            q = 0;
            while k[q] != 0 && k[q] == tok[q] {
                q += 1;
            }
            if k[q] == 0 && tok[q] == b'=' as u16 {
                break;
            }
            i += 1;
        }
        q += 1;
        let mut num = 0u32;
        let mut ok = i < S_COUNT && tok[q] != 0;
        while ok && tok[q] != 0 {
            let c = tok[q];
            ok = c.wrapping_sub(b'0' as u16) < 10 && num <= HK_MAX;
            num = num.wrapping_mul(10).wrapping_add((c as u32).wrapping_sub(b'0' as u32));
            q += 1;
        }
        if !ok || !(if i < S_HOTKEYS { hotkey_ok(num) } else { num <= 1 }) {
            let mut msg = Buf::<80>::new();
            say(sprintf!(msg, c"Not a valid setting: %ls\n", tok.as_ptr()));
            return RC_FAIL;
        }
        v[i] = num;
        mask |= 1 << i;
    }
    for i in 0..S_HOTKEYS {
        for j in i + 1..S_HOTKEYS {
            if v[i] == v[j] && (mask >> i | mask >> j) & 1 != 0 {
                return refuse(c"%s is already used by SnipSnap.\n", v[i]);
            }
        }
    }

    // Without a running copy (or in a preview, which never asks it), check the new hotkeys first
    // by registering them for a moment, as it would.
    let d = if PREVIEW.get() { null_mut() } else { find_daemon() };
    if d.is_null() {
        let mut set = SET.get();
        for i in 0..S_HOTKEYS {
            if mask >> i & 1 != 0 {
                set[i] = v[i];
            }
        }
        SET.set(set);
        bad = register_keys(null_mut()) & mask & HK_MASK;
    }
    if bad == 0 {
        let dir = INI_DIR.get();
        if dir != 0 {
            // Create the settings folder.
            let path = INI.ptr() as *mut u16;
            unsafe {
                *path.add(dir) = 0;
                CreateDirectoryW(path, null());
                *path.add(dir) = b'\\' as u16;
            }
        }
        for i in 0..S_COUNT {
            if mask >> i & 1 != 0 {
                old[i] = ini_get(i);
                if !ini_put(i, v[i]) {
                    bad = BAD_IO;
                }
            }
        }
    }
    if !d.is_null() && bad == 0 {
        // The running copy applies them.
        let r = reload(d);
        bad = if r & RELOAD_DONE != 0 { r as i32 & mask } else { BAD_IO };
        if bad != 0 {
            // Put the previous settings back.
            for i in 0..S_COUNT {
                if mask >> i & 1 != 0 {
                    ini_put(i, old[i]);
                }
            }
            if bad < BAD_IO {
                reload(d);
            }
        }
    }
    if bad == BAD_IO {
        say(b"Couldn't apply the settings.\n");
        return RC_FAIL;
    }
    for i in 0..S_HOTKEYS {
        if bad >> i & 1 != 0 {
            return refuse(c"%s is taken by another program.\n", v[i]);
        }
    }
    RC_OK
}

// --------------------------------------------------------------------------------------- main

/// A position in the command line, never past its terminating zero.
#[derive(Clone, Copy)]
struct Line(*const u16);
impl Line {
    /// The character here; 0 at the end.
    fn c(self) -> u16 {
        unsafe { *self.0 }
    }
    /// The next position (the same one at the end).
    fn next(self) -> Line {
        if self.c() == 0 { self } else { Line(unsafe { self.0.add(1) }) }
    }
}

/// The next command-line argument from `p`, lowercased, without leading '-' or '/', into `out`
/// (zero-terminated); its length is 0 when there are no more. Parsed by hand: no quote rules,
/// and shell32 never loads. A too-long argument ends in '#', so it matches nothing. Returns the
/// length and where it stopped.
fn next_arg<const N: usize>(mut p: Line, out: &mut [u16; N]) -> (usize, Line) {
    let mut n = 0;
    while p.c() == b' ' as u16 || p.c() == b'\t' as u16 {
        p = p.next();
    }
    while p.c() == b'-' as u16 || p.c() == b'/' as u16 {
        p = p.next();
    }
    while p.c() > b' ' as u16 {
        let c = p.c();
        p = p.next();
        if n == N - 1 {
            out[n - 1] = b'#' as u16;
            continue;
        }
        out[n] = if c.wrapping_sub(b'A' as u16) < 26 { c + 32 } else { c };
        n += 1;
    }
    out[n] = 0;
    (n, p)
}

fn run() -> u32 {
    // Must come first: everything below works in physical pixels.
    if unsafe { SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) } == 0 {
        unsafe { SetProcessDPIAware() };
    }
    // Windows gives every process a zero-terminated command line.
    let mut p = Line(unsafe { GetCommandLineW() });
    // The program.
    if p.c() == b'"' as u16 {
        p = p.next();
        while p.c() != 0 && p.c() != b'"' as u16 {
            p = p.next();
        }
        p = p.next();
    } else {
        while p.c() > b' ' as u16 {
            p = p.next();
        }
    }
    let mut buf = [0u16; 16];
    let (n, p) = next_arg(p, &mut buf);
    let arg = &buf[..n];
    settings_load();
    let rc = if arg.is_empty() {
        run_daemon()
    } else if is(arg, b"rect") || is(arg, b"window") || is(arg, b"full") {
        capture::one_shot(arg[0] as u8)
    } else if is(arg, b"rec-rect") {
        capture::rec_trigger(null_mut(), capture::RecKind::Rect)
    } else if is(arg, b"rec-window") {
        capture::rec_trigger(null_mut(), capture::RecKind::Window)
    } else if is(arg, b"rec-full") {
        capture::rec_trigger(null_mut(), capture::RecKind::Full)
    } else if is(arg, b"quit") || is(arg, b"stop") {
        quit_daemon()
    } else if is(arg, b"settings") {
        print_settings()
    } else if is(arg, b"set") {
        set_settings(p)
    } else {
        say(b"SnipSnap - copy or record a rectangle, a window or the screen.\n\n\
              \x20 snipsnap            run in the background (hotkeys below)\n\
              \x20 snipsnap rect       select a rectangle once (Esc / right-click cancels)\n\
              \x20 snipsnap window     pick a window once (click it; Esc / right-click cancels)\n\
              \x20 snipsnap full       capture the monitor under the cursor once\n\
              \x20 snipsnap rec-rect   record a rectangle (spawns snipsnap-rec.exe)\n\
              \x20 snipsnap rec-window record a window (spawns snipsnap-rec.exe)\n\
              \x20 snipsnap rec-full   record the monitor under the cursor\n\
              \x20 snipsnap quit       stop the background instance\n\n\
              Hotkeys: Ctrl+Alt+R/W/F = screenshot (fallbacks: Ctrl+PrtSc, Ctrl+Shift+PrtSc)\n\
              \x20        Ctrl+Alt+Shift+R/W/F = record (press again to stop)\n");
        if is(arg, b"help") || is(arg, b"h") || is(arg, b"?") { RC_OK } else { RC_FAIL }
    };
    rc as u32
}

/// The module's instance handle, for window classes and windows.
fn instance() -> *mut c_void {
    unsafe { GetModuleHandleW(null()) }
}
