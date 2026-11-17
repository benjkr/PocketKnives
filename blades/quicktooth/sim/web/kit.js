// Loads the pieces shared by the simulator page and the parity check: the app's Rust core (renderer,
// tray logic and tunables, compiled to WebAssembly), any tunable overrides, and optional
// alternative glyph SVGs.
import { loadCore } from './core.js';
import { IconRenderer } from './icon.js';

const DEFAULT_FILL = 'assets/airpods-pro-fill.svg';
const DEFAULT_LINE = 'assets/airpods-pro-line.svg';

export async function fetchText(repoPath) {
  const res = await fetch('/' + repoPath.replace(/^\/+/, ''), { cache: 'no-store' });
  if (!res.ok) throw new Error(`${res.status} for ${repoPath}`);
  return res.text();
}

/**
 * @param {object} [o]
 * @param {object} [o.overrides] tunable overrides by name (core/src/tunables.rs), e.g. { Travel: 2.4, AnimFrameMs: 50 }
 * @param {string} [o.fillSvg] repo path of an alternative connected glyph
 * @param {string} [o.lineSvg] repo path of an alternative not-connected glyph
 */
export async function loadKit({ overrides = {}, fillSvg = DEFAULT_FILL, lineSvg = DEFAULT_LINE, variant } = {}) {
  if (variant) {
    throw new Error(`--variant (${variant}) is no longer supported: the icon and its animation are Rust shared with the real app. ` +
      'Change core/src/icon (or core/src/tunables.rs) and the simulator rebuilds it; --tune still overrides numbers.');
  }
  const core = await loadCore();
  core.ResetTunables();
  const warnings = [];
  for (const [name, value] of Object.entries(overrides)) {
    if (!core.SetTunable(name, Number(value))) warnings.push(`unknown tunable ${name} (see core/src/tunables.rs)`);
  }
  core.SetGlyphs(fillSvg === DEFAULT_FILL ? null : await fetchText(fillSvg), lineSvg === DEFAULT_LINE ? null : await fetchText(lineSvg));

  const P = JSON.parse(core.Tunables());
  const provenance = Object.fromEntries(Object.keys(P).map((k) => [k, 'core/src/tunables.rs' + (k in overrides ? ' (overridden)' : '')]));
  const kit = {
    core, P, provenance, warnings,
    renderer: new IconRenderer(core, P),
    /** Changes tunables in the running core; returns the new values. */
    tune(changes) {
      for (const [name, value] of Object.entries(changes)) {
        if (!core.SetTunable(name, Number(value))) throw new Error(`unknown tunable ${name}`);
        provenance[name] = 'core/src/tunables.rs (overridden)';
      }
      Object.assign(kit.P, JSON.parse(core.Tunables()));
      return kit.P;
    },
  };
  return kit;
}
