//! `--preview-menu <dir>`: opens the real Windows menu with the picture header for each place the
//! buds can be (sample levels), at the bottom right of the screen, and saves a PNG of each menu
//! window as Windows drew it. For checking the header without the tray; it sends nothing to any
//! device and doesn't touch a running tray.
use crate::{files, system, tray};
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::ptr::{null, null_mut};
use quicktooth_core::battery::Pods;
use quicktooth_core::host::MenuItem;
use quicktooth_core::popup::{self, HeaderInfo};
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::{
    BI_RGB, BITMAPINFO, BITMAPINFOHEADER, BitBlt, CreateCompatibleDC, CreateDIBSection, DIB_RGB_COLORS, DeleteDC, DeleteObject, GetDC,
    HBITMAP, InvalidateRect, ReleaseDC, SRCCOPY, SelectObject, UpdateWindow,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreatePopupMenu, CreateWindowExW, HMENU, MF_BITMAP, MF_BYCOMMAND, ModifyMenuW, DefWindowProcW, DestroyMenu, DestroyWindow, EndMenu, FindWindowW, GetSystemMetrics, GetWindowRect,
    KillTimer, RegisterClassExW, SM_CXSCREEN, SM_CYSCREEN, SetForegroundWindow, SetTimer, TPM_BOTTOMALIGN, TPM_RETURNCMD, TPM_RIGHTALIGN,
    TrackPopupMenuEx, WM_TIMER, WNDCLASSEXW, WS_EX_TOOLWINDOW, WS_POPUP,
};

struct State {
    shot: Option<(i32, i32, Vec<u8>)>,
    anim: Option<Anim>,
}

/// `--animate`: the header's motion played through a sequence of places in one open menu, a frame
/// captured per Win32 timer tick.
struct Anim {
    motion: popup::Motion,
    info: HeaderInfo,
    scale: f64,
    elapsed: f64,
    /// The menu and the header item's current bitmap (as integers: handles aren't `Send`).
    menu: isize,
    bitmap: isize,
    frames: Vec<(i32, i32, Vec<u8>)>,
}

/// The sequence: each place and how long it's held, ms.
const SEQUENCE: [(Pods, f64); 6] = [
    (Pods::Closed, 1300.0),
    (Pods::Open, 1500.0),
    (Pods::OneOut { left: true }, 2200.0),
    (Pods::Out, 2900.0),
    (Pods::Open, 2900.0),
    (Pods::Closed, 2600.0),
];
const TICK_MS: f64 = 94.8;
const HEADER_ID: i32 = 9;

static STATE: pk::sync::Mutex<Option<RefCell<State>>> = pk::sync::Mutex::new(None);

fn with<R>(f: impl FnOnce(&mut State) -> R) -> Option<R> {
    let g = STATE.lock();
    g.as_ref().map(|s| f(&mut s.borrow_mut()))
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_TIMER if wparam == 2 => {
            if !with(tick).unwrap_or(false) {
                unsafe {
                    KillTimer(hwnd, 2);
                    EndMenu();
                }
            }
            return 0;
        }
        WM_TIMER => {
            unsafe { KillTimer(hwnd, wparam) };
            let menu = unsafe { FindWindowW(pk::w!("#32768"), null()) };
            if !menu.is_null() {
                let shot = capture(menu);
                with(|s| s.shot = shot);
            }
            unsafe { EndMenu() };
            return 0;
        }
        _ => {}
    }
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

/// The menu window as it is on screen (DWM's corners and all), BGRA top-down.
fn capture(menu: HWND) -> Option<(i32, i32, Vec<u8>)> {
    let mut r: RECT = unsafe { core::mem::zeroed() };
    unsafe { GetWindowRect(menu, &mut r) };
    let (w, h) = (r.right - r.left, r.bottom - r.top);
    if w <= 0 || h <= 0 {
        return None;
    }
    unsafe {
        let screen = GetDC(null_mut());
        let dc = CreateCompatibleDC(screen);
        let mut bmi: BITMAPINFO = core::mem::zeroed();
        bmi.bmiHeader.biSize = size_of::<BITMAPINFOHEADER>() as u32;
        bmi.bmiHeader.biWidth = w;
        bmi.bmiHeader.biHeight = -h;
        bmi.bmiHeader.biPlanes = 1;
        bmi.bmiHeader.biBitCount = 32;
        bmi.bmiHeader.biCompression = BI_RGB;
        let mut bits = null_mut();
        let bmp = CreateDIBSection(dc, &bmi, DIB_RGB_COLORS, &mut bits, null_mut(), 0);
        let old = SelectObject(dc, bmp);
        BitBlt(dc, 0, 0, w, h, screen, r.left, r.top, SRCCOPY);
        let mut px = core::slice::from_raw_parts(bits as *const u8, (w * h * 4) as usize).to_vec();
        for p in px.chunks_exact_mut(4) {
            p[3] = 0xFF;
        }
        SelectObject(dc, old);
        DeleteObject(bmp);
        DeleteDC(dc);
        ReleaseDC(null_mut(), screen);
        Some((w, h, px))
    }
}

/// One tick of `--animate`: the next frame shown and captured; false once the sequence is over.
fn tick(s: &mut State) -> bool {
    let Some(a) = s.anim.as_mut() else { return false };
    a.elapsed += TICK_MS;
    let mut at = 0.0;
    let mut place = None;
    for (pods, hold) in SEQUENCE {
        if a.elapsed >= at {
            place = Some(pods);
        }
        at += hold;
    }
    if a.elapsed > at {
        return false;
    }
    if let Some(p) = place {
        a.motion.go(p);
    }
    a.motion.advance(TICK_MS);
    let mut info = a.info.clone();
    info.pods = a.motion.pods();
    let header = popup::header_posed(&info, a.scale, &a.motion.pose());
    let bmp = tray::header_bitmap(&header);
    unsafe {
        ModifyMenuW(a.menu as HMENU, HEADER_ID as u32, MF_BYCOMMAND | MF_BITMAP, HEADER_ID as usize, bmp as *const u16);
        DeleteObject(a.bitmap as HBITMAP);
    }
    a.bitmap = bmp as isize;
    let w = unsafe { FindWindowW(pk::w!("#32768"), null()) };
    if !w.is_null() {
        unsafe {
            InvalidateRect(w, null(), 0);
            UpdateWindow(w);
        }
        if let Some(f) = capture(w) {
            a.frames.push(f);
        }
    }
    true
}

fn sample_info(pods: Pods, light: bool) -> HeaderInfo {
    HeaderInfo {
        name: "AirPods Pro".into(),
        pods,
        connected: true,
        left: Some((80, pods == Pods::Closed)),
        right: Some((if pods == Pods::Out { 15 } else { 70 }, matches!(pods, Pods::Closed | Pods::OneOut { left: true }))),
        case: if pods == Pods::Out { None } else { Some((50, matches!(pods, Pods::Closed | Pods::Open))) },
        low_warning: 20,
        light,
    }
}

fn items(header: popup::Header) -> Vec<MenuItem> {
    vec![
        MenuItem::command(HEADER_ID, "AirPods Pro \u{2014} Connected").grayed(true).with_header(alloc::rc::Rc::new(header)),
        MenuItem::divider(),
        MenuItem::command(2, "Disconnect"),
        MenuItem::submenu("Device", vec![MenuItem::command(100, "AirPods Pro  (connected)").checked(true)]),
        MenuItem::divider(),
        MenuItem::command(3, "Start with Windows").checked(true),
        MenuItem::command(4, "Exit"),
    ]
}

pub fn run(dir: &str, animate: bool) -> u32 {
    *STATE.lock() = Some(RefCell::new(State { shot: None, anim: None }));
    let instance = unsafe { GetModuleHandleW(null()) };
    let class = pk::w!("QuickTooth.MenuPreview");
    let wc = WNDCLASSEXW { cbSize: size_of::<WNDCLASSEXW>() as u32, lpfnWndProc: Some(wndproc), hInstance: instance, lpszClassName: class, ..unsafe { core::mem::zeroed() } };
    unsafe { RegisterClassExW(&wc) };
    let hwnd = unsafe { CreateWindowExW(WS_EX_TOOLWINDOW, class, class, WS_POPUP, 0, 0, 0, 0, null_mut(), null_mut(), instance, null()) };
    if hwnd.is_null() {
        quicktooth_core::log::error("preview menu", &files::last_error_message());
        return 1;
    }
    let light = system::taskbar_is_light();
    system::apply_menu_theme(hwnd, light);
    let scale = unsafe { windows_sys::Win32::UI::HiDpi::GetDpiForWindow(hwnd) } as f64 / 96.0;
    let (sx, sy) = unsafe { (GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN)) };
    if animate {
        let info = sample_info(Pods::Closed, light);
        let motion = popup::Motion::rest(Pods::Closed);
        let menu = unsafe { CreatePopupMenu() };
        let bitmaps = tray::append(menu, &items(popup::header_posed(&info, scale, &motion.pose())));
        let first = bitmaps.first().map_or(0, |b| *b as isize);
        with(|s| s.anim = Some(Anim { motion, info, scale, elapsed: 0.0, menu: menu as isize, bitmap: first, frames: Vec::new() }));
        unsafe {
            SetForegroundWindow(hwnd);
            SetTimer(hwnd, 2, TICK_MS as u32, None);
            TrackPopupMenuEx(menu, TPM_RETURNCMD | TPM_RIGHTALIGN | TPM_BOTTOMALIGN, sx - 12, sy - 60, hwnd, null());
            DestroyMenu(menu);
        }
        let anim = with(|s| s.anim.take()).flatten();
        let mut code = 1;
        if let Some(a) = anim {
            unsafe { DeleteObject(a.bitmap as HBITMAP) };
            for (i, (w, h, px)) in a.frames.iter().enumerate() {
                let path: String = format!("{dir}\\frame-{i:04}.png");
                if files::write_file(&path, &quicktooth_core::icon::png::encode(px, *w as usize, *h as usize)).is_ok() {
                    code = 0;
                }
            }
        }
        unsafe { DestroyWindow(hwnd) };
        return code;
    }
    let states: [(&str, Pods, Option<(i32, bool)>); 5] = [
        ("closed", Pods::Closed, Some((50, true))),
        ("lid-open", Pods::Open, Some((50, true))),
        ("left-out", Pods::OneOut { left: true }, Some((50, false))),
        ("right-out", Pods::OneOut { left: false }, Some((50, false))),
        ("both-out", Pods::Out, None),
    ];
    let mut code = 0;
    for (name, pods, case) in states {
        let info = HeaderInfo { case, ..sample_info(pods, light) };
        let items = items(popup::header(&info, scale));
        with(|s| s.shot = None);
        let menu = unsafe { CreatePopupMenu() };
        let bitmaps = tray::append(menu, &items);
        unsafe {
            SetForegroundWindow(hwnd);
            SetTimer(hwnd, 1, 600, None);
            TrackPopupMenuEx(menu, TPM_RETURNCMD | TPM_RIGHTALIGN | TPM_BOTTOMALIGN, sx - 12, sy - 60, hwnd, null());
            DestroyMenu(menu);
            for b in bitmaps {
                DeleteObject(b);
            }
        }
        let shot = with(|s| s.shot.take()).flatten();
        match shot {
            Some((w, h, px)) => {
                let path: String = format!("{dir}\\menu-{name}.png");
                if let Err(e) = files::write_file(&path, &quicktooth_core::icon::png::encode(&px, w as usize, h as usize)) {
                    quicktooth_core::log::error("preview menu", &e);
                    code = 1;
                }
            }
            None => code = 1,
        }
    }
    unsafe { DestroyWindow(hwnd) };
    code
}
