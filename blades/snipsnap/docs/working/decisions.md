# Decisions: what was replaced, and why

Check the relevant entries before changing a behaviour, look or speed trade-off; don't
reintroduce these.

## Speed and memory

- **GPU capture (DXGI Desktop Duplication)** read the screen in about 5–10 ms, where GDI's
  readback is most of a full-screen copy's ~32–43 ms. But keeping it ready cost about 70 MB of
  graphics-driver memory in the background process. Left out on purpose.
- **A hidden overlay reused across snips** saved only ~2 ms: the gain came from showing without
  activation, which a fresh window gets too. The overlay is now created per snip and destroyed
  after the copy, off the critical path.
- **Hiding the overlay** took ~6.7 ms after mouse-up; DWM cloaking takes ~0.2 ms (mouse-up to
  clipboard went from 10.6 ms to 1.7 ms).
- **Tearing down before a one-shot run exits** delayed the exit by ~20 ms; Windows cleans up instead.
- **A faster opt-level than `z`**: `"s"` made the exe 21.5 KB and 2 or 3 made it 25–26 KB, for
  nothing: at `z` the dimming (SSE2) and the 24-bit rows (8+4-byte stores) were already as fast as
  the C build's `/O1` loops or faster, and the paint is GDI either way.

## Behaviour and look

- **`Ctrl+PrtSc` / `Ctrl+Shift+PrtSc` as the defaults**: now the first fallbacks; the defaults are
  `Ctrl+Alt+R`, `Ctrl+Alt+W` and `Ctrl+Alt+F`.
- **Full screen as the whole virtual desktop**: replaced by the monitor under the cursor.
- **Window mode taking the active window**: replaced by hovering and clicking a window on the
  frozen screen, so any window can be picked and the copy is ready ~8 ms after the click.
- **Painting the dimmed backdrop, then the bright selection on top**: the compositor could show a
  frame in between, so the selection flickered dark while dragging. Every pixel is now painted
  once.

## Building

- **The C build** (`snipsnap.c`, `cl /O1`, 19,456 bytes): replaced by Rust for memory safety, at
  20,480 bytes. Its loops were benchmarked against the Rust ones and its output compared byte for
  byte before it was removed.
- **`pk::args` and `pk::out::Writer`**: not used. `pk::args` applies the C runtime's quote and
  backslash rules, and `Writer` writes UTF-16 to a console; SnipSnap's parser and its byte output
  (and the message boxes keyed on having no output) stay as they were.
- **`core::fmt`** for SnipSnap's few formatted lines: on the `settings` line alone it added
  4.5 KB. User32's `wsprintfA` formats them instead.
- **`build.bat` without `start /wait`**: cmd didn't wait for the GUI exe's `quit`, so the running
  copy could still lock the exe when the new one replaced it (LNK1104 in the C build).
