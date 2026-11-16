/*
 * SnipSnap - a tiny, fast screen snippet tool for Windows.
 *
 * Three captures: a dragged rectangle, the active window, or the monitor
 * under the cursor. The result goes to the clipboard as a 24-bit CF_DIB, and
 * the captured area flashes briefly. Nothing else.
 *
 *   snipsnap            run in the background and listen for hotkeys
 *   snipsnap rect       select a rectangle once, then exit
 *   snipsnap window     capture the active window once, then exit
 *   snipsnap full       capture the monitor under the cursor once, then exit
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
enum { HK_RECT = 1, HK_WINDOW = 2, HK_FULL = 3, HK_ESC = 4 };

#define DAEMON_CLASS  L"SnipSnapDaemon"
#define OVERLAY_CLASS L"SnipSnapOverlay"
#define FLASH_CLASS   L"SnipSnapFlash"
#define MUTEX_NAME    L"Local\\SnipSnap.Daemon"
#define FRAME_COLOR   RGB(0, 120, 215)
#define FLASH_ALPHA   150               /* starting opacity of the white flash */
#define FLASH_MS      150               /* fade-out duration */
#ifndef SS_ROP
#define SS_ROP        (SRCCOPY | CAPTUREBLT)   /* CAPTUREBLT: include layered windows */
#endif

/* Hotkey candidates, tried in order; the first one that registers wins. */
typedef struct { UINT mod, vk; const char *name; } Hotkey;
static const Hotkey RECT_KEYS[] = {
    { MOD_CONTROL | MOD_ALT,             'R',         "Ctrl+Alt+R" },
    { MOD_CONTROL,                       VK_SNAPSHOT, "Ctrl+PrtSc" },
    { MOD_CONTROL | MOD_ALT | MOD_SHIFT, 'R',         "Ctrl+Alt+Shift+R" },
};
static const Hotkey WINDOW_KEYS[] = {
    { MOD_CONTROL | MOD_ALT,             'W',         "Ctrl+Alt+W" },
    { MOD_CONTROL | MOD_ALT | MOD_SHIFT, 'W',         "Ctrl+Alt+Shift+W" },
};
static const Hotkey FULL_KEYS[] = {
    { MOD_CONTROL | MOD_ALT,             'F',         "Ctrl+Alt+F" },
    { MOD_CONTROL | MOD_SHIFT,           VK_SNAPSHOT, "Ctrl+Shift+PrtSc" },
    { MOD_CONTROL | MOD_ALT | MOD_SHIFT, 'F',         "Ctrl+Alt+Shift+F" },
};

static BOOL g_daemon;   /* running as the background instance */

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

/* The monitor under the mouse cursor. */
static RECT cursor_monitor(void)
{
    POINT p;
    MONITORINFO mi;
    mi.cbSize = sizeof mi;
    GetCursorPos(&p);
    if (!GetMonitorInfoW(MonitorFromPoint(p, MONITOR_DEFAULTTONEAREST), &mi)) return vscreen();
    return mi.rcMonitor;
}

/* The visible frame of the active window (no invisible resize borders or
 * shadow), clipped to the screen. Falls back to the cursor's monitor. */
static RECT active_window(void)
{
    HWND w = GetForegroundWindow();
    RECT r, v = vscreen(), out;
    if (!w || IsIconic(w) ||
        (FAILED(DwmGetWindowAttribute(w, DWMWA_EXTENDED_FRAME_BOUNDS, &r, sizeof r)) &&
         !GetWindowRect(w, &r)) ||
        !IntersectRect(&out, &r, &v))
        return cursor_monitor();
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

/* -------------------------------------------------- window and full screen */

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

/* -------------------------------------------------------------- rectangle */

static Shot g_shot, g_dim;
static HWND g_owner;
static HBRUSH g_frame;
static BOOL g_drag;
static int g_ax, g_ay, g_cx, g_cy, g_result = RC_CANCEL;

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
    case WM_PAINT: {
        PAINTSTRUCT ps;
        HDC dc = BeginPaint(h, &ps);
        RECT r = ps.rcPaint, s, i;
        if (g_dim.dc) {
            BitBlt(dc, r.left, r.top, r.right - r.left, r.bottom - r.top,
                   g_dim.dc, r.left, r.top, SRCCOPY);
            if (g_drag) {
                s = sel_rect();
                if (IntersectRect(&i, &s, &r))
                    BitBlt(dc, i.left, i.top, i.right - i.left, i.bottom - i.top,
                           g_shot.dc, i.left, i.top, SRCCOPY);
                FrameRect(dc, &s, g_frame);
            }
        }
        EndPaint(h, &ps);
        return 0;
    }
    case WM_LBUTTONDOWN:
        if (g_result >= 0) return 0;
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
            RECT r, sr;
            track(lp);
            g_drag = FALSE;
            ReleaseCapture();
            if (g_ax == g_cx && g_ay == g_cy) {     /* plain click: keep waiting */
                InvalidateRect(h, NULL, FALSE);
                return 0;
            }
            r = sel_rect();
            cloak(h, TRUE);                         /* vanish first, copy after */
            sr = r;
            OffsetRect(&sr, g_shot.x, g_shot.y);    /* overlay -> screen coordinates */
            flash(sr);
            finish(h, to_clipboard(g_owner, &g_shot, r.left, r.top, r.right - r.left,
                                   r.bottom - r.top) ? RC_OK : RC_FAIL);
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
        wc.hCursor = LoadCursorW(NULL, IDC_CROSS);
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

static int snip_rect(HWND owner)
{
    HWND h;
    MSG m;
    int quit = 0, have_quit = 0;

    ZeroMemory(&g_shot, sizeof g_shot);
    ZeroMemory(&g_dim, sizeof g_dim);
    if (!grab(&g_shot, vscreen())) return RC_FAIL;   /* freeze all monitors */

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
    g_drag = FALSE;
    g_result = -1;
    /* Paint while cloaked so the first visible frame is complete. Showing
     * without activation first, then taking focus, makes it visible ~20 ms
     * sooner than a plain SW_SHOW. */
    cloak(h, TRUE);
    ShowWindow(h, SW_SHOWNOACTIVATE);
    InvalidateRect(h, NULL, FALSE);
    UpdateWindow(h);
    cloak(h, FALSE);
    SetForegroundWindow(h);
    SetCursor(LoadCursorW(NULL, IDC_CROSS));
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

static LRESULT CALLBACK daemon_proc(HWND h, UINT msg, WPARAM wp, LPARAM lp)
{
    switch (msg) {
    case WM_HOTKEY:
        if (!g_busy) {
            g_busy = TRUE;
            if (wp == HK_RECT) snip_rect(h);
            else if (wp == HK_WINDOW) snip_area(h, active_window());
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
    const Hotkey *kr, *kw, *kf;

    if (!mtx || GetLastError() == ERROR_ALREADY_EXISTS) {
        say("SnipSnap is already running. Use 'snipsnap quit' to stop it.\n");
        return RC_OK;
    }
    g_daemon = TRUE;

    ZeroMemory(&wc, sizeof wc);
    wc.lpfnWndProc = daemon_proc;
    wc.hInstance = GetModuleHandleW(NULL);
    wc.lpszClassName = DAEMON_CLASS;
    RegisterClassW(&wc);
    h = CreateWindowExW(0, DAEMON_CLASS, L"SnipSnap", 0, 0, 0, 0, 0,
                        HWND_MESSAGE, NULL, wc.hInstance, NULL);
    if (!h) { say("SnipSnap: could not create window.\n"); return RC_FAIL; }

    kr = register_one(h, HK_RECT, RECT_KEYS, ARRAYSIZE(RECT_KEYS));
    kw = register_one(h, HK_WINDOW, WINDOW_KEYS, ARRAYSIZE(WINDOW_KEYS));
    kf = register_one(h, HK_FULL, FULL_KEYS, ARRAYSIZE(FULL_KEYS));

    if (!kr && !kw && !kf) {
        const char *msg = "SnipSnap: none of the hotkeys could be registered "
                          "(another program owns them).\n"
                          "Use 'snipsnap rect' / 'window' / 'full' from a launcher instead.\n";
        say(msg);
        if (g_out == INVALID_HANDLE_VALUE)
            MessageBoxA(NULL, msg, "SnipSnap", MB_ICONERROR);
        return RC_FAIL;
    }

    say("SnipSnap running in the background.\n  Rectangle:   ");
    say(kr ? kr->name : "(unavailable - hotkeys taken)");
    say("\n  Window:      ");
    say(kw ? kw->name : "(unavailable - hotkeys taken)");
    say("\n  Full screen: ");
    say(kf ? kf->name : "(unavailable - hotkeys taken)");
    say("\n  Stop with:   snipsnap quit\n");
    /* No console to print to: speak up only if the hotkeys aren't the defaults. */
    if (g_out == INVALID_HANDLE_VALUE &&
        (kr != RECT_KEYS || kw != WINDOW_KEYS || kf != FULL_KEYS)) {
        char msg[300];
        wsprintfA(msg, "Some default hotkeys are taken by another program.\n\n"
                       "Rectangle:\t%s\nWindow:\t\t%s\nFull screen:\t%s",
                  kr ? kr->name : "(unavailable)", kw ? kw->name : "(unavailable)",
                  kf ? kf->name : "(unavailable)");
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
    if (eq(arg, L"rect") || eq(arg, L"window") || eq(arg, L"full")) {
        /* A message-only window to own the clipboard data. */
        HWND owner = CreateWindowExW(0, L"STATIC", NULL, 0, 0, 0, 0, 0,
                                     HWND_MESSAGE, NULL, NULL, NULL);
        int rc = arg[0] == L'r' ? snip_rect(owner)
               : arg[0] == L'w' ? snip_area(owner, active_window())
               : snip_area(owner, cursor_monitor());
        flash_wait();                  /* the image is already on the clipboard */
        return rc;
    }
    if (eq(arg, L"quit") || eq(arg, L"stop")) return quit_daemon();

    say("SnipSnap - copy a rectangle, a window or the screen to the clipboard.\n\n"
        "  snipsnap          run in the background (hotkeys below)\n"
        "  snipsnap rect     select a rectangle once (Esc / right-click cancels)\n"
        "  snipsnap window   capture the active window once\n"
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
