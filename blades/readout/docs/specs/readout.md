# Readout

What it does now. Keep it current in the same change, and lean.

## Window

- **An app bar along the top of the main screen** (`SHAppBarMessage`, `ABE_TOP`): Windows keeps
  other windows out from under it, like the taskbar. Full screen width, 37 design pixels tall.
  A layered window (`UpdateLayeredWindow`) that never takes focus, with a 4-pixel shadow drawn
  under the reserved strip. It steps behind full-screen apps and comes back after them.
- **Scaling**: every design pixel is a whole number of device pixels, the nearest to the screen's
  scaling and at least 1 (100% and 125%: 1; 150% to 225%: 2; 250%: 3), so the dot matrix and the
  engraving stay sharp. The panels are half again the bar's size (at 100%, 1.5 device pixels a
  design pixel; at 150%, 3), half a step less at a time where the tallest wouldn't fit under the
  bar. A scale of a whole and a half is drawn at twice that and halved, so every design pixel
  spans the same device pixels.
- **Once a second** it samples the system and redraws. Hovering a metric shows its panel under
  it; the panel stays while the pointer crosses to it and is on it, and goes when the pointer
  leaves both. The panel never takes focus; its keys take clicks. Right-click the bar: Theme
  (every theme, the current one checked) and Exit.

## Look

The approved design "M2" (`src/bar.rs` holds its geometry; one design pixel = one CSS pixel of
the mock):

- A faceplate with, from the left, the metric tiles CPU, memory, GPU, disk and network (each can
  be hidden), then the clock at the right end.
- **Metric tiles share one surround and one glass**: the surround is a band along the bottom with
  a tower for each tile's digit window; the glass under all the graphs is one piece, with a dotted
  divider between metrics. Each metric's icon is engraved in the faceplate in the gap left of its
  tower.
- **Digit windows**: seven-segment readouts leaning 9°, with unlit (ghost) segments; every digit
  has the same cell and a 1 sits in its middle, so nothing moves as values change. Meters show
  `0`–`100` and a `%` drawn on the glass; the network shows up and down rates as at most three
  digits and K, M or G (KB/s, MB/s, GB/s).
- **Graphs** (dot matrix, one design pixel a dot): CPU, a bar per core (pairs and so on merged to
  at most 14, each the busiest core in it); memory and GPU, a stepped line of the last ~45
  seconds; disk, bars; network, upload as a line above a centre axis and download filled below,
  on a log scale to 10 MB/s. While the history fills up, graphs end at the right edge (now).
- **Hot**: a meter above 70%, or network traffic of 1 MB/s or more either way. Hot pixels take
  the `hot` colour, and the metric's engraved icon takes `iconHot`.
- **The clock**: its own window, the date in a 3×5 pixel font over a large seven-segment time.
- **The panel**: the metric's name and value, the last minute at twice the dot size with axes
  (and a dotted 70% line), and a footer: per-core bars and the effective clock speed (CPU);
  used and total memory (memory); used and total dedicated video memory (GPU); read and write
  rates (disk); up and down rates (network). Rates in the panels carry `/S` (K/S, M/S, G/S:
  KB, MB and GB a second).
- **Top 10 processes** (design "P4" with the colour screen of "S2"; CPU, memory, GPU and disk
  panels, not the network's):
  under the screen, behind a parting line, the panel's body holds the list. What can change is
  on glass, what is fixed is engraved in the plastic, and a key only travels when pressed.
  - Engraved: the heading `TOP 10 PROCESSES`, the row numbers 1–10, a tick from each row to
    its key and the bars' scale (0, 1, 5, 10, 25, 50, 100% on a square-root scale; a number
    that would touch the one before it on short bars is left out). Lettering on the plastic is
    in capitals like a calculator's printed labels: Arial Bold (every Windows has it), 5 design
    pixels to the capitals' top, unhinted so the letters space evenly. Text on glass stays in the pixel font.
  - The keys' legend is a multiplication sign `×`.
  - The list window is one cover glass over two displays, with black printed on its back
    round the LCD's opening and over a colour screen at its left. It reflects a window behind
    the viewer (design "G3"): the upper glass a shade lighter, fading out at a horizon, and a
    glint on its top edge. While the panel is open the horizon rises and falls with the pointer
    and the glint follows it across, easing there about 30 times a second; only the glare is
    redrawn, and nothing when it has settled.
  - The LCD: a row per program, 11 pixels apart: its name (9 characters, `~` for the rest), a
    bar of its share of the whole (the CPU, all memory, the GPU, all programs' I/O) and its
    value (disk rates with `/S`, their bars shorter by its width). A program over 70% of the
    whole is hot (not for I/O).
  - The colour screen: each row's program's own icon, read from its file, 8 x 8 pixels of one
    design pixel each, with no tile round it (the screen's black is the print's). A program
    whose file can't be read (system and protected processes) or that has no icon shows a cog;
    the program that couldn't be ended shows a padlock; the selected row's cell lights.
  - A key per row: the first press selects the program (its row in inverse, its icon's cell lit,
    for 5 s); a second press on it ends every process with that name. A second press as quick as
    a double-click is ignored. A program Readout can't end shows `ACCESS DENIED` and a padlock.
  - The rows hold still while the pointer is on the panel or a row is selected, held or refused;
    a program that ends meanwhile leaves its row blank.

## Themes

A theme is one flat JSON object of colours (`"#rrggbb"` or `"#rrggbbaa"`), nothing else, with
every key below and no others; a broken theme file is refused with the reason. The drawing code
never names a theme. The built-in themes are `themes/*.json`, embedded at build time (adding a
file adds a theme); a file in `%APPDATA%\Readout\themes` adds a theme or replaces a built-in of
the same name.

| Key | Role |
|---|---|
| `plateTop`, `plateUpper`, `plateLower`, `plateBottom` | The faceplate's gradient at 0%, 22%, 55% and 100% of its height |
| `plateDotLight`, `plateDotDark` | The faceplate's stipple dots |
| `plateEdgeTop`, `plateEdgeBottom`, `plateShadow` | The faceplate's top and bottom edge lines, its shadow |
| `hover` | The tint behind the tile under the pointer |
| `surroundTop`, `surroundBottom` | The surround's gradient |
| `surroundLipAbove`, `surroundLipBelow` | The lips half a pixel above and below the surround |
| `glassTop`, `glassBottom` | The glass's gradient |
| `glassShadow`, `glassEdge`, `glassGlow` | The shadow under the glass's top edges, its inner edge line, a glow below it |
| `gridLine`, `gridTint` | The lines between dot-matrix pixels, the tint of an unlit pixel |
| `ink`, `inkGhost`, `inkShadow` | Lit pixels and segments, unlit segments, the shadow lit ink casts |
| `fill` | The body of a bar or filled area under its lit top row |
| `hot`, `divider` | Hot pixels; the dotted dividers |
| `iconGroove`, `iconHot` | An engraved icon, and the icon of a hot metric |
| `iconLipAbove`, `iconLipBelow` | The lips one pixel above and below an engraving |
| `panelShadow`, `panelRing`, `panelRingLip` | The panel's shadow, the ring round its screens, the highlight under that ring |
| `keyTop`, `keyBottom` | A key cap's gradient |
| `keySide`, `keyEdge`, `keyShadow` | A key's side wall under the cap, the lit top row of the cap, its shadow on the plate |
| `keyLegend` | The legend on a key |
| `coverMask`, `coverGlare`, `coverEdge` | The list window's cover glass: the black printed on its back, the reflection across it, its lit top edge |
| `appSel`, `appNone`, `appDeny` | The colour screen: a selected row's lit cell, the icon of a program with none, the padlock of one that couldn't be ended |

## Readings

`src/metrics.rs`, sampled once a second: per-core and total CPU load and the effective clock
speed; memory in use; GPU load (the busiest engine type, as Task Manager shows it) and dedicated
video memory; disk active time and read and write rates; network bytes up and down on the
connected physical adapters. No administrator rights; temperatures aren't shown.

Per program (`src/procs.rs`), only while a panel with a list is open: one
`NtQuerySystemInformation` snapshot a second gives each process's CPU time, private working set
(Task Manager's memory column) and I/O bytes; GPU comes from the same GPU engine counter, per
process, the busiest engine type. Processes with one image name are summed. Ending a program
opens each of its processes for termination; one that can't be (elevated, protected, another
user's, System, Readout itself) makes the result denied. Network traffic per process needs
administrator rights, so the network panel has no list.

## Settings and command line

Settings (`%APPDATA%\Readout\settings.ini`; next to the exe in preview mode):
`theme` (a choice of every theme), `cpu`, `memory`, `gpu`, `disk`, `network` and `clock`
(toggles, all on). Printed and changed through the [blade settings](../../../../docs/specs/blade-settings.md)
contract; a running bar applies a change at once.

```text
Readout.exe                          start the bar (single instance)
Readout.exe --exit                   close the running bar
Readout.exe --autostart on|off|status
Readout.exe --status [seconds]       print live readings, one line a second (default 3)
Readout.exe --top [seconds]          print the top 10 programs by CPU, memory, GPU and I/O (default 2)
Readout.exe --settings               print settings for PocketKnives Settings
Readout.exe --set key=value ...      change settings
Readout.exe --preview-render PATH [--theme NAME|FILE.json] [--scale 1|2|3|4] [--width PX] [--panel cpu|mem|gpu|dsk|net] [--live SECONDS] [--select ROW] [--deny ROW] [--press ROW]
Readout.exe --version
```

`--preview-render` draws the bar (and a panel, under its tile) to a PNG without opening a window:
with the design mock's sample values and programs and a fixed time, or with `--live`, that many
seconds of this machine's readings and the time now. The list's icons are those of this
machine's running programs with the rows' names. `--select`, `--deny` and `--press` show a
list row (1–10) selected, refused, or with its key held down.
