# AGENTS.md: QuickTooth

QuickTooth: a Windows 11 notification-area icon that connects or disconnects the user's AirPods in
one click, in about 1–3 s instead of ~16 s through Quick Settings. Rust, `no_std`, shipped as one
small exe that needs no runtime. The same app code also runs in a browser simulator of the taskbar
(WebAssembly).

**The suite's [`AGENTS.md`](../../AGENTS.md) comes first**: its rules hold here too. This file adds
what's specific to QuickTooth. Paths and commands here and in this blade's `docs/` are from
`blades/quicktooth/`.

## Rules for QuickTooth

- **One codebase, two hosts.** Behaviour and looks live in `core/` (the `quicktooth-core` crate),
  which has no Windows APIs: the icon and its animation, the tray state machine, tooltip and menu
  text, settings, and every timing number (`core/src/tunables.rs`). The Windows app (`win/`) and
  the simulator's WebAssembly build (`web/`) only adapt the platform. Never copy core logic into
  JavaScript or a host ([architecture](docs/working/architecture.md)).
- **Show it in the simulator before it ships.** A change to what the tray looks like or does is
  previewed from `sim/` (stills, recordings, before/after), and `bun qt-sim.mjs test` passes on
  the final code.
- **Only the target device is ever sent anything.** Connect/disconnect goes to the stored target
  only (container ID, then MAC), never "the first headset". Other paired devices must never receive
  a one-shot. A live test needs the user's go in that request and a warning first
  ([live testing](docs/working/live-testing.md)).
- **Installing**: on the user's PC QuickTooth comes from the PocketKnives installer, from
  `master` ([installer](../../docs/specs/installer.md)); its own `publish.ps1` and installer
  aren't used there ([build and install](docs/specs/build-and-install.md)).
- **The exe**: `no_std` on the suite's `crates/pk`; COM and WinRT through the `windows` crate
  (no hand-written vtables), plain Win32 through `windows-sys`; no new crates without the user's
  say. Budget: one exe of about 0.3 MB that imports only Windows DLLs, about 1.3 MB private working
  set in the background, start-up and enumeration no slower than the C# build was
  ([build and install](docs/specs/build-and-install.md)).
- **The log**: `quicktooth.log` lines are compared across builds, the simulator and live runs.

## Running things

- Use a top model and high effort for the rasterizer, COM/interop and timing races.
- In the foreground: `cargo build --release -p quicktooth` (about 5 s after a core change, 21 s
  from clean), `bun qt-sim.mjs test` (~1 min; the browser core rebuilds in about 3 s). In the background, with
  `t3_background_wait`: real-time recordings and anything longer.

## Verification

- Core change: `bun qt-sim.mjs test` on the final code. Windows host or device code (the
  simulator doesn't run it): the native read-only checks in the [testing spec](docs/specs/testing.md).
- A refactor that shouldn't change behaviour: record every scenario's log, timeline and final
  state before, and diff them after.
- Visible changes come with simulator media.

## Working rules: load when applicable

| Work | Document |
|---|---|
| Writing code; where code goes; core vs hosts; threading and time | [Architecture and file map](docs/working/architecture.md) |
| Setup, building, the commands and how long they take | [Development](docs/working/development.md) |
| Anything touching real Bluetooth devices or the installed QuickTooth | [Live testing](docs/working/live-testing.md) |
| Changing a behaviour or look that was decided before | [Decisions](docs/working/decisions.md) |

Replies, pull requests, this machine's quirks and the desktop rules are suite-wide:
[`docs/working/`](../../docs/working/) at the repo root.

## Specs: read the one for what you touch

| Area | Spec |
|---|---|
| Clicks, connect/disconnect state machine, tooltip, menu, autostart, Explorer/theme/DPI | [tray](docs/specs/tray.md) |
| Artwork, renderer, static icons, animation phases, the glow | [icon and animation](docs/specs/icon-and-animation.md) |
| Finding the headset, one-shot connect/disconnect, link state, notifications, battery | [bluetooth](docs/specs/bluetooth.md) |
| `settings.json`, PocketKnives Settings (`--settings`, `--set`), the log, command-line options, single instance | [settings and CLI](docs/specs/settings-and-cli.md) |
| The browser simulator: what's real, the bridge, mock Bluetooth, commands | [simulator](docs/specs/simulator.md) |
| The Rust build, installer, `publish.ps1`, what gets installed, releases | [build and install](docs/specs/build-and-install.md) |
| What to run for which change and what each check proves | [testing](docs/specs/testing.md) |

Defaults live in code (`core/src/tunables.rs`), not in the docs' prose.
