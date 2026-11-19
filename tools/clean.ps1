<#
.SYNOPSIS
Deletes the Rust and installer build output (target\ and installer\obj\) in this repo's checkout
and every one of its git worktrees, to get the disk space back.

.DESCRIPTION
Each worktree builds into its own target\ and installer\obj\, and they grow to hundreds of MB
each. They are only caches: the next build makes them again. Nothing else is touched: not dist\
(the built installers), not the Windows App SDK download cache, not the worktrees themselves.

A folder in use by a build or a running exe is skipped. The folder is first renamed to
<name>.pk-clean, which Windows refuses while any file inside is open, so a running build is never
left half-deleted. A leftover <name>.pk-clean (a file that was still locked) is deleted next time.

.EXAMPLE
pwsh tools/clean.ps1 -WhatIf     # lists what would go and how much it frees
.EXAMPLE
pwsh tools/clean.ps1
#>
[CmdletBinding(SupportsShouldProcess)]
param()
$ErrorActionPreference = 'Stop'

function Get-FolderBytes([string]$Path) {
    $sum = (Get-ChildItem -LiteralPath $Path -Recurse -File -Force -ErrorAction SilentlyContinue |
        Measure-Object Length -Sum).Sum
    if ($sum) { [long]$sum } else { 0L }
}
function Format-Size([long]$Bytes) {
    if ($Bytes -ge 1GB) { '{0:N1} GB' -f ($Bytes / 1GB) } else { '{0:N0} MB' -f ($Bytes / 1MB) }
}

$checkouts = git -C $PSScriptRoot worktree list --porcelain |
    Where-Object { $_ -like 'worktree *' } |
    ForEach-Object { $_.Substring(9) } |
    Where-Object { Test-Path -LiteralPath $_ }

$freed = 0L
foreach ($checkout in $checkouts) {
    foreach ($relative in 'target', 'installer\obj') {
        $folder = Join-Path $checkout $relative
        $doomed = "$folder.pk-clean"
        if (-not (Test-Path -LiteralPath $folder) -and -not (Test-Path -LiteralPath $doomed)) { continue }

        if (-not $PSCmdlet.ShouldProcess($folder, 'Delete build output')) {
            $bytes = 0L
            foreach ($f in $folder, $doomed) { if (Test-Path -LiteralPath $f) { $bytes += Get-FolderBytes $f } }
            "Would remove $folder ($(Format-Size $bytes))"
            $freed += $bytes
            continue
        }

        if (Test-Path -LiteralPath $folder) {
            if (Test-Path -LiteralPath $doomed) {
                # A leftover from an earlier run: delete it first so the rename has a free name.
                Remove-Item -LiteralPath $doomed -Recurse -Force -ErrorAction SilentlyContinue
            }
            try {
                Rename-Item -LiteralPath $folder -NewName (Split-Path $doomed -Leaf)
            } catch {
                "Skipped $folder (in use)"
                continue
            }
        }
        $bytes = Get-FolderBytes $doomed
        Remove-Item -LiteralPath $doomed -Recurse -Force -ErrorAction SilentlyContinue
        $left = if (Test-Path -LiteralPath $doomed) { Get-FolderBytes $doomed } else { 0L }
        $freed += $bytes - $left
        if ($left) { "Removed $folder ($(Format-Size ($bytes - $left))); $(Format-Size $left) still locked, left in $doomed" }
        else { "Removed $folder ($(Format-Size $bytes))" }
    }
}
if ($WhatIfPreference) { "Would free $(Format-Size $freed)" } else { "Freed $(Format-Size $freed)" }
