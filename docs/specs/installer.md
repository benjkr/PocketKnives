# The PocketKnives installer

What it does now. Keep it current in the same change, and lean.

- **One per-user installer for every blade**: `installer/build.ps1` builds each blade from this
  checkout and wraps them with Inno Setup 6.7+ (`winget install JRSoftware.InnoSetup`) into
  `dist/PocketKnives-Setup-<version>.exe`. The version is `AppVersion` in
  `installer/PocketKnives.iss`. It builds into `installer/obj` only: it stops nothing and writes
  nothing to the blades' folders (SnipSnap through `build.bat <dir>`).
- **What it installs**: each blade's exe into `%LOCALAPPDATA%\Programs\PocketKnives`, still its own
  program (the runner isn't built: [runner design](../working/runner-design.md)), no admin
  prompt. A "PocketKnives" folder in the Start menu with one entry per blade, and one
  "PocketKnives" entry in Windows' installed apps.
- **Start when I sign in**: one box per blade, offered (ticked) only when that blade isn't
  installed yet; an update leaves it as the user set it. SnipSnap through a `SnipSnap` shortcut
  in `shell:startup`, QuickTooth through its own `--autostart on` (the Run value).
- **Updating**: Setup stops the running blades with their own commands (`snipsnap quit`,
  `QuickTooth.exe --exit`), force-closes one still running after 10 s, and starts both again at
  the end, silent installs included.
- **Uninstalling** stops both and removes the folder, the Start menu entries, SnipSnap's Startup
  shortcut and QuickTooth's Run value (only if it points into the install). QuickTooth's settings
  in `%APPDATA%\QuickTooth` are kept.
- Unsigned, so Windows SmartScreen warns on download.
- **What goes on the user's PC**: only builds of `master`, from a release tag once tags exist. Say
  which commit is installed. The blades' own stand-alone installs (QuickTooth's `publish.ps1` and
  installer, SnipSnap's folder) would run beside this one: they aren't used on the user's PC.
- **Adding a blade**: build it into the stage in `build.ps1`, and give it a `[Files]` line, a Start
  menu entry, a "start when I sign in" task and a stop command in `PocketKnives.iss`.
