# Build, install and release

Part of QuickTooth's specs (index in [`AGENTS.md`](../../AGENTS.md)). What it does now. Keep it current in the same change,
and lean.

## The build

- One exe for win-x64, `QuickTooth.exe`, built from the repo root with
  `cargo build --release -p quicktooth` into `target
elease`. It needs no runtime and no VC++
  redistributable: it imports only Windows DLLs and the universal C runtime's maths
  (`sin`/`cos`/`pow`, as .NET did).
- `no_std` on the suite's `crates/pk`, with the workspace's release profile (size-optimised, LTO,
  abort on panic, stripped) and `crates/pk-build` for the no-CRT link flags and the resources
  (`assets/quicktooth.ico`, `win/app.manifest`, version information) through the Windows SDK's
  `rc.exe`. Every warning is an error (`.cargo/config.toml`).
- Building needs Rust through rustup (`rust-toolchain.toml` pins the toolchain and the wasm32
  target) and Visual Studio's C++ build tools (their libraries and the Windows SDK).
- The version is `version` in `win/Cargo.toml`; `--version` prints it and the installers read it.

## Installing

- `publish.ps1`: builds the exe, asks the running tray to exit, copies it to
  `%LOCALAPPDATA%\Programs\QuickTooth`, removes files left by 1.0 builds and starts it.
- `installer/build.ps1` builds `dist/QuickTooth-Setup-<Version>.exe` with Inno Setup 6.7+
  (`winget install JRSoftware.InnoSetup`).
  - Per-user, no admin prompt. It closes the running tray first (`--exit`, then terminates it
    after 10 s).
  - Start with Windows is offered, and ticked, on a fresh install only; an update leaves the user's
    setting as it was.
  - Uninstalling removes the app, its folder, the Start menu entry and the autostart value, and
    keeps `%APPDATA%\QuickTooth`.
  - Unsigned, so Windows SmartScreen warns on download.
- **What goes on the user's PC**: QuickTooth from the
  [PocketKnives installer](../../../../docs/specs/installer.md), from `master`, with the Tray icon
  pictures its build writes (`--write-setting-pictures`). `publish.ps1` and
  this installer make the stand-alone QuickTooth, for people without the suite; on the user's PC
  it would run beside the suite's copy, so they aren't used there.

## Releases

- Bump `version` in `win/Cargo.toml`, tag `master` as `quicktooth-vX.Y.Z`
  ([pull requests](../../../../docs/working/pull-requests.md)) and build the installer from the tag.

## Before a public release

- The installer is unsigned: say so (SmartScreen) wherever it's offered.
- The README's icon credit must name the artwork's author (Alex Martynov), its CC Attribution
  licence, and say it was modified.
- Never ship `dist/QuickTooth-Setup-1.0.0.exe`: it bundles an unpatched runtime.
