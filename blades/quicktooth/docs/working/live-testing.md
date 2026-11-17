# Real devices and the installed QuickTooth

Read before anything that talks to real Bluetooth devices, or installs, starts or stops
QuickTooth. The suite-wide rules (the installed blades, the desktop) are in
[live testing](../../../../docs/working/live-testing.md).

## Bluetooth devices

- The user's PC has other paired Bluetooth audio devices, a hearing aid among them. **Only the
  stored target may ever receive a connect or disconnect.** Check the log after any live run: every
  `KsProperty` line names one of the target's two IDs.
- Always fine, because they send nothing: `--list`, `--probe`, `--battery`, `--watch-endpoints`,
  `--stress-enumerate` (it activates KS controls but sends no property), `--render-icons` and
  `--selftest-icons`.
- **A live connect or disconnect needs the user's go in the current request.** First:
  1. Warn them (`send_to_user`) that the AirPods will connect and disconnect and must be out of the
     case.
  2. Agree how many cycles.
  3. Leave the AirPods connected or disconnected as they were.
- Live timing varies (one connect may take 4 s); compare several cycles, or the log's history,
  before calling something slower.

## The installed app

- Starting the tray from another path (a scratch build) adds a stray `NotifyIconSettings` entry, so
  ask the user first.
- A tray CLI command (`--exit`, `--toggle`, `--connect`, `--disconnect`, `--show-menu`) acts on the
  running tray exactly like a click, from any copy of the exe. `--autostart on|off` writes the Run
  value.
- `--settings` sends nothing and writes nothing: safe from any copy, but a scratch copy runs it with
  `POCKETKNIVES_SETTINGS_PREVIEW=1` so it reads its own `settings.json`. `--set` changes the user's
  settings (through the running tray, from any copy): from a scratch copy only with
  `POCKETKNIVES_SETTINGS_PREVIEW=1`, after checking that `--settings` reads the planted file.
- Before an install test, record three things:
  - the Run value `HKCU\…\Run\QuickTooth`;
  - the hash of `%APPDATA%\QuickTooth\settings.json`;
  - QuickTooth's `NotifyIconSettings` `IsPromoted` value.

  Afterwards, check all three are back and the tray is running, and say so.
- The taskbar may be hidden or the PC locked. Look at the tray in the simulator instead of
  guessing from the screen.
