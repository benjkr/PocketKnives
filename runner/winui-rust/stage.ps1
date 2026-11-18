<#
.SYNOPSIS
Lays out PocketKnives.Settings.exe's folder: the self-contained Windows App SDK runtime (x64), the
resource index and the exe, without .NET or MSBuild.

.DESCRIPTION
Downloads the pinned NuGet packages from nuget.org (cached in -Packages), checks each against its
pinned SHA-512 and copies the runtime files into -Out exactly as the Windows App SDK's MSBuild
targets do for a self-contained app (Microsoft.WindowsAppSDK.SelfContained.targets,
AddMicrosoftWindowsAppSDKPayloadFilesFromComponents, plus NuGet's native runtime assets).

-Manifest also writes the activatable-class manifest the exe embeds, generated from the packages'
package.appxfragment files as the targets' GenerateAppManifestFromAppxFragments task does (same
order, exclusions and bytes). The repo commits it as runner/winui-rust/app.manifest, so
`cargo build` needs no network; run with -Manifest only when the packages change.

.EXAMPLE
pwsh runner/winui-rust/stage.ps1 -Out $env:TEMP\settings -Exe target/release/pocketknives-settings.exe
.EXAMPLE
pwsh runner/winui-rust/stage.ps1 -Out $env:TEMP\settings -Manifest runner/winui-rust/app.manifest
#>
param(
    [Parameter(Mandatory)] [string]$Out,
    # The built exe (cargo names it pocketknives-settings.exe; Cargo bin names can't have dots).
    [string]$Exe,
    [string]$Manifest,
    [string]$Packages = (Join-Path ([IO.Path]::GetTempPath()) 'pocketknives-winappsdk')
)
$ErrorActionPreference = 'Stop'
$repo = Resolve-Path (Join-Path $PSScriptRoot '../..')

# What Microsoft.WindowsAppSDK.WinUI 2.6.5 (the C# app's only package reference) brings in.
# Component packages are listed in the order their props register them, which is the order of
# the generated manifest. Sha512: nuget.org's (the .nupkg.sha512 NuGet keeps beside a package).
$pins = @(
    @{ Id = 'Microsoft.WindowsAppSDK.InteractiveExperiences'; Version = '2.6.5'; Component = $true
       Sha512 = 'IIwhM0Qec/hdm3Y9r6awMAMTv9mR5FA+TWJvRJzai3Gs2fYL5j4ah4yChkEVVn89F6n1btiVxczUECT5vy5diw==' }
    @{ Id = 'Microsoft.WindowsAppSDK.Foundation'; Version = '2.6.5'; Component = $true
       Sha512 = 'Tr1MqgcGrxmRsxVXimykQyuCRe9pQfv0HWliR9gQA2Jk7cVngWmUZl4vbMFUzvlyEwa7W7edjDOizeTiiGvcDQ==' }
    @{ Id = 'Microsoft.WindowsAppSDK.WinUI'; Version = '2.6.5'; Component = $true
       Sha512 = 'eNmFFLMf3NrkvJDhXCHVfIYcS+xjldnWSHzRZt2REmtQsaRcJvdpfiqBVTqWrPntI30M+urxCQuDoag+4NH4rw==' }
    @{ Id = 'Microsoft.Web.WebView2'; Version = '1.0.3719.77'; Component = $false
       Sha512 = 'Cpq7EGgijiCN0lg1RlTZuZcRkzyo/USU9cEJMSPh6FG4PLjmTEpJ5NKuJqQ06ONnVVARCcTLf31IjjhFJUrApw==' }
)

function Get-Package($pin) {
    $name = "$($pin.Id).$($pin.Version)".ToLowerInvariant()
    $dir = Join-Path $Packages $name
    if (Test-Path (Join-Path $dir '.complete')) { return $dir }
    New-Item -ItemType Directory -Force $Packages | Out-Null
    $nupkg = Join-Path $Packages "$name.nupkg"
    $id = $pin.Id.ToLowerInvariant()
    Write-Host "Downloading $($pin.Id) $($pin.Version)"
    Invoke-WebRequest -Uri "https://api.nuget.org/v3-flatcontainer/$id/$($pin.Version)/$name.nupkg" -OutFile $nupkg
    $hash = (Get-FileHash -Algorithm SHA512 $nupkg).Hash
    $pinned = [Convert]::ToHexString([Convert]::FromBase64String($pin.Sha512))
    if ($hash -ne $pinned) {
        Remove-Item $nupkg
        throw "$name.nupkg doesn't match its pinned SHA-512."
    }
    if (Test-Path $dir) { Remove-Item -Recurse -Force $dir }
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    [IO.Compression.ZipFile]::ExtractToDirectory($nupkg, $dir)
    New-Item -ItemType File (Join-Path $dir '.complete') | Out-Null
    return $dir
}

function Copy-Into($file, $relative) {
    $target = Join-Path $Out $relative
    New-Item -ItemType Directory -Force (Split-Path $target) | Out-Null
    Copy-Item -LiteralPath $file -Destination $target -Force
}

# The targets' payload patterns, under runtimes-framework\win-x64\native (recursive).
$payload = @('*.dll', 'workloads*.json', 'restartAgent.exe', 'map.html', '*.mui', '*.png', '*.winmd', '*.xaml', '*.xbf', '*.pri')

New-Item -ItemType Directory -Force $Out | Out-Null
$fragments = @()
foreach ($pin in $pins) {
    $dir = Get-Package $pin
    if ($pin.Component) {
        $native = Join-Path $dir 'runtimes-framework\win-x64\native'
        foreach ($file in Get-ChildItem -LiteralPath $native -Recurse -File) {
            if ($payload | Where-Object { $file.Name -like $_ }) {
                Copy-Into $file.FullName $file.FullName.Substring($native.Length + 1)
            }
        }
        foreach ($meta in @('metadata', 'metadata\10.0.18362.0')) {
            Get-ChildItem -Path (Join-Path $dir "$meta\*.winmd") -File -ErrorAction SilentlyContinue |
                ForEach-Object { Copy-Into $_.FullName $_.Name }
        }
        $fragment = Join-Path $dir 'runtimes-framework\package.appxfragment'
        if (Test-Path $fragment) { $fragments += $fragment }
    }
    # NuGet's native runtime assets (Foundation's bootstrapper and background task, WebView2's
    # loader), and WebView2's WinRT component, which its targets copy for WinUI apps.
    foreach ($native in @('runtimes\win-x64\native', 'runtimes\win-x64\native_uap')) {
        if ($native -like '*_uap' -and $pin.Id -ne 'Microsoft.Web.WebView2') { continue }
        Get-ChildItem -Path (Join-Path $dir "$native\*") -File -ErrorAction SilentlyContinue |
            ForEach-Object { Copy-Into $_.FullName $_.Name }
    }
}

# The resource index. The C# build merges the packages' .pri files (and its App.xaml) into
# PocketKnives.Settings.pri; without one, XamlControlsResources fails ("Cannot locate resource
# from 'ms-appx:///Microsoft.UI.Xaml/Themes/themeresources.xaml'"). A code-only app needs no
# resources of its own, and WinUI's controls index alone, under the exe's name, is enough.
Copy-Item -LiteralPath (Join-Path $Out 'Microsoft.UI.Xaml.Controls.pri') -Destination (Join-Path $Out 'PocketKnives.Settings.pri') -Force

if ($Exe) {
    Copy-Item -LiteralPath $Exe -Destination (Join-Path $Out 'PocketKnives.Settings.exe') -Force
    Copy-Item -LiteralPath (Join-Path $repo 'assets\pocketknives.ico') -Destination (Join-Path $Out 'pocketknives.ico') -Force
}

if ($Manifest) {
    # GenerateAppManifestFromAppxFragments with RedirectDlls = false (not single-file). Its header
    # is a literal from an XML file (so LF line ends), the rest AppendLine (CRLF), UTF-8 with BOM.
    $sb = [Text.StringBuilder]::new()
    [void]$sb.Append("<?xml version='1.0' encoding='utf-8' standalone='yes'?>`n<assembly manifestVersion='1.0'`n    xmlns:asmv3='urn:schemas-microsoft-com:asm.v3'`n    xmlns:winrtv1='urn:schemas-microsoft-com:winrt.v1'`n    xmlns='urn:schemas-microsoft-com:asm.v1'>`r`n")
    foreach ($fragment in $fragments) {
        $doc = [Xml.XmlDocument]::new()
        $doc.Load($fragment)
        $ns = [Xml.XmlNamespaceManager]::new($doc.NameTable)
        $ns.AddNamespace('m', 'http://schemas.microsoft.com/appx/manifest/foundation/windows10')
        foreach ($server in $doc.SelectNodes('./m:Fragment/m:Extensions/m:Extension/m:InProcessServer', $ns)) {
            [void]$sb.Append("    <asmv3:file name='$($server.SelectSingleNode('./m:Path', $ns).InnerText)'>`r`n")
            foreach ($class in $server.SelectNodes('./m:ActivatableClass', $ns)) {
                [void]$sb.Append("        <winrtv1:activatableClass name='$($class.GetAttribute('ActivatableClassId'))' threadingModel='both'/>`r`n")
            }
            [void]$sb.Append("    </asmv3:file>`r`n")
        }
        foreach ($stub in $doc.SelectNodes('./m:Fragment/m:Extensions/m:Extension/m:ProxyStub', $ns)) {
            $path = $stub.SelectSingleNode('./m:Path', $ns).InnerText
            # The targets leave these out: the first needs the Singleton package (unavailable to
            # self-contained apps), the second isn't verified by the Widgets team.
            if ($path -in @('PushNotificationsLongRunningTask.ProxyStub.dll', 'Microsoft.Windows.Widgets.dll')) { continue }
            [void]$sb.Append("    <asmv3:file name='$path'>`r`n")
            $first = $true
            foreach ($interface in $stub.SelectNodes('./m:Interface', $ns)) {
                if ($first) { [void]$sb.Append("        <asmv3:comClass clsid='{$($stub.GetAttribute('ClassId'))}'/>"); $first = $false }
                [void]$sb.Append("        <asmv3:comInterfaceProxyStub name='$($interface.GetAttribute('Name'))' iid='{$($interface.GetAttribute('InterfaceId'))}'/>`r`n")
            }
            [void]$sb.Append("    </asmv3:file>`r`n")
        }
    }
    [void]$sb.Append("</assembly>`r`n")
    [IO.File]::WriteAllText([IO.Path]::GetFullPath($Manifest, (Get-Location).Path), $sb.ToString(), [Text.UTF8Encoding]::new($true))
}
