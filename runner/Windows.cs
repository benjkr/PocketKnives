using System.Runtime.InteropServices;

namespace PocketKnives;

internal static unsafe class Windows
{
    private const string User = "user32.dll";

    internal const uint Destroy = 0x0002, Close = 0x0010, SetFont = 0x0030, Notify = 0x004e,
        SetIcon = 0x0080, Command = 0x0111, Timer = 0x0113, ThemeChanged = 0x031a;
    internal const uint Child = 0x40000000, Visible = 0x10000000, TabStop = 0x10000, Border = 0x800000,
        Popup = 0x80000000, Caption = 0x00c00000, SysMenu = 0x80000, MinimizeBox = 0x20000;

    [StructLayout(LayoutKind.Sequential)]
    internal struct WindowClass
    {
        public uint Style;
        public delegate* unmanaged<nint, uint, nuint, nint, nint> Procedure;
        public int ExtraClass, ExtraWindow;
        public nint Instance, Icon, Cursor, Background;
        public char* MenuName, ClassName;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct Message
    {
        public nint Window;
        public uint Id;
        public nuint WParam;
        public nint LParam;
        public uint Time;
        public int X, Y;
        public uint Private;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct Rect { public int Left, Top, Right, Bottom; }

    [DllImport("kernel32.dll")] internal static extern nint GetModuleHandleW(char* name);
    [DllImport(User)] internal static extern ushort RegisterClassW(ref WindowClass windowClass);
    [DllImport(User, CharSet = CharSet.Unicode, ExactSpelling = true)]
    internal static extern nint CreateWindowExW(uint exStyle, string className, string title, uint style,
        int x, int y, int width, int height, nint parent, nint menu, nint instance, nint data);
    [DllImport(User)] internal static extern nint DefWindowProcW(nint window, uint message, nuint wParam, nint lParam);
    [DllImport(User)] internal static extern int GetMessageW(out Message message, nint window, uint first, uint last);
    [DllImport(User)] internal static extern int IsDialogMessageW(nint window, ref Message message);
    [DllImport(User)] internal static extern int TranslateMessage(ref Message message);
    [DllImport(User)] internal static extern nint DispatchMessageW(ref Message message);
    [DllImport(User)] internal static extern void PostQuitMessage(int exitCode);
    [DllImport(User)] internal static extern int DestroyWindow(nint window);
    [DllImport(User)] internal static extern nint SendMessageW(nint window, uint message, nuint wParam, nint lParam);
    [DllImport(User)] internal static extern int ShowWindow(nint window, int command);
    [DllImport(User)] internal static extern int IsIconic(nint window);
    [DllImport(User)] internal static extern int SetForegroundWindow(nint window);
    [DllImport(User)] internal static extern uint GetWindowThreadProcessId(nint window, out int processId);
    [DllImport(User)] internal static extern int AllowSetForegroundWindow(int processId);
    [DllImport(User)] internal static extern nuint SetTimer(nint window, nuint id, uint milliseconds, nint callback);
    [DllImport(User)] internal static extern int KillTimer(nint window, nuint id);
    [DllImport(User)] internal static extern uint GetDpiForSystem();
    [DllImport(User)] internal static extern int GetSystemMetricsForDpi(int index, uint dpi);
    [DllImport(User)] internal static extern int AdjustWindowRectExForDpi(ref Rect rect, uint style, int menu, uint exStyle, uint dpi);
    [DllImport(User)] internal static extern int SystemParametersInfoForDpi(uint action, uint param, void* output, uint flags, uint dpi);
    [DllImport(User)] internal static extern nint GetSysColorBrush(int color);
    [DllImport(User)] internal static extern nint LoadCursorW(nint instance, nint name);
    [DllImport(User)] internal static extern nint LoadImageW(nint instance, nint name, uint type, int width, int height, uint flags);
    [DllImport(User)] internal static extern int DestroyIcon(nint icon);
    [DllImport(User)] internal static extern int EnableWindow(nint window, int enabled);
    [DllImport(User)] internal static extern int DrawFrameControl(nint dc, ref Rect rect, uint type, uint state);
    [DllImport(User)] internal static extern int FillRect(nint dc, ref Rect rect, nint brush);
    [DllImport(User)] internal static extern nint CreatePopupMenu();
    [DllImport(User, CharSet = CharSet.Unicode, ExactSpelling = true)]
    internal static extern int AppendMenuW(nint menu, uint flags, nuint id, string text);
    [DllImport(User)] internal static extern int SetMenuDefaultItem(nint menu, uint item, uint byPosition);
    [DllImport(User)] internal static extern int TrackPopupMenuEx(nint menu, uint flags, int x, int y, nint window, nint parameters);
    [DllImport(User)] internal static extern int DestroyMenu(nint menu);
    [DllImport(User, CharSet = CharSet.Unicode, ExactSpelling = true)]
    internal static extern int SetWindowTextW(nint window, string text);
    [DllImport(User, CharSet = CharSet.Unicode, ExactSpelling = true)]
    internal static extern int MessageBoxW(nint owner, string message, string title, uint type);
    [DllImport("gdi32.dll")] internal static extern nint CreateFontIndirectW(void* font);
    [DllImport("gdi32.dll")] internal static extern int DeleteObject(nint obj);
    [DllImport("uxtheme.dll", CharSet = CharSet.Unicode, ExactSpelling = true)]
    internal static extern int SetWindowTheme(nint window, string app, string idList);
    [DllImport("dwmapi.dll")] internal static extern int DwmSetWindowAttribute(nint window, uint attribute, ref int value, uint size);

    internal static nint Module => GetModuleHandleW(null);

    internal static void Register(string name, delegate* unmanaged<nint, uint, nuint, nint, nint> procedure, int background = -1)
    {
        fixed (char* className = name)
        {
            WindowClass value = new()
            {
                Procedure = procedure,
                Instance = Module,
                ClassName = className,
                Cursor = LoadCursorW(0, 32512), // IDC_ARROW
                Background = background < 0 ? 0 : GetSysColorBrush(background),
            };
            if (RegisterClassW(ref value) == 0)
                throw new InvalidOperationException("Could not register the PocketKnives window.");
        }
    }

    internal static nint SuiteIcon(int size) => LoadImageW(Module, 32512, 1, size, size, 0);

    internal static nint FileIcon(string path, int size)
    {
        fixed (char* file = path)
            return LoadImageW(0, (nint)file, 1, size, size, 0x10); // IMAGE_ICON, LR_LOADFROMFILE
    }

    internal static void Activate(nint window)
    {
        if (IsIconic(window) != 0)
            ShowWindow(window, 9); // SW_RESTORE
        else
            ShowWindow(window, 5); // SW_SHOW
        SetForegroundWindow(window);
    }

    internal static void Error(nint owner, string message) => MessageBoxW(owner, message, Settings.Title, 0x10);
}
