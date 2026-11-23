//! SnipSnap Recorder (`snipsnap-rec.exe`): on-demand GPU screen recorder spawned by `snipsnap.exe`.
//!
//! Lifecycle:
//! 1. Spawned with `--warm` the moment a recording hotkey fires -> creates `SnipSnapRec` message
//!    window immediately and pre-warms D3D11 + WinRT Direct3D + Media Foundation on a worker thread.
//! 2. Receives `RecTarget` via `WM_COPYDATA` (or directly via CLI args) -> shows the Armed notch
//!    (`D`) + `B2` muted corner-tick border (`WDA_EXCLUDEFROMCAPTURE`).
//! 3. Keys `1`–`4` (or clicking the pill) pick `480` / `720` (default) / `1080` / `Native`;
//!    `Enter` / `R` (or clicking the shutter circle) starts recording with the 340 ms `S2 + B2`
//!    transition; `Esc` cancels cleanly.
//! 4. Pressing the stop circle or re-pressing any recording hotkey stops recording, finalizes the
//!    H.264 `.mp4` in `Videos\SnipSnap\`, drops all GPU/WGC/MF handles immediately, flashes,
//!    copies `CF_HDROP` to the clipboard, shows a native Windows 11 notification, and exits.
#![no_std]
#![no_main]

extern crate alloc;

mod capture;
mod encode;
mod finish;
mod gpu;
mod ipc;
mod ui;

use alloc::boxed::Box;
use alloc::string::String;
use capture::CaptureSession;
use core::ffi::c_void;
use core::ptr::{null, null_mut};
use core::sync::atomic::{AtomicBool, Ordering};
use gpu::GpuContext;
use ipc::{
    COPYDATA_TARGET, Fps, Quality, REC_CLASS, REC_MUTEX, RecKind, RecTarget, WM_APP_PREP_DONE, WM_APP_REC_ENDED, WM_APP_SAVE_DONE,
    WM_APP_STOP, WM_APP_WARM_DONE,
};
use pk::sync::Mutex;
use ui::{DRUM_ROLL_MS, OverlayUi, PillAction, TRANSITION_MS, UiState, ease_out_expo_like};
use windows::Win32::System::WinRT::{RO_INIT_MULTITHREADED, RO_INIT_SINGLETHREADED, RoInitialize};
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows_sys::Win32::Graphics::Dwm::{DWMWA_EXTENDED_FRAME_BOUNDS, DwmGetWindowAttribute};
use windows_sys::Win32::Graphics::Gdi::{GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromPoint};
use windows_sys::Win32::System::DataExchange::COPYDATASTRUCT;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::SystemInformation::GetTickCount64;
use windows_sys::Win32::UI::HiDpi::{DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, SetProcessDpiAwarenessContext};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    MOD_NOREPEAT, RegisterHotKey, UnregisterHotKey, VK_DOWN, VK_ESCAPE, VK_LEFT, VK_RETURN, VK_RIGHT, VK_UP,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GWLP_USERDATA, GetCursorPos, GetMessageW, GetWindowLongPtrW,
    GetWindowRect, HWND_MESSAGE, IsIconic, IsWindow, KillTimer, MSG, PostMessageW, PostQuitMessage, RegisterClassW, SetProcessDPIAware,
    SetTimer, SetWindowLongPtrW, TranslateMessage, WM_CLOSE, WM_COPYDATA, WM_DESTROY, WM_HOTKEY, WM_TIMER, WNDCLASSW,
};

pk::entry!(run);

const TIMER_IDLE_TIMEOUT: usize = 1;
const TIMER_UI_TICK: usize = 2;
const TIMER_RECONFIG: usize = 3;
const RECONFIG_DEBOUNCE_MS: u32 = 180;

// Temporary armed-only hotkeys (unregistered the instant recording starts or cancels).
const HK_Q1: i32 = 10;
const HK_Q2: i32 = 11;
const HK_Q3: i32 = 12;
const HK_Q4: i32 = 13;
const HK_UP: i32 = 14;
const HK_DOWN: i32 = 15;
const HK_LEFT: i32 = 16;
const HK_RIGHT: i32 = 17;
const HK_F: i32 = 18;
const HK_ENTER: i32 = 19;
const HK_R: i32 = 20;
const HK_ESC: i32 = 21;

static WARM_SLOT: Mutex<Option<Option<GpuContext>>> = Mutex::new(None);
static PREP_SLOT: Mutex<Option<Option<CaptureSession>>> = Mutex::new(None);
static SAVE_SLOT: Mutex<Option<Option<(String, u64)>>> = Mutex::new(None);
static PREP_CANCELLED: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Warming,
    Armed,
    WaitingPrepToRecord,
    Recording,
    Done,
}

struct App {
    host: HWND,
    phase: Phase,
    gpu: Option<GpuContext>,
    warm_failed: bool,
    prepared: Option<CaptureSession>,
    prep_in_flight: bool,
    prep_failed: bool,
    prep_gen: u32,
    prep_thread: Option<pk::thread::Thread>,
    reconfig_pending: bool,
    target: Option<RecTarget>,
    ui: Option<OverlayUi>,
    ui_state: UiState,
    armed_keys_on: bool,
    fast_timer: bool,
    drum_from: f32,
    drum_to: f32,
    drum_t0: u64,
    drum_rolling: bool,
    fps_from: f32,
    fps_to: f32,
    fps_t0: u64,
    fps_rolling: bool,
    trans_t0: u64,
    rec_t0: u64,
    session: Option<CaptureSession>,
    stopping: Option<(CaptureSession, i64, RECT, bool)>,
}

impl App {
    fn register_armed_keys(&mut self) {
        if self.armed_keys_on {
            return;
        }
        unsafe {
            RegisterHotKey(self.host, HK_Q1, MOD_NOREPEAT, b'1' as u32);
            RegisterHotKey(self.host, HK_Q2, MOD_NOREPEAT, b'2' as u32);
            RegisterHotKey(self.host, HK_Q3, MOD_NOREPEAT, b'3' as u32);
            RegisterHotKey(self.host, HK_Q4, MOD_NOREPEAT, b'4' as u32);
            RegisterHotKey(self.host, HK_UP, MOD_NOREPEAT, VK_UP as u32);
            RegisterHotKey(self.host, HK_DOWN, MOD_NOREPEAT, VK_DOWN as u32);
            RegisterHotKey(self.host, HK_LEFT, MOD_NOREPEAT, VK_LEFT as u32);
            RegisterHotKey(self.host, HK_RIGHT, MOD_NOREPEAT, VK_RIGHT as u32);
            RegisterHotKey(self.host, HK_F, MOD_NOREPEAT, b'F' as u32);
            RegisterHotKey(self.host, HK_ENTER, MOD_NOREPEAT, VK_RETURN as u32);
            RegisterHotKey(self.host, HK_R, MOD_NOREPEAT, b'R' as u32);
            RegisterHotKey(self.host, HK_ESC, MOD_NOREPEAT, VK_ESCAPE as u32);
        }
        self.armed_keys_on = true;
    }

    fn set_quality(&mut self, q: Quality) {
        if self.phase != Phase::Armed || self.ui_state.quality == q {
            return;
        }
        self.ui_state.quality = q;
        self.drum_from = self.ui_state.drum_pos;
        self.drum_to = q.index() as f32;
        self.drum_t0 = unsafe { GetTickCount64() };
        self.drum_rolling = true;
        self.fast_timer = true;
        self.reconfig_pending = true;
        unsafe {
            SetTimer(self.host, TIMER_UI_TICK, 16, None);
            SetTimer(self.host, TIMER_RECONFIG, RECONFIG_DEBOUNCE_MS, None);
        }
    }

    fn step_quality(&mut self, delta: i32) {
        if self.phase != Phase::Armed {
            return;
        }
        let cur = self.ui_state.quality.index() as i32;
        let next = (cur + delta).clamp(0, 3) as usize;
        self.set_quality(Quality::from_index(next));
    }

    fn set_fps(&mut self, fps: Fps) {
        if self.phase != Phase::Armed || self.ui_state.fps == fps {
            return;
        }
        self.ui_state.fps = fps;
        self.fps_from = self.ui_state.fps_drum_pos;
        self.fps_to = fps.index() as f32;
        self.fps_t0 = unsafe { GetTickCount64() };
        self.fps_rolling = true;
        self.fast_timer = true;
        self.reconfig_pending = true;
        unsafe {
            SetTimer(self.host, TIMER_UI_TICK, 16, None);
            SetTimer(self.host, TIMER_RECONFIG, RECONFIG_DEBOUNCE_MS, None);
        }
    }

    fn step_fps(&mut self, delta: i32) {
        if self.phase != Phase::Armed {
            return;
        }
        let cur = self.ui_state.fps.index() as i32;
        let next = (cur + delta).clamp(0, 2) as usize;
        self.set_fps(Fps::from_index(next));
    }

    fn cycle_fps(&mut self) {
        if self.phase != Phase::Armed {
            return;
        }
        let next = (self.ui_state.fps.index() + 1) % 3;
        self.set_fps(Fps::from_index(next));
    }

    fn unregister_armed_keys(&mut self) {
        if !self.armed_keys_on {
            return;
        }
        for id in HK_Q1..=HK_ESC {
            unsafe { UnregisterHotKey(self.host, id) };
        }
        self.armed_keys_on = false;
    }

    /// Starts Stage 2 background preparation (WGC item/pool + D3D11 VideoProcessor + H.264 MFT)
    /// as soon as both `gpu` and `target` are available, so pressing Record is instantaneous.
    fn maybe_start_prepare(&mut self) {
        if self.prep_in_flight || self.prepared.is_some() || self.prep_failed {
            return;
        }
        let (Some(target), Some(gpu)) = (self.target, self.gpu.take()) else {
            return;
        };
        let quality = self.ui_state.quality;
        let fps = self.ui_state.fps;
        let host_addr = self.host as usize;
        self.prep_gen = self.prep_gen.wrapping_add(1);
        let gen_id = self.prep_gen;
        self.prep_in_flight = true;
        self.prep_thread = pk::thread::spawn(move || {
            let _ = unsafe { RoInitialize(RO_INIT_MULTITHREADED) };
            if PREP_CANCELLED.load(Ordering::Acquire) {
                drop(gpu);
                return;
            }
            let sess = CaptureSession::prepare(gpu, target, quality, fps, host_addr as HWND, gen_id);
            if PREP_CANCELLED.load(Ordering::Acquire) {
                if let Some(s) = sess {
                    let _ = s.stop(0);
                }
                return;
            }
            *PREP_SLOT.lock() = Some(sess);
            unsafe { PostMessageW(host_addr as HWND, WM_APP_PREP_DONE, 0, 0) };
        });
        if self.prep_thread.is_none() {
            self.prep_in_flight = false;
            self.prep_failed = true;
        }
    }

    /// Rebuilds only the staging `VideoEncoder` in the background when the user rolls either drum
    /// to a different preset while Armed, keeping `GpuContext` and the WGC frame pool hot.
    fn maybe_reconfigure(&mut self) {
        if self.prep_in_flight || self.prep_failed {
            return;
        }
        let target_q = self.ui_state.quality;
        let target_fps = self.ui_state.fps;
        if self.prepared.as_ref().map(|s| s.quality() == target_q && s.fps() == target_fps).unwrap_or(true) {
            return;
        }
        let Some(mut sess) = self.prepared.take() else {
            return;
        };
        let host_addr = self.host as usize;
        self.prep_gen = self.prep_gen.wrapping_add(1);
        let gen_id = self.prep_gen;
        self.prep_in_flight = true;
        self.prep_thread = pk::thread::spawn(move || {
            let _ = unsafe { RoInitialize(RO_INIT_MULTITHREADED) };
            if PREP_CANCELLED.load(Ordering::Acquire) {
                let _ = sess.stop(0);
                return;
            }
            let ok = sess.reconfigure(target_q, target_fps, gen_id);
            if !ok || PREP_CANCELLED.load(Ordering::Acquire) {
                let _ = sess.stop(0);
                if !PREP_CANCELLED.load(Ordering::Acquire) {
                    *PREP_SLOT.lock() = Some(None);
                    unsafe { PostMessageW(host_addr as HWND, WM_APP_PREP_DONE, 0, 0) };
                }
                return;
            }
            *PREP_SLOT.lock() = Some(Some(sess));
            unsafe { PostMessageW(host_addr as HWND, WM_APP_PREP_DONE, 0, 0) };
        });
        if self.prep_thread.is_none() {
            self.prep_in_flight = false;
            self.prep_failed = true;
        }
    }

    fn enter_armed(&mut self, target: RecTarget) {
        unsafe { KillTimer(self.host, TIMER_IDLE_TIMEOUT) };
        self.target = Some(target);
        self.phase = Phase::Armed;
        self.ui_state = UiState {
            target: target.rect,
            quality: Quality::Q720,
            fps: Fps::F60,
            drum_pos: Quality::Q720.index() as f32,
            fps_drum_pos: Fps::F60.index() as f32,
            recording: false,
            trans_t: 0.0,
            elapsed_ms: 0,
            bytes: 0,
            btn_hover: false,
        };
        self.drum_from = self.ui_state.drum_pos;
        self.drum_to = self.ui_state.drum_pos;
        self.drum_rolling = false;
        self.fps_from = self.ui_state.fps_drum_pos;
        self.fps_to = self.ui_state.fps_drum_pos;
        self.fps_rolling = false;
        if self.ui.is_none() {
            self.ui = OverlayUi::new(on_pill_action, on_pill_hover, self as *mut App as *mut c_void);
        }
        if let Some(ui) = self.ui.as_mut() {
            ui.render(&self.ui_state);
        }
        self.register_armed_keys();
        // Track window moves even while armed.
        if target.kind() == RecKind::Window {
            unsafe { SetTimer(self.host, TIMER_UI_TICK, 33, None) };
        }
        // Begin Stage 2 background prep immediately if Stage 1 GPU warm-up already landed.
        self.maybe_start_prepare();
    }

    fn begin_recording_now(&mut self) {
        let Some(mut sess) = self.prepared.take() else {
            self.cancel();
            return;
        };
        if !sess.begin() {
            let _ = sess.stop(0);
            self.cancel();
            return;
        }
        let now = unsafe { GetTickCount64() };
        self.session = Some(sess);
        self.phase = Phase::Recording;
        self.trans_t0 = now;
        self.rec_t0 = now;
        self.ui_state.recording = true;
        self.ui_state.trans_t = 0.0;
        self.ui_state.elapsed_ms = 0;
        self.ui_state.bytes = 0;
        if let Some(ui) = self.ui.as_mut() {
            ui.render(&self.ui_state);
        }
        // ~60 Hz during the 340 ms S2 + B2 transition (and smooth window tracking + live timer).
        self.fast_timer = true;
        unsafe { SetTimer(self.host, TIMER_UI_TICK, 16, None) };
    }

    fn start_recording(&mut self) {
        if self.phase != Phase::Armed && self.phase != Phase::WaitingPrepToRecord {
            return;
        }
        self.unregister_armed_keys();
        self.reconfig_pending = false;
        unsafe { KillTimer(self.host, TIMER_RECONFIG) };
        if self.target.is_none() || self.warm_failed || self.prep_failed {
            self.cancel();
            return;
        }
        // Fast path: pipeline already prepared for the selected presets -> call StartCapture() now!
        if self.prepared.as_ref().map(|s| s.quality() == self.ui_state.quality && s.fps() == self.ui_state.fps).unwrap_or(false) {
            self.begin_recording_now();
            return;
        }
        // Otherwise wait for the background prep / reconfigure to land and trigger immediately.
        self.phase = Phase::WaitingPrepToRecord;
        if self.prepared.is_some() {
            self.maybe_reconfigure();
        } else {
            self.maybe_start_prepare();
        }
    }

    fn stop_recording(&mut self) {
        if self.phase != Phase::Recording {
            self.cancel();
            return;
        }
        self.phase = Phase::Done;
        self.fast_timer = false;
        unsafe {
            KillTimer(self.host, TIMER_UI_TICK);
            KillTimer(self.host, TIMER_RECONFIG);
        }
        self.unregister_armed_keys();
        let elapsed_100ns = (unsafe { GetTickCount64() }.saturating_sub(self.rec_t0) as i64) * 10_000;
        let final_rect = self.ui_state.target;
        let flash_on = self.target.map(|t| t.flash != 0).unwrap_or(true);
        // Halt WGC capture right away (~0.2 ms) before hiding UI and firing the 0 ms flash so no
        // flash frame can ever land in the video while MP4 finalize runs behind the fade.
        if let Some(sess) = self.session.take() {
            sess.halt_capture();
            self.stopping = Some((sess, elapsed_100ns, final_rect, flash_on));
        }
        if let Some(ui) = self.ui.as_mut() {
            ui.hide();
        }
        self.ui = None;
        unsafe { PostQuitMessage(0) };
    }

    fn cancel(&mut self) {
        PREP_CANCELLED.store(true, Ordering::Release);
        self.phase = Phase::Done;
        self.fast_timer = false;
        self.reconfig_pending = false;
        unsafe {
            KillTimer(self.host, TIMER_IDLE_TIMEOUT);
            KillTimer(self.host, TIMER_UI_TICK);
            KillTimer(self.host, TIMER_RECONFIG);
        }
        self.unregister_armed_keys();
        if let Some(ui) = self.ui.as_mut() {
            ui.hide();
        }
        self.ui = None;
        if let Some(sess) = self.session.take() {
            let _ = sess.stop(0);
        }
        if let Some(sess) = self.prepared.take() {
            let _ = sess.stop(0);
        }
        self.gpu = None;
        unsafe { PostQuitMessage(0) };
    }

    fn on_tick(&mut self) {
        let now = unsafe { GetTickCount64() };
        let mut dirty = false;

        // Track target window moves (and stop if it closes or minimizes).
        if let Some(t) = self.target
            && t.kind() == RecKind::Window
        {
            let hwnd = t.hwnd as HWND;
            if unsafe { IsWindow(hwnd) } == 0 {
                if self.phase == Phase::Recording {
                    self.stop_recording();
                } else {
                    self.cancel();
                }
                return;
            }
            if unsafe { IsIconic(hwnd) } == 0
                && let Some(r) = window_bounds(hwnd)
                && (r.left != self.ui_state.target.left
                    || r.top != self.ui_state.target.top
                    || r.right != self.ui_state.target.right
                    || r.bottom != self.ui_state.target.bottom)
            {
                self.ui_state.target = r;
                dirty = true;
            }
        }

        if self.drum_rolling {
            let t = ((now.saturating_sub(self.drum_t0)) as f32 / DRUM_ROLL_MS).clamp(0.0, 1.0);
            let e = ease_out_expo_like(t);
            self.ui_state.drum_pos = self.drum_from + (self.drum_to - self.drum_from) * e;
            dirty = true;
            if t >= 1.0 {
                self.ui_state.drum_pos = self.drum_to;
                self.drum_rolling = false;
            }
        }
        if self.fps_rolling {
            let t = ((now.saturating_sub(self.fps_t0)) as f32 / DRUM_ROLL_MS).clamp(0.0, 1.0);
            let e = ease_out_expo_like(t);
            self.ui_state.fps_drum_pos = self.fps_from + (self.fps_to - self.fps_from) * e;
            dirty = true;
            if t >= 1.0 {
                self.ui_state.fps_drum_pos = self.fps_to;
                self.fps_rolling = false;
            }
        }
        if self.phase == Phase::Armed && self.fast_timer && !self.drum_rolling && !self.fps_rolling {
            self.fast_timer = false;
            if self.target.map(|t| t.kind() == RecKind::Window).unwrap_or(false) {
                unsafe { SetTimer(self.host, TIMER_UI_TICK, 33, None) };
            } else {
                unsafe { KillTimer(self.host, TIMER_UI_TICK) };
            }
        }

        if self.phase == Phase::Recording {
            let trans = ((now.saturating_sub(self.trans_t0)) as f32 / TRANSITION_MS).clamp(0.0, 1.0);
            let elapsed = now.saturating_sub(self.rec_t0);
            let bytes = self.session.as_ref().map(|s| s.live_bytes()).unwrap_or(0);

            if (trans - self.ui_state.trans_t).abs() > 0.001
                || (elapsed / 100) != (self.ui_state.elapsed_ms / 100)
                || (bytes / 8192) != (self.ui_state.bytes / 8192)
            {
                self.ui_state.trans_t = trans;
                self.ui_state.elapsed_ms = elapsed;
                self.ui_state.bytes = bytes;
                dirty = true;
            }

            // Once the 340 ms transition has settled, relax the timer once to 50 ms (or 33 ms for window tracking).
            if trans >= 1.0 && self.fast_timer && !self.drum_rolling && !self.fps_rolling {
                self.fast_timer = false;
                let period = if self.target.map(|t| t.kind() == RecKind::Window).unwrap_or(false) { 33 } else { 50 };
                unsafe { SetTimer(self.host, TIMER_UI_TICK, period, None) };
            }
        }

        if dirty && let Some(ui) = self.ui.as_mut() {
            ui.render(&self.ui_state);
        }
    }
}

fn on_pill_action(ctx: *mut c_void, action: PillAction) {
    let app = unsafe { &mut *(ctx as *mut App) };
    match action {
        PillAction::StepQuality(delta) => app.step_quality(delta),
        PillAction::StepFps(delta) => app.step_fps(delta),
        PillAction::ToggleRecord => {
            if app.phase == Phase::Armed {
                app.start_recording();
            } else if app.phase == Phase::Recording {
                app.stop_recording();
            }
        }
    }
}

fn on_pill_hover(ctx: *mut c_void, hover: bool) {
    let app = unsafe { &mut *(ctx as *mut App) };
    if app.ui_state.btn_hover != hover {
        app.ui_state.btn_hover = hover;
        if let Some(ui) = app.ui.as_mut() {
            ui.render(&app.ui_state);
        }
    }
}

fn window_bounds(hwnd: HWND) -> Option<RECT> {
    let mut r = RECT { left: 0, top: 0, right: 0, bottom: 0 };
    if unsafe { DwmGetWindowAttribute(hwnd, DWMWA_EXTENDED_FRAME_BOUNDS as u32, &mut r as *mut RECT as *mut c_void, 16) } < 0
        && unsafe { GetWindowRect(hwnd, &mut r) } == 0
    {
        return None;
    }
    if r.right <= r.left || r.bottom <= r.top {
        return None;
    }
    Some(r)
}

unsafe extern "system" fn host_proc(h: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    let ptr = unsafe { GetWindowLongPtrW(h, GWLP_USERDATA) } as *mut App;
    if ptr.is_null() {
        return unsafe { DefWindowProcW(h, msg, wp, lp) };
    }
    let app = unsafe { &mut *ptr };
    match msg {
        WM_COPYDATA => {
            let cds = unsafe { &*(lp as *const COPYDATASTRUCT) };
            if cds.dwData == COPYDATA_TARGET && cds.cbData as usize == size_of::<RecTarget>() && !cds.lpData.is_null() {
                let target = unsafe { *(cds.lpData as *const RecTarget) };
                if app.phase == Phase::Warming {
                    app.enter_armed(target);
                }
                return 1;
            }
            0
        }
        WM_APP_WARM_DONE => {
            let result = WARM_SLOT.lock().take().flatten();
            match result {
                Some(gpu) => {
                    app.gpu = Some(gpu);
                    if app.phase == Phase::Armed || app.phase == Phase::WaitingPrepToRecord {
                        app.maybe_start_prepare();
                    }
                }
                None => {
                    app.warm_failed = true;
                    if app.phase == Phase::WaitingPrepToRecord {
                        app.cancel();
                    }
                }
            }
            0
        }
        WM_APP_PREP_DONE => {
            app.prep_in_flight = false;
            app.prep_thread = None;
            let result = PREP_SLOT.lock().take().flatten();
            if app.phase == Phase::Done {
                if let Some(sess) = result {
                    let _ = sess.stop(0);
                }
                return 0;
            }
            match result {
                Some(sess) => {
                    let matches = sess.quality() == app.ui_state.quality && sess.fps() == app.ui_state.fps;
                    app.prepared = Some(sess);
                    if !matches {
                        if app.phase == Phase::WaitingPrepToRecord || !app.reconfig_pending {
                            app.maybe_reconfigure();
                        }
                    } else if app.phase == Phase::WaitingPrepToRecord {
                        app.begin_recording_now();
                    }
                }
                None => {
                    app.prep_failed = true;
                    if app.phase == Phase::WaitingPrepToRecord {
                        app.cancel();
                    }
                }
            }
            0
        }
        WM_APP_STOP | WM_APP_REC_ENDED => {
            if app.phase == Phase::Recording {
                app.stop_recording();
            } else {
                app.cancel();
            }
            0
        }
        WM_HOTKEY => {
            match wp as i32 {
                HK_Q1 => app.set_quality(Quality::Q480),
                HK_Q2 => app.set_quality(Quality::Q720),
                HK_Q3 => app.set_quality(Quality::Q1080),
                HK_Q4 => app.set_quality(Quality::Native),
                HK_UP => app.step_quality(-1),
                HK_DOWN => app.step_quality(1),
                HK_LEFT => app.step_fps(-1),
                HK_RIGHT => app.step_fps(1),
                HK_F => app.cycle_fps(),
                HK_ENTER | HK_R => app.start_recording(),
                HK_ESC => app.cancel(),
                _ => {}
            }
            0
        }
        WM_TIMER => {
            if wp == TIMER_IDLE_TIMEOUT && app.phase == Phase::Warming {
                app.cancel();
            } else if wp == TIMER_UI_TICK {
                app.on_tick();
            } else if wp == TIMER_RECONFIG {
                unsafe { KillTimer(h, TIMER_RECONFIG) };
                app.reconfig_pending = false;
                if app.phase == Phase::Armed || app.phase == Phase::WaitingPrepToRecord {
                    app.maybe_reconfigure();
                }
            }
            0
        }
        WM_CLOSE => {
            app.cancel();
            0
        }
        WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
            0
        }
        _ => unsafe { DefWindowProcW(h, msg, wp, lp) },
    }
}

fn parse_i32(s: &str) -> Option<i32> {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return None;
    }
    let (neg, rest) = if bytes[0] == b'-' { (true, &bytes[1..]) } else { (false, bytes) };
    if rest.is_empty() {
        return None;
    }
    let mut v: i64 = 0;
    for &b in rest {
        if !b.is_ascii_digit() {
            return None;
        }
        v = v * 10 + (b - b'0') as i64;
    }
    if neg { Some((-v) as i32) } else { Some(v as i32) }
}

fn cli_target() -> Option<RecTarget> {
    let args = pk::args::args();
    let cmd = args.first()?.as_str();
    if cmd == "--warm" {
        return None;
    }
    if cmd == "--full" {
        let mut p = POINT { x: 0, y: 0 };
        unsafe { GetCursorPos(&mut p) };
        let mon = unsafe { MonitorFromPoint(p, MONITOR_DEFAULTTONEAREST) };
        let mut mi: MONITORINFO = unsafe { core::mem::zeroed() };
        mi.cbSize = size_of::<MONITORINFO>() as u32;
        if unsafe { GetMonitorInfoW(mon, &mut mi) } == 0 {
            return None;
        }
        return Some(RecTarget { kind: 2, rect: mi.rcMonitor, hwnd: 0, monitor: mon as usize, flash: 1 });
    }
    if cmd == "--rect" && args.len() >= 5 {
        let rect =
            RECT { left: parse_i32(&args[1])?, top: parse_i32(&args[2])?, right: parse_i32(&args[3])?, bottom: parse_i32(&args[4])? };
        let p = POINT { x: (rect.left + rect.right) / 2, y: (rect.top + rect.bottom) / 2 };
        let mon = unsafe { MonitorFromPoint(p, MONITOR_DEFAULTTONEAREST) };
        return Some(RecTarget { kind: 0, rect, hwnd: 0, monitor: mon as usize, flash: 1 });
    }
    None
}

fn run() -> u32 {
    if unsafe { SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) } == 0 {
        unsafe { SetProcessDPIAware() };
    }
    let _ = unsafe { RoInitialize(RO_INIT_SINGLETHREADED) };

    let args = pk::args::args();
    if args.first().map(|s| s.as_str()) == Some("--help") || args.first().map(|s| s.as_str()) == Some("help") {
        let mut out = match pk::out::stdout_kind() {
            pk::out::StdoutKind::None => {
                if pk::out::attach_parent_console() {
                    pk::out::console_output().unwrap_or_else(pk::out::Writer::null)
                } else {
                    pk::out::Writer::null()
                }
            }
            _ => pk::out::Writer::new(pk::out::stdout_handle()),
        };
        out.write_bytes(
            b"snipsnap-rec - on-demand screen recorder spawned by snipsnap.exe\n\
              \x20 snipsnap-rec --warm                pre-warm GPU and wait for snipsnap.exe\n\
              \x20 snipsnap-rec --full                arm full-screen recording on cursor monitor\n\
              \x20 snipsnap-rec --rect L T R B        arm rectangle recording\n",
        );
        return 0;
    }

    let Some(mutex) = pk::instance::NamedMutex::first(REC_MUTEX) else {
        return 0;
    };

    let hinst = unsafe { GetModuleHandleW(null()) };
    let mut wc: WNDCLASSW = unsafe { core::mem::zeroed() };
    wc.lpfnWndProc = Some(host_proc);
    wc.hInstance = hinst;
    wc.lpszClassName = REC_CLASS;
    unsafe { RegisterClassW(&wc) };

    let host = unsafe { CreateWindowExW(0, REC_CLASS, pk::w!("SnipSnapRec"), 0, 0, 0, 0, 0, HWND_MESSAGE, null_mut(), hinst, null()) };
    if host.is_null() {
        return 2;
    }

    let mut app = Box::new(App {
        host,
        phase: Phase::Warming,
        gpu: None,
        warm_failed: false,
        prepared: None,
        prep_in_flight: false,
        prep_failed: false,
        prep_gen: 0,
        prep_thread: None,
        reconfig_pending: false,
        target: None,
        ui: None,
        ui_state: UiState {
            target: RECT { left: 0, top: 0, right: 0, bottom: 0 },
            quality: Quality::Q720,
            fps: Fps::F60,
            drum_pos: Quality::Q720.index() as f32,
            fps_drum_pos: Fps::F60.index() as f32,
            recording: false,
            trans_t: 0.0,
            elapsed_ms: 0,
            bytes: 0,
            btn_hover: false,
        },
        armed_keys_on: false,
        fast_timer: false,
        drum_from: Quality::Q720.index() as f32,
        drum_to: Quality::Q720.index() as f32,
        drum_t0: 0,
        drum_rolling: false,
        fps_from: Fps::F60.index() as f32,
        fps_to: Fps::F60.index() as f32,
        fps_t0: 0,
        fps_rolling: false,
        trans_t0: 0,
        rec_t0: 0,
        session: None,
        stopping: None,
    });
    unsafe { SetWindowLongPtrW(host, GWLP_USERDATA, (&mut *app as *mut App) as isize) };

    // Pre-warm D3D11 + WinRT Direct3D + Media Foundation on a background thread immediately.
    let host_addr = host as usize;
    let _ = pk::thread::spawn(move || {
        let ctx = GpuContext::init();
        *WARM_SLOT.lock() = Some(ctx);
        unsafe { PostMessageW(host_addr as HWND, WM_APP_WARM_DONE, 0, 0) };
    });

    if let Some(target) = cli_target() {
        app.enter_armed(target);
    } else {
        // Guard against leaked `--warm` if parent dies before handing off a target.
        unsafe { SetTimer(host, TIMER_IDLE_TIMEOUT, 60_000, None) };
    }

    let mut m: MSG = unsafe { core::mem::zeroed() };
    while unsafe { GetMessageW(&mut m, null_mut(), 0, 0) } > 0 {
        unsafe {
            TranslateMessage(&m);
            DispatchMessageW(&m);
        }
    }

    // Grab the halted capture session, then drop `app` so overlay Direct2D resources are freed.
    PREP_CANCELLED.store(true, Ordering::Release);
    let prep_thread = app.prep_thread.take();
    let stopping = app.stopping.take();
    unsafe { SetWindowLongPtrW(host, GWLP_USERDATA, 0) };
    drop(app);

    // Overlap the 150 ms white flash (0 ms start!) with the ~72 ms background MP4 finalize + GPU
    // teardown; copy CF_HDROP the instant the save lands mid-flash, then show the Win11 toast.
    if let Some((sess, elapsed_100ns, rect, flash_on)) = stopping {
        let host_addr = host as usize;
        let save_thread = pk::thread::spawn(move || {
            let _ = unsafe { RoInitialize(RO_INIT_MULTITHREADED) };
            let res = sess.stop(elapsed_100ns);
            *SAVE_SLOT.lock() = Some(res);
            unsafe { PostMessageW(host_addr as HWND, WM_APP_SAVE_DONE, 0, 0) };
        });

        let mut saved: Option<(String, u64)> = None;
        let mut copied = false;
        let mut handle_save = || {
            if saved.is_none()
                && let Some(res) = SAVE_SLOT.lock().take().flatten()
            {
                copied = finish::copy_file_to_clipboard(host, &res.0);
                saved = Some(res);
            }
        };

        if flash_on {
            finish::show_flash(rect, &mut handle_save);
        }
        if let Some(t) = save_thread {
            t.join();
        }
        handle_save();

        if let Some((path, bytes)) = saved {
            finish::show_saved_notification(host, &path, bytes, copied);
        }
    }

    unsafe { DestroyWindow(host) };
    // If cancelled mid-preparation, wait for the worker to close and delete its staging file cleanly.
    if let Some(t) = prep_thread {
        t.join();
    }
    if let Some(sess) = PREP_SLOT.lock().take().flatten() {
        let _ = sess.stop(0);
    }
    drop(mutex);
    0
}
