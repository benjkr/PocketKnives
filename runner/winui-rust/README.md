# PocketKnives Settings in Rust (spike)

A feasibility spike: the runner's Windows 11 settings window (today `runner/WinUI`, C# Native AOT)
as a `no_std` Rust exe on `crates/pk`, with no .NET or MSBuild in the build. It shows the C#
window's layout and controls; the settings behind them are stubs (`src/stub.rs`), and a change is
only reported on the page's event line. Folder name: `runner/winui` would be `runner/WinUI` on
Windows.

## Build and run

```powershell
cargo build --release -p pocketknives-settings
pwsh runner/winui-rust/stage.ps1 -Out $env:TEMP\settings -Exe target/release/pocketknives-settings.exe
& "$env:TEMP\settings\PocketKnives.Settings.exe" 1
```

- `cargo build` needs no network: the WinUI bindings (`src/bindings.rs`) and the manifest
  (`app.manifest`) are generated files, committed.
- `stage.ps1` downloads the pinned Windows App SDK 2.6.5 and WebView2 packages from nuget.org
  (SHA-512 checked, cached in `%TEMP%\pocketknives-winappsdk`) and lays out the runtime next to the
  exe, as the SDK's self-contained MSBuild targets do. It also copies WinUI's
  `Microsoft.UI.Xaml.Controls.pri` as `PocketKnives.Settings.pri`: without a resource index the
  control styles don't load.
- The argument is the runner's window handle (any non-zero integer). Exit codes: 0 closed, 2 bad
  arguments, 3 no Windows App SDK runtime next to the exe, 4 WinUI failed. When stdout goes to a
  pipe or a file, a failure prints one line saying why.

## Regenerating

- Bindings, after changing `gen/src/main.rs`'s type list or the packages: stage first, then
  `cargo run -p pocketknives-settings-gen -- <staged folder>` (it reads the folder's winmds).
- Manifest, after changing the packages: `stage.ps1 -Out <folder> -Manifest runner/winui-rust/app.manifest`.
  It is byte-identical to the C# build's `WindowsAppSDK.manifest`; keep `.gitattributes`' `-text`.

## How it works

- `src/app.rs`: the `Application` subclass by COM aggregation, with hand-written vtables:
  `IApplicationOverrides` (`OnLaunched`) and `IXamlMetadataProvider` (delegating to
  `XamlControlsXamlMetaDataProvider`, without which the control styles fail to load); every other
  interface goes to the inner `Application`.
- `src/main.rs`: what the SDK's self-contained auto-initializer does in the C# app (set
  `MICROSOFT_WINDOWSAPPRUNTIME_BASE_DIRECTORY`, load `Microsoft.WindowsAppRuntime.dll`, call
  `WindowsAppRuntime_EnsureIsLoaded`), then `Application.Start`.
- `src/boxed.rs`: `AddHandler` takes the handler boxed as `IReference<KeyEventHandler>`. windows-rs
  signs delegates `{iid}` instead of WinRT's `delegate({iid})`, so the gen tool patches the
  generated delegates' signatures; otherwise the box's IID is wrong and `AddHandler` fails with
  `E_NOINTERFACE`.
