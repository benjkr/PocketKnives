using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Security.Cryptography;
using System.Text;
using static PocketKnives.RunnerLink;
using static PocketKnives.Windows;

namespace PocketKnives;

internal sealed unsafe class Program
{
    private const string ClassName = "PocketKnives.Runner";
    private const uint ChildExited = 0x8002;
    private const nuint TrayRetry = 1;
    private const nuint StopRetry = 2;
    private static readonly Guid TrayIdentity = new("c86c4d67-f167-4393-a49a-935298892f12");
    private static Program? current;

    private readonly uint taskbarCreated = RegisterWindowMessageW("TaskbarCreated");
    private readonly uint open = RegisterWindowMessageW(OpenMessage);
    private readonly uint reopen = RegisterWindowMessageW(ReopenMessage);
    private readonly uint exit = RegisterWindowMessageW(ExitMessage);
    private nint window;
    private TrayIcon? tray;
    private Process? winUi;
    private bool reopenAfterExit, stopping;

    [STAThread]
    private static int Main(string[] args)
    {
        bool preview = args.Length > 0 && args[0] == "--preview";
        if (preview)
            args = args[1..];
        Environment.SetEnvironmentVariable(Settings.PreviewVariable, preview ? "1" : null);
        string command = args.Length == 0 ? "" : args[0];
        if (args.Length > 1 || command is not ("" or "settings" or "--exit"))
            return 2;

        Guid identity = preview ? PreviewIdentity() : TrayIdentity;
        string instance = preview ? $"{ClassName}.Preview.{identity:N}" : ClassName;
        using var mutex = new Mutex(true, $@"Local\{instance}", out bool first);
        using var ready = new EventWaitHandle(false, EventResetMode.ManualReset, $@"Local\{instance}.Ready");
        if (!first)
        {
            if (command == "")
                return 0;
            if (!ready.WaitOne(5000))
                return 2;
            nint host = FindWindowW(instance, null);
            if (command == "settings" && GetWindowThreadProcessId(host, out int pid) != 0)
                AllowSetForegroundWindow(pid);
            return Post(host, command == "--exit" ? ExitMessage : OpenMessage) ? 0 : 2;
        }

        try
        {
            if (command == "--exit")
                return 0;
            current = new Program();
            return current.Run(instance, identity, ready, command == "settings");
        }
        catch (Exception error) when (Settings.IsSettingsError(error) || error is InvalidOperationException)
        {
            Error(0, error.Message);
            return 2;
        }
        finally
        {
            ready.Reset();
            mutex.ReleaseMutex();
        }
    }

    private static Guid PreviewIdentity()
    {
        byte[] folder = Encoding.UTF8.GetBytes(Settings.Root.ToUpperInvariant());
        return new Guid(SHA256.HashData(folder).AsSpan(0, 16));
    }

    private int Run(string instance, Guid identity, EventWaitHandle ready, bool showSettings)
    {
        Register(instance, &WindowProcedure);
        NativeSettings.Initialize();
        // A hidden top-level window receives Explorer's TaskbarCreated broadcast.
        window = CreateWindowExW(0, instance, "", Popup, 0, 0, 0, 0, 0, 0, Module, 0);
        if (window == 0)
            throw new InvalidOperationException("Could not start the PocketKnives tray.");
        using var icon = new TrayIcon(window, identity);
        tray = icon;
        RefreshTray();
        ready.Set();
        if (showSettings)
            OpenSettings();

        int result;
        while ((result = GetMessageW(out Message message, 0, 0, 0)) > 0)
        {
            nint settings = NativeSettings.Handle;
            if (settings != 0 && IsDialogMessageW(settings, ref message) != 0)
                continue;
            TranslateMessage(ref message);
            DispatchMessageW(ref message);
        }
        if (result < 0)
            return 2;
        return 0;
    }

    private void RefreshTray()
    {
        if (tray?.Register() == true)
            KillTimer(window, TrayRetry);
        else
            SetTimer(window, TrayRetry, 2000, 0); // Explorer may not be ready yet at sign-in.
    }

    private void OpenSettings(Settings.Look? overrideLook = null)
    {
        if (stopping)
            return;

        try
        {
            if (NativeSettings.Handle != 0)
            {
                Activate(NativeSettings.Handle);
                return;
            }
            if (winUi is not null)
            {
                if (winUi.HasExited)
                    reopenAfterExit = true;
                else
                {
                    winUi.Refresh();
                    nint settings = winUi.MainWindowHandle;
                    if (settings != 0)
                        Activate(settings);
                    else
                        AllowSetForegroundWindow(winUi.Id);
                }
                return;
            }

            Settings.Look look = overrideLook ?? Settings.ReadLook(Settings.Root);
            if (look != Settings.Look.Windows11)
                NativeSettings.Open(look);
            else
                StartWinUi();
        }
        catch (Exception error) when (Settings.IsSettingsError(error) || error is InvalidOperationException)
        {
            Error(window, error.Message);
        }
    }

    private void StartWinUi()
    {
        if (!File.Exists(Settings.WinUiPath))
        {
            Error(window, "The Windows 11 settings window is missing. Opening the Windows 10 look.");
            NativeSettings.Open(Settings.Look.Windows10);
            return;
        }
        ProcessStartInfo start = new(Settings.WinUiPath)
        {
            UseShellExecute = false,
            WorkingDirectory = Path.GetDirectoryName(Settings.WinUiPath),
        };
        start.ArgumentList.Add(window.ToString());
        Process child = Process.Start(start) ?? throw new InvalidOperationException("Could not open the Windows 11 settings window.");
        winUi = child;
        int processId = child.Id;
        AllowSetForegroundWindow(processId);
        child.Exited += (_, _) => PostMessageW(window, ChildExited, (nuint)processId, 0);
        child.EnableRaisingEvents = true;
    }

    private void OnChildExited(nuint processId)
    {
        if (winUi is null || (nuint)winUi.Id != processId)
            return;
        bool failed = winUi.ExitCode != 0;
        winUi.Dispose();
        winUi = null;
        bool requested = reopenAfterExit;
        reopenAfterExit = false;
        if (stopping)
            DestroyWindow(window);
        else if (requested)
            OpenSettings();
        else if (failed)
        {
            Error(window, "The Windows 11 settings window closed unexpectedly. Opening the Windows 10 look.");
            OpenSettings(Settings.Look.Windows10);
        }
    }

    private void ShowMenu(int x, int y)
    {
        const nuint settingsItem = 1, exitItem = 2;
        nint menu = CreatePopupMenu();
        if (menu == 0)
            return;
        AppendMenuW(menu, 0, settingsItem, "&Settings");
        AppendMenuW(menu, 0, exitItem, "E&xit");
        SetMenuDefaultItem(menu, (uint)settingsItem, 0);
        // Without the foreground, the menu doesn't close when the user clicks elsewhere.
        SetForegroundWindow(window);
        int choice = TrackPopupMenuEx(menu, 0x0100 | 0x0002, x, y, window, 0); // TPM_RETURNCMD, TPM_RIGHTBUTTON
        DestroyMenu(menu);
        if (choice == (int)settingsItem)
            OpenSettings();
        else if (choice == (int)exitItem)
            Stop();
    }

    private void Stop()
    {
        stopping = true;
        tray?.Dispose();
        KillTimer(window, TrayRetry);
        NativeSettings.CloseWindow();
        if (winUi is null)
            DestroyWindow(window);
        else if (!winUi.HasExited)
        {
            winUi.Refresh();
            if (winUi.CloseMainWindow())
                KillTimer(window, StopRetry);
            else
                SetTimer(window, StopRetry, 200, 0); // Wait for WinUI to create its HWND before requesting close.
        }
    }

    [UnmanagedCallersOnly]
    private static nint WindowProcedure(nint handle, uint message, nuint wParam, nint lParam)
    {
        Program? app = current;
        if (app is null || handle != app.window)
            return DefWindowProcW(handle, message, wParam, lParam);

        if (message == Close || message == app.exit || (message == Windows.Timer && wParam == StopRetry))
        {
            app.Stop();
            return 0;
        }
        if (message == app.open)
        {
            app.OpenSettings();
            return 0;
        }
        if (message == app.reopen)
        {
            app.reopenAfterExit = true;
            return 0;
        }
        if (message == ChildExited)
        {
            app.OnChildExited(wParam);
            return 0;
        }
        if (!app.stopping && (message == app.taskbarCreated || message == ThemeChanged ||
                              (message == Windows.Timer && wParam == TrayRetry)))
        {
            app.RefreshTray();
            return 0;
        }
        if (message == TrayIcon.Callback)
        {
            // NOTIFYICON_VERSION_4 packs the event in LOWORD(lParam).
            if ((lParam & 0xffff) is 0x0400 or 0x0401) // NIN_SELECT, NIN_KEYSELECT
                app.OpenSettings();
            else if ((lParam & 0xffff) == 0x007b) // WM_CONTEXTMENU: right-click, Shift+F10 or the menu key
                app.ShowMenu((short)(wParam & 0xffff), (short)((wParam >> 16) & 0xffff));
            return 0;
        }
        if (message == Destroy)
        {
            PostQuitMessage(0);
            return 0;
        }
        return DefWindowProcW(handle, message, wParam, lParam);
    }

    internal static void ReopenNativeSettings()
    {
        NativeSettings.CloseWindow();
        if (current is { } app)
            Post(app.window, OpenMessage);
    }
}
