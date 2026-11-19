# Builds dist\QuickTooth-Setup-<version>.exe: builds QuickTooth's one exe (Rust; it needs no
# runtime on the user's PC) and wraps it with Inno Setup 6.7+.
#   winget install JRSoftware.InnoSetup     # once, for ISCC.exe
#   .\installer\build.ps1
# Needs Rust (rustup; the repo's rust-toolchain.toml picks the version) and Visual Studio (or
# Build Tools) with "Desktop development with C++", for its libraries and the Windows SDK.
$ErrorActionPreference = 'Stop'
$root   = Split-Path $PSScriptRoot
$repo   = Split-Path (Split-Path $root)
$crate  = Join-Path $root 'win\Cargo.toml'
$stage  = Join-Path $PSScriptRoot 'obj\app'
$target = Join-Path $PSScriptRoot 'obj\cargo'

$version = Select-String -Path $crate -Pattern '^version\s*=\s*"([^"]+)"' | Select-Object -First 1 | ForEach-Object { $_.Matches[0].Groups[1].Value }
if (-not $version) { throw "no version in $crate" }

$iscc = @(
    (Get-Command ISCC.exe -ErrorAction SilentlyContinue).Source
    Join-Path $env:LOCALAPPDATA 'Programs\Inno Setup 6\ISCC.exe'
    Join-Path ${env:ProgramFiles(x86)} 'Inno Setup 6\ISCC.exe'
    Join-Path $env:ProgramFiles 'Inno Setup 6\ISCC.exe'
) | Where-Object { $_ -and (Test-Path $_) } | Select-Object -First 1
if (-not $iscc) { throw 'ISCC.exe not found. Install Inno Setup: winget install JRSoftware.InnoSetup' }

if (Test-Path $stage) { Remove-Item $stage -Recurse -Force }
New-Item -ItemType Directory $stage | Out-Null
# From the repo root, so cargo finds rust-toolchain.toml and .cargo/config.toml.
Push-Location $repo
try { cargo build --release -p quicktooth --target-dir $target } finally { Pop-Location }
if ($LASTEXITCODE -ne 0) { throw 'cargo build failed' }
Copy-Item (Join-Path $target 'release\QuickTooth.exe') $stage

& $iscc /Qp "/DAppVersion=$version" "/DAppSource=$stage" (Join-Path $PSScriptRoot 'QuickTooth.iss')
if ($LASTEXITCODE -ne 0) { throw 'ISCC failed' }

$setup = Get-Item (Join-Path $root "dist\QuickTooth-Setup-$version.exe")
Write-Host ("Built {0} ({1:N1} MB)" -f $setup.FullName, ($setup.Length / 1e6))
