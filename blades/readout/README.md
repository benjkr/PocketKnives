# Readout

A calculator-style system monitor bar along the top of the screen: CPU, memory, GPU, disk and
network as seven-segment readouts over dot-matrix graphs, and the time.

## What it does

- **One LCD glass**: each metric has its own seven-segment window, and its graph runs on one
  continuous dot-matrix glass under them all: a bar per CPU core, lines for memory and GPU, bars
  for disk, and a mirrored upload/download graph for the network.
- **Engraved icons**: each metric's icon is cut into the faceplate beside its window, and turns
  orange when the metric runs hot (over 70%, or a network burst over 1 MB/s).
- **Panels**: hover a metric to see its last minute, with details (per-core load and clock
  speed, memory and video memory used, disk read and write, network up and down).
- **Top 10 processes**: the CPU, memory, GPU and disk panels list the ten programs using the
  most, with a calculator key on each row: press X to select a program, X again to end it.
- **Stays put**: every digit and graph column has a fixed place, so nothing shifts as numbers change.
- **Themes are colour files**: ivory, graphite and navy built in. Copy one from `themes/` into
  `%APPDATA%\Readout\themes`, rename it and change its colours to make your own; right-click the
  bar to switch.

## Command line

```text
Readout.exe                          start the bar (single instance)
Readout.exe --exit                   close the running bar
Readout.exe --autostart on|off|status
Readout.exe --status [seconds]       print live readings
Readout.exe --top [seconds]          print the top 10 programs by CPU, memory, GPU and I/O
Readout.exe --settings               print settings for PocketKnives Settings
Readout.exe --set key=value ...      change settings
Readout.exe --preview-render PATH [--theme NAME|FILE.json] [--scale N] [--width PX] [--panel cpu|mem|gpu|dsk|net] [--live SECONDS] [--select ROW] [--deny ROW] [--press ROW]
Readout.exe --version
```

## Build

```powershell
cargo build --release -p readout
```
