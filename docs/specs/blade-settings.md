# Blade settings: how Settings reads and changes a blade's settings

What it does now. Keep it current in the same change, and lean. The [runner](runner.md) builds
each blade's page from what the blade prints; the blade alone reads, checks and writes its own
settings. There's no manifest: a setting is defined once, in the blade's code.

## Reading: `<blade> <settings command>`

SnipSnap: `snipsnap settings`. QuickTooth: `QuickTooth.exe --settings`. Settings runs every
blade's command at the same time when it opens; the XP and Windows 10 looks build their window
while the blades answer. Read-only, safe from any copy. Prints one line per
setting to stdout (a pipe), UTF-8, `\n` or `\r\n`, fields separated by a tab, and exits 0:

```
<key>	<kind>	<value>	<label>[	<choice value>=<choice label>]...
```

- `key`: lower camel case, `[a-zA-Z]+`. Its order is the page's order.
- `kind`:
  - `hotkey`: `value` is `modifiers*256 + virtual key`, in decimal. Modifiers are `RegisterHotKey`'s:
    Alt 1, Ctrl 2, Shift 4, Win 8. `0` means none.
  - `toggle`: `value` is `1` or `0`.
  - `choice`: `value` is one of the choice values that follow, or empty when nothing is chosen
    (QuickTooth with no headset). Choice values never contain a tab, `=` or a line break.
- `label`: shown next to the control, in the blade's words.
- An empty line or a line that doesn't parse is skipped; the page shows what it understood.

## Changing: `<blade> <set command> key=value [key=value...]`

SnipSnap: `snipsnap set ...`. QuickTooth: `QuickTooth.exe --set ...`. Values are written as
`--settings` prints them.

- All pairs are checked before anything is written; an unknown key or bad value changes nothing.
- The blade saves to its own settings file, then a running copy applies the change at once.
- Exit `0`: saved and applied. Exit `2`: nothing changed, and one line on stdout says why, in
  words for the user (for example `Ctrl+Alt+A is taken by another program.`). Settings shows that
  line.
- A hotkey that can't be registered is refused (exit 2) and the previous key stays.

## Preview

When the environment variable `POCKETKNIVES_SETTINGS_PREVIEW` is `1` (set by `pk --preview` for
everything it starts), a blade's settings and set commands use a settings file next to the
blade's exe instead of its real one, and never message a running copy. A hotkey is still checked
by registering it for a moment. So a scratch copy's commands never touch the user's settings or
running blades.
