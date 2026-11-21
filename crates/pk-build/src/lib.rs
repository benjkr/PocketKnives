//! Used from a blade exe's `build.rs` (build scripts run with std; nothing here ships).
//!
//! - [`exe`]: the link flags every `no_std` blade exe uses: its own entry point
//!   (`pk::entry!`), the Windows subsystem, no C runtime or default libraries.
//! - [`resources`]: compiles an icon, a manifest and version information into the exe with the
//!   Windows SDK's `rc.exe` (found without a Visual Studio prompt).
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Link flags for a `no_std` blade exe.
pub fn exe() {
    // /DEBUG:NONE: rustc asks for a debug directory even when symbols are stripped (512 bytes).
    for arg in ["/NODEFAULTLIB", "/ENTRY:WinMainCRTStartup", "/SUBSYSTEM:WINDOWS", "/MANIFEST:NO", "/DEBUG:NONE"] {
        println!("cargo:rustc-link-arg-bins={arg}");
    }
}

/// `CompanyName` and `ProductName` of every exe: they're all parts of one suite.
const SUITE: &str = "PocketKnives";
const COPYRIGHT: &str = "Copyright (C) 2026 PocketKnives";

/// What goes into the exe's resources.
pub struct Resources<'a> {
    /// The exe's icon (`.ico`), shown by Explorer and the Start menu.
    pub icon: Option<&'a Path>,
    /// An application manifest (DPI awareness, common controls, OS support).
    pub manifest: Option<&'a Path>,
    /// `FileDescription`: the name Task Manager and the file's properties show.
    pub description: &'a str,
    /// The exe's file name, for `OriginalFilename` and, without `.exe`, `InternalName`.
    pub file_name: &'a str,
}

/// Compiles `r` into a `.res` and links it into the package's binaries. The version is the
/// package's (`CARGO_PKG_VERSION`). The icon is resource 1.
pub fn resources(r: &Resources) {
    resources_with_icon_id(r, 1);
}

/// As [`resources`], with the icon under `icon_id`: the runner's is 32512 (`IDI_APPLICATION`,
/// the ID .NET gave it), which its own code loads.
pub fn resources_with_icon_id(r: &Resources, icon_id: u16) {
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));
    let version = env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION");
    for p in [r.icon, r.manifest].into_iter().flatten() {
        rerun(p);
    }
    let rc_path = out.join("resources.rc");
    let res_path = out.join("resources.res");
    fs::write(&rc_path, rc_text(r, icon_id, &version)).expect("write resources.rc");
    let status = Command::new(rc_exe()).args(["/nologo", "/fo"]).arg(&res_path).arg(&rc_path).status().expect("run rc.exe");
    assert!(status.success(), "rc.exe failed");
    println!("cargo:rustc-link-arg-bins={}", res_path.display());
}

/// The `.rc` script: `r`'s icon under `icon_id`, its manifest, and `version` as the file and
/// product version.
fn rc_text(r: &Resources, icon_id: u16, version: &str) -> String {
    let mut nums: Vec<u32> = version.split(['.', '-', '+']).take(3).map(|p| p.parse().unwrap_or(0)).collect();
    nums.resize(4, 0);
    let dotted = nums.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",");

    // Numeric IDs and constants instead of #include, so rc needs no SDK include path:
    // RT_MANIFEST = 24, CREATEPROCESS_MANIFEST_RESOURCE_ID = 1, VOS_NT_WINDOWS32 = 0x40004, VFT_APP = 1.
    let mut rc = String::new();
    if let Some(icon) = r.icon {
        writeln!(rc, "{icon_id} ICON \"{}\"", escape(icon)).unwrap();
    }
    if let Some(manifest) = r.manifest {
        writeln!(rc, "1 24 \"{}\"", escape(manifest)).unwrap();
    }
    write!(
        rc,
        "1 VERSIONINFO\nFILEVERSION {dotted}\nPRODUCTVERSION {dotted}\nFILEOS 0x40004\nFILETYPE 0x1\n\
         BEGIN\n BLOCK \"StringFileInfo\"\n BEGIN\n  BLOCK \"040904B0\"\n  BEGIN\n\
         \x20  VALUE \"CompanyName\", \"{SUITE}\"\n   VALUE \"ProductName\", \"{SUITE}\"\n\
         \x20  VALUE \"FileDescription\", \"{d}\"\n   VALUE \"LegalCopyright\", \"{COPYRIGHT}\"\n\
         \x20  VALUE \"FileVersion\", \"{version}\"\n   VALUE \"ProductVersion\", \"{version}\"\n\
         \x20  VALUE \"InternalName\", \"{internal}\"\n   VALUE \"OriginalFilename\", \"{f}\"\n  END\n END\n\
         \x20BLOCK \"VarFileInfo\"\n BEGIN\n  VALUE \"Translation\", 0x409, 1200\n END\nEND\n",
        d = r.description,
        internal = r.file_name.strip_suffix(".exe").unwrap_or(r.file_name),
        f = r.file_name,
    )
    .unwrap();
    rc
}

fn rerun(p: &Path) {
    println!("cargo:rerun-if-changed={}", p.display());
}

fn escape(p: &Path) -> String {
    p.display().to_string().replace('\\', "\\\\")
}

/// `rc.exe`: `$RC`, else on `PATH`, else the newest Windows SDK's x64 one.
fn rc_exe() -> PathBuf {
    println!("cargo:rerun-if-env-changed=RC");
    if let Some(rc) = env::var_os("RC") {
        return rc.into();
    }
    if let Some(path) = env::var_os("PATH") {
        for dir in env::split_paths(&path) {
            let p = dir.join("rc.exe");
            if p.is_file() {
                return p;
            }
        }
    }
    let kits = Path::new(r"C:\Program Files (x86)\Windows Kits\10\bin");
    let mut versions: Vec<PathBuf> = fs::read_dir(kits)
        .map(|d| d.filter_map(|e| e.ok()).map(|e| e.path().join(r"x64\rc.exe")).filter(|p| p.is_file()).collect())
        .unwrap_or_default();
    versions.sort();
    versions.pop().expect("rc.exe not found: install the Windows SDK (Visual Studio's C++ workload has it)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc_script() {
        let r = Resources {
            icon: Some(Path::new(r"C:\a\app.ico")),
            manifest: Some(Path::new(r"C:\a\app.manifest")),
            description: "P app",
            file_name: "p.exe",
        };
        let rc = rc_text(&r, 1, "1.2.3-beta");
        // rc reads a backslash in a string as an escape: each one is doubled.
        assert!(rc.starts_with(r#"1 ICON "C:\\a\\app.ico""#));
        assert!(rc.contains("\n1 24 \"C:\\\\a\\\\app.manifest\"\n1 VERSIONINFO\nFILEVERSION 1,2,3,0\n"));
        assert!(rc.contains("VALUE \"FileVersion\", \"1.2.3-beta\"") && rc.contains("VALUE \"OriginalFilename\", \"p.exe\""));
        assert!(rc.contains("VALUE \"CompanyName\", \"PocketKnives\"") && rc.contains("VALUE \"ProductName\", \"PocketKnives\""));
        assert!(rc.contains("VALUE \"FileDescription\", \"P app\"") && rc.contains("VALUE \"InternalName\", \"p\""));
        assert!(rc.contains("VALUE \"LegalCopyright\", \"Copyright (C) 2026 PocketKnives\""));
        assert!(rc_text(&r, 32512, "0.1.0").starts_with("32512 ICON \"C:"));
        let bare = Resources { icon: None, manifest: None, ..r };
        assert!(rc_text(&bare, 32512, "0.1.0").starts_with("1 VERSIONINFO\n"));
    }
}
