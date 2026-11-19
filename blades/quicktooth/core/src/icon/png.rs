//! A minimal PNG writer for the check images (RGBA, stored deflate blocks), and ICO files made
//! of such PNGs for PocketKnives Settings.
use alloc::vec::Vec;

const CRC: [u32; 256] = {
    let mut t = [0u32; 256];
    let mut n = 0;
    while n < 256 {
        let mut c = n as u32;
        let mut k = 0;
        while k < 8 {
            c = if c & 1 != 0 { 0xEDB8_8320 ^ (c >> 1) } else { c >> 1 };
            k += 1;
        }
        t[n] = c;
        n += 1;
    }
    t
};

/// A PNG of straight-alpha BGRA pixels (RGBA, stored deflate blocks), byte for byte what the
/// C# writer produced.
pub fn encode(bgra: &[u8], width: usize, height: usize) -> Vec<u8> {
    let (w, h) = (width, height);
    let mut raw = alloc::vec![0u8; h * (1 + w * 4)];
    for y in 0..h {
        let o = y * (1 + w * 4); // filter byte 0 (none)
        for x in 0..w {
            let s = (y * w + x) * 4;
            let d = o + 1 + x * 4;
            raw[d] = bgra[s + 2];
            raw[d + 1] = bgra[s + 1];
            raw[d + 2] = bgra[s];
            raw[d + 3] = bgra[s + 3];
        }
    }

    let mut z = Vec::with_capacity(raw.len() + raw.len() / 65535 * 5 + 11);
    z.extend_from_slice(&[0x78, 0x01]);
    let mut pos = 0;
    loop {
        let len = 65535.min(raw.len() - pos);
        z.push(if pos + len >= raw.len() { 1 } else { 0 });
        z.extend_from_slice(&[len as u8, (len >> 8) as u8, !len as u8, (!len >> 8) as u8]);
        z.extend_from_slice(&raw[pos..pos + len]);
        pos += 65535;
        if raw.is_empty() || pos >= raw.len() {
            break;
        }
    }
    let (mut a, mut b) = (1u32, 0u32);
    for &v in &raw {
        a = (a + v as u32) % 65521;
        b = (b + a) % 65521;
    }
    z.extend_from_slice(&((b << 16) | a).to_be_bytes());

    let mut f = Vec::with_capacity(z.len() + 57);
    f.extend_from_slice(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&(w as u32).to_be_bytes());
    ihdr.extend_from_slice(&(h as u32).to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]); // 8-bit RGBA, deflate, no filter, no interlace
    chunk(&mut f, b"IHDR", &ihdr);
    chunk(&mut f, b"IDAT", &z);
    chunk(&mut f, b"IEND", &[]);
    f
}

fn chunk(f: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    f.extend_from_slice(&(data.len() as u32).to_be_bytes());
    let start = f.len();
    f.extend_from_slice(kind);
    f.extend_from_slice(data);
    let mut c = 0xFFFF_FFFFu32;
    for &v in &f[start..] {
        c = CRC[((c ^ v as u32) & 0xFF) as usize] ^ (c >> 8);
    }
    f.extend_from_slice(&(c ^ 0xFFFF_FFFF).to_be_bytes());
}

/// An ICO file of square straight-alpha BGRA images (`(size, pixels)`, at most 256 px each),
/// each stored as a PNG.
pub fn ico(images: &[(usize, Vec<u8>)]) -> Vec<u8> {
    let pngs: Vec<Vec<u8>> = images.iter().map(|(size, px)| encode(px, *size, *size)).collect();
    let mut out = Vec::new();
    out.extend_from_slice(&[0, 0, 1, 0]); // reserved, type 1 (icon)
    out.extend_from_slice(&(images.len() as u16).to_le_bytes());
    let mut offset = 6 + 16 * images.len();
    for ((size, _), png) in images.iter().zip(&pngs) {
        out.push(*size as u8); // 256 is written as 0
        out.push(*size as u8);
        out.extend_from_slice(&[0, 0, 1, 0, 32, 0]); // no palette, reserved, 1 plane, 32 bits
        out.extend_from_slice(&(png.len() as u32).to_le_bytes());
        out.extend_from_slice(&(offset as u32).to_le_bytes());
        offset += png.len();
    }
    for png in &pngs {
        out.extend_from_slice(png);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_pixel_layout() {
        // One red, half-transparent pixel (BGRA in, RGBA out).
        let png = encode(&[0, 0, 255, 128], 1, 1);
        let want: &[u8] = &[
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // signature
            0, 0, 0, 13, b'I', b'H', b'D', b'R', 0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0, 0, 0, 0x1F, 0x15, 0xC4, 0x89, // IHDR
            0, 0, 0, 16, b'I', b'D', b'A', b'T', 0x78, 0x01, 1, 5, 0, 0xFA, 0xFF, 0, 255, 0, 0, 128, // stored block
        ];
        assert_eq!(&png[..want.len()], want);
        // Adler-32 of [0, 255, 0, 0, 128]: a = 1 + 383 = 384, b = 1 + 256 + 256 + 256 + 384 = 1153.
        assert_eq!(&png[want.len()..want.len() + 4], &[0x04, 0x81, 0x01, 0x80]);
        // The IEND chunk is always the same 12 bytes.
        assert_eq!(&png[png.len() - 12..], &[0, 0, 0, 0, b'I', b'E', b'N', b'D', 0xAE, 0x42, 0x60, 0x82]);
    }

    #[test]
    fn splits_into_65535_byte_blocks() {
        // 128 × 128 RGBA rows: 128 × 513 = 65664 raw bytes, so two stored blocks.
        let png = encode(&alloc::vec![7u8; 128 * 128 * 4], 128, 128);
        let idat = 33 + 8;
        assert_eq!(&png[idat..idat + 2], &[0x78, 0x01]);
        assert_eq!(&png[idat + 2..idat + 7], &[0, 0xFF, 0xFF, 0, 0]);
        let second = idat + 7 + 65535;
        assert_eq!(&png[second..second + 5], &[1, 129, 0, 0x7E, 0xFF]);
    }
}
