# Capture: the three modes, the overlay, the clipboard, the flash

Part of SnipSnap's specs (index in [`AGENTS.md`](../../AGENTS.md)). What it does now; the numbers
are the `#define`s at the top of `snipsnap.c`. Keep it current in the same change, and lean.

## Reading the screen

- The process is per-monitor DPI aware (v2, else `SetProcessDPIAware`), set before anything else:
  every coordinate and every capture is in physical pixels.
- One GDI `BitBlt` with `SRCCOPY | CAPTUREBLT` (layered windows included) from the screen into a
  32 bpp top-down DIB section. Most of each capture's time is this readback from the compositor.
- Before each read, any flash still showing is removed, this process's or another SnipSnap's
  (found by its window class), and `DwmFlush` waits until the screen shows that.

## Full screen (`full`)

- The monitor under the cursor (nearest monitor), read and copied at once.

## Rectangle (`rect`) and window (`window`): the frozen overlay

- Both read the whole virtual screen first, so every monitor freezes, and make a dimmed copy
  (each channel halved). A topmost tool window over the virtual screen shows the dimmed copy. It's
  created for each snip, with DWM transitions off.
- To appear fast: painted while DWM-cloaked, shown without activation, uncloaked, then given
  focus. Esc is also registered as a hotkey while it's up, in case Windows refused it focus.
- Every pixel of a repaint is written exactly once: dimmed outside the lit area, the frozen image
  inside, a white border `FRAME_W` (2 px) wide on its ring. Nothing flickers while dragging.
- The result is cut from the frozen image, so it's on the clipboard a few ms after the release.
- Cancel (exit code 1): Esc, right-click, the overlay losing activation, or `WM_CLOSE`.
- At the end the overlay is cloaked at once (faster than hiding it). A one-shot run then exits and
  leaves the teardown to Windows; the background instance destroys the window and frees both
  images after the copy.

### Rectangle

- Crosshair cursor. Left-drag selects, clamped to the virtual screen and inclusive of both corners;
  the selection shows undimmed with the border. Releasing copies it. A click without a drag keeps
  waiting. Losing mouse capture mid-drag drops the selection.

### Window

- Arrow cursor. The window list is taken when the screen freezes, before the overlay exists, top
  of the z-order first (at most 512): visible, not minimised, not cloaked, not click-through, not
  fully transparent. Each window is its visible frame (DWM extended frame bounds: no invisible
  resize border or shadow), else its window rect.
- The topmost window under the cursor is lit, from the moment the overlay opens. Over the bare
  desktop (`Progman`, `WorkerW`) the whole monitor is lit. Clipped to the virtual screen.
- Taken when the left button is released, so the release doesn't land in the app underneath. It's
  what was on screen in that area, so anything covering the window is included.

## Clipboard

- `CF_DIB`: a `BITMAPINFOHEADER`, 24 bpp, bottom-up, rows padded to 4 bytes, `BI_RGB`. It pastes
  into Paint, Office, browsers and chat apps.
- Owned by the background instance's window, or a one-shot run's message-only window. Opening the
  clipboard is retried 20 times, 10 ms apart; if it stays busy the capture fails (exit code 2).

## Flash

- After the pixels are taken, a white, click-through, topmost window that never takes focus covers
  exactly the captured area, starting at opacity `FLASH_ALPHA` (150/255) and fading to nothing in
  `FLASH_MS` (150 ms), paced by elapsed time because timer ticks are coarse.
- Order: full screen reads, flashes, then copies; the overlay is cloaked, then flashes, then
  copies. The flash is never in an image: it comes after the read and is removed before the next.
- A one-shot run stays alive until the flash has faded.
- The `flash` setting ([settings](hotkeys-and-cli.md#settings)) turns it off: nothing is shown
  and a one-shot run exits as soon as the image is on the clipboard.
