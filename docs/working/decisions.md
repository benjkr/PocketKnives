# Decisions for the whole suite

Check these before changing how the suite is organised; don't reintroduce what was replaced. Each
blade's own decisions are in its `docs/working/decisions.md`.

- **Name and terms**: the suite is PocketKnives, and each tool in it is a *blade* (`blades/<name>`
  in folders, docs and code).
- **The original repos stay as they were**, as a fallback: `quicksnip` (SnipSnap) and
  benjkr/quicktooth. The blades were imported with their full history
  (`git filter-repo --to-subdirectory-filter`), so nothing is lost if they're never touched again.
- **A settings window** that changes the behaviour, looks and settings of each blade. In the
  user's words: minimalistic, Windows XP/7-style graphics, a native look (use a native library),
  and each blade's icon in it. How it's built, which native look and which settings it offers:
  being decided ([runner design](runner-design.md)).
- **How blades run together, shared code and the default language** for new blades: being decided
  ([runner design](runner-design.md)).
