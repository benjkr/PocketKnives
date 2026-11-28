//! Themes: a theme is a flat JSON object of colours and nothing else. Every theme has every key;
//! the drawing code never knows a theme's name. Built-ins are the files in `themes/`, embedded at
//! build time (build.rs); more can be dropped into the user's themes folder.
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

include!(concat!(env!("OUT_DIR"), "/themes.rs"));

/// A colour, straight (not premultiplied) alpha, 0..=1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub const CLEAR: Rgba = Rgba { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

    pub fn with_alpha(self, a: f32) -> Rgba {
        Rgba { a: self.a * a, ..self }
    }

    pub fn lerp(self, o: Rgba, t: f32) -> Rgba {
        Rgba { r: self.r + (o.r - self.r) * t, g: self.g + (o.g - self.g) * t, b: self.b + (o.b - self.b) * t, a: self.a + (o.a - self.a) * t }
    }

    /// `#rrggbb` or `#rrggbbaa`.
    fn parse(s: &str) -> Option<Rgba> {
        let h = s.strip_prefix('#')?;
        if !(h.len() == 6 || h.len() == 8) || !h.bytes().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }
        let byte = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).ok().map(|v| v as f32 / 255.0);
        Some(Rgba { r: byte(0)?, g: byte(2)?, b: byte(4)?, a: if h.len() == 8 { byte(6)? } else { 1.0 } })
    }
}

macro_rules! palette {
    ($($(#[doc = $doc:literal])* $field:ident = $key:literal,)*) => {
        /// Every colour a theme sets, by its role in the drawing.
        #[derive(Clone, Copy, Debug)]
        pub struct Palette { $($(#[doc = $doc])* pub $field: Rgba,)* }
        /// The keys every theme file has, in the order the spec lists them.
        pub const KEYS: &[&str] = &[$($key),*];
        impl Palette {
            fn from_pairs(pairs: &[(String, Rgba)]) -> Result<Palette, String> {
                let get = |k: &str| pairs.iter().find(|p| p.0 == k).map(|p| p.1).ok_or_else(|| format!("missing \"{k}\""));
                Ok(Palette { $($field: get($key)?,)* })
            }
        }
    };
}

palette! {
    /// The panel faceplate's vertical gradient, at 0%, 22%, 55% and 100% of its height.
    plate_top = "plateTop",
    plate_upper = "plateUpper",
    plate_lower = "plateLower",
    plate_bottom = "plateBottom",
    /// The panel faceplate's stipple: a light dot and a dark dot in every 2x2 pixels.
    plate_dot_light = "plateDotLight",
    plate_dot_dark = "plateDotDark",
    /// The panel faceplate's top and bottom edge lines (and parting line).
    plate_edge_top = "plateEdgeTop",
    plate_edge_bottom = "plateEdgeBottom",
    /// The tint behind the tile under the pointer.
    hover = "hover",
    /// The surround the screens sit in: its gradient, and the lips half a pixel above and below it.
    surround_top = "surroundTop",
    surround_bottom = "surroundBottom",
    surround_lip_above = "surroundLipAbove",
    surround_lip_below = "surroundLipBelow",
    /// The glass: its gradient, the shadow under its top edges, its inner edge line, and a glow below it.
    glass_top = "glassTop",
    glass_bottom = "glassBottom",
    glass_shadow = "glassShadow",
    glass_edge = "glassEdge",
    glass_glow = "glassGlow",
    /// The dot matrix: the lines between its pixels and the tint of an unlit pixel.
    grid_line = "gridLine",
    grid_tint = "gridTint",
    /// Lit pixels and segments, unlit (ghost) segments, and the shadow lit ink casts on the glass.
    ink = "ink",
    ink_ghost = "inkGhost",
    ink_shadow = "inkShadow",
    /// The body of a bar or filled area under its lit top row; hot pixels; the dotted dividers.
    fill = "fill",
    hot = "hot",
    divider = "divider",
    /// The network graph's download, so it reads apart from the upload (in `ink`): its outer row,
    /// and the area between that and the axis.
    net_down = "netDown",
    net_down_fill = "netDownFill",
    /// The metric icons on the bar: the icon, when hot, and its shadow one pixel above.
    icon = "icon",
    icon_hot = "iconHot",
    icon_shadow = "iconShadow",
    /// Engraved labels and ticks on the panel's faceplate: the groove, and the lips one pixel above and below.
    engrave = "engrave",
    engrave_lip_above = "engraveLipAbove",
    engrave_lip_below = "engraveLipBelow",
    /// The hover panel's shadow, the ring round its screen, and the highlight under that ring.
    panel_shadow = "panelShadow",
    panel_ring = "panelRing",
    panel_ring_lip = "panelRingLip",
    /// The keys: the cap's gradient, its side wall, the lit top row of the cap, the shadow it casts on the plate, its legend.
    key_top = "keyTop",
    key_bottom = "keyBottom",
    key_side = "keySide",
    key_edge = "keyEdge",
    key_shadow = "keyShadow",
    key_legend = "keyLegend",
    /// The list window's cover glass: the black printed on its back round the LCD and over the
    /// colour screen, the reflection across it, the lit top edge.
    cover_mask = "coverMask",
    cover_glare = "coverGlare",
    cover_edge = "coverEdge",
    /// The colour screen: a selected row's lit cell, the icon of a program with none, the padlock
    /// of a program that couldn't be ended.
    app_sel = "appSel",
    app_none = "appNone",
    app_deny = "appDeny",
}

/// Parses a theme file: one flat JSON object whose values are all colour strings, with every key
/// in [`KEYS`] and no others.
pub fn parse(text: &str) -> Result<Palette, String> {
    let pairs = parse_object(text)?;
    if let Some(p) = pairs.iter().find(|p| !KEYS.contains(&p.0.as_str())) {
        return Err(format!("unknown key \"{}\"", p.0));
    }
    Palette::from_pairs(&pairs)
}

/// The theme called `name`: a file in the user's themes folder first, then a built-in.
pub fn load(name: &str) -> Result<Palette, String> {
    if let Some(dir) = crate::settings::themes_dir()
        && let Some(bytes) = crate::settings::read_file(&format!("{dir}\\{name}.json"))
    {
        let text = core::str::from_utf8(&bytes).map_err(|_| format!("{name}.json is not UTF-8"))?;
        return parse(text).map_err(|e| format!("{name}.json: {e}"));
    }
    BUILT_IN.iter().find(|t| t.0 == name).map(|t| parse(t.1)).unwrap_or_else(|| Err(format!("no theme called \"{name}\"")))
}

/// Every theme's name: the built-ins and the user's files, sorted, without repeats.
pub fn names() -> Vec<String> {
    let mut v: Vec<String> = BUILT_IN.iter().map(|t| String::from(t.0)).collect();
    if let Some(dir) = crate::settings::themes_dir() {
        v.extend(crate::settings::list_files(&dir, ".json"));
    }
    v.sort();
    v.dedup();
    v
}

fn parse_object(text: &str) -> Result<Vec<(String, Rgba)>, String> {
    let mut p = Parser { s: text.trim_start_matches('\u{feff}').as_bytes(), i: 0 };
    let mut out = Vec::new();
    p.expect(b'{')?;
    if p.peek() == Some(b'}') {
        p.i += 1;
    } else {
        loop {
            let key = p.string()?;
            p.expect(b':')?;
            let at = p.i;
            let value = p.string()?;
            let colour = Rgba::parse(&value).ok_or_else(|| format!("\"{key}\" at byte {at}: \"{value}\" is not #rrggbb or #rrggbbaa"))?;
            if out.iter().any(|q: &(String, Rgba)| q.0 == key) {
                return Err(format!("\"{key}\" is set twice"));
            }
            out.push((key, colour));
            match p.next() {
                Some(b',') => continue,
                Some(b'}') => break,
                _ => return Err(format!("expected , or }} at byte {}", p.i)),
            }
        }
    }
    if p.peek().is_some() {
        return Err(format!("unexpected text after the object at byte {}", p.i));
    }
    Ok(out)
}

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
}

impl Parser<'_> {
    fn skip(&mut self) {
        while self.i < self.s.len() && matches!(self.s[self.i], b' ' | b'\t' | b'\r' | b'\n') {
            self.i += 1;
        }
    }
    fn peek(&mut self) -> Option<u8> {
        self.skip();
        self.s.get(self.i).copied()
    }
    fn next(&mut self) -> Option<u8> {
        let c = self.peek()?;
        self.i += 1;
        Some(c)
    }
    fn expect(&mut self, c: u8) -> Result<(), String> {
        if self.next() == Some(c) { Ok(()) } else { Err(format!("expected '{}' at byte {}", c as char, self.i)) }
    }
    /// A JSON string with no escapes (keys and colours never need any).
    fn string(&mut self) -> Result<String, String> {
        self.expect(b'"')?;
        let start = self.i;
        while self.i < self.s.len() && self.s[self.i] != b'"' {
            if self.s[self.i] == b'\\' || self.s[self.i] < 0x20 {
                return Err(format!("unexpected character in a string at byte {}", self.i));
            }
            self.i += 1;
        }
        let text = core::str::from_utf8(&self.s[start..self.i]).map_err(|_| String::from("not UTF-8"))?;
        if self.i == self.s.len() {
            return Err(String::from("unterminated string"));
        }
        self.i += 1;
        Ok(String::from(text))
    }
}
