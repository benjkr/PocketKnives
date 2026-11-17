# AGENTS.md

PocketKnives: a suite of small Windows quality-of-life tools. Each tool is a **blade**, in its own
folder `blades/<name>/`. How the blades will run together isn't built yet
([runner design](docs/working/runner-design.md)); today each blade is its own program.

**Read this whole file before changing anything.** Then read the `AGENTS.md` of the blade you
touch, the spec for the area you touch and the working rules that apply (tables below and in the
blade's file); don't load unrelated documents. Keep this file, the working rules and the specs
current in the same change as the code. Paths here and in `docs/` are from the repo root; paths
in a blade's docs are from that blade's folder.

## Blades

| Blade | What it does | Built with | Start here |
|---|---|---|---|
| SnipSnap | Copies a rectangle, a window or the screen to the clipboard from a hotkey | One C file, no CRT, MSVC | [`blades/snipsnap/AGENTS.md`](blades/snipsnap/AGENTS.md) |
| QuickTooth | Connects or disconnects AirPods with one click on a tray icon | C# / .NET 10, Native AOT; browser simulator | [`blades/quicktooth/AGENTS.md`](blades/quicktooth/AGENTS.md) |

## Rules every change follows

- **One source of truth.** No copies of logic, values or text, no configuration for things that
  never vary, and no special case unless the platform forces it, with a comment saying which
  platform quirk. A rule or fact is written once: here if it holds for every blade, in the
  blade's docs if it doesn't. A blade's code, assets and docs stay in its folder.
- **Show it before it ships.** A change to what a blade looks like or does is shown to the user
  before it's merged, the way that blade's `AGENTS.md` says (QuickTooth: the simulator), with
  before/after when it replaces something.
- **Leave the user's system alone**: no theme changes, no Explorer restarts, screenshots of only
  the area that matters, never the whole desktop. The user's installed blades keep running: don't
  stop, replace or reconfigure them. After any install test, their autostart, settings and tray
  state are back as they were ([live testing](docs/working/live-testing.md)).
- **Only `master` is installed on the user's PC** (tagged releases once they exist). Never install
  a branch, an open PR or anything unmerged. Unmerged work is previewed, or checked read-only from
  a build in a scratch folder.
- **Native and AOT builds stay clean**: zero compiler warnings (C builds with `/W4 /WX`), zero
  AOT/trim warnings, no reflection in .NET, and each blade's size and memory budget holds. Security
  pins (such as QuickTooth's runtime patch pin) never go backwards.
- **Log lines are behaviour.** A blade's log lines, console output and exit codes are compared
  across builds or read by scripts and launchers; change their wording only on purpose, with
  whatever reads them.
- Work on a named branch, never on `master`. Commit finished work; open PRs when asked; merge only
  when the user asks in that request ([pull requests](docs/working/pull-requests.md)).
- Project knowledge lives in these documents, not in agent memory: auto-memory is off for this
  repo (`.claude/settings.json`). Don't read or write it.

## Token and execution policy

- Routine work (docs, scripts, tests, mechanical edits) needs no top model or high effort; keep
  those for rendering, interop, timing races and architecture. Honour the user's choice.
- One implementation agent by default. Delegate only independent work with a narrow brief, and
  never give two agents the same files.
- Search first (`rg`) and read the section you need; batch independent calls; keep bulky output
  out of context (logs to files, `| tail`). Don't re-plan settled decisions.
- Builds and test runs of about a minute run in the foreground (each blade's development doc
  lists its own). Anything longer, such as real-time recordings, runs in the background with
  `t3_background_wait`. Never sleep-poll.
- Edit with small targeted edits; a script only for one change across many files.
- No narration between tool calls unless the plan changed or the user is needed.

## Verification and delivery

- Run the checks the touched blade's `AGENTS.md` lists, on the final code; a change to two blades
  runs both blades' checks.
- A refactor that shouldn't change behaviour: record the blade's observable output before (logs,
  rendered images, console output, exit codes) and diff it after: identical, line for line.
- Visible changes come with media; numbers that compare are shown as a chart, never a table.
  Before the final reply of a substantial task read [replies](docs/working/replies.md).

## Working rules: load when applicable

| Work | Document |
|---|---|
| Building, tools, this machine's shell quirks, delegating | [Development](docs/working/development.md) |
| Real devices, the installed blades, the user's desktop and clipboard | [Live testing](docs/working/live-testing.md) |
| Changing something decided for the whole suite | [Decisions](docs/working/decisions.md) |
| Final reply of a task; showing numbers | [Replies](docs/working/replies.md) |
| Commits, branches, PRs, merges, releases, GitHub comments | [Pull requests and releases](docs/working/pull-requests.md) |
| Adding a new blade | [Adding a blade](docs/working/adding-a-blade.md) |
| How blades run together, the settings window, shared code, the default language (not decided yet) | [Runner design](docs/working/runner-design.md) |

Each blade's `AGENTS.md` has its own tables for its working rules and specs.

## Keeping documentation lean

This file holds the rules for every blade and the routing; `docs/working/` holds suite-wide rules
that apply to some kinds of work; each blade's `AGENTS.md` holds what's specific to that blade and
routes to its own working rules and specs. Each spec says what its area does **now**. When
something changes, rewrite the bullet. History, measurements and discarded designs belong in
commits and PRs; a rejected choice goes in a decisions doc (the suite's or the blade's) only so it
isn't reintroduced. A new area gets a spec and a row in its blade's table; a new blade gets a row
in the blades table above. Defaults live in code, not in the docs' prose.

## Owner's notes

This section is written by the owner. Agents don't change it.
