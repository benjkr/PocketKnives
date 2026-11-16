/*
 * SnipSnap - a tiny, fast screen snippet tool for Windows.
 *
 * Two features only: capture a rectangle, or capture the whole (virtual)
 * screen. The result goes to the clipboard as a 24-bit CF_DIB. Nothing else.
 *
 *   snipsnap            run in the background and listen for hotkeys
 *   snipsnap rect       select a rectangle once, then exit
 *   snipsnap full       capture the whole screen once, then exit
 *   snipsnap quit       stop the background instance
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
enum { HK_RECT = 1, HK_FULL = 2, HK_ESC = 3 };

#define DAEMON_CLASS  L"SnipSnapDaemon"
#define OVERLAY_CLASS L"SnipSnapOverlay"
#define MUTEX_NAME    L"Local\\SnipSnap.Daemon"
#define FRAME_COLOR   RGB(0, 120, 215)
#ifndef SS_ROP
#define SS_ROP        (SRCCOPY | CAPTUREBLT)   /* CAPTUREBLT: include layered windows */
#endif

/* Hotkey candidates, tried in order; the first one that registers wins. */
typedef struct { UINT mod, vk; const char *name; } Hotkey;
static const Hotkey RECT_KEYS[] = {
    { MOD_CONTROL,                       VK_SNAPSHOT, "Ctrl+PrtSc" },
    { MOD_CONTROL | MOD_ALT | MOD_SHIFT, 'S',         "Ctrl+Alt+Shift+S" },
};
static const Hotkey FULL_KEYS[] = {
    { MOD_CONTROL | MOD_SHIFT,           VK_SNAPSHOT, "Ctrl+Shift+PrtSc" },
    { MOD_CONTROL | MOD_ALT | MOD_SHIFT, 'F',         "Ctrl+Alt+Shift+F" },
};

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

/* --------------------------------------------------------------- capture */

typedef struct {
    HDC dc;
    HBITMAP bmp, old;
    DWORD *px;          /* top-down 32bpp BGRX */
    int x, y, w, h;     /* virtual-screen origin and size, physical pixels */
} Shot;

static BOOL shot_alloc(Shot *s, HDC ref, int w, int h)
{
    BITMAPINFO bi;
    ZeroMemory(&bi, sizeof bi);
    bi.bmiHeader.biSize = sizeof bi.bmiHeader;
    bi.bmiHeader.biWidth = w;
    bi.bmiHeader.biHeight = -h;
    bi.bmiHeader.biPlanes = 1;
    bi.bmiHeader.biBitCount = 32;
    bi.bmiHeader.biCompression = BI_RGB;
    s->w = w; s->h = h;
    s->dc = CreateCompatibleDC(ref);
    s->bmp = CreateDIBSection(ref, &bi, DIB_RGB_COLORS, (void **)&s->px, NULL, 0);
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

/* Grab the whole virtual desktop (all monitors). */
static BOOL grab(Shot *s)
{
    int x = GetSystemMetrics(SM_XVIRTUALSCREEN), y = GetSystemMetrics(SM_YVIRTUALSCREEN);
    int w = GetSystemMetrics(SM_CXVIRTUALSCREEN), h = GetSystemMetrics(SM_CYVIRTUALSCREEN);
    HDC scr = GetDC(NULL);
    BOOL ok = scr && w > 0 && h > 0 && shot_alloc(s, scr, w, h) &&
              BitBlt(s->dc, 0, 0, w, h, scr, x, y, SS_ROP);
    if (scr) ReleaseDC(NULL, scr);
    s->x = x; s->y = y;
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
static BOOL to_clipboard(HWND owner, const Shot *s, int rx, int ry, int rw, int rh)
{
    DWORD stride = ((DWORD)rw * 3 + 3) & ~3u;
    SIZE_T bytes = (SIZE_T)stride * rh;
    HGLOBAL g = GlobalAlloc(GMEM_MOVEABLE, sizeof(BITMAPINFOHEADER) + bytes);
    BITMAPINFOHEADER *bh;
    BYTE *dst;
    BOOL ok = FALSE;

    if (!g) return FALSE;
    bh = (BITMAPINFOHEADER *)GlobalLock(g);
    if (!bh) { GlobalFree(g); return FALSE; }
    ZeroMemory(bh, sizeof *bh);
    bh->biSize = sizeof *bh;
    bh->biWidth = rw;
    bh->biHeight = rh;                    /* positive = bottom-up */
    bh->biPlanes = 1;
    bh->biBitCount = 24;
    bh->biCompression = BI_RGB;
    bh->biSizeImage = (DWORD)bytes;

    dst = (BYTE *)(bh + 1);
    for (int y = 0; y < rh; y++) {
        const DWORD *src = s->px + (SIZE_T)(ry + rh - 1 - y) * s->w + rx;
        BYTE *d = dst + (SIZE_T)y * stride, *end = d + stride;
        for (int x = 0; x < rw; x++) {
            DWORD p = src[x];
            d[0] = (BYTE)p; d[1] = (BYTE)(p >> 8); d[2] = (BYTE)(p >> 16);
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

static int snip_full(HWND owner)
{
    Shot s;
    BOOL ok;
    ZeroMemory(&s, sizeof s);
    if (!grab(&s)) return RC_FAIL;
    ok = to_clipboard(owner, &s, 0, 0, s.w, s.h);
    shot_free(&s);
    return ok ? RC_OK : RC_FAIL;
}

/* -------------------------------------------------------------- rectangle */

static Shot g_shot, g_dim;
static HWND g_owner;
static HBRUSH g_frame;
static BOOL g_drag;
static int g_ax, g_ay, g_cx, g_cy, g_result;

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

static void finish(HWND h, int result)
{
    if (g_result < 0) g_result = result;
    ShowWindow(h, SW_HIDE);
}

static LRESULT CALLBACK overlay_proc(HWND h, UINT msg, WPARAM wp, LPARAM lp)
{
    switch (msg) {
    case WM_ERASEBKGND:
        return 1;
    case WM_PAINT: {
        PAINTSTRUCT ps;
        HDC dc = BeginPaint(h, &ps);
        RECT r = ps.rcPaint, s, i;
        BitBlt(dc, r.left, r.top, r.right - r.left, r.bottom - r.top,
               g_dim.dc, r.left, r.top, SRCCOPY);
        if (g_drag) {
            s = sel_rect();
            if (IntersectRect(&i, &s, &r))
                BitBlt(dc, i.left, i.top, i.right - i.left, i.bottom - i.top,
                       g_shot.dc, i.left, i.top, SRCCOPY);
            FrameRect(dc, &s, g_frame);
        }
        EndPaint(h, &ps);
        return 0;
    }
    case WM_LBUTTONDOWN:
        track(lp);
        g_ax = g_cx; g_ay = g_cy;
        g_drag = TRUE;
        SetCapture(h);
        return 0;
    case WM_MOUSEMOVE:
        if (g_drag) {
            RECT a = sel_rect(), b, u;
            track(lp);
            b = sel_rect();
            UnionRect(&u, &a, &b);
            InvalidateRect(h, &u, FALSE);
        }
        return 0;
    case WM_LBUTTONUP:
        if (g_drag) {
            RECT r;
            track(lp);
            g_drag = FALSE;
            ReleaseCapture();
            if (g_ax == g_cx && g_ay == g_cy) {     /* plain click: keep waiting */
                InvalidateRect(h, NULL, FALSE);
                return 0;
            }
            r = sel_rect();
            ShowWindow(h, SW_HIDE);                 /* vanish first, copy after */
            finish(h, to_clipboard(g_owner, &g_shot, r.left, r.top,
                                   r.right - r.left, r.bottom - r.top) ? RC_OK : RC_FAIL);
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
        if (g_drag && (HWND)lp != h) { g_drag = FALSE; InvalidateRect(h, NULL, FALSE); }
        return 0;
    case WM_ACTIVATE:
        if (LOWORD(wp) == WA_INACTIVE && IsWindowVisible(h)) finish(h, RC_CANCEL);
        return 0;
    case WM_CLOSE:
        finish(h, RC_CANCEL);
        return 0;
    }
    return DefWindowProcW(h, msg, wp, lp);
}

static int snip_rect(HWND owner)
{
    static BOOL registered;
    HWND h;
    MSG m;
    BOOL on = TRUE;
    int quit = 0, have_quit = 0;

    ZeroMemory(&g_shot, sizeof g_shot);
    ZeroMemory(&g_dim, sizeof g_dim);
    if (!grab(&g_shot)) return RC_FAIL;

    /* Dimmed copy of the frozen screen for the backdrop. */
    if (!shot_alloc(&g_dim, g_shot.dc, g_shot.w, g_shot.h)) {
        shot_free(&g_shot);
        return RC_FAIL;
    }
    {
        const DWORD *src = g_shot.px, *end = src + (SIZE_T)g_shot.w * g_shot.h;
        DWORD *d = g_dim.px;
        while (src < end) *d++ = (*src++ >> 1) & 0x7F7F7F;
    }

    if (!registered) {
        WNDCLASSW wc;
        ZeroMemory(&wc, sizeof wc);
        wc.lpfnWndProc = overlay_proc;
        wc.hInstance = GetModuleHandleW(NULL);
        wc.hCursor = LoadCursorW(NULL, IDC_CROSS);
        wc.lpszClassName = OVERLAY_CLASS;
        RegisterClassW(&wc);
        g_frame = CreateSolidBrush(FRAME_COLOR);
        registered = TRUE;
    }

    g_owner = owner;
    g_drag = FALSE;
    g_result = -1;
    h = CreateWindowExW(WS_EX_TOPMOST | WS_EX_TOOLWINDOW, OVERLAY_CLASS, L"SnipSnap",
                        WS_POPUP, g_shot.x, g_shot.y, g_shot.w, g_shot.h,
                        NULL, NULL, GetModuleHandleW(NULL), NULL);
    if (!h) { shot_free(&g_dim); shot_free(&g_shot); return RC_FAIL; }

    /* Appear instantly: no fade-in, and paint while cloaked so no blank frame. */
    DwmSetWindowAttribute(h, DWMWA_TRANSITIONS_FORCEDISABLED, &on, sizeof on);
    DwmSetWindowAttribute(h, DWMWA_CLOAK, &on, sizeof on);
    ShowWindow(h, SW_SHOW);
    UpdateWindow(h);
    on = FALSE;
    DwmSetWindowAttribute(h, DWMWA_CLOAK, &on, sizeof on);
    SetForegroundWindow(h);
    SetCursor(LoadCursorW(NULL, IDC_CROSS));
    /* Esc works even if Windows refused to give the overlay keyboard focus. */
    RegisterHotKey(h, HK_ESC, MOD_NOREPEAT, VK_ESCAPE);

    while (g_result < 0) {
        BOOL r = GetMessageW(&m, NULL, 0, 0);
        if (r <= 0) { have_quit = 1; quit = (int)m.wParam; g_result = RC_CANCEL; break; }
        TranslateMessage(&m);
        DispatchMessageW(&m);
    }

    UnregisterHotKey(h, HK_ESC);
    DestroyWindow(h);
    shot_free(&g_dim);
    shot_free(&g_shot);
    if (have_quit) PostQuitMessage(quit);
    return g_result;
}

/* ----------------------------------------------------------------- daemon */

static BOOL g_busy;

static LRESULT CALLBACK daemon_proc(HWND h, UINT msg, WPARAM wp, LPARAM lp)
{
    switch (msg) {
    case WM_HOTKEY:
        if (!g_busy) {
            g_busy = TRUE;
            if (wp == HK_RECT) snip_rect(h);
            else if (wp == HK_FULL) snip_full(h);
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

static const Hotkey *register_one(HWND h, int id, const Hotkey *list, int n)
{
    for (int i = 0; i < n; i++)
        if (RegisterHotKey(h, id, list[i].mod | MOD_NOREPEAT, list[i].vk))
            return &list[i];
    return NULL;
}

static int run_daemon(void)
{
    HANDLE mtx = CreateMutexW(NULL, FALSE, MUTEX_NAME);
    WNDCLASSW wc;
    HWND h;
    MSG m;
    const Hotkey *kr, *kf;

    if (!mtx || GetLastError() == ERROR_ALREADY_EXISTS) {
        say("SnipSnap is already running. Use 'snipsnap quit' to stop it.\n");
        return RC_OK;
    }

    ZeroMemory(&wc, sizeof wc);
    wc.lpfnWndProc = daemon_proc;
    wc.hInstance = GetModuleHandleW(NULL);
    wc.lpszClassName = DAEMON_CLASS;
    RegisterClassW(&wc);
    h = CreateWindowExW(0, DAEMON_CLASS, L"SnipSnap", 0, 0, 0, 0, 0,
                        HWND_MESSAGE, NULL, wc.hInstance, NULL);
    if (!h) { say("SnipSnap: could not create window.\n"); return RC_FAIL; }

    kr = register_one(h, HK_RECT, RECT_KEYS, ARRAYSIZE(RECT_KEYS));
    kf = register_one(h, HK_FULL, FULL_KEYS, ARRAYSIZE(FULL_KEYS));

    if (!kr && !kf) {
        const char *msg = "SnipSnap: none of the hotkeys could be registered "
                          "(another program owns them).\n"
                          "Use 'snipsnap rect' / 'snipsnap full' from a launcher instead.\n";
        say(msg);
        if (g_out == INVALID_HANDLE_VALUE)
            MessageBoxA(NULL, msg, "SnipSnap", MB_ICONERROR);
        return RC_FAIL;
    }

    say("SnipSnap running in the background.\n  Rectangle:   ");
    say(kr ? kr->name : "(unavailable - hotkeys taken)");
    say("\n  Full screen: ");
    say(kf ? kf->name : "(unavailable - hotkeys taken)");
    say("\n  Stop with:   snipsnap quit\n");
    if ((!kr || !kf) && g_out == INVALID_HANDLE_VALUE)
        MessageBoxA(NULL, kr ? "Full-screen hotkey could not be registered."
                             : "Rectangle hotkey could not be registered.",
                    "SnipSnap", MB_ICONWARNING);
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

/* ------------------------------------------------------------------- main */

/* First command-line argument (after the program name), lowercased, no
 * leading '-' or '/'. Parsed by hand to avoid loading shell32. */
static void first_arg(wchar_t *out, int cap)
{
    const wchar_t *p = GetCommandLineW();
    int n = 0;
    if (*p == L'"') { p++; while (*p && *p != L'"') p++; if (*p) p++; }
    else while (*p > L' ') p++;
    while (*p == L' ' || *p == L'\t') p++;
    while (*p == L'-' || *p == L'/') p++;
    while (*p > L' ' && n < cap - 1) {
        wchar_t c = *p++;
        out[n++] = (c >= L'A' && c <= L'Z') ? (wchar_t)(c + 32) : c;
    }
    out[n] = 0;
}

static BOOL eq(const wchar_t *a, const wchar_t *b)
{
    while (*a && *a == *b) { a++; b++; }
    return *a == *b;
}

static int run(void)
{
    wchar_t arg[16];

    /* Must come first: everything below works in physical pixels. */
    if (!SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2))
        SetProcessDPIAware();

    first_arg(arg, ARRAYSIZE(arg));
    if (!arg[0]) return run_daemon();
    if (eq(arg, L"rect") || eq(arg, L"full")) {
        /* A message-only window to own the clipboard data. */
        HWND owner = CreateWindowExW(0, L"STATIC", NULL, 0, 0, 0, 0, 0,
                                     HWND_MESSAGE, NULL, NULL, NULL);
        int rc = arg[0] == L'r' ? snip_rect(owner) : snip_full(owner);
        if (owner) DestroyWindow(owner);
        return rc;
    }
    if (eq(arg, L"quit") || eq(arg, L"stop")) return quit_daemon();

    say("SnipSnap - copy a screen rectangle or the whole screen to the clipboard.\n\n"
        "  snipsnap         run in the background (hotkeys below)\n"
        "  snipsnap rect    select a rectangle once (Esc / right-click cancels)\n"
        "  snipsnap full    capture the whole screen once\n"
        "  snipsnap quit    stop the background instance\n\n"
        "Hotkeys: Ctrl+PrtSc = rectangle, Ctrl+Shift+PrtSc = full screen\n"
        "         (fallbacks: Ctrl+Alt+Shift+S / Ctrl+Alt+Shift+F)\n");
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
