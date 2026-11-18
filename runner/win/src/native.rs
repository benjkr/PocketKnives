//! The XP and Windows 10 settings window (C#'s `NativeSettings.cs`): classic Win32 controls, with
//! visual styles for Windows 10. Positions are in 96-DPI pixels, scaled to the system DPI.
use crate::win::{self, UiOnly};
use alloc::format;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};
use core::ptr::{null, null_mut};
use pk::wide::to_wide;
use runner::{hotkey_box, hotkey_value};
use runner_core::Result;
use runner_core::blade_settings::{self, Kind, Setting};
use runner_core::settings::{BLADES, BLADES_LABEL, GENERAL_DESCRIPTION, GENERAL_NAME, LOOK_LABEL, LOOK_NAMES, Look, Startup, Suite, TITLE};
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows_sys::Win32::Graphics::Dwm::{DWMWA_WINDOW_CORNER_PREFERENCE, DwmSetWindowAttribute};
use windows_sys::Win32::Graphics::Gdi::{
    COLOR_BTNFACE, COLOR_WINDOW, CreateBitmap, CreateCompatibleDC, CreateFontIndirectW, DFC_BUTTON, DFCS_CHECKED, DFCS_FLAT, DeleteDC,
    DeleteObject, DrawFrameControl, FillRect, GetSysColorBrush, HFONT, SelectObject,
};
use windows_sys::Win32::UI::Controls::{
    HIMAGELIST, HKM_GETHOTKEY, HKM_SETHOTKEY, HKM_SETRULES, ICC_LISTVIEW_CLASSES, ILC_COLOR24, ILC_COLOR32, INITCOMMONCONTROLSEX,
    ImageList_Add, ImageList_Create, ImageList_Destroy, ImageList_ReplaceIcon, InitCommonControlsEx, LVCF_WIDTH, LVCOLUMNW, LVIF_IMAGE,
    LVIF_TEXT, LVITEMW, LVM_GETITEMSTATE, LVM_INSERTCOLUMNW, LVM_INSERTITEMW, LVM_SETEXTENDEDLISTVIEWSTYLE, LVM_SETIMAGELIST,
    LVM_SETITEMSTATE, LVN_ITEMCHANGED, NMHDR, NMLISTVIEW, SetWindowTheme,
};
use windows_sys::Win32::UI::HiDpi::{AdjustWindowRectExForDpi, GetDpiForSystem, SystemParametersInfoForDpi};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::EnableWindow;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    BM_GETCHECK, BM_SETCHECK, CB_ADDSTRING, CB_GETCURSEL, CB_SETCURSEL, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DestroyIcon,
    DestroyWindow, HICON, HMENU, NONCLIENTMETRICSW, SPI_GETNONCLIENTMETRICS, STM_SETICON, SW_HIDE, SW_SHOW, SendMessageW, SetWindowTextW,
    ShowWindow, WM_CLOSE, WM_COMMAND, WM_NOTIFY, WM_SETFONT, WM_SETICON, WS_BORDER, WS_CAPTION, WS_CHILD, WS_EX_CLIENTEDGE,
    WS_EX_CONTROLPARENT, WS_EX_DLGMODALFRAME, WS_MINIMIZEBOX, WS_SYSMENU, WS_TABSTOP, WS_VISIBLE, WS_VSCROLL,
};

const CLASS_NAME: &str = "PocketKnives.Settings.Win32";
const OK_ID: usize = 1;
const CANCEL_ID: usize = 2;
const APPLY_ID: usize = 3;
const LOOK_ID: usize = 100;
const FIRST_SETTING_ID: usize = 200;
const PAGES: usize = BLADES.len() + 1;

// Styles and states as winuser.h and commctrl.h spell them.
const BS_DEFPUSHBUTTON: u32 = 0x1;
const BS_AUTOCHECKBOX: u32 = 0x3;
const BS_FLAT: u32 = 0x8000;
const CBS_DROPDOWNLIST: u32 = 0x3;
const SS_ICON: u32 = 0x3;
const LVS_REPORT: u32 = 0x1;
const LVS_SINGLESEL: u32 = 0x4;
const LVS_SHOWSELALWAYS: u32 = 0x8;
const LVS_NOCOLUMNHEADER: u32 = 0x4000;
const LVS_EX_FULLROWSELECT: u32 = 0x20;
const LVS_EX_CHECKBOXES: u32 = 0x4;
const LVSIL_SMALL: usize = 1;
const LVSIL_STATE: usize = 2;
const LVIS_FOCUSED: u32 = 1;
const LVIS_SELECTED: u32 = 2;
const LVIS_STATEIMAGEMASK: u32 = 0xf000;
const HKCOMB_NONE: usize = 0x1;
const HKCOMB_S: usize = 0x2;
const HOTKEYF_CONTROL: isize = 0x2;
const HOTKEYF_ALT: isize = 0x4;
const BN_CLICKED: u32 = 0;
const CBN_SELCHANGE: u32 = 1;
const EN_CHANGE: u32 = 0x300;
const DWMWCP_DONOTROUND: i32 = 1;

struct Native {
    suite: &'static Suite,
    look: Look,
    classic: bool,
    dpi: u32,
    window: Cell<HWND>,
    font: Cell<HFONT>,
    bold_font: Cell<HFONT>,
    icon_list: Cell<HIMAGELIST>,
    general_list: Cell<HWND>,
    blade_list: Cell<HWND>,
    apply_button: Cell<HWND>,
    page_title: Cell<HWND>,
    description: Cell<HWND>,
    page_icon: Cell<HWND>,
    look_choice: Cell<HWND>,
    /// Every icon this window loaded, destroyed with it.
    icons: RefCell<Vec<HICON>>,
    page_icons: RefCell<[HICON; PAGES]>,
    /// Every page's own controls, General's first; `show_page` shows one page's, hides the rest.
    page_controls: RefCell<[Vec<HWND>; PAGES]>,
    /// One entry per blade setting (blade, index, control), in control ID order from
    /// `FIRST_SETTING_ID`.
    setting_controls: RefCell<Vec<(usize, usize, HWND)>>,
    blade_settings: Vec<Vec<Setting>>,
    /// What each blade has now: its values when the window opened, then what was applied.
    saved: RefCell<Vec<Vec<String>>>,
    /// The sign-in rows as they are now (as opened, then as applied).
    original: RefCell<Startup>,
}

static CURRENT: UiOnly<RefCell<Option<Rc<Native>>>> = UiOnly::new(RefCell::new(None));

fn current() -> Option<Rc<Native>> {
    CURRENT.get().borrow().clone()
}

/// The open window, or null.
pub fn handle() -> HWND {
    current().map_or(null_mut(), |s| s.window.get())
}

/// Alt+key belongs to a focused hotkey box, not to the dialog's access keys (&Apply).
pub fn owns_keyboard(control: HWND) -> bool {
    current().is_some_and(|s| {
        s.setting_controls.borrow().iter().any(|&(blade, index, c)| c == control && s.blade_settings[blade][index].kind == Kind::Hotkey)
    })
}

pub fn initialize() -> Result<()> {
    win::register(CLASS_NAME, window_procedure, Some(COLOR_BTNFACE))
}

/// Opens the window in `look` (XP or Windows 10), with each blade's settings read from it now.
pub fn open(suite: &'static Suite, look: Look) -> Result<()> {
    let mut blade_settings = Vec::new();
    for page in blade_settings::read_all(suite) {
        blade_settings.push(page?);
    }
    let original = suite.read_startup()?;
    let saved = blade_settings.iter().map(|page: &Vec<Setting>| page.iter().map(|s| s.value.clone()).collect()).collect();
    let settings = Rc::new(Native {
        suite,
        look,
        classic: look == Look::Xp,
        dpi: unsafe { GetDpiForSystem() },
        window: Cell::new(null_mut()),
        font: Cell::new(null_mut()),
        bold_font: Cell::new(null_mut()),
        icon_list: Cell::new(0),
        general_list: Cell::new(null_mut()),
        blade_list: Cell::new(null_mut()),
        apply_button: Cell::new(null_mut()),
        page_title: Cell::new(null_mut()),
        description: Cell::new(null_mut()),
        page_icon: Cell::new(null_mut()),
        look_choice: Cell::new(null_mut()),
        icons: RefCell::new(Vec::new()),
        page_icons: RefCell::new([null_mut(); PAGES]),
        page_controls: RefCell::new(Default::default()),
        setting_controls: RefCell::new(Vec::new()),
        blade_settings,
        saved: RefCell::new(saved),
        original: RefCell::new(original),
    });
    *CURRENT.get().borrow_mut() = Some(settings.clone());
    match settings.build() {
        Ok(()) => {
            win::activate(settings.window.get());
            Ok(())
        }
        Err(e) => {
            close_window();
            Err(e)
        }
    }
}

/// Closes the window, if open, and frees what it loaded.
pub fn close_window() {
    let Some(s) = CURRENT.get().borrow_mut().take() else {
        return;
    };
    unsafe {
        if !s.window.get().is_null() {
            // Both lists use the same small images. Detach them before destroying either list.
            SendMessageW(s.general_list.get(), LVM_SETIMAGELIST, LVSIL_SMALL, 0);
            SendMessageW(s.blade_list.get(), LVM_SETIMAGELIST, LVSIL_SMALL, 0);
            DestroyWindow(s.window.get());
        }
        if s.icon_list.get() != 0 {
            ImageList_Destroy(s.icon_list.get());
        }
        for &icon in s.icons.borrow().iter() {
            DestroyIcon(icon);
        }
        DeleteObject(s.font.get());
        DeleteObject(s.bold_font.get());
    }
}

fn send(window: HWND, message: u32, wparam: usize, lparam: isize) -> isize {
    unsafe { SendMessageW(window, message, wparam, lparam) }
}

impl Native {
    fn scale(&self, px: i32) -> i32 {
        ((px as i64 * self.dpi as i64 + 48) / 96) as i32
    }

    fn build(&self) -> Result<()> {
        let controls = INITCOMMONCONTROLSEX { dwSize: size_of::<INITCOMMONCONTROLSEX>() as u32, dwICC: ICC_LISTVIEW_CLASSES };
        unsafe { InitCommonControlsEx(&controls) };
        self.create_fonts();

        let style = WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX;
        let extended_style = WS_EX_CONTROLPARENT | WS_EX_DLGMODALFRAME;
        let mut bounds = RECT { left: 0, top: 0, right: self.scale(564), bottom: self.scale(262) };
        unsafe { AdjustWindowRectExForDpi(&mut bounds, style, 0, extended_style, self.dpi) };
        let window = unsafe {
            CreateWindowExW(
                extended_style,
                to_wide(CLASS_NAME).as_ptr(),
                to_wide(TITLE).as_ptr(),
                style,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                bounds.right - bounds.left,
                bounds.bottom - bounds.top,
                null_mut(),
                null_mut(),
                win::module(),
                null(),
            )
        };
        if window.is_null() {
            return Err("Could not create the settings window.".into());
        }
        self.window.set(window);

        send(window, WM_SETICON, 0, self.own(win::suite_icon(16)) as isize); // ICON_SMALL
        send(window, WM_SETICON, 1, self.own(win::suite_icon(32)) as isize); // ICON_BIG
        if self.classic {
            let empty = to_wide("");
            let square = DWMWCP_DONOTROUND;
            unsafe {
                SetWindowTheme(window, empty.as_ptr(), empty.as_ptr());
                DwmSetWindowAttribute(window, DWMWA_WINDOW_CORNER_PREFERENCE as u32, (&square as *const i32).cast(), 4);
            }
        }

        let size = self.scale(16);
        self.icon_list.set(unsafe { ImageList_Create(size, size, ILC_COLOR32, PAGES as i32, 1) });
        self.add_row_icon(self.own(win::suite_icon(if size >= 32 { 32 } else { 16 })));
        for blade in &BLADES {
            self.add_row_icon(self.own(win::file_icon(&self.suite.blade_icon(blade), size)));
        }

        let general_list = self.create_list(8, 8, 156, 28, false)?;
        self.general_list.set(general_list);
        add_row(general_list, 0, GENERAL_NAME, 0, 0);
        self.control("STATIC", BLADES_LABEL, 0, (8, 46, 156, 18), 0, true, 0)?;
        let blade_list = self.create_list(8, 64, 156, 128, true)?;
        self.blade_list.set(blade_list);
        let original = *self.original.borrow();
        for (i, blade) in BLADES.iter().enumerate() {
            add_row(blade_list, i, blade.name, i as i32 + 1, if original[i] { 2 } else { 1 });
        }
        self.control("STATIC", "Checked blades start when you sign in.", 0, (8, 198, 156, 36), 0, false, 0)?;

        self.page_title.set(self.control("STATIC", "", 0, (178, 8, 332, 18), 0, true, 0)?);
        self.description.set(self.control("STATIC", "", 0, (178, 28, 330, 38), 0, false, 0)?);
        self.page_icon.set(self.control("STATIC", "", SS_ICON, (522, 8, 32, 32), 0, false, 0)?);
        let look_label = self.control("STATIC", &format!("&{LOOK_LABEL}"), 0, (178, 70, 120, 18), 0, true, 0)?;
        let look_choice =
            self.control("COMBOBOX", "", WS_TABSTOP | WS_VSCROLL | CBS_DROPDOWNLIST, (178, 90, 200, 160), LOOK_ID, false, 0)?;
        self.look_choice.set(look_choice);
        for name in LOOK_NAMES {
            add_choice(look_choice, name);
        }
        send(look_choice, CB_SETCURSEL, self.look.index(), 0);
        self.page_controls.borrow_mut()[0].extend([look_label, look_choice]);
        for i in 0..BLADES.len() {
            self.build_blade_page(i)?;
        }

        let button_style = WS_TABSTOP | if self.classic { BS_FLAT } else { 0 };
        self.control("BUTTON", "OK", button_style | BS_DEFPUSHBUTTON, (322, 230, 75, 24), OK_ID, false, 0)?;
        self.control("BUTTON", "Cancel", button_style, (402, 230, 75, 24), CANCEL_ID, false, 0)?;
        let apply = self.control("BUTTON", "&Apply", button_style, (482, 230, 75, 24), APPLY_ID, false, 0)?;
        self.apply_button.set(apply);
        unsafe { EnableWindow(apply, 0) };
        set_state(general_list, 0, LVIS_SELECTED | LVIS_FOCUSED, LVIS_SELECTED | LVIS_FOCUSED);
        self.show_page(0);
        Ok(())
    }

    fn build_blade_page(&self, blade: usize) -> Result<()> {
        let settings = &self.blade_settings[blade];
        if settings.is_empty() {
            let text = format!("{} has no settings to show.", BLADES[blade].name);
            let label = self.control("STATIC", &text, 0, (178, 74, 330, 18), 0, false, 0)?;
            self.page_controls.borrow_mut()[blade + 1].push(label);
        }
        let flat = if self.classic { BS_FLAT } else { 0 };
        let mut y = 72;
        for (i, setting) in settings.iter().enumerate() {
            let id = FIRST_SETTING_ID + self.setting_controls.borrow().len();
            let control = if setting.kind == Kind::Toggle {
                let c = self.control("BUTTON", &setting.label, WS_TABSTOP | BS_AUTOCHECKBOX | flat, (178, y, 330, 22), id, false, 0)?;
                send(c, BM_SETCHECK, (setting.value == "1") as usize, 0);
                c
            } else {
                let label = self.control("STATIC", &setting.label, 0, (178, y + 3, 116, 18), 0, false, 0)?;
                self.page_controls.borrow_mut()[blade + 1].push(label);
                if setting.kind == Kind::Hotkey {
                    let border = if self.classic { WS_BORDER } else { 0 };
                    let edge = if self.classic { 0 } else { WS_EX_CLIENTEDGE };
                    let c = self.control("msctls_hotkey32", "", WS_TABSTOP | border, (298, y, 210, 22), id, false, edge)?;
                    // A key with no modifier, or Shift alone, would take that key from every program.
                    send(c, HKM_SETRULES, HKCOMB_NONE | HKCOMB_S, HOTKEYF_CONTROL | HOTKEYF_ALT); // they become Ctrl+Alt
                    send(c, HKM_SETHOTKEY, hotkey_box(&setting.value), 0);
                    c
                } else {
                    let c = self.control("COMBOBOX", "", WS_TABSTOP | WS_VSCROLL | CBS_DROPDOWNLIST, (298, y, 210, 200), id, false, 0)?;
                    for choice in &setting.choices {
                        add_choice(c, &choice.label);
                    }
                    let selected = setting.choices.iter().position(|c| c.value == setting.value);
                    send(c, CB_SETCURSEL, selected.unwrap_or(usize::MAX), 0);
                    c
                }
            };
            self.page_controls.borrow_mut()[blade + 1].push(control);
            self.setting_controls.borrow_mut().push((blade, i, control));
            y += 30;
        }
        Ok(())
    }

    /// The blade's value for what a setting's control holds now.
    fn read_setting(&self, blade: usize, index: usize, control: HWND) -> String {
        let setting = &self.blade_settings[blade][index];
        let before = self.saved.borrow()[blade][index].clone();
        match setting.kind {
            Kind::Toggle => String::from(if send(control, BM_GETCHECK, 0, 0) == 1 { "1" } else { "0" }),
            Kind::Choice => {
                let selected = send(control, CB_GETCURSEL, 0, 0);
                match usize::try_from(selected).ok().and_then(|i| setting.choices.get(i)) {
                    Some(choice) => choice.value.clone(),
                    None => before,
                }
            }
            Kind::Hotkey => hotkey_value(send(control, HKM_GETHOTKEY, 0, 0) as u32, &before),
        }
    }

    /// Each blade's changed settings (key, value), in [`BLADES`] order.
    fn read_changes(&self) -> Vec<Vec<(String, String)>> {
        let mut changes: Vec<Vec<(String, String)>> = BLADES.iter().map(|_| Vec::new()).collect();
        let controls = self.setting_controls.borrow().clone();
        for (blade, index, control) in controls {
            let value = self.read_setting(blade, index, control);
            if value != self.saved.borrow()[blade][index] {
                changes[blade].push((self.blade_settings[blade][index].key.clone(), value));
            }
        }
        changes
    }

    fn create_fonts(&self) {
        let mut metrics: NONCLIENTMETRICSW = unsafe { core::mem::zeroed() };
        metrics.cbSize = size_of::<NONCLIENTMETRICSW>() as u32;
        let p = (&mut metrics as *mut NONCLIENTMETRICSW).cast();
        if unsafe { SystemParametersInfoForDpi(SPI_GETNONCLIENTMETRICS, metrics.cbSize, p, 0, self.dpi) } == 0 {
            return;
        }
        self.font.set(unsafe { CreateFontIndirectW(&metrics.lfMessageFont) });
        metrics.lfMessageFont.lfWeight = 700; // FW_BOLD
        self.bold_font.set(unsafe { CreateFontIndirectW(&metrics.lfMessageFont) });
    }

    /// A child control at (x, y, width, height) in 96-DPI pixels, in the window's font.
    #[allow(clippy::too_many_arguments)]
    fn control(
        &self,
        class: &str,
        text: &str,
        style: u32,
        (x, y, width, height): (i32, i32, i32, i32),
        id: usize,
        bold: bool,
        extended_style: u32,
    ) -> Result<HWND> {
        let control = unsafe {
            CreateWindowExW(
                extended_style,
                to_wide(class).as_ptr(),
                to_wide(text).as_ptr(),
                WS_CHILD | WS_VISIBLE | style,
                self.scale(x),
                self.scale(y),
                self.scale(width),
                self.scale(height),
                self.window.get(),
                id as HMENU,
                win::module(),
                null(),
            )
        };
        if control.is_null() {
            return Err("Could not create a settings control.".into());
        }
        let font = if bold { self.bold_font.get() } else { self.font.get() };
        send(control, WM_SETFONT, font as usize, 0);
        if self.classic {
            let empty = to_wide("");
            unsafe { SetWindowTheme(control, empty.as_ptr(), empty.as_ptr()) };
        }
        Ok(control)
    }

    fn create_list(&self, x: i32, y: i32, width: i32, height: i32, checkboxes: bool) -> Result<HWND> {
        let list_style = LVS_REPORT | LVS_SINGLESEL | LVS_SHOWSELALWAYS | LVS_NOCOLUMNHEADER;
        let border = if self.classic { WS_BORDER } else { 0 };
        let edge = if self.classic { 0 } else { WS_EX_CLIENTEDGE };
        let list = self.control("SysListView32", "", WS_TABSTOP | list_style | border, (x, y, width, height), 0, false, edge)?;
        if !self.classic {
            unsafe { SetWindowTheme(list, to_wide("Explorer").as_ptr(), to_wide("").as_ptr()) };
        }
        let styles = LVS_EX_FULLROWSELECT | if checkboxes { LVS_EX_CHECKBOXES } else { 0 };
        send(list, LVM_SETEXTENDEDLISTVIEWSTYLE, styles as usize, styles as isize);
        if checkboxes && self.classic {
            // Windows themes the list's built-in checkboxes even after SetWindowTheme.
            let state_images = self.classic_checkboxes();
            if state_images != 0 {
                unsafe { ImageList_Destroy(send(list, LVM_SETIMAGELIST, LVSIL_STATE, state_images as isize) as HIMAGELIST) };
            }
        }
        send(list, LVM_SETIMAGELIST, LVSIL_SMALL, self.icon_list.get() as isize);
        let mut column: LVCOLUMNW = unsafe { core::mem::zeroed() };
        column.mask = LVCF_WIDTH;
        column.cx = self.scale(width - 6);
        send(list, LVM_INSERTCOLUMNW, 0, &column as *const LVCOLUMNW as isize);
        Ok(list)
    }

    /// Flat, unthemed check boxes for the XP list: unchecked, then checked.
    fn classic_checkboxes(&self) -> HIMAGELIST {
        let size = self.scale(16);
        let box_size = self.scale(13);
        let inset = (size - box_size) / 2;
        unsafe {
            let images = ImageList_Create(size, size, ILC_COLOR24, 2, 0);
            let dc = CreateCompatibleDC(null_mut());
            let bitmap = CreateBitmap(size, size, 1, 32, null());
            let previous = SelectObject(dc, bitmap);
            for state in [DFCS_FLAT, DFCS_FLAT | DFCS_CHECKED] {
                let background = RECT { left: 0, top: 0, right: size, bottom: size };
                FillRect(dc, &background, GetSysColorBrush(COLOR_WINDOW));
                let mut check = RECT { left: inset, top: inset, right: inset + box_size, bottom: inset + box_size };
                DrawFrameControl(dc, &mut check, DFC_BUTTON, state); // DFCS_BUTTONCHECK is 0
                ImageList_Add(images, bitmap, null_mut());
            }
            SelectObject(dc, previous);
            DeleteObject(bitmap);
            DeleteDC(dc);
            images
        }
    }

    fn add_row_icon(&self, icon: HICON) {
        let icon = if icon.is_null() { self.own(win::suite_icon(16)) } else { icon };
        if !icon.is_null() {
            unsafe { ImageList_ReplaceIcon(self.icon_list.get(), -1, icon) };
        }
    }

    /// Keeps `icon` to destroy with the window.
    fn own(&self, icon: HICON) -> HICON {
        if !icon.is_null() {
            self.icons.borrow_mut().push(icon);
        }
        icon
    }

    fn show_page(&self, page: usize) {
        let blade = page.checked_sub(1).map(|i| &BLADES[i]);
        let title = blade.map_or(GENERAL_NAME, |b| b.name);
        let description = blade.map_or(GENERAL_DESCRIPTION, |b| b.description);
        unsafe {
            SetWindowTextW(self.page_title.get(), to_wide(title).as_ptr());
            SetWindowTextW(self.description.get(), to_wide(description).as_ptr());
        }
        let fit = self.scale(32);
        let size = if fit >= 64 {
            64
        } else if fit >= 48 {
            48
        } else {
            32
        };
        if self.page_icons.borrow()[page].is_null() {
            let icon = match blade {
                None => win::suite_icon(size),
                Some(b) => win::file_icon(&self.suite.blade_icon(b), fit),
            };
            self.page_icons.borrow_mut()[page] = self.own(icon);
        }
        send(self.page_icon.get(), STM_SETICON, self.page_icons.borrow()[page] as usize, 0);
        let pages = self.page_controls.borrow().clone();
        for (i, controls) in pages.iter().enumerate() {
            for &control in controls {
                unsafe { ShowWindow(control, if i == page { SW_SHOW } else { SW_HIDE }) };
            }
        }
    }

    /// Which blade rows are checked.
    fn read_selection(&self) -> Startup {
        let mut rows = [false; BLADES.len()];
        for (i, row) in rows.iter_mut().enumerate() {
            *row = send(self.blade_list.get(), LVM_GETITEMSTATE, i, LVIS_STATEIMAGEMASK as isize) == 0x2000;
        }
        rows
    }

    fn update_apply(&self) {
        let unchanged = self.read_selection() == *self.original.borrow() && self.read_changes().iter().all(Vec::is_empty);
        unsafe { EnableWindow(self.apply_button.get(), (!unchanged) as i32) };
    }

    fn apply(&self) -> Result<()> {
        let result = self.apply_changes();
        self.update_apply();
        result
    }

    fn apply_changes(&self) -> Result<()> {
        let selection = self.read_selection();
        self.suite.apply_startup(&mut self.original.borrow_mut(), &selection)?;
        let changes = self.read_changes();
        for (blade, changes) in changes.iter().enumerate() {
            blade_settings::apply(self.suite, &BLADES[blade], changes)?;
            for (key, value) in changes {
                if let Some(index) = self.blade_settings[blade].iter().position(|s| &s.key == key) {
                    self.saved.borrow_mut()[blade][index] = value.clone();
                }
            }
        }
        Ok(())
    }

    fn execute(&self, id: usize, notification: u32) {
        let result = match id {
            OK_ID => self.apply().map(|()| close_window()),
            CANCEL_ID => {
                close_window();
                Ok(())
            }
            APPLY_ID => self.apply(),
            LOOK_ID if notification == CBN_SELCHANGE => self.switch_look(),
            _ if id >= FIRST_SETTING_ID
                && id - FIRST_SETTING_ID < self.setting_controls.borrow().len()
                && matches!(notification, BN_CLICKED | CBN_SELCHANGE | EN_CHANGE) =>
            {
                self.update_apply();
                Ok(())
            }
            _ => Ok(()),
        };
        if let Err(message) = result {
            send(self.look_choice.get(), CB_SETCURSEL, self.look.index(), 0);
            self.update_apply();
            win::error(self.window.get(), &message);
        }
    }

    /// The look box changed: apply pending changes, save the look and reopen Settings in it.
    fn switch_look(&self) -> Result<()> {
        let selected = send(self.look_choice.get(), CB_GETCURSEL, 0, 0);
        if let Some(look) = usize::try_from(selected).ok().and_then(Look::from_index)
            && look != self.look
        {
            self.apply()?;
            self.suite.save_look(look)?;
            crate::reopen_native_settings();
        }
        Ok(())
    }

    /// A `WM_NOTIFY`: a list row's selection or check box changed. False for anything else.
    fn item_changed(&self, header: &NMHDR) -> bool {
        let (general, blades) = (self.general_list.get(), self.blade_list.get());
        if header.code != LVN_ITEMCHANGED {
            return false;
        }
        // An LVN_ITEMCHANGED header starts an NMLISTVIEW.
        let change = unsafe { &*(header as *const NMHDR).cast::<NMLISTVIEW>() };
        if change.iItem < 0 || (header.hwndFrom != general && header.hwndFrom != blades) {
            return false;
        }
        if change.uNewState & LVIS_SELECTED != 0 && change.uOldState & LVIS_SELECTED == 0 {
            let is_general = header.hwndFrom == general;
            set_state(if is_general { blades } else { general }, usize::MAX, 0, LVIS_SELECTED | LVIS_FOCUSED);
            self.show_page(if is_general { 0 } else { change.iItem as usize + 1 });
        }
        if (change.uNewState ^ change.uOldState) & LVIS_STATEIMAGEMASK != 0 {
            self.update_apply();
        }
        true
    }
}

fn add_choice(combo: HWND, text: &str) {
    send(combo, CB_ADDSTRING, 0, to_wide(text).as_ptr() as isize);
}

fn add_row(list: HWND, row: usize, name: &str, image: i32, checkbox: u32) {
    let text = to_wide(name);
    let mut item: LVITEMW = unsafe { core::mem::zeroed() };
    item.mask = LVIF_TEXT | LVIF_IMAGE;
    item.iItem = row as i32;
    item.pszText = text.as_ptr() as *mut u16;
    item.iImage = image;
    send(list, LVM_INSERTITEMW, 0, &item as *const LVITEMW as isize);
    set_state(list, row, checkbox << 12, LVIS_STATEIMAGEMASK);
}

/// Sets a row's state bits under `mask`; `usize::MAX` is every row.
fn set_state(list: HWND, row: usize, state: u32, mask: u32) {
    let mut item: LVITEMW = unsafe { core::mem::zeroed() };
    item.state = state;
    item.stateMask = mask;
    send(list, LVM_SETITEMSTATE, row, &item as *const LVITEMW as isize);
}

unsafe extern "system" fn window_procedure(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let Some(settings) = current().filter(|s| s.window.get() == window) else {
        return unsafe { DefWindowProcW(window, message, wparam, lparam) };
    };
    match message {
        WM_CLOSE => {
            close_window();
            return 0;
        }
        WM_COMMAND => {
            settings.execute(wparam & 0xffff, (wparam >> 16 & 0xffff) as u32);
            return 0;
        }
        WM_NOTIFY if !settings.apply_button.get().is_null() => {
            if settings.item_changed(unsafe { &*(lparam as *const NMHDR) }) {
                return 0;
            }
        }
        _ => {}
    }
    unsafe { DefWindowProcW(window, message, wparam, lparam) }
}
