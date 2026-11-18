//! Regenerates `runner/winui-rust/src/bindings.rs`, the Windows App SDK (WinUI 3) bindings of
//! PocketKnives.Settings.exe, from the winmds in a staged payload folder (`stage.ps1` puts them
//! there). Windows.* types come from the `windows` crate; only Microsoft.* types are generated.
//!
//! `cargo run -p pocketknives-settings-gen -- <payload folder>`
use std::path::Path;

/// The WinUI types the settings window uses. Their dependencies come along.
const TYPES: &[&str] = &[
    "Microsoft.UI.Xaml.Application",
    "Microsoft.UI.Xaml.IApplicationFactory",
    "Microsoft.UI.Xaml.IApplicationOverrides",
    "Microsoft.UI.Xaml.ApplicationInitializationCallback",
    "Microsoft.UI.Xaml.LaunchActivatedEventArgs",
    "Microsoft.UI.Xaml.Window",
    "Microsoft.UI.Xaml.WindowEventArgs",
    "Microsoft.UI.Xaml.ResourceDictionary",
    "Microsoft.UI.Xaml.UIElement",
    "Microsoft.UI.Xaml.FrameworkElement",
    "Microsoft.UI.Xaml.RoutedEventArgs",
    "Microsoft.UI.Xaml.RoutedEventHandler",
    "Microsoft.UI.Xaml.Thickness",
    "Microsoft.UI.Xaml.CornerRadius",
    "Microsoft.UI.Xaml.GridLength",
    "Microsoft.UI.Xaml.HorizontalAlignment",
    "Microsoft.UI.Xaml.VerticalAlignment",
    "Microsoft.UI.Xaml.TextWrapping",
    "Microsoft.UI.Xaml.Automation.AutomationProperties",
    "Microsoft.UI.Xaml.Controls.Border",
    "Microsoft.UI.Xaml.Controls.Button",
    "Microsoft.UI.Xaml.Controls.ColumnDefinition",
    "Microsoft.UI.Xaml.Controls.ComboBox",
    "Microsoft.UI.Xaml.Controls.ContentControl",
    "Microsoft.UI.Xaml.Controls.Grid",
    "Microsoft.UI.Xaml.Controls.Image",
    "Microsoft.UI.Xaml.Controls.ImageIcon",
    "Microsoft.UI.Xaml.Controls.InfoBar",
    "Microsoft.UI.Xaml.Controls.NavigationView",
    "Microsoft.UI.Xaml.Controls.NavigationViewItem",
    "Microsoft.UI.Xaml.Controls.NavigationViewItemHeader",
    "Microsoft.UI.Xaml.Controls.NavigationViewSelectionChangedEventArgs",
    "Microsoft.UI.Xaml.Controls.ScrollViewer",
    "Microsoft.UI.Xaml.Controls.SelectionChangedEventHandler",
    "Microsoft.UI.Xaml.Controls.StackPanel",
    "Microsoft.UI.Xaml.Controls.TextBlock",
    "Microsoft.UI.Xaml.Controls.ToggleSwitch",
    "Microsoft.UI.Xaml.Controls.XamlControlsResources",
    "Microsoft.UI.Xaml.Input.KeyEventHandler",
    "Microsoft.UI.Xaml.Input.KeyRoutedEventArgs",
    "Microsoft.UI.Xaml.Markup.IXamlMetadataProvider",
    "Microsoft.UI.Xaml.Media.MicaBackdrop",
    "Microsoft.UI.Xaml.Media.SolidColorBrush",
    "Microsoft.UI.Xaml.Media.Imaging.BitmapImage",
    "Microsoft.UI.Xaml.XamlTypeInfo.XamlControlsXamlMetaDataProvider",
    "Microsoft.UI.Text.FontWeights",
    "Microsoft.UI.Colors",
    "Microsoft.UI.Windowing.AppWindow",
    // Base classes of the above (their conversions name them).
    "Microsoft.UI.Xaml.DependencyObject",
    "Microsoft.UI.Xaml.Controls.Control",
    "Microsoft.UI.Xaml.Controls.Panel",
    "Microsoft.UI.Xaml.Controls.IconElement",
    "Microsoft.UI.Xaml.Controls.ItemsControl",
    "Microsoft.UI.Xaml.Controls.NavigationViewItemBase",
    "Microsoft.UI.Xaml.Controls.Primitives.ButtonBase",
    "Microsoft.UI.Xaml.Controls.Primitives.Selector",
    "Microsoft.UI.Xaml.Media.Brush",
    "Microsoft.UI.Xaml.Media.ImageSource",
    "Microsoft.UI.Xaml.Media.SystemBackdrop",
    "Microsoft.UI.Xaml.Media.Imaging.BitmapSource",
    // Types in the signatures the window calls.
    "Microsoft.UI.Xaml.DependencyProperty",
    "Microsoft.UI.Xaml.RoutedEvent",
    "Microsoft.UI.Xaml.Controls.UIElementCollection",
    "Microsoft.UI.Xaml.Controls.ItemCollection",
    "Microsoft.UI.Xaml.GridUnitType",
    "Microsoft.UI.Xaml.Controls.ColumnDefinitionCollection",
    "Microsoft.UI.Xaml.Controls.SelectionChangedEventArgs",
    "Microsoft.UI.Xaml.Controls.InfoBarSeverity",
    "Microsoft.UI.Xaml.Controls.NavigationViewPaneDisplayMode",
    "Microsoft.UI.Xaml.Controls.NavigationViewBackButtonVisible",
    "Microsoft.UI.Xaml.Markup.IXamlType",
    "Microsoft.UI.Xaml.Markup.XmlnsDefinition",
    "Windows.UI.Xaml.Interop.TypeName",
    "Windows.UI.Xaml.Interop.TypeKind",
];

/// Windows types taken from windows-rs crates other than `windows`, as windows 0.62 does.
const REFERENCES: &[&str] = &[
    "windows_collections,flat,Windows.Foundation.Collections.IIterable",
    "windows_collections,flat,Windows.Foundation.Collections.IIterator",
    "windows_collections,flat,Windows.Foundation.Collections.IKeyValuePair",
    "windows_collections,flat,Windows.Foundation.Collections.IMap",
    "windows_collections,flat,Windows.Foundation.Collections.IMapView",
    "windows_collections,flat,Windows.Foundation.Collections.IVector",
    "windows_collections,flat,Windows.Foundation.Collections.IVectorView",
    "windows_numerics,flat,Windows.Foundation.Numerics",
    "windows_future,flat,Windows.Foundation.Async*",
    "windows_future,flat,Windows.Foundation.IAsync*",
];

/// The Windows namespaces and types taken from the `windows` crate.
const WINDOWS: &[&str] = &[
    "Windows.ApplicationModel",
    "Windows.Foundation",
    "Windows.Graphics",
    "Windows.Media",
    "Windows.Storage",
    "Windows.System",
    "Windows.UI.Color",
    "Windows.UI.Core",
    "Windows.UI.Text",
];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [payload] = args.as_slice() else {
        eprintln!("usage: pocketknives-settings-gen <payload folder with the Windows App SDK winmds>");
        std::process::exit(2);
    };
    let out = Path::new(env!("CARGO_MANIFEST_DIR")).join("../src/bindings.rs");
    let mut bindgen: Vec<String> = vec!["--in".into(), "default".into()];
    // Every winmd in the payload: the types above refer to types across several of them.
    for entry in std::fs::read_dir(payload).expect("read the payload folder") {
        let path = entry.expect("read the payload folder").path();
        if path.extension().is_some_and(|e| e.eq_ignore_ascii_case("winmd")) {
            bindgen.push(path.display().to_string());
        }
    }
    bindgen.extend(["--out".into(), out.display().to_string()]);
    // Windows.* comes from the windows-rs crates, except Windows.UI.Xaml (UWP XAML), which they
    // no longer have: WinUI still uses Windows.UI.Xaml.Interop.TypeName, so that is generated
    // here. `--no-deps` and explicit references: bindgen 0.66's own defaults send all of
    // Windows.Foundation.Collections to windows-collections, whose 0.3 (windows 0.62's) has only
    // some of it. The first reference that matches a type wins.
    bindgen.push("--no-deps".into());
    for reference in REFERENCES.iter().map(|r| r.to_string()).chain(WINDOWS.iter().map(|n| format!("windows,skip-root,{n}"))) {
        bindgen.extend(["--reference".into(), reference]);
    }
    bindgen.extend(["--no-comment".into(), "--filter".into()]);
    bindgen.extend(TYPES.iter().map(|t| t.to_string()));
    let warnings = windows_bindgen::bindgen(&bindgen);
    print!("{warnings}");
}
