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
    /// Which bud charges, when the advert says (both false otherwise).
    pub left_charging: bool,
    pub right_charging: bool,
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
    // Bits 0 and 1 go with the low and high level nibbles; bit 2 is the case.
    let charging = (b[7] >> 4) & 0x07;
    let (lo_charging, hi_charging) = (charging & 0b001 != 0, charging & 0b010 != 0);
    let (left_charging, right_charging) = if flipped { (hi_charging, lo_charging) } else { (lo_charging, hi_charging) };
    let case = level((b[7] & 0x0F) as i32);
    if left.is_none() && right.is_none() && case.is_none() {
        return None;
    }
    Some(BatteryReading {
        left,
        right,
        case,
        buds_charging: charging & 0b011 != 0,
        left_charging,
        right_charging,
        case_charging: charging & 0b100 != 0,
        rssi,
        at_epoch_ms: now_epoch_ms,
        source: "ble".into(),
    })
}

/// Where the buds are, from the same status advert.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pods {
    /// Both buds in the case, lid closed.
    Closed,
    /// Both buds in the case, lid open.
    Open,
    /// One bud out of the case (`left`: which one).
    OneOut { left: bool },
    /// Both buds out of the case.
    Out,
}

impl Pods {
    /// For the log and the simulator's timeline.
    pub fn name(self) -> &'static str {
        match self {
            Pods::Closed => "closed",
            Pods::Open => "lid open",
            Pods::OneOut { left: true } => "left out",
            Pods::OneOut { left: false } => "right out",
            Pods::Out => "out",
        }
    }
}

/// The buds' placement in a status advert, and whether a bud is in an ear. Bits measured on
/// AirPods Pro (0x2027), from both buds' adverts (each bud advertises its own view):
/// status `[5]`: 0x04 both buds in the case, 0x10 at least one in the case, 0x0A a bud in an ear,
/// 0x20 the sender is the left bud; lid `[8]`: 0x08 lid closed, 0x20 the sender is in the case.
pub fn parse_pods(b: &[u8], model: u16) -> Option<(Pods, bool)> {
    if b.len() < 9 || b[0] != 0x07 || b[1] < 7 || u16::from_le_bytes([b[3], b[4]]) != model {
        return None;
    }
    let (status, lid) = (b[5], b[8]);
    let pods = if status & 0x04 != 0 {
        if lid & 0x08 != 0 { Pods::Closed } else { Pods::Open }
    } else if status & 0x10 != 0 {
        let sender_left = status & 0x20 != 0;
        let sender_in_case = lid & 0x20 != 0;
        Pods::OneOut { left: sender_left != sender_in_case }
    } else {
        Pods::Out
    };
    Some((pods, status & 0x0A != 0))
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    /// Real adverts from AirPods Pro (0x2027), in the order a test run produced them: lid closed,
    /// opened, the left bud taken out and put in an ear, the right one too, both taken out of the
    /// ears, back in the case, lid closed. Two per step where the two buds disagreed in form.
    /// The same moment from each bud (left out of the case, right charging in it): each says the
    /// right bud charges.
    #[test]
    fn which_bud_charges() {
        for b in [[0x07, 0x19, 0x01, 0x27, 0x20, 0x51, 0x88, 0x95, 0x32], [0x07, 0x19, 0x01, 0x27, 0x20, 0x33, 0x88, 0xA5, 0x12]] {
            let r = parse_apple_status(&b, 0x2027, -50, 1.0).unwrap();
            assert_eq!((r.left_charging, r.right_charging, r.buds_charging), (false, true, true), "{b:02X?}");
        }
    }

    #[test]
    fn pods_from_real_adverts() {
        let cases: &[(&str, Pods, bool)] = &[
            ("07 19 01 27 20 75 88 B5 39 00 04", Pods::Closed, false),
            ("07 19 01 27 20 15 88 B5 39 00 04", Pods::Closed, false),
            ("07 19 01 27 20 15 88 B5 32 00 04", Pods::Open, false),
            ("07 19 01 27 20 75 88 B5 32 00 04", Pods::Open, false),
            ("07 19 01 27 20 51 88 95 32 00 04", Pods::OneOut { left: true }, false), // right bud, in the case
            ("07 19 01 27 20 33 88 A5 12 00 04", Pods::OneOut { left: true }, true),  // left bud, in an ear
            ("07 19 01 27 20 53 88 95 32 00 04", Pods::OneOut { left: true }, true),
            ("07 19 01 27 20 23 88 8F 12 00 04", Pods::Out, true),
            ("07 19 01 27 20 21 88 8F 12 00 04", Pods::Out, false),
            ("07 19 01 27 20 15 88 BF 12 00 04", Pods::Open, false),
            ("07 19 01 27 20 75 88 B5 3B 00 04", Pods::Closed, false),
        ];
        for &(hex, pods, in_ear) in cases {
            let b: std::vec::Vec<u8> = hex.split(' ').map(|h| u8::from_str_radix(h, 16).unwrap()).collect();
            assert_eq!(parse_pods(&b, 0x2027), Some((pods, in_ear)), "{hex}");
            assert!(parse_pods(&b, 0x2014).is_none());
        }
        assert!(parse_pods(&[0x07, 0x19, 0x01, 0x27, 0x20, 0x75, 0x88, 0xB5], 0x2027).is_none());
        // The right bud taken out instead: the senders' left/right bits are the other way round.
        assert_eq!(parse_pods(&[0x07, 0x19, 0x01, 0x27, 0x20, 0x71, 0x88, 0x95, 0x32], 0x2027), Some((Pods::OneOut { left: false }, false)));
        assert_eq!(parse_pods(&[0x07, 0x19, 0x01, 0x27, 0x20, 0x13, 0x88, 0xA5, 0x12], 0x2027), Some((Pods::OneOut { left: false }, true)));
    }

    #[test]
    fn apple_status_advert() {
        // model 0x2027 (LE), status 0x20 set: low nibble is left; buds 9/8, charging bit 1, case 5.
        let b = [0x07, 0x19, 0x01, 0x27, 0x20, 0x20, 0x89, 0x15, 0x00];
        let r = parse_apple_status(&b, 0x2027, -60, 1.0).unwrap();
        assert_eq!((r.left, r.right, r.case), (Some(90), Some(80), Some(50)));
        assert!(r.buds_charging && !r.case_charging);
        assert_eq!(r.buds(), Some(80));
        assert_eq!(r.text().as_deref(), Some("80% charging · case 50%"));
        assert_eq!(r.describe(), "buds 80% (L 90, R 80), case 50, rssi -60, ble");
        // Bit 0x20 clear: the nibbles swap; 15 = unknown; case charging is bit 2.
        let b = [0x07, 0x19, 0x01, 0x27, 0x20, 0x00, 0x3F, 0x4F, 0x00];
        let r = parse_apple_status(&b, 0x2027, -60, 1.0).unwrap();
        assert_eq!((r.left, r.right, r.case), (Some(30), None, None));
        assert!(!r.buds_charging && r.case_charging);
        assert_eq!(r.text().as_deref(), Some("30%"));
        // Another model, a short message, another type, nothing known: no reading.
        assert!(parse_apple_status(&[0x07, 0x19, 0x01, 0x14, 0x20, 0x20, 0x89, 0x15, 0x00], 0x2027, -60, 1.0).is_none());
        assert!(parse_apple_status(&[0x07, 0x19, 0x01, 0x27, 0x20, 0x20, 0x89, 0x15], 0x2027, -60, 1.0).is_none());
        assert!(parse_apple_status(&[0x10, 0x19, 0x01, 0x27, 0x20, 0x20, 0x89, 0x15, 0x00], 0x2027, -60, 1.0).is_none());
        assert!(parse_apple_status(&[0x07, 0x19, 0x01, 0x27, 0x20, 0x20, 0xFF, 0x0F, 0x00], 0x2027, -60, 1.0).is_none());
    }
}
