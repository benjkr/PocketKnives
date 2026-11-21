use std::path::Path;

fn main() {
    pk_build::exe();
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    pk_build::resources(&pk_build::Resources {
        icon: Some(&dir.join("../../assets/pocketknives.ico")),
        // The Windows App SDK's activatable classes (stage.ps1 -Manifest generates it).
        manifest: Some(&dir.join("app.manifest")),
        description: "PocketKnives Settings",
        file_name: "PocketKnives.Settings.exe",
    });
}
