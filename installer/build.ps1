# Builds dist\PocketKnives-Setup-<version>.exe: the runner and blades from this checkout, in one
# per-user installer (Inno Setup 6.7+).
#   winget install JRSoftware.InnoSetup     # once, for ISCC.exe
#   .\installer\build.ps1
# Needs what each part's build needs: Visual Studio (or Build Tools) with "Desktop development
# with C++" (the linker and resource compiler for every part), Rust through rustup (SnipSnap and
# QuickTooth; rust-toolchain.toml picks the version) and the .NET 10 SDK (the runner).
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

# QuickTooth: one exe (Rust), built from the repo root so cargo finds rust-toolchain.toml and
# .cargo/config.toml, into installer\obj.
$cargoOut = Join-Path $PSScriptRoot 'obj\cargo'
Push-Location $root
try { cargo build --release -p quicktooth --target-dir $cargoOut } finally { Pop-Location }
if ($LASTEXITCODE -ne 0) { throw 'QuickTooth build failed' }
Copy-Item (Join-Path $cargoOut 'release\QuickTooth.exe') $stage

$runner = Join-Path $root 'runner\pk.csproj'
dotnet publish $runner -c Release -nologo -o $stage
if ($LASTEXITCODE -ne 0) { throw 'PocketKnives runner publish failed' }

$winUi = Join-Path $root 'runner\WinUI\PocketKnives.Settings.csproj'
dotnet publish $winUi -c Release -nologo -o (Join-Path $stage 'winui')
if ($LASTEXITCODE -ne 0) { throw 'PocketKnives Windows 11 settings publish failed' }

& $iscc /Qp "/DAppSource=$stage" (Join-Path $PSScriptRoot 'PocketKnives.iss')
if ($LASTEXITCODE -ne 0) { throw 'ISCC failed' }

Get-ChildItem (Join-Path $root 'dist') -Filter 'PocketKnives-Setup-*.exe' |
    Sort-Object LastWriteTime | Select-Object -Last 1 |
    ForEach-Object { "Built $($_.FullName) ($([math]::Round($_.Length / 1MB, 1)) MB)" }
