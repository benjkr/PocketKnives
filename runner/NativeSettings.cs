using System.Runtime.InteropServices;
using static PocketKnives.Windows;

namespace PocketKnives;

internal sealed unsafe class NativeSettings(Settings.Look look, bool[] original)
{
    private const string ClassName = "PocketKnives.Settings.Win32";
    private const uint SetImageList = 0x1003, SetItemState = 0x102b, GetItemState = 0x102c;
    private const uint StateImageMask = 0xf000, Selected = 2, Focused = 1;
    private const int OkId = 1, CancelId = 2, ApplyId = 3, LookId = 100;
    private static NativeSettings? current;

    private readonly uint dpi = GetDpiForSystem();
    private bool Classic => look == Settings.Look.Xp;
    private nint window, font, boldFont, iconList, generalList, bladeList, applyButton;
    private nint pageTitle, description, pageIcon, lookLabel, lookChoice;
    private readonly List<nint> icons = [];
    private readonly nint[] pageIcons = new nint[Settings.Blades.Length + 1];

    internal static nint Handle => current?.window ?? 0;

    internal static void Initialize() => Register(ClassName, &WindowProcedure, 15); // COLOR_BTNFACE

    internal static void Open(Settings.Look look)
    {
        current = new NativeSettings(look, Settings.ReadStartup(Settings.Root));
        try
        {
            current.Build();
            Activate(current.window);
        }
        catch
        {
            CloseWindow();
            throw;
        }
    }

    internal static void CloseWindow()
    {
        if (current is not { } settings)
            return;
        current = null;
        if (settings.window != 0)
        {
            // Both lists use the same small images. Detach them before destroying either list.
            SendMessageW(settings.generalList, SetImageList, 1, 0);
            SendMessageW(settings.bladeList, SetImageList, 1, 0);
            DestroyWindow(settings.window);
        }
        if (settings.iconList != 0)
            ImageList_Destroy(settings.iconList);
        foreach (nint icon in settings.icons)
            DestroyIcon(icon);
        DeleteObject(settings.font);
        DeleteObject(settings.boldFont);
    }

    private int Scale(int px) => (int)((px * dpi + 48) / 96);

    private void Build()
    {
        uint* commonControls = stackalloc uint[] { 8, 1 }; // ICC_LISTVIEW_CLASSES
        InitCommonControlsEx(commonControls);
        CreateFonts();

        const uint style = Caption | SysMenu | MinimizeBox;
        const uint extendedStyle = 0x10000 | 0x1; // WS_EX_CONTROLPARENT, WS_EX_DLGMODALFRAME
        Rect bounds = new() { Right = Scale(564), Bottom = Scale(262) };
        AdjustWindowRectExForDpi(ref bounds, style, 0, extendedStyle, dpi);
        window = CreateWindowExW(extendedStyle, ClassName, Settings.Title, style, int.MinValue, int.MinValue,
            bounds.Right - bounds.Left, bounds.Bottom - bounds.Top, 0, 0, Module, 0);
        if (window == 0)
            throw new InvalidOperationException("Could not create the settings window.");

        SendMessageW(window, SetIcon, 0, Own(SuiteIcon(16)));
        SendMessageW(window, SetIcon, 1, Own(SuiteIcon(32)));
        if (Classic)
        {
            SetWindowTheme(window, "", "");
            int square = 1;
            DwmSetWindowAttribute(window, 33, ref square, sizeof(int)); // DWMWA_WINDOW_CORNER_PREFERENCE
        }

        int size = Scale(16);
        iconList = ImageList_Create(size, size, 0x20, Settings.Blades.Length + 1, 1); // ILC_COLOR32
        AddRowIcon(Own(SuiteIcon(size >= 32 ? 32 : 16)));
        foreach (Settings.Blade blade in Settings.Blades)
            AddRowIcon(Own(FileIcon(Settings.BladeIcon(Settings.Root, blade), size)));

        generalList = CreateList(8, 8, 156, 28, false);
        AddRow(generalList, 0, Settings.GeneralName, 0, 0);
        Control("STATIC", Settings.BladesLabel, 0, 8, 46, 156, 18, useBold: true);
        bladeList = CreateList(8, 64, 156, 128, true);
        for (int i = 0; i < Settings.Blades.Length; i++)
            AddRow(bladeList, i, Settings.Blades[i].Name, i + 1, original[i] ? 2u : 1u);
        Control("STATIC", "Checked blades start when you sign in.", 0, 8, 198, 156, 36);

        pageTitle = Control("STATIC", "", 0, 178, 8, 332, 18, useBold: true);
        description = Control("STATIC", "", 0, 178, 28, 330, 38);
        pageIcon = Control("STATIC", "", 3, 522, 8, 32, 32); // SS_ICON
        lookLabel = Control("STATIC", "&" + Settings.LookLabel, 0, 178, 70, 120, 18, useBold: true);
        lookChoice = Control("COMBOBOX", "", TabStop | 0x200000 | 3, 178, 90, 200, 160, LookId);
        foreach (string name in Settings.LookNames)
            fixed (char* text = name)
                SendMessageW(lookChoice, 0x0143, 0, (nint)text); // CB_ADDSTRING
        SendMessageW(lookChoice, 0x014e, (nuint)look, 0); // CB_SETCURSEL

        uint buttonStyle = TabStop | (Classic ? 0x8000u : 0); // BS_FLAT
        Control("BUTTON", "OK", buttonStyle | 1, 322, 230, 75, 24, OkId);
        Control("BUTTON", "Cancel", buttonStyle, 402, 230, 75, 24, CancelId);
        applyButton = Control("BUTTON", "&Apply", buttonStyle, 482, 230, 75, 24, ApplyId);
        EnableWindow(applyButton, 0);
        SetState(generalList, 0, Selected | Focused, Selected | Focused);
        ShowPage(0);
    }

    private void CreateFonts()
    {
        const int nonClientMetricsSize = 504;
        byte* metrics = stackalloc byte[nonClientMetricsSize];
        new Span<byte>(metrics, nonClientMetricsSize).Clear();
        *(uint*)metrics = nonClientMetricsSize;
        if (SystemParametersInfoForDpi(0x29, nonClientMetricsSize, metrics, 0, dpi) == 0) // SPI_GETNONCLIENTMETRICS
            return;
        byte* messageFont = metrics + 408; // NONCLIENTMETRICSW.lfMessageFont
        font = CreateFontIndirectW(messageFont);
        *(int*)(messageFont + 16) = 700; // LOGFONTW.lfWeight
        boldFont = CreateFontIndirectW(messageFont);
    }

    private nint Control(string className, string text, uint style, int x, int y, int width, int height,
        int id = 0, bool useBold = false, uint extendedStyle = 0)
    {
        nint control = CreateWindowExW(extendedStyle, className, text, Child | Visible | style,
            Scale(x), Scale(y), Scale(width), Scale(height), window, id, Module, 0);
        if (control == 0)
            throw new InvalidOperationException("Could not create a settings control.");
        SendMessageW(control, SetFont, (nuint)(useBold ? boldFont : font), 0);
        if (Classic)
            SetWindowTheme(control, "", "");
        return control;
    }

    private nint CreateList(int x, int y, int width, int height, bool checkboxes)
    {
        const uint listStyle = 0x1 | 0x4 | 0x8 | 0x4000; // REPORT, SINGLESEL, SHOWSELALWAYS, NOCOLUMNHEADER
        nint list = Control("SysListView32", "", TabStop | listStyle | (Classic ? Border : 0),
            x, y, width, height, extendedStyle: Classic ? 0u : 0x200); // WS_EX_CLIENTEDGE
        if (!Classic)
            SetWindowTheme(list, "Explorer", "");
        uint styles = 0x20 | (checkboxes ? 0x4u : 0); // LVS_EX_FULLROWSELECT, LVS_EX_CHECKBOXES
        SendMessageW(list, 0x1036, styles, (nint)styles); // LVM_SETEXTENDEDLISTVIEWSTYLE
        if (checkboxes && Classic)
        {
            // Windows themes the list's built-in checkboxes even after SetWindowTheme.
            nint stateImages = ClassicCheckboxes();
            if (stateImages != 0)
                ImageList_Destroy(SendMessageW(list, SetImageList, 2, stateImages));
        }
        SendMessageW(list, SetImageList, 1, iconList);
        ListColumn column = new() { Mask = 2, Width = Scale(width - 6) }; // LVCF_WIDTH
        SendMessageW(list, 0x1061, 0, (nint)(&column)); // LVM_INSERTCOLUMNW
        return list;
    }

    private nint ClassicCheckboxes()
    {
        int size = Scale(16), box = Scale(13), inset = (size - box) / 2;
        nint images = ImageList_Create(size, size, 0x18, 2, 0); // ILC_COLOR24
        nint dc = CreateCompatibleDC(0);
        nint bitmap = CreateBitmap(size, size, 1, 32, 0);
        nint previous = SelectObject(dc, bitmap);
        foreach (uint state in new uint[] { 0x4000, 0x4400 }) // DFCS_FLAT, DFCS_FLAT | DFCS_CHECKED
        {
            Rect background = new() { Right = size, Bottom = size };
            FillRect(dc, ref background, GetSysColorBrush(5)); // COLOR_WINDOW
            Rect check = new() { Left = inset, Top = inset, Right = inset + box, Bottom = inset + box };
            DrawFrameControl(dc, ref check, 4, state); // DFC_BUTTON, DFCS_BUTTONCHECK
            ImageList_Add(images, bitmap, 0);
        }
        SelectObject(dc, previous);
        DeleteObject(bitmap);
        DeleteDC(dc);
        return images;
    }

    private void AddRowIcon(nint icon)
    {
        if (icon == 0)
            icon = Own(SuiteIcon(16));
        if (icon != 0)
            ImageList_ReplaceIcon(iconList, -1, icon);
    }

    private nint Own(nint icon)
    {
        if (icon != 0)
            icons.Add(icon);
        return icon;
    }

    private static void AddRow(nint list, int row, string name, int image, uint checkbox)
    {
        fixed (char* text = name)
        {
            ListItem item = new() { Mask = 3, Index = row, Text = text, Image = image };
            SendMessageW(list, 0x104d, 0, (nint)(&item)); // LVM_INSERTITEMW
        }
        SetState(list, (nuint)row, checkbox << 12, StateImageMask);
    }

    private static void SetState(nint list, nuint row, uint state, uint mask)
    {
        ListItem item = new() { State = state, StateMask = mask };
        SendMessageW(list, SetItemState, row, (nint)(&item));
    }

    private void ShowPage(int page)
    {
        bool general = page == 0;
        Settings.Blade? blade = general ? null : Settings.Blades[page - 1];
        SetWindowTextW(pageTitle, blade?.Name ?? Settings.GeneralName);
        SetWindowTextW(description, blade?.Description ?? Settings.GeneralDescription);
        int fit = Scale(32);
        int size = fit >= 64 ? 64 : fit >= 48 ? 48 : 32;
        if (pageIcons[page] == 0)
            pageIcons[page] = Own(blade is null ? SuiteIcon(size) : FileIcon(Settings.BladeIcon(Settings.Root, blade), fit));
        SendMessageW(pageIcon, 0x0170, (nuint)pageIcons[page], 0); // STM_SETICON
        ShowWindow(lookLabel, general ? 5 : 0);
        ShowWindow(lookChoice, general ? 5 : 0);
    }

    private bool[] ReadSelection()
    {
        var checkedRows = new bool[Settings.Blades.Length];
        for (int i = 0; i < checkedRows.Length; i++)
            checkedRows[i] = SendMessageW(bladeList, GetItemState, (nuint)i, (nint)StateImageMask) == 0x2000;
        return checkedRows;
    }

    private void UpdateApply() =>
        EnableWindow(applyButton, ReadSelection().AsSpan().SequenceEqual(original) ? 0 : 1);

    private void Apply()
    {
        Settings.ApplyStartup(Settings.Root, original, ReadSelection());
        UpdateApply();
    }

    private void Execute(int id, int notification)
    {
        try
        {
            switch (id)
            {
                case OkId:
                    Apply();
                    CloseWindow();
                    break;
                case CancelId:
                    CloseWindow();
                    break;
                case ApplyId:
                    Apply();
                    break;
                case LookId when notification == 1: // CBN_SELCHANGE
                    var selected = (Settings.Look)SendMessageW(lookChoice, 0x0147, 0, 0); // CB_GETCURSEL
                    if ((uint)selected < Settings.LookNames.Length && selected != look)
                    {
                        Apply();
                        Settings.SaveLook(Settings.Root, selected);
                        Program.ReopenNativeSettings();
                    }
                    break;
            }
        }
        catch (Exception error) when (Settings.IsSettingsError(error))
        {
            SendMessageW(lookChoice, 0x014e, (nuint)look, 0);
            UpdateApply();
            Error(window, error.Message);
        }
    }

    [UnmanagedCallersOnly]
    private static nint WindowProcedure(nint window, uint message, nuint wParam, nint lParam)
    {
        if (current is not { } settings || settings.window != window)
            return DefWindowProcW(window, message, wParam, lParam);

        switch (message)
        {
            case Close:
                CloseWindow();
                return 0;
            case Command:
                settings.Execute((int)(wParam & 0xffff), (int)((wParam >> 16) & 0xffff));
                return 0;
            case Notify when settings.applyButton != 0:
                var change = (ListNotification*)lParam;
                if (change->Code != -101 || change->Index < 0 ||
                    (change->From != settings.generalList && change->From != settings.bladeList))
                    break;
                if ((change->NewState & Selected) != 0 && (change->OldState & Selected) == 0)
                {
                    bool general = change->From == settings.generalList;
                    SetState(general ? settings.bladeList : settings.generalList, nuint.MaxValue, 0, Selected | Focused);
                    settings.ShowPage(general ? 0 : change->Index + 1);
                }
                if (((change->NewState ^ change->OldState) & StateImageMask) != 0)
                    settings.UpdateApply();
                return 0;
        }
        return DefWindowProcW(window, message, wParam, lParam);
    }

    [StructLayout(LayoutKind.Sequential)]
    private struct ListItem
    {
        public uint Mask;
        public int Index, SubItem;
        public uint State, StateMask;
        public char* Text;
        public int TextCapacity, Image;
        public nint Data;
        public int Indent, GroupId;
        public uint Columns;
        public nint ColumnIndices, ColumnFormats;
        public int Group;
    }

    [StructLayout(LayoutKind.Sequential)]
    private struct ListColumn
    {
        public uint Mask;
        public int Format, Width;
        public nint Text;
        public int TextCapacity, SubItem, Image, Order, MinWidth, DefaultWidth, IdealWidth;
    }

    [StructLayout(LayoutKind.Sequential)]
    private struct ListNotification
    {
        public nint From;
        public nuint Id;
        public int Code;
        private int padding; // sizeof(NMHDR) includes tail padding on x64.
        public int Index, SubItem;
        public uint NewState, OldState, Changed;
        public int X, Y;
        public nint Data;
    }

    [DllImport("comctl32.dll")] private static extern int InitCommonControlsEx(uint* controls);
    [DllImport("comctl32.dll")] private static extern nint ImageList_Create(int x, int y, uint flags, int initial, int grow);
    [DllImport("comctl32.dll")] private static extern int ImageList_Destroy(nint list);
    [DllImport("comctl32.dll")] private static extern int ImageList_ReplaceIcon(nint list, int index, nint icon);
    [DllImport("comctl32.dll")] private static extern int ImageList_Add(nint list, nint image, nint mask);
    [DllImport("gdi32.dll")] private static extern nint CreateCompatibleDC(nint dc);
    [DllImport("gdi32.dll")] private static extern nint CreateBitmap(int width, int height, uint planes, uint bpp, nint pixels);
    [DllImport("gdi32.dll")] private static extern nint SelectObject(nint dc, nint obj);
    [DllImport("gdi32.dll")] private static extern int DeleteDC(nint dc);
}
