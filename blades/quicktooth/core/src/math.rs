//! Floating-point maths for the renderer. Exact operations (sqrt, floor, ...) give the same bits
//! everywhere. `sin`, `cos` and `pow` don't: the C# exe got them from the Windows C runtime and
//! the browser build from musl's libm, so each build takes them from the same place to keep its
//! pixels (the simulator's parity check allows the two builds 1/255 apart, as before).
//
// Platform quirk: the exe imports sin/cos/pow from the universal C runtime's math API set
// (what .NET's Math.Sin/Cos/Pow called), raw-dylib so no import library is needed.

#[cfg(windows)]
mod crt {
    #[link(name = "api-ms-win-crt-math-l1-1-0", kind = "raw-dylib")]
    unsafe extern "C" {
        pub safe fn sin(x: f64) -> f64;
        pub safe fn cos(x: f64) -> f64;
        pub safe fn pow(x: f64, y: f64) -> f64;
    }
}

#[cfg(windows)]
pub use crt::{cos, pow, sin};

#[cfg(not(windows))]
pub use libm::{cos, pow, sin};

pub use libm::{ceil, fabs as abs, floor, sqrt, trunc};

/// `Math.Round(x)`: ties to even.
pub fn round(x: f64) -> f64 {
    libm::rint(x)
}

pub const PI: f64 = core::f64::consts::PI;
