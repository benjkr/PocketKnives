using System.Text;

namespace QuickTooth;

/// <summary>What became of a --set. The tray returns it to the CLI as the WM_COPYDATA result, so 0 stays "not handled".</summary>
internal enum SetResult { Applied = 1, Busy = 2, DeviceNotAvailable = 3, SaveFailed = 4, Invalid = 5 }

/// <summary>A --set whose form is checked. Device: a container ID in "B" format, "" for none; null = not asked to change.</summary>
internal sealed record SettingsChange(string? Device, int? LowBattery);

/// <summary>
/// QuickTooth's side of the PocketKnives Settings contract (docs/specs/blade-settings.md at the repo
/// root): the lines --settings prints, the checks --set makes and the words for a refusal. Used by
/// the CLI, by the running tray a --set is handed to, and by the simulator.
/// </summary>
internal static class BladeSettings
{
    public const string DeviceKey = "device", LowBatteryKey = "lowBattery";
    public static readonly int[] LowBatteryChoices = [10, 15, 20, 25, 30];

    /// <summary>
    /// The --settings lines: the headset (the devices the Device menu offers, plus the stored one
    /// marked "(not available)" when it's missing, as in the menu) and the low-battery warning.
    /// </summary>
    public static string Describe(Settings settings, IReadOnlyList<BtDevice> devices)
    {
        var sb = new StringBuilder();
        var target = settings.HasTarget ? settings.Resolve(devices) : null;
        sb.Append(DeviceKey).Append("\tchoice\t").Append(CurrentDevice(settings, devices)).Append("\tHeadset");
        foreach (var d in devices) Choice(sb, d.ContainerId, d.Name);
        if (target == null && settings.TargetGuid is Guid missing)
            Choice(sb, missing, $"{settings.TargetName ?? "Saved device"} (not available)");
        sb.Append('\n');

        sb.Append(LowBatteryKey).Append("\tchoice\t").Append(settings.LowBatteryWarning).Append("\tLow battery warning");
        foreach (int p in LowBatteryChoices) sb.Append('\t').Append(p).Append('=').Append(p).Append('%');
        sb.Append('\n');
        return sb.ToString();

        static void Choice(StringBuilder sb, Guid id, string label) =>
            sb.Append('\t').Append(id.ToString("B")).Append('=').Append(label.Replace('\t', ' ').Replace('\r', ' ').Replace('\n', ' '));
    }

    /// <summary>The headset --settings shows as chosen: the stored target (as resolved), "" if none.</summary>
    private static string CurrentDevice(Settings settings, IReadOnlyList<BtDevice> devices) =>
        (settings.HasTarget ? settings.Resolve(devices)?.ContainerId ?? settings.TargetGuid : null)?.ToString("B") ?? "";

    /// <summary>
    /// Checks the pairs' form, keys and values, as far as that's possible without the device list.
    /// Null, with the line to show, when one is wrong.
    /// </summary>
    public static SettingsChange? Parse(IReadOnlyList<string> pairs, out string error)
    {
        error = "";
        string? device = null;
        int? low = null;
        if (pairs.Count == 0) { error = "Nothing to set."; return null; }
        foreach (string pair in pairs)
        {
            int eq = pair.IndexOf('=');
            if (eq <= 0) { error = $"Expected key=value, got \"{pair}\"."; return null; }
            string key = pair[..eq], value = pair[(eq + 1)..];
            switch (key)
            {
                case DeviceKey:
                    if (value.Length == 0) device = "";
                    else if (Guid.TryParse(value, out var g)) device = g.ToString("B");
                    else { error = Message(SetResult.DeviceNotAvailable); return null; }
                    break;
                case LowBatteryKey:
                    low = null;
                    foreach (int c in LowBatteryChoices)
                        if (value == c.ToString(System.Globalization.CultureInfo.InvariantCulture)) low = c;
                    if (low == null)
                    {
                        error = "Low battery warning can be " + ChoiceList() + ".";
                        return null;
                    }
                    break;
                default:
                    error = $"QuickTooth has no setting called \"{key}\".";
                    return null;
            }
        }
        return new SettingsChange(device, low);
    }

    /// <summary>
    /// Checks a headset change against <paramref name="devices"/> (what the Device menu offers).
    /// The headset already chosen is no change, even while it's missing; any other must be there.
    /// <paramref name="device"/> is the one to choose, or null for no change.
    /// </summary>
    public static SetResult Check(SettingsChange change, Settings settings, IReadOnlyList<BtDevice> devices, out BtDevice? device)
    {
        device = null;
        if (change.Device == null || change.Device == CurrentDevice(settings, devices)) return SetResult.Applied;
        device = devices.FirstOrDefault(d => d.ContainerId.ToString("B") == change.Device);
        return device != null ? SetResult.Applied : SetResult.DeviceNotAvailable;
    }

    /// <summary>Applies a checked change (<paramref name="device"/> from <see cref="Check"/>) and saves it, if it changes anything.</summary>
    public static SetResult Apply(SettingsChange change, Settings settings, BtDevice? device)
    {
        bool changed = device != null;
        if (change.LowBattery is int p && p != settings.LowBatteryWarning)
        {
            changed = true;
            settings.LowBatteryPercent = p;
            Log.Info($"low battery warning set: {p}%");
        }
        if (device != null) settings.ChooseTarget(device);
        return !changed || settings.Save() ? SetResult.Applied : SetResult.SaveFailed;
    }

    public static bool IsLowBatteryChoice(int percent) => Array.IndexOf(LowBatteryChoices, percent) >= 0;

    /// <summary>"10%, 15%, 20%, 25% or 30%".</summary>
    private static string ChoiceList()
    {
        var sb = new StringBuilder();
        for (int i = 0; i < LowBatteryChoices.Length; i++)
            sb.Append(i == 0 ? "" : i == LowBatteryChoices.Length - 1 ? " or " : ", ").Append(LowBatteryChoices[i]).Append('%');
        return sb.ToString();
    }

    /// <summary>The line a refusal shows in Settings.</summary>
    public static string Message(SetResult result) => result switch
    {
        SetResult.Applied => "",
        SetResult.Busy => "QuickTooth is connecting or disconnecting. Try again in a moment.",
        SetResult.DeviceNotAvailable => "That headset isn't available right now.",
        SetResult.SaveFailed => "QuickTooth couldn't save its settings.",
        _ => "QuickTooth didn't understand that change.",
    };
}
