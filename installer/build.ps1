# Builds dist\PocketKnives-Setup-<version>.exe: the runner and blades from this checkout, in one
# per-user installer (Inno Setup 6.7+).
#   winget install JRSoftware.InnoSetup     # once, for ISCC.exe
#   .\installer\build.ps1
# Needs Visual Studio (or Build Tools) with "Desktop development with C++" (its libraries and the resource
# compiler) and Rust through rustup (rust-toolchain.toml picks the version). The first build
# downloads the Windows App SDK runtime from nuget.org (runner/winui/stage.ps1; pinned, hash-checked).
# Nothing running is stopped and nothing is written to the blades' folders.
$ErrorActionPreference = 'Stop'
$root  = Split-Path $PSScriptRoot
$stage = Join-Path $PSScriptRoot 'obj\app'

$iscc = @(
    (Get-Command ISCC.exe -ErrorAction SilentlyContinue).Source
    Join-Path $env:LOCALAPPDATA 'Programs\Inno Setup 6\ISCC.exe'
    Join-Path ${env:ProgramFiles(x86)} 'Inno Setup 6\ISCC.exe'
    Join-Path $env:ProgramFiles 'Inno Setup 6\ISCC.exe'
) | Where-Object { $_ -and (Test-Path $_) } | Select-Object -First 1
if (-not $iscc) { throw 'ISCC.exe not found. Install Inno Setup: winget install JRSoftware.InnoSetup' }

if (Test-Path $stage) { Remove-Item $stage -Recurse -Force }
New-Item -ItemType Directory $stage | Out-Null

# SnipSnap: its own build.bat, into the stage.
& cmd /c (Join-Path $root 'blades\snipsnap\build.bat') $stage
if ($LASTEXITCODE -ne 0) { throw 'SnipSnap build failed' }

# QuickTooth, the runner (pk.exe) and its Windows 11 settings app: Rust, built from the repo root
# so cargo finds rust-toolchain.toml and .cargo/config.toml, into installer\obj. One package per
# build, so each exe is exactly what its own `cargo build -p` makes.
$cargoOut = Join-Path $PSScriptRoot 'obj\cargo'
$release = Join-Path $cargoOut 'release'
Push-Location $root
try {
    foreach ($package in 'quicktooth', 'pocketknives-runner', 'pocketknives-settings') {
        cargo build --release -p $package --target-dir $cargoOut
        if ($LASTEXITCODE -ne 0) { throw "$package build failed" }
    }
} finally { Pop-Location }
Copy-Item (Join-Path $release 'QuickTooth.exe'), (Join-Path $release 'pk.exe') $stage

# The blade icons the settings windows show.
$icons = New-Item -ItemType Directory (Join-Path $stage 'icons')
Copy-Item (Join-Path $root 'blades\snipsnap\assets\snipsnap.ico'), (Join-Path $root 'blades\quicktooth\assets\quicktooth.ico') $icons

# The Windows 11 settings app, with the Windows App SDK runtime beside it.
& (Join-Path $root 'runner\winui\stage.ps1') -Out (Join-Path $stage 'winui') -Exe (Join-Path $release 'pocketknives-settings.exe')

& $iscc /Qp "/DAppSource=$stage" (Join-Path $PSScriptRoot 'PocketKnives.iss')
if ($LASTEXITCODE -ne 0) { throw 'ISCC failed' }

Get-ChildItem (Join-Path $root 'dist') -Filter 'PocketKnives-Setup-*.exe' |
    Sort-Object LastWriteTime | Select-Object -Last 1 |
    ForEach-Object { "Built $($_.FullName) ($([math]::Round($_.Length / 1MB, 1)) MB)" }
