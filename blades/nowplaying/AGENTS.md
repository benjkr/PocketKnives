# AGENTS.md: NowPlaying

NowPlaying: a Windows 11 taskbar strip that shows the current media's album cover, app logo,
title/artist, live 5-band audio frequency meter, transport controls, and a hover-reveal seekable
timeline. Rust, `no_std` on `pk`, shipped as one small exe with no runtime.

**The suite's [`AGENTS.md`](../../AGENTS.md) comes first**: its rules hold here too. Paths and
commands here and in this blade's `docs/` are from `blades/nowplaying/`.

## Rules for NowPlaying

- **The exe**: `no_std` on `crates/pk`; WinRT SMTC, WASAPI loopback, Direct2D, DirectWrite and
  WIC through the workspace's `crates/pk-windows` (generated `windows-bindgen` bindings, no
  hand-written vtables), plain Win32 through `windows-sys`. No new crates without the user's say.
- **Zero-flicker taskbar docking**: the layered window is owned by `Shell_TrayWnd`
  (`GWLP_HWNDPARENT`), uses `WS_EX_NOACTIVATE` + `MA_NOACTIVATE` on left-clicks, and sits flush to
  the taskbar's left/top/bottom edges for the hover area while the `344×40` strip content keeps
  equal left/top/bottom padding (`(tb_height - 40) / 2`) with a flush `40×40` cover.
- **Zero-jump scrubbing**: the progress bar track stays a fixed `4 px` centered inside its `14 px`
  hit area; hover/drag only animates color, ghost fill and knob opacity/scale.
- **Budget**: one exe under 0.5 MB importing only Windows DLLs; idle CPU 0% when paused/hidden
  (loopback capture stops when paused or hidden).

## Running things

- Build in scratch (foreground after warm-up, ~5 s; ~25 s clean):
  `cargo build --release -p nowplaying --target-dir <dir>`
- Visual check without touching the live taskbar:
  `<dir>\release\NowPlaying.exe --preview-render <out.png>`
- Read-only SMTC check against the active player:
  `<dir>\release\NowPlaying.exe --status`

## Verification

- `cargo build --release -p nowplaying --target-dir <dir>` succeeds with zero warnings.
- `--version`, `--help`, `--settings`, `--set` (with `POCKETKNIVES_SETTINGS_PREVIEW=1`),
  `--status`, and `--preview-render` all exit cleanly and produce expected output.
- Inspect the rendered PNG from `--preview-render` on visual changes.

## Specs

| Area | Spec |
|---|---|
| Look, SMTC, WASAPI frequency meter, taskbar docking, settings and CLI | [nowplaying](docs/specs/nowplaying.md) |
