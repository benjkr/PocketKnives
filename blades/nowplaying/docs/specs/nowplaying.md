# NowPlaying

What it does now. Keep it current in the same change, and lean.

## Look and layout

- **C1 Pure Clear**, `344 × 40` DIPs, rendered with Direct2D + DirectWrite + WIC into a
  premultiplied 32-bit BGRA layered window (`UpdateLayeredWindow`, grayscale text AA).
- **Docking**: equal padding on left, top, and bottom of the taskbar (`equal_pad = (tb_h - 40) / 2`,
  4 px on a standard 48 px taskbar). Always at the left end of the main taskbar.
- **Hover plate**: native Windows 11 Fluent tile (`CornerRadius = 4`, lifted translucent fill,
  top-lit vertical gradient elevation border, `85 ms` ease-out in, `167 ms` ease-out out).
- **Cover**: `40 × 40` flush on the left (`CornerRadius = 4`, WIC Fant scaling at native DPI, 1 px
  subtle rim). Dominant saturated color is extracted from the cover (or, without one, the app
  icon) for the hover scrub accent.
- **Row 1**: the playing app's own icon from the shell (`src/appicon.rs`: the session's
  AppUserModelID in `shell:AppsFolder`, else the running process whose exe has that name, as for
  Spotify's `Spotify.exe`; a neutral play glyph when neither has one), `Title · Artist` with ellipsis trimming, **white M2 5-pill
  center-expanding frequency meter** (dark ink on Light taskbar, collapses to 5 soft dots when
  paused), and `Prev / Play-Pause / Next` buttons with animated hover plates.
- **Row 2**: **P1 Spotify hover-reveal progress bar** — fixed `4 px` track centered in a `14 px`
  hit target (zero vertical jump), white fill at rest transitioning to album accent on hover/drag,
  cursor ghost seek fill, `10 px` smooth fade/scale knob, and tabular `cur / dur` with live hover
  timestamp preview. The time column reserves the measured width of the track's longest bold
  timestamp so multi-digit times never clip and the bar never jumps while hovering or scrubbing.

## Media and audio

- **SMTC** (`src/smtc.rs`): MTA worker using `GlobalSystemMediaTransportControlsSessionManager`.
  Subscribes to session, property, playback and timeline changes, interpolates live position while
  playing (`Position + (now_utc - LastUpdatedTime)`), decodes thumbnails via WIC, and executes
  play/pause, skip and seek commands asynchronously.
- **Audio spectrum** (`src/audio.rs`, `src/fft.rs`): WASAPI shared-mode loopback on the default
  render endpoint, 512-point Hann-windowed radix-2 FFT into 5 log-spaced perceptual frequency
  buckets (`40–150`, `150–420`, `420–1200`, `1200–3600`, `3600–14000 Hz`) with smooth AGC and
  fast-attack/slow-decay smoothing. Capture stops automatically when paused, hidden, or disabled,
  and re-opens automatically after default audio device switches.

## Z-order and window behavior

- Owned by `Shell_TrayWnd` (`GWLP_HWNDPARENT`) so Windows moves it atomically with the taskbar on
  every frame without flicker, plus an `EVENT_SYSTEM_FOREGROUND` hook, `TaskbarCreated` handler,
  and Z-order checks that re-raise the strip above `Shell_TrayWnd` whenever the taskbar rises over
  it (such as after a fullscreen window or SnipSnap overlay closes).
- `WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE` and `WM_MOUSEACTIVATE -> MA_NOACTIVATE` on left-clicks so
  clicking controls or scrubbing never steals focus.
- Hidden when no media session is active, and behind visible, uncloaked fullscreen windows
  (ignoring desktop shell windows like `Progman` and `WorkerW`).

## Settings, single instance and CLI

- Single instance via `Local\PocketKnives.NowPlaying.v1` mutex and `PocketKnives.NowPlaying.Strip`
  window class. Reload message: `PocketKnives.NowPlaying.Reload.v1`; exit message:
  `PocketKnives.NowPlaying.Exit.v1`.
- Settings live in `%APPDATA%\NowPlaying\settings.ini` (or `nowplaying-preview.ini` beside the exe
  when `POCKETKNIVES_SETTINGS_PREVIEW=1`): `visualizer` (`1` | `0`).
- Sign-in autostart uses `HKCU\Software\Microsoft\Windows\CurrentVersion\Run\NowPlaying` via
  `--autostart on|off|status`.
- Exit codes: `0` ok, `1` no active media (`--status`), `2` invalid arguments or error.
