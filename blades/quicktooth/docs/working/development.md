# Development workflow

Read before building or running tools for QuickTooth. Commands are from `blades/quicktooth/`
unless they say the repo root. This machine's shell quirks and delegating are suite-wide:
[development](../../../../docs/working/development.md).

## Setup

- Rust through rustup. The repo's `rust-toolchain.toml` pins the version and the
  `wasm32-unknown-unknown` target; rustup installs both on first use. Visual Studio (or its Build
  Tools) with "Desktop development with C++", for the MSVC linker and the Windows SDK's `rc.exe`.
- `cd sim; bun install` once (Bun 1.4.4+; Playwright 1.60 with its Chromium). ffmpeg on `PATH` for
  recordings, GIFs and `--zoom`.
- Inno Setup 6.7+ for the installer (`winget install JRSoftware.InnoSetup`). Python with `shapely`
  only for regenerating the glyphs.

## Commands and how long they take

Run cargo from the repo root, so it finds `rust-toolchain.toml` and `.cargo/config.toml` (every
warning is an error).

- `cargo build --release -p quicktooth`: `target\release\QuickTooth.exe`. Nothing is installed.
- `cargo test -p quicktooth-core`: the core's unit tests.
- `cd sim; bun qt-sim.mjs test`: about a minute; rebuilds the browser core first if its Rust
  changed (`cargo build --release --target wasm32-unknown-unknown -p quicktooth-web`).
- Real-time recordings (`run` of a recorded scenario, `examples/compare-profiles.mjs`) take as long
  as what they show: run them in the background and wait with `t3_background_wait`.
- `installer/build.ps1` and `publish.ps1` only per [build and install](../specs/build-and-install.md);
  never install with them on the user's PC, which has QuickTooth from the
  [PocketKnives installer](../../../../docs/specs/installer.md).
