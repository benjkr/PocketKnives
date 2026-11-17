<img src="assets/icon.svg" width="64" height="64" alt="">

# SnipSnap

A very fast, very small screen snippet tool for Windows 11. It does three things:
copy a **rectangle**, a **window** you pick, or the **screen** under the cursor to
the clipboard. The captured area flashes briefly so you know it worked. There's
no editor, no saved files, no tray icon, and no UI beyond the selection overlay.

It's a single C file built into a ~19 KB native exe. It has no runtime or CRT and
uses only system DLLs. The background instance uses about 1–2 MB of its own
memory (private working set).

## Usage

```
snipsnap          run in the background and listen for hotkeys
snipsnap rect     select a rectangle once, then exit
snipsnap window   pick a window once, then exit
snipsnap full     capture the monitor under the cursor once, then exit
snipsnap quit     stop the background instance
```

| Hotkey (background mode) | Action      |
| ------------------------ | ----------- |
| `Ctrl+Alt+R`             | Rectangle   |
| `Ctrl+Alt+W`             | Window      |
| `Ctrl+Alt+F`             | Full screen |

If another program already owns a hotkey, SnipSnap falls back to `Ctrl+PrtSc`
(rectangle), `Ctrl+Alt+Shift+W` (window) or `Ctrl+Shift+PrtSc` (full screen),
then `Ctrl+Alt+Shift+R` / `Ctrl+Alt+Shift+F`. When started from a terminal it
prints the hotkeys it actually registered. Started without a console, it shows a
message box only if it had to use a fallback or a hotkey could not be registered. Plain `PrtSc`,
`Win+Shift+S` and the other Windows screenshot keys are left alone.

The hotkeys and the flash can be changed in the PocketKnives Settings window. A hotkey chosen
there is used exactly, with no fallbacks; Settings refuses one another program already owns.

**Rectangle:** all monitors freeze and dim. Drag to select (a white border shows
the selection); the image is copied when you release the mouse. `Esc` or
right-click cancels.

**Window:** all monitors freeze and dim, and the window under the cursor lights
up with a white border. Click to copy it; `Esc` or right-click cancels. The
copy is the window's visible frame (no invisible resize border or shadow), cut
from the frozen image, so it's on the clipboard a few ms after you click. It
copies what was on screen in that area, so anything covering the window is
included. Over the bare desktop, the whole monitor is picked.

**Full screen:** copies the monitor the mouse cursor is on.

After each capture the copied area flashes white for about 150 ms. The flash
appears after the pixels are taken and is cleared before the next capture, so
it never shows up in an image.

Captures are in physical pixels (per-monitor DPI aware) and go on the clipboard
as a 24-bit DIB, which pastes into Paint, Office, browsers, chat apps and so on.

The one-shot modes suit launchers (PowerToys Run, AutoHotkey, a shortcut key on
a `.lnk`). Exit codes: `0` copied, `1` cancelled, `2` failed. A one-shot run
stays alive for the ~150 ms flash after the image is on the clipboard. The exe
is a GUI program, so PowerShell doesn't wait for it unless you pipe it
(`snipsnap full | Out-Null`) or use `Start-Process -Wait`.

To start it with Windows, put a shortcut to `snipsnap.exe` in `shell:startup`.

## Speed

Medians on a two-monitor desktop (1920×1080 + 2560×1440, GTX 1660 Super,
Windows 11), measured from outside the program:

| Path                                          | Time   |
| --------------------------------------------- | ------ |
| `Ctrl+Alt+R` → rectangle overlay visible      | ~70 ms |
| Click on a window → on clipboard (486×343)    | ~8 ms  |
| `Ctrl+Alt+F` → screen on clipboard (1920×1080) | ~32 ms |
| `Ctrl+Alt+F` → screen on clipboard (2560×1440) | ~43 ms |
| Mouse release → selection on clipboard        | ~2 ms  |

Most of the time is GDI reading the screen back from the compositor. GPU capture
via DXGI Desktop Duplication was measured at about 5–10 ms for that step. It was
left out on purpose: keeping it ready costs about 70 MB of graphics-driver
memory in the background process.

## Build

Needs Visual Studio (or the Build Tools) with the C++ workload:

```
build.bat
```

The script finds VS itself if you're not in a developer prompt. It stops a
running instance first so the exe isn't locked, then produces `snipsnap.exe`.

Working on SnipSnap (people and coding agents alike): start with the suite's
[`AGENTS.md`](../../AGENTS.md), then this blade's [`AGENTS.md`](AGENTS.md), which
points to a spec for each area in `docs/specs/`.
