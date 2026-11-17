# Hotkeys and command line

Part of SnipSnap's specs (index in [`AGENTS.md`](../../AGENTS.md)). What it does now; the hotkey
lists are the `FALLBACKS` table and the settings the `SETTINGS` table at the top of `snipsnap.c`.
Keep it current in the same change, and lean.

## The background instance (`snipsnap` with no argument)

- One per user session: the mutex `Local\SnipSnap.Daemon`. A second start prints
  `SnipSnap is already running. Use 'snipsnap quit' to stop it.` and exits 0.
- A message-only window of class `SnipSnapDaemon` owns the hotkeys and the clipboard data; `quit`
  finds it by that class. A hotkey pressed while a capture is running is ignored.
- Once the hotkeys are registered it lets go of any inherited console or pipe, so it never keeps a
  terminal or a capturing parent waiting.
- Start with Windows: a shortcut to `snipsnap.exe` in `shell:startup`.

## Hotkeys

A capture with a saved hotkey ([settings](#settings)) registers exactly that key, with no
fallbacks. A capture without one tries its list in order and keeps the first that registers (all
`MOD_NOREPEAT`):

| Capture | Default | Fallbacks, in order |
|---|---|---|
| Rectangle | `Ctrl+Alt+R` | `Ctrl+PrtSc`, `Ctrl+Alt+Shift+R` |
| Window | `Ctrl+Alt+W` | `Ctrl+Alt+Shift+W` |
| Full screen | `Ctrl+Alt+F` | `Ctrl+Shift+PrtSc`, `Ctrl+Alt+Shift+F` |

- Plain `PrtSc`, `Win+Shift+S` and Windows' other screenshot keys are left alone.
- From a terminal (its stdout, or the parent's console) it prints the keys it registered, with
  `(unavailable - hotkeys taken)` for a capture that got none, and `Stop with: snipsnap quit`. A
  saved key is printed by its Windows key name with `Ctrl+`, `Alt+`, `Shift+`, `Win+` in front
  (`GetKeyNameTextW`, UTF-8; `F13`–`F24` have no Windows name and print as such).
- Without a console: a warning box listing the keys when a capture didn't get its first choice
  (its saved key, or the first of its list). If no capture got a key: an error box (or the console
  message) and exit code 2. With no settings file, all of this is exactly as before settings
  existed.

## Settings

The [blade settings contract](../../../../docs/specs/blade-settings.md), for the PocketKnives
Settings window: `rect`, `window`, `full` (hotkeys) and `flash` (toggle, on by default;
[capture](capture.md#flash)).

- Saved in `%APPDATA%\SnipSnap\snipsnap.ini`, section `[SnipSnap]`, through kernel32's
  `GetPrivateProfileIntW` / `WritePrivateProfileStringW`; the folder is made on the first `set`.
  A hotkey is `modifiers*256 + virtual key` in decimal. No file or no value: the defaults. A
  hotkey value that isn't valid counts as no value.
- Every run reads it at start: the background instance for its hotkeys and the flash, one-shot
  runs for the flash.
- In a preview (`POCKETKNIVES_SETTINGS_PREVIEW=1` in the environment) the file is `snipsnap.ini`
  next to the exe, and `set` never looks for or messages a running copy.
- Reload: the background instance's window handles the registered message
  `PocketKnives.Reload.v1`. It re-reads the file, unregisters its three hotkeys, registers them
  again as at start (so two captures can swap keys) and replies `0x100` plus bit 0/1/2 for
  rectangle/window/full screen left without a key.

## Command line: `snipsnap [command]`

The first argument is the command; only `set` reads the ones after it. Arguments are
case-insensitive, and leading `-` or `/` are ignored. They're parsed by hand, so shell32 is never
loaded.

| Command | Does |
|---|---|
| (none) | The background instance |
| `rect` | Select a rectangle once, then exit |
| `window` | Pick a window once, then exit |
| `full` | Copy the monitor under the cursor once, then exit |
| `quit` or `stop` | Stop the background instance: posts it `WM_CLOSE` and waits up to 3 s. Prints `SnipSnap stopped.` or `SnipSnap is not running.`; exit 0 either way |
| `settings` | Prints the four settings, one line each, as the contract says; exit 0. Read-only. A hotkey with no saved value shows the first of its list |
| `set key=value ...` | Checks every pair (known key; a number; a hotkey with a key and only Alt, Ctrl, Shift or Win; no two captures on one key; `flash` 0 or 1), then saves them and applies them. Prints one line and exits 2 if anything's wrong, changing nothing. With no pairs it just re-applies the file |
| `help`, `h` or `?` | Usage; exit 0 (the usage doesn't list `settings` and `set`) |
| anything else | Usage; exit 2 |

- Exit codes: `0` copied (or ok), `1` cancelled, `2` failed or bad usage.
- `set` applying: with the background instance running (and not a preview) it writes the file and
  sends the reload message (`SendMessageTimeoutW`, 5 s). Otherwise it registers the hotkeys itself
  for a moment, the same way. If a capture being set got no key, the previous values go back in
  the file (a value that wasn't there is removed), the instance reloads again, and `set` prints
  `<key> is taken by another program.` If the instance doesn't answer (an older build, or hung):
  the file is put back and it prints `Couldn't apply the settings.` Its other lines:
  `Not a valid setting: <pair>` and `<key> is already used by SnipSnap.`
- One-shot runs need no background instance and work alongside it. They exit about 150 ms after
  the image is on the clipboard, once the flash has faded.
- It's a GUI program: PowerShell doesn't wait for it unless it's piped (`snipsnap full | Out-Null`)
  or started with `Start-Process -Wait`.
