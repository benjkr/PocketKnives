# Development workflow

Read before building or running tools for QuickTooth. Commands are from `blades/quicktooth/`.
This machine's shell quirks and delegating are suite-wide:
[development](../../../../docs/working/development.md).

## Setup

- .NET 10 SDK; Visual Studio (or its Build Tools) with "Desktop development with C++" for the
  Native AOT linker.
- `cd sim; bun install` once (Bun 1.4.4+; Playwright 1.60 with its Chromium). ffmpeg on `PATH` for
  recordings, GIFs and `--zoom`.
- Inno Setup 6.7+ for the installer (`winget install JRSoftware.InnoSetup`). Python with `shapely`
  only for regenerating the glyphs.

## Commands and how long they take

- `dotnet build src/QuickTooth -c Release`: a quick JIT build to run CLI checks; nothing installed.
- `dotnet publish src/QuickTooth/QuickTooth.csproj -c Release -r win-x64 -o <scratch>`: the native
  exe, about a minute. Must show zero warnings.
- `cd sim; bun qt-sim.mjs test`: about a minute; rebuilds the browser core first if needed
  (5–10 s).
- Real-time recordings (`run` of a recorded scenario, `examples/compare-profiles.mjs`) take as long
  as what they show: run them in the background and wait with `t3_background_wait`.
- `installer/build.ps1` and `publish.ps1` only per [build and install](../specs/build-and-install.md);
  never install with them on the user's PC, which has QuickTooth from the
  [PocketKnives installer](../../../../docs/specs/installer.md).
