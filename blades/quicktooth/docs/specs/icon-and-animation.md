# Icon and animation

Part of QuickTooth's specs (index in [`AGENTS.md`](../../AGENTS.md)). What the app does now; numbers are in
`core/src/tunables.rs`. Keep it current in the same change, and lean.

## Artwork

- Two artworks, picked with the Tray icon setting ([settings and CLI](settings-and-cli.md)); each
  is a pair of glyphs, connected (`-fill`) and not connected (`-line`), embedded in the core
  (`Artwork` in `core/src/icon/glyph.rs`). A change redraws the static icons at once. Settings
  shows each artwork's connected icon as its picture (`icon::setting_picture`, an ICO of PNGs).
  `tools/airpods_glyph.py` (needs `pip install shapely svgpathtools`) generates all four files.
- **AirPods Pro** (default), `assets/airpods-pro-*.svg`: from `tools/airpods_pro_shapes.json`,
  one earbud traced from the SVG Repo drawing #324521.
- **AirPods**, `assets/airpods-*.svg`: MingCute's "airpods" icons (Apache 2.0,
  `assets/MingCute-LICENSE.txt`) as published, from `tools/airpods_shapes.json`, curves and arcs
  flattened into segments at most `FLAT_STEP` 0.3 units long. One layer each: solid when
  connected, the outline at 0.45 when not; its `data-tracer` is the left head's circle at the
  middle of the 1-unit stroke. Laid out as MingCute drew it (no pixel snapping).
- Straight-line polygons only (`M`/`L`/`Z`): the renderer refuses curves. Each `<path>`'s
  `fill-opacity` dims its part (AirPods Pro: outline 0.85, ear tips 1); a line glyph's outline
  stays below 1 so the glow can brighten it. The AirPods Pro outline is 0.7 px wide at 16 px
  (`OUTLINE` in the generator). The line glyph's `data-tracer="cx cy rx ry"` is the head orbit;
  the glow's speed comes from it. Each bud stays on its own side of x = 12 (one per half).
- Drawn at `Zoom` 1.125 of the SVG's 24-unit box, so the artwork fills the 16 px width. The
  generator lays the AirPods Pro art out for that zoom (stems on whole pixels): change both
  together.

## Low-battery dot

- The connected glyph plus an orange dot (`#FF8C00` on a dark taskbar, `#CA5010` on a light one)
  centred at (13.5, 13) of the 16 px grid, radius `LowBatteryDotRadius`, with a ring
  `LowBatteryDotGap` wide cut out of the artwork around it (it clips the right stem's tip, or with
  AirPods the right head's lower edge). Scaled
  with the icon size. It's a third static icon (`IconState.LowBattery`). When it shows:
  [tray](tray.md).
- **Ripple** (`AnimPhase.LowBattery`): one thin orange ring spreads from just outside the gap
  (3 units further out, easing out) and fades, `LowBatteryAnimSeconds` 0.7 s, ending exactly on the
  static dot. It plays when the dot appears and then every `LowBatteryReplayMs` 3 s while it shows,
  through the same animation timer; a connect or disconnect animation takes over, and the timer
  skips a ripple while one is running.

## Where the buds are (the case icon)

- `core/src/icon/case.rs`, on the 16-unit grid: an outlined case (1.2 px lines) with a lid hinged
  on the body's top corners. Closed: the case alone with its status light. Lid open: the lid swings
  up and the buds sit inside at 0.72 of their size. One bud out: the case flows down out of the
  icon and fades while the buds stay where the plain glyph has them, the one still in the case at
  0.32. Both out: exactly the plain icon (and so the static icons, low-battery dot included).
  Connected buds are solid and knock the lid's lines out behind them; not connected, hollow.
- Each change eases (`CaseAnimSeconds` 0.52 s, in-out cubic) from wherever the icon is to the
  new pose, on its own timer at the animation frame rate; the case fades out late when it leaves
  and in early when it comes back, and buds appear only once the lid has started to open.

## Renderer

- `core/src/icon` (shared): a small scanline rasterizer with nonzero fill, 16 sub-scanlines per
  pixel and exact horizontal coverage. Clips (rectangles, or a union of discs) are intersected
  with each span geometrically. Output is straight-alpha BGRA.
- The same code draws in the exe and in the simulator; `qt-sim parity` holds them within 1/255.
- Windows: `win/src/icons.rs` turns pixels into HICONs. `QuickTooth.exe --render-icons <dir>` writes
  check sheets through the real HICON + `DrawIconEx` path: 16/20/24/32 px on both taskbars (connected,
  not connected, low battery), every
  animation frame and 240 px glyphs.

## Animation (while an action is pending)

- Driven by a 83 ms timer, which Windows fires about every 94.8 ms (~10.5 fps). Each frame is drawn
  live, and the previous frame's HICON is destroyed once the new one is shown.
- **Connecting**: the earbuds spring apart (`Travel` 1.9 units over `SpringSeconds` 0.2 s),
  shrinking toward a zoom at which nothing clips. Then a glow runs around each earbud's outer
  outline (`TracerOutline` 1): a band `TracerLength` 6 units long reaching `TracerWidth` 1 either
  side of the edge, clockwise on the left earbud and mirrored on the right. It moves at the head
  orbit's speed (a lap ≈ 1.52 s) and brightens the dimmed outline (85% → ~98%) on one side at a
  time. `TracerOutline` 0 is the earlier spot (`SpotRadius` 3) circling the head.
- **Landing** (connected): the earbuds close up in the first half of `LandSeconds` 0.5 s while
  the solid fills from the bottom; the glow fades.
- **Disconnecting**: the solid drains from the top while the earbuds part, then they settle back
  (`DisconnectSeconds` 0.8 s). If still pending after that, the glow runs on the outline until it
  lands.
- **Cancelling** (a connect that failed or timed out): the earbuds glide back (`CancelSeconds`
  0.3 s) and the glow fades.
- Every phase ends exactly on the static icon. Nothing pulses while connecting or disconnecting
  ([decisions](../working/decisions.md)); the low-battery ripple repeating every 3 s is the user's
  choice.
