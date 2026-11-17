//! Icon pixels to HICONs, and the `--render-icons` check sheets drawn through the real
//! HICON + `DrawIconEx` path (16/20/24/32 px on both taskbars, every animation frame, 240 px glyphs).
//!
//! GROUNDWORK STUB: the API is final; the bodies are placeholders until it's ported.
use alloc::string::String;
use alloc::vec::Vec;
use quicktooth_core::host::IconImage;
use quicktooth_core::icon::{AnimPhase, IconState};
use windows_sys::Win32::UI::WindowsAndMessaging::HICON;

/// The HICON for `icon`, made on first use and kept in `icon.host_handle`.
#[allow(dead_code)]
pub fn handle_for(icon: &IconImage) -> HICON {
    from_bgra(&icon.bgra, icon.size)
}

/// Destroys `icon`'s HICON, if one was made.
#[allow(dead_code)]
pub fn release(_icon: &IconImage) {}

#[allow(dead_code)]
pub fn create_icon(state: IconState, light_taskbar: bool, size: i32) -> HICON {
    from_bgra(&quicktooth_core::icon::render_bgra(state, light_taskbar, size), size)
}

#[allow(dead_code)]
pub fn create_anim_frame(phase: AnimPhase, t: f64, tracer_t: f64, light_taskbar: bool, size: i32) -> HICON {
    from_bgra(&quicktooth_core::icon::render_anim(phase, t, tracer_t, light_taskbar, size), size)
}

/// A 32-bit HICON from straight-alpha BGRA pixels.
#[allow(dead_code)]
pub fn from_bgra(_bgra: &[u8], _size: i32) -> HICON {
    core::ptr::null_mut()
}

/// Writes the check sheets into `dir` (created if needed); returns the files written.
pub fn write_check_images(_dir: &str) -> Vec<String> {
    Vec::new()
}
