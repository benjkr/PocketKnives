# Decisions: what was replaced, and why

Check the relevant entries before changing a behaviour or look; don't reintroduce these.

## Connecting

- **Sending the reconnect to the A2DP control only**: 9 s, 15 s, or not within 40 s. Sending it to
  A2DP and HFP together: 1–3 s.
- **Reconnecting during the link teardown** (~3 s after a disconnect) stalled for 15 s or more. The
  connect now waits for the link flag to drop.
- **Judging success by the send's result**: the state is the truth; only a refusal from every
  control ends the attempt early.

## Look and animation

- **MingCute icons** were replaced by the two-tone AirPods Pro glyphs as the default; at the
  user's request they came back as the second choice of the Tray icon setting ("AirPods"), drawn
  as published with their original 0.45 dimming when not connected.
- **A 0.45 px outline** on the not-connected glyph was too faint to see in the taskbar: now 0.7 px.
  Making the buds bigger instead was tried and rejected by the user. Stretching them 1.3× taller
  looked thin and distorted; turning the heads 25° tip-down with longer stems (about a quarter
  bigger) wasn't chosen. The pair already fills the 16 px width.
- **Low battery**: chosen from five designs, an orange dot in the corner (shown in red first; the
  user picked orange). Not chosen: a small battery badge cut into the right stem, the whole icon in
  red, a level bar under the buds, and the stems as red/faint gauges. Its animation, from five:
  a ripple, repeated every 3 s at the user's request (the first proposal replayed once a minute).
  Not chosen: the dot popping in, filling up like the buds, a glint across it, a drop running
  down the stem into it.
- **Pulsing** while connecting: the user asked why it pulsed. Replaced, from five non-pulsing
  designs, by the glow ("tracer") with the earbuds snapping together and filling up when they land.
- **The glow as a spot circling the head**, then as a disc as wide as the stem (both sides of the
  stem lit at once, so it seemed to bounce): replaced by a thin band running along the outline.

## Building and shipping

- **WPF to draw the icons**: pulled in the UI framework and ruled out Native AOT. Replaced by the
  shared rasterizer, which is also closer to the exact shape at the edges.
- **C# / .NET 10 Native AOT** (a 5.2 MB exe, 15–18 MB private working set in the tray; 1.5 MB
  of it the CsWinRT projection for the battery scan): replaced by Rust, `no_std`, one exe with no
  runtime. Before that, a self-contained .NET bundle (165 MB installed), a compressed single file
  (69 MB of RAM) and trimming (it switched off the COM and JSON the app used) were tried and
  replaced.
- **Rust with std**: about 100 KB more per exe (measured 108 KB for an empty program against 7 KB
  `no_std`); `crates/pk` supplies what QuickTooth needs instead.
- **Hand-written COM and WinRT vtables**: measured within a few KB of the `windows` crate's
  generated bindings, which cost about 18 s more on a clean build only. The generated bindings
  were kept: a wrong vtable order silently calls the wrong method.
- **Where a panic happened, in the log**: 14 KB (every panic site's file and line kept), its
  message 28 KB more. A panic logs `ERROR panic: QuickTooth stopped on a bug` and ends the process.
- **Rust's general float parser**: 14 KB of tables for numbers that are short decimals; replaced by
  an exact parser for up to 15 digits (`num::parse_f64`).

## Simulator

- **Node and npm to run the simulator**: replaced by Bun (`bun install`, `bun qt-sim.mjs`). Bun
  1.3.8 hung in Playwright's Chromium launch on Windows, over the pipe and over `connectOverCDP`;
  1.4.4 passes the whole suite, so the launcher refuses anything older. pnpm was the fallback and
  isn't needed.

- **JavaScript copies of the renderer and tray logic**, kept in step by parity checks: replaced by
  the shared core compiled to WebAssembly (first C#, now Rust). JavaScript `--variant` sketches
  went with them; `--tune` stays.
- **wasm-bindgen** for the browser bridge: it needs its own CLI tool matched to the crate version;
  plain exports and imports with a small JS glue (`sim/web/core.js`) need nothing extra.
- **Wall-clock or host-independent timers in shared code** broke the virtual clock's determinism.
  All time comes from the host.
