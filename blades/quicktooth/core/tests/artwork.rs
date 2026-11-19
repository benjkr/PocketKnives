//! The tray icon's artwork setting (`icon`): what `--settings` and `--set` say about it, and both
//! artworks through every animation phase. Its own test binary with one test, because the
//! artwork is process-wide and the other tests draw with the default.
use quicktooth_core::blade_settings::{self, SetResult};
use quicktooth_core::icon::{self, AnimPhase, Artwork, IconState};
use quicktooth_core::settings::Settings;

#[test]
fn artwork_setting_and_both_artworks() {
    // --settings: the current artwork and both choices.
    let mut settings = Settings::default();
    let line = |s: &Settings| blade_settings::describe(s, &[]).lines().find(|l| l.starts_with("icon\t")).map(String::from);
    assert_eq!(line(&settings).as_deref(), Some("icon\tpicture\tairpodsPro\tTray icon\tairpodsPro=AirPods Pro\tairpods=AirPods"));

    // --set: checked first, applied once, the same pick again is no change.
    assert_eq!(blade_settings::parse(&["icon=beats"]).unwrap_err(), "Tray icon can be airpodsPro or airpods.");
    let change = blade_settings::parse(&["icon=airpods"]).unwrap();
    assert_eq!(change.icon, Some(Artwork::AirPods));
    assert_eq!(blade_settings::apply(&change, &mut settings, None), SetResult::SaveFailed); // no save handler here
    assert_eq!((settings.icon.as_deref(), settings.artwork()), (Some("airpods"), Artwork::AirPods));
    assert!(line(&settings).unwrap().starts_with("icon\tpicture\tairpods\t"));
    assert_eq!(blade_settings::apply(&change, &mut settings, None), SetResult::Applied);

    // Both artworks: distinct icons, and every phase ends exactly on the static icon.
    let mut connected = Vec::new();
    for artwork in Artwork::ALL {
        icon::set_artwork(artwork);
        for light in [false, true] {
            for size in [16, 20, 24, 32] {
                let on = icon::render_bgra(IconState::Connected, light, size);
                let off = icon::render_bgra(IconState::Disconnected, light, size);
                let low = icon::render_bgra(IconState::LowBattery, light, size);
                assert_eq!(icon::render_anim(AnimPhase::Landing, 0.5, 2.0, light, size), on);
                assert_eq!(icon::render_anim(AnimPhase::Cancelling, 0.3, 2.0, light, size), off);
                assert_eq!(icon::render_anim(AnimPhase::Disconnecting, 0.8, 0.0, light, size), off);
                assert_eq!(icon::render_anim(AnimPhase::LowBattery, 0.7, 0.0, light, size), low);
                assert_ne!(icon::render_anim(AnimPhase::Connecting, 0.5, 0.5, light, size), off);
                // The glow brightens the outline it runs along.
                let glow = icon::render_anim(AnimPhase::Connecting, 1.0, 0.5, light, size);
                assert!(glow.chunks(4).zip(off.chunks(4)).any(|(g, o)| g[3] > o[3]));
            }
        }
        connected.push(icon::render_bgra(IconState::Connected, false, 16));
    }
    assert_ne!(connected[0], connected[1]);

    // The Settings picture: an ICO of PNGs, one per size.
    let ico = icon::setting_picture();
    assert_eq!(&ico[..4], &[0, 0, 1, 0]);
    assert_eq!(u16::from_le_bytes([ico[4], ico[5]]), 9);
    assert_eq!(&ico[6..8], &[16, 16]);
    icon::set_artwork(Artwork::default());
}
