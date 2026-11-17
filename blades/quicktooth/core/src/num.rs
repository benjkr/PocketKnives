//! Numbers written and rounded exactly as the C# version did, because log lines, timelines and
//! JSON were recorded with .NET's rules and are compared across builds.
use alloc::string::String;
use core::fmt::Write;

/// `double.ToString("R", InvariantCulture)` (.NET Core 3.0+): the shortest digits that read back
/// as the same double, in fixed notation unless the decimal exponent is below -4 or above 14
/// (`1E-05`, `1E+15`, at least two exponent digits).
pub fn fmt_r(v: f64) -> String {
    let mut s = String::new();
    write_r(&mut s, v);
    s
}

pub fn write_r(out: &mut String, v: f64) {
    if v.is_nan() {
        out.push_str("NaN");
        return;
    }
    if v.is_infinite() {
        out.push_str(if v > 0.0 { "Infinity" } else { "-Infinity" });
        return;
    }
    if v == 0.0 {
        out.push_str(if v.is_sign_negative() { "-0" } else { "0" });
        return;
    }
    // Rust's `{:e}` gives the same shortest round-trip digits: "d.ddddde-x".
    let mut sci = String::new();
    let _ = write!(sci, "{:e}", v.abs());
    let (mantissa, exp) = sci.split_once('e').unwrap_or((&sci, "0"));
    let exp: i32 = exp.parse().unwrap_or(0);
    let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
    if v < 0.0 {
        out.push('-');
    }
    // .NET's "scale": the decimal point sits after `scale` digits (0.d1d2... × 10^scale).
    let scale = exp + 1;
    if scale > 15 || scale < -3 {
        out.push_str(&digits[..1]);
        if digits.len() > 1 {
            out.push('.');
            out.push_str(&digits[1..]);
        }
        let _ = write!(out, "E{}{:02}", if exp < 0 { '-' } else { '+' }, exp.unsigned_abs());
    } else if scale <= 0 {
        out.push_str("0.");
        for _ in 0..(-scale) {
            out.push('0');
        }
        out.push_str(&digits);
    } else {
        let scale = scale as usize;
        if digits.len() <= scale {
            out.push_str(&digits);
            for _ in digits.len()..scale {
                out.push('0');
            }
        } else {
            out.push_str(&digits[..scale]);
            out.push('.');
            out.push_str(&digits[scale..]);
        }
    }
}

/// `Math.Round(value)`: to the nearest integer, ties to even.
pub fn round_even(v: f64) -> f64 {
    libm::rint(v)
}

/// `Math.Round(value, digits, MidpointRounding.AwayFromZero)` as .NET 8+ computes it.
pub fn round_away(v: f64, digits: u32) -> f64 {
    const LIMIT: f64 = 1e16;
    if libm::fabs(v) >= LIMIT {
        return v;
    }
    let p = POW10[digits as usize];
    let scaled = v * p;
    // BitDecrement(0.5) = 0.49999999999999994: halves round away, nothing below them does.
    let half = libm::copysign(0.499_999_999_999_999_94, scaled);
    libm::trunc(scaled + half) / p
}

const POW10: [f64; 16] = [1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15];

/// `(long)value` in C#: truncation toward zero (only used on values far inside `i64`).
pub fn trunc_i64(v: f64) -> i64 {
    v as i64
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn r_format_matches_dotnet() {
        let cases: &[(f64, &str)] = &[
            (0.0, "0"),
            (-0.0, "-0"),
            (1.0, "1"),
            (20000.0, "20000"),
            (0.2, "0.2"),
            (1.1, "1.1"),
            (0.1 + 0.2, "0.30000000000000004"),
            (123.456, "123.456"),
            (0.0001, "0.0001"),
            (0.00001, "1E-05"),
            (0.000123, "0.000123"),
            (0.0000123, "1.23E-05"),
            (1e14, "100000000000000"),
            (1e15, "1E+15"),
            (123456789012345680.0, "1.2345678901234568E+17"),
            (5.551115123125783e-17, "5.551115123125783E-17"),
            (-2.5, "-2.5"),
            (1.7976931348623157e308, "1.7976931348623157E+308"),
            (f64::NAN, "NaN"),
        ];
        for (v, want) in cases {
            assert_eq!(fmt_r(*v), *want, "{v:e}");
        }
    }

    #[test]
    fn rounding() {
        assert_eq!(round_away(0.12345, 4), 0.1235);
        assert_eq!(round_away(2.5, 0), 3.0);
        assert_eq!(round_away(-2.5, 0), -3.0);
        assert_eq!(round_away(1.00005, 4), 1.0001);
        assert_eq!(round_even(2.5), 2.0);
        assert_eq!(round_even(3.5), 4.0);
        assert_eq!(round_even(-0.5), -0.0);
    }
}
