# AGENTS.md: SnipSnap

SnipSnap: a screen snippet tool. A hotkey copies a dragged rectangle, a window picked by hovering,
or the monitor under the cursor to the clipboard, and the captured area flashes. No editor, no
files, no tray icon. One C file (`snipsnap.c`), no CRT, only system DLLs.

**The suite's [`AGENTS.md`](../../AGENTS.md) comes first**: its rules hold here too. This file adds
what's specific to SnipSnap. Paths and commands here and in this blade's `docs/` are from
`blades/snipsnap/`.

## Rules for SnipSnap

- **Small and fast is the product.** Budget: an exe of about 16 KB, about 1–2 MB private working
  set in the background, hotkey to rectangle overlay visible in about 70 ms. A change that costs
  any of these is measured and shown before and after.
- **No CRT, no extra DLLs.** Only `kernel32`, `user32`, `gdi32` and `dwmapi`, linked with
  `/NODEFAULTLIB` and its own entry point; everything else is written in `snipsnap.c`. Anything
  that would load another DLL (shell32 for argument parsing, a C runtime, COM) is left out.
- **Never build with `build.bat`, and don't run any `snipsnap.exe`** (a scratch build included;
  only `help`, `settings` and a scratch build's `set` in a settings preview are harmless). `build.bat` and `snipsnap quit` stop the user's running copy (found by
  window class), `rect` and `window` freeze every monitor, and every capture replaces the
  clipboard. Build with the scratch
  command in [build and test](docs/specs/build-and-test.md). Running it is the user's call
  ([live testing](docs/working/live-testing.md)).
- **Nothing SnipSnap draws may end up in a capture**: the flash appears after the pixels are taken
  and is cleared before the next capture; the overlay is cloaked before the copy.
- **Console output and exit codes are behaviour.** Launchers and scripts read the exit codes
  (`0` copied, `1` cancelled, `2` failed), and the hotkey banner tells the user which keys work.

## Showing a change

There's no simulator. A visible or behaviour change is shown as a scratch build the user runs
themselves (`<scratch>\snipsnap.exe rect`), with what to try and, for speed or memory, numbers
before and after.

## Verification

- Every change: the scratch build with zero warnings (`/W4 /WX`), its size, and
  `dumpbin /dependents` listing only the four system DLLs ([build and test](docs/specs/build-and-test.md)).
- Timing and memory claims are measured from outside the program, the way the README's numbers
  were, and only with the user's go (it means running it).

## Working rules: load when applicable

| Work | Document |
|---|---|
| Building and checking (the commands are in the spec) | [Build and test](docs/specs/build-and-test.md) |
| Running SnipSnap, the installed copy, the clipboard and screen | [Live testing](docs/working/live-testing.md) |
| Changing a behaviour, look or speed trade-off decided before | [Decisions](docs/working/decisions.md) |

Replies, pull requests, this machine's quirks and the desktop rules are suite-wide:
[`docs/working/`](../../docs/working/) at the repo root.

## Specs: read the one for what you touch

| Area | Spec |
|---|---|
| The three captures, the frozen overlay, window picking, the clipboard format, the flash | [capture](docs/specs/capture.md) |
| Background instance, hotkeys and their fallbacks, settings and reload, messages, command line, exit codes | [hotkeys and CLI](docs/specs/hotkeys-and-cli.md) |
| Compiler and linker flags, size, imports, scratch builds, what to check | [build and test](docs/specs/build-and-test.md) |

Defaults live in code (the `#define`s and hotkey tables at the top of `snipsnap.c`), not in the
docs' prose.
