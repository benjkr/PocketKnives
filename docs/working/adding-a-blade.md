# Adding a blade

Read before starting a new blade or importing an existing tool as one. How blades are launched
together, what code they share and which language a new blade uses by default aren't decided yet
([runner design](runner-design.md)). This page says what's true today; it gains the runner's part
once the user picks one. Until then, ask the user which language a new blade is written in. A
settings window is decided: every blade will get a page in it, with its icon and its settings
(how they're declared is part of the runner design).

## Folder layout

```text
blades/<name>/              one lower-case word: the folder and exe name
  AGENTS.md                 what's specific to this blade (rules, budget, checks, routing)
  README.md                 for people: what it does, how to use and build it
  assets/icon.svg           its logo, shown in the root README
  docs/specs/<area>.md      what each area does now
  docs/working/<topic>.md   only the blade-specific working rules it needs
  .gitignore                its own build output
  …                         source and the build script
```

## What a blade provides

- **One program and one build command** with zero warnings, which an agent can point at a scratch
  folder without touching the user's installed copy (QuickTooth: `dotnet publish … -o <dir>`;
  SnipSnap: its `cl` line with `/Fo` and `/OUT:`).
- **One instance per user session**, found by its window class or mutex, so a second start exits
  and a command reaches the running copy.
- **A command line**: its one-shot actions, a way to stop the running copy, help text, and exit
  codes. Its spec lists them; scripts and launchers depend on them ("log lines are behaviour").
- **A budget** in its `AGENTS.md` (exe size, background memory, the latency of its main action),
  and how to measure each one.
- **Its checks**: what to run for which change, and what each check proves.
- **A preview**: how a change is shown to the user before it ships.
- **An install location** (`%LOCALAPPDATA%\Programs\<Name>`) and how it starts with Windows,
  listed in [live testing](live-testing.md).

## Checklist

1. Work on a branch (`t3/<name>-blade`). Create the folder layout above.
2. Write the blade's `AGENTS.md` in the shape of the others: one paragraph on what it is, then
   a link to the root `AGENTS.md`, its own rules, budget, checks, and tables for its working rules
   and specs. Nothing that the root already says.
3. Write its README and lean specs (each says what the area does now).
4. Add the blade to the root: a row in the blades table in `AGENTS.md`, a section in `README.md`
   (icon, one line, link to its README) and, once it's installed, a row in
   [live testing](live-testing.md).
5. Build it into a scratch folder with zero warnings and run its checks.
6. Importing an existing repo: clone the one branch into a scratch folder, rewrite it with
   `git filter-repo --to-subdirectory-filter blades/<name>`, fetch it and merge with
   `--allow-unrelated-histories`. Check that `HEAD:blades/<name>` has the same tree hash as the
   source branch and that `git log --follow` shows a file's old commits. Leave the original repo
   as it is, and say in the README where the history came from.
