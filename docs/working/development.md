# Development workflow

Read before building, running tools, setting up a machine or delegating. Each blade's own setup,
commands and how long they take are in its `docs/working/development.md`.

## Building

- Build into a scratch folder when the output only serves a check. A blade's build script may
  also stop or replace the user's running copy: its development doc says which ones are safe
  (SnipSnap's `blades/snipsnap/build.bat` stops it).
- Build output stays out of git: each blade's `.gitignore` covers its own.
- The workspace links with Rust's own `rust-lld` (`.cargo/config.toml`). Visual Studio's C++
  tools are still needed for their libraries (build scripts and tests link them) and the
  Windows SDK's `rc.exe`.

## This Windows machine

- Git Bash and PowerShell 7 are both available. Use PowerShell for the registry, processes and
  installers.
  - PowerShell 7's `Start-Process -Wait` also waits for child processes, and an installer may
    start the app. Use `-PassThru` and `.WaitForExit()`.
  - Running a GUI exe with `&` doesn't wait either; use `Start-Process … -Wait`.
- Python's console is cp1252 unless `PYTHONUTF8=1` (set for agents in `.claude/settings.json`).
  `python -I` ignores environment variables, so call `sys.stdout.reconfigure(encoding='utf-8')`
  there.
- Write multi-line scripts to a file and run them. Quoted heredocs with mixed quotes break in Git
  Bash. Edit source with targeted edits, not scripts.

## Delegating

Allowed, for independent work with a narrow brief and the right model and effort (a fast model for
searches and mechanical edits, a strong one for interop and rendering). Never two agents on the
same files, and keep design work that depends on the conversation in the agent holding it.

## Knowledge

Project knowledge goes in `AGENTS.md` (the suite's and each blade's), `docs/working/` and the
specs. Auto-memory is off for this repo (`.claude/settings.json`): don't read or write it.
