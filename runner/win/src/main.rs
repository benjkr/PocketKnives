//! pk.exe, the PocketKnives runner (docs/specs/runner.md): the suite's tray icon, and the
//! settings window it opens. C#'s `Program.cs`.
//!
//! ```text
//! pk [--preview] [settings | --exit]
//! ```
//!
//! - [`tray`]: the notification-area icon.
//! - [`native`]: the XP and Windows 10 settings window. The Windows 11 look is a separate app,
//!   `winui\PocketKnives.Settings.exe`, started here and given this runner's window.
//! - [`win`]: the Win32 pieces both use.
//!
//! Exit codes: 0, or 2 for a bad command line, a runner that didn't answer, or one that couldn't
//! start.
#![no_std]
#![no_main]

extern crate alloc;

mod native;
mod tray;
mod win;

use alloc::string::ToString;
use core::cell::{Cell, OnceCell, RefCell};
use core::ffi::c_void;
use core::ptr::{null, null_mut};
use core::sync::atomic::{AtomicPtr, Ordering};
use pk::wide::to_wide;
use runner::{Command, TRAY_IDENTITY};
use runner_core::link::{self, EXIT_MESSAGE, OPEN_MESSAGE, REOPEN_MESSAGE};
use runner_core::process::{self, Child};
use runner_core::settings::{Look, PREVIEW_VARIABLE, Suite};
use runner_core::{Result, join};
use tray::TrayIcon;
use win::UiOnly;
use windows_sys::Win32::Foundation::{ERROR_ALREADY_EXISTS, GetLastError, HANDLE, HWND, LPARAM, LRESULT, TRUE, WAIT_OBJECT_0, WPARAM};
use windows_sys::Win32::System::Environment::SetEnvironmentVariableW;
use windows_sys::Win32::System::Threading::{
    CreateEventW, CreateMutexW, INFINITE, RegisterWaitForSingleObject, ReleaseMutex, ResetEvent, SetEvent, UnregisterWait,
    WT_EXECUTEONLYONCE, WaitForSingleObject,
};
use windows_sys::Win32::UI::Shell::NIN_SELECT;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    AllowSetForegroundWindow, AppendMenuW, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyMenu, DestroyWindow, DispatchMessageW,
    EnumWindows, FindWindowW, GW_OWNER, GWL_STYLE, GetMessageW, GetWindow, GetWindowLongW, GetWindowThreadProcessId, IsDialogMessageW,
    IsWindowVisible, KillTimer, MF_STRING, MSG, PostMessageW, PostQuitMessage, RegisterWindowMessageW, SetForegroundWindow,
    SetMenuDefaultItem, SetTimer, TPM_RETURNCMD, TPM_RIGHTBUTTON, TrackPopupMenuEx, TranslateMessage, WM_APP, WM_CLOSE, WM_CONTEXTMENU,
    WM_DESTROY, WM_SYSDEADCHAR, WM_SYSKEYDOWN, WM_THEMECHANGED, WM_TIMER, WS_DISABLED, WS_POPUP,
};

pk::entry!(main);

/// The Windows 11 app ended (its process ID in wParam), posted from a thread-pool thread.
const CHILD_EXITED: u32 = WM_APP + 2;
/// Space or Enter on the focused icon (shellapi.h: `NIN_SELECT | NINF_KEY`).
const NIN_KEYSELECT: u32 = NIN_SELECT + 1;
const TRAY_RETRY: usize = 1;
const STOP_RETRY: usize = 2;
const SETTINGS_ITEM: usize = 1;
const EXIT_ITEM: usize = 2;

fn main() -> u32 {
    let (preview, command) = runner::parse(&pk::args::args());
    // Everything this runner starts (the blades' settings commands, the Windows 11 app) sees it.
    let value = to_wide("1");
    unsafe { SetEnvironmentVariableW(to_wide(PREVIEW_VARIABLE).as_ptr(), if preview { value.as_ptr() } else { null() }) };
    let Some(command) = command else {
        return 2;
    };

    let root = pk::env::exe_dir();
    let identity = if preview { runner::preview_identity(&root) } else { TRAY_IDENTITY };
    let instance = runner::instance_name(preview, &identity);
    let mutex = unsafe { CreateMutexW(null(), TRUE, to_wide(&alloc::format!("Local\\{instance}")).as_ptr()) };
    if mutex.is_null() {
        return 2;
    }
    let first = unsafe { GetLastError() } != ERROR_ALREADY_EXISTS;
    let ready = unsafe { CreateEventW(null(), TRUE, 0, to_wide(&alloc::format!("Local\\{instance}.Ready")).as_ptr()) };
    if ready.is_null() {
        return 2;
    }
    if !first {
        return match command {
            Command::Run => 0,
            _ if unsafe { WaitForSingleObject(ready, 5000) } != WAIT_OBJECT_0 => 2,
            _ => {
                let host = unsafe { FindWindowW(to_wide(&instance).as_ptr(), null()) };
                let mut pid = 0;
                if command == Command::Settings && unsafe { GetWindowThreadProcessId(host, &mut pid) } != 0 {
                    unsafe { AllowSetForegroundWindow(pid) };
                }
                if link::post(host, if command == Command::Exit { EXIT_MESSAGE } else { OPEN_MESSAGE }) { 0 } else { 2 }
            }
        };
    }

    let code = if command == Command::Exit {
        0
    } else {
        // The settings read the preview switch from this, never from the environment.
        let app = App::new(Suite::new(root, preview));
        let app = match APP.get().set(app) {
            Ok(()) => APP.get().get().expect("set"),
            Err(_) => return 2,
        };
        match app.run(&instance, identity, ready, command == Command::Settings) {
            Ok(code) => code,
            Err(message) => {
                win::error(null_mut(), &message);
                2
            }
        }
    };
    unsafe {
        ResetEvent(ready);
        ReleaseMutex(mutex);
    }
    code
}

/// The running Windows 11 app, and the wait that posts [`CHILD_EXITED`] when it ends.
struct WinUi {
    child: Child,
    wait: HANDLE,
}

impl WinUi {
    fn has_exited(&self) -> bool {
        (unsafe { WaitForSingleObject(self.child.handle(), 0) }) == WAIT_OBJECT_0
    }
}

impl Drop for WinUi {
    fn drop(&mut self) {
        // Before the process handle closes (the child drops after this).
        unsafe { UnregisterWait(self.wait) };
    }
}

/// The runner, made once (C#'s `Program`).
struct App {
    suite: Suite,
    taskbar_created: u32,
    open: u32,
    reopen: u32,
    exit: u32,
    window: Cell<HWND>,
    tray: RefCell<Option<TrayIcon>>,
    win_ui: RefCell<Option<WinUi>>,
    reopen_after_exit: Cell<bool>,
    stopping: Cell<bool>,
}

static APP: UiOnly<OnceCell<App>> = UiOnly::new(OnceCell::new());
/// The runner's window, for the thread-pool callback that reports the Windows 11 app's exit.
static WINDOW: AtomicPtr<c_void> = AtomicPtr::new(null_mut());

impl App {
    fn new(suite: Suite) -> App {
        App {
            suite,
            taskbar_created: unsafe { RegisterWindowMessageW(pk::w!("TaskbarCreated")) },
            open: link::register(OPEN_MESSAGE),
            reopen: link::register(REOPEN_MESSAGE),
            exit: link::register(EXIT_MESSAGE),
            window: Cell::new(null_mut()),
            tray: RefCell::new(None),
            win_ui: RefCell::new(None),
            reopen_after_exit: Cell::new(false),
            stopping: Cell::new(false),
        }
    }

    fn window(&self) -> HWND {
        self.window.get()
    }

    fn run(&'static self, instance: &str, identity: windows_sys::core::GUID, ready: HANDLE, show_settings: bool) -> Result<u32> {
        win::register(instance, window_procedure, None)?;
        native::initialize()?;
        // A hidden top-level window receives Explorer's TaskbarCreated broadcast.
        let window = unsafe {
            CreateWindowExW(0, to_wide(instance).as_ptr(), pk::w!(""), WS_POPUP, 0, 0, 0, 0, null_mut(), null_mut(), win::module(), null())
        };
        if window.is_null() {
            return Err("Could not start the PocketKnives tray.".into());
        }
        self.window.set(window);
        WINDOW.store(window, Ordering::Release);
        *self.tray.borrow_mut() = Some(TrayIcon::new(window, identity));
        self.refresh_tray();
        unsafe { SetEvent(ready) };
        if show_settings {
            self.open_settings(None);
        }

        let mut message: MSG = unsafe { core::mem::zeroed() };
        let result = loop {
            let result = unsafe { GetMessageW(&mut message, null_mut(), 0, 0) };
            if result <= 0 {
                break result;
            }
            let settings = native::handle();
            let hotkey_box = (WM_SYSKEYDOWN..=WM_SYSDEADCHAR).contains(&message.message) && native::owns_keyboard(message.hwnd);
            if !settings.is_null() && !hotkey_box && unsafe { IsDialogMessageW(settings, &message) } != 0 {
                continue;
            }
            unsafe {
                TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        };
        drop(self.tray.borrow_mut().take());
        Ok(if result < 0 { 2 } else { 0 })
    }

    fn refresh_tray(&self) {
        // Busy: Explorer's broadcast arrived while the icon is being added (a cross-process send
        // waits for it there); that registration carries on.
        let Ok(mut tray) = self.tray.try_borrow_mut() else {
            return;
        };
        if tray.as_mut().is_some_and(TrayIcon::register) {
            unsafe { KillTimer(self.window(), TRAY_RETRY) };
        } else {
            unsafe { SetTimer(self.window(), TRAY_RETRY, 2000, None) }; // Explorer may not be ready yet at sign-in.
        }
    }

    fn open_settings(&'static self, look: Option<Look>) {
        if self.stopping.get() {
            return;
        }
        if let Err(message) = self.try_open_settings(look) {
            win::error(self.window(), &message);
        }
    }

    fn try_open_settings(&'static self, look: Option<Look>) -> Result<()> {
        let settings = native::handle();
        if !settings.is_null() {
            win::activate(settings);
            return Ok(());
        }
        let running = self.win_ui.borrow().as_ref().map(|ui| (ui.has_exited(), ui.child.id));
        if let Some((exited, id)) = running {
            if exited {
                self.reopen_after_exit.set(true);
            } else {
                let window = main_window(id);
                if !window.is_null() {
                    win::activate(window);
                } else {
                    unsafe { AllowSetForegroundWindow(id) };
                }
            }
            return Ok(());
        }

        let look = match look {
            Some(look) => look,
            None => self.suite.read_look()?,
        };
        if look != Look::Windows11 { native::open(&self.suite, look) } else { self.start_win_ui() }
    }

    fn start_win_ui(&'static self) -> Result<()> {
        let exe = self.suite.winui_exe();
        if !runner_core::exists(&exe) {
            win::error(self.window(), "The Windows 11 settings window is missing. Opening the Windows 10 look.");
            return native::open(&self.suite, Look::Windows10);
        }
        let window = (self.window() as isize).to_string();
        let child = process::start_in(&exe, &[&window], &join(&self.suite.root, "winui"))?;
        let id = child.id;
        unsafe { AllowSetForegroundWindow(id) };
        let mut wait = null_mut();
        unsafe {
            RegisterWaitForSingleObject(
                &mut wait,
                child.handle(),
                Some(child_exited),
                id as usize as *const c_void,
                INFINITE,
                WT_EXECUTEONLYONCE,
            )
        };
        *self.win_ui.borrow_mut() = Some(WinUi { child, wait });
        Ok(())
    }

    fn on_child_exited(&'static self, id: usize) {
        let ui = {
            let mut win_ui = self.win_ui.borrow_mut();
            if !win_ui.as_ref().is_some_and(|ui| ui.child.id as usize == id) {
                return;
            }
            win_ui.take()
        };
        let failed = ui.as_ref().and_then(|ui| ui.child.wait(0)) != Some(0);
        drop(ui);
        let requested = self.reopen_after_exit.replace(false);
        if self.stopping.get() {
            unsafe { DestroyWindow(self.window()) };
        } else if requested {
            self.open_settings(None);
        } else if failed {
            win::error(self.window(), "The Windows 11 settings window closed unexpectedly. Opening the Windows 10 look.");
            self.open_settings(Some(Look::Windows10));
        }
    }

    fn show_menu(&'static self, x: i32, y: i32) {
        let menu = unsafe { CreatePopupMenu() };
        if menu.is_null() {
            return;
        }
        let choice = unsafe {
            AppendMenuW(menu, MF_STRING, SETTINGS_ITEM, pk::w!("&Settings"));
            AppendMenuW(menu, MF_STRING, EXIT_ITEM, pk::w!("E&xit"));
            SetMenuDefaultItem(menu, SETTINGS_ITEM as u32, 0);
            // Without the foreground, the menu doesn't close when the user clicks elsewhere.
            SetForegroundWindow(self.window());
            let choice = TrackPopupMenuEx(menu, TPM_RETURNCMD | TPM_RIGHTBUTTON, x, y, self.window(), null());
            DestroyMenu(menu);
            choice as usize
        };
        if choice == SETTINGS_ITEM {
            self.open_settings(None);
        } else if choice == EXIT_ITEM {
            self.suite.stop_blades();
            self.stop();
        }
    }

    /// Closes Settings, the icon and this runner (once the Windows 11 app, if open, has closed).
    fn stop(&self) {
        self.stopping.set(true);
        if let Ok(mut tray) = self.tray.try_borrow_mut()
            && let Some(tray) = tray.as_mut()
        {
            tray.remove();
        }
        unsafe { KillTimer(self.window(), TRAY_RETRY) };
        native::close_window();
        let running = self.win_ui.borrow().as_ref().map(|ui| (ui.has_exited(), ui.child.id));
        match running {
            None => unsafe {
                DestroyWindow(self.window());
            },
            Some((false, id)) => {
                if close_main_window(id) {
                    unsafe { KillTimer(self.window(), STOP_RETRY) };
                } else {
                    unsafe { SetTimer(self.window(), STOP_RETRY, 200, None) }; // Wait for WinUI to create its HWND before requesting close.
                }
            }
            // Its exit is on its way: CHILD_EXITED ends the runner.
            Some((true, _)) => {}
        }
    }
}

/// The look changed in the XP or Windows 10 window: close it and open Settings again.
fn reopen_native_settings() {
    native::close_window();
    if let Some(app) = APP.get().get() {
        link::post(app.window(), OPEN_MESSAGE);
    }
}

/// The process's main window as .NET's `Process.MainWindowHandle` finds it: its first unowned,
/// visible top-level window, or null.
fn main_window(id: u32) -> HWND {
    unsafe extern "system" fn each(window: HWND, found: LPARAM) -> windows_sys::core::BOOL {
        let found = unsafe { &mut *(found as *mut (u32, HWND)) };
        let mut pid = 0;
        unsafe { GetWindowThreadProcessId(window, &mut pid) };
        if pid == found.0 && unsafe { GetWindow(window, GW_OWNER) }.is_null() && unsafe { IsWindowVisible(window) } != 0 {
            found.1 = window;
            return 0;
        }
        1
    }
    let mut found: (u32, HWND) = (id, null_mut());
    unsafe { EnumWindows(Some(each), &mut found as *mut (u32, HWND) as LPARAM) };
    found.1
}

/// .NET's `Process.CloseMainWindow`: WM_CLOSE to its main window; false if it has none (yet) or
/// that window is disabled.
fn close_main_window(id: u32) -> bool {
    let window = main_window(id);
    if window.is_null() || unsafe { GetWindowLongW(window, GWL_STYLE) } as u32 & WS_DISABLED != 0 {
        return false;
    }
    unsafe { PostMessageW(window, WM_CLOSE, 0, 0) };
    true
}

/// The Windows 11 app's process ended (on a thread-pool thread): tell the UI thread.
unsafe extern "system" fn child_exited(id: *mut c_void, _: bool) {
    unsafe { PostMessageW(WINDOW.load(Ordering::Acquire), CHILD_EXITED, id as usize, 0) };
}

unsafe extern "system" fn window_procedure(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let Some(app) = APP.get().get().filter(|app| app.window() == window) else {
        return unsafe { DefWindowProcW(window, message, wparam, lparam) };
    };
    if message == WM_CLOSE || message == app.exit || (message == WM_TIMER && wparam == STOP_RETRY) {
        app.stop();
        return 0;
    }
    if message == app.open {
        app.open_settings(None);
        return 0;
    }
    if message == app.reopen {
        app.reopen_after_exit.set(true);
        return 0;
    }
    if message == CHILD_EXITED {
        app.on_child_exited(wparam);
        return 0;
    }
    if !app.stopping.get()
        && (message == app.taskbar_created || message == WM_THEMECHANGED || (message == WM_TIMER && wparam == TRAY_RETRY))
    {
        app.refresh_tray();
        return 0;
    }
    if message == tray::CALLBACK {
        // NOTIFYICON_VERSION_4 packs the event in LOWORD(lParam).
        let event = (lparam & 0xffff) as u32;
        if event == NIN_SELECT || event == NIN_KEYSELECT {
            app.open_settings(None);
        } else if event == WM_CONTEXTMENU {
            // Right-click, Shift+F10 or the menu key; the anchor is in wParam.
            app.show_menu((wparam & 0xffff) as i16 as i32, (wparam >> 16 & 0xffff) as i16 as i32);
        }
        return 0;
    }
    if message == WM_DESTROY {
        unsafe { PostQuitMessage(0) };
        return 0;
    }
    unsafe { DefWindowProcW(window, message, wparam, lparam) }
}
