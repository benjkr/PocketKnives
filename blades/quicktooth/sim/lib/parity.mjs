// `qt-sim parity`: renders the icon check sheets with the real app (QuickTooth.exe --render-icons)
// and with the simulator's WebAssembly build of the same code, then diffs them pixel by pixel. Fails if they drift apart.
import { execFile } from 'node:child_process';
import { mkdir, readFile, writeFile, stat } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { promisify } from 'node:util';
import { launch, ORIGIN, REPO_ROOT } from './browser.mjs';

const run = promisify(execFile);

// The simulator draws with the app's own Rust renderer (core/, compiled to WebAssembly),
// so the sheets should match QuickTooth.exe's to within 1 level: the only differences come from
// the browser's and Windows' maths libraries (sin/cos/pow) rounding a few edge pixels the other way.
// Measured (Nov 2026): static mean |Δ| 0.03, anim 0.05, max 1 in both.
export const THRESHOLDS = {
  static: { meanAbs: 0.15, p99: 1, max: 1 },
  anim: { meanAbs: 0.15, p99: 1, max: 1, worstFrameMeanAbs: 0.6, worstCentroidShiftPx: 0.03, worstMassDiff: 0.015 },
};

function check(name, r) {
  const t = THRESHOLDS[name], failures = [];
  for (const [k, limit] of Object.entries(t)) {
    const v = typeof r[k] === 'object' ? r[k].v : r[k];
    if (v > limit) failures.push(`${k} ${v.toFixed(3)} > ${limit}${r[k]?.where ? ` (${r[k].where})` : ''}`);
  }
  return failures;
}

// What `cargo build --release -p quicktooth` produces (the Cargo workspace is the repository root).
const WORKSPACE = path.resolve(REPO_ROOT, '..', '..');
const BUILT_EXE = path.join(WORKSPACE, 'target', 'release', 'QuickTooth.exe');

export async function parity({ outDir, build = true, exe, log = console.log }) {
  await mkdir(outDir, { recursive: true });
  if (!exe) {
    if (build) {
      log('building quicktooth (release)…');
      // Build only: nothing is installed or started.
      await run('cargo', ['build', '--release', '-p', 'quicktooth', '--quiet'], { cwd: WORKSPACE, windowsHide: true, maxBuffer: 1 << 24 });
    }
    if (!await stat(BUILT_EXE).catch(() => null)) throw new Error('target/release/QuickTooth.exe not found; build it first (cargo build --release -p quicktooth)');
    exe = BUILT_EXE;
  }
  const refDir = path.join(os.tmpdir(), `qt-sim-parity-${process.pid}`);
  await mkdir(refDir, { recursive: true });
  log(`real renderer: ${path.relative(WORKSPACE, exe)} --render-icons`);
  // --render-icons is a CLI mode: draws to PNGs and exits; it never starts the tray.
  await run(exe, ['--render-icons', refDir], { windowsHide: true });

  const sheets = [
    { name: 'static', file: 'tray-check-1x.png' },
    { name: 'anim', file: 'anim-frames-1x.png' },
  ];
  const refs = {};
  for (const s of sheets) refs[s.name] = 'data:image/png;base64,' + (await readFile(path.join(refDir, s.file))).toString('base64');

  const { browser, page, version } = await launch({ viewport: { width: 400, height: 300 } });
  try {
    await page.goto(`${ORIGIN}/sim/web/blank.html`);
    const result = await page.evaluate(async (refs) => {
      const { loadKit } = await import('/sim/web/kit.js');
      const { staticSheet, animSheet, compare } = await import('/sim/web/parity.js');
      const kit = await loadKit();
      const anim = animSheet(kit.renderer);
      return {
        warnings: kit.warnings,
        rows: anim.rows,
        static: await compare(refs.static, staticSheet(kit.renderer), 4),
        anim: await compare(refs.anim, anim.image, 6, anim.cells),
      };
    }, refs);
    log(`chromium ${version}`);
    for (const w of result.warnings) log('warning: ' + w);
    let ok = true;
    const report = { thresholds: THRESHOLDS, sheets: {} };
    for (const s of sheets) {
      const r = result[s.name];
      if (!r.ok) { log(`${s.name}: ${r.error}`); ok = false; continue; }
      const failures = check(s.name, r);
      const pass = failures.length === 0;
      ok &&= pass;
      const png = path.join(outDir, `parity-${s.name}.png`);
      await writeFile(png, Buffer.from(r.sideBySidePng, 'base64'));
      delete r.sideBySidePng;
      report.sheets[s.name] = { ...r, pass, failures, image: png };
      const frames = r.worstCentroidShiftPx ? `  worst frame: mean ${r.worstFrameMeanAbs.v.toFixed(2)}, centroid ${r.worstCentroidShiftPx.v.toFixed(3)} px, mass ${(r.worstMassDiff.v * 100).toFixed(1)}%` : '';
      log(`${pass ? 'PASS' : 'FAIL'} ${s.name.padEnd(6)} ${r.width}×${r.height}  mean |Δ| ${r.meanAbs.toFixed(2)}  p99 ${r.p99}  max ${r.max}${frames}`);
      for (const f of failures) log('     ' + f);
      log(`     side by side (real | sim | diff): ${png}`);
    }
    report.animRows = result.rows;
    report.ok = ok;
    await writeFile(path.join(outDir, 'parity.json'), JSON.stringify(report, null, 2));
    return report;
  } finally {
    await browser.close();
  }
}
