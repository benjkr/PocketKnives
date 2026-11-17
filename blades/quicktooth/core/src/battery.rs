//! Battery readings, and Apple's BLE status advert that AirPods broadcast.
use crate::devices::BtDevice;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// A battery reading. Apple status adverts give per-bud and case levels in 10 % steps.
#[derive(Clone, Debug, PartialEq)]
pub struct BatteryReading {
    pub left: Option<i32>,
    pub right: Option<i32>,
    pub case: Option<i32>,
    pub buds_charging: bool,
    pub case_charging: bool,
    pub rssi: i32,
    /// When it was read: the host's wall clock, milliseconds since 1970 (C#'s `DateTime.Now`).
    pub at_epoch_ms: f64,
    /// `"ble"` or `"windows"`.
    pub source: String,
}

impl BatteryReading {
    /// The level that matters for listening: the emptier bud (or the single reported level).
    pub fn buds(&self) -> Option<i32> {
        match (self.left, self.right) {
            (Some(l), Some(r)) => Some(l.min(r)),
            (l, r) => l.or(r),
        }
    }

    /// e.g. `30%` or `30% · case 80%`; `None` when nothing useful is known.
    pub fn text(&self) -> Option<String> {
        let mut parts: Vec<String> = Vec::new();
        if let Some(b) = self.buds() {
            parts.push(format!("{b}%{}", if self.buds_charging { " charging" } else { "" }));
        }
        if let Some(c) = self.case {
            parts.push(format!("case {c}%{}", if self.case_charging { " charging" } else { "" }));
        }
        (!parts.is_empty()).then(|| parts.join(" · "))
    }

    pub fn describe(&self) -> String {
        let o = |v: Option<i32>| v.map(|v| format!("{v}")).unwrap_or_else(|| "?".into());
        format!(
            "buds {}% (L {}, R {}), case {}, rssi {}, {}",
            o(self.buds()),
            o(self.left),
            o(self.right),
            o(self.case),
            self.rssi,
            self.source
        )
    }
}

/// The Apple model ID (= the Bluetooth PID, e.g. 0x2027) of a device, if it's an Apple accessory:
/// the first `_vid&0001004c_pid&XXXX` in an endpoint's KS filter ID.
pub fn model_of(device: &BtDevice) -> Option<u16> {
    const MARK: &[u8] = b"_vid&0001004c_pid&";
    for e in &device.endpoints {
        let id = e.bth_device_id.as_bytes();
        let mut i = 0;
        while i + MARK.len() + 4 <= id.len() {
            if id[i..i + MARK.len()].eq_ignore_ascii_case(MARK) {
                let hex = &id[i + MARK.len()..i + MARK.len() + 4];
                if hex.iter().all(u8::is_ascii_hexdigit) {
                    let s = core::str::from_utf8(hex).ok()?;
                    return u16::from_str_radix(s, 16).ok();
                }
            }
            i += 1;
        }
    }
    None
}

/// Parses Apple's "proximity pairing" status message (manufacturer data, company 0x004C, type
/// 0x07), which AirPods broadcast while out of the case:
/// `[0]=0x07 [1]=length [2]=prefix [3..4]=model (LE) [5]=status [6]=bud levels (two nibbles)
/// [7]=charging flags (high nibble) + case level (low nibble) …`.
/// Levels are 0–10 (×10 %); 15 = unknown. Bit 0x20 of the status byte tells which nibble is left.
pub fn parse_apple_status(b: &[u8], model: u16, rssi: i32, now_epoch_ms: f64) -> Option<BatteryReading> {
    if b.len() < 9 || b[0] != 0x07 || b[1] < 7 {
        return None;
    }
    if u16::from_le_bytes([b[3], b[4]]) != model {
        return None;
    }
    let flipped = b[5] & 0x20 == 0;
    let (hi, lo) = ((b[6] >> 4) as i32, (b[6] & 0x0F) as i32);
    let level = |n: i32| (n <= 10).then_some(n * 10);
    let (left, right) = if flipped { (level(hi), level(lo)) } else { (level(lo), level(hi)) };
    let charging = (b[7] >> 4) & 0x07;
    let case = level((b[7] & 0x0F) as i32);
    if left.is_none() && right.is_none() && case.is_none() {
        return None;
    }
    Some(BatteryReading {
        left,
        right,
        case,
        buds_charging: charging & 0b011 != 0,
        case_charging: charging & 0b100 != 0,
        rssi,
        at_epoch_ms: now_epoch_ms,
        source: "ble".into(),
    })
}
