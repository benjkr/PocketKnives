//! The suite's notification-area icon (C#'s `TrayIcon.cs`), identified by its GUID so Windows
//! keeps the user's placement.
use crate::win;
use runner_core::settings::TITLE;
use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::UI::HiDpi::{GetDpiForSystem, GetSystemMetricsForDpi};
use windows_sys::Win32::UI::Shell::{
    NIF_GUID, NIF_ICON, NIF_MESSAGE, NIF_SHOWTIP, NIF_TIP, NIM_ADD, NIM_DELETE, NIM_SETVERSION, NOTIFYICON_VERSION_4, NOTIFYICONDATAW,
    Shell_NotifyIconW,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{DestroyIcon, SM_CXSMICON, WM_APP};
use windows_sys::core::GUID;

/// The icon's callback message.
pub const CALLBACK: u32 = WM_APP + 1;

pub struct TrayIcon {
    data: NOTIFYICONDATAW,
    registered: bool,
}

impl TrayIcon {
    pub fn new(window: HWND, identity: GUID) -> TrayIcon {
        let mut data: NOTIFYICONDATAW = unsafe { core::mem::zeroed() };
        data.cbSize = size_of::<NOTIFYICONDATAW>() as u32;
        data.hWnd = window;
        data.uID = 1;
        data.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP | NIF_GUID | NIF_SHOWTIP;
        data.uCallbackMessage = CALLBACK;
        data.guidItem = identity;
        data.Anonymous.uVersion = NOTIFYICON_VERSION_4;
        for (d, s) in data.szTip.iter_mut().zip(TITLE.encode_utf16()) {
            *d = s;
        }
        TrayIcon { data, registered: false }
    }

    /// Adds the icon afresh; false if Explorer isn't ready (or the icon can't load), to retry.
    pub fn register(&mut self) -> bool {
        self.remove();
        let size = unsafe { GetSystemMetricsForDpi(SM_CXSMICON, GetDpiForSystem()) };
        self.data.hIcon = win::suite_icon(if size >= 32 { 32 } else { 16 });
        if self.data.hIcon.is_null() {
            return false;
        }
        // A runner that was terminated (Setup force-closes one after 10 s) leaves its GUID icon
        // registered, and NIM_ADD with that GUID then fails. Deleting a missing icon is harmless.
        unsafe { Shell_NotifyIconW(NIM_DELETE, &self.data) };
        self.registered = unsafe { Shell_NotifyIconW(NIM_ADD, &self.data) } != 0;
        if self.registered && unsafe { Shell_NotifyIconW(NIM_SETVERSION, &self.data) } != 0 {
            return true;
        }
        self.remove();
        false
    }

    /// Takes the icon out of the notification area.
    pub fn remove(&mut self) {
        if self.registered {
            unsafe { Shell_NotifyIconW(NIM_DELETE, &self.data) };
        }
        self.registered = false;
        if !self.data.hIcon.is_null() {
            unsafe { DestroyIcon(self.data.hIcon) };
        }
        self.data.hIcon = core::ptr::null_mut();
    }
}

impl Drop for TrayIcon {
    fn drop(&mut self) {
        self.remove();
    }
}
