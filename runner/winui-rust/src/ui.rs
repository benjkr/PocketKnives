//! The settings window, built in code as the C# app builds it (runner/WinUI/Program.cs): a
//! NavigationView with General and the blades, each blade's sign-in switch in the pane, and each
//! page's settings as cards. Changes go to [`stub`] and are reported on the event line under the
//! cards, which the spike adds to show that events arrive.
use crate::bindings::Microsoft::UI::Colors;
use crate::bindings::Microsoft::UI::Text::FontWeights;
use crate::bindings::Microsoft::UI::Xaml::Automation::AutomationProperties;
use crate::bindings::Microsoft::UI::Xaml::Controls::{
    Border, Button, ColumnDefinition, ComboBox, ContentControl, Grid, Image, ImageIcon, InfoBar, InfoBarSeverity, NavigationView,
    NavigationViewBackButtonVisible, NavigationViewItem, NavigationViewItemHeader, NavigationViewPaneDisplayMode,
    NavigationViewSelectionChangedEventArgs, ScrollViewer, SelectionChangedEventHandler, StackPanel, TextBlock, ToggleSwitch,
    XamlControlsResources,
};
use crate::bindings::Microsoft::UI::Xaml::Input::{KeyEventHandler, KeyRoutedEventArgs};
use crate::bindings::Microsoft::UI::Xaml::Media::Imaging::BitmapImage;
use crate::bindings::Microsoft::UI::Xaml::Media::{MicaBackdrop, SolidColorBrush};
use crate::bindings::Microsoft::UI::Xaml::{
    Application, CornerRadius, FrameworkElement, GridLength, GridUnitType, HorizontalAlignment, ResourceDictionary, RoutedEventArgs,
    RoutedEventHandler, TextWrapping, Thickness, UIElement, VerticalAlignment, Window,
};
use crate::stub::{self, Kind};
use alloc::format;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::{Cell, OnceCell, RefCell};
use windows::Foundation::{IReference, PropertyValue, TypedEventHandler, Uri};
use windows::Graphics::SizeInt32;
use windows_core::{HRESULT, HSTRING, IInspectable, IUnknown, IUnknown_Vtbl, Interface, Ref, Result};
use windows_sys::Win32::UI::HiDpi::GetDpiForWindow;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetKeyState;

/// What the handlers share. Only the UI thread touches it: XAML calls handlers on the thread that
/// made the element.
struct Ui {
    window: Window,
    page: ContentControl,
    status: InfoBar,
    events: TextBlock,
    root: String,
    suite_icon: String,
    saved: RefCell<Vec<Vec<String>>>,
    updating: Cell<bool>,
    ready: Cell<bool>,
}

struct Global(OnceCell<Ui>);
// The UI thread is the only one that reads or writes it (see `Ui`).
unsafe impl Sync for Global {}
static UI: Global = Global(OnceCell::new());

fn ui() -> &'static Ui {
    // Set in `launched` before any element exists, so before any handler can run.
    UI.0.get().unwrap_or_else(|| pk::exit(crate::EXIT_WINUI))
}

/// A value a handler captures. Handlers must be `Send` in windows-rs, but XAML only ever calls
/// them on the UI thread, where the value was made.
struct Local<T>(T);
unsafe impl<T> Send for Local<T> {}
impl<T> Local<T> {
    /// Through a method, so a closure captures the whole `Local`, not just the value inside.
    fn get(&self) -> &T {
        &self.0
    }
}

#[windows_core::interface("EECDBF0E-BAE9-4CB6-A68E-9598E1CB57BB")]
unsafe trait IWindowNative: IUnknown {
    fn window_handle(&self, hwnd: *mut isize) -> HRESULT;
}

/// `OnLaunched`: the control styles (what App.xaml does in C#), then the window.
pub fn launched(app: &Application) -> Result<()> {
    let resources = ResourceDictionary::new()?;
    resources.MergedDictionaries()?.Append(&XamlControlsResources::new()?)?;
    app.SetResources(&resources)?;

    let dir = pk::env::exe_dir();
    let root = dir.rsplit_once('\\').map_or(dir.as_str(), |(parent, _)| parent).into();
    let saved = (0..stub::BLADES.len()).map(|b| stub::settings(b).iter().map(|s| String::from(s.value)).collect()).collect();
    let state = Ui {
        window: Window::new()?,
        page: ContentControl::new()?,
        status: InfoBar::new()?,
        events: TextBlock::new()?,
        root,
        suite_icon: format!("{dir}\\pocketknives.ico"),
        saved: RefCell::new(saved),
        updating: Cell::new(false),
        ready: Cell::new(false),
    };
    if UI.0.set(state).is_err() {
        return Err(windows_core::Error::empty());
    }
    let ui = ui();
    ui.window.SetTitle(&HSTRING::from(stub::TITLE))?;
    ui.window.SetSystemBackdrop(&MicaBackdrop::new()?)?;
    ui.page.SetHorizontalContentAlignment(HorizontalAlignment::Stretch)?;
    ui.status.SetSeverity(InfoBarSeverity::Error)?;
    ui.status.SetIsClosable(true)?;
    ui.window.AppWindow()?.SetIcon(&HSTRING::from(&ui.suite_icon))?;
    ui.window.SetContent(&build()?)?;
    // AppWindow.Resize uses physical pixels.
    let mut hwnd = 0isize;
    unsafe { ui.window.cast::<IWindowNative>()?.window_handle(&mut hwnd) }.ok()?;
    let scale = unsafe { GetDpiForWindow(hwnd as _) } as f64 / 96.0;
    ui.window.AppWindow()?.Resize(SizeInt32 { Width: (830.0 * scale) as i32, Height: (500.0 * scale) as i32 })?;
    ui.window.Activate()?;
    ui.ready.set(true);
    Ok(())
}

fn build() -> Result<NavigationView> {
    let ui = ui();
    let navigation = NavigationView::new()?;
    navigation.SetPaneDisplayMode(NavigationViewPaneDisplayMode::Left)?;
    navigation.SetIsPaneOpen(true)?;
    navigation.SetIsPaneToggleButtonVisible(false)?;
    navigation.SetIsBackButtonVisible(NavigationViewBackButtonVisible::Collapsed)?;
    navigation.SetIsSettingsVisible(false)?;
    navigation.SetOpenPaneLength(252.0)?;
    navigation.SetHorizontalContentAlignment(HorizontalAlignment::Stretch)?;
    let footer = text("On: starts at sign-in")?;
    footer.SetMargin(Thickness { Left: 16.0, Top: 8.0, Right: 8.0, Bottom: 12.0 })?;
    navigation.SetPaneFooter(&footer)?;
    let items = navigation.MenuItems()?;
    let mut first = None;
    for i in 0..=stub::BLADES.len() {
        if i == 1 {
            let header = NavigationViewItemHeader::new()?;
            header.SetContent(&boxed(stub::BLADES_LABEL)?)?;
            items.Append(&header)?;
        }
        let item = NavigationViewItem::new()?;
        let icon = ImageIcon::new()?;
        icon.SetSource(&load_icon(&page_icon(i), 16)?)?;
        item.SetIcon(&icon)?;
        if i == 0 {
            item.SetContent(&boxed(stub::GENERAL_NAME)?)?;
        } else {
            item.SetContent(&blade_row(i - 1)?)?;
        }
        item.SetTag(&PropertyValue::CreateInt32(i as i32)?)?;
        items.Append(&item)?;
        first.get_or_insert(item);
    }

    let content = StackPanel::new()?;
    content.SetSpacing(12.0)?;
    content.SetMargin(uniform(24.0))?;
    let children = content.Children()?;
    children.Append(&ui.status)?;
    children.Append(&ui.page)?;
    children.Append(&ui.events)?;
    let scroll = ScrollViewer::new()?;
    scroll.SetContent(&content)?;
    scroll.SetHorizontalContentAlignment(HorizontalAlignment::Stretch)?;
    navigation.SetContent(&scroll)?;
    navigation.SelectionChanged(&TypedEventHandler::new(
        |_, change: Ref<NavigationViewSelectionChangedEventArgs>| {
            let Some(change) = change.as_ref() else { return Ok(()) };
            let index = change.SelectedItem()?.cast::<FrameworkElement>()?.Tag()?.cast::<IReference<i32>>()?.Value()?;
            show_page(index as usize)?;
            event(&format!("Navigation: page {index}"))
        },
    ))?;
    if let Some(first) = first {
        navigation.SetSelectedItem(&first)?;
    }
    Ok(navigation)
}

fn page_icon(index: usize) -> String {
    match index {
        0 => ui().suite_icon.clone(),
        i => format!("{}\\icons\\{}", ui().root, stub::BLADES[i - 1].icon),
    }
}

fn blade_row(index: usize) -> Result<Grid> {
    let name = stub::BLADES[index].name;
    let row = Grid::new()?;
    row.SetWidth(167.0)?;
    two_columns(&row)?;
    let label = text(name)?;
    label.SetVerticalAlignment(VerticalAlignment::Center)?;
    row.Children()?.Append(&label)?;
    let toggle = ToggleSwitch::new()?;
    toggle.SetIsOn(true)?;
    toggle.SetOnContent(&boxed("")?)?;
    toggle.SetOffContent(&boxed("")?)?;
    toggle.SetMinWidth(0.0)?;
    AutomationProperties::SetName(&toggle, &HSTRING::from(format!("Start {name} at sign-in")))?;
    Grid::SetColumn(&toggle, 1)?;
    row.Children()?.Append(&toggle)?;
    toggle.Toggled(&RoutedEventHandler::new(move |sender, _| {
        let on = sender.as_ref().map(|s| s.cast::<ToggleSwitch>()).transpose()?.map(|t| t.IsOn()).transpose()?;
        event(&format!("Sign-in switch: {name} {}", if on == Some(true) { "on" } else { "off" }))
    }))?;
    Ok(row)
}

fn show_page(index: usize) -> Result<()> {
    let ui = ui();
    let blade = index.checked_sub(1).map(|b| &stub::BLADES[b]);
    let content = StackPanel::new()?;
    content.SetSpacing(20.0)?;
    let header = Grid::new()?;
    two_columns(&header)?;
    let texts = StackPanel::new()?;
    texts.SetSpacing(4.0)?;
    let title = text(blade.map_or(stub::GENERAL_NAME, |b| b.name))?;
    title.SetFontSize(28.0)?;
    title.SetFontWeight(FontWeights::SemiBold()?)?;
    texts.Children()?.Append(&title)?;
    let description = text(blade.map_or(stub::GENERAL_DESCRIPTION, |b| b.description))?;
    description.SetTextWrapping(TextWrapping::Wrap)?;
    texts.Children()?.Append(&description)?;
    header.Children()?.Append(&texts)?;
    let icon = Image::new()?;
    icon.SetSource(&load_icon(&page_icon(index), 48)?)?;
    icon.SetWidth(48.0)?;
    icon.SetHeight(48.0)?;
    icon.SetMargin(Thickness { Left: 12.0, Top: 0.0, Right: 0.0, Bottom: 0.0 })?;
    Grid::SetColumn(&icon, 1)?;
    header.Children()?.Append(&icon)?;
    content.Children()?.Append(&header)?;
    match index {
        0 => content.Children()?.Append(&look_card()?)?,
        b => {
            for (i, setting) in stub::settings(b - 1).iter().enumerate() {
                let control: FrameworkElement = match setting.kind {
                    Kind::Toggle => toggle_control(b - 1, i)?.cast()?,
                    Kind::Choice(choices) => choice_control(b - 1, i, choices)?.cast()?,
                    Kind::Hotkey => hotkey_control(b - 1, i)?.cast()?,
                };
                AutomationProperties::SetName(&control, &HSTRING::from(setting.label))?;
                content.Children()?.Append(&card(setting.label, &control)?)?;
            }
        }
    }
    ui.page.SetContent(&content)
}

/// A change to one of a blade's settings. The stub refuses only the "Refused" choice, to show
/// the error bar; the real app asks the blade (BladeSettings.Apply).
fn change(blade: usize, index: usize, value: String, shown: &str) -> Result<bool> {
    let ui = ui();
    if ui.saved.borrow()[blade][index] == value {
        return Ok(true);
    }
    let label = stub::settings(blade)[index].label;
    if shown.starts_with("Refused") {
        show_error(&format!("{} refused the change to {label}.", stub::BLADES[blade].name))?;
        event(&format!("Refused: {label} = {shown}"))?;
        return Ok(false);
    }
    ui.saved.borrow_mut()[blade][index] = value;
    ui.status.SetIsOpen(false)?;
    event(&format!("Changed: {label} = {shown}"))?;
    Ok(true)
}

fn toggle_control(blade: usize, index: usize) -> Result<ToggleSwitch> {
    let toggle = ToggleSwitch::new()?;
    toggle.SetIsOn(ui().saved.borrow()[blade][index] == "1")?;
    toggle.SetOnContent(&boxed("On")?)?;
    toggle.SetOffContent(&boxed("Off")?)?;
    toggle.SetMinWidth(0.0)?;
    toggle.Toggled(&RoutedEventHandler::new(move |sender, _: Ref<RoutedEventArgs>| {
        let ui = ui();
        let Some(toggle) = sender.as_ref() else { return Ok(()) };
        let toggle = toggle.cast::<ToggleSwitch>()?;
        let on = toggle.IsOn()?;
        if ui.updating.get() || change(blade, index, String::from(if on { "1" } else { "0" }), if on { "On" } else { "Off" })? {
            return Ok(());
        }
        ui.updating.set(true);
        toggle.SetIsOn(!on)?;
        ui.updating.set(false);
        Ok(())
    }))?;
    Ok(toggle)
}

fn choice_control(blade: usize, index: usize, choices: &'static [&'static str]) -> Result<ComboBox> {
    let combo = ComboBox::new()?;
    let items = combo.Items()?;
    for choice in choices {
        items.Append(&boxed(choice)?)?;
    }
    combo.SetSelectedIndex(ui().saved.borrow()[blade][index].parse().unwrap_or(-1))?;
    combo.SetMinWidth(200.0)?;
    combo.SelectionChanged(&SelectionChangedEventHandler::new(move |sender, _| {
        let ui = ui();
        let Some(combo) = sender.as_ref() else { return Ok(()) };
        let combo = combo.cast::<ComboBox>()?;
        let selected = combo.SelectedIndex()?;
        if ui.updating.get() || selected < 0 || change(blade, index, format!("{selected}"), choices[selected as usize])? {
            return Ok(());
        }
        ui.updating.set(true);
        let saved = ui.saved.borrow()[blade][index].parse().unwrap_or(-1);
        combo.SetSelectedIndex(saved)?;
        ui.updating.set(false);
        Ok(())
    }))?;
    Ok(combo)
}

/// WinUI has no hotkey box: click, then press the keys. Esc cancels.
fn hotkey_control(blade: usize, index: usize) -> Result<Button> {
    let button = Button::new()?;
    button.SetContent(&boxed(&stub::hotkey_name(&ui().saved.borrow()[blade][index]))?)?;
    button.SetMinWidth(200.0)?;
    let listening = Rc::new(Cell::new(false));
    let stop = move |button: &Button, listening: &Cell<bool>| -> Result<()> {
        listening.set(false);
        button.SetContent(&boxed(&stub::hotkey_name(&ui().saved.borrow()[blade][index]))?)
    };
    let l = Local(listening.clone());
    button.Click(&RoutedEventHandler::new(move |sender, _| {
        let Some(button) = sender.as_ref() else { return Ok(()) };
        if l.get().get() {
            return Ok(());
        }
        l.get().set(true);
        button.cast::<Button>()?.SetContent(&boxed("Press the new keys")?)
    }))?;
    let l = Local(listening.clone());
    button.LostFocus(&RoutedEventHandler::new(move |sender, _| match sender.as_ref() {
        Some(button) if l.get().get() => stop(&button.cast()?, l.get()),
        _ => Ok(()),
    }))?;
    let l = Local(listening);
    let handler = KeyEventHandler::new(move |sender, key: Ref<KeyRoutedEventArgs>| {
        let (Some(button), Some(key)) = (sender.as_ref(), key.as_ref()) else { return Ok(()) };
        let button = button.cast::<Button>()?;
        if !l.get().get() {
            return Ok(());
        }
        key.SetHandled(true)?;
        let code = key.Key()?.0;
        if code == 0x1b {
            event("Hotkey: Esc cancels")?;
            return stop(&button, l.get());
        }
        if matches!(code, 0x10 | 0x11 | 0x12 | 0xa0..=0xa5 | 0x5b | 0x5c) {
            return Ok(()); // a modifier alone
        }
        let modifiers = if down(0x11) { stub::CTRL } else { 0 }
            | if down(0x12) { stub::ALT } else { 0 }
            | if down(0x10) { stub::SHIFT } else { 0 }
            | if down(0x5b) || down(0x5c) { stub::WIN } else { 0 };
        if modifiers & !stub::SHIFT == 0 {
            event(&format!("Hotkey: key {code} without Ctrl, Alt or Win"))?;
            return button.SetContent(&boxed("Add Ctrl, Alt or Win")?);
        }
        let value = stub::join_hotkey(modifiers, code);
        let shown = stub::hotkey_name(&value);
        change(blade, index, value, &shown)?;
        stop(&button, l.get())
    });
    button.AddHandler(&UIElement::KeyDownEvent()?, &crate::boxed::key_handler(handler), true)?;
    Ok(button)
}

fn down(key: i32) -> bool {
    unsafe { GetKeyState(key) as u16 & 0x8000 != 0 }
}

fn card(label: &str, control: &FrameworkElement) -> Result<Border> {
    let row = Grid::new()?;
    row.SetPadding(uniform(16.0))?;
    row.SetColumnSpacing(12.0)?;
    two_columns(&row)?;
    let text = text(label)?;
    text.SetVerticalAlignment(VerticalAlignment::Center)?;
    text.SetTextWrapping(TextWrapping::Wrap)?;
    row.Children()?.Append(&text)?;
    control.SetVerticalAlignment(VerticalAlignment::Center)?;
    Grid::SetColumn(control, 1)?;
    row.Children()?.Append(control)?;
    let border = Border::new()?;
    border.SetChild(&row)?;
    border.SetCornerRadius(CornerRadius { TopLeft: 6.0, TopRight: 6.0, BottomRight: 6.0, BottomLeft: 6.0 })?;
    let brush = SolidColorBrush::CreateInstanceWithColor(Colors::Gray()?)?;
    brush.SetOpacity(0.12)?;
    border.SetBackground(&brush)?;
    Ok(border)
}

fn look_card() -> Result<Border> {
    let choices = ComboBox::new()?;
    let items = choices.Items()?;
    for name in stub::LOOK_NAMES {
        items.Append(&boxed(name)?)?;
    }
    choices.SetSelectedIndex(stub::LOOK_WINDOWS11)?;
    AutomationProperties::SetName(&choices, &HSTRING::from(stub::LOOK_LABEL))?;
    choices.SelectionChanged(&SelectionChangedEventHandler::new(|sender, _| {
        let Some(choices) = sender.as_ref() else { return Ok(()) };
        let choices = choices.cast::<ComboBox>()?;
        let selected = choices.SelectedIndex()?;
        if !(0..=1).contains(&selected) {
            return Ok(());
        }
        // The real app saves the look, asks the runner to reopen Settings and closes. The spike
        // never messages the runner's window.
        event(&format!(
            "Look: {} (the runner, window {}, isn't messaged in the spike)",
            stub::LOOK_NAMES[selected as usize],
            crate::runner_window()
        ))?;
        choices.SetSelectedIndex(stub::LOOK_WINDOWS11)
    }))?;
    card(stub::LOOK_LABEL, &choices.cast()?)
}

fn show_error(message: &str) -> Result<()> {
    let ui = ui();
    ui.status.SetMessage(&HSTRING::from(message))?;
    ui.status.SetIsOpen(true)
}

/// Shows an event on the page's event line (not the ones the window raises while it opens).
fn event(what: &str) -> Result<()> {
    let ui = ui();
    if !ui.ready.get() {
        return Ok(());
    }
    ui.events.SetText(&HSTRING::from(format!("Last event: {what}")))
}

fn load_icon(path: &str, size: i32) -> Result<BitmapImage> {
    let uri = Uri::CreateUri(&HSTRING::from(format!("file:///{}", path.replace('\\', "/"))))?;
    let image = BitmapImage::CreateInstanceWithUriSource(&uri)?;
    image.SetDecodePixelWidth(size)?;
    image.SetDecodePixelHeight(size)?;
    Ok(image)
}

fn text(text: &str) -> Result<TextBlock> {
    let block = TextBlock::new()?;
    block.SetText(&HSTRING::from(text))?;
    Ok(block)
}

fn boxed(text: &str) -> Result<IInspectable> {
    PropertyValue::CreateString(&HSTRING::from(text))
}

fn uniform(n: f64) -> Thickness {
    Thickness { Left: n, Top: n, Right: n, Bottom: n }
}

/// A star column and an auto column.
fn two_columns(grid: &Grid) -> Result<()> {
    let columns = grid.ColumnDefinitions()?;
    columns.Append(&ColumnDefinition::new()?)?;
    let auto = ColumnDefinition::new()?;
    auto.SetWidth(GridLength { Value: 1.0, GridUnitType: GridUnitType::Auto })?;
    columns.Append(&auto)
}

