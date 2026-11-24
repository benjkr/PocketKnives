//! The XP and Windows 10 settings window (C#'s `NativeSettings.cs`): classic Win32 controls, with
//! visual styles for Windows 10. Positions are in 96-DPI pixels, scaled to the system DPI.
use crate::win::{self, UiOnly};
use alloc::format;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::{Cell, OnceCell, RefCell};
use core::ptr::{null, null_mut};
use pk::wide::to_wide;
use runner::{
    CONTROL_LOCAL_X, FRAME_WIDTH, MIN_WINDOW_HEIGHT, MIN_WINDOW_WIDTH, NativeLayout, PAGE_HOST_X, PAGE_HOST_Y, PAGE_LOCAL_X, PAGE_LOCAL_Y,
    PICTURE_GAP, PICTURE_HEIGHT, PICTURE_SIZE, PICTURE_WIDTH, PageSize, ROW_HEIGHT, ROW_STEP, hotkey_box, hotkey_value, measure_blade_page,
};
use runner_core::Result;
use runner_core::blade_settings::{self, Kind, Setting};
use runner_core::settings::{BLADES, BLADES_LABEL, GENERAL_DESCRIPTION, GENERAL_NAME, LOOK_LABEL, LOOK_NAMES, Look, Startup, Suite, TITLE};
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows_sys::Win32::Graphics::Dwm::{DWMWA_WINDOW_CORNER_PREFERENCE, DwmSetWindowAttribute};
use windows_sys::Win32::Graphics::Gdi::{
    COLOR_BTNFACE, COLOR_HIGHLIGHT, COLOR_WINDOW, CreateBitmap, CreateCompatibleDC, CreateFontIndirectW, DFC_BUTTON, DFCS_CHECKED,
    DFCS_FLAT, DeleteDC, DeleteObject, DrawFrameControl, FillRect, GetSysColorBrush, HFONT, MapWindowPoints, SelectObject,
};
use windows_sys::Win32::UI::Controls::{
    HIMAGELIST, HKM_GETHOTKEY, HKM_SETHOTKEY, HKM_SETRULES, ICC_LISTVIEW_CLASSES, ILC_COLOR24, ILC_COLOR32, INITCOMMONCONTROLSEX,
    ImageList_Add, ImageList_Create, ImageList_Destroy, ImageList_ReplaceIcon, InitCommonControlsEx, LVCF_WIDTH, LVCOLUMNW, LVIF_IMAGE,
    LVIF_TEXT, LVITEMW, LVM_GETITEMSTATE, LVM_INSERTCOLUMNW, LVM_INSERTITEMW, LVM_SETEXTENDEDLISTVIEWSTYLE, LVM_SETIMAGELIST,
    LVM_SETITEMSTATE, LVN_ITEMCHANGED, NMHDR, NMLISTVIEW, SetScrollInfo, SetWindowTheme, ShowScrollBar,
};
use windows_sys::Win32::UI::HiDpi::{AdjustWindowRectExForDpi, GetDpiForSystem, SystemParametersInfoForDpi};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::EnableWindow;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    BM_GETCHECK, BM_SETCHECK, CB_ADDSTRING, CB_GETCURSEL, CB_SETCURSEL, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DestroyIcon,
    DestroyWindow, GetMessageTime, GetParent, GetScrollInfo, GetWindowRect, HICON, HMENU, HWND_TOP, KillTimer, NONCLIENTMETRICSW,
    SB_BOTTOM, SB_LINEDOWN, SB_LINEUP, SB_PAGEDOWN, SB_PAGEUP, SB_THUMBPOSITION, SB_THUMBTRACK, SB_TOP, SB_VERT, SCROLLINFO, SIF_PAGE,
    SIF_POS, SIF_RANGE, SIF_TRACKPOS, SPI_GETNONCLIENTMETRICS, STM_SETICON, SW_ERASE, SW_HIDE, SW_INVALIDATE, SW_SCROLLCHILDREN, SW_SHOW,
    SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOZORDER, ScrollWindowEx, SendMessageW, SetTimer, SetWindowPos, SetWindowTextW, ShowWindow, WM_CLOSE,
    WM_COMMAND, WM_CTLCOLORSTATIC, WM_MOUSEWHEEL, WM_NOTIFY, WM_SETFONT, WM_SETICON, WM_TIMER, WM_VSCROLL, WS_BORDER, WS_CAPTION,
    WS_CHILD, WS_CLIPCHILDREN, WS_CLIPSIBLINGS, WS_EX_CLIENTEDGE, WS_EX_CONTROLPARENT, WS_EX_DLGMODALFRAME, WS_GROUP, WS_MINIMIZEBOX,
    WS_SYSMENU, WS_TABSTOP, WS_VISIBLE, WS_VSCROLL,
};

const CLASS_NAME: &str = "PocketKnives.Settings.Win32";
const PAGE_CLASS_NAME: &str = "PocketKnives.Settings.Page";
const OK_ID: usize = 1;
const CANCEL_ID: usize = 2;
const APPLY_ID: usize = 3;
const LOOK_ID: usize = 100;
const FIRST_SETTING_ID: usize = 200;
const PAGES: usize = BLADES.len() + 1;

// Styles and states as winuser.h and commctrl.h spell them.
const BS_DEFPUSHBUTTON: u32 = 0x1;
const BS_AUTOCHECKBOX: u32 = 0x3;
const BS_AUTORADIOBUTTON: u32 = 0x9;
const BS_PUSHLIKE: u32 = 0x1000;
const BCM_SETIMAGELIST: u32 = 0x1602;
const BUTTON_IMAGELIST_ALIGN_TOP: u32 = 2;
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
const FRAME_TIMER: usize = 1;

// Picture setting constants live in `runner` (`lib.rs`) alongside the layout math; the slide
// duration is only used here.
const FRAME_SLIDE_MS: u32 = 180;

/// `BUTTON_IMAGELIST` (commctrl.h).
#[repr(C)]
struct ButtonImageList {
    images: HIMAGELIST,
    margin: RECT,
    align: u32,
}

/// A picture setting's buttons and the frame around the chosen one: four thin statics in the
/// highlight colour, sliding from `from` to `to` (client rectangles) since `started`.
struct Picker {
    buttons: Vec<HWND>,
    edges: [HWND; 4],
    from: Cell<RECT>,
    to: Cell<RECT>,
    started: Cell<u32>,
}

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
    hint_label: Cell<HWND>,
    page_host: Cell<HWND>,
    apply_button: Cell<HWND>,
    page_title: Cell<HWND>,
    description: Cell<HWND>,
    page_icon: Cell<HWND>,
    look_choice: Cell<HWND>,
    /// Scaled content height of each page inside `page_host`, for vertical scrolling when capped.
    page_heights: RefCell<[i32; PAGES]>,
    viewport_height: Cell<i32>,
    scroll_y: Cell<i32>,
    max_scroll: Cell<i32>,
    /// Every icon this window loaded, destroyed with it.
    icons: RefCell<Vec<HICON>>,
    page_icons: RefCell<[HICON; PAGES]>,
    /// Every page's own controls, General's first; `show_page` shows one page's, hides the rest.
    page_controls: RefCell<[Vec<HWND>; PAGES]>,
    /// One entry per blade setting (blade, index, controls), in control ID order from
    /// `FIRST_SETTING_ID`: one control, or a picture setting's buttons (which share its ID).
    setting_controls: RefCell<Vec<(usize, usize, Vec<HWND>)>>,
    pickers: RefCell<Vec<Picker>>,
    /// The picture settings' images, destroyed with the window.
    picture_lists: RefCell<Vec<HIMAGELIST>>,
    /// Each blade's settings as it printed them, once its answer is in (while `build` runs).
    blade_settings: OnceCell<Vec<Vec<Setting>>>,
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
        s.setting_controls
            .borrow()
            .iter()
            .any(|(blade, index, c)| c.contains(&control) && s.blade_settings()[*blade][*index].kind == Kind::Hotkey)
    })
}

pub fn initialize() -> Result<()> {
    win::register(CLASS_NAME, window_procedure, Some(COLOR_BTNFACE))?;
    win::register(PAGE_CLASS_NAME, page_procedure, Some(COLOR_BTNFACE))
}

/// Opens the window in `look` (XP or Windows 10), with each blade's settings read from it now.
pub fn open(suite: &'static Suite, look: Look) -> Result<()> {
    // The blades answer while the window is made.
    let asked = blade_settings::ask_all(suite);
    let original = suite.read_startup()?;
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
        hint_label: Cell::new(null_mut()),
        page_host: Cell::new(null_mut()),
        apply_button: Cell::new(null_mut()),
        page_title: Cell::new(null_mut()),
        description: Cell::new(null_mut()),
        page_icon: Cell::new(null_mut()),
        look_choice: Cell::new(null_mut()),
        page_heights: RefCell::new([0; PAGES]),
        viewport_height: Cell::new(0),
        scroll_y: Cell::new(0),
        max_scroll: Cell::new(0),
        icons: RefCell::new(Vec::new()),
        page_icons: RefCell::new([null_mut(); PAGES]),
        page_controls: RefCell::new(Default::default()),
        setting_controls: RefCell::new(Vec::new()),
        pickers: RefCell::new(Vec::new()),
        picture_lists: RefCell::new(Vec::new()),
        blade_settings: OnceCell::new(),
        saved: RefCell::new(Vec::new()),
        original: RefCell::new(original),
    });
    *CURRENT.get().borrow_mut() = Some(settings.clone());
    match settings.build(asked) {
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
        for &images in s.picture_lists.borrow().iter() {
            ImageList_Destroy(images);
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

    /// Each blade's settings; none before `build` has them.
    fn blade_settings(&self) -> &[Vec<Setting>] {
        self.blade_settings.get().map_or(&[], Vec::as_slice)
    }

    fn build(&self, asked: blade_settings::Asked) -> Result<()> {
        let controls = INITCOMMONCONTROLSEX { dwSize: size_of::<INITCOMMONCONTROLSEX>() as u32, dwICC: ICC_LISTVIEW_CLASSES };
        unsafe { InitCommonControlsEx(&controls) };
        self.create_fonts();

        // WS_CLIPCHILDREN: a picture setting's sliding frame doesn't make the window paint over its buttons.
        let style = WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_CLIPCHILDREN;
        let extended_style = WS_EX_CONTROLPARENT | WS_EX_DLGMODALFRAME;
        let mut bounds = RECT { left: 0, top: 0, right: self.scale(MIN_WINDOW_WIDTH), bottom: self.scale(MIN_WINDOW_HEIGHT) };
        unsafe { AdjustWindowRectExForDpi(&mut bounds, style, 0, extended_style, self.dpi) };
        let nc_width = (bounds.right - bounds.left) - self.scale(MIN_WINDOW_WIDTH);
        let nc_height = (bounds.bottom - bounds.top) - self.scale(MIN_WINDOW_HEIGHT);
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
        let hint = self.control("STATIC", "Checked blades start when you sign in.", 0, (8, 198, 156, 36), 0, false, 0)?;
        self.hint_label.set(hint);

        self.page_title.set(self.control("STATIC", "", 0, (178, 8, 332, 18), 0, true, 0)?);
        self.description.set(self.control("STATIC", "", 0, (178, 28, 330, 38), 0, false, 0)?);
        self.page_icon.set(self.control("STATIC", "", SS_ICON, (522, 8, 32, 32), 0, false, 0)?);

        // Child host for page controls: keeps the header and OK/Cancel/Apply fixed and scrolls
        // vertically if a page is still taller than the work-area-capped window.
        let page_host = self.control(
            PAGE_CLASS_NAME,
            "",
            WS_CLIPCHILDREN,
            (PAGE_HOST_X, PAGE_HOST_Y, MIN_WINDOW_WIDTH - PAGE_HOST_X - 8, 150),
            0,
            false,
            WS_EX_CONTROLPARENT,
        )?;
        self.page_host.set(page_host);

        let look_label = self.page_control("STATIC", &format!("&{LOOK_LABEL}"), 0, (PAGE_LOCAL_X, 2, 120, 18), 0, true, 0)?;
        let look_choice = self.page_control(
            "COMBOBOX",
            "",
            WS_TABSTOP | WS_VSCROLL | CBS_DROPDOWNLIST,
            (PAGE_LOCAL_X, 22, 200, 160),
            LOOK_ID,
            false,
            0,
        )?;
        self.look_choice.set(look_choice);
        for name in LOOK_NAMES {
            add_choice(look_choice, name);
        }
        send(look_choice, CB_SETCURSEL, self.look.index(), 0);
        self.page_controls.borrow_mut()[0].extend([look_label, look_choice]);
        let mut pages = Vec::new();
        for page in asked.answers() {
            pages.push(page?);
        }
        *self.saved.borrow_mut() = pages.iter().map(|page: &Vec<Setting>| page.iter().map(|s| s.value.clone()).collect()).collect();
        let _ = self.blade_settings.set(pages);

        // Load picture icons once so measurement knows which `picture` settings fall back to a
        // drop-down, then size the window to fit the tallest page (within the monitor work area).
        let mut loaded_pictures: Vec<Vec<Option<Vec<HICON>>>> = Vec::with_capacity(BLADES.len());
        let mut page_sizes = [PageSize { width: PAGE_LOCAL_X + 200, height: 48 }; PAGES];
        for i in 0..BLADES.len() {
            let settings = &self.blade_settings()[i];
            let pics: Vec<Option<Vec<HICON>>> =
                settings.iter().map(|s| (s.kind == Kind::Picture).then(|| self.pictures(i, s)).flatten()).collect();
            let has_pics: Vec<bool> = pics.iter().map(Option::is_some).collect();
            page_sizes[i + 1] = measure_blade_page(settings, &has_pics);
            loaded_pictures.push(pics);
        }
        let dpi = self.dpi.max(96) as i64;
        let work_client = runner_core::work_area(self.dpi).map(|(w, h)| {
            let nc_w_logical = (nc_width as i64 * 96 + dpi - 1) / dpi;
            let nc_h_logical = (nc_height as i64 * 96 + dpi - 1) / dpi;
            ((w - nc_w_logical as i32).max(160), (h - nc_h_logical as i32).max(160))
        });
        let layout = NativeLayout::compute(&page_sizes, work_client);
        let mut final_bounds = RECT { left: 0, top: 0, right: self.scale(layout.client_width), bottom: self.scale(layout.client_height) };
        unsafe {
            AdjustWindowRectExForDpi(&mut final_bounds, style, 0, extended_style, self.dpi);
            SetWindowPos(
                window,
                null_mut(),
                0,
                0,
                final_bounds.right - final_bounds.left,
                final_bounds.bottom - final_bounds.top,
                SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE,
            );
            SetWindowPos(blade_list, null_mut(), self.scale(8), self.scale(64), self.scale(156), self.scale(layout.blade_list_height), SWP_NOZORDER | SWP_NOACTIVATE);
            SetWindowPos(hint, null_mut(), self.scale(8), self.scale(layout.hint_y), self.scale(156), self.scale(36), SWP_NOZORDER | SWP_NOACTIVATE);
            SetWindowPos(
                self.description.get(),
                null_mut(),
                self.scale(178),
                self.scale(28),
                self.scale(layout.description_width),
                self.scale(38),
                SWP_NOZORDER | SWP_NOACTIVATE,
            );
            SetWindowPos(self.page_icon.get(), null_mut(), self.scale(layout.page_icon_x), self.scale(8), self.scale(32), self.scale(32), SWP_NOZORDER | SWP_NOACTIVATE);
            SetWindowPos(
                page_host,
                null_mut(),
                self.scale(PAGE_HOST_X),
                self.scale(PAGE_HOST_Y),
                self.scale(layout.page_host_width),
                self.scale(layout.page_host_height),
                SWP_NOZORDER | SWP_NOACTIVATE,
            );
        }
        self.viewport_height.set(self.scale(layout.page_host_height));
        for (i, ps) in page_sizes.iter().enumerate() {
            self.page_heights.borrow_mut()[i] = self.scale(ps.height);
        }

        for (i, pics) in loaded_pictures.into_iter().enumerate() {
            self.build_blade_page(i, &pics)?;
        }

        let button_style = WS_TABSTOP | if self.classic { BS_FLAT } else { 0 };
        // WS_GROUP ends any preceding radio-button group.
        self.control("BUTTON", "OK", button_style | BS_DEFPUSHBUTTON | WS_GROUP, (layout.ok_x, layout.button_y, 75, 24), OK_ID, false, 0)?;
        self.control("BUTTON", "Cancel", button_style, (layout.cancel_x, layout.button_y, 75, 24), CANCEL_ID, false, 0)?;
        let apply = self.control("BUTTON", "&Apply", button_style, (layout.apply_x, layout.button_y, 75, 24), APPLY_ID, false, 0)?;
        self.apply_button.set(apply);
        unsafe { EnableWindow(apply, 0) };
        set_state(general_list, 0, LVIS_SELECTED | LVIS_FOCUSED, LVIS_SELECTED | LVIS_FOCUSED);
        self.show_page(0);
        Ok(())
    }

    fn build_blade_page(&self, blade: usize, pictures: &[Option<Vec<HICON>>]) -> Result<()> {
        let settings = &self.blade_settings()[blade];
        if settings.is_empty() {
            let text = format!("{} has no settings to show.", BLADES[blade].name);
            let label = self.page_control("STATIC", &text, 0, (PAGE_LOCAL_X, 6, 330, 18), 0, false, 0)?;
            self.page_controls.borrow_mut()[blade + 1].push(label);
        }
        let flat = if self.classic { BS_FLAT } else { 0 };
        let mut y = PAGE_LOCAL_Y;
        let mut end_group = false;
        for (i, setting) in settings.iter().enumerate() {
            let id = FIRST_SETTING_ID + self.setting_controls.borrow().len();
            if setting.kind == Kind::Picture
                && let Some(icons) = pictures.get(i).and_then(Option::as_deref)
            {
                let label = self.page_control("STATIC", &setting.label, 0, (PAGE_LOCAL_X, y + 3, 116, 18), 0, false, 0)?;
                let (buttons, edges) = self.picker(setting, icons, y, id)?;
                let mut pages = self.page_controls.borrow_mut();
                pages[blade + 1].push(label);
                pages[blade + 1].extend(buttons.iter().chain(&edges));
                drop(pages);
                self.setting_controls.borrow_mut().push((blade, i, buttons));
                y += PICTURE_HEIGHT + PICTURE_GAP;
                end_group = true;
                continue;
            }
            let group = if end_group {
                end_group = false;
                WS_GROUP
            } else {
                0
            };
            let control = if setting.kind == Kind::Toggle {
                let c = self.page_control(
                    "BUTTON",
                    &setting.label,
                    WS_TABSTOP | BS_AUTOCHECKBOX | flat | group,
                    (PAGE_LOCAL_X, y, 330, ROW_HEIGHT),
                    id,
                    false,
                    0,
                )?;
                send(c, BM_SETCHECK, (setting.value == "1") as usize, 0);
                c
            } else {
                let label = self.page_control("STATIC", &setting.label, 0, (PAGE_LOCAL_X, y + 3, 116, 18), 0, false, 0)?;
                self.page_controls.borrow_mut()[blade + 1].push(label);
                if setting.kind == Kind::Hotkey {
                    let border = if self.classic { WS_BORDER } else { 0 };
                    let edge = if self.classic { 0 } else { WS_EX_CLIENTEDGE };
                    let c = self.page_control(
                        "msctls_hotkey32",
                        "",
                        WS_TABSTOP | border | group,
                        (CONTROL_LOCAL_X, y, 210, ROW_HEIGHT),
                        id,
                        false,
                        edge,
                    )?;
                    // A key with no modifier, or Shift alone, would take that key from every program.
                    send(c, HKM_SETRULES, HKCOMB_NONE | HKCOMB_S, HOTKEYF_CONTROL | HOTKEYF_ALT); // they become Ctrl+Alt
                    send(c, HKM_SETHOTKEY, hotkey_box(&setting.value), 0);
                    c
                } else {
                    let c = self.page_control(
                        "COMBOBOX",
                        "",
                        WS_TABSTOP | WS_VSCROLL | CBS_DROPDOWNLIST | group,
                        (CONTROL_LOCAL_X, y, 210, 200),
                        id,
                        false,
                        0,
                    )?;
                    for choice in &setting.choices {
                        add_choice(c, &choice.label);
                    }
                    let selected = setting.choices.iter().position(|c| c.value == setting.value);
                    send(c, CB_SETCURSEL, selected.unwrap_or(usize::MAX), 0);
                    c
                }
            };
            self.page_controls.borrow_mut()[blade + 1].push(control);
            self.setting_controls.borrow_mut().push((blade, i, alloc::vec![control]));
            y += ROW_STEP;
        }
        Ok(())
    }

    /// Each choice's picture at its size here; none if one is missing, and the setting is then a
    /// drop-down like a `choice`.
    fn pictures(&self, blade: usize, setting: &Setting) -> Option<Vec<HICON>> {
        let size = self.scale(PICTURE_SIZE);
        let mut icons = Vec::new();
        for path in blade_settings::pictures(self.suite, &BLADES[blade], setting)? {
            let icon = win::file_icon(&path, size);
            if icon.is_null() {
                return None;
            }
            icons.push(self.own(icon));
        }
        Some(icons)
    }

    /// A picture setting's buttons at `y` (push-like radio buttons, each its picture above its
    /// label) and the edges of the frame around the chosen one.
    fn picker(&self, setting: &Setting, icons: &[HICON], y: i32, id: usize) -> Result<(Vec<HWND>, [HWND; 4])> {
        let flat = if self.classic { BS_FLAT } else { 0 };
        let size = self.scale(PICTURE_SIZE);
        let mut buttons = Vec::new();
        for (i, (choice, &icon)) in setting.choices.iter().zip(icons).enumerate() {
            let first = if i == 0 { WS_GROUP | WS_TABSTOP } else { 0 };
            let x = CONTROL_LOCAL_X + i as i32 * (PICTURE_WIDTH + PICTURE_GAP);
            // Clipped so the frame, sliding above them, is never painted over.
            let style = first | BS_AUTORADIOBUTTON | BS_PUSHLIKE | flat | WS_CLIPSIBLINGS;
            let button = self.page_control("BUTTON", &choice.label, style, (x, y, PICTURE_WIDTH, PICTURE_HEIGHT), id, false, 0)?;
            let images = unsafe { ImageList_Create(size, size, ILC_COLOR32, 1, 0) };
            unsafe { ImageList_ReplaceIcon(images, -1, icon) };
            self.picture_lists.borrow_mut().push(images);
            let margin = RECT { left: 0, top: self.scale(6), right: 0, bottom: 0 };
            let list = ButtonImageList { images, margin, align: BUTTON_IMAGELIST_ALIGN_TOP };
            send(button, BCM_SETIMAGELIST, 0, &list as *const ButtonImageList as isize);
            send(button, BM_SETCHECK, (choice.value == setting.value) as usize, 0);
            buttons.push(button);
        }
        let mut edges = [null_mut(); 4];
        for edge in &mut edges {
            *edge = self.page_control("STATIC", "", 0, (0, 0, 0, 0), 0, false, 0)?;
        }
        let chosen = setting.choices.iter().position(|c| c.value == setting.value);
        let at = chosen.map_or(NO_FRAME, |i| self.frame_around(buttons[i]));
        let picker = Picker { buttons: buttons.clone(), edges, from: Cell::new(at), to: Cell::new(at), started: Cell::new(0) };
        place_frame(&picker, at, self.scale(FRAME_WIDTH));
        self.pickers.borrow_mut().push(picker);
        Ok((buttons, edges))
    }

    /// The frame's rectangle around `button`, in `page_host`'s client coordinates.
    fn frame_around(&self, button: HWND) -> RECT {
        let mut r = NO_FRAME;
        unsafe {
            GetWindowRect(button, &mut r);
            MapWindowPoints(null_mut(), self.page_host.get(), (&mut r as *mut RECT).cast(), 2);
        }
        let w = self.scale(FRAME_WIDTH);
        RECT { left: r.left - w, top: r.top - w, right: r.right + w, bottom: r.bottom + w }
    }

    /// A picture button was clicked (or chosen with the arrow keys): its frame slides to it.
    fn picked(&self, button: HWND) {
        let pickers = self.pickers.borrow();
        let Some(picker) = pickers.iter().find(|p| p.buttons.contains(&button)) else { return };
        let to = self.frame_around(button);
        if rect_eq(picker.to.get(), to) {
            return;
        }
        picker.from.set(frame_now(picker));
        picker.to.set(to);
        picker.started.set(unsafe { GetMessageTime() } as u32);
        unsafe { SetTimer(self.window.get(), FRAME_TIMER, 15, None) };
    }

    /// Moves the sliding frames on; stops the timer once they've all arrived.
    fn slide_frames(&self) {
        let mut moving = false;
        for picker in self.pickers.borrow().iter() {
            place_frame(picker, frame_now(picker), self.scale(FRAME_WIDTH));
            moving |= !rect_eq(frame_now(picker), picker.to.get());
        }
        if !moving {
            unsafe { KillTimer(self.window.get(), FRAME_TIMER) };
        }
    }

    fn is_frame_edge(&self, control: HWND) -> bool {
        self.pickers.borrow().iter().any(|p| p.edges.contains(&control))
    }

    /// The blade's value for what a setting's control holds now.
    fn read_setting(&self, blade: usize, index: usize, controls: &[HWND]) -> String {
        let setting = &self.blade_settings()[blade][index];
        let before = self.saved.borrow()[blade][index].clone();
        let control = controls[0];
        if setting.kind == Kind::Picture && controls.len() > 1 {
            // Buttons; a single control is the drop-down shown when a picture was missing.
            return match controls.iter().position(|&b| send(b, BM_GETCHECK, 0, 0) == 1) {
                Some(i) => setting.choices[i].value.clone(),
                None => before,
            };
        }
        match setting.kind {
            Kind::Toggle => String::from(if send(control, BM_GETCHECK, 0, 0) == 1 { "1" } else { "0" }),
            Kind::Choice | Kind::Picture => {
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
            let value = self.read_setting(blade, index, &control);
            if value != self.saved.borrow()[blade][index] {
                changes[blade].push((self.blade_settings()[blade][index].key.clone(), value));
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

    /// A child control in the main window at `(x, y, width, height)` in 96-DPI pixels.
    #[allow(clippy::too_many_arguments)]
    fn control(
        &self,
        class: &str,
        text: &str,
        style: u32,
        rect: (i32, i32, i32, i32),
        id: usize,
        bold: bool,
        extended_style: u32,
    ) -> Result<HWND> {
        self.control_in(self.window.get(), class, text, style, rect, id, bold, extended_style)
    }

    /// A child control inside `page_host` at `(x, y, width, height)` in 96-DPI pixels.
    #[allow(clippy::too_many_arguments)]
    fn page_control(
        &self,
        class: &str,
        text: &str,
        style: u32,
        rect: (i32, i32, i32, i32),
        id: usize,
        bold: bool,
        extended_style: u32,
    ) -> Result<HWND> {
        self.control_in(self.page_host.get(), class, text, style, rect, id, bold, extended_style)
    }

    #[allow(clippy::too_many_arguments)]
    fn control_in(
        &self,
        parent: HWND,
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
                parent,
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
        self.scroll_to(0);
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
        self.configure_scroll(page);
    }

    fn configure_scroll(&self, page: usize) {
        let host = self.page_host.get();
        let content_h = self.page_heights.borrow()[page];
        let view_h = self.viewport_height.get();
        let max_scroll = (content_h - view_h).max(0);
        self.max_scroll.set(max_scroll);
        self.scroll_y.set(0);
        let info = SCROLLINFO {
            cbSize: size_of::<SCROLLINFO>() as u32,
            fMask: SIF_RANGE | SIF_PAGE | SIF_POS,
            nMin: 0,
            nMax: (content_h - 1).max(0),
            nPage: view_h.max(0) as u32,
            nPos: 0,
            nTrackPos: 0,
        };
        unsafe {
            SetScrollInfo(host, SB_VERT, &info, 1);
            ShowScrollBar(host, SB_VERT, (max_scroll > 0) as i32);
        }
    }

    fn scroll_to(&self, pos: i32) {
        let new_pos = pos.clamp(0, self.max_scroll.get());
        let old_pos = self.scroll_y.get();
        if new_pos == old_pos {
            return;
        }
        let dy = old_pos - new_pos;
        let host = self.page_host.get();
        self.scroll_y.set(new_pos);
        unsafe {
            ScrollWindowEx(host, 0, dy, null(), null(), null_mut(), null_mut(), SW_SCROLLCHILDREN | SW_INVALIDATE | SW_ERASE);
        }
        for picker in self.pickers.borrow().iter() {
            picker.from.set(offset_y(picker.from.get(), dy));
            picker.to.set(offset_y(picker.to.get(), dy));
        }
        let info = SCROLLINFO {
            cbSize: size_of::<SCROLLINFO>() as u32,
            fMask: SIF_POS,
            nMin: 0,
            nMax: 0,
            nPage: 0,
            nPos: new_pos,
            nTrackPos: 0,
        };
        unsafe { SetScrollInfo(host, SB_VERT, &info, 1) };
    }

    fn on_vscroll(&self, code: i32) {
        let step = self.scale(ROW_STEP);
        let page = (self.viewport_height.get() - step).max(step);
        let target = match code {
            SB_LINEUP => self.scroll_y.get() - step,
            SB_LINEDOWN => self.scroll_y.get() + step,
            SB_PAGEUP => self.scroll_y.get() - page,
            SB_PAGEDOWN => self.scroll_y.get() + page,
            SB_TOP => 0,
            SB_BOTTOM => self.max_scroll.get(),
            SB_THUMBTRACK | SB_THUMBPOSITION => {
                let mut info = SCROLLINFO {
                    cbSize: size_of::<SCROLLINFO>() as u32,
                    fMask: SIF_TRACKPOS,
                    nMin: 0,
                    nMax: 0,
                    nPage: 0,
                    nPos: 0,
                    nTrackPos: 0,
                };
                unsafe { GetScrollInfo(self.page_host.get(), SB_VERT, &mut info) };
                info.nTrackPos
            }
            _ => return,
        };
        self.scroll_to(target);
    }

    fn on_mousewheel(&self, delta: i16) -> bool {
        if self.max_scroll.get() <= 0 {
            return false;
        }
        let step = self.scale(ROW_STEP) * 2;
        self.scroll_to(self.scroll_y.get() - (delta as i32 * step) / 120);
        true
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
                if let Some(index) = self.blade_settings()[blade].iter().position(|s| &s.key == key) {
                    self.saved.borrow_mut()[blade][index] = value.clone();
                }
            }
        }
        Ok(())
    }

    fn execute(&self, id: usize, notification: u32, control: HWND) {
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
                if notification == BN_CLICKED {
                    self.picked(control);
                }
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

const NO_FRAME: RECT = RECT { left: 0, top: 0, right: 0, bottom: 0 };

fn rect_eq(a: RECT, b: RECT) -> bool {
    (a.left, a.top, a.right, a.bottom) == (b.left, b.top, b.right, b.bottom)
}

fn offset_y(r: RECT, dy: i32) -> RECT {
    if rect_eq(r, NO_FRAME) { r } else { RECT { left: r.left, top: r.top + dy, right: r.right, bottom: r.bottom + dy } }
}

/// Where a picker's frame is now: partway from `from` to `to`, easing out. With nothing chosen
/// before, it appears where it's going.
fn frame_now(picker: &Picker) -> RECT {
    let (from, to) = (picker.from.get(), picker.to.get());
    if rect_eq(from, NO_FRAME) {
        return to;
    }
    let elapsed = (unsafe { GetMessageTime() } as u32).wrapping_sub(picker.started.get()).min(FRAME_SLIDE_MS);
    let u = 1.0 - elapsed as f64 / FRAME_SLIDE_MS as f64;
    let eased = 1.0 - u * u * u;
    let mix = |a: i32, b: i32| a + ((b - a) as f64 * eased) as i32;
    RECT {
        left: mix(from.left, to.left),
        top: mix(from.top, to.top),
        right: mix(from.right, to.right),
        bottom: mix(from.bottom, to.bottom),
    }
}

/// Puts a picker's four edges on the outline of `at`, `width` thick, above the buttons.
fn place_frame(picker: &Picker, at: RECT, width: i32) {
    let (w, h) = (at.right - at.left, at.bottom - at.top);
    let width = if w == 0 { 0 } else { width };
    let sides = [
        (at.left, at.top, w, width),
        (at.left, at.bottom - width, w, width),
        (at.left, at.top, width, h),
        (at.right - width, at.top, width, h),
    ];
    for (&edge, (x, y, cx, cy)) in picker.edges.iter().zip(sides) {
        unsafe { SetWindowPos(edge, HWND_TOP, x, y, cx, cy, SWP_NOACTIVATE) };
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
            settings.execute(wparam & 0xffff, (wparam >> 16 & 0xffff) as u32, lparam as HWND);
            return 0;
        }
        WM_TIMER if wparam == FRAME_TIMER => {
            settings.slide_frames();
            return 0;
        }
        WM_CTLCOLORSTATIC if settings.is_frame_edge(lparam as HWND) => {
            return unsafe { GetSysColorBrush(COLOR_HIGHLIGHT) } as LRESULT;
        }
        WM_MOUSEWHEEL => {
            if settings.on_mousewheel((wparam >> 16 & 0xffff) as i16) {
                return 0;
            }
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

unsafe extern "system" fn page_procedure(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match message {
        WM_COMMAND | WM_CTLCOLORSTATIC => {
            let parent = unsafe { GetParent(window) };
            if !parent.is_null() {
                return send(parent, message, wparam, lparam);
            }
        }
        WM_VSCROLL => {
            if let Some(settings) = current().filter(|s| s.page_host.get() == window) {
                settings.on_vscroll((wparam & 0xffff) as i32);
                return 0;
            }
        }
        WM_MOUSEWHEEL => {
            if let Some(settings) = current().filter(|s| s.page_host.get() == window)
                && settings.on_mousewheel((wparam >> 16 & 0xffff) as i16)
            {
                return 0;
            }
        }
        _ => {}
    }
    unsafe { DefWindowProcW(window, message, wparam, lparam) }
}
