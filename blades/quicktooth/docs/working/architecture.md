# Architecture and file map

Read "Where code goes" and "Rules" before writing code; use the file map to locate an area.

## Where code goes

- **A behaviour, a look, a piece of text or a number belongs in `core/`** (the `quicktooth-core`
  crate). A host only adapts the platform. If both hosts would need the same code, it's core code.
- **Windows-only** (calls Win32, COM, WinRT, the registry or files): `win/` (the `quicktooth`
  crate, `QuickTooth.exe`).
- **Browser glue**: `web/` (the `quicktooth-web` WebAssembly crate) and `sim/web/core.js`,
  `sim/web/tray.js` (JS side).
- **What Windows itself draws** (taskbar, tooltip box, menu box): the simulator's `sim/web/shell.js`
  and `menu.js`, kept true to the measurements in `sim/reference/`.
- **What any Rust blade needs instead of std** (entry point, heap, arguments, console, threads):
  the suite's `crates/pk`, not here.

## Rules

- Hosts implement `TrayHost` and `DeviceBackend` (`core/src/host.rs`). Add a member there only for
  something the platform really provides, and implement it in both hosts in the same change.
- **Async the way the C# version ran it** (`core/src/exec.rs`): `exec::spawn` runs a task up to its
  first wait that isn't complete; a host's `Done` completes an `Op` and wakes the task through the
  host's `Schedule`: the exe posts a window message per wake-up, the browser runs it at once in
  order (`exec::Inline`). `Op`/`Done` are `Rc`: they never leave the UI thread, so a host whose work
  finishes on another thread posts the result to its UI thread first.
- **No `RefCell` borrow is alive across a host call or an `.await`**: the exe re-enters the
  controller from `exit()` (`DestroyWindow`), the menu's modal loop and messages sent during
  `Shell_NotifyIcon`. A panic aborts the process.
- **Time comes only from the host** (`now_ms`, `now_epoch_ms`, `delay`, `set_timer`): the
  simulator's virtual clock depends on it.
- Numbers go in `core/src/tunables.rs` (named, settable by the simulator), not inline.
- No per-app state in statics: the simulator runs several app instances in one page. Per-process
  hooks (the log sink, the settings saver) are set by each host at start.
- Exactness: numbers are printed and rounded the .NET way (`core/src/num.rs`), and `sin`/`cos`/`pow`
  come from `core/src/math.rs` (the Windows C runtime in the exe, libm in the browser), because the
  logs, timelines and icon pixels are compared with recorded output.
- COM and WinRT go through the `windows` crate (generated bindings, released on drop); plain Win32
  through `windows-sys`. No hand-written vtables.
- Only `win/src/audio.rs` `Audio::send_one_shot` sends to a device, and only to one container ID.
- One source of truth (a suite rule, [`AGENTS.md`](../../../../AGENTS.md)): the simulator never gets
  a JavaScript copy of core logic or numbers.

## File map

- `core/src` (`no_std` + alloc, no Windows APIs; also built for wasm32):
  - `controller.rs`: the tray state machine. `host.rs`: the host traits, `IconImage`, `MenuItem`,
    tray commands. `exec.rs`: the executor.
  - `icon/`: the renderer (`mod.rs` phases, `raster.rs`, `glyph.rs` SVG and outline, `png.rs`,
    `sort.rs` .NET's sort). `tunables.rs`, `math.rs`, `num.rs`.
  - `settings.rs` (settings.json), `blade_settings.rs` (`--settings` lines, `--set` checks),
    `devices.rs`, `battery.rs` (readings, the Apple advert), `log.rs`, `json.rs`, `guid.rs`.
- `win/src` (`QuickTooth.exe`, `no_std` on `pk`): `main.rs`, `tray.rs` (window, notify icon,
  timers, menus, the hosts and scheduler), `cli.rs`, `worker.rs` (the device thread), `audio.rs`
  (Core Audio, KS), `pnp.rs` (cfgmgr32), `ble.rs` (battery scan), `files.rs` (settings folder, log,
  saving), `system.rs` (autostart, theme, promotion, dark menus), `ipc.rs` (names other programs
  use), `clock.rs`, `selftest.rs`, `icons.rs` (HICONs and the `--render-icons` sheets). `build.rs`
  and `app.manifest`: resources through `crates/pk-build`.
- `web/src/lib.rs`: the WebAssembly exports and `qt-host` imports.
- `sim/`: `qt-sim.mjs` (CLI), `lib/` (session, scenarios, checks, parity, core build), `web/` (the
  page: `core.js`, `tray.js`, `icon.js`, `shell.js`, `menu.js`, `bluetooth.js`, `clock.js`,
  `main.js`, `panel.js`), `scenarios/`, `reference/` (measured metrics and probes).
- `installer/` (Inno Setup script, build script), `publish.ps1`, `tools/` (glyph generator),
  `assets/`, `docs/`.
