use std::path::Path;

fn main() {
    pk_build::exe();
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    // The suite icon's ID, `runner::SUITE_ICON` (a build script can't use its own package's lib).
    pk_build::resources_with_icon_id(
        &pk_build::Resources {
            icon: Some(&dir.join("../../assets/pocketknives.ico")),
            manifest: Some(&dir.join("app.manifest")),
            description: "PocketKnives tray icon and settings",
            file_name: "pk.exe",
        },
        32512,
    );
}
