# Readout

What it does now. Keep it current in the same change, and lean.

## Window

- **An app bar along the top of the main screen** (`SHAppBarMessage`, `ABE_TOP`): Windows keeps
  other windows out from under it, like the taskbar. Full screen width, 37 design pixels tall.
  A layered window (`UpdateLayeredWindow`) that never takes focus, with a 4-pixel shadow drawn
  under the reserved strip. It steps behind full-screen apps and comes back after them.
- **Scaling**: every design pixel is a whole number of device pixels, the nearest to the screen's
  scaling and at least 1 (100% and 125%: 1; 150% to 225%: 2; 250%: 3), so the dot matrix and the
  engraving stay sharp.
- **Once a second** it samples the system and redraws. Hovering a metric shows its panel under
  it; leaving the bar hides it. Right-click: Theme (every theme, the current one checked) and Exit.

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
  rates (disk); up and down rates (network).

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
| `panelShadow`, `panelRing`, `panelRingLip` | The panel's shadow, the ring round its screen, the highlight under that ring |

## Readings

`src/metrics.rs`, sampled once a second: per-core and total CPU load and the effective clock
speed; memory in use; GPU load (the busiest engine type, as Task Manager shows it) and dedicated
video memory; disk active time and read and write rates; network bytes up and down on the
connected physical adapters. No administrator rights; temperatures aren't shown.

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
Readout.exe --settings               print settings for PocketKnives Settings
Readout.exe --set key=value ...      change settings
Readout.exe --preview-render PATH [--theme NAME|FILE.json] [--scale 1|2|3|4] [--width PX] [--panel cpu|mem|gpu|dsk|net] [--live SECONDS]
Readout.exe --version
```

`--preview-render` draws the bar (and a panel, under its tile) to a PNG without opening a window:
with the design mock's sample values and a fixed time, or with `--live`, that many seconds of this
machine's readings and the time now.
