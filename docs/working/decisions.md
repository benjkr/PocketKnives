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
  modern) in WinUI 3. A Windows 7 look drawn by hand was dropped. How it's built and which settings
  it offers are still open ([runner design](runner-design.md)).
- **How blades run together, shared code and the default language** for new blades: being decided
  ([runner design](runner-design.md)).
