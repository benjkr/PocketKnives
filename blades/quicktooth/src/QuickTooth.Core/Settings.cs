using System.Text.Json;
using System.Text.Json.Serialization;
using System.Text.Json.Serialization.Metadata;

namespace QuickTooth;

[JsonSourceGenerationOptions(WriteIndented = true, PropertyNamingPolicy = JsonKnownNamingPolicy.CamelCase,
    DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull)]
[JsonSerializable(typeof(Settings))]
internal sealed partial class SettingsJsonContext : JsonSerializerContext;

/// <summary>Persisted in %APPDATA%\QuickTooth\settings.json (or wherever <see cref="SaveHandler"/> says).</summary>
internal sealed class Settings
{
    /// <summary>Where settings.json and the log live. A per-process hook: --settings and --set in a
    /// PocketKnives preview point it next to the exe before anything loads or logs.</summary>
    public static string Directory { get; set; } =
        Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData), "QuickTooth");

    public static string FilePath => Path.Combine(Directory, "settings.json");

    /// <summary>Target device, by container ID (primary) and Bluetooth MAC (fallback).</summary>
    public string? TargetContainerId { get; set; }
    public string? TargetMac { get; set; }
    /// <summary>Display only; never used to pick a device.</summary>
    public string? TargetName { get; set; }

    /// <summary>The tray icon was promoted out of the overflow once; never again (respect the user's choice).</summary>
    public bool TrayPromotedOnce { get; set; }

    /// <summary>The low-battery warning picked in Settings; absent (null) means the default, <see cref="Tunables.LowBatteryPercent"/>.</summary>
    public int? LowBatteryPercent { get; set; }

    /// <summary>The connected icon gets the orange dot at or below this level: the picked one if it's one of the choices, else the default.</summary>
    [JsonIgnore]
    public int LowBatteryWarning =>
        LowBatteryPercent is int p && BladeSettings.IsLowBatteryChoice(p) ? p : Tunables.LowBatteryPercent;

    [JsonIgnore]
    public Guid? TargetGuid => Guid.TryParse(TargetContainerId, out var g) && g != Guid.Empty ? g : null;

    [JsonIgnore]
    public bool HasTarget => TargetGuid != null || !string.IsNullOrEmpty(TargetMac);

    // Source-generated metadata (Native AOT has no reflection-based serializer).
    private static readonly JsonTypeInfo<Settings> Json = (JsonTypeInfo<Settings>)new JsonSerializerOptions
    {
        WriteIndented = true,
        PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
        DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull,
        Encoder = System.Text.Encodings.Web.JavaScriptEncoder.UnsafeRelaxedJsonEscaping, // keep "’" readable
        TypeInfoResolver = SettingsJsonContext.Default,
    }.GetTypeInfo(typeof(Settings));

    public static Settings Load() => Load(FilePath);

    /// <summary>Loads settings from <paramref name="path"/> (the real file, or a copy for --settings-roundtrip).</summary>
    public static Settings Load(string path)
    {
        try
        {
            if (File.Exists(path))
                return JsonSerializer.Deserialize(File.ReadAllText(path), Json) ?? new Settings();
        }
        catch (Exception ex)
        {
            Log.Error("load settings", ex);
        }
        return new Settings();
    }

    public string ToJson() => JsonSerializer.Serialize(this, Json);

    public static Settings FromJson(string json) => JsonSerializer.Deserialize(json, Json) ?? new Settings();

    /// <summary>
    /// Where Save() goes instead of the file, when set. The simulator keeps its settings in the page.
    /// </summary>
    public static Action<Settings>? SaveHandler { get; set; }

    /// <summary>Saves through a temp file and a move; false if the file couldn't be written.</summary>
    public bool Save()
    {
        if (SaveHandler is { } handler) { handler(this); return true; }
        try
        {
            System.IO.Directory.CreateDirectory(Directory);
            string tmp = FilePath + ".tmp";
            File.WriteAllText(tmp, ToJson());
            File.Move(tmp, FilePath, overwrite: true);
            return true;
        }
        catch (Exception ex)
        {
            Log.Error("save settings", ex);
            return false;
        }
    }

    public void SetTarget(BtDevice device)
    {
        TargetContainerId = device.ContainerId.ToString("B");
        TargetMac = device.Mac;
        TargetName = device.Name;
    }

    /// <summary>The user picked <paramref name="device"/> (the tray's Device menu, or --set device=…); not saved yet.</summary>
    public void ChooseTarget(BtDevice device)
    {
        SetTarget(device);
        Log.Info($"target set: {device.Name} {device.ContainerId:B} MAC {device.Mac}");
    }

    /// <summary>
    /// Finds the stored target: by container ID, then by MAC. If a target is stored but absent,
    /// returns null — it never falls back to some other device.
    /// </summary>
    public BtDevice? Resolve(IReadOnlyList<BtDevice> devices)
    {
        if (TargetGuid is Guid g && devices.FirstOrDefault(d => d.ContainerId == g) is { } byId) return byId;
        if (!string.IsNullOrEmpty(TargetMac) &&
            devices.FirstOrDefault(d => string.Equals(d.Mac, TargetMac, StringComparison.OrdinalIgnoreCase)) is { } byMac)
            return byMac;
        return null;
    }

    /// <summary>Stored target if set (never substituted); otherwise the first-run default, saved if <paramref name="persist"/>.</summary>
    public BtDevice? ResolveOrDefault(IReadOnlyList<BtDevice> devices, bool persist)
    {
        if (HasTarget) return Resolve(devices);
        var pick = DefaultTarget(devices);
        if (pick != null && persist)
        {
            SetTarget(pick);
            Save();
            Log.Info($"default target chosen: {pick.Name} {pick.ContainerId:B} MAC {pick.Mac}");
        }
        return pick;
    }

    /// <summary>
    /// First-run default: a device whose name contains "AirPods" (prefer one that's connected).
    /// Only used when no target has ever been stored.
    /// </summary>
    public static BtDevice? DefaultTarget(IReadOnlyList<BtDevice> devices) =>
        devices.Where(d => d.Name.Contains("AirPods", StringComparison.OrdinalIgnoreCase))
               .OrderByDescending(d => d.Connected)
               .FirstOrDefault();
}
