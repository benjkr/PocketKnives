# NowPlaying

Live media cover, seekable timeline and 5-band audio frequency meter docked in the Windows 11 taskbar.

## What it does

- **Clear native look**: sits directly on the Windows 11 taskbar with a `40×40` flush cover, equal
  left/top/bottom taskbar padding, and an animated Windows 11 Fluent hover plate (`Dark` and `Light`
  taskbars).
- **Live SMTC control**: displays title, artist, source app icon, album art (decoded at native DPI),
  play/pause, previous/next, and an interpolated, click-and-drag seekable progress bar with hover
  timestamp preview. Clicking the title or app icon brings the playing app to the front.
- **Live audio frequency meter**: reads PC audio via WASAPI loopback through a 512-point FFT into
  5 center-expanding white pills (pauses capture automatically when nothing is playing).
- **Zero-flicker docking**: owned by `Shell_TrayWnd` so clicking between apps and the taskbar never
  blinks, never steals focus, and hides behind fullscreen windows.

## Command line

```text
NowPlaying.exe                        start the taskbar strip (single instance)
NowPlaying.exe --status               print the current SMTC media session (read-only)
NowPlaying.exe --autostart on|off|status
NowPlaying.exe --exit                 close the running strip
NowPlaying.exe --settings             print settings for PocketKnives Settings
NowPlaying.exe --set key=value ...    update settings
NowPlaying.exe --preview-render PATH  render a 2x preview PNG of the strip
NowPlaying.exe --version              print version
```

## Build

```powershell
cargo build --release -p nowplaying
```
