using System.Runtime.InteropServices;
using Microsoft.UI.Dispatching;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Media.Imaging;
using Windows.Graphics;

namespace PocketKnives;

internal static class Program
{
    internal static nint RunnerWindow { get; private set; }

    [STAThread]
    private static int Main(string[] args)
    {
        if (args.Length != 1 || !nint.TryParse(args[0], out nint runner) || runner == 0)
            return 2;
        RunnerWindow = runner;
        WinRT.ComWrappersSupport.InitializeComWrappers();
        Application.Start(_ =>
        {
            SynchronizationContext.SetSynchronizationContext(
                new DispatcherQueueSynchronizationContext(DispatcherQueue.GetForCurrentThread()));
            new App();
        });
        return 0;
    }
}

public partial class App : Application
{
    private readonly string root = Path.GetFullPath(Path.Combine(Settings.Root, ".."));
    private readonly string suiteIcon = Path.Combine(Settings.Root, "pocketknives.ico");
    private readonly List<NavigationViewItem> pages = [];
    private ContentControl page = null!;
    private InfoBar status = null!;
    private Window window = null!;
    private bool[] startup = [];
    private BladeSettings.Setting[][] bladeSettings = [];
    private string[][] saved = [];
    private bool updating;

    public App() => InitializeComponent();

    protected override void OnLaunched(LaunchActivatedEventArgs args)
    {
        startup = Settings.ReadStartup(root);
        bladeSettings = [.. Settings.Blades.Select(blade => BladeSettings.Read(root, blade))];
        saved = [.. bladeSettings.Select(page => page.Select(setting => setting.Value).ToArray())];
        window = new Window { Title = Settings.Title, SystemBackdrop = new MicaBackdrop() };
        page = new ContentControl { HorizontalContentAlignment = HorizontalAlignment.Stretch };
        status = new InfoBar { Severity = InfoBarSeverity.Error, IsClosable = true };
        window.AppWindow.SetIcon(suiteIcon);
        window.Content = Build();
        // AppWindow.Resize uses physical pixels.
        nint hwnd = WinRT.Interop.WindowNative.GetWindowHandle(window);
        double scale = GetDpiForWindow(hwnd) / 96.0;
        window.AppWindow.Resize(new SizeInt32((int)(830 * scale), (int)(500 * scale)));
        window.Activate();
    }

    private NavigationView Build()
    {
        var navigation = new NavigationView
        {
            PaneDisplayMode = NavigationViewPaneDisplayMode.Left,
            IsPaneOpen = true,
            IsPaneToggleButtonVisible = false,
            IsBackButtonVisible = NavigationViewBackButtonVisible.Collapsed,
            IsSettingsVisible = false,
            OpenPaneLength = 252,
            HorizontalContentAlignment = HorizontalAlignment.Stretch,
            PaneFooter = new TextBlock { Text = "On: starts at sign-in", Margin = new Thickness(16, 8, 8, 12) },
        };
        for (int i = 0; i <= Settings.Blades.Length; i++)
        {
            if (i == 1)
                navigation.MenuItems.Add(new NavigationViewItemHeader { Content = Settings.BladesLabel });
            Settings.Blade? blade = i == 0 ? null : Settings.Blades[i - 1];
            var item = new NavigationViewItem
            {
                Icon = new ImageIcon { Source = LoadIcon(blade is null ? suiteIcon : Settings.BladeIcon(root, blade), 16) },
                Content = blade is null ? Settings.GeneralName : BladeRow(i - 1, blade.Name),
            };
            pages.Add(item);
            navigation.MenuItems.Add(item);
        }

        var content = new StackPanel { Spacing = 12, Margin = new Thickness(24) };
        content.Children.Add(status);
        content.Children.Add(page);
        navigation.Content = new ScrollViewer
        {
            Content = content,
            HorizontalContentAlignment = HorizontalAlignment.Stretch,
        };
        navigation.SelectionChanged += (_, change) =>
        {
            if (change.SelectedItem is NavigationViewItem selected)
                ShowPage(pages.IndexOf(selected));
        };
        navigation.SelectedItem = pages[0];
        return navigation;
    }

    private Grid BladeRow(int index, string name)
    {
        var row = new Grid { Width = 167 };
        row.ColumnDefinitions.Add(new ColumnDefinition());
        row.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        row.Children.Add(new TextBlock { Text = name, VerticalAlignment = VerticalAlignment.Center });
        var toggle = new ToggleSwitch
        {
            IsOn = startup[index],
            OnContent = "",
            OffContent = "",
            MinWidth = 0,
        };
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(toggle, $"Start {name} at sign-in");
        Grid.SetColumn(toggle, 1);
        row.Children.Add(toggle);
        toggle.Toggled += (_, _) =>
        {
            if (updating)
                return;
            bool[] selected = (bool[])startup.Clone();
            selected[index] = toggle.IsOn;
            try
            {
                Settings.ApplyStartup(root, startup, selected);
                status.IsOpen = false;
            }
            catch (Exception error) when (Settings.IsSettingsError(error))
            {
                updating = true;
                toggle.IsOn = startup[index];
                updating = false;
                ShowError(error.Message);
            }
        };
        return row;
    }

    private void ShowPage(int index)
    {
        Settings.Blade? blade = index == 0 ? null : Settings.Blades[index - 1];
        var content = new StackPanel { Spacing = 20 };
        var header = new Grid();
        header.ColumnDefinitions.Add(new ColumnDefinition());
        header.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        var texts = new StackPanel { Spacing = 4 };
        texts.Children.Add(new TextBlock
        {
            Text = blade?.Name ?? Settings.GeneralName, FontSize = 28, FontWeight = Microsoft.UI.Text.FontWeights.SemiBold,
        });
        texts.Children.Add(new TextBlock
        {
            Text = blade?.Description ?? Settings.GeneralDescription,
            TextWrapping = TextWrapping.Wrap,
        });
        header.Children.Add(texts);
        var icon = new Image
        {
            Source = LoadIcon(blade is null ? suiteIcon : Settings.BladeIcon(root, blade), 48),
            Width = 48,
            Height = 48,
            Margin = new Thickness(12, 0, 0, 0),
        };
        Grid.SetColumn(icon, 1);
        header.Children.Add(icon);
        content.Children.Add(header);
        if (blade is null)
            content.Children.Add(LookCard());
        else if (bladeSettings[index - 1].Length == 0)
            content.Children.Add(new TextBlock { Text = $"{blade.Name} has no settings to show." });
        else
            for (int i = 0; i < bladeSettings[index - 1].Length; i++)
                content.Children.Add(SettingCard(index - 1, i));
        page.Content = content;
    }

    private Border SettingCard(int blade, int index)
    {
        BladeSettings.Setting setting = bladeSettings[blade][index];
        FrameworkElement control = setting.Kind switch
        {
            BladeSettings.Kind.Toggle => ToggleControl(blade, index),
            BladeSettings.Kind.Choice => ChoiceControl(blade, index),
            _ => HotkeyControl(blade, index),
        };
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(control, setting.Label);
        return Card(setting.Label, control);
    }

    // Changes apply at once, as in Windows Settings. Returns false, with the blade's reason shown, when refused.
    private bool Change(int blade, int index, string value)
    {
        if (value == saved[blade][index])
            return true;
        try
        {
            BladeSettings.Apply(root, Settings.Blades[blade], [(bladeSettings[blade][index].Key, value)]);
            saved[blade][index] = value;
            status.IsOpen = false;
            return true;
        }
        catch (Exception error) when (Settings.IsSettingsError(error))
        {
            ShowError(error.Message);
            return false;
        }
    }

    private ToggleSwitch ToggleControl(int blade, int index)
    {
        var toggle = new ToggleSwitch { IsOn = saved[blade][index] == "1", OnContent = "On", OffContent = "Off", MinWidth = 0 };
        toggle.Toggled += (_, _) =>
        {
            if (updating || Change(blade, index, toggle.IsOn ? "1" : "0"))
                return;
            updating = true;
            toggle.IsOn = !toggle.IsOn;
            updating = false;
        };
        return toggle;
    }

    private ComboBox ChoiceControl(int blade, int index)
    {
        BladeSettings.Choice[] choices = bladeSettings[blade][index].Choices;
        var combo = new ComboBox
        {
            ItemsSource = choices.Select(choice => choice.Label).ToList(),
            SelectedIndex = Array.FindIndex(choices, choice => choice.Value == saved[blade][index]),
            MinWidth = 200,
        };
        combo.SelectionChanged += (_, _) =>
        {
            if (updating || combo.SelectedIndex < 0 || Change(blade, index, choices[combo.SelectedIndex].Value))
                return;
            updating = true;
            combo.SelectedIndex = Array.FindIndex(choices, choice => choice.Value == saved[blade][index]);
            updating = false;
        };
        return combo;
    }

    // WinUI has no hotkey box: click, then press the keys. Esc cancels.
    private Button HotkeyControl(int blade, int index)
    {
        var button = new Button { Content = BladeSettings.HotkeyName(saved[blade][index]), MinWidth = 200 };
        bool listening = false;
        void Stop()
        {
            listening = false;
            button.Content = BladeSettings.HotkeyName(saved[blade][index]);
        }
        button.Click += (_, _) =>
        {
            if (listening)
                return;
            listening = true;
            button.Content = "Press the new keys";
        };
        button.LostFocus += (_, _) => { if (listening) Stop(); };
        button.AddHandler(UIElement.KeyDownEvent, new Microsoft.UI.Xaml.Input.KeyEventHandler((_, key) =>
        {
            if (!listening)
                return;
            key.Handled = true;
            int code = (int)key.Key;
            if (code == 0x1b) // Esc
            {
                Stop();
                return;
            }
            if (code is 0x10 or 0x11 or 0x12 or >= 0xa0 and <= 0xa5 or 0x5b or 0x5c) // modifier keys alone
                return;
            int modifiers = (Down(0x11) ? BladeSettings.Ctrl : 0) | (Down(0x12) ? BladeSettings.Alt : 0) |
                            (Down(0x10) ? BladeSettings.Shift : 0) | (Down(0x5b) || Down(0x5c) ? BladeSettings.Win : 0);
            if ((modifiers & ~BladeSettings.Shift) == 0)
            {
                button.Content = "Add Ctrl, Alt or Win";
                return;
            }
            Change(blade, index, BladeSettings.JoinHotkey(modifiers, code));
            Stop();
        }), handledEventsToo: true);
        return button;
    }

    private static bool Down(int key) => (GetKeyState(key) & 0x8000) != 0;

    private static Border Card(string label, FrameworkElement control)
    {
        var row = new Grid { Padding = new Thickness(16), ColumnSpacing = 12 };
        row.ColumnDefinitions.Add(new ColumnDefinition());
        row.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        row.Children.Add(new TextBlock { Text = label, VerticalAlignment = VerticalAlignment.Center, TextWrapping = TextWrapping.Wrap });
        control.VerticalAlignment = VerticalAlignment.Center;
        Grid.SetColumn(control, 1);
        row.Children.Add(control);
        return new Border
        {
            Child = row,
            CornerRadius = new CornerRadius(6),
            Background = new SolidColorBrush(Microsoft.UI.Colors.Gray) { Opacity = 0.12 },
        };
    }

    private Border LookCard()
    {
        var choices = new ComboBox { ItemsSource = Settings.LookNames, SelectedIndex = (int)Settings.Look.Windows11 };
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(choices, Settings.LookLabel);
        choices.SelectionChanged += (_, _) =>
        {
            if (choices.SelectedIndex is not (0 or 1))
                return;
            try
            {
                nint runner = Program.RunnerWindow;
                if (GetWindowThreadProcessId(runner, out int pid) == 0)
                    throw new IOException("The PocketKnives runner has closed.");
                Settings.SaveLook(root, (Settings.Look)choices.SelectedIndex);
                AllowSetForegroundWindow(pid);
                if (!RunnerLink.Post(runner, RunnerLink.ReopenMessage))
                    throw new IOException("The PocketKnives runner could not reopen Settings.");
                window.Close();
            }
            catch (Exception error) when (Settings.IsSettingsError(error))
            {
                choices.SelectedIndex = (int)Settings.Look.Windows11;
                ShowError(error.Message);
            }
        };
        return Card(Settings.LookLabel, choices);
    }

    private void ShowError(string message)
    {
        status.Message = message;
        status.IsOpen = true;
    }

    private static BitmapImage LoadIcon(string path, int size) =>
        new(new Uri(path)) { DecodePixelWidth = size, DecodePixelHeight = size };

    [DllImport("user32.dll")] private static extern uint GetDpiForWindow(nint window);
    [DllImport("user32.dll")] private static extern uint GetWindowThreadProcessId(nint window, out int processId);
    [DllImport("user32.dll")] private static extern int AllowSetForegroundWindow(int processId);
    [DllImport("user32.dll")] private static extern short GetKeyState(int key);
}
