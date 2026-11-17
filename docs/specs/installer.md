# The PocketKnives installer

What it does now. Keep it current in the same change, and lean.

- **One per-user installer for the runner and every blade**: `installer/build.ps1` builds them from this
  checkout and wraps them with Inno Setup 6.7+ (`winget install JRSoftware.InnoSetup`) into
  `dist/PocketKnives-Setup-<version>.exe`. The version is `AppVersion` in
  `installer/PocketKnives.iss`. It builds into `installer/obj` only: it stops nothing and writes
  nothing to the blades' folders (SnipSnap through `build.bat <dir>`).
- **What it installs**: the [runner](runner.md), its Windows 11 settings app, and each blade's exe
  into `%LOCALAPPDATA%\Programs\PocketKnives`, without an admin prompt. The Start menu's
  PocketKnives folder contains PocketKnives Settings and one entry per blade. Windows lists one
  PocketKnives app.
- **Start when I sign in**: the runner starts at sign-in through a PocketKnives Startup shortcut
  and shows its tray icon. Setup offers a checked box for each blade that isn't
  installed yet; an update leaves it as the user set it. SnipSnap through a `SnipSnap` shortcut
  in `shell:startup`, QuickTooth through its own `--autostart on` (the Run value).
- **Updating**: Setup stops the runner and running blades with their own commands, force-closes one
  still running after 10 s, and starts them again at the end, silent installs included.
- **Uninstalling** stops the runner and blades and removes the installed files, Start menu entries,
  Startup shortcuts and QuickTooth's Run value (only if it points into the install). The saved
  window look and QuickTooth's settings are kept.
- Unsigned, so Windows SmartScreen warns on download.
- **What goes on the user's PC**: only builds of `master`, from a release tag once tags exist. Say
  which commit is installed. The blades' own stand-alone installs (QuickTooth's `publish.ps1` and
  installer, SnipSnap's folder) would run beside this one: they aren't used on the user's PC.
- **Adding a blade**: build it into the stage in `build.ps1`, and give it a `[Files]` line, a Start
  menu entry, a "start when I sign in" task and a stop command in `PocketKnives.iss`.
