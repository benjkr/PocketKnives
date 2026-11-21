//! Passive BLE for the target's Apple status advert (docs/specs/bluetooth.md): short battery scans,
//! and the advert watch. Never connects to anything. A scan stops at the first strong match (the
//! AirPods are in the ears, so they're close), or after its window with the strongest match seen.
use crate::worker::Handle;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::ptr::null;
use core::sync::atomic::{AtomicU32, Ordering};
use pk::sync::Mutex;
use quicktooth_core::battery::{BatteryReading, parse_apple_status};
use windows::Devices::Bluetooth::Advertisement::{
    BluetoothLEAdvertisementReceivedEventArgs, BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementWatcherStoppedEventArgs,
    BluetoothLEScanningMode,
};
use windows::Devices::Bluetooth::BluetoothError;
use windows::Foundation::TypedEventHandler;
use windows::Storage::Streams::DataReader;
use windows_sys::Win32::System::Com::{COINIT_MULTITHREADED, CoInitializeEx};

const STRONG_RSSI: i32 = -70;
const MIN_RSSI: i32 = -85;
const APPLE: u16 = 0x004C;

/// What a scan heard: the best reading and the counters that prove the event path works.
pub struct Scan {
    pub reading: Option<BatteryReading>,
    pub adverts: u32,
    pub apple: u32,
    pub errors: u32,
}

struct State {
    model: u16,
    adverts: AtomicU32,
    apple: AtomicU32,
    errors: AtomicU32,
    best: Mutex<Option<BatteryReading>>,
    /// Set by a strong match or the watcher stopping on its own.
    finish: Handle,
}

/// Scans on a thread of its own and hands the result to `done` there.
pub fn scan_async(model: u16, window_ms: u32, done: impl FnOnce(Scan) + Send + 'static) {
    // `done` goes to the thread; if it can't start, it comes back and gets an empty scan.
    let slot = Arc::new(Mutex::new(Some(done)));
    let s = slot.clone();
    let started = pk::thread::spawn(move || {
        unsafe { CoInitializeEx(null(), COINIT_MULTITHREADED as u32) };
        let scan = scan(model, window_ms);
        let done = s.lock().take();
        if let Some(done) = done {
            done(scan);
        }
    });
    if started.is_none() {
        quicktooth_core::log::error("battery scan", "the thread could not be started");
        let done = slot.lock().take();
        if let Some(done) = done {
            done(Scan { reading: None, adverts: 0, apple: 0, errors: 0 });
        }
    }
}

/// Scans and waits for the result (the command line).
pub fn scan_blocking(model: u16, window_ms: u32) -> Scan {
    let slot = Arc::new((Mutex::new(None), Handle::event()));
    let s = slot.clone();
    scan_async(model, window_ms, move |r| {
        *s.0.lock() = Some(r);
        s.1.set();
    });
    slot.1.wait(windows_sys::Win32::System::Threading::INFINITE);
    slot.0.lock().take().expect("scan result")
}

fn scan(model: u16, window_ms: u32) -> Scan {
    let state = Arc::new(State {
        model,
        adverts: AtomicU32::new(0),
        apple: AtomicU32::new(0),
        errors: AtomicU32::new(0),
        best: Mutex::new(None),
        finish: Handle::event(),
    });
    let unavailable = |e: windows_core::Error| {
        quicktooth_core::log!("battery scan unavailable: {}", e.message());
        Scan { reading: None, adverts: 0, apple: 0, errors: 0 }
    };
    let watcher = match BluetoothLEAdvertisementWatcher::new() {
        Ok(w) => w,
        Err(e) => return unavailable(e),
    };
    let _ = watcher.SetScanningMode(BluetoothLEScanningMode::Passive);
    let logged = Arc::new(AtomicU32::new(0));
    let (s, l) = (state.clone(), logged.clone());
    let _ = watcher.Received(&TypedEventHandler::new(move |_, args: windows_core::Ref<BluetoothLEAdvertisementReceivedEventArgs>| {
        if let Some(a) = args.as_ref() {
            if let Err(e) = on_advert(&s, a) {
                s.errors.fetch_add(1, Ordering::Relaxed);
                if l.fetch_add(1, Ordering::Relaxed) == 0 {
                    quicktooth_core::log::error("battery advert handler", &e.message());
                }
            }
        }
        Ok(())
    }));
    let s = state.clone();
    let _ = watcher.Stopped(&TypedEventHandler::new(move |_, args: windows_core::Ref<BluetoothLEAdvertisementWatcherStoppedEventArgs>| {
        let error = args.as_ref().and_then(|a| a.Error().ok()).unwrap_or(BluetoothError::Success);
        if error != BluetoothError::Success {
            quicktooth_core::log!("battery scan stopped: {}", error_name(error));
        }
        s.finish.set();
        Ok(())
    }));
    if let Err(e) = watcher.Start() {
        return unavailable(e);
    }
    state.finish.wait(window_ms);
    let _ = watcher.Stop();
    let reading = state.best.lock().clone();
    let (adverts, apple, errors) =
        (state.adverts.load(Ordering::Relaxed), state.apple.load(Ordering::Relaxed), state.errors.load(Ordering::Relaxed));
    let result = match &reading {
        Some(r) => alloc::format!("reading {}% (rssi {})", r.buds().map(|b| alloc::format!("{b}")).unwrap_or_default(), r.rssi),
        None => String::from("no reading"),
    };
    quicktooth_core::log!("battery scan: {adverts} adverts, {apple} Apple, {errors} handler errors, {result}");
    Scan { reading, adverts, apple, errors }
}

fn on_advert(s: &State, a: &BluetoothLEAdvertisementReceivedEventArgs) -> windows_core::Result<()> {
    s.adverts.fetch_add(1, Ordering::Relaxed);
    let rssi = a.RawSignalStrengthInDBm()? as i32;
    if rssi < MIN_RSSI {
        return Ok(());
    }
    for md in a.Advertisement()?.ManufacturerData()? {
        if md.CompanyId()? != APPLE {
            continue;
        }
        s.apple.fetch_add(1, Ordering::Relaxed);
        let data = md.Data()?;
        let len = data.Length()? as usize;
        if len < 9 {
            continue;
        }
        let mut bytes = alloc::vec![0u8; len];
        DataReader::FromBuffer(&data)?.ReadBytes(&mut bytes)?;
        let Some(reading) = parse_apple_status(&bytes, s.model, rssi, crate::clock::epoch_ms()) else { continue };
        let strong = {
            let mut best = s.best.lock();
            let strong = reading.rssi >= STRONG_RSSI;
            if best.as_ref().is_none_or(|b| reading.rssi > b.rssi) {
                *best = Some(reading);
            }
            strong
        };
        if strong {
            s.finish.set();
        }
    }
    Ok(())
}

/// The enum member's name, as .NET printed it.
fn error_name(e: BluetoothError) -> String {
    let name = match e.0 {
        0 => "Success",
        1 => "RadioNotAvailable",
        2 => "ResourceInUse",
        3 => "DeviceNotConnected",
        4 => "OtherError",
        5 => "DisabledByPolicy",
        6 => "NotSupported",
        7 => "DisabledByUser",
        8 => "ConsentRequired",
        9 => "TransportNotSupported",
        n => return alloc::format!("{n}"),
    };
    name.into()
}

/// The advert watch: a passive watcher that hands each status advert of one model to `deliver`
/// (on a thread-pool thread). It stops when dropped.
pub struct Watch(BluetoothLEAdvertisementWatcher);

impl Drop for Watch {
    fn drop(&mut self) {
        let _ = self.0.Stop();
    }
}

pub fn watch(model: u16, deliver: impl Fn(Vec<u8>, i32) + Send + Sync + 'static) -> Option<Watch> {
    let start = || -> windows_core::Result<Watch> {
        let watcher = BluetoothLEAdvertisementWatcher::new()?;
        watcher.SetScanningMode(BluetoothLEScanningMode::Passive)?;
        watcher.Received(&TypedEventHandler::new(move |_, args: windows_core::Ref<BluetoothLEAdvertisementReceivedEventArgs>| {
            if let Some(a) = args.as_ref() {
                // A failed read is a dropped advert; the next one comes within a second or two.
                let _ = status_adverts(a, model, &deliver);
            }
            Ok(())
        }))?;
        watcher.Start()?;
        Ok(Watch(watcher))
    };
    match start() {
        Ok(w) => Some(w),
        Err(e) => {
            quicktooth_core::log!("advert watch unavailable: {}", e.message());
            None
        }
    }
}

/// Passes on the advert's Apple status messages of `model` (the core reads them).
fn status_adverts(a: &BluetoothLEAdvertisementReceivedEventArgs, model: u16, deliver: &impl Fn(Vec<u8>, i32)) -> windows_core::Result<()> {
    let rssi = a.RawSignalStrengthInDBm()? as i32;
    if rssi < MIN_RSSI {
        return Ok(());
    }
    for md in a.Advertisement()?.ManufacturerData()? {
        if md.CompanyId()? != APPLE {
            continue;
        }
        let data = md.Data()?;
        let len = data.Length()? as usize;
        if len < 9 {
            continue;
        }
        let mut bytes = alloc::vec![0u8; len];
        DataReader::FromBuffer(&data)?.ReadBytes(&mut bytes)?;
        if bytes[0] == 0x07 && u16::from_le_bytes([bytes[3], bytes[4]]) == model {
            deliver(bytes, rssi);
        }
    }
    Ok(())
}
