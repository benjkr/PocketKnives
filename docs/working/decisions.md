# Decisions for the whole suite

Check these before changing how the suite is organised; don't reintroduce what was replaced. Each
blade's own decisions are in its `docs/working/decisions.md`.

- **Name and terms**: the suite is PocketKnives, and each tool in it is a *blade* (`blades/<name>`
  in folders, docs and code).
- **The original repos stay as they were**, as a fallback: `quicksnip` (SnipSnap) and
  benjkr/quicktooth. The blades were imported with their full history
  (`git filter-repo --to-subdirectory-filter`), so nothing is lost if they're never touched again.
- **A settings window** that changes the behaviour, looks and settings of each blade, with each
  blade's logo at the top right of its page (the PocketKnives logo on General). On the left, General
  alone, then a "Blades" list: each blade's icon and a box (a switch in Windows 11) for "starts when
  you sign in", so that setting isn't on General. Rejected for that list: General indented with no
  box, General renamed PocketKnives with its own box, the boxes on the right. It has **three looks,
  switched with a setting** (General > Look): **XP**, the raw look (in the user's words: minimal,
  nothing fancy, brutalist, lists on the side, native Windows stuff); **Windows 10**, the same
  native controls with visual styles on; **Windows 11**, the Windows 11 Settings style (glassy,
  modern) in WinUI 3. A Windows 7 look drawn by hand was dropped. The suite icon in the taskbar's
  hidden icons section opens this window. See the [runner spec](../specs/runner.md).
- **One installer for the suite**: `PocketKnives-Setup` (Inno Setup, per user) installs every
  blade into `%LOCALAPPDATA%\Programs\PocketKnives`, each still its own program, with one entry in
  installed apps ([installer](../specs/installer.md)). The blades' separate installs were removed
  from the user's PC.
- **Blade settings come from the blade, with no manifest**: each blade prints its settings (key,
  kind, value, label, choices) and changes them itself
  ([blade settings](../specs/blade-settings.md)), so a setting is defined once, in the blade's
  code. The `blade.json` manifest proposed in the runner design was not built. The settings
  offered are the runner design's list: SnipSnap's three hotkeys and its flash, QuickTooth's
  headset and low-battery level.
- **Blades are written in Rust**, `no_std`, each its own small exe on the shared `crates/pk`
  (entry point, heap, arguments, console, single instance, threads); COM and WinRT through the
  `windows` crate. QuickTooth was ported from C# / .NET Native AOT: its exe went from 5.4 MB to about
  0.3 MB with the same behaviour, log lines and pixels, checked against recordings of the C# build.
  The runner and its settings windows stay C# (WinUI 3 has no Rust equivalent worth its cost).
  Rejected: C# Native AOT as the default for blades (a 0.85 MB, 1.2 MB floor per blade), and Rust
  with std (about 100 KB more per exe).
- **How blades run together** beyond the runner and settings contract that exist: being decided
  ([runner design](runner-design.md)).
