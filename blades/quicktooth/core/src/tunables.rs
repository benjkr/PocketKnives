//! Every timing and animation number the app uses, in one place. The exe only ever uses the
//! defaults; the simulator can override any of them by name (`qt-sim --tune Travel=2.4`) to try a
//! change before it's committed here. Stored as atomics so they're safe to read from any thread.
use alloc::string::String;
use core::sync::atomic::{AtomicU64, Ordering};

/// Defines the table once: the enum, names, defaults and whether a value is an `int` in C#
/// (set with truncation, as `(int)v` did).
macro_rules! tunables {
    ($($name:ident = $default:expr, $int:expr;)*) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        #[repr(usize)]
        pub enum T { $($name,)* }

        const NAMES: &[&str] = &[$(stringify!($name),)*];
        const DEFAULTS: &[f64] = &[$($default as f64,)*];
        const IS_INT: &[bool] = &[$($int,)*];
    };
}

tunables! {
    // ---- connect / disconnect (the controller). Measured on the dev PC: with the Hands-Free
    // control included, reconnects land in ~1–3 s; the timeout leaves room for one resend.
    PendingTimeoutMs = 20000, true;
    IdlePollMs = 20000, true;
    PendingPollMs = 500, true;       // notifications may not fire for hidden endpoints
    ResendAfterMs = 8000, true;
    LinkTeardownWaitMs = 5000, true;
    TeardownPollMs = 200, true;
    TeardownLogAboveMs = 300, true;
    DebounceMs = 200, true;          // endpoint notification -> refresh
    AddRetryMs = 2000, true;
    AddMaxAttempts = 60, true;
    PromoteRetryMs = 3000, true;
    PromoteMaxAttempts = 10, true;
    TooltipMaxChars = 127, true;     // NOTIFYICONDATA.szTip
    // ---- battery
    BatteryCheckMs = 60000, true;
    FirstBatteryMs = 2500, true;     // first reading soon after connecting
    BleScanSeconds = 12, true;
    BatteryMaxAgeMin = 5, true;
    BatteryShowAgeMin = 15, true;
    MenuBatteryMaxAgeS = 60, true;
    // ---- where the buds are (the advert watch): continuous while connected; otherwise a
    // PodsScanMs window every PodsScanEveryMs. Forgotten PodsStaleMs after the last advert.
    PodsScanEveryMs = 60000, true;
    PodsScanMs = 6000, true;
    PodsStaleMs = 90000, true;
    CaseAnimSeconds = 0.52, false;   // the case icon's move from one pose to the next
    // The default low-battery warning: the connected icon gets the orange dot at or below this
    // level (the emptier bud, not while charging). Settings can pick another.
    LowBatteryPercent = 20, true;
    // ---- icon and animation (the renderer)
    AnimFrameMs = 83, true;          // ≈ 12 fps nominal (Win32 timers make it ~10.5)
    SpringSeconds = 0.2, false;
    LandSeconds = 0.5, false;
    DisconnectSeconds = 0.8, false;
    CancelSeconds = 0.3, false;
    Travel = 1.9, false;             // how far each earbud moves apart (24-unit glyph space)
    TracerPeriod = 1.1, false;       // seconds per lap
    SpotRadius = 3.0, false;
    // Tracer path: 1 = a short glow running around each earbud's whole outline; 0 = the earlier
    // spot circling each head.
    TracerOutline = 1, true;
    // Outline tracer: how long the lit stretch is, and how far it reaches either side of the edge.
    TracerLength = 6.0, false;
    TracerWidth = 1.0, false;
    // How much bigger than the SVG's 24-unit box the glyph is drawn: the artwork spans x 1.33–22.67
    // / y 5–18.7, so 1.125 makes it fill the 16 px width (16 × 10.3 px). tools/airpods_glyph.py lays
    // the artwork out for exactly this zoom at 16 px (stems on whole pixels): change both together.
    Zoom = 1.125, false;
    // The low-battery dot's radius, and the gap cut around it, in pixels at 16 px.
    LowBatteryDotRadius = 2.4, false;
    LowBatteryDotGap = 0.8, false;
    // The dot's ripple: how long one lasts, and how often it starts while the dot shows.
    LowBatteryAnimSeconds = 0.7, false;
    LowBatteryReplayMs = 3000, true;
}

pub const ON_DARK_TASKBAR: &str = "#FFFFFF";
pub const ON_LIGHT_TASKBAR: &str = "#1A1A1A";
pub const LOW_BATTERY_ON_DARK: &str = "#FF8C00";
pub const LOW_BATTERY_ON_LIGHT: &str = "#CA5010";

const N: usize = NAMES.len();

static OVERRIDES: [AtomicU64; N] = [const { AtomicU64::new(NOT_SET) }; N];
/// A NaN pattern no value is ever set to: "use the default".
const NOT_SET: u64 = 0x7FF8_DEAD_BEEF_0001;

/// The current value.
pub fn get(t: T) -> f64 {
    value_at(t as usize)
}

fn value_at(i: usize) -> f64 {
    let bits = OVERRIDES[i].load(Ordering::Relaxed);
    if bits == NOT_SET { DEFAULTS[i] } else { f64::from_bits(bits) }
}

/// The current value of an `int` tunable.
pub fn int(t: T) -> i32 {
    get(t) as i32
}

/// Sets one tunable by name (`--tune`); false if there's no such name.
pub fn set(name: &str, value: f64) -> bool {
    let Some(i) = NAMES.iter().position(|n| *n == name) else { return false };
    let v = if IS_INT[i] { value as i32 as f64 } else { value };
    OVERRIDES[i].store(v.to_bits(), Ordering::Relaxed);
    true
}

pub fn set_value(t: T, value: f64) {
    set(NAMES[t as usize], value);
}

pub fn reset_all() {
    for o in &OVERRIDES {
        o.store(NOT_SET, Ordering::Relaxed);
    }
}

/// All tunables and their current values as a JSON object, plus the two icon colours.
pub fn to_json() -> String {
    let mut s = String::from("{");
    for (i, name) in NAMES.iter().enumerate() {
        s.push('"');
        s.push_str(name);
        s.push_str("\":");
        crate::num::write_r(&mut s, value_at(i));
        s.push(',');
    }
    s.push_str("\"OnDarkTaskbar\":\"");
    s.push_str(ON_DARK_TASKBAR);
    s.push_str("\",\"OnLightTaskbar\":\"");
    s.push_str(ON_LIGHT_TASKBAR);
    s.push_str("\"}");
    s
}
