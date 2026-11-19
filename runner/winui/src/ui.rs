//! The settings window, built in code (the C# app before it had no XAML pages either): a
//! NavigationView with General and the blades, each blade's sign-in switch in the pane, and each
//! page's settings as cards. Every change applies at once through `runner-core`; a refusal shows
//! the reason in the error bar and puts the control back.
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
use alloc::format;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::{Cell, OnceCell, RefCell};
use runner_core::blade_settings::{self, Kind, Setting};
use runner_core::settings::{self, BLADES, Look, Startup, Suite};
use runner_core::{join, link};
use windows::Foundation::{PropertyValue, TypedEventHandler, Uri};
use windows::Graphics::SizeInt32;
use windows_core::{HRESULT, HSTRING, IInspectable, IUnknown, IUnknown_Vtbl, Interface, Ref, Result};
use windows_sys::Win32::UI::HiDpi::GetDpiForWindow;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetKeyState;
use windows_sys::Win32::UI::WindowsAndMessaging::{AllowSetForegroundWindow, GetWindowThreadProcessId};

/// What the handlers share. Only the UI thread touches it: XAML calls handlers on the thread that
/// made the element. No `RefCell` borrow is held across a XAML call, which can run a handler.
struct Ui {
    window: Window,
    page: ContentControl,
    status: InfoBar,
    suite: Suite,
    suite_icon: String,
    /// General, then the blades: the pane's items, in page order.
    pages: RefCell<Vec<NavigationViewItem>>,
    /// Which blades start at sign-in, as last applied.
    startup: Cell<Startup>,
    /// Each blade's settings as it printed them when the window opened.
    settings: Vec<Vec<Setting>>,
    /// Each setting's value as the blade last accepted it.
    saved: RefCell<Vec<Vec<String>>>,
    /// Set while a handler puts a control back, so the change it raises isn't applied.
    updating: Cell<bool>,
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

/// `OnLaunched`: the control styles (what App.xaml did in C#), the settings, then the window.
/// A failed read (a blade that doesn't answer) ends the process with [`crate::EXIT_SETTINGS`],
/// as the C# app ended on the exception.
pub fn launched(app: &Application) -> Result<()> {
    let resources = ResourceDictionary::new()?;
    resources.MergedDictionaries()?.Append(&XamlControlsResources::new()?)?;
    app.SetResources(&resources)?;

    let dir = pk::env::exe_dir();
    let root = dir.rsplit_once('\\').map_or(dir.as_str(), |(parent, _)| parent).into();
    let suite = Suite::from_env(root);
    let startup = suite.read_startup().unwrap_or_else(|e| unreadable(&e));
    let settings: Vec<Vec<Setting>> =
        blade_settings::ask_all(&suite).answers().into_iter().map(|page| page.unwrap_or_else(|e| unreadable(&e))).collect();
    let saved = settings.iter().map(|page| page.iter().map(|s| s.value.clone()).collect()).collect();
    let state = Ui {
        window: Window::new()?,
        page: ContentControl::new()?,
        status: InfoBar::new()?,
        suite,
        suite_icon: join(&dir, "pocketknives.ico"),
        pages: RefCell::new(Vec::new()),
        startup: Cell::new(startup),
        settings,
        saved: RefCell::new(saved),
        updating: Cell::new(false),
    };
    if UI.0.set(state).is_err() {
        return Err(windows_core::Error::empty());
    }
    let ui = ui();
    ui.window.SetTitle(&HSTRING::from(settings::TITLE))?;
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
    ui.window.Activate()
}

fn unreadable(error: &str) -> ! {
    crate::report_line(&format!("The settings could not be read: {error}"));
    pk::exit(crate::EXIT_SETTINGS)
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
    for i in 0..=BLADES.len() {
        if i == 1 {
            let header = NavigationViewItemHeader::new()?;
            header.SetContent(&boxed(settings::BLADES_LABEL)?)?;
            items.Append(&header)?;
        }
        let item = NavigationViewItem::new()?;
        let icon = ImageIcon::new()?;
        icon.SetSource(&load_icon(&page_icon(i), 16)?)?;
        item.SetIcon(&icon)?;
        if i == 0 {
            item.SetContent(&boxed(settings::GENERAL_NAME)?)?;
        } else {
            item.SetContent(&blade_row(i - 1)?)?;
        }
        ui.pages.borrow_mut().push(item.clone());
        items.Append(&item)?;
    }

    let content = StackPanel::new()?;
    content.SetSpacing(12.0)?;
    content.SetMargin(uniform(24.0))?;
    let children = content.Children()?;
    children.Append(&ui.status)?;
    children.Append(&ui.page)?;
    let scroll = ScrollViewer::new()?;
    scroll.SetContent(&content)?;
    scroll.SetHorizontalContentAlignment(HorizontalAlignment::Stretch)?;
    navigation.SetContent(&scroll)?;
    navigation.SelectionChanged(&TypedEventHandler::new(|_, change: Ref<NavigationViewSelectionChangedEventArgs>| {
        let Some(change) = change.as_ref() else { return Ok(()) };
        let Ok(selected) = change.SelectedItem()?.cast::<IUnknown>() else { return Ok(()) };
        // COM identity: the same object answers the same IUnknown.
        let index = self::ui().pages.borrow().iter().position(|p| p.cast::<IUnknown>().is_ok_and(|p| p == selected));
        match index {
            Some(index) => show_page(index),
            None => Ok(()),
        }
    }))?;
    let first = ui.pages.borrow()[0].clone();
    navigation.SetSelectedItem(&first)?;
    Ok(navigation)
}

fn page_icon(index: usize) -> String {
    match index {
        0 => ui().suite_icon.clone(),
        i => ui().suite.blade_icon(&BLADES[i - 1]),
    }
}

fn blade_row(index: usize) -> Result<Grid> {
    let name = BLADES[index].name;
    let row = Grid::new()?;
    row.SetWidth(167.0)?;
    two_columns(&row)?;
    let label = text(name)?;
    label.SetVerticalAlignment(VerticalAlignment::Center)?;
    row.Children()?.Append(&label)?;
    let toggle = ToggleSwitch::new()?;
    toggle.SetIsOn(ui().startup.get()[index])?;
    toggle.SetOnContent(&boxed("")?)?;
    toggle.SetOffContent(&boxed("")?)?;
    toggle.SetMinWidth(0.0)?;
    AutomationProperties::SetName(&toggle, &HSTRING::from(format!("Start {name} at sign-in")))?;
    Grid::SetColumn(&toggle, 1)?;
    row.Children()?.Append(&toggle)?;
    toggle.Toggled(&RoutedEventHandler::new(move |sender, _| {
        let ui = ui();
        let Some(toggle) = sender.as_ref() else { return Ok(()) };
        if ui.updating.get() {
            return Ok(());
        }
        let toggle = toggle.cast::<ToggleSwitch>()?;
        let mut startup = ui.startup.get();
        let mut selected = startup;
        selected[index] = toggle.IsOn()?;
        // `startup` follows each row as it's applied, also when a later one fails.
        let applied = ui.suite.apply_startup(&mut startup, &selected);
        ui.startup.set(startup);
        match applied {
            Ok(()) => ui.status.SetIsOpen(false),
            Err(error) => {
                put_back(|| toggle.SetIsOn(startup[index]))?;
                show_error(&error)
            }
        }
    }))?;
    Ok(row)
}

fn show_page(index: usize) -> Result<()> {
    let ui = ui();
    let blade = index.checked_sub(1).map(|b| &BLADES[b]);
    let content = StackPanel::new()?;
    content.SetSpacing(20.0)?;
    let header = Grid::new()?;
    two_columns(&header)?;
    let texts = StackPanel::new()?;
    texts.SetSpacing(4.0)?;
    let title = text(blade.map_or(settings::GENERAL_NAME, |b| b.name))?;
    title.SetFontSize(28.0)?;
    title.SetFontWeight(FontWeights::SemiBold()?)?;
    texts.Children()?.Append(&title)?;
    let description = text(blade.map_or(settings::GENERAL_DESCRIPTION, |b| b.description))?;
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
    match blade {
        None => content.Children()?.Append(&look_card()?)?,
        Some(blade) if ui.settings[index - 1].is_empty() => {
            content.Children()?.Append(&text(&format!("{} has no settings to show.", blade.name))?)?
        }
        Some(_) => {
            for i in 0..ui.settings[index - 1].len() {
                content.Children()?.Append(&setting_card(index - 1, i)?)?;
            }
        }
    }
    ui.page.SetContent(&content)
}

fn setting_card(blade: usize, index: usize) -> Result<Border> {
    let setting = &ui().settings[blade][index];
    let control: FrameworkElement = match setting.kind {
        Kind::Toggle => toggle_control(blade, index)?.cast()?,
        Kind::Choice => choice_control(blade, index)?.cast()?,
        Kind::Hotkey => hotkey_control(blade, index)?.cast()?,
    };
    AutomationProperties::SetName(&control, &HSTRING::from(&setting.label))?;
    card(&setting.label, &control)
}

fn saved(blade: usize, index: usize) -> String {
    ui().saved.borrow()[blade][index].clone()
}

/// Changes apply at once, as in Windows Settings. False, with the blade's reason shown, when the
/// blade refuses.
fn change(blade: usize, index: usize, value: String) -> Result<bool> {
    let ui = ui();
    if value == saved(blade, index) {
        return Ok(true);
    }
    let key = ui.settings[blade][index].key.clone();
    match blade_settings::apply(&ui.suite, &BLADES[blade], &[(key, value.clone())]) {
        Ok(()) => {
            ui.saved.borrow_mut()[blade][index] = value;
            ui.status.SetIsOpen(false)?;
            Ok(true)
        }
        Err(error) => {
            show_error(&error)?;
            Ok(false)
        }
    }
}

/// Puts a control back without applying the change that raises.
fn put_back(revert: impl FnOnce() -> Result<()>) -> Result<()> {
    let ui = ui();
    ui.updating.set(true);
    let reverted = revert();
    ui.updating.set(false);
    reverted
}

fn toggle_control(blade: usize, index: usize) -> Result<ToggleSwitch> {
    let toggle = ToggleSwitch::new()?;
    toggle.SetIsOn(saved(blade, index) == "1")?;
    toggle.SetOnContent(&boxed("On")?)?;
    toggle.SetOffContent(&boxed("Off")?)?;
    toggle.SetMinWidth(0.0)?;
    toggle.Toggled(&RoutedEventHandler::new(move |sender, _: Ref<RoutedEventArgs>| {
        let Some(toggle) = sender.as_ref() else { return Ok(()) };
        let toggle = toggle.cast::<ToggleSwitch>()?;
        if ui().updating.get() || change(blade, index, String::from(if toggle.IsOn()? { "1" } else { "0" }))? {
            return Ok(());
        }
        put_back(|| toggle.SetIsOn(!toggle.IsOn()?))
    }))?;
    Ok(toggle)
}

/// The position of the saved value among the setting's choices; -1 (nothing selected) if none.
fn saved_choice(blade: usize, index: usize) -> i32 {
    let value = saved(blade, index);
    ui().settings[blade][index].choices.iter().position(|c| c.value == value).map_or(-1, |i| i as i32)
}

fn choice_control(blade: usize, index: usize) -> Result<ComboBox> {
    let combo = ComboBox::new()?;
    let items = combo.Items()?;
    for choice in &ui().settings[blade][index].choices {
        items.Append(&boxed(&choice.label)?)?;
    }
    combo.SetSelectedIndex(saved_choice(blade, index))?;
    combo.SetMinWidth(200.0)?;
    combo.SelectionChanged(&SelectionChangedEventHandler::new(move |sender, _| {
        let Some(combo) = sender.as_ref() else { return Ok(()) };
        let combo = combo.cast::<ComboBox>()?;
        let selected = combo.SelectedIndex()?;
        if ui().updating.get() || selected < 0 {
            return Ok(());
        }
        let value = ui().settings[blade][index].choices[selected as usize].value.clone();
        if change(blade, index, value)? {
            return Ok(());
        }
        put_back(|| combo.SetSelectedIndex(saved_choice(blade, index)))
    }))?;
    Ok(combo)
}

/// WinUI has no hotkey box: click, then press the keys. Esc cancels.
fn hotkey_control(blade: usize, index: usize) -> Result<Button> {
    let button = Button::new()?;
    button.SetContent(&boxed(&blade_settings::hotkey_name(&saved(blade, index)))?)?;
    button.SetMinWidth(200.0)?;
    let listening = Rc::new(Cell::new(false));
    let stop = move |button: &Button, listening: &Cell<bool>| -> Result<()> {
        listening.set(false);
        button.SetContent(&boxed(&blade_settings::hotkey_name(&saved(blade, index)))?)
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
        if !l.get().get() {
            return Ok(());
        }
        let button = button.cast::<Button>()?;
        key.SetHandled(true)?;
        let code = key.Key()?.0;
        if code == 0x1b {
            return stop(&button, l.get()); // Esc
        }
        if matches!(code, 0x10 | 0x11 | 0x12 | 0xa0..=0xa5 | 0x5b | 0x5c) {
            return Ok(()); // a modifier key alone
        }
        let modifiers = if down(0x11) { blade_settings::CTRL } else { 0 }
            | if down(0x12) { blade_settings::ALT } else { 0 }
            | if down(0x10) { blade_settings::SHIFT } else { 0 }
            | if down(0x5b) || down(0x5c) { blade_settings::WIN } else { 0 };
        if modifiers & !blade_settings::SHIFT == 0 {
            return button.SetContent(&boxed("Add Ctrl, Alt or Win")?);
        }
        change(blade, index, blade_settings::join_hotkey(modifiers, code as u32))?;
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
    for name in settings::LOOK_NAMES {
        items.Append(&boxed(name)?)?;
    }
    choices.SetSelectedIndex(Look::Windows11.index() as i32)?;
    AutomationProperties::SetName(&choices, &HSTRING::from(settings::LOOK_LABEL))?;
    choices.SelectionChanged(&SelectionChangedEventHandler::new(|sender, _| {
        let Some(choices) = sender.as_ref() else { return Ok(()) };
        let choices = choices.cast::<ComboBox>()?;
        let look = match choices.SelectedIndex()? {
            0 => Look::Xp,
            1 => Look::Windows10,
            _ => return Ok(()),
        };
        if let Err(error) = switch_look(look) {
            // Selecting Windows 11 raises this handler again, which ignores it.
            choices.SetSelectedIndex(Look::Windows11.index() as i32)?;
            show_error(&error)?;
        }
        Ok(())
    }))?;
    card(settings::LOOK_LABEL, &choices.cast()?)
}

/// Saves the look and asks the runner to reopen Settings in it, then closes this window.
fn switch_look(look: Look) -> runner_core::Result<()> {
    let runner = crate::runner_window() as windows_sys::Win32::Foundation::HWND;
    let mut pid = 0;
    if unsafe { GetWindowThreadProcessId(runner, &mut pid) } == 0 {
        return Err(String::from("The PocketKnives runner has closed."));
    }
    ui().suite.save_look(look)?;
    unsafe { AllowSetForegroundWindow(pid) };
    if !link::post(runner, link::REOPEN_MESSAGE) {
        return Err(String::from("The PocketKnives runner could not reopen Settings."));
    }
    ui().window.Close().map_err(|e| e.message().to_string())
}

fn show_error(message: &str) -> Result<()> {
    let ui = ui();
    ui.status.SetMessage(&HSTRING::from(message))?;
    ui.status.SetIsOpen(true)
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
