//! `QuickTooth.exe <option>`: every command-line option (docs/specs/settings-and-cli.md). The
//! output text and exit codes are behaviour: scripts and the PocketKnives runner read them.
use crate::audio::Audio;
use crate::clock::Stopwatch;
use crate::worker::Worker;
use crate::{ble, files, icons, ipc, pnp, selftest, system};
use alloc::format;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::fmt::Write;
use pk::out::Writer;
use quicktooth_core::battery;
use quicktooth_core::blade_settings::{self, SetResult, SettingsChange};
use quicktooth_core::devices::{BtDevice, OneShot, device_state};
use quicktooth_core::host::tray_commands;
use quicktooth_core::settings::{Settings, default_target};
use windows_sys::Win32::Foundation::GetLastError;
use windows_sys::Win32::System::DataExchange::COPYDATASTRUCT;
use windows_sys::Win32::System::Threading::{GetCurrentProcess, GetProcessHandleCount, Sleep};
use windows_sys::Win32::UI::Shell::{NOTIFYICONIDENTIFIER, Shell_NotifyIconGetRect};
use windows_sys::Win32::UI::WindowsAndMessaging::{PostMessageW, SMTO_ABORTIFHUNG, SendMessageTimeoutW, WM_COPYDATA};

/// One line, CRLF (.NET's `WriteLine` on Windows).
macro_rules! wl {
    ($out:expr) => { $out.write_bytes(b"\r\n") };
    ($out:expr, $($t:tt)*) => {{ let _ = write!($out, $($t)*); $out.write_bytes(b"\r\n"); }};
}

/// A command's exit code, or the text of what went wrong (printed as `Error: …`, exit 1).
type Outcome = Result<u32, String>;

pub fn run(args: &[String]) -> u32 {
    let mut out = open_console();
    match dispatch(&mut out, args) {
        Ok(code) => code,
        Err(e) => {
            wl!(out, "Error: {e}");
            quicktooth_core::log::error("cli", &e);
            1
        }
    }
}

/// To stdout when it's a file or a pipe (UTF-8); otherwise to the parent's console, after a
/// blank line, as the C# version did; otherwise nowhere.
fn open_console() -> Writer {
    use pk::out::{StdoutKind, Writer, stdout_handle, stdout_kind};
    if stdout_kind() == StdoutKind::FileOrPipe {
        return Writer::new(stdout_handle());
    }
    if pk::out::attach_parent_console() {
        if let Some(mut con) = pk::out::console_output() {
            wl!(con);
            return con;
        }
    }
    Writer::null()
}

fn dispatch(out: &mut Writer, args: &[String]) -> Outcome {
    // Every option is ASCII, so ASCII lower-casing matches what ToLowerInvariant matched.
    let cmd = args[0].to_ascii_lowercase();
    let has = |flag: &str| args.iter().any(|a| a.eq_ignore_ascii_case(flag));
    match cmd.as_str() {
        "--list" => Ok(list(out, has("--all"))),
        "--battery" => Ok(battery_cmd(out)),
        "--settings" => Ok(settings_cmd(out)),
        "--set" => Ok(set_cmd(out, &args[1..])),
        "--toggle" => action(out, None, !has("--no-wait"), has("--direct"), has("--a2dp-only")),
        "--connect" => action(out, Some(OneShot::Connect), !has("--no-wait"), has("--direct"), has("--a2dp-only")),
        "--disconnect" => action(out, Some(OneShot::Disconnect), !has("--no-wait"), has("--direct"), has("--a2dp-only")),
        "--exit" => Ok(exit_tray(out)),
        "--autostart" => Ok(autostart(out, args.get(1).map_or("status", String::as_str))),
        "--render-icons" => render_icons(out, args),
        "--preview-menu" => Ok(crate::menu_preview::run(arg(args, 1)?)),
        "--write-setting-pictures" => write_setting_pictures(out, arg(args, 1)?),
        "--icon-rect" => Ok(icon_rect(out)),
        "--probe" => probe(out, arg(args, 1)?),
        // test hooks: open or close the tray menu without touching the mouse
        "--show-menu" => Ok(forward(out, tray_commands::SHOW_MENU)),
        "--close-menu" => Ok(forward(out, tray_commands::CLOSE_MENU)),
        // test hooks (read-only): settings load/save fidelity, and COM release under load
        "--settings-roundtrip" => {
            let dest = arg(args, 2)?;
            let s = files::load_settings_from(arg(args, 1)?);
            files::write_file(dest, s.to_json("\r\n").as_bytes())?;
            Ok(0)
        }
        "--stress-enumerate" => Ok(stress_enumerate(out, parse_int(arg(args, 1)?)?)),
        "--watch-endpoints" => watch_endpoints(out, parse_int(arg(args, 1)?)?),
        "--watch-pods" => Ok(watch_pods(out, parse_int(arg(args, 1)?)?)),
        "--selftest-icons" => {
            let cycles = if args.len() > 1 { parse_int(&args[1])? } else { 200 };
            Ok(selftest::icons(out, cycles))
        }
        "--version" => {
            wl!(out, "QuickTooth {}  (Rust, no_std)", env!("CARGO_PKG_VERSION"));
            Ok(0)
        }
        "--help" | "-h" | "/?" => Ok(help(out, 0)),
        _ => {
            wl!(out, "Unknown option: {}", args[0]);
            Ok(help(out, 2))
        }
    }
}

/// `args[i]`, or the error .NET gave for a missing one.
fn arg(args: &[String], i: usize) -> Result<&str, String> {
    args.get(i).map(String::as_str).ok_or_else(|| "System.IndexOutOfRangeException: Index was outside the bounds of the array.".into())
}

/// `int.Parse(s, InvariantCulture)`.
fn parse_int(s: &str) -> Result<i32, String> {
    let t = s.trim();
    t.parse::<i32>().map_err(|_| {
        let digits = t.strip_prefix(['+', '-']).unwrap_or(t);
        if !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) {
            "System.OverflowException: Value was either too large or too small for an Int32.".into()
        } else {
            format!("System.FormatException: The input string '{s}' was not in a correct format.")
        }
    })
}

const HELP: &str = "QuickTooth \u{2014} tray toggle for a Bluetooth audio device.\r
\r
  QuickTooth.exe                 start the tray icon (single instance)\r
  QuickTooth.exe --list [--all]  list Bluetooth audio devices (read-only)\r
  QuickTooth.exe --toggle        connect/disconnect the target device\r
  QuickTooth.exe --connect       connect the target device\r
  QuickTooth.exe --disconnect    disconnect the target device\r
                                 (add --no-wait to return immediately)\r
  QuickTooth.exe --autostart on|off|status\r
  QuickTooth.exe --exit          close the running tray instance\r
  QuickTooth.exe --settings      the settings, for the PocketKnives Settings window\r
  QuickTooth.exe --set key=value [key=value...]\r
\r
When the tray is running, --toggle/--connect/--disconnect are handed to it, so they\r
behave exactly like clicking the icon.";

fn help(out: &mut Writer, code: u32) -> u32 {
    wl!(out, "{HELP}");
    code
}

// ---------------------------------------------------------------------------- devices

fn enumerate(worker: &Worker) -> Vec<BtDevice> {
    worker.call(|a: &mut Audio| a.enumerate(false, false)).devices // what the tray's refresh enumerates
}

fn list(out: &mut Writer, include_others: bool) -> u32 {
    let worker = Worker::start();
    let snap = worker.call(move |a: &mut Audio| a.enumerate(true, include_others));
    if let Some(e) = &snap.error {
        wl!(out, "Enumeration error: {e}");
    }
    let settings = files::load_settings();
    let target = if settings.has_target() { settings.resolve(&snap.devices) } else { default_target(&snap.devices) };
    let how = match (settings.has_target(), target.is_some()) {
        (true, true) => "stored in settings",
        (true, false) => "STORED TARGET NOT FOUND \u{2014} nothing would be sent",
        (false, true) => "default: first-run pick, name contains \"AirPods\" (not saved yet)",
        (false, false) => "none",
    };
    wl!(out, "Settings: {}", files::settings_path());
    let name = target.map(|t| t.name.as_str()).or(settings.target_name.as_deref()).unwrap_or("-");
    wl!(out, "Target:   {name}  [{how}]");
    if settings.has_target() {
        wl!(
            out,
            "          stored container {}  MAC {}",
            settings.target_container_id.as_deref().unwrap_or("-"),
            settings.target_mac.as_deref().unwrap_or("-")
        );
    }
    wl!(out);
    wl!(out, "Bluetooth audio devices ({}), grouped by container ID:", snap.devices.len());
    for d in &snap.devices {
        let is_target = target.is_some_and(|t| t.container_id == d.container_id);
        wl!(out);
        wl!(
            out,
            "{} {}   {}{}",
            if is_target { "*" } else { " " },
            d.name,
            if d.connected() { "CONNECTED" } else { "not connected" },
            if is_target { "   <== target" } else { "" }
        );
        wl!(out, "    container {}   MAC {}", d.container_id, d.mac.as_deref().unwrap_or("-"));
        for e in &d.endpoints {
            wl!(out, "    - {}", e.name);
            wl!(out, "        state       {} ({})", device_state::name(e.state), e.state);
            let hidden = if e.hidden_from_enumeration { "   (not returned by EnumAudioEndpoints; found via PnP)" } else { "" };
            wl!(out, "        endpoint    {}{hidden}", e.endpoint_id);
            wl!(out, "        interface   {}", e.interface_name.as_deref().unwrap_or("-"));
            wl!(out, "        bth device  {}", e.bth_device_id);
            let ks =
                if e.ks_activate_hr == 0 { "activates (hr=0x00000000)".into() } else { format!("FAILED hr=0x{:08X}", e.ks_activate_hr) };
            let note = if is_target && e.ks_activate_hr == 0 { "   -> a send would hit this control" } else { "" };
            wl!(out, "        IKsControl  {ks}{note}");
        }
    }
    wl!(out);
    if include_others {
        wl!(out, "Other render endpoints ({}):", snap.others.len());
        for o in &snap.others {
            wl!(
                out,
                "  {:<10} {}\n             {}\n             topology: {}",
                device_state::name(o.state),
                o.name,
                o.endpoint_id,
                o.topology_ids.as_deref().unwrap_or("-")
            );
        }
    } else {
        wl!(out, "(Non-Bluetooth endpoints hidden; add --all to show them.)");
    }
    0
}

fn probe(out: &mut Writer, endpoint_id: &str) -> Outcome {
    let worker = Worker::start();
    let id = String::from(endpoint_id);
    let text = worker.call(move |a: &mut Audio| a.probe(&id))?;
    wl!(out, "{text}");
    Ok(0)
}

/// Read-only: Windows' battery property, then one passive BLE scan for the AirPods status advert.
fn battery_cmd(out: &mut Writer) -> u32 {
    let worker = Worker::start();
    let snap = worker.call(|a: &mut Audio| a.enumerate(false, false));
    let mut settings = files::load_settings();
    let Some(target) = settings.resolve_or_default(&snap.devices, false) else {
        wl!(out, "No target device.");
        return 3;
    };
    wl!(out, "Target: {} \u{2014} {}", target.name, if target.connected() { "connected" } else { "not connected" });
    let windows = target.mac.as_deref().and_then(pnp::bluetooth_battery);
    wl!(out, "Windows battery property: {}", windows.map_or_else(|| "none".into(), |w| format!("{w}%")));
    let Some(model) = battery::model_of(&target) else {
        wl!(out, "Not an Apple accessory; no BLE status advert to read.");
        return 0;
    };
    wl!(out, "Listening for Apple status adverts (model 0x{model:04X}) for up to 12 s\u{2026}");
    let scan = ble::scan_blocking(model, 12_000);
    match &scan.reading {
        None => wl!(out, "No status advert heard (the AirPods only broadcast it while out of the case)."),
        Some(r) => wl!(out, "Battery: {}   [{}]", r.text().unwrap_or_default(), r.describe()),
    }
    wl!(out, "Scan: {} Bluetooth LE adverts heard, {} from Apple devices, {} handler errors.", scan.adverts, scan.apple, scan.errors);
    0
}

/// Read-only: runs the tray's advert watch for `seconds` and prints each change of where the
/// target's buds are, as the tray reads it.
fn watch_pods(out: &mut Writer, seconds: i32) -> u32 {
    let worker = Worker::start();
    let snap = worker.call(|a: &mut Audio| a.enumerate(false, false));
    let mut settings = files::load_settings();
    let Some(model) = settings.resolve_or_default(&snap.devices, false).as_ref().and_then(battery::model_of) else {
        wl!(out, "No Apple target device.");
        return 3;
    };
    wl!(out, "Watching status adverts (model 0x{model:04X}) for {seconds} s\u{2026}");
    let clock = Stopwatch::start();
    type Heard = (u32, Vec<String>, Option<(battery::Pods, bool)>);
    let heard: Arc<pk::sync::Mutex<Heard>> = Arc::new(pk::sync::Mutex::new((0, Vec::new(), None)));
    let h = heard.clone();
    let watch = ble::watch(model, move |data, rssi| {
        let Some(now) = battery::parse_pods(&data, model) else { return };
        let mut h = h.lock();
        h.0 += 1;
        if h.2 != Some(now) {
            h.2 = Some(now);
            let hex: Vec<String> = data.iter().map(|b| format!("{b:02X}")).collect();
            let ear = if now.1 { ", in an ear" } else { "" };
            h.1.push(format!("{:>8} ms  {}{ear}  (rssi {rssi})  {}", clock.elapsed_whole_ms(), now.0.name(), hex.join(" ")));
        }
    });
    unsafe { Sleep(seconds.max(0) as u32 * 1000) };
    drop(watch);
    let h = heard.lock();
    for line in &h.1 {
        wl!(out, "{line}");
    }
    wl!(out, "{} status adverts of the model heard.", h.0);
    0
}

/// Enumerates (activating every Bluetooth KS control) `n` times and prints the process's handle
/// count, which must stay flat if COM objects are released.
fn stress_enumerate(out: &mut Writer, n: i32) -> u32 {
    let worker = Worker::start();
    let handles = || {
        let mut count = 0u32;
        unsafe { GetProcessHandleCount(GetCurrentProcess(), &mut count) };
        count
    };
    let mut i = 0;
    while i <= n {
        if i % 50 == 0 {
            wl!(out, "after {i:>4} enumerations: {} handles", handles());
        }
        if i < n {
            worker.call(|a: &mut Audio| drop(a.enumerate(true, false)));
        }
        i += 1;
    }
    0
}

/// Read-only: checks the endpoint-notification callback object through its COM vtable, then
/// registers it with the audio service and counts the callbacks that arrive in `seconds`.
fn watch_endpoints(out: &mut Writer, seconds: i32) -> Outcome {
    let worker = Worker::start();
    let ok = worker.call(|a: &mut Audio| a.self_test_callback());
    wl!(out, "callback self-test through the COM vtable: {}", if ok { "OK" } else { "FAILED" });
    worker.call(|a: &mut Audio| a.register_notifications(Arc::new(|_: &str| {})))?;
    wl!(out, "registered; listening {seconds} s\u{2026}");
    unsafe { Sleep(seconds.max(0) as u32 * 1000) };
    let count = worker.call(|a: &mut Audio| a.notification_count());
    wl!(out, "callbacks received from the audio service: {count}");
    Ok(0)
}

// ---------------------------------------------------------------------------- --settings / --set

/// In a PocketKnives preview, settings.json and the log are the ones next to this exe and the
/// running tray is never messaged. Called before anything loads or logs.
fn use_preview_folder() -> bool {
    let preview = pk::env::settings_preview();
    if preview {
        files::use_exe_dir();
    }
    preview
}

/// Read-only: one line per setting, tab-separated, for the Settings window.
fn settings_cmd(out: &mut Writer) -> u32 {
    use_preview_folder();
    let settings = files::load_settings();
    let worker = Worker::start();
    let _ = write!(out, "{}", blade_settings::describe(&settings, &enumerate(&worker)));
    0
}

/// Checks every pair, then hands the change to the running tray (which saves its whole file, so
/// it must be the one to apply it) or, with no tray, applies and saves it here.
/// Exit 0: saved and applied; 2: nothing changed, and one line says why.
fn set_cmd(out: &mut Writer, pairs: &[String]) -> u32 {
    let preview = use_preview_folder();
    let change = match blade_settings::parse(pairs) {
        Ok(c) => c,
        Err(e) => {
            wl!(out, "{e}");
            return 2;
        }
    };
    let hwnd = if preview { core::ptr::null_mut() } else { ipc::find_tray() };
    let result = if hwnd.is_null() { Some(set_here(&change)) } else { hand_to_tray(hwnd, pairs) };
    match result {
        Some(SetResult::Applied) => 0,
        // No answer in time, or an older tray that doesn't know the message: not known to be unchanged.
        None => {
            wl!(out, "The running QuickTooth didn't answer. Restart it and try again.");
            1
        }
        Some(r) => {
            wl!(out, "{}", blade_settings::message(r));
            2
        }
    }
}

fn set_here(change: &SettingsChange) -> SetResult {
    let mut settings: Settings = files::load_settings();
    let devices = if change.device.is_some() { enumerate(&Worker::start()) } else { Vec::new() };
    let (result, device) = blade_settings::check(change, &settings, &devices);
    if result == SetResult::Applied { blade_settings::apply(change, &mut settings, device.as_ref()) } else { result }
}

/// The tray's answer, or `None` if it didn't handle the message in time.
fn hand_to_tray(hwnd: windows_sys::Win32::Foundation::HWND, pairs: &[String]) -> Option<SetResult> {
    let text: Vec<u16> = pairs.join("\n").encode_utf16().collect();
    let cds = COPYDATASTRUCT { dwData: ipc::SET_SETTINGS_DATA, cbData: (text.len() * 2) as u32, lpData: text.as_ptr() as *mut _ };
    let mut answer = 0usize;
    let sent = unsafe {
        SendMessageTimeoutW(
            hwnd,
            WM_COPYDATA,
            0,
            &cds as *const COPYDATASTRUCT as isize,
            SMTO_ABORTIFHUNG,
            ipc::SET_SETTINGS_TIMEOUT_MS,
            &mut answer,
        )
    };
    match (sent, answer) {
        (0, _) | (_, 0) => None,
        // A code this build doesn't know: refused (C# printed the generic refusal).
        _ => Some(SetResult::from_i32(answer as i32).unwrap_or(SetResult::Invalid)),
    }
}

// ---------------------------------------------------------------------------- --toggle / --connect / --disconnect

/// `direct`: send from this process even if the tray is running (diagnostics).
/// `a2dp_only`: send only to the A2DP control (diagnostics / A-B test).
fn action(out: &mut Writer, requested: Option<OneShot>, wait: bool, direct: bool, a2dp_only: bool) -> Outcome {
    let worker = Worker::start();
    let mut settings = files::load_settings();
    let snap = worker.call(|a: &mut Audio| a.enumerate(false, false));
    let Some(target) = settings.resolve_or_default(&snap.devices, true) else {
        if settings.has_target() {
            wl!(
                out,
                "Target '{}' ({}) is not available. Nothing sent.",
                settings.target_name.as_deref().unwrap_or(""),
                settings.target_container_id.as_deref().unwrap_or("")
            );
        } else {
            wl!(out, "No target device (no Bluetooth audio device named *AirPods*). Pick one from the tray menu. Nothing sent.");
        }
        return Ok(3);
    };

    let op = requested.unwrap_or(if target.connected() { OneShot::Disconnect } else { OneShot::Connect });
    let expect_connected = op == OneShot::Connect;
    wl!(
        out,
        "Target: {}  container {}  \u{2014} currently {}",
        target.name,
        target.container_id,
        if target.connected() { "connected" } else { "not connected" }
    );
    if target.connected() == expect_connected {
        wl!(out, "Already {}; nothing to do.", if expect_connected { "connected" } else { "disconnected" });
        return Ok(0);
    }

    let hwnd = if direct { core::ptr::null_mut() } else { ipc::find_tray() };
    let mut sent = Stopwatch::start();
    if !hwnd.is_null() {
        let code = match requested {
            None => tray_commands::TOGGLE,
            Some(OneShot::Connect) => tray_commands::CONNECT,
            Some(OneShot::Disconnect) => tray_commands::DISCONNECT,
        };
        let ok = unsafe { PostMessageW(hwnd, ipc::command_message(), code as usize, 0) } != 0;
        if ok {
            let what = requested.map_or_else(|| "toggle".into(), |o| format!("{o}"));
            wl!(out, "Handed '{what}' to the running tray instance (same path as a click).");
        } else {
            wl!(out, "PostMessage to tray failed (error {}).", unsafe { GetLastError() });
            return Ok(1);
        }
    } else {
        if let (OneShot::Connect, Some(mac)) = (op, target.mac.as_deref()) {
            // Same as the tray: let a lingering link from a recent disconnect close first.
            let hold = Stopwatch::start();
            while hold.elapsed_whole_ms() < 5000 && pnp::bluetooth_connected(mac) == Some(true) {
                unsafe { Sleep(200) };
            }
            if hold.elapsed_whole_ms() > 300 {
                wl!(out, "Waited {} ms for the previous link to close.", hold.elapsed_whole_ms());
            }
        }
        let t = files::local_time();
        wl!(
            out,
            "{} sending {op}{} at {:02}:{:02}:{:02}.{:03}.",
            if direct { "Direct" } else { "Tray not running;" },
            if a2dp_only { " (A2DP control only)" } else { "" },
            t.wHour,
            t.wMinute,
            t.wSecond,
            t.wMilliseconds
        );
        sent.restart();
        let container = target.container_id;
        let results = worker.call(move |a: &mut Audio| a.send_one_shot(container, op, a2dp_only))?;
        for r in &results {
            wl!(out, "  KsProperty hr=0x{:08X}  {}", r.hr, r.endpoint_name);
        }
        if results.iter().all(|r| r.hr != 0) {
            wl!(out, "No control accepted the request.");
            return Ok(1);
        }
    }

    if !wait {
        return Ok(0);
    }
    Ok(if wait_for_state(out, &worker, &target, expect_connected, &sent) { 0 } else { 4 })
}

/// Read-only polling: prints every endpoint state change until the device reaches the
/// expected state, then watches 4 s more.
fn wait_for_state(out: &mut Writer, worker: &Worker, target: &BtDevice, expect_connected: bool, sw: &Stopwatch) -> bool {
    let mut last: Vec<(String, u32)> = Vec::new();
    let mut reached_at: Option<f64> = None;
    let mut last_link: Option<Option<bool>> = None;
    while sw.elapsed_ms() < 40_000.0 {
        if let Some(mac) = target.mac.as_deref() {
            let link = pnp::bluetooth_connected(mac);
            if last_link != Some(link) {
                let text = match link {
                    None => "?",
                    Some(true) => "connected",
                    Some(false) => "not connected",
                };
                wl!(out, "  +{:>6} ms  Bluetooth link flag: {text}", sw.elapsed_whole_ms());
                last_link = Some(link);
            }
        }
        let snap = worker.call(|a: &mut Audio| a.enumerate(false, false));
        if let Some(dev) = snap.devices.iter().find(|d| d.container_id == target.container_id) {
            for e in &dev.endpoints {
                let pos = last.iter().position(|(id, _)| *id == e.endpoint_id);
                if pos.is_none_or(|p| last[p].1 != e.state) {
                    wl!(out, "  +{:>6} ms  {:<10} {}", sw.elapsed_whole_ms(), device_state::name(e.state), e.name);
                    match pos {
                        Some(p) => last[p].1 = e.state,
                        None => last.push((e.endpoint_id.clone(), e.state)),
                    }
                }
            }
            if dev.connected() == expect_connected && reached_at.is_none() {
                reached_at = Some(sw.elapsed_ms());
                wl!(
                    out,
                    "  Reached {} after {} ms; watching 4 s more for changes\u{2026}",
                    if expect_connected { "connected" } else { "disconnected" },
                    sw.elapsed_whole_ms()
                );
            }
            if reached_at.is_some() && dev.connected() != expect_connected {
                wl!(out, "  !! state flipped back after reaching it (+{} ms)", sw.elapsed_whole_ms());
            }
            if let Some(at) = reached_at {
                if sw.elapsed_ms() - at > 4000.0 {
                    return dev.connected() == expect_connected;
                }
            }
        }
        unsafe { Sleep(100) };
    }
    wl!(out, "{}", if reached_at.is_none() { "  Timed out after 40 s." } else { "  Done." });
    reached_at.is_some()
}

// ---------------------------------------------------------------------------- the running tray, autostart, icons

fn forward(out: &mut Writer, cmd: i32) -> u32 {
    let hwnd = ipc::find_tray();
    if hwnd.is_null() {
        wl!(out, "Tray instance is not running.");
        return 1;
    }
    unsafe { PostMessageW(hwnd, ipc::command_message(), cmd as usize, 0) };
    0
}

fn exit_tray(out: &mut Writer) -> u32 {
    let hwnd = ipc::find_tray();
    if hwnd.is_null() {
        wl!(out, "Tray instance is not running.");
        return 0;
    }
    unsafe { PostMessageW(hwnd, ipc::command_message(), tray_commands::EXIT as usize, 0) };
    wl!(out, "Asked the tray instance to exit.");
    0
}

fn autostart(out: &mut Writer, arg: &str) -> u32 {
    match arg.to_ascii_lowercase().as_str() {
        "on" => system::set_autostart(true),
        "off" => system::set_autostart(false),
        _ => {}
    }
    let value = system::autostart_value();
    wl!(
        out,
        "Start with Windows: {}  ({})",
        if system::autostart_enabled() { "on" } else { "off" },
        value.as_deref().unwrap_or("no Run entry")
    );
    0
}

fn icon_rect(out: &mut Writer) -> u32 {
    let hwnd = ipc::find_tray();
    if hwnd.is_null() {
        wl!(out, "Tray instance is not running.");
        return 1;
    }
    let id = NOTIFYICONIDENTIFIER {
        cbSize: size_of::<NOTIFYICONIDENTIFIER>() as u32,
        hWnd: hwnd,
        uID: ipc::ICON_ID,
        guidItem: unsafe { core::mem::zeroed() },
    };
    let mut r = unsafe { core::mem::zeroed() };
    let hr = unsafe { Shell_NotifyIconGetRect(&id, &mut r) };
    if hr == 0 {
        wl!(out, "{} {} {} {}", r.left, r.top, r.right, r.bottom);
        0
    } else {
        wl!(out, "Shell_NotifyIconGetRect hr=0x{hr:08X}");
        1
    }
}

/// For the suite's build: each Tray icon choice's picture, `<dir>\icon.<choice>.ico`, as
/// PocketKnives Settings loads them (docs/specs/blade-settings.md at the repo root).
fn write_setting_pictures(out: &mut Writer, dir: &str) -> Outcome {
    let files = icons::write_setting_pictures(dir)?;
    for file in files {
        wl!(out, "{file}");
    }
    Ok(0)
}

fn render_icons(out: &mut Writer, args: &[String]) -> Outcome {
    if let Some(i) = args.iter().position(|a| a.eq_ignore_ascii_case("--zoom")) {
        if let Some(z) = args.get(i + 1) {
            let Some(zoom) = quicktooth_core::num::parse_f64(z.trim()) else {
                return Err(format!("System.FormatException: The input string '{z}' was not in a correct format."));
            };
            quicktooth_core::tunables::set_value(quicktooth_core::tunables::T::Zoom, zoom);
        }
    }
    let dir = match args.get(1) {
        Some(d) if !d.starts_with("--") => d.clone(),
        _ => format!("{}\\icon-check", pk::env::current_dir()),
    };
    for file in icons::write_check_images(&dir) {
        wl!(out, "{file}");
    }
    Ok(0)
}
