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
    for arg in ["/NODEFAULTLIB", "/ENTRY:WinMainCRTStartup", "/SUBSYSTEM:WINDOWS", "/MANIFEST:NO"] {
        println!("cargo:rustc-link-arg-bins={arg}");
    }
}

/// What goes into the exe's resources.
pub struct Resources<'a> {
    /// The exe's icon (`.ico`), shown by Explorer and the Start menu.
    pub icon: Option<&'a Path>,
    /// An application manifest (DPI awareness, common controls, OS support).
    pub manifest: Option<&'a Path>,
    /// `FileDescription` and `ProductName`.
    pub product: &'a str,
    /// The exe's file name, for `OriginalFilename`.
    pub file_name: &'a str,
}

/// Compiles `r` into a `.res` and links it into the package's binaries. The version is the
/// package's (`CARGO_PKG_VERSION`).
pub fn resources(r: &Resources) {
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));
    let version = env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION");
    let mut nums: Vec<u32> = version.split(['.', '-', '+']).take(3).map(|p| p.parse().unwrap_or(0)).collect();
    nums.resize(4, 0);
    let dotted = nums.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",");

    // Numeric IDs and constants instead of #include, so rc needs no SDK include path:
    // RT_MANIFEST = 24, CREATEPROCESS_MANIFEST_RESOURCE_ID = 1, VOS_NT_WINDOWS32 = 0x40004, VFT_APP = 1.
    let mut rc = String::new();
    if let Some(icon) = r.icon {
        rerun(icon);
        writeln!(rc, "1 ICON \"{}\"", escape(icon)).unwrap();
    }
    if let Some(manifest) = r.manifest {
        rerun(manifest);
        writeln!(rc, "1 24 \"{}\"", escape(manifest)).unwrap();
    }
    write!(
        rc,
        "1 VERSIONINFO\nFILEVERSION {dotted}\nPRODUCTVERSION {dotted}\nFILEOS 0x40004\nFILETYPE 0x1\n\
         BEGIN\n BLOCK \"StringFileInfo\"\n BEGIN\n  BLOCK \"040904B0\"\n  BEGIN\n\
         \x20  VALUE \"FileDescription\", \"{p}\"\n   VALUE \"ProductName\", \"{p}\"\n\
         \x20  VALUE \"FileVersion\", \"{version}\"\n   VALUE \"ProductVersion\", \"{version}\"\n\
         \x20  VALUE \"InternalName\", \"{p}\"\n   VALUE \"OriginalFilename\", \"{f}\"\n  END\n END\n\
         \x20BLOCK \"VarFileInfo\"\n BEGIN\n  VALUE \"Translation\", 0x409, 1200\n END\nEND\n",
        p = r.product,
        f = r.file_name,
    )
    .unwrap();

    let rc_path = out.join("resources.rc");
    let res_path = out.join("resources.res");
    fs::write(&rc_path, rc).expect("write resources.rc");
    let status = Command::new(rc_exe()).args(["/nologo", "/fo"]).arg(&res_path).arg(&rc_path).status().expect("run rc.exe");
    assert!(status.success(), "rc.exe failed");
    println!("cargo:rustc-link-arg-bins={}", res_path.display());
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
