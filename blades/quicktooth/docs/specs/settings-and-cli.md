# Settings, log and command line

Part of QuickTooth's specs (index in [`AGENTS.md`](../../AGENTS.md)). What the app does now. Keep it current in the same
change, and lean.

## Settings: `%APPDATA%\QuickTooth\settings.json`

- `targetContainerId` (primary), `targetMac` (fallback), `targetName` (display only),
  `trayPromotedOnce` and, optionally, `lowBatteryPercent` (the low-battery warning; absent means
  the default in `core/src/tunables.rs`) and `icon` (the tray icon's artwork, `airpodsPro` or
  `airpods`; absent or unknown means AirPods Pro). A file without them round-trips unchanged. Read and written
  exactly as System.Text.Json did (`core/src/settings.rs`: camelCase, nulls left out, indented,
  relaxed escaping, CRLF in the exe); saved through a temp file and a move.
- A stored target is never swapped for another device: if it's missing, the tray says
  "Not available". With no target saved yet, the first paired device whose name contains
  "AirPods" (a connected one first) is chosen and saved.

## Changing them from PocketKnives Settings

QuickTooth's side of the suite's [blade settings contract](../../../../docs/specs/blade-settings.md),
in `core/src/blade_settings.rs`:

- `--settings` prints three `choice` settings: `device` ("Headset": container IDs in `B` format,
  labelled with the devices the tray's Device menu offers; the value is the stored target, empty
  if none; a stored target that's missing is listed as `<name> (not available)`, as in the menu),
  `lowBattery` ("Low battery warning": 10, 15, 20, 25 or 30 %) and `icon` ("Tray icon":
  `airpodsPro` "AirPods Pro" or `airpods` "AirPods"). It enumerates like the tray
  and writes nothing; about as fast as `--list`.
- `--set key=value...` checks every pair first (form, key, battery and icon value); a bad one exits 2 with
  one line and changes nothing. The headset already chosen is no change, even while missing.
- With the tray running, the pairs go to it as `WM_COPYDATA` (`SendMessageTimeout`, 5 s): it saves
  its whole file, so it applies the change itself ([tray](tray.md)) and answers with the result.
  No answer (an older tray answers 0, or a timeout) exits 1 with one line, and the CLI never
  writes the file itself then. With no tray, the
  CLI checks the headset against a fresh enumeration, applies and saves. Exit 0: saved (nothing is
  written when nothing changes).
- Logged: `low battery warning set: N%`, `tray icon set: AirPods` (or `AirPods Pro`, followed by
  `icons rebuilt: …` in a running tray) and the Device menu's `target set: …`.
- `POCKETKNIVES_SETTINGS_PREVIEW=1`: both use `settings.json` and `quicktooth.log` next to the
  exe and never message a running tray.

## Log: `%APPDATA%\QuickTooth\quicktooth.log`

- `yyyy-MM-dd HH:mm:ss.fff [pid] message`, local time, CRLF, moved to `.old` at 512 KB. Shared code
  logs through `core/src/log.rs` (its sink is set by each host).
- The lines are compared across builds and with the simulator's log, so treat their wording as
  behaviour (`AGENTS.md`).
- `ERROR <context>: <text>` lines carry a short message (the C# build wrote .NET's exception and
  stack). A bug ends the process after `ERROR panic: QuickTooth stopped on a bug`; the C# build's
  `ERROR wndproc …`, `ERROR ui callback …` and `read endpoint: …` lines (caught exceptions) are
  gone. Failures the C# build only showed as exceptions log `ERROR icon`, `ERROR autostart`,
  `ERROR delay`, `ERROR device worker`, `ERROR battery scan` or `ERROR tray`.

## Command line: `QuickTooth.exe [option]` (no option starts the tray)

- `--version` prints `QuickTooth <version>  (Rust, no_std)`.
- For people: `--toggle`, `--connect`, `--disconnect` (forwarded to the running tray, so they act
  exactly like a click; `--no-wait` returns at once), `--list [--all]`, `--battery`,
  `--autostart on|off|status`, `--exit`, `--version`, `--help`.
- For PocketKnives Settings: `--settings` and `--set key=value...` (above).
- Read-only checks for agents and tests: `--probe <endpoint id>`, `--render-icons <dir> [--zoom z]`,
  `--icon-rect`, `--show-menu` / `--close-menu`, `--settings-roundtrip <in> <out>`,
  `--stress-enumerate N`, `--watch-endpoints S`, `--selftest-icons [cycles]`
  ([testing](testing.md) says what each proves).
- Experiments: `--direct` (send without the tray) and `--a2dp-only` (A2DP control only; slow, see
  [decisions](../working/decisions.md)).
- Start with Windows is the value `HKCU\Software\Microsoft\Windows\CurrentVersion\Run\QuickTooth`
  = `"<exe path>"`.
