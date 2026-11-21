use std::path::Path;

fn main() {
    pk_build::exe();
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    pk_build::resources(&pk_build::Resources {
        icon: Some(&dir.join("assets/nowplaying.ico")),
        manifest: Some(&dir.join("app.manifest")),
        product: "NowPlaying",
        file_name: "NowPlaying.exe",
    });
}
