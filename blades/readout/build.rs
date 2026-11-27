use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    pk_build::exe();
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    pk_build::resources(&pk_build::Resources {
        icon: Some(&dir.join("assets/readout.ico")),
        manifest: Some(&dir.join("app.manifest")),
        description: "Readout",
        file_name: "Readout.exe",
    });
    embed_themes(&dir.join("themes"));
}

/// Embeds every `themes/*.json` as a built-in theme, named after its file, so no theme is
/// named in the code: adding a theme is adding a file.
fn embed_themes(themes: &Path) {
    println!("cargo:rerun-if-changed={}", themes.display());
    let mut files: Vec<PathBuf> = fs::read_dir(themes)
        .expect("read themes")
        .map(|e| e.expect("theme entry").path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    files.sort();
    let mut src = String::from("pub const BUILT_IN: &[(&str, &str)] = &[\n");
    for f in &files {
        println!("cargo:rerun-if-changed={}", f.display());
        let name = f.file_stem().unwrap().to_str().unwrap();
        writeln!(src, "    ({name:?}, include_str!({:?})),", f.display().to_string()).unwrap();
    }
    src.push_str("];\n");
    let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    fs::write(out.join("themes.rs"), src).expect("write themes.rs");
}
