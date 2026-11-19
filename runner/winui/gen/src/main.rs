//! Regenerates `runner/winui/src/bindings.rs`, the Windows App SDK (WinUI 3) bindings of
//! PocketKnives.Settings.exe, from the winmds in a staged payload folder (`stage.ps1` puts them
//! there). Windows.* types come from the `windows` crate; only Microsoft.* types are generated.
//!
//! `cargo run -p pocketknives-settings-gen -- <payload folder>`
//!
//! The methods are then trimmed to the ones the app's sources name ([`trim`]): windows-rs gives
//! every class all the methods of its base classes, about 2,000 lines per control.
use std::collections::BTreeSet;
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
    "Microsoft.UI.Xaml.Vector3Transition",
    "Microsoft.UI.Xaml.CornerRadius",
    "Microsoft.UI.Xaml.GridLength",
    "Microsoft.UI.Xaml.HorizontalAlignment",
    "Microsoft.UI.Xaml.VerticalAlignment",
    "Microsoft.UI.Xaml.TextWrapping",
    "Microsoft.UI.Xaml.Automation.AutomationProperties",
    "Microsoft.UI.Xaml.Controls.BitmapIcon",
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
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../src");
    let out = src.join("bindings.rs");
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
    let code = std::fs::read_to_string(&out).expect("read the bindings");
    let (code, fixed) = delegate_signatures(&code);
    println!("delegate signatures fixed: {fixed}");
    let (code, kept, removed) = trim(&code, &app_names(&src));
    assert!(removed > 0, "trimming removed no methods: is the generated layout still `pub fn` blocks?");
    println!("methods kept: {kept}, removed: {removed}");
    std::fs::write(&out, code).expect("write the bindings");
}

/// Every identifier in the app's own sources (src/*.rs but the bindings), and the methods the
/// generated code calls on itself (`self.First()` in its `IntoIterator` impls).
fn app_names(src: &Path) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for entry in std::fs::read_dir(src).expect("read src") {
        let path = entry.expect("read src").path();
        if path.extension().is_some_and(|e| e == "rs") && !path.ends_with("bindings.rs") {
            let code = std::fs::read_to_string(&path).expect("read a source file");
            names.extend(code.split(|c: char| !(c.is_alphanumeric() || c == '_')).filter(|w| !w.is_empty()).map(String::from));
        }
    }
    names
}

/// Removes each `pub fn` whose name isn't in `used` and isn't called by the generated code
/// itself. Vtables stay whole (their layout is the ABI); only the safe wrappers go. A wrapper is
/// rustfmt's layout: from its `pub fn` line to the `}` at the same indent.
fn trim(code: &str, used: &BTreeSet<String>) -> (String, usize, usize) {
    let lines: Vec<&str> = code.lines().collect();
    let mut keep: BTreeSet<String> = used.clone();
    for line in &lines {
        let mut rest = *line;
        while let Some(at) = rest.find("self.") {
            rest = &rest[at + 5..];
            let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
            if rest[name.len()..].starts_with('(') {
                keep.insert(name);
            }
        }
    }
    let mut out = String::with_capacity(code.len());
    let (mut kept, mut removed) = (0, 0);
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let indent = line.len() - line.trim_start().len();
        if let Some(rest) = line.trim_start().strip_prefix("pub fn ") {
            let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
            let end = if line.trim_end().ends_with('}') {
                i
            } else {
                let close = format!("{}}}", &line[..indent]);
                (i + 1..lines.len()).find(|&j| lines[j].trim_end() == close).expect("the end of a method")
            };
            if !keep.contains(&name) {
                removed += 1;
                i = end + 1;
                continue;
            }
            kept += 1;
        }
        out.push_str(line);
        out.push('\n');
        i += 1;
    }
    (out, kept, removed)
}

/// windows-rs signs a delegate like an interface, `{iid}`, where WinRT signs it `delegate({iid})`
/// (no "delegate(" exists in windows-rs 0.62). The signature makes the IIDs of generic interfaces
/// over the delegate: XAML's `AddHandler` asks the boxed handler for the real
/// `IReference<KeyEventHandler>` (BF4375EF-DACF-532E-90D9-AA1202B6055F) and windows-rs's box
/// answers another. Delegates are the types whose `RuntimeType` impl is followed by a
/// closure-taking `new`.
fn delegate_signatures(code: &str) -> (String, usize) {
    const INTERFACE: &str = "windows_core::imp::ConstBuffer::for_interface::<Self>();";
    const DELEGATE: &str = "windows_core::imp::ConstBuffer::new().push_slice(b\"delegate(\").push_other(windows_core::imp::ConstBuffer::for_interface::<Self>()).push_slice(b\")\");";
    let mut out = String::with_capacity(code.len() + 4096);
    let mut fixed = 0;
    let mut rest = code;
    while let Some(at) = rest.find(INTERFACE) {
        let after = &rest[at + INTERFACE.len()..];
        // `}` closing the RuntimeType impl, then `impl Name {` and `pub fn new<` with an `F: Fn`.
        let tail: String = after.chars().filter(|c| !c.is_whitespace()).take(200).collect();
        let is_delegate = tail.starts_with('}') && tail[1..].starts_with("impl") && tail.contains("{pubfnnew<F:Fn(");
        out.push_str(&rest[..at]);
        out.push_str(if is_delegate { DELEGATE } else { INTERFACE });
        fixed += is_delegate as usize;
        rest = after;
    }
    out.push_str(rest);
    (out, fixed)
}
