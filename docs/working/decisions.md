# Decisions for the whole suite

Check these before changing how the suite is organised; don't reintroduce what was replaced. Each
blade's own decisions are in its `docs/working/decisions.md`.

- **Name and terms**: the suite is PocketKnives, and each tool in it is a *blade* (`blades/<name>`
  in folders, docs and code).
- **The original repos stay as they were**, as a fallback: `quicksnip` (SnipSnap) and
  benjkr/quicktooth. The blades were imported with their full history
  (`git filter-repo --to-subdirectory-filter`), so nothing is lost if they're never touched again.
- **A settings window** changes each blade's behaviour, looks and settings. It's minimalistic, in
  the Windows XP/7 style, built from native Win32 controls (no UI framework, no hand-drawn
  chrome), and shows each blade's icon. Its themed or classic look and which settings it offers:
  being decided ([runner design](runner-design.md)).
- **How blades run together, shared code and the default language** for new blades: being decided
  ([runner design](runner-design.md)).
