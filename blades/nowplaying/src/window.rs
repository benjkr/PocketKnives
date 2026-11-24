//! Taskbar-docked layered Win32 window for NowPlaying.
//! Owns the message loop, atomic `GWLP_HWNDPARENT = Shell_TrayWnd` Z-order docking, adaptive
//! 60fps animation timer, mouse hover/scrub hit-testing, and dark/light right-click context menu.
use crate::app;
use crate::audio::AudioMeter;
use crate::ipc;
use crate::render::{
    DEFAULT_TASKBAR_H, FrameState, HoverTarget, NEXT_RECT, PLAY_RECT, PREV_RECT, Renderer, window_metrics,
};
use crate::settings::{self, Settings};
use crate::smtc::{MediaCommand, MediaSnapshot, SmtcClient};
use alloc::boxed::Box;
use core::ffi::c_void;
use core::ptr::{null, null_mut};
use pk::w;
use windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx};
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows_sys::Win32::Graphics::Dwm::{DWMWA_CLOAKED, DwmGetWindowAttribute};
use windows_sys::Win32::Graphics::Gdi::{GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromWindow};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::SystemInformation::GetTickCount64;
use windows_sys::Win32::UI::Accessibility::{HWINEVENTHOOK, SetWinEventHook, UnhookWinEvent};
use windows_sys::Win32::UI::HiDpi::GetDpiForWindow;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    ReleaseCapture, SetCapture, TME_LEAVE, TRACKMOUSEEVENT, TrackMouseEvent,
};
const WM_MOUSELEAVE: u32 = 0x02A3;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyMenu, DestroyWindow, DispatchMessageW,
    EVENT_SYSTEM_FOREGROUND, FindWindowW, GW_HWNDPREV, GWLP_HWNDPARENT, GWLP_USERDATA, GetClassNameW, GetCursorPos,
    GetForegroundWindow, GetMessageW, GetShellWindow, GetSystemMetrics, GetWindow, GetWindowLongPtrW, GetWindowRect, IDC_ARROW,
    IDC_HAND, IsIconic, IsWindowVisible, KillTimer, LoadCursorW, MA_NOACTIVATE, MF_CHECKED, MF_SEPARATOR, MF_STRING, MSG,
    PostQuitMessage, RegisterClassExW, RegisterWindowMessageW, SM_CYSCREEN, SW_HIDE, SWP_HIDEWINDOW, SWP_NOACTIVATE, SWP_NOMOVE,
    SWP_NOSIZE, SWP_NOZORDER, SWP_SHOWWINDOW, SetCursor, SetTimer, SetWindowLongPtrW, SetWindowPos, ShowWindow, TPM_BOTTOMALIGN,
    TPM_LEFTALIGN, TPM_NONOTIFY, TPM_RETURNCMD, TPM_RIGHTBUTTON, TrackPopupMenu, TranslateMessage, WINEVENT_OUTOFCONTEXT, WM_APP,
    WM_CAPTURECHANGED, WM_DESTROY, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEACTIVATE, WM_MOUSEMOVE, WM_RBUTTONUP, WM_SETCURSOR,
    WM_SETTINGCHANGE, WM_TIMER, WNDCLASSEXW, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_POPUP,
};

const WM_SMTC_UPDATED: u32 = WM_APP + 1;
const WM_FG_CHANGED: u32 = WM_APP + 2;
const TIMER_FRAME: usize = 1;
const TIMER_HOUSEKEEP: usize = 2;
const TIMER_ZORDER: usize = 3;

const MENU_PLAY_PAUSE: usize = 101;
const MENU_PREV: usize = 102;
const MENU_NEXT: usize = 103;
const MENU_VIS_TOGGLE: usize = 104;
const MENU_EXIT: usize = 105;

static mut GLOBAL_HWND: HWND = null_mut();

struct AppState {
    hwnd: HWND,
    renderer: Renderer,
    smtc: SmtcClient,
    audio: AudioMeter,
    settings: Settings,
    media: MediaSnapshot,
    light_taskbar: bool,
    reload_msg: u32,
    exit_msg: u32,
    taskbar_created_msg: u32,
    owned_tray: HWND,
    win_x: i32,
    win_y: i32,
    win_w: i32,
    win_h: i32,
    pad_px: i32,
    dpi: u32,
    hidden: bool,
    // Hover & animation state
    strip_hovered: bool,
    tracking_leave: bool,
    hover_target: HoverTarget,
    pressed: HoverTarget,
    scrub_dragging: bool,
    scrub_hover_ratio: f32,
    drag_pos_sec: f64,
    strip_hover_alpha: f32,
    btn_hover_alpha: [f32; 3],
    scrub_active_alpha: f32,
    last_frame_ms: u64,
    fast_timer_on: bool,
    zorder_checks_left: u8,
}

pub fn run() -> u32 {
    let Some(guard) = ipc::acquire_single_instance() else {
        return 0;
    };
    guard.keep();

    let _ = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
    let hinst = unsafe { GetModuleHandleW(null()) };

    let wc = WNDCLASSEXW {
        cbSize: core::mem::size_of::<WNDCLASSEXW>() as u32,
        lpfnWndProc: Some(wnd_proc),
        hInstance: hinst,
        hCursor: unsafe { LoadCursorW(null_mut(), IDC_ARROW) },
        lpszClassName: ipc::WINDOW_CLASS,
        ..unsafe { core::mem::zeroed() }
    };
    if unsafe { RegisterClassExW(&wc) } == 0 {
        return 1;
    }

    let init = window_metrics(96, DEFAULT_TASKBAR_H as i32);
    let hwnd = unsafe {
        CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_TOPMOST,
            ipc::WINDOW_CLASS,
            w!("NowPlaying"),
            WS_POPUP,
            0,
            0,
            init.win_w,
            init.win_h,
            null_mut(),
            null_mut(),
            hinst,
            null(),
        )
    };
    if hwnd.is_null() {
        return 1;
    }

    let dpi = read_taskbar_dpi();
    let Some(renderer) = Renderer::new(dpi) else {
        unsafe { DestroyWindow(hwnd) };
        return 1;
    };

    let settings = Settings::load();
    let light_taskbar = settings::taskbar_is_light();
    settings::apply_menu_theme(hwnd, light_taskbar);

    let smtc = SmtcClient::start(hwnd, WM_SMTC_UPDATED);
    let audio = AudioMeter::start();

    let mut state = Box::new(AppState {
        hwnd,
        renderer,
        smtc,
        audio,
        settings,
        media: MediaSnapshot::default(),
        light_taskbar,
        reload_msg: ipc::reload_message(),
        exit_msg: ipc::exit_message(),
        taskbar_created_msg: unsafe { RegisterWindowMessageW(w!("TaskbarCreated")) },
        owned_tray: null_mut(),
        win_x: i32::MIN,
        win_y: i32::MIN,
        win_w: 0,
        win_h: 0,
        pad_px: 0,
        dpi,
        hidden: true,
        strip_hovered: false,
        tracking_leave: false,
        hover_target: HoverTarget::None,
        pressed: HoverTarget::None,
        scrub_dragging: false,
        scrub_hover_ratio: 0.0,
        drag_pos_sec: 0.0,
        strip_hover_alpha: 0.0,
        btn_hover_alpha: [0.0; 3],
        scrub_active_alpha: 0.0,
        last_frame_ms: unsafe { GetTickCount64() },
        fast_timer_on: false,
        zorder_checks_left: 0,
    });

    state.dock_to_taskbar(true);
    state.render_now();

    let state_ptr = Box::into_raw(state);
    unsafe {
        GLOBAL_HWND = hwnd;
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, state_ptr as isize);
        SetTimer(hwnd, TIMER_HOUSEKEEP, 500, None);
    }

    let hook = unsafe {
        SetWinEventHook(
            EVENT_SYSTEM_FOREGROUND,
            EVENT_SYSTEM_FOREGROUND,
            null_mut(),
            Some(win_event_proc),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        )
    };

    let mut msg: MSG = unsafe { core::mem::zeroed() };
    while unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) } > 0 {
        unsafe {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    if !hook.is_null() {
        unsafe { UnhookWinEvent(hook) };
    }
    unsafe {
        GLOBAL_HWND = null_mut();
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
        let boxed = Box::from_raw(state_ptr);
        boxed.smtc.stop();
        boxed.audio.stop();
        if !boxed.owned_tray.is_null() {
            SetWindowLongPtrW(hwnd, GWLP_HWNDPARENT, 0);
        }
    }
    0
}

unsafe extern "system" fn win_event_proc(
    _hook: HWINEVENTHOOK,
    _event: u32,
    fg_hwnd: HWND,
    _id_obj: i32,
    _id_child: i32,
    _thread: u32,
    _time: u32,
) {
    let hwnd = unsafe { GLOBAL_HWND };
    if !hwnd.is_null() {
        unsafe { windows_sys::Win32::UI::WindowsAndMessaging::PostMessageW(hwnd, WM_FG_CHANGED, fg_hwnd as usize, 0) };
    }
}

unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) } as *mut AppState;
    if ptr.is_null() {
        return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
    }
    let st = unsafe { &mut *ptr };

    if msg == st.exit_msg {
        unsafe { DestroyWindow(hwnd) };
        return 0;
    }
    if msg == st.reload_msg {
        st.settings = Settings::load();
        st.update_audio_active();
        st.dock_to_taskbar(true);
        st.render_now();
        return 0;
    }
    if st.taskbar_created_msg != 0 && msg == st.taskbar_created_msg {
        st.owned_tray = null_mut();
        st.dock_to_taskbar(true);
        st.render_now();
        return 0;
    }

    match msg {
        WM_MOUSEACTIVATE => {
            let mouse_msg = ((lparam >> 16) & 0xFFFF) as u32;
            if mouse_msg != WM_RBUTTONUP && mouse_msg != 0x0204 {
                return MA_NOACTIVATE as LRESULT;
            }
        }
        WM_SETCURSOR => {
            let cursor = if st.hover_target == HoverTarget::None { IDC_ARROW } else { IDC_HAND };
            unsafe { SetCursor(LoadCursorW(null_mut(), cursor)) };
            return 1;
        }
        WM_SMTC_UPDATED => {
            st.media = st.smtc.snapshot();
            st.update_audio_active();
            st.dock_to_taskbar(false);
            st.sync_frame_timer();
            st.render_now();
            return 0;
        }
        WM_FG_CHANGED => {
            let fg = wparam as HWND;
            st.dock_to_taskbar(false);
            if !st.hidden && !st.owned_tray.is_null() {
                if fg == st.owned_tray {
                    unsafe { SetWindowPos(hwnd, (-1isize) as HWND, 0, 0, 0, 0, SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOSIZE) };
                }
                st.schedule_zorder_settle();
            }
            return 0;
        }
        WM_SETTINGCHANGE => {
            let light = settings::taskbar_is_light();
            if light != st.light_taskbar {
                st.light_taskbar = light;
                settings::apply_menu_theme(hwnd, light);
                st.render_now();
            }
            st.dock_to_taskbar(false);
            return 0;
        }
        WM_TIMER => {
            if wparam == TIMER_FRAME {
                st.tick_animation();
            } else if wparam == TIMER_HOUSEKEEP {
                let light = settings::taskbar_is_light();
                if light != st.light_taskbar {
                    st.light_taskbar = light;
                    settings::apply_menu_theme(hwnd, light);
                }
                st.dock_to_taskbar(false);
                if !st.hidden && st.media.playing && !st.fast_timer_on {
                    st.render_now();
                }
            } else if wparam == TIMER_ZORDER {
                st.dock_to_taskbar(false);
                st.zorder_checks_left = st.zorder_checks_left.saturating_sub(1);
                if st.zorder_checks_left == 0 || st.hidden {
                    st.zorder_checks_left = 0;
                    unsafe { KillTimer(hwnd, TIMER_ZORDER) };
                }
            }
            return 0;
        }
        WM_MOUSEMOVE => {
            let (x, y) = st.lparam_to_dips(lparam);
            if !st.tracking_leave {
                let mut tme = TRACKMOUSEEVENT {
                    cbSize: core::mem::size_of::<TRACKMOUSEEVENT>() as u32,
                    dwFlags: TME_LEAVE,
                    hwndTrack: hwnd,
                    dwHoverTime: 0,
                };
                unsafe { TrackMouseEvent(&mut tme) };
                st.tracking_leave = true;
            }
            st.strip_hovered = true;
            if st.scrub_dragging {
                st.scrub_hover_ratio = st.scrub_ratio(x);
                st.drag_pos_sec = (st.scrub_hover_ratio as f64) * st.media.duration_sec();
                st.render_now();
            } else {
                st.hover_target = st.hit_test(x, y);
                if st.hover_target == HoverTarget::Scrub {
                    st.scrub_hover_ratio = st.scrub_ratio(x);
                }
                st.sync_frame_timer();
                st.render_now();
            }
            return 0;
        }
        WM_MOUSELEAVE => {
            st.tracking_leave = false;
            st.strip_hovered = false;
            if !st.scrub_dragging {
                st.hover_target = HoverTarget::None;
                st.pressed = HoverTarget::None;
            }
            st.sync_frame_timer();
            return 0;
        }
        WM_LBUTTONDOWN => {
            let (x, y) = st.lparam_to_dips(lparam);
            match st.hit_test(x, y) {
                HoverTarget::Scrub if st.media.duration_sec() > 0.0 => {
                    st.scrub_dragging = true;
                    st.scrub_hover_ratio = st.scrub_ratio(x);
                    st.drag_pos_sec = (st.scrub_hover_ratio as f64) * st.media.duration_sec();
                    unsafe { SetCapture(hwnd) };
                    st.sync_frame_timer();
                    st.render_now();
                }
                t @ (HoverTarget::Prev | HoverTarget::Play | HoverTarget::Next | HoverTarget::App) => {
                    st.pressed = t;
                    st.render_now();
                }
                _ => {}
            }
            return 0;
        }
        WM_LBUTTONUP => {
            let (x, y) = st.lparam_to_dips(lparam);
            if st.scrub_dragging {
                st.scrub_dragging = false;
                unsafe { ReleaseCapture() };
                let dur = st.media.duration_sec();
                if dur > 0.0 {
                    let target_sec = (st.scrub_ratio(x) as f64) * dur;
                    let target_100ns = st.media.start_100ns + (target_sec * 10_000_000.0) as i64;
                    st.media.position_100ns = target_100ns;
                    st.media.last_updated_ft = crate::smtc::now_filetime();
                    st.smtc.send(MediaCommand::Seek(target_100ns));
                }
                st.hover_target = st.hit_test(x, y);
                st.sync_frame_timer();
                st.render_now();
                return 0;
            }
            let pressed = core::mem::replace(&mut st.pressed, HoverTarget::None);
            if pressed != HoverTarget::None {
                if st.hit_test(x, y) == pressed {
                    match pressed {
                        HoverTarget::Prev => st.smtc.send(MediaCommand::Previous),
                        HoverTarget::Play => st.smtc.send(MediaCommand::PlayPause),
                        HoverTarget::Next => st.smtc.send(MediaCommand::Next),
                        HoverTarget::App => app::activate(&st.media.aumid, &st.media.title),
                        _ => {}
                    }
                }
                st.render_now();
            }
            return 0;
        }
        WM_CAPTURECHANGED => {
            st.scrub_dragging = false;
            st.pressed = HoverTarget::None;
            st.sync_frame_timer();
            return 0;
        }
        WM_RBUTTONUP => {
            st.show_context_menu();
            return 0;
        }
        WM_DESTROY => {
            unsafe {
                KillTimer(hwnd, TIMER_FRAME);
                KillTimer(hwnd, TIMER_HOUSEKEEP);
                KillTimer(hwnd, TIMER_ZORDER);
                PostQuitMessage(0);
            }
            return 0;
        }
        _ => {}
    }

    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

impl AppState {
    fn hit_test(&mut self, x: f32, y: f32) -> HoverTarget {
        let scrub = self.renderer.scrub_rect(&self.media);
        if in_rect(x, y, PREV_RECT) {
            HoverTarget::Prev
        } else if in_rect(x, y, PLAY_RECT) {
            HoverTarget::Play
        } else if in_rect(x, y, NEXT_RECT) {
            HoverTarget::Next
        } else if in_rect(x, y, scrub) {
            HoverTarget::Scrub
        } else if !self.media.aumid.is_empty() && in_rect(x, y, self.renderer.app_rect(&self.media, self.settings.visualizer)) {
            HoverTarget::App
        } else {
            HoverTarget::None
        }
    }

    fn scrub_ratio(&mut self, x: f32) -> f32 {
        let scrub = self.renderer.scrub_rect(&self.media);
        ((x - scrub.0) / (scrub.2 - scrub.0)).clamp(0.0, 1.0)
    }

    fn schedule_zorder_settle(&mut self) {
        self.zorder_checks_left = 6;
        unsafe { SetTimer(self.hwnd, TIMER_ZORDER, 60, None) };
    }

    fn stop_zorder_settle(&mut self) {
        if self.zorder_checks_left > 0 {
            self.zorder_checks_left = 0;
            unsafe { KillTimer(self.hwnd, TIMER_ZORDER) };
        }
    }

    fn lparam_to_dips(&self, lparam: LPARAM) -> (f32, f32) {
        let px_x = (lparam & 0xFFFF) as i16 as f32 - self.pad_px as f32;
        let px_y = ((lparam >> 16) & 0xFFFF) as i16 as f32 - self.pad_px as f32;
        let scale = (self.dpi.max(96)) as f32 / 96.0;
        (px_x / scale, px_y / scale)
    }

    fn update_audio_active(&self) {
        let on = !self.hidden && self.settings.visualizer && self.media.active && self.media.playing;
        self.audio.set_active(on);
    }

    fn needs_fast_timer(&self) -> bool {
        if self.hidden {
            return false;
        }
        let target_strip = if self.strip_hovered || self.scrub_dragging { 1.0 } else { 0.0 };
        if (self.strip_hover_alpha - target_strip).abs() > 0.005 {
            return true;
        }
        let target_scrub = if self.hover_target == HoverTarget::Scrub || self.scrub_dragging { 1.0 } else { 0.0 };
        if (self.scrub_active_alpha - target_scrub).abs() > 0.005 {
            return true;
        }
        for i in 0..3 {
            let t = if btn_index(self.hover_target) == Some(i) { 1.0 } else { 0.0 };
            if (self.btn_hover_alpha[i] - t).abs() > 0.005 {
                return true;
            }
        }
        self.settings.visualizer && self.media.active && self.media.playing
    }

    fn sync_frame_timer(&mut self) {
        let want = self.needs_fast_timer();
        if want && !self.fast_timer_on {
            self.last_frame_ms = unsafe { GetTickCount64() };
            unsafe { SetTimer(self.hwnd, TIMER_FRAME, 16, None) };
            self.fast_timer_on = true;
        } else if !want && self.fast_timer_on {
            unsafe { KillTimer(self.hwnd, TIMER_FRAME) };
            self.fast_timer_on = false;
        }
    }

    fn tick_animation(&mut self) {
        let now = unsafe { GetTickCount64() };
        let dt_ms = (now.saturating_sub(self.last_frame_ms)).clamp(1, 50) as f32;
        self.last_frame_ms = now;

        // Win11 Fluent hover timings: 85 ms in, 167 ms out
        let target_strip = if self.strip_hovered || self.scrub_dragging { 1.0 } else { 0.0 };
        self.strip_hover_alpha = step_fade(self.strip_hover_alpha, target_strip, dt_ms, 85.0, 167.0);

        for i in 0..3 {
            let t = if btn_index(self.hover_target) == Some(i) { 1.0 } else { 0.0 };
            self.btn_hover_alpha[i] = step_fade(self.btn_hover_alpha[i], t, dt_ms, 85.0, 150.0);
        }

        let target_scrub = if self.hover_target == HoverTarget::Scrub || self.scrub_dragging { 1.0 } else { 0.0 };
        self.scrub_active_alpha = step_fade(self.scrub_active_alpha, target_scrub, dt_ms, 110.0, 130.0);

        self.render_now();
        self.sync_frame_timer();
    }

    fn render_now(&mut self) {
        if self.hidden {
            return;
        }
        let bands = if self.settings.visualizer && self.media.playing { self.audio.read_bands() } else { [0.0; 5] };
        let frame = FrameState {
            media: &self.media,
            light_taskbar: self.light_taskbar,
            show_visualizer: self.settings.visualizer,
            bands,
            strip_hover_alpha: self.strip_hover_alpha,
            btn_hover_alpha: self.btn_hover_alpha,
            app_hover: self.hover_target == HoverTarget::App || self.pressed == HoverTarget::App,
            btn_pressed: btn_index(self.pressed),
            scrub_active_alpha: self.scrub_active_alpha,
            scrub_hover: self.hover_target == HoverTarget::Scrub,
            scrub_dragging: self.scrub_dragging,
            scrub_hover_ratio: self.scrub_hover_ratio,
            drag_pos_sec: self.drag_pos_sec,
        };
        self.renderer.render_to_window(self.hwnd, self.win_x, self.win_y, &frame);
    }

    fn dock_to_taskbar(&mut self, force: bool) {
        let tray = unsafe { FindWindowW(w!("Shell_TrayWnd"), null()) };
        let hide_for_idle = !self.media.active;

        if tray.is_null() {
            // Sandbox fallback (no Shell_TrayWnd)
            let hide = hide_for_idle;
            if hide {
                if !self.hidden || force {
                    self.hidden = true;
                    unsafe { ShowWindow(self.hwnd, SW_HIDE) };
                    self.update_audio_active();
                }
                return;
            }
            let sy = unsafe { GetSystemMetrics(SM_CYSCREEN) };
            let tb_h = DEFAULT_TASKBAR_H as i32;
            let m = window_metrics(96, tb_h);
            self.dpi = 96;
            self.renderer.set_layout(96, m);
            self.win_w = m.win_w;
            self.win_h = m.win_h;
            self.pad_px = m.pad;
            self.win_x = 0;
            self.win_y = sy - tb_h;
            self.hidden = false;
            unsafe {
                SetWindowPos(self.hwnd, (-1isize) as HWND, self.win_x, self.win_y, self.win_w, self.win_h, SWP_NOACTIVATE | SWP_SHOWWINDOW);
            }
            self.update_audio_active();
            return;
        }

        let owner_changed = tray != self.owned_tray;
        if owner_changed {
            // Win32 owner relationship keeps this strip atomically above Shell_TrayWnd on every frame
            unsafe { SetWindowLongPtrW(self.hwnd, GWLP_HWNDPARENT, tray as isize) };
            self.owned_tray = tray;
        }

        let mut tb = RECT { left: 0, top: 0, right: 0, bottom: 0 };
        if unsafe { GetWindowRect(tray, &mut tb) } == 0 {
            return;
        }
        let dpi = unsafe { GetDpiForWindow(tray) }.max(96);
        let tb_h = tb.bottom - tb.top;
        // Hover area spans the taskbar edge-to-edge (no L/T/B padding); the strip content inside
        // keeps equal left/top/bottom padding (`m.pad`, 4 DIPs on a standard 48 DIP taskbar).
        let m = window_metrics(dpi, tb_h);

        let x = tb.left;
        let y = tb.top;
        let w = m.win_w;
        let h = m.win_h;

        let should_hide = hide_for_idle || is_monitor_fullscreen(tray, self.hwnd);
        let visible_now = unsafe { IsWindowVisible(self.hwnd) } != 0;
        let need_show = !should_hide && !visible_now;
        let below_tray = !should_hide && is_below_window(self.hwnd, tray);
        let dpi_changed = dpi != self.dpi;
        let layout_changed = w != self.win_w || h != self.win_h || m.pad != self.pad_px || dpi_changed;
        let geom_changed = force || owner_changed || x != self.win_x || y != self.win_y || layout_changed;
        let vis_changed = should_hide != self.hidden || need_show;
        if !geom_changed && !vis_changed && !below_tray {
            return;
        }

        let was_hidden = self.hidden;
        self.win_x = x;
        self.win_y = y;
        self.win_w = w;
        self.win_h = h;
        self.pad_px = m.pad;
        self.dpi = dpi;
        self.hidden = should_hide;
        self.renderer.set_layout(dpi, m);

        if should_hide {
            self.stop_zorder_settle();
            unsafe { SetWindowPos(self.hwnd, null_mut(), x, y, w, h, SWP_NOACTIVATE | SWP_NOZORDER | SWP_HIDEWINDOW) };
        } else {
            unsafe { SetWindowPos(self.hwnd, (-1isize) as HWND, x, y, w, h, SWP_NOACTIVATE | SWP_SHOWWINDOW) };
            if was_hidden || need_show || layout_changed {
                self.render_now();
            }
            if was_hidden || need_show {
                self.schedule_zorder_settle();
            }
        }
        self.update_audio_active();
        self.sync_frame_timer();
    }

    fn show_context_menu(&mut self) {
        let menu = unsafe { CreatePopupMenu() };
        if menu.is_null() {
            return;
        }
        let vis_flags = MF_STRING | if self.settings.visualizer { MF_CHECKED } else { 0 };

        unsafe {
            AppendMenuW(menu, MF_STRING, MENU_PLAY_PAUSE, if self.media.playing { w!("Pause") } else { w!("Play") });
            AppendMenuW(menu, MF_STRING, MENU_PREV, w!("Previous track"));
            AppendMenuW(menu, MF_STRING, MENU_NEXT, w!("Next track"));
            AppendMenuW(menu, MF_SEPARATOR, 0, null());
            AppendMenuW(menu, vis_flags, MENU_VIS_TOGGLE, w!("Live audio frequency meter"));
            AppendMenuW(menu, MF_SEPARATOR, 0, null());
            AppendMenuW(menu, MF_STRING, MENU_EXIT, w!("Exit NowPlaying"));
        }

        let mut pt = POINT { x: 0, y: 0 };
        unsafe { GetCursorPos(&mut pt) };
        let cmd = unsafe {
            TrackPopupMenu(
                menu,
                TPM_LEFTALIGN | TPM_BOTTOMALIGN | TPM_RIGHTBUTTON | TPM_RETURNCMD | TPM_NONOTIFY,
                pt.x,
                pt.y,
                0,
                self.hwnd,
                null(),
            ) as usize
        };
        unsafe { DestroyMenu(menu) };

        match cmd {
            MENU_PLAY_PAUSE => self.smtc.send(MediaCommand::PlayPause),
            MENU_PREV => self.smtc.send(MediaCommand::Previous),
            MENU_NEXT => self.smtc.send(MediaCommand::Next),
            MENU_VIS_TOGGLE => {
                self.settings.visualizer = !self.settings.visualizer;
                let _ = self.settings.save();
                self.update_audio_active();
                self.sync_frame_timer();
                self.render_now();
            }
            MENU_EXIT => unsafe {
                DestroyWindow(self.hwnd);
            },
            _ => {}
        }
    }
}

fn step_fade(cur: f32, target: f32, dt_ms: f32, in_ms: f32, out_ms: f32) -> f32 {
    if (cur - target).abs() < 0.005 {
        return target;
    }
    let dur = if target > cur { in_ms } else { out_ms };
    let step = dt_ms / dur;
    if target > cur { (cur + step).min(1.0) } else { (cur - step).max(0.0) }
}

fn in_rect(x: f32, y: f32, r: (f32, f32, f32, f32)) -> bool {
    x >= r.0 && x <= r.2 && y >= r.1 && y <= r.3
}

fn btn_index(t: HoverTarget) -> Option<usize> {
    match t {
        HoverTarget::Prev => Some(0),
        HoverTarget::Play => Some(1),
        HoverTarget::Next => Some(2),
        _ => None,
    }
}

fn read_taskbar_dpi() -> u32 {
    let tray = unsafe { FindWindowW(w!("Shell_TrayWnd"), null()) };
    if tray.is_null() { 96 } else { unsafe { GetDpiForWindow(tray) }.max(96) }
}

/// Returns true if `other` sits above `hwnd` in the Z-order (so it can cover `hwnd`).
fn is_below_window(hwnd: HWND, other: HWND) -> bool {
    let mut cur = unsafe { GetWindow(hwnd, GW_HWNDPREV) };
    for _ in 0..512 {
        if cur.is_null() {
            return false;
        }
        if cur == other {
            return true;
        }
        cur = unsafe { GetWindow(cur, GW_HWNDPREV) };
    }
    false
}

fn is_shell_desktop_or_tray(hwnd: HWND) -> bool {
    let mut buf = [0u16; 32];
    let len = unsafe { GetClassNameW(hwnd, buf.as_mut_ptr(), buf.len() as i32) };
    if len <= 0 {
        return false;
    }
    let name = &buf[..len as usize];
    eq_ascii(name, b"Progman")
        || eq_ascii(name, b"WorkerW")
        || eq_ascii(name, b"Shell_TrayWnd")
        || eq_ascii(name, b"Shell_SecondaryTrayWnd")
}

fn eq_ascii(wide: &[u16], ascii: &[u8]) -> bool {
    wide.len() == ascii.len() && wide.iter().zip(ascii.iter()).all(|(&w, &a)| w == a as u16)
}

fn is_monitor_fullscreen(tray: HWND, own: HWND) -> bool {
    let fg = unsafe { GetForegroundWindow() };
    if fg.is_null() || fg == tray || fg == own || fg == unsafe { GetShellWindow() } {
        return false;
    }
    if unsafe { IsWindowVisible(fg) == 0 || IsIconic(fg) != 0 } {
        return false;
    }
    let mut cloaked = 0u32;
    if unsafe { DwmGetWindowAttribute(fg, DWMWA_CLOAKED as u32, &mut cloaked as *mut u32 as *mut c_void, 4) } >= 0
        && cloaked != 0
    {
        return false;
    }
    if is_shell_desktop_or_tray(fg) {
        return false;
    }
    let mut fg_rc = RECT { left: 0, top: 0, right: 0, bottom: 0 };
    if unsafe { GetWindowRect(fg, &mut fg_rc) } == 0 {
        return false;
    }
    let mon = unsafe { MonitorFromWindow(tray, MONITOR_DEFAULTTONEAREST) };
    if mon.is_null() {
        return false;
    }
    let mut mi = MONITORINFO {
        cbSize: core::mem::size_of::<MONITORINFO>() as u32,
        ..unsafe { core::mem::zeroed() }
    };
    if unsafe { GetMonitorInfoW(mon, &mut mi) } == 0 {
        return false;
    }
    fg_rc.left <= mi.rcMonitor.left
        && fg_rc.top <= mi.rcMonitor.top
        && fg_rc.right >= mi.rcMonitor.right
        && fg_rc.bottom >= mi.rcMonitor.bottom
}
