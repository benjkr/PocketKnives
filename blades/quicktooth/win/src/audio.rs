//! Bluetooth audio through Core Audio, its device topology and the Bluetooth audio driver's
//! one-shot KS properties (docs/specs/bluetooth.md). Every method runs on the device worker's
//! MTA thread ([`crate::worker`]), which owns this object and every COM interface it holds.
//! COM interfaces are the `windows` crate's: each is released when it's dropped.
use crate::pnp;
use alloc::format;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::cmp::Ordering as Order;
use core::sync::atomic::{AtomicU32, Ordering};
use pk::sync::Mutex;
use pk::wide::to_wide;
use quicktooth_core::Guid;
use quicktooth_core::devices::{BtDevice, BtEndpoint, KS_NOT_ATTEMPTED, OneShot, OtherEndpoint, SendResult, Snapshot, device_state};
use pk_windows::Win32::Devices::FunctionDiscovery::{PKEY_Device_ContainerId, PKEY_Device_FriendlyName, PKEY_DeviceInterface_FriendlyName};
use pk_windows::Win32::Foundation::PROPERTYKEY;
use pk_windows::Win32::Media::Audio::{
    DEVICE_STATE, DEVICE_STATE_ACTIVE, DEVICE_STATEMASK_ALL, EDataFlow, ERole, IDeviceTopology, IMMDevice, IMMDeviceEnumerator,
    IMMNotificationClient, IMMNotificationClient_Impl, IPart, MMDeviceEnumerator, eRender,
};
use pk_windows::Win32::Media::KernelStreaming::{IKsControl, KSIDENTIFIER, KSIDENTIFIER_0, KSIDENTIFIER_0_0};
use pk_windows::Win32::System::Com::StructuredStorage::{PROPVARIANT, PropVariantClear};
use pk_windows::Win32::System::Com::{CLSCTX_ALL, CoCreateInstance, CoTaskMemFree, STGM_READ};
use pk_windows::Win32::UI::Shell::PropertiesSystem::IPropertyStore;
use windows_core::{GUID, Interface, PCWSTR, PWSTR, implement};

/// KSPROPSETID_BtAudio (ksmedia.h) and its two one-shot properties.
const KSPROPSETID_BTAUDIO: GUID = GUID::from_u128(0x7FA06C40_B8F6_4C7E_8556_E8C33A12E54D);
const KSPROPERTY_ONESHOT_RECONNECT: u32 = 0;
const KSPROPERTY_ONESHOT_DISCONNECT: u32 = 1;
const KSPROPERTY_TYPE_GET: u32 = 1;

const VT_LPWSTR: u16 = 31;
const VT_CLSID: u16 = 72;

/// Called on an audio-service thread for every state change, addition or removal of an endpoint.
pub type OnEndpointChanged = Arc<dyn Fn(&str) + Send + Sync>;

pub struct Audio {
    enumerator: Option<IMMDeviceEnumerator>,
    /// Registered with the audio service, which holds only a raw pointer: kept alive here.
    client: Option<IMMNotificationClient>,
    /// Callbacks received from the audio service so far (all kinds; diagnostics).
    notifications: Arc<AtomicU32>,
}

impl Audio {
    pub fn new() -> Audio {
        Audio { enumerator: None, client: None, notifications: Arc::new(AtomicU32::new(0)) }
    }

    fn enumerator(&mut self) -> Result<IMMDeviceEnumerator, String> {
        if self.enumerator.is_none() {
            let e: IMMDeviceEnumerator = unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) }.map_err(com_message)?;
            self.enumerator = Some(e);
        }
        Ok(self.enumerator.clone().unwrap())
    }

    pub fn notification_count(&self) -> u32 {
        self.notifications.load(Ordering::Relaxed)
    }

    pub fn register_notifications(&mut self, on_change: OnEndpointChanged) -> Result<(), String> {
        if self.client.is_some() {
            return Ok(());
        }
        let e = self.enumerator()?;
        let client: IMMNotificationClient = Client { notifications: self.notifications.clone(), on_change }.into();
        let hr = unsafe { e.RegisterEndpointNotificationCallback(&client) }.err().map_or(0, |e| e.code().0);
        quicktooth_core::log!("RegisterEndpointNotificationCallback hr=0x{hr:08X}");
        if hr == 0 {
            self.client = Some(client);
        }
        Ok(())
    }

    /// Test hook: calls a callback object of ours the way the audio service does, through its COM
    /// vtable (`OnDeviceStateChanged`), and reports whether our handler saw the same endpoint ID.
    pub fn self_test_callback(&mut self) -> bool {
        const ID: &str = "{0.0.0.00000000}.{self-test}";
        let seen: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
        let s = seen.clone();
        let client: IMMNotificationClient =
            Client { notifications: self.notifications.clone(), on_change: Arc::new(move |id: &str| *s.lock() = Some(id.into())) }.into();
        let w = to_wide(ID);
        let ok = unsafe { client.OnDeviceStateChanged(PCWSTR(w.as_ptr()), DEVICE_STATE_ACTIVE) }.is_ok();
        ok && seen.lock().as_deref() == Some(ID)
    }

    /// Every render endpoint, with the Bluetooth ones grouped into devices by container ID.
    pub fn enumerate(&mut self, probe_ks: bool, include_others: bool) -> Snapshot {
        let mut endpoints: Vec<BtEndpoint> = Vec::new();
        let mut others = Vec::new();
        let mut error = None;
        match self.enumerator().and_then(|e| render_endpoints(&e).map(|list| (e, list))) {
            Ok((e, list)) => {
                for (device, hidden) in &list {
                    if let Some(ep) = read_endpoint(&e, device, *hidden, probe_ks, include_others.then_some(&mut others)) {
                        endpoints.push(ep);
                    }
                }
            }
            Err(msg) => {
                quicktooth_core::log::error("enumerate", &msg);
                error = Some(msg);
            }
        }
        Snapshot { devices: group(endpoints), others, error }
    }

    /// Sends the one-shot connect or disconnect to EVERY Bluetooth KS control (A2DP, then HFP)
    /// of the device with exactly this container ID, as read from each endpoint's own property
    /// store. This is the only code in the app that sends anything to a device.
    pub fn send_one_shot(&mut self, container: Guid, op: OneShot, a2dp_only: bool) -> Result<Vec<SendResult>, String> {
        if container.is_empty() {
            return Err("empty container id".into());
        }
        let e = self.enumerator()?;
        let mut targets: Vec<(String, String)> = Vec::new();
        for (device, _) in render_endpoints(&e)? {
            let store = open_store(&device);
            if store.as_ref().and_then(read_guid_container) != Some(container) {
                continue; // never touch any other device
            }
            let name = store.as_ref().and_then(|s| read_string(s, &PKEY_Device_FriendlyName)).unwrap_or_else(|| "?".into());
            for bth in topology_ids(&device).0 {
                if a2dp_only && !is_a2dp(&bth) {
                    continue; // the A/B experiment switch
                }
                if !targets.iter().any(|(_, t)| t.eq_ignore_ascii_case(&bth)) {
                    targets.push((name.clone(), bth));
                }
            }
        }
        // A2DP first, then HFP (stable).
        sort_stable(&mut targets, |a, b| is_a2dp(&b.1).cmp(&is_a2dp(&a.1)));
        let mut results = Vec::new();
        for (name, bth) in targets {
            let (hr, bytes) = match activate_ks(&e, &bth) {
                Ok(ks) => ks_one_shot(&ks, op),
                Err(hr) => (hr, 0),
            };
            results.push(SendResult { endpoint_name: name, bth_device_id: bth, hr, bytes_returned: bytes });
        }
        quicktooth_core::devices::send_log(container, op, &results);
        Ok(results)
    }

    /// Read-only diagnostics for one endpoint ID (`--probe`), CRLF lines.
    pub fn probe(&mut self, endpoint_id: &str) -> Result<String, String> {
        let e = self.enumerator()?;
        let w = to_wide(endpoint_id);
        let device = match unsafe { e.GetDevice(PCWSTR(w.as_ptr())) } {
            Ok(d) => d,
            Err(err) => return Ok(format!("GetDevice hr=0x{:08X}", err.code().0)),
        };
        let state = unsafe { device.GetState() }.map(|s| s.0).unwrap_or(0);
        let (ids, all) = topology_ids(&device);
        let store = open_store(&device);
        let name = store.as_ref().and_then(|s| read_string(s, &PKEY_Device_FriendlyName)).unwrap_or_else(|| "?".into());
        let container = store.as_ref().and_then(read_guid_container).unwrap_or(Guid::EMPTY);
        let mut s = String::new();
        s.push_str(&format!("name       {name}\r\n"));
        s.push_str(&format!("state      {} ({state})\r\n", device_state::name(state)));
        s.push_str(&format!("container  {container}\r\n"));
        s.push_str(&format!("topology   {}\r\n", if all.is_empty() { "-".into() } else { all.join(" | ") }));
        for id in ids {
            let hr = activate_ks(&e, &id).err().unwrap_or(0);
            s.push_str(&format!("IKsControl hr=0x{hr:08X}  {id}\r\n"));
        }
        Ok(s)
    }
}

/// All render endpoints: Core Audio's `EnumAudioEndpoints(eRender, all states)`, then the PnP
/// render endpoints it didn't return, present or not (opened with `GetDevice`; flagged hidden).
fn render_endpoints(e: &IMMDeviceEnumerator) -> Result<Vec<(IMMDevice, bool)>, String> {
    let collection = unsafe { e.EnumAudioEndpoints(eRender, DEVICE_STATE(DEVICE_STATEMASK_ALL)) }
        .map_err(|_| String::from("EnumAudioEndpoints failed"))?;
    let mut seen: Vec<String> = Vec::new();
    let mut list = Vec::new();
    for i in 0..unsafe { collection.GetCount() }.unwrap_or(0) {
        let Ok(device) = (unsafe { collection.Item(i) }) else { continue };
        if let Ok(id) = unsafe { device.GetId() } {
            seen.push(take_pwstr(id).to_ascii_lowercase());
        }
        list.push((device, false));
    }
    let pnp_ids = match pnp::render_endpoint_ids() {
        Ok(ids) => ids,
        Err(msg) => {
            quicktooth_core::log!("PnP endpoint lookup failed, using Core Audio enumeration only: {msg}");
            return Ok(list);
        }
    };
    for id in pnp_ids {
        if seen.iter().any(|s| s.eq_ignore_ascii_case(&id)) {
            continue;
        }
        let w = to_wide(&id);
        let Ok(device) = (unsafe { e.GetDevice(PCWSTR(w.as_ptr())) }) else { continue };
        seen.push(id);
        list.push((device, true));
    }
    Ok(list)
}

fn read_endpoint(
    e: &IMMDeviceEnumerator,
    device: &IMMDevice,
    hidden: bool,
    probe_ks: bool,
    others: Option<&mut Vec<OtherEndpoint>>,
) -> Option<BtEndpoint> {
    let id = take_pwstr(unsafe { device.GetId() }.ok()?);
    let state = unsafe { device.GetState() }.map(|s| s.0).unwrap_or(0);
    let store = open_store(device);
    let name = store.as_ref().and_then(|s| read_string(s, &PKEY_Device_FriendlyName)).unwrap_or_else(|| id.clone());
    let (bth_ids, all_ids) = topology_ids(device);
    let Some(bth) = bth_ids.into_iter().next() else {
        if let Some(others) = others {
            let topology_ids = (!all_ids.is_empty()).then(|| all_ids.join(" | "));
            others.push(OtherEndpoint { endpoint_id: id, name, state, topology_ids });
        }
        return None;
    };
    let ks_activate_hr = if probe_ks { activate_ks(e, &bth).err().unwrap_or(0) } else { KS_NOT_ATTEMPTED };
    Some(BtEndpoint {
        interface_name: store.as_ref().and_then(|s| read_string(s, &PKEY_DeviceInterface_FriendlyName)),
        container_id: store.as_ref().and_then(read_guid_container).unwrap_or(Guid::EMPTY),
        mac: mac_of(&bth),
        endpoint_id: id,
        name,
        state,
        bth_device_id: bth,
        hidden_from_enumeration: hidden,
        ks_activate_hr,
    })
}

/// Groups the endpoints by container ID (first-seen order), A2DP first then by name within a
/// device, and sorts the devices by name, ignoring case.
fn group(endpoints: Vec<BtEndpoint>) -> Vec<BtDevice> {
    let mut groups: Vec<(Guid, Vec<BtEndpoint>)> = Vec::new();
    for ep in endpoints.into_iter().filter(|e| !e.container_id.is_empty()) {
        match groups.iter_mut().find(|(g, _)| *g == ep.container_id) {
            Some((_, eps)) => eps.push(ep),
            None => groups.push((ep.container_id, alloc::vec![ep])),
        }
    }
    let mut devices: Vec<BtDevice> = groups
        .into_iter()
        .map(|(container_id, mut eps)| {
            sort_stable(&mut eps, |a, b| b.is_a2dp().cmp(&a.is_a2dp()).then_with(|| ordinal(&a.name, &b.name)));
            let mac = eps.iter().find_map(|e| e.mac.clone());
            BtDevice { container_id, name: display_name(&eps), mac, endpoints: eps }
        })
        .collect();
    sort_stable(&mut devices, |a, b| ordinal_ignore_case(&a.name, &b.name));
    devices
}

/// The Bluetooth device's own name: the first interface name, else the endpoint name's part
/// in parentheses, else the endpoint name.
fn display_name(eps: &[BtEndpoint]) -> String {
    if let Some(n) = eps.iter().filter_map(|e| e.interface_name.as_deref()).find(|n| !n.trim().is_empty()) {
        return n.into();
    }
    let n = &eps[0].name;
    match n.find('(') {
        Some(open) if n.ends_with(')') => n[open + 1..n.len() - 1].into(),
        _ => n.clone(),
    }
}

/// A stable insertion sort (LINQ's `OrderBy` is stable). The lists are a few dozen items;
/// `slice::sort_by` would bring a 4 KB stack buffer, which needs the CRT's `__chkstk`.
fn sort_stable<T>(v: &mut [T], cmp: impl Fn(&T, &T) -> Order) {
    for i in 1..v.len() {
        let mut j = i;
        while j > 0 && cmp(&v[j - 1], &v[j]) == Order::Greater {
            v.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// .NET's ordinal comparison (UTF-16 code units).
fn ordinal(a: &str, b: &str) -> Order {
    a.encode_utf16().cmp(b.encode_utf16())
}

/// .NET's culture comparison ignoring case under invariant globalization: ordinal on each
/// UTF-16 unit's simple upper-case mapping.
fn ordinal_ignore_case(a: &str, b: &str) -> Order {
    fn upper(u: u16) -> u16 {
        match char::from_u32(u as u32) {
            Some(c) => {
                let mut up = c.to_uppercase();
                match (up.next(), up.next()) {
                    (Some(x), None) if (x as u32) <= 0xFFFF => x as u16,
                    _ => u,
                }
            }
            None => u,
        }
    }
    a.encode_utf16().map(upper).cmp(b.encode_utf16().map(upper))
}

/// Endpoint → IDeviceTopology → each connector → the connector it's connected to → IPart →
/// its topology object → device ID. Returns (the `{2}.\\?\bth…` IDs, every ID).
fn topology_ids(device: &IMMDevice) -> (Vec<String>, Vec<String>) {
    let (mut bth, mut all) = (Vec::new(), Vec::new());
    let Ok(topo) = (unsafe { device.Activate::<IDeviceTopology>(CLSCTX_ALL, None) }) else { return (bth, all) };
    let n = unsafe { topo.GetConnectorCount() }.unwrap_or(0);
    for c in 0..n {
        let Ok(connector) = (unsafe { topo.GetConnector(c) }) else { continue };
        let Ok(other) = (unsafe { connector.GetConnectedTo() }) else { continue }; // normal for unconnected connectors
        let Ok(part) = other.cast::<IPart>() else { continue };
        let Ok(other_topo) = (unsafe { part.GetTopologyObject() }) else { continue };
        let Ok(id) = (unsafe { other_topo.GetDeviceId() }) else { continue };
        let id = take_pwstr(id);
        if id.len() >= 11 && id.is_char_boundary(11) && id[..11].eq_ignore_ascii_case("{2}.\\\\?\\bth") {
            bth.push(id.clone());
        }
        all.push(id);
    }
    (bth, all)
}

fn is_a2dp(bth_id: &str) -> bool {
    quicktooth_core::devices::contains_ignore_case(bth_id, "\\bthenum#")
}

/// The KS control of a Bluetooth KS filter, or the failing HRESULT.
fn activate_ks(e: &IMMDeviceEnumerator, bth_id: &str) -> Result<IKsControl, i32> {
    let w = to_wide(bth_id);
    let device = unsafe { e.GetDevice(PCWSTR(w.as_ptr())) }.map_err(|err| err.code().0)?;
    unsafe { device.Activate::<IKsControl>(CLSCTX_ALL, None) }.map_err(|err| err.code().0)
}

/// The one `KsProperty` call in the app: the one-shot property, flags GET, 24-byte KSPROPERTY.
/// Called through the generated vtable so the exact HRESULT (success codes too) is logged.
fn ks_one_shot(ks: &IKsControl, op: OneShot) -> (i32, u32) {
    let id = match op {
        OneShot::Connect => KSPROPERTY_ONESHOT_RECONNECT,
        OneShot::Disconnect => KSPROPERTY_ONESHOT_DISCONNECT,
    };
    let prop = KSIDENTIFIER {
        Anonymous: KSIDENTIFIER_0 { Anonymous: KSIDENTIFIER_0_0 { Set: KSPROPSETID_BTAUDIO, Id: id, Flags: KSPROPERTY_TYPE_GET } },
    };
    let mut bytes = 0u32;
    let hr = unsafe {
        (Interface::vtable(ks).KsProperty)(
            Interface::as_raw(ks),
            &prop,
            size_of::<KSIDENTIFIER>() as u32,
            core::ptr::null_mut(),
            0,
            &mut bytes,
        )
    };
    (hr.0, bytes)
}

/// The 12 hex digits in `…&<mac>_c<hex>#…` of a `bthenum` ID, upper case.
fn mac_of(bth_id: &str) -> Option<String> {
    let b = bth_id.as_bytes();
    for i in 0..b.len() {
        if b[i] != b'&' || i + 15 > b.len() {
            continue;
        }
        let mac = &b[i + 1..i + 13];
        if !mac.iter().all(u8::is_ascii_hexdigit) || b[i + 13] != b'_' || !b[i + 14].eq_ignore_ascii_case(&b'c') {
            continue;
        }
        let rest = &b[i + 15..];
        let digits = rest.iter().take_while(|c| c.is_ascii_hexdigit()).count();
        if digits > 0 && rest.get(digits) == Some(&b'#') {
            return Some(core::str::from_utf8(mac).ok()?.to_ascii_uppercase());
        }
    }
    None
}

fn open_store(device: &IMMDevice) -> Option<IPropertyStore> {
    unsafe { device.OpenPropertyStore(STGM_READ) }.ok()
}

/// A PROPVARIANT that is always cleared.
struct Prop(PROPVARIANT);

impl Drop for Prop {
    fn drop(&mut self) {
        let _ = unsafe { PropVariantClear(&mut self.0) };
    }
}

fn read_prop(store: &IPropertyStore, key: &PROPERTYKEY) -> Option<Prop> {
    unsafe { store.GetValue(key) }.ok().map(Prop)
}

fn read_string(store: &IPropertyStore, key: &PROPERTYKEY) -> Option<String> {
    let p = read_prop(store, key)?;
    unsafe {
        let v = &p.0.Anonymous.Anonymous;
        let s = v.Anonymous.pwszVal;
        (v.vt.0 == VT_LPWSTR && !s.is_null()).then(|| String::from_utf16_lossy(s.as_wide()))
    }
}

fn read_guid_container(store: &IPropertyStore) -> Option<Guid> {
    let p = read_prop(store, &PKEY_Device_ContainerId)?;
    unsafe {
        let v = &p.0.Anonymous.Anonymous;
        let g = v.Anonymous.puuid;
        (v.vt.0 == VT_CLSID && !g.is_null()).then(|| {
            let g = *g;
            Guid { data1: g.data1, data2: g.data2, data3: g.data3, data4: g.data4 }
        })
    }
}

/// A string COM allocated for us, freed.
fn take_pwstr(p: PWSTR) -> String {
    let s = unsafe { String::from_utf16_lossy(p.as_wide()) };
    unsafe { CoTaskMemFree(Some(p.0 as _)) };
    s
}

fn com_message(e: windows_core::Error) -> String {
    let m = e.message();
    if m.is_empty() { format!("HRESULT 0x{:08X}", e.code().0) } else { m }
}

#[implement(IMMNotificationClient)]
struct Client {
    notifications: Arc<AtomicU32>,
    on_change: OnEndpointChanged,
}

impl Client {
    fn changed(&self, id: &PCWSTR) {
        self.notifications.fetch_add(1, Ordering::Relaxed);
        (self.on_change)(&unsafe { id.to_string() }.unwrap_or_default());
    }
}

impl IMMNotificationClient_Impl for Client_Impl {
    fn OnDeviceStateChanged(&self, id: &PCWSTR, _: DEVICE_STATE) -> windows_core::Result<()> {
        self.changed(id);
        Ok(())
    }
    fn OnDeviceAdded(&self, id: &PCWSTR) -> windows_core::Result<()> {
        self.changed(id);
        Ok(())
    }
    fn OnDeviceRemoved(&self, id: &PCWSTR) -> windows_core::Result<()> {
        self.changed(id);
        Ok(())
    }
    fn OnDefaultDeviceChanged(&self, _: EDataFlow, _: ERole, _: &PCWSTR) -> windows_core::Result<()> {
        self.notifications.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
    fn OnPropertyValueChanged(&self, _: &PCWSTR, _: &PROPERTYKEY) -> windows_core::Result<()> {
        self.notifications.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}
