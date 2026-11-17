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

/// A decimal number (`[+-]digits[.digits][e[+-]digits]`) as `double.Parse` with the invariant
/// culture reads it, for inputs of up to 15 significant digits and a decimal exponent within ±22:
/// the digits and the power of ten are then both exact doubles, so one IEEE multiply or divide
/// gives the correctly rounded result, bit for bit what a full parser returns. Anything else is
/// `None`. (Rust's general float parser would add ~14 KB of tables to the exe for numbers the
/// glyph files and `--zoom` never use.)
pub fn parse_f64(s: &str) -> Option<f64> {
    let b = s.as_bytes();
    let mut i = 0;
    let neg = match b.first() {
        Some(b'-') => {
            i += 1;
            true
        }
        Some(b'+') => {
            i += 1;
            false
        }
        _ => false,
    };
    let (mut mantissa, mut digits, mut scale, mut any) = (0u64, 0u32, 0i32, false);
    let mut seen_point = false;
    while i < b.len() {
        match b[i] {
            c @ b'0'..=b'9' => {
                any = true;
                if mantissa == 0 && c == b'0' {
                    // Leading zeros carry no significant digits.
                } else {
                    digits += 1;
                    if digits > 15 {
                        return None;
                    }
                    mantissa = mantissa * 10 + (c - b'0') as u64;
                }
                if seen_point {
                    scale -= 1;
                }
            }
            b'.' if !seen_point => seen_point = true,
            _ => break,
        }
        i += 1;
    }
    if !any {
        return None;
    }
    if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
        i += 1;
        let eneg = match b.get(i) {
            Some(b'-') => {
                i += 1;
                true
            }
            Some(b'+') => {
                i += 1;
                false
            }
            _ => false,
        };
        let (mut e, mut edigits) = (0i32, 0);
        while i < b.len() && b[i].is_ascii_digit() {
            e = e.saturating_mul(10).saturating_add((b[i] - b'0') as i32);
            edigits += 1;
            i += 1;
        }
        if edigits == 0 {
            return None;
        }
        scale = scale.saturating_add(if eneg { -e } else { e });
    }
    if i != b.len() {
        return None;
    }
    let m = mantissa as f64; // exact: at most 15 digits
    let v = if mantissa == 0 {
        0.0
    } else if (0..=22).contains(&scale) {
        m * POW10_22[scale as usize]
    } else if (-22..0).contains(&scale) {
        m / POW10_22[(-scale) as usize]
    } else {
        return None;
    };
    Some(if neg { -v } else { v })
}

/// The powers of ten that are exact doubles.
const POW10_22: [f64; 23] =
    [1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16, 1e17, 1e18, 1e19, 1e20, 1e21, 1e22];

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
    fn parse_matches_the_full_parser() {
        let cases = [
            "0",
            "-0",
            "1",
            "1.25",
            "1.125",
            "-2.5",
            "+3",
            ".5",
            "5.",
            "0.000123",
            "6.333",
            "22.667",
            "123456789012345",
            "1e3",
            "2.5E-3",
            "007.10",
        ];
        for c in cases {
            let want: f64 = c.parse().unwrap();
            assert_eq!(parse_f64(c).map(f64::to_bits), Some(want.to_bits()), "{c}");
        }
        for bad in ["", "-", ".", "1.2.3", "1e", "abc", "1x", "1234567890123456"] {
            assert_eq!(parse_f64(bad), None, "{bad}");
        }
        // Every number in the built-in glyphs, tokenised as SVG path data is ("1.5-2.3.4" is
        // 1.5, -2.3, .4).
        for svg in [include_str!("../../assets/airpods-pro-fill.svg"), include_str!("../../assets/airpods-pro-line.svg")] {
            let b = svg.as_bytes();
            let (mut i, mut n) = (0, 0);
            while i < b.len() {
                if !(b[i].is_ascii_digit()
                    || b[i] == b'.'
                    || (b[i] == b'-' && b.get(i + 1).is_some_and(|c| c.is_ascii_digit() || *c == b'.')))
                {
                    i += 1;
                    continue;
                }
                let start = i;
                i += 1;
                let mut dot = b[start] == b'.';
                while i < b.len() && (b[i].is_ascii_digit() || (b[i] == b'.' && !dot)) {
                    dot |= b[i] == b'.';
                    i += 1;
                }
                let tok = &svg[start..i];
                let Ok(want) = tok.parse::<f64>() else { continue }; // a stray "." in the XML text
                assert_eq!(parse_f64(tok).map(f64::to_bits), Some(want.to_bits()), "{tok}");
                n += 1;
            }
            assert!(n > 1000);
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
