//! Bluetooth audio devices as the tray sees them: render endpoints grouped by container ID.
use crate::Guid;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OneShot {
    Connect,
    Disconnect,
}

impl fmt::Display for OneShot {
    /// The C# enum's name (`Connect`, `Disconnect`), as the log and timeline write it.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            OneShot::Connect => "Connect",
            OneShot::Disconnect => "Disconnect",
        })
    }
}

/// Core Audio endpoint states (`mmdeviceapi.h` `DEVICE_STATE_*`).
pub mod device_state {
    use alloc::format;
    use alloc::string::String;

    pub const ACTIVE: u32 = 0x1;
    pub const DISABLED: u32 = 0x2;
    pub const NOT_PRESENT: u32 = 0x4;
    pub const UNPLUGGED: u32 = 0x8;

    pub fn name(state: u32) -> String {
        match state {
            ACTIVE => "ACTIVE".into(),
            DISABLED => "DISABLED".into(),
            NOT_PRESENT => "NOTPRESENT".into(),
            UNPLUGGED => "UNPLUGGED".into(),
            _ => format!("0x{state:X}"),
        }
    }
}

/// ASCII case-insensitive `contains` (the C# code used `OrdinalIgnoreCase` on ASCII IDs).
pub fn contains_ignore_case(haystack: &str, needle: &str) -> bool {
    let (h, n) = (haystack.as_bytes(), needle.as_bytes());
    n.is_empty() || h.windows(n.len()).any(|w| w.eq_ignore_ascii_case(n))
}

/// One render endpoint whose topology leads to a Bluetooth KS filter.
#[derive(Clone, Debug, PartialEq)]
pub struct BtEndpoint {
    pub endpoint_id: String,
    pub name: String,
    pub interface_name: Option<String>,
    pub state: u32,
    pub bth_device_id: String,
    pub container_id: Guid,
    pub mac: Option<String>,
    /// Found only through PnP (Core Audio's enumeration omits it).
    pub hidden_from_enumeration: bool,
    /// The KS control's activation result; [`KS_NOT_ATTEMPTED`] when it wasn't tried.
    pub ks_activate_hr: i32,
}

pub const KS_NOT_ATTEMPTED: i32 = i32::MIN;

impl BtEndpoint {
    pub fn active(&self) -> bool {
        self.state == device_state::ACTIVE
    }

    pub fn is_a2dp(&self) -> bool {
        contains_ignore_case(&self.bth_device_id, "\\bthenum#")
    }
}

/// A Bluetooth audio device: all its render endpoints, grouped by container ID.
#[derive(Clone, Debug, PartialEq)]
pub struct BtDevice {
    pub container_id: Guid,
    pub name: String,
    pub mac: Option<String>,
    pub endpoints: Vec<BtEndpoint>,
}

impl BtDevice {
    /// Connected if any of its endpoints (A2DP or HFP) is ACTIVE.
    pub fn connected(&self) -> bool {
        self.endpoints.iter().any(BtEndpoint::active)
    }

    /// e.g. `A2DP=ACTIVE, HFP=NOTPRESENT`, logged on change.
    pub fn state_signature(&self) -> String {
        let parts: Vec<String> = self
            .endpoints
            .iter()
            .map(|e| format!("{}={}", if e.is_a2dp() { "A2DP" } else { "HFP" }, device_state::name(e.state)))
            .collect();
        parts.join(", ")
    }
}

/// A render endpoint that isn't Bluetooth (kept only for `--list --all`).
#[derive(Clone, Debug, PartialEq)]
pub struct OtherEndpoint {
    pub endpoint_id: String,
    pub name: String,
    pub state: u32,
    pub topology_ids: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SendResult {
    pub endpoint_name: String,
    pub bth_device_id: String,
    pub hr: i32,
    pub bytes_returned: u32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Snapshot {
    pub devices: Vec<BtDevice>,
    pub others: Vec<OtherEndpoint>,
    pub error: Option<String>,
}

/// Logs a one-shot send's outcome, one line per KS control.
pub fn send_log(container: Guid, op: OneShot, results: &[SendResult]) {
    let prop = match op {
        OneShot::Connect => "ONESHOT_RECONNECT",
        OneShot::Disconnect => "ONESHOT_DISCONNECT",
    };
    for r in results {
        crate::log!("KsProperty {prop} -> hr=0x{:08X} bytes={}  [{}]  {}", r.hr, r.bytes_returned, r.endpoint_name, r.bth_device_id);
    }
    if results.is_empty() {
        crate::log!("KsProperty {prop}: no Bluetooth KS controls found for container {container}");
    }
}
