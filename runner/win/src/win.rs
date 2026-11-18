//! The Win32 pieces both windows use (C#'s `Windows.cs`).
use core::cell::UnsafeCell;
use core::ptr::null;
use pk::wide::to_wide;
use runner::SUITE_ICON;
use runner_core::settings::TITLE;
use windows_sys::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::GetSysColorBrush;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    HICON, IDC_ARROW, IMAGE_ICON, IsIconic, LR_LOADFROMFILE, LoadCursorW, LoadImageW, MB_ICONERROR, MessageBoxW, RegisterClassW,
    SW_RESTORE, SW_SHOW, SetForegroundWindow, ShowWindow, WNDCLASSW,
};

pub type WindowProc = unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT;

/// State that only the UI thread touches (every window procedure runs there), in a static.
pub struct UiOnly<T>(UnsafeCell<T>);

// Only the UI thread ever reaches the value.
unsafe impl<T> Sync for UiOnly<T> {}

impl<T> UiOnly<T> {
    pub const fn new(value: T) -> Self {
        UiOnly(UnsafeCell::new(value))
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.0.get() }
    }
}

pub fn module() -> HINSTANCE {
    unsafe { GetModuleHandleW(null()) }
}

/// Registers a window class with the arrow cursor and, when given, a system colour's brush.
pub fn register(name: &str, procedure: WindowProc, background: Option<i32>) -> runner_core::Result<()> {
    let name = to_wide(name);
    let class = WNDCLASSW {
        style: 0,
        lpfnWndProc: Some(procedure),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: module(),
        hIcon: core::ptr::null_mut(),
        hCursor: unsafe { LoadCursorW(core::ptr::null_mut(), IDC_ARROW) },
        hbrBackground: background.map_or(core::ptr::null_mut(), |c| unsafe { GetSysColorBrush(c) }),
        lpszMenuName: null(),
        lpszClassName: name.as_ptr(),
    };
    if unsafe { RegisterClassW(&class) } == 0 {
        return Err("Could not register the PocketKnives window.".into());
    }
    Ok(())
}

/// The suite icon at `size` pixels, from this exe's resources (null if it can't load).
pub fn suite_icon(size: i32) -> HICON {
    unsafe { LoadImageW(module(), SUITE_ICON as usize as *const u16, IMAGE_ICON, size, size, 0) as HICON }
}

/// An icon file at `size` pixels (null if it can't load).
pub fn file_icon(path: &str, size: i32) -> HICON {
    unsafe { LoadImageW(core::ptr::null_mut(), to_wide(path).as_ptr(), IMAGE_ICON, size, size, LR_LOADFROMFILE) as HICON }
}

/// Shows `window` (restored if minimised) in front.
pub fn activate(window: HWND) {
    unsafe {
        ShowWindow(window, if IsIconic(window) != 0 { SW_RESTORE } else { SW_SHOW });
        SetForegroundWindow(window);
    }
}

/// An error box titled PocketKnives Settings.
pub fn error(owner: HWND, message: &str) {
    unsafe { MessageBoxW(owner, to_wide(message).as_ptr(), to_wide(TITLE).as_ptr(), MB_ICONERROR) };
}
