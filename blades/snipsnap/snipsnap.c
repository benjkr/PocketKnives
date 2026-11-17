/*
 * SnipSnap - a tiny, fast screen snippet tool for Windows.
 *
 * Three captures: a dragged rectangle, a window you click, or the monitor
 * under the cursor. The result goes to the clipboard as a 24-bit CF_DIB, and
 * the captured area flashes briefly. Nothing else.
 *
 *   snipsnap            run in the background and listen for hotkeys
 *   snipsnap rect       select a rectangle once, then exit
 *   snipsnap window     pick a window once, then exit
 *   snipsnap full       capture the monitor under the cursor once, then exit
 *   snipsnap quit       stop the background instance
 *   snipsnap settings   print the settings (for PocketKnives Settings)
 *   snipsnap set k=v..  change settings and apply them to the background instance
 *
 * Exit codes: 0 = copied / ok, 1 = cancelled, 2 = failed / bad usage.
 */
#define WIN32_LEAN_AND_MEAN
#define UNICODE
#define _UNICODE
#define _WIN32_WINNT 0x0A00
#define WINVER 0x0A00
#include <windows.h>
#include <windowsx.h>
#include <dwmapi.h>

#pragma comment(lib, "kernel32.lib")
#pragma comment(lib, "user32.lib")
#pragma comment(lib, "gdi32.lib")
#pragma comment(lib, "dwmapi.lib")

enum { RC_OK = 0, RC_CANCEL = 1, RC_FAIL = 2 };
enum { HK_RECT = 1, HK_WINDOW = 2, HK_FULL = 3, HK_ESC = 4 };

#define DAEMON_CLASS  L"SnipSnapDaemon"
#define OVERLAY_CLASS L"SnipSnapOverlay"
#define FLASH_CLASS   L"SnipSnapFlash"
#define MUTEX_NAME    L"Local\\SnipSnap.Daemon"
#define FRAME_COLOR   RGB(255, 255, 255)   /* selection / hovered-window border */
#define FRAME_W       2                    /* border thickness, px */
#define FLASH_ALPHA   150               /* starting opacity of the white flash */
#define FLASH_MS      150               /* fade-out duration */
#ifndef SS_ROP
#define SS_ROP        (SRCCOPY | CAPTUREBLT)   /* CAPTUREBLT: include layered windows */
#endif

/* A hotkey as one number, modifiers * 256 + virtual key: the settings format. */
#define HK(mod, vk)   ((UINT)(mod) << 8 | (UINT)(vk))
#define HK_MAX        HK(MOD_ALT | MOD_CONTROL | MOD_SHIFT | MOD_WIN, 0xFF)

/* Hotkey candidates for a capture with no saved hotkey, tried in order; the
 * first one that registers wins. A capture's list ends where the next one's
 * starts (SETTINGS below). */
typedef struct { UINT v; const char *name; } Hotkey;
static const Hotkey FALLBACKS[] = {
    /* Rectangle */
    { HK(MOD_CONTROL | MOD_ALT,             'R'),         "Ctrl+Alt+R" },
    { HK(MOD_CONTROL,                       VK_SNAPSHOT), "Ctrl+PrtSc" },
    { HK(MOD_CONTROL | MOD_ALT | MOD_SHIFT, 'R'),         "Ctrl+Alt+Shift+R" },
    /* Window */
    { HK(MOD_CONTROL | MOD_ALT,             'W'),         "Ctrl+Alt+W" },
    { HK(MOD_CONTROL | MOD_ALT | MOD_SHIFT, 'W'),         "Ctrl+Alt+Shift+W" },
    /* Full screen */
    { HK(MOD_CONTROL | MOD_ALT,             'F'),         "Ctrl+Alt+F" },
    { HK(MOD_CONTROL | MOD_SHIFT,           VK_SNAPSHOT), "Ctrl+Shift+PrtSc" },
    { HK(MOD_CONTROL | MOD_ALT | MOD_SHIFT, 'F'),         "Ctrl+Alt+Shift+F" },
};

/* The settings, in the order PocketKnives Settings shows them. The first
 * three are the hotkeys of captures HK_RECT, HK_WINDOW and HK_FULL. */
enum { S_HOTKEYS = 3, S_FLASH = 3, S_COUNT = 4 };
typedef struct { const wchar_t *key; const char *label; const Hotkey *keys; } Setting;
static const Setting SETTINGS[S_COUNT] = {
    { L"rect",   "Rectangle",               FALLBACKS },
    { L"window", "Window",                  FALLBACKS + 3 },
    { L"full",   "Full screen",             FALLBACKS + 5 },
    { L"flash",  "Flash the captured area", FALLBACKS + 8 },   /* the end of the list */
};
#define INI_SECTION   L"SnipSnap"
#define RELOAD_MSG    L"PocketKnives.Reload.v1"
#define RELOAD_DONE   0x100     /* reload reply: handled; low bits = captures without a key */

static BOOL g_daemon;   /* running as the background instance */

static BOOL eq(const wchar_t *a, const wchar_t *b)
{
    while (*a && *a == *b) { a++; b++; }
    return *a == *b;
}

/* ---------------------------------------------------------------- output */

static HANDLE g_out;

static void say(const char *s)
{
    DWORD n = 0;
    if (!g_out) {
        HANDLE h = GetStdHandle(STD_OUTPUT_HANDLE);
        if (h && h != INVALID_HANDLE_VALUE && GetFileType(h) != FILE_TYPE_UNKNOWN)
            g_out = h;
        else if (AttachConsole(ATTACH_PARENT_PROCESS))
            g_out = CreateFileW(L"CONOUT$", GENERIC_WRITE, FILE_SHARE_WRITE, NULL,
                                OPEN_EXISTING, 0, NULL);
        if (!g_out) g_out = INVALID_HANDLE_VALUE;
    }
    if (g_out == INVALID_HANDLE_VALUE) return;
    while (s[n]) n++;
    WriteFile(g_out, s, n, &n, NULL);
}

/* Let go of any inherited console / pipe so a background instance never keeps
 * a terminal or a capturing parent waiting. */
static void detach_output(void)
{
    if (g_out && g_out != INVALID_HANDLE_VALUE &&
        g_out != GetStdHandle(STD_OUTPUT_HANDLE))
        CloseHandle(g_out);
    g_out = INVALID_HANDLE_VALUE;
    for (DWORD id = STD_ERROR_HANDLE; id <= STD_INPUT_HANDLE; id++) {
        HANDLE h = GetStdHandle(id);
        if (h && h != INVALID_HANDLE_VALUE) CloseHandle(h);
        SetStdHandle(id, NULL);
    }
    FreeConsole();
}

/* -------------------------------------------------------------- settings
 *
 * An INI file read and written through kernel32: %APPDATA%\SnipSnap\snipsnap.ini,
 * or snipsnap.ini next to the exe in a PocketKnives preview. A hotkey is saved
 * as HK(mod, vk); a capture without one uses its fallback list. */

static wchar_t g_ini[MAX_PATH + 32];   /* empty: no settings file */
static DWORD g_ini_dir;                /* where its folder's name ends; 0 = don't create it */
static BOOL g_preview;                 /* POCKETKNIVES_SETTINGS_PREVIEW=1 */
static UINT g_set[S_COUNT];            /* hotkeys: saved value or 0; flash: 1 or 0 */

static BOOL hotkey_ok(UINT v) { return v <= HK_MAX && (v & 0xFF); }

/* A capture's hotkey: the saved one, or the first of its fallbacks. (Not
 * inlining this, ini_get and refuse makes the code smaller.) */
static __declspec(noinline) UINT hotkey_of(int i) { return g_set[i] ? g_set[i] : SETTINGS[i].keys[0].v; }

/* The saved value, or (UINT)-1 when there's none. */
static __declspec(noinline) UINT ini_get(int i)
{
    return g_ini[0] ? GetPrivateProfileIntW(INI_SECTION, SETTINGS[i].key, -1, g_ini) : (UINT)-1;
}

/* Save a value; (UINT)-1 removes it. */
static BOOL ini_put(int i, UINT v)
{
    wchar_t num[12], *d = num + 11;
    UINT k = v;
    *d = 0;
    do *--d = (wchar_t)(L'0' + k % 10); while (k /= 10);
    return g_ini[0] && WritePrivateProfileStringW(INI_SECTION, SETTINGS[i].key,
                                                  v == (UINT)-1 ? NULL : d, g_ini);
}

static void settings_load(void)
{
    static const wchar_t TAIL[] = L"\\SnipSnap\\snipsnap.ini";   /* a preview's: its end */
    const wchar_t *tail = TAIL;
    wchar_t e[2];
    DWORD n;
    g_preview = GetEnvironmentVariableW(L"POCKETKNIVES_SETTINGS_PREVIEW", e, 2) == 1 && e[0] == L'1';
    if (g_preview) {
        n = GetModuleFileNameW(NULL, g_ini, MAX_PATH);
        while (n && g_ini[--n] != L'\\') ;                      /* the exe's folder */
        tail += 9;
    } else {
        n = GetEnvironmentVariableW(L"APPDATA", g_ini, MAX_PATH);
        if (n >= MAX_PATH) n = 0;
    }
    g_ini_dir = tail == TAIL ? n + 9 : 0;
    for (DWORD k = 0; (g_ini[n + k] = tail[k]) != 0; k++) ;
    if (!n) g_ini[0] = 0;
    for (int i = 0; i < S_COUNT; i++) {
        UINT v = ini_get(i);
        g_set[i] = i < S_HOTKEYS ? (hotkey_ok(v) ? v : 0) : v != 0;
    }
}

/* A hotkey's name for people, such as "Ctrl+Alt+A", in UTF-8. */
static char *key_name(char out[96], UINT v)
{
    /* Each modifier's bit, then its name. */
    const char *m = "\2Ctrl+\0\1Alt+\0\4Shift+\0\10Win+\0";
    char *o = out;
    wchar_t w[24];
    UINT vk = v & 0xFF, sc = MapVirtualKeyW(vk, MAPVK_VK_TO_VSC_EX);
    while (*m) {
        UINT bit = (BYTE)*m++;
        for (; *m; m++)
            if (v >> 8 & bit) *o++ = *m;
        m++;
    }
    /* Windows quirk: MapVirtualKey doesn't mark the navigation keys and Num
     * Lock as extended (so they'd be named as keypad keys), and gives Pause an
     * E1 scan code (named "Right Ctrl"). */
    if (vk == VK_PAUSE) sc = 0x45;
    else if (vk - VK_PRIOR <= VK_DELETE - VK_PRIOR || vk == VK_NUMLOCK) sc |= 0xE000;
    /* Scan code in bits 16-23, extended-key flag in bit 24. F13-F24 have no
     * name; PrtSc is named as in FALLBACKS (Windows: "Prnt Scrn"). */
    if (vk == VK_SNAPSHOT)
        wsprintfA(o, "PrtSc");
    else if (GetKeyNameTextW((LONG)((sc & 0xFF) << 16 | (sc > 0xFF) << 24), w, ARRAYSIZE(w)))
        WideCharToMultiByte(CP_UTF8, 0, w, -1, o, 96 - (int)(o - out), NULL, NULL);
    else if (vk - VK_F1 < 24)
        wsprintfA(o, "F%u", vk - VK_F1 + 1);
    else
        wsprintfA(o, "Key %u", vk);
    return out;
}

static BOOL try_key(HWND h, int id, UINT v)
{
    return RegisterHotKey(h, id, v >> 8 | MOD_NOREPEAT, v & 0xFF);
}

/* ----------------------------------------------------------------- flash */

/* A white, click-through layer over the captured area that fades out in
 * ~150 ms. It appears only after the pixels are captured, so it never ends
 * up in an image. */
static HWND g_flash;
static ULONGLONG g_flash_t0;

static LRESULT CALLBACK flash_proc(HWND h, UINT msg, WPARAM wp, LPARAM lp)
{
    if (msg == WM_TIMER) {
        /* Fade by elapsed time: timer ticks are coarse (~16-31 ms). */
        ULONGLONG ms = GetTickCount64() - g_flash_t0;
        if (ms >= FLASH_MS) { DestroyWindow(h); return 0; }
        SetLayeredWindowAttributes(h, 0, (BYTE)(FLASH_ALPHA * (FLASH_MS - ms) / FLASH_MS),
                                   LWA_ALPHA);
        return 0;
    }
    if (msg == WM_DESTROY && h == g_flash) g_flash = NULL;
    return DefWindowProcW(h, msg, wp, lp);
}

/* Remove any flash still fading - ours, or one from another SnipSnap
 * process - and wait for the screen to show that, so it can't end up in the
 * next capture. */
static void flash_clear(void)
{
    HWND w = NULL;
    BOOL any = FALSE;
    if (g_flash) { DestroyWindow(g_flash); any = TRUE; }
    while ((w = FindWindowExW(NULL, w, FLASH_CLASS, NULL)) != NULL)
        if (IsWindowVisible(w)) { ShowWindow(w, SW_HIDE); any = TRUE; }
    if (any) DwmFlush();
}

static void flash(RECT r)
{
    static BOOL registered;
    BOOL on = TRUE;
    HWND h;
    if (!g_set[S_FLASH]) return;
    if (!registered) {
        WNDCLASSW wc;
        ZeroMemory(&wc, sizeof wc);
        wc.lpfnWndProc = flash_proc;
        wc.hInstance = GetModuleHandleW(NULL);
        wc.hbrBackground = (HBRUSH)GetStockObject(WHITE_BRUSH);
        wc.lpszClassName = FLASH_CLASS;
        RegisterClassW(&wc);
        registered = TRUE;
    }
    flash_clear();
    h = CreateWindowExW(WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST | WS_EX_TOOLWINDOW |
                        WS_EX_NOACTIVATE, FLASH_CLASS, NULL, WS_POPUP,
                        r.left, r.top, r.right - r.left, r.bottom - r.top,
                        NULL, NULL, GetModuleHandleW(NULL), NULL);
    if (!h) return;
    g_flash = h;
    DwmSetWindowAttribute(h, DWMWA_TRANSITIONS_FORCEDISABLED, &on, sizeof on);
    SetLayeredWindowAttributes(h, 0, FLASH_ALPHA, LWA_ALPHA);
    ShowWindow(h, SW_SHOWNOACTIVATE);
    UpdateWindow(h);
    g_flash_t0 = GetTickCount64();
    SetTimer(h, 1, USER_TIMER_MINIMUM, NULL);
}

/* One-shot runs: keep the process alive until the flash has faded. */
static void flash_wait(void)
{
    MSG m;
    while (g_flash && GetMessageW(&m, NULL, 0, 0) > 0) {
        TranslateMessage(&m);
        DispatchMessageW(&m);
    }
}

/* ---------------------------------------------------------------- pixels */

typedef struct {
    HDC dc;
    HBITMAP bmp, old;
    DWORD *px;          /* top-down 32bpp BGRX */
    int x, y, w, h;     /* screen position and size, physical pixels */
} Shot;

static RECT vscreen(void)
{
    RECT r;
    r.left = GetSystemMetrics(SM_XVIRTUALSCREEN);
    r.top = GetSystemMetrics(SM_YVIRTUALSCREEN);
    r.right = r.left + GetSystemMetrics(SM_CXVIRTUALSCREEN);
    r.bottom = r.top + GetSystemMetrics(SM_CYVIRTUALSCREEN);
    return r;
}

/* The monitor containing a point. */
static RECT monitor_at(POINT p)
{
    MONITORINFO mi;
    mi.cbSize = sizeof mi;
    if (!GetMonitorInfoW(MonitorFromPoint(p, MONITOR_DEFAULTTONEAREST), &mi)) return vscreen();
    return mi.rcMonitor;
}

/* The monitor under the mouse cursor. */
static RECT cursor_monitor(void)
{
    POINT p;
    GetCursorPos(&p);
    return monitor_at(p);
}

/* ---------------------------------------------------------- window picking
 *
 * The window list is taken when the screen is frozen, top of the z-order
 * first, so hovering always matches the frozen image. Each entry is the
 * window's visible frame (DWM extended frame bounds: no invisible resize
 * border or shadow). */

#define MAX_WINS 512
typedef struct { RECT r; BOOL desktop; } Win;
static Win g_wins[MAX_WINS];
static int g_nwins;

static BOOL CALLBACK collect_window(HWND w, LPARAM lp)
{
    RECT r;
    DWORD cloaked = 0, fl;
    LONG ex;
    BYTE a;
    wchar_t cls[16];
    (void)lp;
    if (g_nwins >= MAX_WINS || !IsWindowVisible(w) || IsIconic(w)) return TRUE;
    ex = GetWindowLongW(w, GWL_EXSTYLE);
    if (ex & WS_EX_TRANSPARENT) return TRUE;                  /* click-through overlays */
    if (SUCCEEDED(DwmGetWindowAttribute(w, DWMWA_CLOAKED, &cloaked, sizeof cloaked)) && cloaked)
        return TRUE;                                          /* other desktops, suspended apps */
    if ((ex & WS_EX_LAYERED) && GetLayeredWindowAttributes(w, NULL, &a, &fl) &&
        (fl & LWA_ALPHA) && a == 0)
        return TRUE;                                          /* fully transparent */
    if (FAILED(DwmGetWindowAttribute(w, DWMWA_EXTENDED_FRAME_BOUNDS, &r, sizeof r)) &&
        !GetWindowRect(w, &r))
        return TRUE;
    if (r.right <= r.left || r.bottom <= r.top) return TRUE;
    GetClassNameW(w, cls, ARRAYSIZE(cls));
    g_wins[g_nwins].r = r;
    g_wins[g_nwins].desktop = eq(cls, L"Progman") || eq(cls, L"WorkerW");
    g_nwins++;
    return TRUE;
}

static void collect_windows(void)
{
    g_nwins = 0;
    EnumWindows(collect_window, 0);
}

/* What to capture at a screen point: the topmost window there, or the whole
 * monitor over the bare desktop. Clipped to the screen. */
static RECT pick_at(POINT p)
{
    RECT r = monitor_at(p), v = vscreen(), out;
    for (int i = 0; i < g_nwins; i++)
        if (PtInRect(&g_wins[i].r, p)) {
            if (!g_wins[i].desktop) r = g_wins[i].r;
            break;
        }
    if (!IntersectRect(&out, &r, &v)) return monitor_at(p);
    return out;
}

static BOOL shot_alloc(Shot *s, int x, int y, int w, int h)
{
    BITMAPINFO bi;
    ZeroMemory(&bi, sizeof bi);
    bi.bmiHeader.biSize = sizeof bi.bmiHeader;
    bi.bmiHeader.biWidth = w;
    bi.bmiHeader.biHeight = -h;
    bi.bmiHeader.biPlanes = 1;
    bi.bmiHeader.biBitCount = 32;
    bi.bmiHeader.biCompression = BI_RGB;
    s->x = x; s->y = y; s->w = w; s->h = h;
    s->dc = CreateCompatibleDC(NULL);
    s->bmp = CreateDIBSection(NULL, &bi, DIB_RGB_COLORS, (void **)&s->px, NULL, 0);
    if (!s->dc || !s->bmp) return FALSE;
    s->old = (HBITMAP)SelectObject(s->dc, s->bmp);
    return TRUE;
}

static void shot_free(Shot *s)
{
    if (s->dc) { if (s->old) SelectObject(s->dc, s->old); DeleteDC(s->dc); }
    if (s->bmp) DeleteObject(s->bmp);
    ZeroMemory(s, sizeof *s);
}

/* Grab a screen rectangle (physical pixels). */
static BOOL grab(Shot *s, RECT v)
{
    HDC scr;
    BOOL ok;
    if (v.right <= v.left || v.bottom <= v.top) return FALSE;
    flash_clear();
    scr = GetDC(NULL);
    ok = scr && shot_alloc(s, v.left, v.top, v.right - v.left, v.bottom - v.top) &&
         BitBlt(s->dc, 0, 0, s->w, s->h, scr, v.left, v.top, SS_ROP);
    if (scr) ReleaseDC(NULL, scr);
    if (!ok) { shot_free(s); return FALSE; }
    GdiFlush();
    return TRUE;
}

static BOOL open_clipboard(HWND owner)
{
    for (int i = 0; i < 20; i++) {
        if (OpenClipboard(owner)) return TRUE;
        Sleep(10);
    }
    return FALSE;
}

/* Copy a sub-rectangle of a shot to the clipboard as a 24bpp bottom-up DIB. */
static BOOL to_clipboard(HWND owner, const Shot *s, int rx, int ry, int w, int h)
{
    DWORD stride = ((DWORD)w * 3 + 3) & ~3u;
    SIZE_T bytes = (SIZE_T)stride * h;
    BOOL ok = FALSE;
    HGLOBAL g;
    BITMAPINFOHEADER *bh;
    BYTE *dst;

    g = GlobalAlloc(GMEM_MOVEABLE, sizeof *bh + bytes);
    if (!g) return FALSE;
    bh = (BITMAPINFOHEADER *)GlobalLock(g);
    if (!bh) { GlobalFree(g); return FALSE; }
    ZeroMemory(bh, sizeof *bh);
    bh->biSize = sizeof *bh;
    bh->biWidth = w;
    bh->biHeight = h;                     /* positive = bottom-up */
    bh->biPlanes = 1;
    bh->biBitCount = 24;
    bh->biCompression = BI_RGB;
    bh->biSizeImage = (DWORD)bytes;

    dst = (BYTE *)(bh + 1);
    for (int y = 0; y < h; y++) {
        const DWORD *src = s->px + (SIZE_T)(ry + h - 1 - y) * s->w + rx;
        BYTE *d = dst + (SIZE_T)y * stride, *end = d + stride;
        for (int x = 0; x < w; x++) {
            DWORD c = src[x];
            d[0] = (BYTE)c; d[1] = (BYTE)(c >> 8); d[2] = (BYTE)(c >> 16);
            d += 3;
        }
        while (d < end) *d++ = 0;
    }
    GlobalUnlock(g);

    if (open_clipboard(owner)) {
        EmptyClipboard();
        ok = SetClipboardData(CF_DIB, g) != NULL;
        CloseClipboard();
    }
    if (!ok) GlobalFree(g);
    return ok;
}

/* ------------------------------------------------------------ full screen */

static int snip_area(HWND owner, RECT r)
{
    Shot s;
    BOOL ok;
    ZeroMemory(&s, sizeof s);
    if (!grab(&s, r)) return RC_FAIL;
    flash(r);
    ok = to_clipboard(owner, &s, 0, 0, s.w, s.h);
    if (g_daemon) shot_free(&s);       /* a one-shot run just exits */
    return ok ? RC_OK : RC_FAIL;
}

/* ------------------------------------------------- rectangle and window pick
 *
 * Both freeze all monitors and show the frozen image dimmed. Rectangle: drag
 * to select. Window: the window under the cursor shows at full brightness with
 * a border; click to take it. Either way the result is cropped from the
 * frozen image, so copying after mouse-up takes only a few ms. */

enum { MODE_RECT, MODE_WINDOW };

static Shot g_shot, g_dim;
static HWND g_owner;
static HBRUSH g_frame;
static BOOL g_drag, g_pressed, g_has_hover;
static RECT g_hover;                       /* hovered window, overlay coordinates */
static int g_mode, g_ax, g_ay, g_cx, g_cy, g_result = RC_CANCEL;

static void cloak(HWND h, BOOL on)
{
    DwmSetWindowAttribute(h, DWMWA_CLOAK, &on, sizeof on);
}

static RECT sel_rect(void)
{
    RECT r;
    r.left   = g_ax < g_cx ? g_ax : g_cx;
    r.top    = g_ay < g_cy ? g_ay : g_cy;
    r.right  = (g_ax > g_cx ? g_ax : g_cx) + 1;   /* inclusive of both corners */
    r.bottom = (g_ay > g_cy ? g_ay : g_cy) + 1;
    return r;
}

static void track(LPARAM lp)
{
    int x = GET_X_LPARAM(lp), y = GET_Y_LPARAM(lp);
    g_cx = x < 0 ? 0 : x >= g_shot.w ? g_shot.w - 1 : x;
    g_cy = y < 0 ? 0 : y >= g_shot.h ? g_shot.h - 1 : y;
}

/* Point the window pick at the window under an overlay point. */
static void hover_at(HWND h, int x, int y)
{
    POINT p;
    RECT r;
    p.x = x + g_shot.x; p.y = y + g_shot.y;
    r = pick_at(p);
    OffsetRect(&r, -g_shot.x, -g_shot.y);   /* screen -> overlay coordinates */
    if (g_has_hover && EqualRect(&r, &g_hover)) return;
    if (h && g_has_hover) InvalidateRect(h, &g_hover, FALSE);
    g_hover = r;
    g_has_hover = TRUE;
    if (h) InvalidateRect(h, &g_hover, FALSE);
}

/* Paint the update area: dimmed backdrop, plus an optional bright (undimmed)
 * area with a border - the selection or the hovered window. Every pixel is
 * written exactly once (dim outside, bright inside, border on its ring), so
 * the compositor can never show a half-painted, flickering state. */
static void paint_backdrop(HDC dc, RECT clip, const RECT *lit)
{
    RECT i, f, inner;
    int saved;
    if (!lit) {
        BitBlt(dc, clip.left, clip.top, clip.right - clip.left, clip.bottom - clip.top,
               g_dim.dc, clip.left, clip.top, SRCCOPY);
        return;
    }
    saved = SaveDC(dc);
    ExcludeClipRect(dc, lit->left, lit->top, lit->right, lit->bottom);
    BitBlt(dc, clip.left, clip.top, clip.right - clip.left, clip.bottom - clip.top,
           g_dim.dc, clip.left, clip.top, SRCCOPY);
    RestoreDC(dc, saved);
    inner = *lit;
    InflateRect(&inner, -FRAME_W, -FRAME_W);
    if (IntersectRect(&i, &inner, &clip))
        BitBlt(dc, i.left, i.top, i.right - i.left, i.bottom - i.top,
               g_shot.dc, i.left, i.top, SRCCOPY);
    f = *lit;
    for (int k = 0; k < FRAME_W && f.right > f.left && f.bottom > f.top; k++) {
        FrameRect(dc, &f, g_frame);
        InflateRect(&f, -1, -1);
    }
}

/* Copy an overlay-coordinate area of the frozen image, with the flash. */
static int take(HWND h, RECT r)
{
    RECT sr = r;
    cloak(h, TRUE);                         /* vanish first, copy after */
    OffsetRect(&sr, g_shot.x, g_shot.y);    /* overlay -> screen coordinates */
    flash(sr);
    return to_clipboard(g_owner, &g_shot, r.left, r.top, r.right - r.left, r.bottom - r.top)
           ? RC_OK : RC_FAIL;
}

/* End the session; cloaking makes the overlay vanish at once (~0.2 ms vs ~7
 * ms for a real hide), the slower cleanup happens after. */
static void finish(HWND h, int result)
{
    if (g_result >= 0) return;
    cloak(h, TRUE);
    g_result = result;
}

static LRESULT CALLBACK overlay_proc(HWND h, UINT msg, WPARAM wp, LPARAM lp)
{
    switch (msg) {
    case WM_ERASEBKGND:
        return 1;
    case WM_SETCURSOR:
        SetCursor(LoadCursorW(NULL, g_mode == MODE_WINDOW ? IDC_ARROW : IDC_CROSS));
        return TRUE;
    case WM_PAINT: {
        PAINTSTRUCT ps;
        HDC dc = BeginPaint(h, &ps);
        RECT r = ps.rcPaint, s = sel_rect();
        if (g_dim.dc)
            paint_backdrop(dc, r, g_mode == MODE_RECT ? (g_drag ? &s : NULL)
                                                      : (g_has_hover ? &g_hover : NULL));
        EndPaint(h, &ps);
        return 0;
    }
    case WM_LBUTTONDOWN:
        if (g_result >= 0) return 0;
        if (g_mode == MODE_WINDOW) {
            hover_at(h, GET_X_LPARAM(lp), GET_Y_LPARAM(lp));
            g_pressed = TRUE;
        } else {
            track(lp);
            g_ax = g_cx; g_ay = g_cy;
            g_drag = TRUE;
        }
        SetCapture(h);
        return 0;
    case WM_MOUSEMOVE:
        if (g_mode == MODE_WINDOW) {
            if (!g_pressed) hover_at(h, GET_X_LPARAM(lp), GET_Y_LPARAM(lp));
        } else if (g_drag) {
            RECT a = sel_rect(), b, u;
            track(lp);
            b = sel_rect();
            UnionRect(&u, &a, &b);
            InvalidateRect(h, &u, FALSE);
        }
        return 0;
    case WM_LBUTTONUP:
        if (g_mode == MODE_WINDOW && g_pressed) {
            /* Take the window on release, so the release doesn't land in the
             * app underneath once the overlay is gone. */
            g_pressed = FALSE;
            ReleaseCapture();
            if (g_has_hover) finish(h, take(h, g_hover));
        } else if (g_drag) {
            track(lp);
            g_drag = FALSE;
            ReleaseCapture();
            if (g_ax == g_cx && g_ay == g_cy) {     /* plain click: keep waiting */
                InvalidateRect(h, NULL, FALSE);
                return 0;
            }
            finish(h, take(h, sel_rect()));
        }
        return 0;
    case WM_RBUTTONDOWN:
        finish(h, RC_CANCEL);
        return 0;
    case WM_KEYDOWN:
        if (wp == VK_ESCAPE) finish(h, RC_CANCEL);
        return 0;
    case WM_HOTKEY:
        if (wp == HK_ESC) finish(h, RC_CANCEL);
        return 0;
    case WM_CAPTURECHANGED:
        if ((HWND)lp != h && (g_drag || g_pressed)) {
            g_drag = g_pressed = FALSE;
            InvalidateRect(h, NULL, FALSE);
        }
        return 0;
    case WM_ACTIVATE:
        if (LOWORD(wp) == WA_INACTIVE) finish(h, RC_CANCEL);
        return 0;
    case WM_CLOSE:
        finish(h, RC_CANCEL);
        return 0;
    }
    return DefWindowProcW(h, msg, wp, lp);
}

static HWND overlay_create(const Shot *s)
{
    static BOOL registered;
    BOOL on = TRUE;
    HWND h;
    if (!registered) {
        WNDCLASSW wc;
        ZeroMemory(&wc, sizeof wc);
        wc.lpfnWndProc = overlay_proc;
        wc.hInstance = GetModuleHandleW(NULL);
        wc.lpszClassName = OVERLAY_CLASS;
        RegisterClassW(&wc);
        g_frame = CreateSolidBrush(FRAME_COLOR);
        registered = TRUE;
    }
    h = CreateWindowExW(WS_EX_TOPMOST | WS_EX_TOOLWINDOW, OVERLAY_CLASS, L"SnipSnap", WS_POPUP,
                        s->x, s->y, s->w, s->h, NULL, NULL, GetModuleHandleW(NULL), NULL);
    if (h) DwmSetWindowAttribute(h, DWMWA_TRANSITIONS_FORCEDISABLED, &on, sizeof on);
    return h;
}

static int snip_select(HWND owner, int mode)
{
    HWND h;
    MSG m;
    POINT cur;
    int quit = 0, have_quit = 0;

    ZeroMemory(&g_shot, sizeof g_shot);
    ZeroMemory(&g_dim, sizeof g_dim);
    if (!grab(&g_shot, vscreen())) return RC_FAIL;   /* freeze all monitors */
    if (mode == MODE_WINDOW) collect_windows();     /* before our overlay exists */

    /* Dimmed copy of the frozen screen for the backdrop. */
    if (!shot_alloc(&g_dim, g_shot.x, g_shot.y, g_shot.w, g_shot.h)) {
        shot_free(&g_dim);
        shot_free(&g_shot);
        return RC_FAIL;
    }
    {
        const DWORD *src = g_shot.px, *end = src + (SIZE_T)g_shot.w * g_shot.h;
        DWORD *d = g_dim.px;
        while (src < end) *d++ = (*src++ >> 1) & 0x7F7F7F;
    }

    h = overlay_create(&g_shot);
    if (!h) { shot_free(&g_dim); shot_free(&g_shot); return RC_FAIL; }

    g_owner = owner;
    g_mode = mode;
    g_drag = g_pressed = g_has_hover = FALSE;
    g_result = -1;
    if (mode == MODE_WINDOW) {              /* light up the window under the cursor now */
        GetCursorPos(&cur);
        hover_at(NULL, cur.x - g_shot.x, cur.y - g_shot.y);
    }
    /* Paint while cloaked so the first visible frame is complete. Showing
     * without activation first, then taking focus, makes it visible ~20 ms
     * sooner than a plain SW_SHOW. */
    cloak(h, TRUE);
    ShowWindow(h, SW_SHOWNOACTIVATE);
    InvalidateRect(h, NULL, FALSE);
    UpdateWindow(h);
    cloak(h, FALSE);
    SetForegroundWindow(h);
    SetCursor(LoadCursorW(NULL, mode == MODE_WINDOW ? IDC_ARROW : IDC_CROSS));
    /* Esc works even if Windows refused to give the overlay keyboard focus. */
    RegisterHotKey(h, HK_ESC, MOD_NOREPEAT, VK_ESCAPE);

    while (g_result < 0) {
        BOOL r = GetMessageW(&m, NULL, 0, 0);
        if (r <= 0) { have_quit = 1; quit = (int)m.wParam; finish(h, RC_CANCEL); break; }
        TranslateMessage(&m);
        DispatchMessageW(&m);
    }
    UnregisterHotKey(h, HK_ESC);
    if (!g_daemon) return g_result;    /* a one-shot run just exits */

    DestroyWindow(h);                  /* the copy is done; nobody waits for this */
    shot_free(&g_dim);
    shot_free(&g_shot);
    if (have_quit) PostQuitMessage(quit);
    return g_result;
}

/* ----------------------------------------------------------------- daemon */

static BOOL g_busy;
static UINT g_reload;                   /* RELOAD_MSG */
static const char *g_key[S_HOTKEYS];    /* each capture's key name, NULL = none */
static char g_key_buf[S_HOTKEYS][96];

/* (Re)register the capture hotkeys: a saved one exactly, otherwise the first
 * of the fallbacks that registers. Returns bit i set for capture i without a
 * key, bit i + 4 for one on a fallback. */
static int register_keys(HWND h)
{
    int r = 0;
    for (int i = 0; i < S_HOTKEYS; i++) UnregisterHotKey(h, HK_RECT + i);
    for (int i = 0; i < S_HOTKEYS; i++) {
        const Hotkey *k = SETTINGS[i].keys;
        g_key[i] = NULL;
        if (g_set[i]) {
            if (try_key(h, HK_RECT + i, g_set[i])) g_key[i] = key_name(g_key_buf[i], g_set[i]);
        } else {
            for (; k < SETTINGS[i + 1].keys; k++)
                if (try_key(h, HK_RECT + i, k->v)) { g_key[i] = k->name; break; }
        }
        if (!g_key[i]) r |= 1 << i;
        else if (k != SETTINGS[i].keys) r |= 0x10 << i;
    }
    return r;
}

/* Re-read the settings and re-register the hotkeys: the reply to RELOAD_MSG,
 * RELOAD_DONE plus a bit for each capture left without a key. */
static int reload_keys(HWND h)
{
    settings_load();
    return RELOAD_DONE | (register_keys(h) & 7);
}

static LRESULT CALLBACK daemon_proc(HWND h, UINT msg, WPARAM wp, LPARAM lp)
{
    if (msg == g_reload) return reload_keys(h);   /* `snipsnap set` saved new settings */
    switch (msg) {
    case WM_HOTKEY:
        if (!g_busy) {
            g_busy = TRUE;
            if (wp == HK_RECT) snip_select(h, MODE_RECT);
            else if (wp == HK_WINDOW) snip_select(h, MODE_WINDOW);
            else if (wp == HK_FULL) snip_area(h, cursor_monitor());
            g_busy = FALSE;
        }
        return 0;
    case WM_CLOSE:
        DestroyWindow(h);
        return 0;
    case WM_DESTROY:
        PostQuitMessage(0);
        return 0;
    }
    return DefWindowProcW(h, msg, wp, lp);
}

static int run_daemon(void)
{
    HANDLE mtx = CreateMutexW(NULL, FALSE, MUTEX_NAME);
    WNDCLASSW wc;
    HWND h;
    MSG m;
    int r;

    if (!mtx || GetLastError() == ERROR_ALREADY_EXISTS) {
        say("SnipSnap is already running. Use 'snipsnap quit' to stop it.\n");
        return RC_OK;
    }
    g_daemon = TRUE;
    g_reload = RegisterWindowMessageW(RELOAD_MSG);

    ZeroMemory(&wc, sizeof wc);
    wc.lpfnWndProc = daemon_proc;
    wc.hInstance = GetModuleHandleW(NULL);
    wc.lpszClassName = DAEMON_CLASS;
    RegisterClassW(&wc);
    h = CreateWindowExW(0, DAEMON_CLASS, L"SnipSnap", 0, 0, 0, 0, 0,
                        HWND_MESSAGE, NULL, wc.hInstance, NULL);
    if (!h) { say("SnipSnap: could not create window.\n"); return RC_FAIL; }

    r = register_keys(h);
    if ((r & 7) == 7) {
        const char *msg = "SnipSnap: none of the hotkeys could be registered "
                          "(another program owns them).\n"
                          "Use 'snipsnap rect' / 'window' / 'full' from a launcher instead.\n";
        say(msg);
        if (g_out == INVALID_HANDLE_VALUE)
            MessageBoxA(NULL, msg, "SnipSnap", MB_ICONERROR);
        return RC_FAIL;
    }

    say("SnipSnap running in the background.\n");
    for (int i = 0; i < S_HOTKEYS; i++) {   /* "  Rectangle:   Ctrl+Alt+R" */
        char line[32];
        int n = wsprintfA(line, "  %s:", SETTINGS[i].label);
        while (n < 15) line[n++] = ' ';
        line[n] = 0;
        say(line);
        say(g_key[i] ? g_key[i] : "(unavailable - hotkeys taken)");
        say("\n");
    }
    say("  Stop with:   snipsnap quit\n");
    /* No console to print to: speak up only if a capture didn't get its first
     * choice (its saved hotkey, or its first fallback). A saved key's name is
     * UTF-8, so a non-ASCII one would show garbled here; converting it for
     * MessageBoxW costs the exe a 512-byte section step. */
    if (g_out == INVALID_HANDLE_VALUE && r) {
        char msg[400];
        wsprintfA(msg, "Some default hotkeys are taken by another program.\n\n"
                       "Rectangle:\t%s\nWindow:\t\t%s\nFull screen:\t%s",
                  g_key[0] ? g_key[0] : "(unavailable)", g_key[1] ? g_key[1] : "(unavailable)",
                  g_key[2] ? g_key[2] : "(unavailable)");
        MessageBoxA(NULL, msg, "SnipSnap", MB_ICONWARNING);
    }
    detach_output();

    while (GetMessageW(&m, NULL, 0, 0) > 0) {
        TranslateMessage(&m);
        DispatchMessageW(&m);
    }
    CloseHandle(mtx);
    return RC_OK;
}

static int quit_daemon(void)
{
    HWND h = FindWindowExW(HWND_MESSAGE, NULL, DAEMON_CLASS, NULL);
    DWORD pid = 0;
    HANDLE proc;
    if (!h) { say("SnipSnap is not running.\n"); return RC_OK; }
    GetWindowThreadProcessId(h, &pid);
    proc = OpenProcess(SYNCHRONIZE, FALSE, pid);
    PostMessageW(h, WM_CLOSE, 0, 0);
    if (proc) { WaitForSingleObject(proc, 3000); CloseHandle(proc); }
    say("SnipSnap stopped.\n");
    return RC_OK;
}

/* --------------------------------------------- settings and set commands
 *
 * The PocketKnives blade settings contract (docs/specs/blade-settings.md at
 * the repo root): `settings` prints one tab-separated line per setting; `set`
 * checks every key=value, saves, and has the background instance apply it. */

static int print_settings(void)
{
    char line[96];
    for (int i = 0; i < S_COUNT; i++) {
        wsprintfA(line, "%ls\t%s\t%u\t%s\n", SETTINGS[i].key, i < S_HOTKEYS ? "hotkey" : "toggle",
                  i < S_HOTKEYS ? hotkey_of(i) : g_set[i], SETTINGS[i].label);
        say(line);
    }
    return RC_OK;
}

/* Refuse with one line naming a hotkey; exit code 2. */
static __declspec(noinline) int refuse(const char *fmt, UINT v)
{
    char name[96], msg[160];
    wsprintfA(msg, fmt, key_name(name, v));
    say(msg);
    return RC_FAIL;
}

static const wchar_t *next_arg(const wchar_t *p, wchar_t *out, int cap);

/* Ask the running copy to reload; its reply, or 0 if it didn't answer. */
static DWORD_PTR reload(HWND d)
{
    DWORD_PTR r;
    if (!SendMessageTimeoutW(d, RegisterWindowMessageW(RELOAD_MSG), 0, 0, SMTO_ABORTIFHUNG, 5000,
                             &r))
        r = 0;
    return r;
}

static int set_settings(const wchar_t *p)
{
    UINT v[S_COUNT], old[S_COUNT], n;
    wchar_t tok[32];
    const wchar_t *q;
    int mask = 0, bad = 0, i, j;
    BOOL ok;
    HWND d;

    for (i = 0; i < S_COUNT; i++) v[i] = i < S_HOTKEYS ? hotkey_of(i) : g_set[i];
    for (;;) {                          /* check every key=value first */
        p = next_arg(p, tok, ARRAYSIZE(tok));
        if (!tok[0]) break;
        for (i = 0; i < S_COUNT; i++) {
            const wchar_t *k = SETTINGS[i].key;
            for (q = tok; *k && *k == *q; k++, q++) ;
            if (!*k && *q == L'=') break;
        }
        n = 0;
        ok = i < S_COUNT && *++q;
        for (; ok && *q; q++) {
            ok = *q >= L'0' && *q <= L'9' && n <= HK_MAX;
            n = n * 10 + (UINT)(*q - L'0');
        }
        if (!ok || !(i < S_HOTKEYS ? hotkey_ok(n) : n <= 1)) {
            char msg[80];
            wsprintfA(msg, "Not a valid setting: %ls\n", tok);
            say(msg);
            return RC_FAIL;
        }
        v[i] = n;
        mask |= 1 << i;
    }
    for (i = 0; i < S_HOTKEYS; i++)
        for (j = i + 1; j < S_HOTKEYS; j++)
            if (v[i] == v[j] && (mask >> i | mask >> j) & 1)
                return refuse("%s is already used by SnipSnap.\n", v[i]);

    /* Without a running copy (or in a preview, which never asks it), check the
     * new hotkeys first by registering them for a moment, as it would. */
    d = g_preview ? NULL : FindWindowExW(HWND_MESSAGE, NULL, DAEMON_CLASS, NULL);
    if (!d) {
        for (i = 0; i < S_HOTKEYS; i++)
            if (mask >> i & 1) g_set[i] = v[i];
        bad = register_keys(NULL) & mask & 7;
    }
    if (!bad) {
        if (g_ini_dir) {                /* create the settings folder */
            g_ini[g_ini_dir] = 0;
            CreateDirectoryW(g_ini, NULL);
            g_ini[g_ini_dir] = L'\\';
        }
        for (i = 0; i < S_COUNT; i++)
            if (mask >> i & 1) {
                old[i] = ini_get(i);
                if (!ini_put(i, v[i])) bad = 8;
            }
    }
    if (d && !bad) {                    /* the running copy applies them */
        DWORD_PTR r = reload(d);
        bad = r & RELOAD_DONE ? (int)r & mask : 8;
        if (bad) {                      /* put the previous settings back */
            for (i = 0; i < S_COUNT; i++)
                if (mask >> i & 1) ini_put(i, old[i]);
            if (bad < 8) reload(d);
        }
    }
    if (bad == 8) {
        say("Couldn't apply the settings.\n");
        return RC_FAIL;
    }
    for (i = 0; i < S_HOTKEYS; i++)
        if (bad >> i & 1) return refuse("%s is taken by another program.\n", v[i]);
    return RC_OK;
}

/* ------------------------------------------------------------------- main */

/* The next command-line argument from p, lowercased, without leading '-' or
 * '/', into out; empty when there are no more. Parsed by hand to avoid loading
 * shell32. A too-long argument ends in '#', so it matches nothing. Returns
 * where it stopped. */
static const wchar_t *next_arg(const wchar_t *p, wchar_t *out, int cap)
{
    int n = 0;
    while (*p == L' ' || *p == L'\t') p++;
    while (*p == L'-' || *p == L'/') p++;
    while (*p > L' ') {
        wchar_t c = *p++;
        if (n == cap - 1) { out[n - 1] = L'#'; continue; }
        out[n++] = (c >= L'A' && c <= L'Z') ? (wchar_t)(c + 32) : c;
    }
    out[n] = 0;
    return p;
}

static int run(void)
{
    wchar_t arg[16];
    const wchar_t *p = GetCommandLineW();

    /* Must come first: everything below works in physical pixels. */
    if (!SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2))
        SetProcessDPIAware();

    if (*p == L'"') { p++; while (*p && *p != L'"') p++; if (*p) p++; }   /* the program */
    else while (*p > L' ') p++;
    p = next_arg(p, arg, ARRAYSIZE(arg));
    settings_load();
    if (!arg[0]) return run_daemon();
    if (eq(arg, L"rect") || eq(arg, L"window") || eq(arg, L"full")) {
        /* A message-only window to own the clipboard data. */
        HWND owner = CreateWindowExW(0, L"STATIC", NULL, 0, 0, 0, 0, 0,
                                     HWND_MESSAGE, NULL, NULL, NULL);
        int rc = arg[0] == L'r' ? snip_select(owner, MODE_RECT)
               : arg[0] == L'w' ? snip_select(owner, MODE_WINDOW)
               : snip_area(owner, cursor_monitor());
        flash_wait();                  /* the image is already on the clipboard */
        return rc;
    }
    if (eq(arg, L"quit") || eq(arg, L"stop")) return quit_daemon();
    if (eq(arg, L"settings")) return print_settings();
    if (eq(arg, L"set")) return set_settings(p);

    say("SnipSnap - copy a rectangle, a window or the screen to the clipboard.\n\n"
        "  snipsnap          run in the background (hotkeys below)\n"
        "  snipsnap rect     select a rectangle once (Esc / right-click cancels)\n"
        "  snipsnap window   pick a window once (click it; Esc / right-click cancels)\n"
        "  snipsnap full     capture the monitor under the cursor once\n"
        "  snipsnap quit     stop the background instance\n\n"
        "Hotkeys: Ctrl+Alt+R = rectangle, Ctrl+Alt+W = window, Ctrl+Alt+F = full screen\n"
        "         (if taken: Ctrl+PrtSc / Ctrl+Alt+Shift+W / Ctrl+Shift+PrtSc, then\n"
        "          Ctrl+Alt+Shift+R / - / Ctrl+Alt+Shift+F)\n");
    return (eq(arg, L"help") || eq(arg, L"h") || eq(arg, L"?")) ? RC_OK : RC_FAIL;
}

#ifdef SS_NOCRT
#include <intrin.h>
/* CRT-free build: tiny exe, no CRT startup. */
#pragma function(memset)
void *__cdecl memset(void *d, int c, size_t n)
{
    __stosb((unsigned char *)d, (unsigned char)c, n);
    return d;
}
void __stdcall WinMainCRTStartup(void) { ExitProcess((UINT)run()); }
#else
int WINAPI wWinMain(HINSTANCE hi, HINSTANCE hp, LPWSTR cmd, int show)
{
    (void)hi; (void)hp; (void)cmd; (void)show;
    return run();
}
#endif
