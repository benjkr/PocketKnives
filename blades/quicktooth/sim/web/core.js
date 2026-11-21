// Loads QuickTooth's shared Rust code (blades/quicktooth/core, built for the browser by
// sim/lib/core-build.mjs into sim/web/core/quicktooth_web.wasm) — the same icon renderer and tray
// state machine the Windows app runs. The page supplies the platform through the "qt-host" imports
// below; each call carries the handle of the JS host object of the app instance that made it (see
// tray.js TrayAppModel). The ABI is described in blades/quicktooth/web/src/lib.rs.

const WASM_URL = new URL('./core/quicktooth_web.wasm', import.meta.url);

// Log lines and settings saves come from per-process hooks in the core, so they go to the most
// recently started app (all apps in a page share one log and one settings object anyway).
let latest = null;
export function setLatestHost(host) { latest = host; }

let loading = null;

/** The core's exports, wrapped as the API the simulator uses (RenderStatic, AppCreate, …); loaded once per page. */
export function loadCore() {
  loading ??= (async () => {
    const t0 = performance.now();
    const res = await fetch(WASM_URL, { cache: 'no-store' });
    if (!res.ok) throw new Error(`${res.status} for ${WASM_URL.pathname} (the simulator builds it: sim/lib/core-build.mjs)`);
    let ex = null;
    const { instance } = await WebAssembly.instantiate(await res.arrayBuffer(), { 'qt-host': makeImports(() => ex) });
    ex = instance.exports;
    const core = wrap(ex);
    core.Init();
    core.loadMs = performance.now() - t0;
    return core;
  })();
  return loading;
}

const enc = new TextEncoder();
const dec = new TextDecoder();

// Host objects by handle (index); a handle stays valid for the page's lifetime, like the C#
// JSObject proxies did, so an old app's leftover work still reaches its own host.
const hosts = [null];
function handleOf(host) {
  let h = hosts.indexOf(host);
  if (h < 0) { h = hosts.length; hosts.push(host); }
  return h;
}

function makeImports(exports) {
  const mem = () => exports().memory.buffer;
  const str = (p, n) => dec.decode(new Uint8Array(mem(), p, n));
  const bytes = (p, n) => new Uint8Array(mem(), p, n).slice();
  /** Writes `s` into a block from qt_alloc; returns [ptr, len] (ptr 0 for null). */
  const alloc = (s) => {
    if (s == null) return [0, 0];
    const b = enc.encode(String(s));
    const p = exports().qt_alloc(b.length);
    new Uint8Array(mem(), p, b.length).set(b);
    return [p, b.length];
  };
  const out = (o, s) => { const [p, n] = alloc(s); new Uint32Array(mem(), o, 2).set([p, n]); };
  const H = (h) => hosts[h];
  /** Awaits the page's promise for async op `op`, then completes it in the core. */
  const settle = (op, run, kind) => {
    let p;
    try { p = Promise.resolve(run()); } catch (e) { p = Promise.reject(e); }
    p.then(
      (v) => {
        if (kind === 'str') { const [ptr, n] = alloc(v ?? ''); exports().qt_complete(op, 1, 0, ptr, n); }
        else exports().qt_complete(op, 1, kind === 'num' ? Number(v) : 0, 0, 0);
      },
      (e) => { const [ptr, n] = alloc(String(e?.message ?? e)); exports().qt_complete(op, 0, 0, ptr, n); },
    );
  };
  return {
    exePath: (h, o) => out(o, H(h).exePath), // a property: the mock shell reads app.exePath too
    nowMs: (h) => H(h).nowMs(),
    nowEpochMs: (h) => H(h).nowEpochMs(),
    delay: (h, op, ms) => settle(op, () => H(h).delay(ms)),
    setTimer: (h, id, ms) => H(h).setTimer(id, ms),
    killTimer: (h, id) => H(h).killTimer(id),
    killAllTimers: (h) => H(h).killAllTimers(),
    lightTaskbar: (h) => (H(h).lightTaskbar() ? 1 : 0),
    iconSize: (h) => H(h).iconSize(),
    addIcon: (h, rp, rn, size, tp, tn, dp, dn) => (H(h).addIcon(bytes(rp, rn), size, str(tp, tn), str(dp, dn)) ? 1 : 0),
    modifyIcon: (h, rp, rn, size, tp, tn, dp, dn) => (H(h).modifyIcon(bytes(rp, rn), size, str(tp, tn), str(dp, dn)) ? 1 : 0),
    removeIcon: (h) => H(h).removeIcon(),
    cursorPos: (h, o) => out(o, H(h).cursorPos()),     // "x,y" or ""
    iconAnchor: (h, o) => out(o, H(h).iconAnchor()),   // "x,y" or ""
    trackPopupMenu: (h, op, ip, n, x, y, light) => { const items = str(ip, n); settle(op, () => H(h).trackPopupMenu(items, x, y, !!light), 'num'); },
    endMenu: (h) => H(h).endMenu(),
    promoteSelf: (h, o) => out(o, H(h).promoteSelf()),
    autostartEnabled: (h) => (H(h).autostartEnabled() ? 1 : 0),
    setAutostart: (h, on) => H(h).setAutostart(!!on),
    exitApp: (h) => H(h).exitApp(),
    emit: (h, p, n) => H(h).emit(str(p, n)),
    log: (p, n) => { const line = str(p, n); latest ? latest.log(line) : console.warn('[core log]', line); },
    saveSettings: (p, n) => latest?.saveSettings(str(p, n)),
    panic: (p, n) => console.error('[core panic]', str(p, n)),
    // The mock Bluetooth stack (sim/web/bluetooth.js), behind the app's device worker.
    btRegister: (h, op) => settle(op, () => H(h).btRegister()),
    btRefresh: (h, op, p, n) => { const mac = str(p, n); settle(op, () => H(h).btRefresh(mac), 'str'); }, // "" = no target MAC
    btSend: (h, op, cp, cn, op2, on) => { const c = str(cp, cn), o = str(op2, on); settle(op, () => H(h).btSend(c, o), 'str'); },
    btConnected: (h, op, p, n) => { const mac = str(p, n); settle(op, () => H(h).btConnected(mac), 'num'); }, // -1 unknown, 0, 1
    btScan: (h, op, model, windowMs) => settle(op, () => H(h).btScan(model, windowMs), 'str'), // JSON or "null"
    btWatchStart: (h, model) => H(h).btWatchStart(model),
    btWatchStop: (h) => H(h).btWatchStop(),
  };
}

/** The exports with JS types: strings in, strings and Uint8Array (RGBA) out. */
function wrap(ex) {
  const mem = () => ex.memory.buffer;
  const pass = (s) => {
    if (s == null) return [0, 0];
    const b = enc.encode(String(s));
    const p = ex.qt_alloc(b.length);
    new Uint8Array(mem(), p, b.length).set(b);
    return [p, b.length];
  };
  const result = (p, what) => {
    if (p === 0) throw new Error(what);
    return new Uint8Array(mem(), p, ex.qt_result_len()).slice();
  };
  const text = (p) => dec.decode(result(p, 'no result'));
  const b = (v) => (v ? 1 : 0);
  return {
    Init: () => ex.qt_init(),
    // ---- tunables and glyphs
    Tunables: () => text(ex.qt_tunables()),
    SetTunable: (name, value) => ex.qt_set_tunable(...pass(name), Number(value)) !== 0,
    ResetTunables: () => ex.qt_reset_tunables(),
    SetGlyphs: (fillSvg, lineSvg) => { const f = pass(fillSvg), l = pass(lineSvg); ex.qt_set_glyphs(...f, ...l); },
    // ---- renderer (RGBA, straight alpha, for ImageData)
    /** state: "connected", "disconnected" or "lowBattery". */
    RenderStatic: (state, light, size) => result(ex.qt_render_static(...pass(state), b(light), size), `no icon state ${state}`),
    RenderAnim: (phase, t, tracerT, light, size) => result(ex.qt_render_anim(...pass(phase), t, tracerT, b(light), size), `no phase ${phase}`),
    PhaseDone: (phase, t, stillPendingDisconnect) => {
      const r = ex.qt_phase_done(...pass(phase), t, b(stillPendingDisconnect));
      if (r < 0) throw new Error(`no phase ${phase}`);
      return r === 1;
    },
    // ---- tray app instances (a restart creates a new one; the old one keeps its ID)
    AppCreate: (host, settingsJson) => {
      const id = ex.qt_app_create(handleOf(host), ...pass(settingsJson));
      if (id === 0) throw new Error(`settings JSON not readable: ${settingsJson}`);
      return id;
    },
    AppStart: (id) => ex.qt_app_start(id),
    AppExit: (id) => ex.qt_app_exit(id),
    AppRunning: (id) => ex.qt_app_running(id) !== 0,
    AppTrayEvent: (id, ev, x, y) => ex.qt_app_tray_event(id, ...pass(ev), x, y),
    AppCommand: (id, cmd) => ex.qt_app_command(id, cmd),
    AppTimer: (id, timer) => ex.qt_app_timer(id, timer),
    AppSettingChange: (id, area) => ex.qt_app_setting_change(id, ...pass(area)),
    AppDisplayChange: (id) => ex.qt_app_display_change(id),
    AppTaskbarCreated: (id) => ex.qt_app_taskbar_created(id),
    AppEndpointChanged: (id) => ex.qt_app_endpoint_changed(id),
    /** A status advert (Uint8Array, Apple's manufacturer data) from the watch: WM_APP_ADVERT. */
    AppAdvert: (id, data, rssi) => {
      const p = ex.qt_alloc(data.length);
      new Uint8Array(mem(), p, data.length).set(data);
      ex.qt_app_advert(id, p, data.length, rssi);
    },
    AppRebuildIcons: (id) => ex.qt_app_rebuild_icons(id),
    AppSnapshot: (id) => text(ex.qt_app_snapshot(id)),
    AppStatusLine: (id) => text(ex.qt_app_status_line(id)),
    /** QuickTooth.exe --set handed to the running app (key=value lines): "" when applied, else the line the CLI prints. */
    AppSet: (id, pairs) => text(ex.qt_app_set(id, ...pass(pairs))),
  };
}
