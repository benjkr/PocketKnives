# Running SnipSnap, the installed copy, the screen and the clipboard

Read before running any `snipsnap.exe`, a scratch build included. The suite-wide rules (the
installed blades, the desktop) are in [live testing](../../../../docs/working/live-testing.md).

## What running it does to the user's PC

Every command acts on the user's session, whichever copy of the exe runs it:

| Run | Effect |
|---|---|
| `snipsnap` (no argument) | Exits at once while the user's copy runs (the mutex). Otherwise it takes the hotkeys |
| `snipsnap quit` / `stop`, `build.bat` with no folder | Stops the user's running copy (found by window class) |
| `snipsnap rect` / `window` | Freezes and dims every monitor until a click, Esc or right-click |
| `snipsnap full`, or any finished capture | Replaces the user's clipboard and flashes the screen |
| `snipsnap help` | Prints usage; harmless |

- So agents don't run it at all unless the user says so in that request. A change is shown as a
  scratch build the user runs themselves.
- Before a run the user agreed to, check the user's copy is running (`Get-Process snipsnap`, its
  path under `%LOCALAPPDATA%\Programs\PocketKnives`) and that the `shell:startup` shortcut exists.
  Afterwards check both again, and say so.
- A one-shot scratch build can run alongside the user's copy; a background one can't (the mutex),
  so trying a new background build means stopping the user's copy first: ask.

## Measuring

- Memory: the private working set of the idle background instance (the `Working Set - Private`
  performance counter), read-only.
- Speed is timed from outside the program, as for the README: the user's go is needed, because it
  means pressing hotkeys and capturing their screen.
