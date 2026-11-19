# Builds QuickTooth (one exe, Rust) and installs it to %LOCALAPPDATA%\Programs\QuickTooth.
# A running copy is asked to exit first, because a running exe can't be overwritten.
# Needs Rust (rustup) and Visual Studio (or Build Tools) with "Desktop development with C++", for
# its libraries and the Windows SDK.
param([switch]$NoStart)
$ErrorActionPreference = 'Stop'
$dest = Join-Path $env:LOCALAPPDATA 'Programs\QuickTooth'
$exe  = Join-Path $dest 'QuickTooth.exe'

$running = Get-Process QuickTooth -ErrorAction SilentlyContinue | Where-Object { $_.Path -eq $exe }
if ($running) {
    Start-Process -FilePath $exe -ArgumentList '--exit' -Wait
    $running | Wait-Process -Timeout 10 -ErrorAction SilentlyContinue
    Get-Process QuickTooth -ErrorAction SilentlyContinue | Where-Object { $_.Path -eq $exe } | Stop-Process -Force
}

$repo = Split-Path (Split-Path $PSScriptRoot)
# From the repo root, so cargo finds rust-toolchain.toml and .cargo/config.toml.
Push-Location $repo
try { cargo build --release -p quicktooth } finally { Pop-Location }
if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }
New-Item -ItemType Directory -Force $dest | Out-Null
Copy-Item (Join-Path $repo 'target\release\QuickTooth.exe') $exe -Force
# Leftovers from 1.0.x builds, which bundled the .NET runtime and WPF next to the exe.
Get-ChildItem $dest -Filter '*_cor3.dll' -ErrorAction SilentlyContinue | Remove-Item -Force

if (-not $NoStart) { Start-Process -FilePath $exe }
Write-Host "Installed to $exe"
