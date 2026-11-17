using System.ComponentModel;
using System.Diagnostics;
using System.Runtime.InteropServices;
using Microsoft.Win32;

namespace PocketKnives;

internal static class Settings
{
    internal const string Title = "PocketKnives Settings";
    internal const string GeneralName = "General";
    internal const string GeneralDescription = "Settings for the whole suite.";
    internal const string BladesLabel = "Blades";
    internal const string LookLabel = "Look";
    internal const string PreviewVariable = "POCKETKNIVES_SETTINGS_PREVIEW";
    private const string SuiteKey = @"Software\PocketKnives";
    private const string RunKey = @"Software\Microsoft\Windows\CurrentVersion\Run";

    internal enum Look { Xp, Windows10, Windows11 }
    internal static readonly string[] LookNames = ["XP", "Windows 10", "Windows 11"];
    private static readonly string[] LookValues = ["xp", "win10", "win11"];

    // The commands follow docs/specs/blade-settings.md.
    internal sealed record Blade(string Name, string Description, string Icon, string Exe, string StopCommand,
        string SettingsCommand, string SetCommand);
    internal static readonly Blade[] Blades =
    [
        new("SnipSnap", "Copies a rectangle, a window or the screen to the clipboard.", "snipsnap.ico",
            "snipsnap.exe", "quit", "settings", "set"),
        new("QuickTooth", "Connects or disconnects your AirPods with one click.", "quicktooth.ico",
            "QuickTooth.exe", "--exit", "--settings", "--set"),
    ];

    internal static bool IsPreview => Environment.GetEnvironmentVariable(PreviewVariable) == "1";
    internal static string Root => AppContext.BaseDirectory.TrimEnd(Path.DirectorySeparatorChar);
    internal static string WinUiPath => Path.Combine(Root, "winui", "PocketKnives.Settings.exe");
    internal static string BladeIcon(string root, Blade blade) => Path.Combine(root, "icons", blade.Icon);
    private static string StartupShortcut => Path.Combine(
        Environment.GetFolderPath(Environment.SpecialFolder.Startup), Blades[0].Name + ".lnk");
    private static string PreviewPath(string root) => Path.Combine(root, "preview-settings.txt");

    internal static Look ReadLook(string root)
    {
        if (IsPreview)
            return ParseLook(ReadPreview(root).FirstOrDefault());

        using var key = Registry.CurrentUser.OpenSubKey(SuiteKey);
        return ParseLook(key?.GetValue("Look") as string);
    }

    private static Look ParseLook(string? value)
    {
        int index = Array.IndexOf(LookValues, value);
        return index < 0 ? Look.Xp : (Look)index;
    }

    internal static void SaveLook(string root, Look look)
    {
        string value = LookValues[(int)look];
        if (IsPreview)
        {
            string[] preview = ReadPreview(root);
            preview[0] = value;
            File.WriteAllLines(PreviewPath(root), preview);
            return;
        }

        using var key = Registry.CurrentUser.CreateSubKey(SuiteKey);
        key.SetValue("Look", value);
    }

    internal static bool[] ReadStartup(string root)
    {
        if (IsPreview)
        {
            string[] preview = ReadPreview(root);
            return [preview[1] == "on", preview[2] == "on"];
        }

        using var run = Registry.CurrentUser.OpenSubKey(RunKey);
        return [File.Exists(StartupShortcut), !string.IsNullOrWhiteSpace(run?.GetValue(Blades[1].Name) as string)];
    }

    private static string[] ReadPreview(string root)
    {
        var path = PreviewPath(root);
        string[] values = File.Exists(path) ? File.ReadAllLines(path) : [];
        return values.Length == 3 ? values : [LookValues[0], "on", "on"];
    }

    // Apply edits only. An unchanged row must not overwrite a change made from QuickTooth's tray menu.
    internal static void ApplyStartup(string root, bool[] original, bool[] selected)
    {
        if (original.AsSpan().SequenceEqual(selected))
            return;

        if (IsPreview)
        {
            string[] preview = ReadPreview(root);
            for (int i = 0; i < selected.Length; i++)
                if (original[i] != selected[i])
                    preview[i + 1] = selected[i] ? "on" : "off";
            File.WriteAllLines(PreviewPath(root), preview);
            selected.CopyTo(original, 0);
            return;
        }

        if (original[0] != selected[0])
        {
            if (selected[0])
                CreateSnipSnapShortcut(root);
            else
                File.Delete(StartupShortcut);
            original[0] = selected[0];
        }

        if (original[1] != selected[1])
        {
            string exe = Path.Combine(root, Blades[1].Exe);
            using var child = Process.Start(new ProcessStartInfo(exe, selected[1] ? "--autostart on" : "--autostart off")
            {
                UseShellExecute = false,
                CreateNoWindow = true,
            });
            if (child is null || !child.WaitForExit(5000) || child.ExitCode != 0)
                throw new IOException("QuickTooth could not update its sign-in setting.");
            original[1] = selected[1];
        }
    }

    private static void CreateSnipSnapShortcut(string root)
    {
        string target = Path.Combine(root, Blades[0].Exe);
        if (!File.Exists(target))
            throw new FileNotFoundException("SnipSnap is missing from the PocketKnives folder.", target);
        ShellLink.Save(target, StartupShortcut);
    }

    // Each blade's own stop command finds its running copy by window or mutex, not by path, so a
    // preview never runs them: they would stop the user's installed blades.
    internal static void StopBlades(string root)
    {
        if (IsPreview)
            return;
        foreach (Blade blade in Blades)
        {
            string exe = Path.Combine(root, blade.Exe);
            if (!File.Exists(exe))
                continue;
            try
            {
                using var stop = Process.Start(new ProcessStartInfo(exe, blade.StopCommand)
                {
                    UseShellExecute = false,
                    CreateNoWindow = true,
                });
            }
            catch (Win32Exception)
            {
                // A blade that can't be asked to stop keeps running; Exit still closes the runner.
            }
        }
    }

    internal static bool IsSettingsError(Exception error) =>
        error is IOException or UnauthorizedAccessException or Win32Exception or COMException or System.Security.SecurityException;
}

// The two methods used from IShellLinkW and IPersistFile. Using the COM ABI keeps this AOT-compatible.
internal static unsafe class ShellLink
{
    [DllImport("ole32.dll")]
    private static extern int CoInitializeEx(nint reserved, uint apartment);
    [DllImport("ole32.dll")]
    private static extern void CoUninitialize();
    [DllImport("ole32.dll")]
    private static extern int CoCreateInstance(Guid* clsid, nint outer, uint context, Guid* iid, nint* result);

    internal static void Save(string target, string path)
    {
        Guid clsid = new("00021401-0000-0000-C000-000000000046");
        Guid shellLinkW = new("000214F9-0000-0000-C000-000000000046");
        Guid persistFile = new("0000010B-0000-0000-C000-000000000046");
        nint link = 0, file = 0;
        const int changedMode = unchecked((int)0x80010106); // RPC_E_CHANGED_MODE
        int apartment = CoInitializeEx(0, 2); // COINIT_APARTMENTTHREADED
        // COM can already be initialized in another apartment model. Keep that apartment.
        if (apartment != changedMode)
            Marshal.ThrowExceptionForHR(apartment);
        try
        {
            Marshal.ThrowExceptionForHR(CoCreateInstance(&clsid, 0, 1, &shellLinkW, &link));
            nint* methods = *(nint**)link;
            var setPath = (delegate* unmanaged[Stdcall]<nint, char*, int>)methods[20];
            fixed (char* value = target)
                Marshal.ThrowExceptionForHR(setPath(link, value));

            var queryInterface = (delegate* unmanaged[Stdcall]<nint, Guid*, nint*, int>)methods[0];
            Marshal.ThrowExceptionForHR(queryInterface(link, &persistFile, &file));
            var save = (delegate* unmanaged[Stdcall]<nint, char*, int, int>)(*(nint**)file)[6];
            fixed (char* value = path)
                Marshal.ThrowExceptionForHR(save(file, value, 1));
        }
        finally
        {
            Release(file);
            Release(link);
            if (apartment >= 0)
                CoUninitialize();
        }
    }

    private static void Release(nint instance)
    {
        if (instance != 0)
            ((delegate* unmanaged[Stdcall]<nint, uint>)(*(nint**)instance)[2])(instance);
    }
}
