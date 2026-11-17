# AGENTS.md: QuickTooth

QuickTooth: a Windows 11 notification-area icon that connects or disconnects the user's AirPods in
one click, in about 1–3 s instead of ~16 s through Quick Settings. C# / .NET 10, shipped as one
Native AOT exe. The same app code also runs in a browser simulator of the taskbar.

**The suite's [`AGENTS.md`](../../AGENTS.md) comes first**: its rules hold here too. This file adds
what's specific to QuickTooth. Paths and commands here and in this blade's `docs/` are from
`blades/quicktooth/`.

## Rules for QuickTooth

- **One codebase, two hosts.** Behaviour and looks live in `src/QuickTooth.Core`, which has no
  Windows APIs: the icon and its animation, the tray state machine, tooltip and menu text,
  settings, and every timing number (`Tunables.cs`). The Windows app (`src/QuickTooth`) and the
  simulator's WebAssembly build (`src/QuickTooth.Web`) only adapt the platform. Never copy core
  logic into JavaScript or a host ([architecture](docs/working/architecture.md)).
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
- **Native AOT**: JSON and COM are source-generated (no reflection), and the runtime patch pin in
  `src/Directory.Build.targets` never goes backwards. Budget: one exe of about 5.3 MB that needs no
  .NET runtime.
- **The log**: `quicktooth.log` lines are compared across builds, the simulator and live runs.

## Running things

- Use a top model and high effort for the rasterizer, COM/interop and timing races.
- In the foreground: native AOT publish (~1 min), `bun qt-sim.mjs test` (~1 min), browser core
  build (5–10 s). In the background, with `t3_background_wait`: real-time recordings and anything
  longer.

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
| `settings.json`, the log, command-line options, single instance | [settings and CLI](docs/specs/settings-and-cli.md) |
| The browser simulator: what's real, the bridge, mock Bluetooth, commands | [simulator](docs/specs/simulator.md) |
| Native AOT build, runtime pin, installer, `publish.ps1`, what gets installed, releases | [build and install](docs/specs/build-and-install.md) |
| What to run for which change and what each check proves | [testing](docs/specs/testing.md) |

Defaults live in code (`Tunables.cs`), not in the docs' prose.
