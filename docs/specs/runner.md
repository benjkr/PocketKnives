# PocketKnives runner

- The [installer](installer.md) starts `pk.exe` at sign-in. It registers one PocketKnives icon
  in the Windows notification area, available under the hidden icons arrow unless the user pins
  it to the taskbar. Windows retains the user's placement. The icon comes back after Explorer
  recreates the taskbar.
- Running `pk.exe` (or `pk settings`) ensures every enabled blade is running, whether the runner is
  starting fresh or already running: for each blade whose sign-in entry is enabled, it checks the
  blade's single-instance mutex and starts the blade's exe if it isn't already running (disabled
  blades stay off).
- One click, Enter, or Space on the icon opens PocketKnives Settings or restores its existing
  window. Right-click, Shift+F10 or the menu key on the icon shows **Settings** (the default
  item) and **Exit**. Exit closes Settings, the icon and the running blades, each through its own
  stop command (`snipsnap quit`, `QuickTooth.exe --exit`, `NowPlaying.exe --exit`). The installer's
  `pk --exit` closes only Settings and the icon; Setup stops the blades itself. The Start menu
  shortcut runs `pk settings` and opens the same window, and starts the runner and enabled blades
  again after Exit; any blade can also be started on its own from its Start menu entry. Closing
  Settings leaves the icon available.
- General offers **Look**. Choosing XP, Windows 10, or Windows 11 saves the look under
  `HKCU\Software\PocketKnives` and reopens Settings on General. XP uses classic Win32
  controls and Windows 10 enables visual styles. Windows 11 is a separate WinUI window with Mica
  where Windows supports it. The runner opens the Windows 10 look if WinUI fails to start.
- The left pane shows General and a Blades list with the name and icon of each blade. Each blade's
  check box or switch controls its existing sign-in entry. The window edits SnipSnap's Startup
  shortcut and uses QuickTooth's and NowPlaying's `--autostart` commands for their Run entries. Windows launches
  enabled blades at the next sign-in.
- Each blade's page shows that blade's own settings, read from the blade when Settings opens and
  changed through it ([blade settings](blade-settings.md)): SnipSnap's six hotkeys and the
  flash, QuickTooth's headset, low-battery level and tray icon, NowPlaying's frequency meter. A hotkey is a hotkey box in XP and Windows
  10 (Alt+key goes to the box, not to the window's access keys; the box can't take PrtSc or Win,
  and keeps a saved key it can't show). In Windows 11 the hotkey is a button: click it, press the
  keys, Esc cancels; Ctrl, Alt or Win is required. When a blade refuses a change (a hotkey taken
  by another program), Settings shows the blade's reason and the blade keeps its old value; Windows
  11 puts the control back, XP and Windows 10 leave the edit in place to fix or Cancel.
- All three looks size the window on open to fit the tallest and widest blade page (keeping the
  default minimum size and capping to 85% of the monitor's work area). If a page still overflows
  the capped height, the page area scrolls vertically while the header and buttons stay fixed.
- A `picture` setting is a row of buttons, each its picture above its label, with a frame in the
  highlight colour (Windows 11: the accent colour) around the chosen one that slides to a new
  pick. XP and Windows 10: push-like radio buttons (arrow keys move between them) and four thin
  statics for the frame, moved by a timer; the window clips its children so the frame never
  flickers over the buttons. Windows 11: buttons with the picture tinted like the text (dark
  mode too) and a frame that slides by an implicit animation. With a picture missing, the setting
  is a drop-down, like a `choice`. Sizes are constants in `runner/win/src/lib.rs` and
  `runner/winui/src/ui.rs`.
- XP and Windows 10 save changes with OK or Apply, and apply pending changes before switching
  looks. Windows 11 applies each change at once.

## Build and preview

Build with [the suite installer script](installer.md). It stages the runner (`runner/win`,
`pk.exe`) and its Windows 11 settings app (`runner/winui`) under `installer/obj/app`. Both are
`no_std` Rust and share `runner/core`. The WinUI app carries the Windows App SDK runtime, which
`runner/winui/stage.ps1` lays out from pinned, hash-checked NuGet packages, and starts only for
that look.

From a scratch copy of that stage, run `pk --preview`, then click the tray icon or run
`pk --preview settings`. Preview instances use their own tray identity and save changes to
`preview-settings.txt` next to the scratch executable, and the scratch blades keep their settings
next to their own exes ([blade settings](blade-settings.md#preview)). They never write the
installed settings, start blades or change what starts with Windows, and their Exit leaves the
blades running.
To close the preview and its tray icon, run `pk --preview --exit` from the same folder.
