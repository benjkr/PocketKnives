# Tray icon

Part of QuickTooth's specs (index in [`AGENTS.md`](../../AGENTS.md)). What the app does now; defaults are in
`core/src/tunables.rs`. Keep it current in the same change as the code, and lean:
rewrite a bullet when something changes; history goes in commits and PRs.

The behaviour is `core/src/controller.rs` (shared); `win/src/tray.rs` is its Windows host.

## The icon and its text

- Connected: the solid glyph with dimmed ear tips. Not connected, not available or no device: the
  hollow outline with solid ear tips. White on a dark taskbar, `#1A1A1A` on a light one (Windows
  mode, `SystemUsesLightTheme`). Low battery: the connected glyph with an orange dot in the
  bottom-right corner while the tooltip shows a reading at or below the low-battery warning
  (a setting, [settings and CLI](settings-and-cli.md)) for the emptier bud, not while charging; it ripples when it appears and every 3 s ([icon](icon-and-animation.md)). Rebuilt when the theme flips (`WM_SETTINGCHANGE`
  "ImmersiveColorSet") and when the small-icon size changes (DPI or display change, checked again
  on every idle poll).
- Tooltip and menu header: `<device> — <status>`, plus ` · <battery>` while connected and the
  reading is under 15 min old. Status: Checking…, Connected, Not connected, Not available (the
  saved device is missing), No device selected, Connecting…, Disconnecting…. Capped at 127
  UTF-16 units (`szTip`; a character the cut would split is dropped whole).

- Where the buds are (from the advert watch, [bluetooth](bluetooth.md)): while it's known and
  not "both out", the icon is the case icon instead ([icon](icon-and-animation.md)); a connect,
  disconnect or ripple animation takes over while it runs. Not heard for `PodsStaleMs` (90 s): back
  to the plain icon. Each change is logged (`pods: lid open`, `pods: left out, in an ear`,
  `pods: not heard`).

## Clicks and commands

- Left click or keyboard select (`NIN_SELECT`, `NIN_KEYSELECT`) toggles the target. With no
  target, the menu opens at the cursor. A click while an action is pending is ignored (logged).
- Another `QuickTooth.exe` sends commands through the registered message `QuickTooth.Command.v1`
  to the window class `QuickTooth.TrayWindow`: toggle, connect, disconnect, exit, show-menu
  (anchored at the icon's top centre) and close-menu.
- `--set` (the PocketKnives Settings window) reaches the tray as `WM_COPYDATA` and gets an answer
  back: refused while an action is pending (logged); a headset change is applied like a Device menu
  pick (only a device the menu offers; the same `target set:` line); a new low-battery warning
  shows or hides the dot at once. Then the tray saves.

## Connect and disconnect

- **Connect**: if the Bluetooth link is still up from a recent disconnect (it lingers ~3 s), wait
  for it to close, checking every 200 ms for up to 5 s (logged when over 300 ms). Then send the
  one-shot to every KS control of the target, A2DP first, then HFP. Start a 20 s timeout, poll
  the state every 0.5 s, and resend once if still not connected after 8 s.
- **Disconnect**: send at once; same timeout and polling; no resend.
- Success is judged from the endpoints' state, never from the send's result. The attempt ends
  early only if no control accepted the request. On timeout the icon goes back to the real state
  (logged "timed out … showing the real state").
- While idle: a refresh every 20 s, and 200 ms after any endpoint notification.
- Battery while connected: first scan 2.5 s after connecting, then a check every 60 s that rescans
  when the reading is over 5 min old; opening the menu rescans when it's over 60 s old. How it's
  read: [bluetooth](bluetooth.md).

## Menu

- Order: the status line (grayed), separator, Connect or Disconnect, Device ▸, separator,
  Start with Windows, Exit.
- While the buds' place is known, the status line is a picture instead (its text stays the item's
  text; `core/src/popup`) on the menu's own background (dark or light, with the taskbar): the
  device name centred; the case, open with the buds seated in it while a bud is in it, closed when
  both are in, closed, faint (35 %) and sunk a little with its bottom fading out in steps when both are out, a bud that's out standing beside it; under the
  left bud, the case and the right bud a Fluent battery gauge and its level from the last advert
  (or the connected reading): green with a bolt while charging, orange at or below the
  low-battery warning, "—" when unknown. The pictures are shaded white AirPods
  (`assets/header-*.svg`, [icon](icon-and-animation.md)), all grey when not connected.
  236 × 102 px at 100 %. The exe makes it a bitmap item (the core's pixels with Segoe UI text by
  GDI), enabled so Windows doesn't dim it (picking it does nothing): an owner-drawn item would
  turn off Windows 11's theme for the whole menu. The rows below stay Windows' own.
- While the menu is open the header moves with the buds (`core/src/popup/motion.rs`, the approved
  "B1 alive"): springs for the lid (through three in-between lid drawings), each bud's way out
  and the case's fade; the lid snaps shut with a small bump; buds hop out on a short arc, the
  right after the left, and bob a whole pixel while out; the light is green while open and
  blinks amber for 5.2 s after the case closes. Every animation tick a new frame is drawn and,
  only if it changed, swapped into the open menu (`ModifyMenu` and a redraw); once still, nothing
  is redrawn.
- While an action is pending, the second item is the grayed status (Connecting…) and Device is
  grayed. With no target, the second item is a grayed Connect.
- Device ▸ lists the paired Bluetooth audio devices (the target checked, `  (connected)` after
  connected ones). A saved target that's missing appears grayed and checked as
  `<name> (not available)`; none at all: "No Bluetooth audio devices". Picking one saves it as
  the target.
- Start with Windows is checked when the `HKCU\…\Run\QuickTooth` value exists; clicking toggles
  it. The menu's dark/light look follows the taskbar theme.

## Taskbar integration

- A hidden top-level window (a message-only window gets neither "TaskbarCreated" nor
  `WM_SETTINGCHANGE`), `Shell_NotifyIcon` version 4.
- Adding the icon retries every 2 s, up to 60 times (a stale icon, or Explorer not ready at
  sign-in). On "TaskbarCreated" (Explorer restarted) it's added again with fresh icons.
- First run only: marks its own `NotifyIconSettings` entry `IsPromoted=1`, so the icon sits on the
  taskbar instead of in the overflow. Retries every 3 s, up to 10 times, until Explorer has written
  the entry; then never again (`trayPromotedOnce`), so the user's own choice stands.
- One tray per user session (mutex `Local\QuickTooth.Tray.6F0C2B1E`); a second start exits.
