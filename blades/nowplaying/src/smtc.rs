//! Windows System Media Transport Controls (SMTC) client on an MTA background thread.
//! Reads live title, artist, source app, playback status, cover art bytes and timeline, and
//! executes play/pause, next, previous and seek commands asynchronously.
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};
use pk::sync::Mutex;
use pk_windows::Foundation::TypedEventHandler;
use pk_windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession, GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus,
};
use pk_windows::Storage::Streams::{Buffer, InputStreamOptions};
use pk_windows::Win32::System::Com::{COINIT_MULTITHREADED, CoInitializeEx};
use windows_sys::Win32::Foundation::{CloseHandle, FILETIME, HANDLE, HWND};
use windows_sys::Win32::System::SystemInformation::GetSystemTimeAsFileTime;
use windows_sys::Win32::System::Threading::{CreateEventW, INFINITE, SetEvent, WaitForSingleObject};
use windows_sys::Win32::UI::WindowsAndMessaging::PostMessageW;

#[derive(Clone)]
pub struct MediaSnapshot {
    pub active: bool,
    pub playing: bool,
    pub title: String,
    pub artist: String,
    /// The source app's AppUserModelID (its icon comes from the shell, see `app`).
    pub aumid: String,
    pub can_seek: bool,
    pub start_100ns: i64,
    pub end_100ns: i64,
    pub position_100ns: i64,
    /// Windows FILETIME (100-ns ticks since 1601-01-01 UTC) when `position_100ns` was reported.
    pub last_updated_ft: i64,
    /// Monotonic counter incremented only when `thumbnail` bytes actually changed.
    pub art_version: u64,
    pub thumbnail: Vec<u8>,
}

impl Default for MediaSnapshot {
    fn default() -> Self {
        Self {
            active: false,
            playing: false,
            title: String::new(),
            artist: String::new(),
            aumid: String::new(),
            can_seek: false,
            start_100ns: 0,
            end_100ns: 0,
            position_100ns: 0,
            last_updated_ft: 0,
            art_version: 0,
            thumbnail: Vec::new(),
        }
    }
}

impl MediaSnapshot {
    pub fn duration_sec(&self) -> f64 {
        let span = (self.end_100ns - self.start_100ns).max(0);
        (span as f64) / 10_000_000.0
    }

    /// Interpolates live playback position while playing (`position + (now - last_updated)`).
    pub fn live_position_sec(&self) -> f64 {
        let dur = self.duration_sec();
        if dur <= 0.0 {
            return 0.0;
        }
        let mut pos_100ns = self.position_100ns - self.start_100ns;
        if self.playing && self.last_updated_ft > 0 {
            let now = now_filetime();
            let delta = (now - self.last_updated_ft).clamp(0, 36_000_000_000);
            pos_100ns = pos_100ns.saturating_add(delta);
        }
        let sec = (pos_100ns as f64) / 10_000_000.0;
        sec.clamp(0.0, dur)
    }
}

#[derive(Clone, Copy)]
pub enum MediaCommand {
    PlayPause,
    Previous,
    Next,
    Seek(i64),
}

struct EventHandle(HANDLE);
unsafe impl Send for EventHandle {}
unsafe impl Sync for EventHandle {}

impl EventHandle {
    fn new() -> Self {
        Self(unsafe { CreateEventW(core::ptr::null(), 0, 0, core::ptr::null()) })
    }
    fn signal(&self) {
        if !self.0.is_null() {
            unsafe { SetEvent(self.0) };
        }
    }
    fn wait(&self, ms: u32) {
        if !self.0.is_null() {
            unsafe { WaitForSingleObject(self.0, ms) };
        }
    }
}

impl Drop for EventHandle {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { CloseHandle(self.0) };
        }
    }
}

struct Shared {
    snapshot: Mutex<MediaSnapshot>,
    commands: Mutex<Vec<MediaCommand>>,
    wake: EventHandle,
    stop: AtomicBool,
    notify_hwnd: usize,
    notify_msg: u32,
}

pub struct SmtcClient {
    shared: Arc<Shared>,
}

impl SmtcClient {
    pub fn start(notify_hwnd: HWND, notify_msg: u32) -> Self {
        let shared = Arc::new(Shared {
            snapshot: Mutex::new(MediaSnapshot::default()),
            commands: Mutex::new(Vec::new()),
            wake: EventHandle::new(),
            stop: AtomicBool::new(false),
            notify_hwnd: notify_hwnd as usize,
            notify_msg,
        });
        let s = shared.clone();
        let _ = pk::thread::spawn(move || worker_loop(s));
        Self { shared }
    }

    pub fn snapshot(&self) -> MediaSnapshot {
        self.shared.snapshot.lock().clone()
    }

    pub fn send(&self, cmd: MediaCommand) {
        self.shared.commands.lock().push(cmd);
        self.shared.wake.signal();
    }

    pub fn stop(&self) {
        self.shared.stop.store(true, Ordering::Relaxed);
        self.shared.wake.signal();
    }
}

/// One-shot read-only SMTC query for `NowPlaying.exe --status`.
pub fn read_status_once() -> Result<MediaSnapshot, String> {
    let slot = Arc::new((Mutex::new(None::<Result<MediaSnapshot, String>>), EventHandle::new()));
    let s = slot.clone();
    let started = pk::thread::spawn(move || {
        let _ = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
        let res = query_once_mta();
        *s.0.lock() = Some(res);
        s.1.signal();
    });
    if started.is_none() {
        return Err(String::from("Could not start SMTC worker thread."));
    }
    slot.1.wait(INFINITE);
    slot.0.lock().take().unwrap_or_else(|| Err(String::from("SMTC query returned no result.")))
}

fn query_once_mta() -> Result<MediaSnapshot, String> {
    let op = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
        .map_err(|e| alloc::format!("SMTC RequestAsync failed: {}", e.message()))?;
    let mgr = op.join().map_err(|e| alloc::format!("SMTC manager unavailable: {}", e.message()))?;
    let Some(session) = pick_session(&mgr) else {
        return Ok(MediaSnapshot::default());
    };
    let mut prev = MediaSnapshot::default();
    let mut last_key = String::new();
    read_session(&session, &mut prev, &mut last_key);
    Ok(prev)
}

fn worker_loop(shared: Arc<Shared>) {
    let _ = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };

    let mgr = loop {
        if shared.stop.load(Ordering::Relaxed) {
            return;
        }
        if let Ok(op) = GlobalSystemMediaTransportControlsSessionManager::RequestAsync() {
            if let Ok(m) = op.join() {
                break m;
            }
        }
        shared.wake.wait(1000);
    };

    // Wake worker whenever SMTC reports a session switch
    let s_cur = shared.clone();
    let _ = mgr.CurrentSessionChanged(&TypedEventHandler::new(move |_, _| {
        s_cur.wake.signal();
        Ok(())
    }));
    let s_list = shared.clone();
    let _ = mgr.SessionsChanged(&TypedEventHandler::new(move |_, _| {
        s_list.wake.signal();
        Ok(())
    }));

    let mut attached_session: Option<GlobalSystemMediaTransportControlsSession> = None;
    let mut attached_aumid = String::new();
    let mut current = MediaSnapshot::default();
    let mut last_art_key = String::new();

    while !shared.stop.load(Ordering::Relaxed) {
        // 1. Drain any pending UI commands first
        let cmds: Vec<MediaCommand> = {
            let mut q = shared.commands.lock();
            core::mem::take(&mut *q)
        };
        if !cmds.is_empty() {
            if let Some(session) = attached_session.as_ref().cloned().or_else(|| pick_session(&mgr)) {
                for cmd in cmds {
                    execute_command(&session, cmd);
                }
            }
        }

        // 2. Refresh active session & attach event handlers if session changed
        let picked = pick_session(&mgr);
        match picked {
            Some(session) => {
                let aumid = session.SourceAppUserModelId().map(|s| s.to_string_lossy()).unwrap_or_default();
                let need_attach = attached_session.is_none() || aumid != attached_aumid;
                if need_attach {
                    let s1 = shared.clone();
                    let _ = session.MediaPropertiesChanged(&TypedEventHandler::new(move |_, _| {
                        s1.wake.signal();
                        Ok(())
                    }));
                    let s2 = shared.clone();
                    let _ = session.PlaybackInfoChanged(&TypedEventHandler::new(move |_, _| {
                        s2.wake.signal();
                        Ok(())
                    }));
                    let s3 = shared.clone();
                    let _ = session.TimelinePropertiesChanged(&TypedEventHandler::new(move |_, _| {
                        s3.wake.signal();
                        Ok(())
                    }));
                    attached_aumid = aumid;
                    attached_session = Some(session.clone());
                }
                read_session(&session, &mut current, &mut last_art_key);
            }
            None => {
                attached_session = None;
                attached_aumid.clear();
                if current.active {
                    current = MediaSnapshot::default();
                    last_art_key.clear();
                }
            }
        }

        *shared.snapshot.lock() = current.clone();
        if shared.notify_hwnd != 0 {
            unsafe { PostMessageW(shared.notify_hwnd as HWND, shared.notify_msg, 0, 0) };
        }

        // Periodic light poll (1.5 s) catches players that skip timeline/property events
        shared.wake.wait(1500);
    }
}

fn pick_session(mgr: &GlobalSystemMediaTransportControlsSessionManager) -> Option<GlobalSystemMediaTransportControlsSession> {
    if let Ok(cur) = mgr.GetCurrentSession() {
        if is_usable_session(&cur) {
            return Some(cur);
        }
    }
    if let Ok(list) = mgr.GetSessions() {
        let count = list.Size().unwrap_or(0);
        let mut fallback = None;
        for i in 0..count {
            if let Ok(s) = list.GetAt(i) {
                if let Ok(info) = s.GetPlaybackInfo() {
                    if let Ok(st) = info.PlaybackStatus() {
                        if st == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing {
                            return Some(s);
                        }
                        if fallback.is_none() && st == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Paused {
                            fallback = Some(s);
                        }
                    }
                }
            }
        }
        if fallback.is_some() {
            return fallback;
        }
    }
    mgr.GetCurrentSession().ok()
}

fn is_usable_session(s: &GlobalSystemMediaTransportControlsSession) -> bool {
    s.GetPlaybackInfo()
        .and_then(|i| i.PlaybackStatus())
        .is_ok_and(|st| {
            st == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing
                || st == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Paused
        })
}

fn read_session(session: &GlobalSystemMediaTransportControlsSession, out: &mut MediaSnapshot, last_art_key: &mut String) {
    let aumid = session.SourceAppUserModelId().map(|s| s.to_string_lossy()).unwrap_or_default();
    let mut playing = false;
    let mut can_seek = false;
    let mut active = false;

    if let Ok(info) = session.GetPlaybackInfo() {
        if let Ok(st) = info.PlaybackStatus() {
            playing = st == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing;
            active = playing || st == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Paused;
        }
        if let Ok(ctrl) = info.Controls() {
            can_seek = ctrl.IsPlaybackPositionEnabled().unwrap_or(false);
        }
    }

    let mut start_100ns = 0i64;
    let mut end_100ns = 0i64;
    let mut position_100ns = 0i64;
    let mut last_updated_ft = 0i64;
    if let Ok(tl) = session.GetTimelineProperties() {
        start_100ns = tl.StartTime().map(|t| t.Duration).unwrap_or(0);
        end_100ns = tl.EndTime().map(|t| t.Duration).unwrap_or(0);
        position_100ns = tl.Position().map(|t| t.Duration).unwrap_or(0);
        last_updated_ft = tl.LastUpdatedTime().map(|d| d.UniversalTime).unwrap_or(0);
    }

    let mut title = out.title.clone();
    let mut artist = out.artist.clone();

    if let Ok(op) = session.TryGetMediaPropertiesAsync() {
        if let Ok(props) = op.join() {
            title = props.Title().map(|s| s.to_string_lossy()).unwrap_or_default();
            artist = props.Artist().map(|s| s.to_string_lossy()).unwrap_or_default();
            if !title.is_empty() {
                active = true;
            }

            // Re-read thumbnail only when the track identity changes or we don't have art yet
            let art_key = alloc::format!("{aumid}|{title}|{artist}");
            if art_key != *last_art_key || out.thumbnail.is_empty() {
                let mut bytes = Vec::new();
                if let Ok(thumb_ref) = props.Thumbnail() {
                    if let Some(b) = read_thumbnail_bytes(&thumb_ref) {
                        bytes = b;
                    }
                }
                if !bytes.is_empty() || art_key != *last_art_key {
                    out.thumbnail = bytes;
                    out.art_version = out.art_version.wrapping_add(1);
                    *last_art_key = art_key;
                }
            }
        }
    }

    out.active = active && !title.is_empty();
    out.playing = playing;
    out.title = title;
    out.artist = artist;
    out.aumid = aumid;
    out.can_seek = can_seek || (end_100ns > start_100ns);
    out.start_100ns = start_100ns;
    out.end_100ns = end_100ns;
    out.position_100ns = position_100ns;
    out.last_updated_ft = last_updated_ft;
}

fn read_thumbnail_bytes(thumb_ref: &pk_windows::Storage::Streams::IRandomAccessStreamReference) -> Option<Vec<u8>> {
    let stream = thumb_ref.OpenReadAsync().ok()?.join().ok()?;
    let size = stream.Size().ok()?;
    if size == 0 || size > 4 * 1024 * 1024 {
        return None;
    }
    let cap = size as u32;
    let buf = Buffer::Create(cap).ok()?;
    let read_buf = stream.ReadAsync(&buf, cap, InputStreamOptions::None).ok()?.join().ok()?;
    let len = read_buf.Length().ok()? as usize;
    if len == 0 {
        return None;
    }
    let reader = pk_windows::Storage::Streams::DataReader::FromBuffer(&read_buf).ok()?;
    let mut out = vec![0u8; len];
    reader.ReadBytes(&mut out).ok()?;
    Some(out)
}

fn execute_command(session: &GlobalSystemMediaTransportControlsSession, cmd: MediaCommand) {
    match cmd {
        MediaCommand::PlayPause => {
            if let Ok(op) = session.TryTogglePlayPauseAsync() {
                let _ = op.join();
            }
        }
        MediaCommand::Previous => {
            if let Ok(op) = session.TrySkipPreviousAsync() {
                let _ = op.join();
            }
        }
        MediaCommand::Next => {
            if let Ok(op) = session.TrySkipNextAsync() {
                let _ = op.join();
            }
        }
        MediaCommand::Seek(target_100ns) => {
            if let Ok(op) = session.TryChangePlaybackPositionAsync(target_100ns) {
                let _ = op.join();
            }
        }
    }
}

pub fn now_filetime() -> i64 {
    let mut ft = FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 };
    unsafe { GetSystemTimeAsFileTime(&mut ft) };
    ((ft.dwHighDateTime as i64) << 32) | (ft.dwLowDateTime as i64)
}
