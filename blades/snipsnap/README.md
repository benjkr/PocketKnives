<img src="assets/icon.svg" width="64" height="64" alt="">

# SnipSnap

A very fast, very small screen snippet tool for Windows 11. It does two things:
copy a **rectangle** or the **whole screen** to the clipboard. There's no editor,
no saved files, no tray icon, and no UI beyond the selection overlay.

It's a single C file built into a ~13 KB native exe. It has no runtime or CRT and
uses only system DLLs. The background instance uses about 1–2 MB of its own
memory (private working set).

## Usage

```
snipsnap         run in the background and listen for hotkeys
snipsnap rect    select a rectangle once, then exit
snipsnap full    capture the whole screen once, then exit
snipsnap quit    stop the background instance
```

| Hotkey (background mode) | Action       |
| ------------------------ | ------------ |
| `Ctrl+Alt+R`             | Rectangle    |
| `Ctrl+Alt+W`             | Whole screen |

If another program already owns a hotkey, SnipSnap falls back to
`Ctrl+PrtSc` / `Ctrl+Shift+PrtSc`, then `Ctrl+Alt+Shift+S` / `Ctrl+Alt+Shift+F`
(rectangle / whole screen). When started
from a terminal it prints the hotkeys it actually registered. Started without a
console, it shows a message box only if a hotkey could not be registered. Plain
`PrtSc`, `Win+Shift+S` and the other Windows screenshot keys are left alone.

**Rectangle:** the screen freezes and dims. Drag to select; the image is copied
when you release the mouse. `Esc` or right-click cancels.

**Full screen:** copies all monitors as one image.

Captures are in physical pixels (per-monitor DPI aware) and go on the clipboard
as a 24-bit DIB, which pastes into Paint, Office, browsers, chat apps and so on.

The one-shot modes `rect` and `full` suit launchers (PowerToys Run, AutoHotkey,
a shortcut key on a `.lnk`). Exit codes: `0` copied, `1` cancelled, `2` failed.
The exe is a GUI program, so PowerShell doesn't wait for it unless you pipe it
(`snipsnap full | Out-Null`) or use `Start-Process -Wait`.

To start it with Windows, put a shortcut to `snipsnap.exe` in `shell:startup`.

## Speed

Medians on a 4480×1440 two-monitor desktop (GTX 1660 Super, Windows 11):

| Path                                   | Time   |
| -------------------------------------- | ------ |
| Hotkey → rectangle overlay visible     | ~52 ms |
| Hotkey → full screen on clipboard      | ~76 ms |
| `snipsnap rect` launch → overlay       | ~83 ms |
| `snipsnap full` launch → clipboard     | ~85 ms |
| Mouse release → overlay gone           | <1 ms  |
| Mouse release → selection on clipboard | ~2 ms  |

Most of the remaining time is GDI reading the screen back from the compositor
(about 40–50 ms for 6.45 MP). GPU capture via DXGI Desktop Duplication was
measured at about 5–10 ms for this step. It was left out on purpose: keeping it
ready costs about 70 MB of graphics-driver memory in the background process.

## Build

Needs Visual Studio (or the Build Tools) with the C++ workload:

```
build.bat
```

The script finds VS itself if you're not in a developer prompt. It stops a
running instance first so the exe isn't locked, then produces `snipsnap.exe`.
