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
use crt as imp;

#[cfg(not(windows))]
use libm as imp;

// LLVM knows these names: it folds calls on constants with its own maths and rewrites
// `pow(x, 2.0)` as `x * x`. .NET always called the library, so the arguments are made opaque.

/// `Math.Sin`.
pub fn sin(x: f64) -> f64 {
    imp::sin(core::hint::black_box(x))
}

/// `Math.Cos`.
pub fn cos(x: f64) -> f64 {
    imp::cos(core::hint::black_box(x))
}

/// `Math.Pow`.
pub fn pow(x: f64, y: f64) -> f64 {
    imp::pow(core::hint::black_box(x), core::hint::black_box(y))
}

// `fmod` is C#'s `%` on doubles (exact), which a no_std exe has no runtime for.
pub use libm::{ceil, fabs as abs, floor, fmod, sqrt, trunc};

/// `Math.Round(x)`: ties to even.
pub fn round(x: f64) -> f64 {
    libm::rint(x)
}

/// `Math.Max(double, double)`: NaN wins, and +0 is above -0.
pub fn max(a: f64, b: f64) -> f64 {
    if a != b {
        if a.is_nan() {
            return a;
        }
        return if b < a { a } else { b };
    }
    if b.is_sign_negative() { a } else { b }
}

/// `Math.Min(double, double)`: NaN wins, and -0 is below +0.
pub fn min(a: f64, b: f64) -> f64 {
    if a.is_nan() {
        return a; // .NET's NaN has the sign bit set, so its `IsNegative(val1)` test returns it
    }
    if a != b {
        return if a < b { a } else { b };
    }
    if a.is_sign_negative() { a } else { b }
}

/// `Math.Clamp(double, min, max)` (for `min <= max`).
pub fn clamp(v: f64, lo: f64, hi: f64) -> f64 {
    if v < lo {
        lo
    } else if v > hi {
        hi
    } else {
        v
    }
}

pub const PI: f64 = core::f64::consts::PI;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_min_match_dotnet() {
        assert!(max(0.0, -0.0).is_sign_positive());
        assert!(max(-0.0, 0.0).is_sign_positive());
        assert!(min(0.0, -0.0).is_sign_negative());
        assert!(min(-0.0, 0.0).is_sign_negative());
        assert!(max(f64::NAN, 1.0).is_nan());
        assert!(max(1.0, f64::NAN).is_nan());
        assert!(min(f64::NAN, 1.0).is_nan());
        assert!(min(1.0, f64::NAN).is_nan());
        assert_eq!(max(1.0, 2.0), 2.0);
        assert_eq!(min(1.0, 2.0), 1.0);
        assert_eq!(max(f64::NEG_INFINITY, 0.0), 0.0);
    }
}
