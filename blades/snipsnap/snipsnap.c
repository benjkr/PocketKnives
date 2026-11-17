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

static LRESULT CALLBACK daemon_proc(HWND h, UINT msg, WPARAM wp, LPARAM lp)
{
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
        int rc = arg[0] == L'r' ? snip_select(owner, MODE_RECT)
               : arg[0] == L'w' ? snip_select(owner, MODE_WINDOW)
               : snip_area(owner, cursor_monitor());
        flash_wait();                  /* the image is already on the clipboard */
        return rc;
    }
    if (eq(arg, L"quit") || eq(arg, L"stop")) return quit_daemon();

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
