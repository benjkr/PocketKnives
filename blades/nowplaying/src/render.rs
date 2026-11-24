//! Direct2D + DirectWrite + WIC renderer for NowPlaying (C1 Pure Clear + P1 + White M2 Spectrum).
//! Draws into a 32-bit premultiplied BGRA DIB for `UpdateLayeredWindow`, and also supports
//! offline PNG export (`--preview-render`) through WIC.
use crate::app;
use crate::fft::BANDS;
use crate::smtc::MediaSnapshot;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;
use core::ptr::{null, null_mut};
use pk::wide::to_wide;
use windows_numerics::{Matrix3x2, Vector2};
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Direct2D::Common::{
    D2D_RECT_F, D2D_SIZE_U, D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_COLOR_F, D2D1_GRADIENT_STOP, D2D1_PIXEL_FORMAT,
};
use windows::Win32::Graphics::Direct2D::{
    D2D1_BITMAP_INTERPOLATION_MODE_LINEAR, D2D1_BITMAP_PROPERTIES, D2D1_DRAW_TEXT_OPTIONS_CLIP, D2D1_DRAW_TEXT_OPTIONS_NONE,
    D2D1_ELLIPSE, D2D1_EXTEND_MODE_CLAMP, D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1_GAMMA_2_2,
    D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, D2D1_RENDER_TARGET_PROPERTIES, D2D1_RENDER_TARGET_TYPE_DEFAULT, D2D1_ROUNDED_RECT,
    D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE, D2D1CreateFactory, ID2D1Bitmap, ID2D1DCRenderTarget, ID2D1Factory, ID2D1PathGeometry,
    ID2D1RoundedRectangleGeometry,
};
use windows::Win32::Graphics::DirectWrite::{
    DWRITE_FACTORY_TYPE_SHARED, DWRITE_FONT_FEATURE, DWRITE_FONT_FEATURE_TAG_TABULAR_FIGURES, DWRITE_FONT_STRETCH_NORMAL,
    DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_WEIGHT_BOLD, DWRITE_FONT_WEIGHT_MEDIUM, DWRITE_FONT_WEIGHT_SEMI_BOLD,
    DWRITE_PARAGRAPH_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_LEADING, DWRITE_TEXT_ALIGNMENT_TRAILING, DWRITE_TEXT_METRICS,
    DWRITE_TEXT_RANGE, DWRITE_TRIMMING, DWRITE_TRIMMING_GRANULARITY_CHARACTER, DWRITE_WORD_WRAPPING_NO_WRAP, DWriteCreateFactory,
    IDWriteFactory, IDWriteTextFormat, IDWriteTextLayout,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::Graphics::Gdi::HDC as WinHdc;
use windows::Win32::Graphics::Imaging::{
    CLSID_WICImagingFactory, GUID_ContainerFormatPng, GUID_WICPixelFormat32bppPBGRA, IWICImagingFactory,
    IWICBitmapSource, WICBitmapDitherTypeNone, WICBitmapUseAlpha, WICBitmapInterpolationModeFant, WICBitmapPaletteTypeCustom, WICDecodeMetadataCacheOnDemand,
};
use windows::Win32::System::Com::{CLSCTX_INPROC_SERVER, CoCreateInstance};
use windows_core::{GUID, Interface, PCWSTR};
use windows_sys::Win32::Foundation::{GENERIC_WRITE, HWND, POINT, SIZE};
use windows_sys::Win32::Graphics::Gdi::{
    BI_RGB, BITMAPINFO, BITMAPINFOHEADER, CreateCompatibleDC, CreateDIBSection, DIB_RGB_COLORS, DeleteDC, DeleteObject, HBITMAP, HDC,
    HGDIOBJ, SelectObject,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{ULW_ALPHA, UpdateLayeredWindow};

pub const STRIP_W: f32 = 344.0;
pub const STRIP_H: f32 = 40.0;
pub const DEFAULT_TASKBAR_H: f32 = 48.0;
pub const COVER_SIZE: f32 = 40.0;
pub const CORNER_R: f32 = 4.0;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct WindowMetrics {
    pub win_w: i32,
    pub win_h: i32,
    pub pad: i32,
}

/// Computes the layered window size (flush to taskbar left/top/bottom) and the equal left/top
/// padding that keeps the 344×40 strip content centered at the same taskbar position.
pub fn window_metrics(dpi: u32, taskbar_h_px: i32) -> WindowMetrics {
    let scale = (dpi.max(96) as f32) / 96.0;
    let strip_w = (STRIP_W * scale + 0.5) as i32;
    let strip_h = (STRIP_H * scale + 0.5) as i32;
    let win_h = taskbar_h_px.max(strip_h);
    let pad = ((win_h - strip_h) / 2).max(0);
    WindowMetrics { win_w: pad + strip_w, win_h, pad }
}
/// The app icon before the title, in DIPs.
const APP_ICON_SIZE: f32 = 13.0;
const APP_ICON_LEFT: f32 = 48.0;
const TITLE_LEFT: f32 = 66.0;
/// Row 1's controls (the app icon and title, the transport buttons) span these DIPs.
const ROW1_TOP: f32 = 3.0;
const ROW1_BOTTOM: f32 = 21.0;
/// Scrubber accent when neither the cover nor the app icon gives one.
const DEFAULT_ACCENT: D2D1_COLOR_F = D2D1_COLOR_F { r: 1.0, g: 107.0 / 255.0, b: 139.0 / 255.0, a: 1.0 };

// Hit regions in logical DIPs (relative to 344x40 strip)
pub const PREV_RECT: (f32, f32, f32, f32) = (277.0, 3.0, 296.0, 21.0);
pub const PLAY_RECT: (f32, f32, f32, f32) = (297.0, 3.0, 316.0, 21.0);
pub const NEXT_RECT: (f32, f32, f32, f32) = (317.0, 3.0, 336.0, 21.0);
pub const EQ_RECT: (f32, f32, f32, f32) = (253.0, 4.0, 273.0, 20.0);
const SCRUB_LEFT: f32 = 48.0;
const SCRUB_TOP: f32 = 24.0;
const SCRUB_BOTTOM: f32 = 38.0;
const TIME_RIGHT: f32 = STRIP_W - 8.0;
const TIME_GAP: f32 = 8.0;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HoverTarget {
    None,
    Prev,
    Play,
    Next,
    Scrub,
    /// The app icon and title: a click brings the playing app to the front.
    App,
}

pub struct FrameState<'a> {
    pub media: &'a MediaSnapshot,
    pub light_taskbar: bool,
    pub show_visualizer: bool,
    pub bands: [f32; BANDS],
    pub strip_hover_alpha: f32,
    pub btn_hover_alpha: [f32; 4], // prev, play, next, app
    pub btn_pressed: Option<usize>,
    pub scrub_active_alpha: f32,
    pub scrub_hover: bool,
    pub scrub_dragging: bool,
    pub scrub_hover_ratio: f32,
    pub drag_pos_sec: f64,
}

pub struct Renderer {
    d2d: ID2D1Factory,
    rt: ID2D1DCRenderTarget,
    dwrite: IDWriteFactory,
    wic: IWICImagingFactory,
    fmt_title: IDWriteTextFormat,
    fmt_time: IDWriteTextFormat,
    cover_clip: ID2D1RoundedRectangleGeometry,
    mem_dc: HDC,
    dib: HBITMAP,
    old_obj: HGDIOBJ,
    dib_bits: *mut u8,
    px_w: i32,
    px_h: i32,
    pad_px: i32,
    win_w_dip: f32,
    win_h_dip: f32,
    pad_dip: f32,
    dpi: f32,
    art_version: u64,
    art_bmp: Option<ID2D1Bitmap>,
    accent: D2D1_COLOR_F,
    /// `aumid` and DPI the app icon was loaded for, the icon, and its accent for coverless tracks.
    icon_key: (alloc::string::String, u32),
    icon_bmp: Option<ID2D1Bitmap>,
    icon_accent: Option<D2D1_COLOR_F>,
    /// Cached `(is_live, cur_min_digits, dur_min_digits)` and measured width of the worst-case time label.
    time_slot_key: (bool, u32, u32),
    time_slot_w: f32,
}

impl Renderer {
    pub fn new(dpi: u32) -> Option<Self> {
        let d2d: ID2D1Factory = unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None) }.ok()?;
        let rt_props = D2D1_RENDER_TARGET_PROPERTIES {
            r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
            pixelFormat: D2D1_PIXEL_FORMAT {
                format: DXGI_FORMAT_B8G8R8A8_UNORM,
                alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
            },
            dpiX: 96.0,
            dpiY: 96.0,
            ..Default::default()
        };
        let rt = unsafe { d2d.CreateDCRenderTarget(&rt_props) }.ok()?;
        // Grayscale AA is mandatory for clean per-pixel alpha in UpdateLayeredWindow!
        unsafe { rt.SetTextAntialiasMode(D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE) };

        let dwrite: IDWriteFactory = unsafe { DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED) }.ok()?;
        let wic: IWICImagingFactory = unsafe { CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER) }.ok()?;

        let font = to_wide("Segoe UI Variable Text");
        let locale = to_wide("en-us");
        let fmt_title = unsafe {
            dwrite.CreateTextFormat(
                PCWSTR(font.as_ptr()),
                None,
                DWRITE_FONT_WEIGHT_SEMI_BOLD,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                11.5,
                PCWSTR(locale.as_ptr()),
            )
        }
        .ok()?;
        let _ = unsafe { fmt_title.SetWordWrapping(DWRITE_WORD_WRAPPING_NO_WRAP) };
        let _ = unsafe { fmt_title.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER) };
        let _ = unsafe { fmt_title.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_LEADING) };
        if let Ok(sign) = unsafe { dwrite.CreateEllipsisTrimmingSign(&fmt_title) } {
            let trim = DWRITE_TRIMMING {
                granularity: DWRITE_TRIMMING_GRANULARITY_CHARACTER,
                delimiter: 0,
                delimiterCount: 0,
            };
            let _ = unsafe { fmt_title.SetTrimming(&trim, &sign) };
        }

        let fmt_time = unsafe {
            dwrite.CreateTextFormat(
                PCWSTR(font.as_ptr()),
                None,
                DWRITE_FONT_WEIGHT_MEDIUM,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                9.5,
                PCWSTR(locale.as_ptr()),
            )
        }
        .ok()?;
        let _ = unsafe { fmt_time.SetWordWrapping(DWRITE_WORD_WRAPPING_NO_WRAP) };
        let _ = unsafe { fmt_time.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER) };
        let _ = unsafe { fmt_time.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_TRAILING) };

        let cover_clip = unsafe {
            d2d.CreateRoundedRectangleGeometry(&D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F { left: 0.0, top: 0.0, right: COVER_SIZE, bottom: COVER_SIZE },
                radiusX: CORNER_R,
                radiusY: CORNER_R,
            })
        }
        .ok()?;

        let mut r = Self {
            d2d,
            rt,
            dwrite,
            wic,
            fmt_title,
            fmt_time,
            cover_clip,
            mem_dc: null_mut(),
            dib: null_mut(),
            old_obj: null_mut(),
            dib_bits: null_mut(),
            px_w: 0,
            px_h: 0,
            pad_px: 0,
            win_w_dip: STRIP_W,
            win_h_dip: STRIP_H,
            pad_dip: 0.0,
            dpi: 96.0,
            art_version: u64::MAX,
            art_bmp: None,
            accent: DEFAULT_ACCENT,
            icon_key: (alloc::string::String::new(), 0),
            icon_bmp: None,
            icon_accent: None,
            time_slot_key: (false, 0, 0),
            time_slot_w: 54.0,
        };
        let scale = (dpi.max(96) as f32) / 96.0;
        let default_tb_h = (DEFAULT_TASKBAR_H * scale + 0.5) as i32;
        r.set_layout(dpi, window_metrics(dpi, default_tb_h));
        Some(r)
    }

    pub fn set_layout(&mut self, dpi: u32, m: WindowMetrics) {
        let dpi_f = (dpi.max(96)) as f32;
        let scale = dpi_f / 96.0;
        let dpi_changed = (dpi_f - self.dpi).abs() >= 0.1;
        let size_changed = m.win_w != self.px_w || m.win_h != self.px_h;
        self.pad_px = m.pad;
        self.pad_dip = (m.pad as f32) / scale;
        self.win_w_dip = (m.win_w as f32) / scale;
        self.win_h_dip = (m.win_h as f32) / scale;
        if !size_changed && !dpi_changed {
            return;
        }
        self.dpi = dpi_f;
        if size_changed {
            self.recreate_dib(m.win_w, m.win_h);
        }
        unsafe { self.rt.SetDpi(dpi_f, dpi_f) };
        if dpi_changed {
            // Force cover re-decode at the new DPI pixel size
            self.art_version = u64::MAX;
            self.time_slot_key = (false, 0, 0);
        }
    }

    /// Logical DIP bounds of the progress bar for `media`, leaving the exact room needed by the
    /// track's widest bold `cur / dur` readout so timestamps never clip and scrubbing never jumps.
    pub fn scrub_rect(&mut self, media: &MediaSnapshot) -> (f32, f32, f32, f32) {
        let dur = media.duration_sec();
        let is_live = dur <= 0.0 && media.active && media.playing;
        let cur = media.live_position_sec();
        let time_w = self.time_slot_width(is_live, dur, cur);
        let right = (TIME_RIGHT - time_w - TIME_GAP).max(SCRUB_LEFT + 40.0);
        (SCRUB_LEFT, SCRUB_TOP, right, SCRUB_BOTTOM)
    }

    /// The app icon and the drawn `Title · Artist` text, with the transport buttons' plate margin.
    pub fn app_rect(&self, media: &MediaSnapshot, show_visualizer: bool) -> (f32, f32, f32, f32) {
        let mut metrics = DWRITE_TEXT_METRICS::default();
        let w = match self.title_layout(media, show_visualizer) {
            Some((layout, _)) if unsafe { layout.GetMetrics(&mut metrics) }.is_ok() => metrics.width,
            _ => 0.0,
        };
        (APP_ICON_LEFT - 3.0, ROW1_TOP, TITLE_LEFT + w.min(title_max_w(show_visualizer)) + 3.0, ROW1_BOTTOM)
    }

    /// `Title · Artist` laid out to its trimmed width, with the dot's and artist's ranges.
    fn title_layout(
        &self,
        media: &MediaSnapshot,
        show_visualizer: bool,
    ) -> Option<(IDWriteTextLayout, Option<(DWRITE_TEXT_RANGE, DWRITE_TEXT_RANGE)>)> {
        let title = if media.title.is_empty() { "Nothing playing" } else { media.title.as_str() };
        let has_artist = !media.artist.is_empty();
        let full = if has_artist { format!("{title} · {}", media.artist) } else { alloc::string::String::from(title) };
        let wide: Vec<u16> = full.encode_utf16().collect();
        let layout =
            unsafe { self.dwrite.CreateTextLayout(&wide, &self.fmt_title, title_max_w(show_visualizer), ROW1_BOTTOM - ROW1_TOP) }
                .ok()?;
        if !has_artist {
            return Some((layout, None));
        }
        let title_len = title.encode_utf16().count() as u32;
        let dot = DWRITE_TEXT_RANGE { startPosition: title_len, length: 3 };
        let artist = DWRITE_TEXT_RANGE { startPosition: title_len + 3, length: (wide.len() as u32).saturating_sub(title_len + 3) };
        let _ = unsafe { layout.SetFontWeight(DWRITE_FONT_WEIGHT_MEDIUM, artist) };
        let _ = unsafe { layout.SetFontSize(10.5, artist) };
        Some((layout, Some((dot, artist))))
    }

    fn time_slot_width(&mut self, is_live: bool, dur: f64, cur: f64) -> f32 {
        let key = if is_live {
            (true, 0, 0)
        } else {
            (false, minute_digits(dur.max(cur)), minute_digits(dur))
        };
        if key == self.time_slot_key {
            return self.time_slot_w;
        }

        let (sample, bold_len) = if is_live {
            (alloc::string::String::from("LIVE"), 4u32)
        } else {
            let mut s = alloc::string::String::with_capacity((key.1 + key.2 + 9) as usize);
            for _ in 0..key.1 {
                s.push('8');
            }
            s.push_str(":88");
            let bold_len = s.len() as u32;
            s.push_str(" / ");
            for _ in 0..key.2 {
                s.push('8');
            }
            s.push_str(":88");
            (s, bold_len)
        };
        let wide: Vec<u16> = sample.encode_utf16().collect();
        let measured = (|| -> Option<f32> {
            let layout = unsafe { self.dwrite.CreateTextLayout(&wide, &self.fmt_time, 200.0, 14.0) }.ok()?;
            if let Ok(typo) = unsafe { self.dwrite.CreateTypography() } {
                let _ = unsafe {
                    typo.AddFontFeature(DWRITE_FONT_FEATURE { nameTag: DWRITE_FONT_FEATURE_TAG_TABULAR_FIGURES, parameter: 1 })
                };
                let _ = unsafe { layout.SetTypography(&typo, DWRITE_TEXT_RANGE { startPosition: 0, length: wide.len() as u32 }) };
            }
            let _ = unsafe { layout.SetFontWeight(DWRITE_FONT_WEIGHT_BOLD, DWRITE_TEXT_RANGE { startPosition: 0, length: bold_len }) };
            let mut metrics = DWRITE_TEXT_METRICS::default();
            unsafe { layout.GetMetrics(&mut metrics) }.ok()?;
            let w = metrics.widthIncludingTrailingWhitespace.max(metrics.width);
            Some((libm::ceilf(w) + 3.0).clamp(28.0, 120.0))
        })();

        self.time_slot_key = key;
        self.time_slot_w = measured.unwrap_or(62.0);
        self.time_slot_w
    }

    fn recreate_dib(&mut self, w: i32, h: i32) {
        self.free_dib();
        let dc = unsafe { CreateCompatibleDC(null_mut()) };
        if dc.is_null() {
            return;
        }
        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: core::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: w,
                biHeight: -h, // top-down DIB
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB,
                ..unsafe { core::mem::zeroed() }
            },
            bmiColors: [unsafe { core::mem::zeroed() }],
        };
        let mut bits = null_mut();
        let bmp = unsafe { CreateDIBSection(dc, &bmi, DIB_RGB_COLORS, &mut bits, null_mut(), 0) };
        if bmp.is_null() {
            unsafe { DeleteDC(dc) };
            return;
        }
        let old = unsafe { SelectObject(dc, bmp as HGDIOBJ) };
        self.mem_dc = dc;
        self.dib = bmp;
        self.old_obj = old;
        self.dib_bits = bits as *mut u8;
        self.px_w = w;
        self.px_h = h;
    }

    fn free_dib(&mut self) {
        if !self.mem_dc.is_null() {
            if !self.old_obj.is_null() {
                unsafe { SelectObject(self.mem_dc, self.old_obj) };
                self.old_obj = null_mut();
            }
            if !self.dib.is_null() {
                unsafe { DeleteObject(self.dib as HGDIOBJ) };
                self.dib = null_mut();
            }
            unsafe { DeleteDC(self.mem_dc) };
            self.mem_dc = null_mut();
            self.dib_bits = null_mut();
        }
    }

    fn sync_cover(&mut self, media: &MediaSnapshot) {
        self.sync_app_icon(&media.aumid);
        if media.art_version == self.art_version {
            return;
        }
        self.art_version = media.art_version;
        self.art_bmp = None;
        self.accent = self.icon_accent.unwrap_or(DEFAULT_ACCENT);

        if media.thumbnail.is_empty() {
            return;
        }
        let target_px = ((COVER_SIZE * (self.dpi / 96.0)) + 0.5) as u32;
        let Some(frame) = decode_image(&self.wic, &media.thumbnail) else { return };
        if let Some((bmp, accent)) = self.d2d_bitmap(&frame, target_px.max(40)) {
            self.art_bmp = Some(bmp);
            self.accent = accent;
        }
    }

    /// Loads the playing app's own icon from the shell once per app and DPI.
    fn sync_app_icon(&mut self, aumid: &str) {
        let dpi = self.dpi as u32;
        if self.icon_key.0 == aumid && self.icon_key.1 == dpi {
            return;
        }
        self.icon_key = (alloc::string::String::from(aumid), dpi);
        self.icon_bmp = None;
        self.icon_accent = None;
        let px = ((APP_ICON_SIZE * (self.dpi / 96.0)) + 0.5) as u32;
        let Some(hbmp) = app::bitmap(aumid, px) else { return };
        let source = unsafe { self.wic.CreateBitmapFromHBITMAP(hbmp, Default::default(), WICBitmapUseAlpha) };
        unsafe { DeleteObject(hbmp.0 as HGDIOBJ) };
        if let Some((bmp, accent)) = source.ok().and_then(|s| self.d2d_bitmap(&s.cast().ok()?, px)) {
            self.icon_bmp = Some(bmp);
            self.icon_accent = Some(accent);
        }
    }

    /// Scales a WIC image to `size`×`size` and uploads it, with the accent picked from its pixels.
    fn d2d_bitmap(&self, source: &IWICBitmapSource, size: u32) -> Option<(ID2D1Bitmap, D2D1_COLOR_F)> {
        let (bgra, accent) = scale_to_pbgra(&self.wic, source, size)?;
        let props = D2D1_BITMAP_PROPERTIES {
            pixelFormat: D2D1_PIXEL_FORMAT {
                format: DXGI_FORMAT_B8G8R8A8_UNORM,
                alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
            },
            dpiX: self.dpi,
            dpiY: self.dpi,
        };
        let size_u = D2D_SIZE_U { width: size, height: size };
        let bmp = unsafe { self.rt.CreateBitmap(size_u, Some(bgra.as_ptr().cast()), size * 4, &props) }.ok()?;
        Some((bmp, accent))
    }

    /// Renders a frame and presents it atomically to the layered window at `(screen_x, screen_y)`.
    pub fn render_to_window(&mut self, hwnd: HWND, screen_x: i32, screen_y: i32, state: &FrameState) {
        if self.mem_dc.is_null() {
            return;
        }
        self.paint_internal(state);

        let dst_pt = POINT { x: screen_x, y: screen_y };
        let size = SIZE { cx: self.px_w, cy: self.px_h };
        let src_pt = POINT { x: 0, y: 0 };
        let blend = windows_sys::Win32::Graphics::Gdi::BLENDFUNCTION {
            BlendOp: windows_sys::Win32::Graphics::Gdi::AC_SRC_OVER as u8,
            BlendFlags: 0,
            SourceConstantAlpha: 255,
            AlphaFormat: windows_sys::Win32::Graphics::Gdi::AC_SRC_ALPHA as u8,
        };
        unsafe {
            UpdateLayeredWindow(hwnd, null_mut(), &dst_pt, &size, self.mem_dc, &src_pt, 0, &blend, ULW_ALPHA);
        }
    }

    /// Renders a frame over a dark Windows 11 taskbar swatch and saves it as a PNG (`--preview-render`).
    pub fn save_preview_png(&mut self, path: &str, state: &FrameState) -> Result<(), alloc::string::String> {
        if self.mem_dc.is_null() {
            return Err(alloc::string::String::from("DIB surface not initialized."));
        }
        self.paint_internal(state);

        // Composite onto a realistic 48-DIP dark Win11 taskbar strip (DIB already spans full
        // taskbar height and left edge; add a small right margin to show the taskbar edge).
        let right_extra = self.pad_px.max(0) as usize;
        let out_w = self.px_w as usize + right_extra;
        let out_h = self.px_h as usize;
        let mut out = vec![0u8; out_w * out_h * 4];

        for y in 0..out_h {
            let is_top_hairline = y == 0;
            for x in 0..out_w {
                let idx = (y * out_w + x) * 4;
                if is_top_hairline {
                    out[idx] = 69;
                    out[idx + 1] = 66;
                    out[idx + 2] = 68;
                    out[idx + 3] = 255;
                } else {
                    out[idx] = 38;
                    out[idx + 1] = 32;
                    out[idx + 2] = 36;
                    out[idx + 3] = 255;
                }
            }
        }

        let src = unsafe { core::slice::from_raw_parts(self.dib_bits, (self.px_w * self.px_h * 4) as usize) };
        for y in 0..(self.px_h as usize) {
            for x in 0..(self.px_w as usize) {
                let si = (y * (self.px_w as usize) + x) * 4;
                let di = (y * out_w + x) * 4;
                let sa = src[si + 3] as u32;
                let inv = 255 - sa;
                // Source is premultiplied BGRA
                out[di] = (src[si] as u32 + (out[di] as u32 * inv) / 255).min(255) as u8;
                out[di + 1] = (src[si + 1] as u32 + (out[di + 1] as u32 * inv) / 255).min(255) as u8;
                out[di + 2] = (src[si + 2] as u32 + (out[di + 2] as u32 * inv) / 255).min(255) as u8;
                out[di + 3] = 255;
            }
        }

        write_png_wic(&self.wic, path, &out, out_w as u32, out_h as u32)
    }

    fn paint_internal(&mut self, state: &FrameState) {
        self.sync_cover(state.media);

        let rect = RECT { left: 0, top: 0, right: self.px_w, bottom: self.px_h };
        if unsafe { self.rt.BindDC(WinHdc(self.mem_dc), &rect) }.is_err() {
            return;
        }

        // Win32 UpdateLayeredWindow passes mouse events through alpha == 0 pixels. Keep a
        // visually transparent 1/255 alpha floor so the full taskbar-flush hover area catches
        // pointer entry even at rest.
        let hit_clear = if state.light_taskbar {
            D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 / 255.0 }
        } else {
            D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 1.0 / 255.0 }
        };

        unsafe {
            self.rt.BeginDraw();
            self.rt.SetTransform(&Matrix3x2::identity());
            self.rt.Clear(Some(&hit_clear));
        }

        // 1. Native Windows 11 Fluent animated hover plate (flush to taskbar edges, no padding)
        if state.strip_hover_alpha > 0.005 {
            self.draw_win11_hover_plate(state.light_taskbar, state.strip_hover_alpha);
        }

        // Shift content by the taskbar padding so the actual layout stays in the exact same place
        unsafe {
            self.rt.SetTransform(&Matrix3x2::translation(self.pad_dip, self.pad_dip));
        }

        // 2. Full-size 40x40 flush album cover (or fallback synthwave/vinyl art)
        self.draw_cover(state.light_taskbar);

        // 3. Row 1: App icon + Title · Artist + White M2 5-pill frequency meter + Prev/Play/Next
        self.draw_row_top(state);

        // 4. Row 2: P1 Spotify hover-reveal zero-jump progress bar + tabular cur / dur
        self.draw_row_bottom(state);

        unsafe {
            self.rt.SetTransform(&Matrix3x2::identity());
        }
        let _ = unsafe { self.rt.EndDraw(None, None) };
    }

    fn draw_win11_hover_plate(&self, light: bool, alpha: f32) {
        let w = self.win_w_dip;
        let h = self.win_h_dip;
        let rr = D2D1_ROUNDED_RECT {
            rect: D2D_RECT_F { left: 0.5, top: 0.5, right: w - 0.5, bottom: h - 0.5 },
            radiusX: CORNER_R,
            radiusY: CORNER_R,
        };
        let fill = if light {
            rgba(255, 255, 255, 0.53 * alpha)
        } else {
            rgba(255, 255, 255, 0.086 * alpha)
        };
        if let Ok(b) = unsafe { self.rt.CreateSolidColorBrush(&fill, None) } {
            unsafe { self.rt.FillRoundedRectangle(&rr, &b) };
        }

        let stops = if light {
            [
                D2D1_GRADIENT_STOP { position: 0.0, color: rgba(0, 0, 0, 0.07 * alpha) },
                D2D1_GRADIENT_STOP { position: 0.5, color: rgba(0, 0, 0, 0.095 * alpha) },
                D2D1_GRADIENT_STOP { position: 1.0, color: rgba(0, 0, 0, 0.125 * alpha) },
            ]
        } else {
            [
                D2D1_GRADIENT_STOP { position: 0.0, color: rgba(255, 255, 255, 0.11 * alpha) },
                D2D1_GRADIENT_STOP { position: 0.35, color: rgba(255, 255, 255, 0.05 * alpha) },
                D2D1_GRADIENT_STOP { position: 1.0, color: rgba(255, 255, 255, 0.045 * alpha) },
            ]
        };
        if let Ok(sc) = unsafe { self.rt.CreateGradientStopCollection(&stops, D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP) } {
            let props = D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
                startPoint: Vector2::new(0.0, 0.0),
                endPoint: Vector2::new(0.0, h),
            };
            if let Ok(gb) = unsafe { self.rt.CreateLinearGradientBrush(&props, None, &sc) } {
                unsafe { self.rt.DrawRoundedRectangle(&rr, &gb, 1.0, None) };
            }
        }
    }

    fn draw_cover(&self, light: bool) {
        let dest = D2D_RECT_F { left: 0.0, top: 0.0, right: COVER_SIZE, bottom: COVER_SIZE };
        let mut pushed = false;
        if let Ok(layer) = unsafe { self.rt.CreateLayer(None) } {
            let lp = windows::Win32::Graphics::Direct2D::D2D1_LAYER_PARAMETERS {
                contentBounds: dest,
                geometricMask: core::mem::ManuallyDrop::new(Some(self.cover_clip.cast().unwrap())),
                maskAntialiasMode: windows::Win32::Graphics::Direct2D::D2D1_ANTIALIAS_MODE_PER_PRIMITIVE,
                maskTransform: Matrix3x2::identity(),
                opacity: 1.0,
                opacityBrush: core::mem::ManuallyDrop::new(None),
                layerOptions: windows::Win32::Graphics::Direct2D::D2D1_LAYER_OPTIONS_NONE,
            };
            unsafe { self.rt.PushLayer(&lp, &layer) };
            let mut m = lp.geometricMask;
            unsafe { core::mem::ManuallyDrop::drop(&mut m) };
            pushed = true;
        }

        if let Some(bmp) = &self.art_bmp {
            unsafe { self.rt.DrawBitmap(bmp, Some(&dest), 1.0, D2D1_BITMAP_INTERPOLATION_MODE_LINEAR, None) };
        } else {
            self.draw_fallback_cover();
        }

        if pushed {
            unsafe { self.rt.PopLayer() };
        }

        // Crisp 1px cover rim
        let rim = if light { rgba(0, 0, 0, 0.14) } else { rgba(255, 255, 255, 0.16) };
        if let Ok(b) = unsafe { self.rt.CreateSolidColorBrush(&rim, None) } {
            let rr = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F { left: 0.5, top: 0.5, right: COVER_SIZE - 0.5, bottom: COVER_SIZE - 0.5 },
                radiusX: CORNER_R,
                radiusY: CORNER_R,
            };
            unsafe { self.rt.DrawRoundedRectangle(&rr, &b, 1.0, None) };
        }
    }

    fn draw_fallback_cover(&self) {
        let stops = [
            D2D1_GRADIENT_STOP { position: 0.0, color: rgba(26, 16, 64, 1.0) },
            D2D1_GRADIENT_STOP { position: 0.55, color: rgba(107, 29, 110, 1.0) },
            D2D1_GRADIENT_STOP { position: 1.0, color: rgba(232, 74, 95, 1.0) },
        ];
        if let Ok(sc) = unsafe { self.rt.CreateGradientStopCollection(&stops, D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP) } {
            let props = D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
                startPoint: Vector2::new(0.0, 0.0),
                endPoint: Vector2::new(34.0, 40.0),
            };
            if let Ok(gb) = unsafe { self.rt.CreateLinearGradientBrush(&props, None, &sc) } {
                unsafe { self.rt.FillRectangle(&D2D_RECT_F { left: 0.0, top: 0.0, right: COVER_SIZE, bottom: COVER_SIZE }, &gb) };
            }
        }
        // Subtle sunset disc + music note silhouette
        if let Ok(sun) = unsafe { self.rt.CreateSolidColorBrush(&rgba(255, 210, 126, 0.92), None) } {
            unsafe {
                self.rt.FillEllipse(
                    &D2D1_ELLIPSE { point: Vector2::new(20.0, 18.0), radiusX: 9.0, radiusY: 9.0 },
                    &sun,
                );
            }
        }
        if let Ok(mt) = unsafe { self.rt.CreateSolidColorBrush(&rgba(18, 10, 36, 1.0), None) } {
            if let Some(geo) = make_polygon(&self.d2d, &[(0.0, 29.0), (10.0, 22.5), (17.5, 27.5), (26.0, 20.5), (40.0, 29.0), (40.0, 40.0), (0.0, 40.0)]) {
                unsafe { self.rt.FillGeometry(&geo, &mt, None) };
            }
        }
    }

    fn draw_row_top(&self, state: &FrameState) {
        let fg = if state.light_taskbar { rgba(20, 23, 29, 1.0) } else { rgba(255, 255, 255, 1.0) };
        let fg_sub = if state.light_taskbar { rgba(20, 23, 29, 0.65) } else { rgba(255, 255, 255, 0.70) };
        let fg_dim = if state.light_taskbar { rgba(20, 23, 29, 0.44) } else { rgba(255, 255, 255, 0.46) };

        // App icon and Title · Artist share one hover plate: a click opens the app.
        let (app_hover, app_pressed) = (state.btn_hover_alpha[3], state.btn_pressed == Some(3));
        if !state.media.aumid.is_empty() && (app_hover > 0.01 || app_pressed) {
            let rect = self.app_rect(state.media, state.show_visualizer);
            self.draw_hover_plate(rect, state.light_taskbar, app_hover, app_pressed);
        }
        self.draw_app_icon(APP_ICON_LEFT, 5.5, state.light_taskbar);

        // Title · Artist text up to the EQ meter (or the controls if the EQ is hidden)
        if let Some((layout, ranges)) = self.title_layout(state.media, state.show_visualizer) {
            if let Some((dot, artist)) = ranges {
                if let Ok(dot_brush) = unsafe { self.rt.CreateSolidColorBrush(&fg_dim, None) } {
                    let _ = unsafe { layout.SetDrawingEffect(&dot_brush, dot) };
                }
                if let Ok(art_brush) = unsafe { self.rt.CreateSolidColorBrush(&fg_sub, None) } {
                    let _ = unsafe { layout.SetDrawingEffect(&art_brush, artist) };
                }
            }
            if let Ok(fg_brush) = unsafe { self.rt.CreateSolidColorBrush(&fg, None) } {
                unsafe {
                    self.rt.DrawTextLayout(Vector2::new(TITLE_LEFT, ROW1_TOP), &layout, &fg_brush, D2D1_DRAW_TEXT_OPTIONS_CLIP);
                }
            }
        }

        // White M2 5-pill center-expanding live frequency meter
        if state.show_visualizer {
            self.draw_m2_spectrum(state);
        }

        // Top-right transport controls (Prev / Play-Pause / Next)
        let rects = [PREV_RECT, PLAY_RECT, NEXT_RECT];
        for (idx, &(l, t, r, b)) in rects.iter().enumerate() {
            let hover_a = state.btn_hover_alpha[idx];
            self.draw_hover_plate((l, t, r, b), state.light_taskbar, hover_a, state.btn_pressed == Some(idx));

            let icon_col = if state.light_taskbar {
                rgba(20, 23, 29, 0.85 + 0.15 * hover_a)
            } else {
                rgba(255, 255, 255, 0.86 + 0.14 * hover_a)
            };
            if let Ok(br) = unsafe { self.rt.CreateSolidColorBrush(&icon_col, None) } {
                let cx = (l + r) * 0.5;
                let cy = (t + b) * 0.5;
                match idx {
                    0 => self.draw_prev_icon(cx, cy, &br),
                    1 => self.draw_play_pause_icon(cx, cy, state.media.playing, &br),
                    _ => self.draw_next_icon(cx, cy, &br),
                }
            }
        }
    }

    fn draw_m2_spectrum(&self, state: &FrameState) {
        let (l, t, r, b) = EQ_RECT;
        let cy = (t + b) * 0.5;
        let pill_w = 2.2f32;
        let gap = 1.7f32;
        let total_w = (BANDS as f32) * pill_w + ((BANDS - 1) as f32) * gap;
        let start_x = l + ((r - l) - total_w) * 0.5;
        let max_h = 11.0f32;

        let alpha = if state.media.playing { 0.65 } else { 0.22 };
        let col = if state.light_taskbar { rgba(20, 23, 29, alpha) } else { rgba(255, 255, 255, alpha) };
        let Ok(brush) = (unsafe { self.rt.CreateSolidColorBrush(&col, None) }) else { return };

        for i in 0..BANDS {
            let level = if state.media.playing { state.bands[i].clamp(0.0, 1.0) } else { 0.0 };
            let h = pill_w + level * (max_h - pill_w);
            let x = start_x + (i as f32) * (pill_w + gap);
            let rr = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F { left: x, top: cy - h * 0.5, right: x + pill_w, bottom: cy + h * 0.5 },
                radiusX: pill_w * 0.5,
                radiusY: pill_w * 0.5,
            };
            unsafe { self.rt.FillRoundedRectangle(&rr, &brush) };
        }
    }

    fn draw_row_bottom(&mut self, state: &FrameState) {
        let (sl, st, sr, sb) = self.scrub_rect(state.media);
        let cy = (st + sb) * 0.5; // exact integer 31.0 inside 24..38 -> ZERO vertical jump!
        let track_h = 4.0f32;
        let track_top = cy - track_h * 0.5;
        let track_bot = cy + track_h * 0.5;
        let track_w = (sr - sl).max(1.0);

        let dur = state.media.duration_sec();
        let is_live_stream = dur <= 0.0 && state.media.active && state.media.playing;
        let cur = if state.scrub_dragging { state.drag_pos_sec } else { state.media.live_position_sec() };
        let ratio = if is_live_stream {
            1.0f32
        } else if dur > 0.0 {
            (cur / dur).clamp(0.0, 1.0) as f32
        } else {
            0.0
        };
        let fill_w = track_w * ratio;

        // Track background
        let track_col = if state.light_taskbar { rgba(0, 0, 0, 0.14) } else { rgba(255, 255, 255, 0.18) };
        if let Ok(br) = unsafe { self.rt.CreateSolidColorBrush(&track_col, None) } {
            let rr = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F { left: sl, top: track_top, right: sr, bottom: track_bot },
                radiusX: 2.0,
                radiusY: 2.0,
            };
            unsafe { self.rt.FillRoundedRectangle(&rr, &br) };
        }

        // Hover ghost preview fill
        if state.scrub_hover && !state.scrub_dragging && state.scrub_active_alpha > 0.01 {
            let gw = (track_w * state.scrub_hover_ratio.clamp(0.0, 1.0)).max(2.0);
            let ghost_col = if state.light_taskbar {
                rgba(0, 0, 0, 0.16 * state.scrub_active_alpha)
            } else {
                rgba(255, 255, 255, 0.20 * state.scrub_active_alpha)
            };
            if let Ok(br) = unsafe { self.rt.CreateSolidColorBrush(&ghost_col, None) } {
                let rr = D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F { left: sl, top: track_top, right: sl + gw, bottom: track_bot },
                    radiusX: 2.0,
                    radiusY: 2.0,
                };
                unsafe { self.rt.FillRoundedRectangle(&rr, &br) };
            }
        }

        // Fill capsule: crisp white at rest -> album accent on hover/drag (P1 Spotify Hover-Reveal)
        if fill_w > 0.5 {
            let rest_col = if state.light_taskbar { rgba(20, 23, 29, 0.82) } else { rgba(255, 255, 255, 0.88) };
            let fill_col = lerp_color(rest_col, self.accent, state.scrub_active_alpha);
            if let Ok(br) = unsafe { self.rt.CreateSolidColorBrush(&fill_col, None) } {
                let rr = D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F { left: sl, top: track_top, right: sl + fill_w.max(3.5), bottom: track_bot },
                    radiusX: 2.0,
                    radiusY: 2.0,
                };
                unsafe { self.rt.FillRoundedRectangle(&rr, &br) };
            }
        }

        // Smooth fade/scale knob (zero layout shift!)
        if state.scrub_active_alpha > 0.02 {
            let rad = 5.0 * (0.55 + 0.45 * state.scrub_active_alpha);
            let kx = sl + fill_w;
            // Soft shadow under knob
            if let Ok(sh) = unsafe { self.rt.CreateSolidColorBrush(&rgba(0, 0, 0, 0.35 * state.scrub_active_alpha), None) } {
                unsafe {
                    self.rt.FillEllipse(
                        &D2D1_ELLIPSE { point: Vector2::new(kx, cy + 0.8), radiusX: rad + 0.8, radiusY: rad + 0.8 },
                        &sh,
                    );
                }
            }
            let knob_col = if state.light_taskbar {
                rgba(20, 23, 29, state.scrub_active_alpha)
            } else {
                rgba(255, 255, 255, state.scrub_active_alpha)
            };
            if let Ok(br) = unsafe { self.rt.CreateSolidColorBrush(&knob_col, None) } {
                unsafe {
                    self.rt.FillEllipse(&D2D1_ELLIPSE { point: Vector2::new(kx, cy), radiusX: rad, radiusY: rad }, &br);
                }
            }
        }

        // Compact tabular cur / dur readout (or LIVE for livestreams with duration == 0)
        let previewing = !is_live_stream && state.scrub_hover && !state.scrub_dragging && dur > 0.0;
        let shown_cur = if previewing { (state.scrub_hover_ratio as f64) * dur } else { cur };
        let (time_text, cur_len) = if is_live_stream {
            (alloc::string::String::from("LIVE"), 4u32)
        } else {
            let cur_str = fmt_time(shown_cur);
            let dur_str = fmt_time(dur);
            let len = cur_str.len() as u32;
            (format!("{cur_str} / {dur_str}"), len)
        };
        let wide: Vec<u16> = time_text.encode_utf16().collect();

        let time_left = sr + TIME_GAP;
        let time_w = (TIME_RIGHT - time_left).max(28.0);
        if let Ok(layout) = unsafe { self.dwrite.CreateTextLayout(&wide, &self.fmt_time, time_w, 14.0) } {
            if let Ok(typo) = unsafe { self.dwrite.CreateTypography() } {
                let _ = unsafe {
                    typo.AddFontFeature(DWRITE_FONT_FEATURE { nameTag: DWRITE_FONT_FEATURE_TAG_TABULAR_FIGURES, parameter: 1 })
                };
                let _ = unsafe { layout.SetTypography(&typo, DWRITE_TEXT_RANGE { startPosition: 0, length: wide.len() as u32 }) };
            }

            let cur_col = if is_live_stream {
                rgba(255, 78, 78, 0.95)
            } else if previewing {
                self.accent
            } else if state.light_taskbar {
                rgba(20, 23, 29, 0.86)
            } else {
                rgba(255, 255, 255, 0.86)
            };
            let dim_col = if state.light_taskbar { rgba(20, 23, 29, 0.46) } else { rgba(255, 255, 255, 0.48) };

            if previewing || is_live_stream {
                let _ = unsafe { layout.SetFontWeight(DWRITE_FONT_WEIGHT_BOLD, DWRITE_TEXT_RANGE { startPosition: 0, length: cur_len }) };
            }
            if !is_live_stream {
                if let Ok(dim_b) = unsafe { self.rt.CreateSolidColorBrush(&dim_col, None) } {
                    let rest = (wide.len() as u32).saturating_sub(cur_len);
                    let _ = unsafe { layout.SetDrawingEffect(&dim_b, DWRITE_TEXT_RANGE { startPosition: cur_len, length: rest }) };
                }
            }
            if let Ok(cur_b) = unsafe { self.rt.CreateSolidColorBrush(&cur_col, None) } {
                unsafe {
                    self.rt.DrawTextLayout(Vector2::new(time_left, st), &layout, &cur_b, D2D1_DRAW_TEXT_OPTIONS_NONE);
                }
            }
        }
    }

    /// A control's rounded hover plate, fading in with `hover_a` and darker while pressed.
    fn draw_hover_plate(&self, (l, t, r, b): (f32, f32, f32, f32), light: bool, hover_a: f32, pressed: bool) {
        if hover_a <= 0.01 && !pressed {
            return;
        }
        let bg = if light {
            rgba(0, 0, 0, if pressed { 0.13 } else { 0.08 * hover_a })
        } else {
            rgba(255, 255, 255, if pressed { 0.17 } else { 0.11 * hover_a })
        };
        if let Ok(br) = unsafe { self.rt.CreateSolidColorBrush(&bg, None) } {
            let rr = D2D1_ROUNDED_RECT { rect: D2D_RECT_F { left: l, top: t, right: r, bottom: b }, radiusX: 4.0, radiusY: 4.0 };
            unsafe { self.rt.FillRoundedRectangle(&rr, &br) };
        }
    }

    /// The playing app's own icon, or a neutral play glyph when the shell has none for it.
    fn draw_app_icon(&self, x: f32, y: f32, light: bool) {
        let size = APP_ICON_SIZE;
        if let Some(bmp) = &self.icon_bmp {
            let dst = D2D_RECT_F { left: x, top: y, right: x + size, bottom: y + size };
            unsafe { self.rt.DrawBitmap(bmp, Some(&dst), 1.0, D2D1_BITMAP_INTERPOLATION_MODE_LINEAR, None) };
            return;
        }
        let (cx, cy, r) = (x + size * 0.5, y + size * 0.5, size * 0.48);
        let (bg, fg) = if light {
            (rgba(20, 23, 29, 0.16), rgba(20, 23, 29, 0.9))
        } else {
            (rgba(255, 255, 255, 0.16), rgba(255, 255, 255, 0.95))
        };
        if let Ok(b) = unsafe { self.rt.CreateSolidColorBrush(&bg, None) } {
            unsafe { self.rt.FillEllipse(&D2D1_ELLIPSE { point: Vector2::new(cx, cy), radiusX: r, radiusY: r }, &b) };
        }
        if let Ok(w) = unsafe { self.rt.CreateSolidColorBrush(&fg, None) }
            && let Some(tri) = make_polygon(&self.d2d, &[(cx - 1.5, cy - 2.5), (cx - 1.5, cy + 2.5), (cx + 2.6, cy)])
        {
            unsafe { self.rt.FillGeometry(&tri, &w, None) };
        }
    }

    fn draw_prev_icon(&self, cx: f32, cy: f32, br: &windows::Win32::Graphics::Direct2D::ID2D1SolidColorBrush) {
        let bar = D2D1_ROUNDED_RECT {
            rect: D2D_RECT_F { left: cx - 4.0, top: cy - 3.8, right: cx - 2.6, bottom: cy + 3.8 },
            radiusX: 0.5,
            radiusY: 0.5,
        };
        unsafe { self.rt.FillRoundedRectangle(&bar, br) };
        if let Some(tri) = make_polygon(&self.d2d, &[(cx + 3.8, cy - 3.8), (cx - 1.7, cy), (cx + 3.8, cy + 3.8)]) {
            unsafe { self.rt.FillGeometry(&tri, br, None) };
        }
    }

    fn draw_next_icon(&self, cx: f32, cy: f32, br: &windows::Win32::Graphics::Direct2D::ID2D1SolidColorBrush) {
        let bar = D2D1_ROUNDED_RECT {
            rect: D2D_RECT_F { left: cx + 2.6, top: cy - 3.8, right: cx + 4.0, bottom: cy + 3.8 },
            radiusX: 0.5,
            radiusY: 0.5,
        };
        unsafe { self.rt.FillRoundedRectangle(&bar, br) };
        if let Some(tri) = make_polygon(&self.d2d, &[(cx - 3.8, cy - 3.8), (cx + 1.7, cy), (cx - 3.8, cy + 3.8)]) {
            unsafe { self.rt.FillGeometry(&tri, br, None) };
        }
    }

    fn draw_play_pause_icon(&self, cx: f32, cy: f32, playing: bool, br: &windows::Win32::Graphics::Direct2D::ID2D1SolidColorBrush) {
        if playing {
            for dx in [-3.4f32, 1.0f32] {
                let rr = D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F { left: cx + dx, top: cy - 4.4, right: cx + dx + 2.4, bottom: cy + 4.4 },
                    radiusX: 0.7,
                    radiusY: 0.7,
                };
                unsafe { self.rt.FillRoundedRectangle(&rr, br) };
            }
        } else if let Some(tri) = make_polygon(&self.d2d, &[(cx - 3.0, cy - 4.5), (cx - 3.0, cy + 4.5), (cx + 4.2, cy)]) {
            unsafe { self.rt.FillGeometry(&tri, br, None) };
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.free_dib();
    }
}

/// Width of the `Title · Artist` text, which ends before the EQ meter (or the controls without it).
fn title_max_w(show_visualizer: bool) -> f32 {
    let right = if show_visualizer { EQ_RECT.0 - 6.0 } else { PREV_RECT.0 - 6.0 };
    (right - TITLE_LEFT).max(20.0)
}

fn rgba(r: u8, g: u8, b: u8, a: f32) -> D2D1_COLOR_F {
    D2D1_COLOR_F { r: (r as f32) / 255.0, g: (g as f32) / 255.0, b: (b as f32) / 255.0, a }
}

fn lerp_color(a: D2D1_COLOR_F, b: D2D1_COLOR_F, t: f32) -> D2D1_COLOR_F {
    let t = t.clamp(0.0, 1.0);
    D2D1_COLOR_F {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}

fn fmt_time(sec: f64) -> alloc::string::String {
    let s = if sec.is_finite() && sec > 0.0 { sec as u64 } else { 0 };
    format!("{}:{:02}", s / 60, s % 60)
}

fn minute_digits(sec: f64) -> u32 {
    let mut mins = if sec.is_finite() && sec > 0.0 { (sec as u64) / 60 } else { 0 };
    let mut digits = 1u32;
    while mins >= 10 {
        mins /= 10;
        digits += 1;
    }
    digits
}

fn make_polygon(d2d: &ID2D1Factory, pts: &[(f32, f32)]) -> Option<ID2D1PathGeometry> {
    let geo = unsafe { d2d.CreatePathGeometry() }.ok()?;
    let sink = unsafe { geo.Open() }.ok()?;
    if let Some(&(x0, y0)) = pts.first() {
        unsafe { sink.BeginFigure(Vector2::new(x0, y0), windows::Win32::Graphics::Direct2D::Common::D2D1_FIGURE_BEGIN_FILLED) };
        for &(x, y) in &pts[1..] {
            unsafe { sink.AddLine(Vector2::new(x, y)) };
        }
        unsafe { sink.EndFigure(windows::Win32::Graphics::Direct2D::Common::D2D1_FIGURE_END_CLOSED) };
    }
    let _ = unsafe { sink.Close() };
    Some(geo)
}

/// The first frame of encoded JPEG/PNG bytes, through WIC.
fn decode_image(wic: &IWICImagingFactory, bytes: &[u8]) -> Option<IWICBitmapSource> {
    let stream = unsafe { wic.CreateStream() }.ok()?;
    unsafe { stream.InitializeFromMemory(bytes) }.ok()?;
    let decoder = unsafe { wic.CreateDecoderFromStream(&stream, null(), WICDecodeMetadataCacheOnDemand) }.ok()?;
    unsafe { decoder.GetFrame(0) }.ok()?.cast().ok()
}

/// Scales a WIC image into a crisp `size x size` PBGRA buffer and extracts a vibrant accent
/// color for the P1 hover scrubber.
fn scale_to_pbgra(wic: &IWICImagingFactory, source: &IWICBitmapSource, size: u32) -> Option<(Vec<u8>, D2D1_COLOR_F)> {
    let scaler = unsafe { wic.CreateBitmapScaler() }.ok()?;
    unsafe { scaler.Initialize(source, size, size, WICBitmapInterpolationModeFant) }.ok()?;

    let conv = unsafe { wic.CreateFormatConverter() }.ok()?;
    unsafe { conv.Initialize(&scaler, &GUID_WICPixelFormat32bppPBGRA, WICBitmapDitherTypeNone, None, 0.0, WICBitmapPaletteTypeCustom) }
        .ok()?;

    let stride = size * 4;
    let mut buf = vec![0u8; (stride * size) as usize];
    unsafe { conv.CopyPixels(null(), stride, &mut buf) }.ok()?;

    let accent = extract_accent(&buf);
    Some((buf, accent))
}

/// Picks the most vibrant bright average hue from decoded BGRA pixels so the P1 hover scrub bar
/// always harmonizes with the album cover while remaining high-contrast on the taskbar.
fn extract_accent(bgra: &[u8]) -> D2D1_COLOR_F {
    let mut sum_r = 0.0f32;
    let mut sum_g = 0.0f32;
    let mut sum_b = 0.0f32;
    let mut sum_w = 0.0f32;

    for px in bgra.chunks_exact(4).step_by(3) {
        let a = (px[3] as f32) / 255.0;
        if a < 0.5 {
            continue;
        }
        let b = (px[0] as f32) / 255.0;
        let g = (px[1] as f32) / 255.0;
        let r = (px[2] as f32) / 255.0;
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let sat = if max > 0.01 { (max - min) / max } else { 0.0 };
        if sat < 0.22 || max < 0.22 {
            continue;
        }
        let w = sat * sat * max;
        sum_r += r * w;
        sum_g += g * w;
        sum_b += b * w;
        sum_w += w;
    }

    if sum_w < 0.05 {
        return rgba(255, 107, 139, 1.0);
    }
    let mut r = sum_r / sum_w;
    let mut g = sum_g / sum_w;
    let mut b = sum_b / sum_w;
    // Lift luminance slightly so dark covers still pop crisply on dark taskbars
    let max = r.max(g).max(b).max(0.01);
    if max < 0.85 {
        let boost = 0.88 / max;
        r = (r * boost).min(1.0);
        g = (g * boost).min(1.0);
        b = (b * boost).min(1.0);
    }
    D2D1_COLOR_F { r, g, b, a: 1.0 }
}

fn write_png_wic(wic: &IWICImagingFactory, path: &str, bgra: &[u8], w: u32, h: u32) -> Result<(), alloc::string::String> {
    let stream = unsafe { wic.CreateStream() }.map_err(|e| alloc::format!("WIC CreateStream: {}", e.message()))?;
    let wpath = to_wide(path);
    unsafe { stream.InitializeFromFilename(PCWSTR(wpath.as_ptr()), GENERIC_WRITE) }
        .map_err(|e| alloc::format!("WIC InitializeFromFilename: {}", e.message()))?;
    let encoder = unsafe { wic.CreateEncoder(&GUID_ContainerFormatPng, null()) }
        .map_err(|e| alloc::format!("WIC CreateEncoder: {}", e.message()))?;
    unsafe { encoder.Initialize(&stream, windows::Win32::Graphics::Imaging::WICBitmapEncoderNoCache) }
        .map_err(|e| alloc::format!("WIC encoder init: {}", e.message()))?;
    let mut frame = None;
    let mut props = None;
    unsafe { encoder.CreateNewFrame(&mut frame, &mut props) }.map_err(|e| alloc::format!("WIC CreateNewFrame: {}", e.message()))?;
    let frame = frame.ok_or_else(|| alloc::string::String::from("WIC frame missing"))?;
    unsafe { frame.Initialize(props.as_ref()) }.map_err(|e| alloc::format!("WIC frame init: {}", e.message()))?;
    unsafe { frame.SetSize(w, h) }.map_err(|e| alloc::format!("WIC SetSize: {}", e.message()))?;
    let mut fmt: GUID = GUID_WICPixelFormat32bppPBGRA;
    unsafe { frame.SetPixelFormat(&mut fmt) }.map_err(|e| alloc::format!("WIC SetPixelFormat: {}", e.message()))?;
    unsafe { frame.WritePixels(h, w * 4, bgra) }.map_err(|e| alloc::format!("WIC WritePixels: {}", e.message()))?;
    unsafe { frame.Commit() }.map_err(|e| alloc::format!("WIC frame commit: {}", e.message()))?;
    unsafe { encoder.Commit() }.map_err(|e| alloc::format!("WIC encoder commit: {}", e.message()))?;
    Ok(())
}
