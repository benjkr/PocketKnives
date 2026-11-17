# Hotkeys and command line

Part of SnipSnap's specs (index in [`AGENTS.md`](../../AGENTS.md)). What it does now; the hotkey
lists are the `*_KEYS` tables at the top of `snipsnap.c`. Keep it current in the same change, and
lean.

## The background instance (`snipsnap` with no argument)

- One per user session: the mutex `Local\SnipSnap.Daemon`. A second start prints
  `SnipSnap is already running. Use 'snipsnap quit' to stop it.` and exits 0.
- A message-only window of class `SnipSnapDaemon` owns the hotkeys and the clipboard data; `quit`
  finds it by that class. A hotkey pressed while a capture is running is ignored.
- Once the hotkeys are registered it lets go of any inherited console or pipe, so it never keeps a
  terminal or a capturing parent waiting.
- Start with Windows: a shortcut to `snipsnap.exe` in `shell:startup`.

## Hotkeys

Each capture tries its list in order and keeps the first that registers (all `MOD_NOREPEAT`):

| Capture | Default | Fallbacks, in order |
|---|---|---|
| Rectangle | `Ctrl+Alt+R` | `Ctrl+PrtSc`, `Ctrl+Alt+Shift+R` |
| Window | `Ctrl+Alt+W` | `Ctrl+Alt+Shift+W` |
| Full screen | `Ctrl+Alt+F` | `Ctrl+Shift+PrtSc`, `Ctrl+Alt+Shift+F` |

- Plain `PrtSc`, `Win+Shift+S` and Windows' other screenshot keys are left alone.
- From a terminal (its stdout, or the parent's console) it prints the keys it registered, with
  `(unavailable - hotkeys taken)` for a capture that got none, and `Stop with: snipsnap quit`.
- Without a console: a warning box listing the keys when any of them isn't the default. If no
  capture got a key: an error box (or the console message) and exit code 2.

## Command line: `snipsnap [command]`

Only the first argument counts. It's case-insensitive, and leading `-` or `/` are ignored. It's
parsed by hand, so shell32 is never loaded.

| Command | Does |
|---|---|
| (none) | The background instance |
| `rect` | Select a rectangle once, then exit |
| `window` | Pick a window once, then exit |
| `full` | Copy the monitor under the cursor once, then exit |
| `quit` or `stop` | Stop the background instance: posts it `WM_CLOSE` and waits up to 3 s. Prints `SnipSnap stopped.` or `SnipSnap is not running.`; exit 0 either way |
| `help`, `h` or `?` | Usage; exit 0 |
| anything else | Usage; exit 2 |

- Exit codes: `0` copied (or ok), `1` cancelled, `2` failed or bad usage.
- One-shot runs need no background instance and work alongside it. They exit about 150 ms after
  the image is on the clipboard, once the flash has faded.
- It's a GUI program: PowerShell doesn't wait for it unless it's piped (`snipsnap full | Out-Null`)
  or started with `Start-Process -Wait`.
