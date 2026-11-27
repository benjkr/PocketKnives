# Real devices, the installed blades and the desktop

Read before anything that talks to real devices, installs, starts or stops a blade, or looks at
the user's screen. Each blade's `docs/working/live-testing.md` adds what's specific to it.

## The installed blades

| Blade | Installed at | Starts with Windows through |
|---|---|---|
| SnipSnap | `%LOCALAPPDATA%\Programs\PocketKnives\snipsnap.exe` | a shortcut in `shell:startup` |
| QuickTooth | `%LOCALAPPDATA%\Programs\PocketKnives\QuickTooth.exe` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Run\QuickTooth` |
| NowPlaying | `%LOCALAPPDATA%\Programs\PocketKnives\NowPlaying.exe` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Run\NowPlaying` |
| Readout | `%LOCALAPPDATA%\Programs\PocketKnives\Readout.exe` | `HKCU\Software\Microsoft\Windows\CurrentVersion\Run\Readout` |
| [Runner](../specs/runner.md) | `%LOCALAPPDATA%\Programs\PocketKnives\pk.exe` | a `PocketKnives` shortcut in `shell:startup` |

- All of them come from the PocketKnives installer ([installer](../specs/installer.md)), and only
  `master`'s builds go there (`AGENTS.md`). Don't replace, stop or restart them to try a branch.
  Run checks from a build in a scratch folder instead.
- A running blade is found by its window class or mutex, not its path. So a command run from
  **any** copy of a blade, such as `snipsnap quit` or `QuickTooth.exe --exit`, acts on the user's
  running copy. Each blade's live-testing doc lists the commands that are safe.
- The runner's Settings window writes the blades' real sign-in entries and the look under
  `HKCU\Software\PocketKnives`. A scratch `pk.exe` always runs with `--preview` first
  (`pk --preview`, `pk --preview settings`, `pk --preview --exit`). Without it, `pk settings`
  or `pk --exit` from any copy acts on the user's installed runner, and a scratch runner that
  starts first takes its tray icon and points Startup entries at the scratch folder.
- Before an install test, record the blade's autostart entry, its settings file's hash and its
  tray state (the blade's doc lists them). Afterwards check they're back, the blade is running,
  and say so. Then free the build folders ([development](development.md#building)).

## The desktop and the clipboard

- Never change the theme or restart Explorer. Screenshots only of the area that matters (the tray
  corner, say), never the whole desktop.
- A capture or paste test replaces the user's clipboard, and an overlay covers every monitor. Ask
  first.
- The taskbar may be hidden or the PC locked. Use a blade's preview (QuickTooth's simulator)
  instead of guessing from the screen.
