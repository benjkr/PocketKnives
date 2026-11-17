//! A Windows GUID with .NET's parsing (`Guid.TryParse`) and its `"B"` format.
use alloc::string::String;
use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl Guid {
    pub const EMPTY: Guid = Guid { data1: 0, data2: 0, data3: 0, data4: [0; 8] };

    pub const fn from_u128(v: u128) -> Guid {
        Guid {
            data1: (v >> 96) as u32,
            data2: (v >> 80) as u16,
            data3: (v >> 64) as u16,
            data4: (v as u64).to_be_bytes(),
        }
    }

    pub fn is_empty(&self) -> bool {
        *self == Guid::EMPTY
    }

    /// `Guid.TryParse`: the `D` (`xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`), `N` (32 digits), `B`
    /// (`{D}`) and `P` (`(D)`) forms, any case, surrounding whitespace ignored. (.NET's `X` form,
    /// `{0x...,{0x...}}`, isn't accepted: nothing writes it.)
    pub fn parse(s: &str) -> Option<Guid> {
        let s = s.trim();
        let (inner, braced) = if let Some(rest) = s.strip_prefix('{') {
            (rest.strip_suffix('}')?, true)
        } else if let Some(rest) = s.strip_prefix('(') {
            (rest.strip_suffix(')')?, true)
        } else {
            (s, false)
        };
        let hex: [u8; 32] = if inner.len() == 36 {
            let b = inner.as_bytes();
            if b[8] != b'-' || b[13] != b'-' || b[18] != b'-' || b[23] != b'-' {
                return None;
            }
            let mut out = [0u8; 32];
            let mut n = 0;
            for (i, &c) in b.iter().enumerate() {
                if matches!(i, 8 | 13 | 18 | 23) {
                    continue;
                }
                out[n] = c;
                n += 1;
            }
            out
        } else if inner.len() == 32 && !braced {
            inner.as_bytes().try_into().ok()?
        } else {
            return None;
        };
        let mut v: u128 = 0;
        for c in hex {
            v = (v << 4) | (c as char).to_digit(16)? as u128;
        }
        Some(Guid::from_u128(v))
    }

    /// `ToString("B")`: `{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}`, lower case.
    pub fn to_b(&self) -> String {
        alloc::format!("{self}")
    }
}

impl fmt::Display for Guid {
    /// The `B` form.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = &self.data4;
        write!(
            f,
            "{{{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}}}",
            self.data1, self.data2, self.data3, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]
        )
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::Guid;

    #[test]
    fn parse_and_format() {
        let b = "{5a1b2c3d-0001-0002-a1b2-0123456789ab}";
        let g = Guid::parse(b).unwrap();
        assert_eq!(g.to_b(), b);
        assert_eq!(Guid::parse("5A1B2C3D-0001-0002-A1B2-0123456789AB"), Some(g));
        assert_eq!(Guid::parse("5a1b2c3d00010002a1b20123456789ab"), Some(g));
        assert_eq!(Guid::parse(" (5a1b2c3d-0001-0002-a1b2-0123456789ab) "), Some(g));
        assert_eq!(Guid::parse("{5a1b2c3d00010002a1b20123456789ab}"), None);
        assert_eq!(Guid::parse("5a1b2c3d-0001-0002-a1b2-0123456789a"), None);
        assert_eq!(Guid::parse("xyz"), None);
        assert_eq!(Guid::parse(""), None);
    }
}
