# Build and test

Part of SnipSnap's specs (index in [`AGENTS.md`](../../AGENTS.md)). What it does now. Keep it
current in the same change, and lean.

## The build

- MSVC from Visual Studio (or the Build Tools) with the C++ workload. One compile and link, about
  2 s.
- `cl /nologo /W4 /WX /O1 /GS- /DSS_NOCRT snipsnap.c /link /NODEFAULTLIB /ENTRY:WinMainCRTStartup
  /SUBSYSTEM:WINDOWS /OPT:REF /OPT:ICF /MANIFEST:NO kernel32.lib user32.lib gdi32.lib dwmapi.lib`
  - `SS_NOCRT` supplies the entry point (`WinMainCRTStartup`, ending in `ExitProcess`) and the one
    CRT function the compiler emits calls to (`memset`, via `__stosb`). Without it the file builds
    as a normal `wWinMain` program with the CRT, for debugging.
  - `/GS-`, because security cookies need the CRT. No manifest: DPI awareness is set in code.
- The exe is 16,384 bytes and imports only `KERNEL32`, `USER32`, `GDI32` and `dwmapi`.
- `build.bat` sets up Visual Studio through `vswhere` when needed and runs that line.
  - `build.bat` alone (for people) runs `snipsnap.exe quit` (with `start /wait`) if an exe is
    already in the folder, and builds `snipsnap.exe` next to the source. That `quit` stops
    **any** running SnipSnap, the user's installed copy included.
  - `build.bat <dir>` (an absolute path) builds `<dir>\snipsnap.exe` instead. It runs nothing and
    writes nothing to the blade folder. Agents and the
    [suite installer](../../../../docs/specs/installer.md) build this way, always into a scratch
    folder.

## Checks

- Every change: `build.bat <scratch dir>` with zero warnings; the exe's size (say when it changes); and
  `dumpbin /dependents <out>\snipsnap.exe` listing the four DLLs and nothing else.
- Behaviour, speed and memory need SnipSnap running, so only with the user's go
  ([live testing](../working/live-testing.md)). Speed is measured from outside the program; the
  README's figures are medians on a two-monitor desktop. Memory is the private working set of the
  idle background instance.

## Installing

- The [PocketKnives installer](../../../../docs/specs/installer.md) installs it, with a shortcut in
  `shell:startup`; SnipSnap has no installer of its own. On the user's PC only from `master`.
