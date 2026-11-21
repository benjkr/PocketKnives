# The PocketKnives installer

What it does now. Keep it current in the same change, and lean.

- **One per-user installer for the runner and every blade**: `installer/build.ps1` builds them from this
  checkout and wraps them with Inno Setup 6.7+ (`winget install JRSoftware.InnoSetup`) into
  `dist/PocketKnives-Setup-<version>.exe`. The version is `AppVersion` in
  `installer/PocketKnives.iss`. It builds into `installer/obj` only: it stops nothing and writes
  nothing to the blades' folders (SnipSnap through `build.bat <dir>`). The Windows App SDK
  packages for the Windows 11 settings app are downloaded from nuget.org on the first build,
  checked against their pinned SHA-512 and cached in `%TEMP%\pocketknives-winappsdk`
  (`runner/winui/stage.ps1`).
- **What it installs**: the [runner](runner.md), its Windows 11 settings app, each blade's exe and
  the icons Settings shows (each blade's, and the pictures a blade draws for its `picture`
  settings, see [blade settings](blade-settings.md)) into `%LOCALAPPDATA%\Programs\PocketKnives`,
  without an admin prompt. The Start menu's
  PocketKnives folder contains PocketKnives Settings and one entry per blade. Windows lists one
  PocketKnives app.
- **Start when I sign in**: the runner starts at sign-in through a PocketKnives Startup shortcut
  and shows its tray icon. Setup offers a checked box for each blade that isn't
  installed yet; an update leaves it as the user set it. SnipSnap through a `SnipSnap` shortcut
  in `shell:startup`, QuickTooth and NowPlaying through their own `--autostart on` (their Run
  values).
- **Updating**: Setup stops the runner and running blades with their own commands, force-closes one
  still running after 10 s, and starts them again at the end, silent installs included.
- **Uninstalling** stops the runner and blades and removes the installed files, Start menu entries,
  Startup shortcuts and QuickTooth's and NowPlaying's Run values (only if they point into the
  install). The saved window look and the blades' settings (`%APPDATA%\SnipSnap`,
  `%APPDATA%\QuickTooth`, `%APPDATA%\NowPlaying`) are kept.
- Unsigned, so Windows SmartScreen warns on download.
- **What goes on the user's PC**: only builds of `master`, from a release tag once tags exist. Say
  which commit is installed. The blades' own stand-alone installs (QuickTooth's `publish.ps1` and
  installer, SnipSnap's folder) would run beside this one: they aren't used on the user's PC.
- **Adding a blade**: build it into the stage in `build.ps1` (with its setting pictures, if it
  has `picture` settings), and give it a `[Files]` line, a Start
  menu entry, a "start when I sign in" task and a stop command in `PocketKnives.iss`.
