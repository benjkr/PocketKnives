# Build and test

Part of SnipSnap's specs (index in [`AGENTS.md`](../../AGENTS.md)). What it does now. Keep it
current in the same change, and lean.

## The build

- Rust (the toolchain pinned in `rust-toolchain.toml` at the repo root; rustup installs it) and
  Visual Studio (or the Build Tools) with the C++ workload, for its libraries and the Windows
  SDK; cargo finds them without a developer prompt. A rebuild takes about 1 s.
- `cargo build --release -p snipsnap`, in the workspace at the repo root, with its size-optimised
  release profile (root `Cargo.toml`). Every warning fails the build (`.cargo/config.toml`).
  - `no_std`, `no_main`: `pk::entry!` is the entry point (`WinMainCRTStartup`, ending in
    `ExitProcess`), and `pk` supplies the `mem*` functions LLVM calls. `build.rs` adds `pk-build`'s
    link flags (`/NODEFAULTLIB`, `/ENTRY`, `/SUBSYSTEM:WINDOWS`, no manifest: DPI awareness is set
    in code) and `/DEBUG:NONE`, which keeps a debug directory out of `.rdata`.
  - The code: `src/main.rs` (defaults, command line, output, settings, hotkeys, the background
    instance), `src/capture.rs` (captures, overlay, clipboard, flash) and `src/pixels.rs` (the
    dimming with SSE2 and streaming stores, which every x64 CPU has; the overlay paint; the 24-bit
    rows with 8+4-byte stores).
- The exe is 20,480 bytes and imports only `KERNEL32`, `USER32`, `GDI32` and `dwmapi`. Sections
  are padded to 512 bytes, so growth comes in those steps.
- `assets/snipsnap.ico` is the SVG blade icon rendered for the PocketKnives settings window.
- `build.bat` runs that cargo line and copies the exe out of the workspace's `target\release`.
  - `build.bat` alone (for people) runs `snipsnap.exe quit` (with `start /wait`) if an exe is
    already in the folder, and puts `snipsnap.exe` next to the source. That `quit` stops
    **any** running SnipSnap, the user's installed copy included.
  - `build.bat <dir>` (an absolute path) puts the exe at `<dir>\snipsnap.exe` instead. It runs
    nothing and writes nothing to the blade folder. Agents and the
    [suite installer](../../../../docs/specs/installer.md) build this way, always into a scratch
    folder.

## Checks

- Every change: `build.bat <scratch dir>` with zero warnings and `cargo fmt --all`; the exe's size
  (say when it changes); and `dumpbin /dependents <out>\snipsnap.exe` listing the four DLLs and
  nothing else.
- Output and exit codes: run the harmless commands (`help`, an unknown one, and `settings` / `set`
  with `POCKETKNIVES_SETTINGS_PREVIEW=1` in that process's environment, [live
  testing](../working/live-testing.md)) on the old and new scratch builds, stdout through a pipe,
  and compare bytes, exit codes and the preview INI.
- A change to `src/pixels.rs`: benchmark it against the build it replaces on synthetic 4480×1440
  buffers (never the screen or the clipboard): six alternating runs, each the median of 31, with
  the release profile. Compare the outputs byte for byte, odd widths and offsets included.
- Behaviour, speed and memory need SnipSnap running, so only with the user's go
  ([live testing](../working/live-testing.md)). Speed is measured from outside the program; the
  README's figures are medians on a two-monitor desktop. Memory is the private working set of the
  idle background instance.

## Installing

- The [PocketKnives installer](../../../../docs/specs/installer.md) installs it, with a shortcut in
  `shell:startup`; SnipSnap has no installer of its own. On the user's PC only from `master`.
