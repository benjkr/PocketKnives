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
- `build.bat` (for people) sets up Visual Studio through `vswhere` when needed, runs
  `snipsnap.exe quit` (with `start /wait`) if an exe is already in the folder, and builds
  `snipsnap.exe` next to the source. That `quit` stops **any** running SnipSnap, the user's
  installed copy included.

## Building as an agent

Never `build.bat`. From a developer environment (`vcvars64.bat`), with `<out>` a scratch folder:

```bat
cl /nologo /W4 /WX /O1 /GS- /DSS_NOCRT snipsnap.c /Fo<out>\snipsnap.obj ^
   /link /NODEFAULTLIB /ENTRY:WinMainCRTStartup /SUBSYSTEM:WINDOWS ^
   /OPT:REF /OPT:ICF /MANIFEST:NO /OUT:<out>\snipsnap.exe ^
   kernel32.lib user32.lib gdi32.lib dwmapi.lib
```

The same flags as `build.bat`, so the same exe. Nothing is written to the blade folder and nothing
is run.

## Checks

- Every change: the build above with zero warnings; the exe's size (say when it changes); and
  `dumpbin /dependents <out>\snipsnap.exe` listing the four DLLs and nothing else.
- Behaviour, speed and memory need SnipSnap running, so only with the user's go
  ([live testing](../working/live-testing.md)). Speed is measured from outside the program; the
  README's figures are medians on a two-monitor desktop. Memory is the private working set of the
  idle background instance.

## Installing

- The user's copy is `%LOCALAPPDATA%\Programs\SnipSnap\snipsnap.exe`, started by a shortcut in
  `shell:startup`. There's no installer or install script. Replacing it means stopping the running
  copy, copying a `master` build over it and starting it again: only when the user asks.
