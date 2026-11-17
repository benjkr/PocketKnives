//! PnP (cfgmgr32) helpers.
//!
//! Render endpoint IDs: on recent Windows 11 builds Core Audio's `EnumAudioEndpoints` hides a
//! Bluetooth device's hands-free (HFP) render endpoint, but its `SWD\MMDEVAPI` devnode still
//! exists and `IMMDeviceEnumerator::GetDevice` opens it. Neither source is complete on its own,
//! so the enumeration unions this list with `EnumAudioEndpoints`.
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use pk::sync::Mutex;
use pk::wide::to_wide;
use quicktooth_core::Guid;
use windows_sys::Win32::Devices::DeviceAndDriverInstallation::{
    CM_GETIDLIST_FILTER_ENUMERATOR, CM_Get_DevNode_PropertyW, CM_Get_Device_ID_List_SizeW, CM_Get_Device_ID_ListW,
    CM_LOCATE_DEVNODE_PHANTOM, CM_Locate_DevNodeW, CR_BUFFER_SMALL, CR_SUCCESS,
};
use windows_sys::Win32::Foundation::DEVPROPKEY;

const PREFIX: &str = "SWD\\MMDEVAPI\\";

/// The flag the Bluetooth stack sets on `BTHENUM\DEV_<mac>` while the link is up (what Settings shows).
const BLUETOOTH_CONNECTED: DEVPROPKEY = key(0x83DA6326_97A6_4088_9453_A1923F573B29, 15);
/// The battery percentage (a BYTE) some headsets report (HFP battery indicators).
const BLUETOOTH_BATTERY: DEVPROPKEY = key(0x104EA319_6EE2_4701_BD47_8DDBF425BBE5, 2);

const fn key(fmtid: u128, pid: u32) -> DEVPROPKEY {
    let g = Guid::from_u128(fmtid);
    DEVPROPKEY { fmtid: windows_sys::core::GUID { data1: g.data1, data2: g.data2, data3: g.data3, data4: g.data4 }, pid }
}

/// Render endpoint IDs of `SWD\MMDEVAPI` devnodes, present or not, lower case
/// (`{0.0.0.00000000}.{guid}`).
pub fn render_endpoint_ids() -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    for id in device_ids("SWD")? {
        let Some(rest) = strip_prefix_ignore_case(&id, PREFIX) else { continue };
        // {0.0.0} = render, {0.0.1} = capture
        if rest.starts_with("{0.0.0.") {
            out.push(rest.to_ascii_lowercase());
        }
    }
    Ok(out)
}

/// Whether Windows considers the Bluetooth device with this MAC connected (`None` = unknown).
pub fn bluetooth_connected(mac: &str) -> Option<bool> {
    bluetooth_property(mac, &BLUETOOTH_CONNECTED).filter(|b| !b.is_empty()).map(|b| b[0] != 0)
}

/// The battery percentage Windows has for the Bluetooth device with this MAC, if any.
pub fn bluetooth_battery(mac: &str) -> Option<i32> {
    bluetooth_property(mac, &BLUETOOTH_BATTERY).filter(|b| !b.is_empty() && b[0] <= 100).map(|b| b[0] as i32)
}

/// MAC (as given) → its `BTHENUM\DEV_<mac>\…` instance ID, looked up once per MAC.
static DEVNODE_BY_MAC: Mutex<Vec<(String, Option<String>)>> = Mutex::new(Vec::new());

fn bluetooth_property(mac: &str, key: &DEVPROPKEY) -> Option<Vec<u8>> {
    let instance = {
        let mut cache = DEVNODE_BY_MAC.lock();
        match cache.iter().find(|(m, _)| m.eq_ignore_ascii_case(mac)) {
            Some((_, id)) => id.clone(),
            None => {
                let prefix = format!("BTHENUM\\DEV_{mac}\\");
                let id = device_ids("BTHENUM").ok()?.into_iter().find(|i| strip_prefix_ignore_case(i, &prefix).is_some());
                cache.push((mac.into(), id.clone()));
                id
            }
        }
    }?;
    let w = to_wide(&instance);
    let mut devinst = 0u32;
    if unsafe { CM_Locate_DevNodeW(&mut devinst, w.as_ptr(), CM_LOCATE_DEVNODE_PHANTOM) } != CR_SUCCESS {
        return None;
    }
    let mut buf = [0u8; 16];
    let mut size = buf.len() as u32;
    let mut ty = 0u32;
    if unsafe { CM_Get_DevNode_PropertyW(devinst, key, &mut ty, buf.as_mut_ptr(), &mut size, 0) } != CR_SUCCESS || size == 0 {
        return None;
    }
    Some(buf[..size as usize].to_vec())
}

fn device_ids(enumerator: &str) -> Result<Vec<String>, String> {
    let filter = to_wide(enumerator);
    for _ in 0..3 {
        let mut len = 0u32;
        let cr = unsafe { CM_Get_Device_ID_List_SizeW(&mut len, filter.as_ptr(), CM_GETIDLIST_FILTER_ENUMERATOR) };
        if cr != CR_SUCCESS {
            return Err(format!("CM_Get_Device_ID_List_Size cr={cr}"));
        }
        let mut buf = vec![0u16; len as usize];
        let cr = unsafe { CM_Get_Device_ID_ListW(filter.as_ptr(), buf.as_mut_ptr(), len, CM_GETIDLIST_FILTER_ENUMERATOR) };
        if cr == CR_BUFFER_SMALL {
            continue; // the list changed between the two calls
        }
        if cr != CR_SUCCESS {
            return Err(format!("CM_Get_Device_ID_List cr={cr}"));
        }
        return Ok(buf.split(|&c| c == 0).filter(|s| !s.is_empty()).map(String::from_utf16_lossy).collect());
    }
    Err("device list kept changing".into())
}

fn strip_prefix_ignore_case<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    let n = prefix.len();
    (s.len() >= n && s.is_char_boundary(n) && s[..n].eq_ignore_ascii_case(prefix)).then(|| &s[n..])
}
