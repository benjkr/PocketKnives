# Builds dist\PocketKnives-Setup-<version>.exe: every blade, built from this checkout, in one
# per-user installer (Inno Setup 6.7+).
#   winget install JRSoftware.InnoSetup     # once, for ISCC.exe
#   .\installer\build.ps1
# Needs what each blade's build needs: Visual Studio (or Build Tools) with "Desktop development
# with C++" (SnipSnap, and QuickTooth's native linker) and the .NET 10 SDK (QuickTooth).
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

# QuickTooth: one native exe (Native AOT), as its own installer builds it.
$qt = Join-Path $root 'blades\quicktooth\src\QuickTooth\QuickTooth.csproj'
$qtOut = Join-Path $PSScriptRoot 'obj\quicktooth'
dotnet publish $qt -c Release -r win-x64 -nologo -o $qtOut
if ($LASTEXITCODE -ne 0) { throw 'QuickTooth publish failed' }
Copy-Item (Join-Path $qtOut 'QuickTooth.exe') $stage

& $iscc /Qp "/DAppSource=$stage" (Join-Path $PSScriptRoot 'PocketKnives.iss')
if ($LASTEXITCODE -ne 0) { throw 'ISCC failed' }

Get-ChildItem (Join-Path $root 'dist') -Filter 'PocketKnives-Setup-*.exe' |
    Sort-Object LastWriteTime | Select-Object -Last 1 |
    ForEach-Object { "Built $($_.FullName) ($([math]::Round($_.Length / 1MB, 1)) MB)" }
