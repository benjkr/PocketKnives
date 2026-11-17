# PocketKnives runner

- The [installer](installer.md) starts `pk.exe` at sign-in. It registers one PocketKnives icon
  in the Windows notification area, available under the hidden icons arrow unless the user pins
  it to the taskbar. Windows retains the user's placement. The icon comes back after Explorer
  recreates the taskbar.
- One click, Enter, or Space on the icon opens PocketKnives Settings or restores its existing
  window. The Start menu shortcut runs `pk settings` and opens the same window. Closing Settings
  leaves the icon available. The installer's `pk --exit` command closes both.
- General offers **Look**. Choosing XP, Windows 10, or Windows 11 saves the look under
  `HKCU\Software\PocketKnives` and reopens Settings on General. XP uses classic Win32
  controls and Windows 10 enables visual styles. Windows 11 is a separate WinUI window with Mica
  where Windows supports it. The runner opens the Windows 10 look if WinUI fails to start.
- The left pane shows General and a Blades list with the name and icon of each blade. Each blade's
  check box or switch controls its existing sign-in entry. The window edits SnipSnap's Startup
  shortcut and uses QuickTooth's `--autostart` command for its Run entry. Windows launches
  enabled blades at the next sign-in. XP and Windows 10 save changes with OK or Apply, and apply
  pending changes before switching looks. Windows 11 switches apply immediately.

## Build and preview

Build with [the suite installer script](installer.md). It stages the runner and its Windows 11
settings app under `installer/obj/app`. Both are Native AOT and use the same .NET runtime pin as
QuickTooth. The WinUI app carries its Windows App SDK runtime and starts only for that look.

From a scratch copy of that stage, run `pk --preview`, then click the tray icon or run
`pk --preview settings`. Preview instances use their own tray identity and save changes to
`preview-settings.txt` next to the scratch executable. They never write the installed settings
or change what starts with Windows. To close the preview and its tray icon, run
`pk --preview --exit` from the same folder.
