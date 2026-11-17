//! The command line, split the way the Microsoft C runtime (and so .NET and Rust's std) split it:
//! whitespace separates arguments; `"` toggles quoting; `2n` backslashes before a `"` become `n`
//! and the quote counts, `2n+1` become `n` and a literal `"`; `""` inside quotes is a literal `"`.
//! The program name (the first argument) is taken as-is up to its closing quote or whitespace.
use alloc::string::String;
use alloc::vec::Vec;
use windows_sys::Win32::System::Environment::GetCommandLineW;

/// Every argument after the program name, lossily decoded from UTF-16.
pub fn args() -> Vec<String> {
    let line = unsafe { GetCommandLineW() };
    let units = unsafe { crate::wide::slice_from_ptr(line) };
    let mut all = split(units);
    if !all.is_empty() {
        all.remove(0);
    }
    all.into_iter().map(|a| String::from_utf16_lossy(&a)).collect()
}

const QUOTE: u16 = b'"' as u16;
const BACKSLASH: u16 = b'\\' as u16;

fn is_space(c: u16) -> bool {
    c == b' ' as u16 || c == b'\t' as u16
}

/// Splits a whole command line, program name included (Rust std's `parse_lp_cmd_line`).
pub fn split(line: &[u16]) -> Vec<Vec<u16>> {
    let mut out = Vec::new();
    let mut i = 0;
    // The program name: no escapes, a quote just toggles.
    let mut name = Vec::new();
    let mut in_quotes = false;
    while i < line.len() {
        let c = line[i];
        if c == QUOTE {
            in_quotes = !in_quotes;
        } else if is_space(c) && !in_quotes {
            break;
        } else {
            name.push(c);
        }
        i += 1;
    }
    out.push(name);
    // Whitespace after the name, then the arguments.
    while i < line.len() && is_space(line[i]) {
        i += 1;
    }
    if i >= line.len() {
        return out;
    }
    let mut cur = Vec::new();
    let mut in_quotes = false;
    let mut backslashes = 0usize;
    while i < line.len() {
        let c = line[i];
        if c == BACKSLASH {
            backslashes += 1;
        } else if c == QUOTE {
            cur.extend(core::iter::repeat_n(BACKSLASH, backslashes / 2));
            if backslashes % 2 == 1 {
                cur.push(QUOTE);
            } else if in_quotes && line.get(i + 1) == Some(&QUOTE) {
                // `""` while quoted: a literal quote, still quoted.
                cur.push(QUOTE);
                i += 1;
            } else {
                in_quotes = !in_quotes;
            }
            backslashes = 0;
        } else if is_space(c) && !in_quotes {
            cur.extend(core::iter::repeat_n(BACKSLASH, backslashes));
            backslashes = 0;
            out.push(core::mem::take(&mut cur));
            while i + 1 < line.len() && is_space(line[i + 1]) {
                i += 1;
            }
            if i + 1 >= line.len() {
                return out;
            }
        } else {
            cur.extend(core::iter::repeat_n(BACKSLASH, backslashes));
            backslashes = 0;
            cur.push(c);
        }
        i += 1;
    }
    cur.extend(core::iter::repeat_n(BACKSLASH, backslashes));
    out.push(cur);
    out
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::split;
    use alloc::string::String;
    use alloc::vec::Vec;

    fn parse(s: &str) -> Vec<String> {
        let w: Vec<u16> = s.encode_utf16().collect();
        split(&w).into_iter().map(|a| String::from_utf16_lossy(&a)).collect()
    }

    #[test]
    fn msvcrt_rules() {
        assert_eq!(parse(r#""C:\a b\x.exe" --set device={1-2} lowBattery=15"#), ["C:\\a b\\x.exe", "--set", "device={1-2}", "lowBattery=15"]);
        assert_eq!(parse(r#"x "a b" c"#), ["x", "a b", "c"]);
        assert_eq!(parse(r#"x a\\\"b"#), ["x", "a\\\"b"]);
        assert_eq!(parse(r#"x "a\\" b"#), ["x", "a\\", "b"]);
        assert_eq!(parse(r#"x a\b c\\"#), ["x", "a\\b", "c\\\\"]);
        assert_eq!(parse(r#"x "a""b" c"#), ["x", "a\"b", "c"]);
        assert_eq!(parse("x   "), ["x"]);
        assert_eq!(parse("x \"\""), ["x", ""]);
        assert_eq!(parse("x"), ["x"]);
    }
}
