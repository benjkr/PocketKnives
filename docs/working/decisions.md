# Decisions for the whole suite

Check these before changing how the suite is organised; don't reintroduce what was replaced. Each
blade's own decisions are in its `docs/working/decisions.md`.

- **Name and terms**: the suite is PocketKnives, and each tool in it is a *blade* (`blades/<name>`
  in folders, docs and code).
- **The original repos stay as they were**, as a fallback: `quicksnip` (SnipSnap) and
  benjkr/quicktooth. The blades were imported with their full history
  (`git filter-repo --to-subdirectory-filter`), so nothing is lost if they're never touched again.
- **A settings window** that changes the behaviour, looks and settings of each blade, with each
  blade's icon in it. In the user's words: a raw look, minimal, nothing fancy, brutalist; simple,
  maybe lists on the side; native Windows stuff. (This replaced asking for an XP/7 look, and a
  Windows 7 look drawn by hand was dropped.) The user then asked about a Windows 11 Settings-style
  look (glassy, modern), so which look it has is open again, as are how it's built and which
  settings it offers ([runner design](runner-design.md)).
- **How blades run together, shared code and the default language** for new blades: being decided
  ([runner design](runner-design.md)).
