using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Text;

namespace PocketKnives;

// A blade's settings as it prints them (docs/specs/blade-settings.md). Both settings windows use this.
internal static class BladeSettings
{
    internal enum Kind { Hotkey, Toggle, Choice }

    internal sealed record Choice(string Value, string Label);

    internal sealed record Setting(string Key, Kind Kind, string Value, string Label, Choice[] Choices);

    private const int TimeoutMs = 5000;

    internal static Setting[] Read(string root, Settings.Blade blade)
    {
        string exe = Path.Combine(root, blade.Exe);
        if (!File.Exists(exe))
            return [];
        (int code, string output) = Run(exe, [blade.SettingsCommand]);
        return code == 0 ? Parse(output) : [];
    }

    internal static Setting[] Parse(string output)
    {
        var settings = new List<Setting>();
        foreach (string line in output.Split('\n'))
        {
            string[] fields = line.TrimEnd('\r').Split('\t');
            if (fields.Length < 4 || fields[0].Length == 0)
                continue;
            Kind? kind = fields[1] switch
            {
                "hotkey" => Kind.Hotkey,
                "toggle" => Kind.Toggle,
                "choice" => Kind.Choice,
                _ => null,
            };
            if (kind is null)
                continue;
            var choices = new List<Choice>();
            foreach (string choice in fields.AsSpan(4))
            {
                int split = choice.IndexOf('=');
                if (split > 0)
                    choices.Add(new Choice(choice[..split], choice[(split + 1)..]));
            }
            settings.Add(new Setting(fields[0], kind.Value, fields[2], fields[3], [.. choices]));
        }
        return [.. settings];
    }

    // Throws IOException with the blade's own words when it refuses the change.
    internal static void Apply(string root, Settings.Blade blade, IEnumerable<(string Key, string Value)> changes)
    {
        var args = new List<string> { blade.SetCommand };
        foreach ((string key, string value) in changes)
            args.Add($"{key}={value}");
        if (args.Count == 1)
            return;
        string exe = Path.Combine(root, blade.Exe);
        if (!File.Exists(exe))
            throw new FileNotFoundException($"{blade.Name} is missing from the PocketKnives folder.", exe);
        (int code, string output) = Run(exe, args);
        if (code != 0)
        {
            string reason = output.Split('\n', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
                .FirstOrDefault() ?? $"{blade.Name} could not change its settings.";
            throw new IOException(reason);
        }
    }

    private static (int Code, string Output) Run(string exe, IEnumerable<string> args)
    {
        var start = new ProcessStartInfo(exe)
        {
            UseShellExecute = false,
            CreateNoWindow = true,
            RedirectStandardOutput = true,
            StandardOutputEncoding = new UTF8Encoding(false),
        };
        foreach (string arg in args)
            start.ArgumentList.Add(arg);
        using Process process = Process.Start(start)
            ?? throw new IOException($"Could not start {Path.GetFileName(exe)}.");
        Task<string> output = process.StandardOutput.ReadToEndAsync();
        if (!process.WaitForExit(TimeoutMs))
        {
            process.Kill();
            throw new IOException($"{Path.GetFileNameWithoutExtension(exe)} did not answer.");
        }
        return (process.ExitCode, output.GetAwaiter().GetResult());
    }

    // RegisterHotKey's modifiers in the blade's value: Alt 1, Ctrl 2, Shift 4, Win 8.
    internal const int Alt = 1, Ctrl = 2, Shift = 4, Win = 8;

    internal static (int Modifiers, int Key) SplitHotkey(string value) =>
        int.TryParse(value, out int number) && number > 0 ? (number >> 8 & 0xf, number & 0xff) : (0, 0);

    internal static string JoinHotkey(int modifiers, int key) => (modifiers << 8 | key).ToString();

    internal static string HotkeyName(string value)
    {
        (int modifiers, int key) = SplitHotkey(value);
        if (key == 0)
            return "None";
        var name = new StringBuilder();
        if ((modifiers & Ctrl) != 0) name.Append("Ctrl+");
        if ((modifiers & Alt) != 0) name.Append("Alt+");
        if ((modifiers & Shift) != 0) name.Append("Shift+");
        if ((modifiers & Win) != 0) name.Append("Win+");
        return name.Append(KeyName(key)).ToString();
    }

    private static unsafe string KeyName(int key)
    {
        if (key == 0x2c)
            return "PrtSc"; // Windows names it "Prnt Scrn"; the blades' own text says PrtSc.
        uint scan = MapVirtualKeyW((uint)key, 0); // MAPVK_VK_TO_VSC
        char* text = stackalloc char[64];
        int length = GetKeyNameTextW((int)(scan << 16 | (IsExtended(key) ? 1u << 24 : 0)), text, 64);
        return length > 0 ? new string(text, 0, length) : $"Key {key}";
    }

    // Navigation keys share scan codes with the number pad; the extended bit tells them apart.
    internal static bool IsExtended(int key) => key is >= 0x21 and <= 0x2e or >= 0x5b and <= 0x5d or 0x6f or 0x90;

    [DllImport("user32.dll")] private static extern uint MapVirtualKeyW(uint code, uint mapType);
    [DllImport("user32.dll")] private static extern unsafe int GetKeyNameTextW(int lParam, char* text, int size);
}
