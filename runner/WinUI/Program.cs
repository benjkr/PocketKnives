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
    private bool updating;

    public App() => InitializeComponent();

    protected override void OnLaunched(LaunchActivatedEventArgs args)
    {
        startup = Settings.ReadStartup(root);
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
        page.Content = content;
    }

    private Border LookCard()
    {
        var choices = new ComboBox { ItemsSource = Settings.LookNames, SelectedIndex = (int)Settings.Look.Windows11 };
        Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(choices, Settings.LookLabel);
        var row = new Grid { Padding = new Thickness(16) };
        row.ColumnDefinitions.Add(new ColumnDefinition());
        row.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
        row.Children.Add(new TextBlock { Text = Settings.LookLabel, VerticalAlignment = VerticalAlignment.Center });
        Grid.SetColumn(choices, 1);
        row.Children.Add(choices);
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
        return new Border
        {
            Child = row,
            CornerRadius = new CornerRadius(6),
            Background = new SolidColorBrush(Microsoft.UI.Colors.Gray) { Opacity = 0.12 },
        };
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
}
