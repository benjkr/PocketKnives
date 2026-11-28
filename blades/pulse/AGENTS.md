# AGENTS.md: Pulse

Pulse: a calculator-style system monitor bar along the top of the main screen. CPU, memory,
GPU, disk and network as seven-segment readouts over dot-matrix graphs on one LCD glass, and the
time; hovering a metric shows its last minute. Rust, `no_std` on `pk`, one small exe.

**The suite's [`AGENTS.md`](../../AGENTS.md) comes first**: its rules hold here too. Paths and
commands here and in this blade's `docs/` are from `blades/pulse/`.

## Rules for Pulse

- **The exe**: `no_std` on `crates/pk`, plain Win32 through `windows-sys`. It draws with its own
  rasterizer (`src/raster.rs`) into the layered window's pixels: no Direct2D, no COM. No new crates
  without the user's say.
- **A theme is only colours**: `themes/*.json` and user theme files hold colour keys and nothing
  else. A look that differs by theme is a colour key, never a flag, an offset or a theme's name
  in the code. A new key goes in every theme file and in the spec's table.
- **Nothing moves when values change**: every readout has fixed cells and every graph fixed
  columns. A change to a value's format keeps its width.
- **The geometry is the approved mocks'** (the bar "M2", the process list "P4" with two presses
  to kill and the colour screen of "S2"; one design pixel = one CSS pixel); show a visible
  change with `--preview-render` before and after.
- **Lettering on the plastic is Arial Bold** (`src/letters.rs`), like a calculator's printed
  labels; text on glass is the LCD's pixel font or seven segments.
- **What can change is on glass, what is fixed is engraved**, and a key only travels when
  pressed: it never changes colour or legend; states show on the glass.
- **Ending programs is tested only on a throwaway process** you started (a renamed copy of a
  system tool), never on the user's programs: one kill ends every process with that name.
- **Budget**: one exe under 0.25 MB importing only Windows DLLs; a second's sample and redraw
  under 1% of one core.
- **Live runs**: a running bar reserves the top of the screen and moves the user's windows. Don't
  start one on the user's desktop without asking; use `--preview-render`.

## Running things

- Build in scratch: `cargo build --release -p pulse --target-dir <dir>`
- Visual check, nothing live: `<dir>\release\Pulse.exe --preview-render <out.png> --theme navy --scale 2 [--panel cpu]`
- Live readings, read-only: `<dir>\release\Pulse.exe --status 5`

## Verification

- `cargo build --release -p pulse --target-dir <dir>` with zero warnings.
- `--version`, `--help`, `--settings`, `--set` (with `POCKETKNIVES_SETTINGS_PREVIEW=1`),
  `--status 3`, `--top 2` and `--preview-render` for every theme exit cleanly with the expected
  output.
- On visual changes, look at `--preview-render` PNGs for every theme at `--scale 1` and `2`, with
  a panel (and the list's states: `--select`, `--deny`, `--press`).

## Specs

| Area | Spec |
|---|---|
| Window, look, themes, readings, settings and CLI | [pulse](docs/specs/pulse.md) |
