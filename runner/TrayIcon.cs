using System.Runtime.InteropServices;
using static PocketKnives.Windows;

namespace PocketKnives;

internal sealed unsafe class TrayIcon(nint window, Guid identity) : IDisposable
{
    internal const uint Callback = 0x8001; // WM_APP
    private const uint Add = 0, Delete = 2, SetVersion = 4;

    private NotifyIconData data = new()
    {
        Size = (uint)sizeof(NotifyIconData),
        Window = window,
        Id = 1,
        Flags = 0x01 | 0x02 | 0x04 | 0x20 | 0x80, // MESSAGE, ICON, TIP, GUID, SHOWTIP
        CallbackMessage = Callback,
        Guid = identity,
        Version = 4,
    };
    private bool registered;

    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Unicode)]
    private struct NotifyIconData
    {
        public uint Size;
        public nint Window;
        public uint Id, Flags, CallbackMessage;
        public nint Icon;
        public fixed char Tooltip[128];
        public uint State, StateMask;
        public fixed char Info[256];
        public uint Version;
        public fixed char InfoTitle[64];
        public uint InfoFlags;
        public Guid Guid;
        public nint BalloonIcon;
    }

    [DllImport("shell32.dll")]
    private static extern int Shell_NotifyIconW(uint message, ref NotifyIconData icon);

    internal bool Register()
    {
        Dispose();
        int size = GetSystemMetricsForDpi(49, GetDpiForSystem()); // SM_CXSMICON
        data.Icon = SuiteIcon(size >= 32 ? 32 : 16);
        if (data.Icon == 0)
            return false;

        fixed (char* tooltip = data.Tooltip)
            Settings.Title.AsSpan().CopyTo(new Span<char>(tooltip, 128));

        registered = Shell_NotifyIconW(Add, ref data) != 0;
        if (registered && Shell_NotifyIconW(SetVersion, ref data) != 0)
            return true;

        Dispose();
        return false;
    }

    public void Dispose()
    {
        if (registered)
            Shell_NotifyIconW(Delete, ref data);
        registered = false;
        if (data.Icon != 0)
            DestroyIcon(data.Icon);
        data.Icon = 0;
    }
}
