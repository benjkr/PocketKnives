// Builds the app's shared Rust code (blades/quicktooth/core, through the quicktooth-web crate) for
// the browser, into sim/web/core, whenever its sources are newer than the last build. Every
// simulator run goes through ensureCore(), so the page never runs stale app code. Needs the Rust
// toolchain (rust-toolchain.toml at the repo root; rustup installs it and the wasm target).
import { execFile } from 'node:child_process';
import { copyFile, mkdir, readFile, readdir, stat, writeFile, rm } from 'node:fs/promises';
import path from 'node:path';
import { promisify } from 'node:util';
import { REPO_ROOT, SIM_DIR } from './browser.mjs';

const run = promisify(execFile);
export const CORE_DIR = path.join(SIM_DIR, 'web', 'core');
const STAMP = path.join(CORE_DIR, 'build.json');
// The Cargo workspace is the repository root (REPO_ROOT is this blade's folder).
const WORKSPACE = path.resolve(REPO_ROOT, '..', '..');
const TARGET = 'wasm32-unknown-unknown';
const WASM = path.join(WORKSPACE, 'target', TARGET, 'release', 'quicktooth_web.wasm');
// Relative to REPO_ROOT, as git and the log lines name them.
const SOURCES = ['core', 'web', 'assets/airpods-pro-fill.svg', 'assets/airpods-pro-line.svg',
  '../../Cargo.toml', '../../Cargo.lock', '../../rust-toolchain.toml', '../../.cargo/config.toml'];

/** Newest modification time (ms) under the shared sources, ignoring build output. */
async function newestSource() {
  let newest = 0, files = 0;
  async function visit(p) {
    const s = await stat(p).catch(() => null);
    if (!s) return;
    if (s.isDirectory()) {
      if (path.basename(p) === 'target') return;
      for (const e of await readdir(p)) await visit(path.join(p, e));
    } else {
      files++;
      newest = Math.max(newest, s.mtimeMs);
    }
  }
  for (const rel of SOURCES) await visit(path.join(REPO_ROOT, rel));
  return { newest, files };
}

async function gitDescribe() {
  try {
    const { stdout: head } = await run('git', ['rev-parse', '--short', 'HEAD'], { cwd: REPO_ROOT, windowsHide: true });
    const { stdout: dirty } = await run('git', ['status', '--porcelain', '--', ...SOURCES], { cwd: REPO_ROOT, windowsHide: true });
    return head.trim() + (dirty.trim() ? '+local changes' : '');
  } catch {
    return 'unknown commit';
  }
}

/**
 * Makes sure sim/web/core holds a build of the current Rust core. Returns { built, label, ms }.
 * @param {{ log?: (line:string)=>void, force?: boolean }} [o]
 */
export async function ensureCore({ log = console.log, force = false } = {}) {
  const src = await newestSource();
  const stamp = JSON.parse(await readFile(STAMP, 'utf8').catch(() => 'null'));
  const label = await gitDescribe();
  if (!force && stamp && stamp.kind === 'rust' && stamp.sourceMtime >= src.newest && stamp.sourceFiles === src.files) {
    log(`app core: Rust build from ${stamp.label}, ${new Date(stamp.builtAt).toLocaleString()} (up to date)`);
    return { built: false, label: stamp.label, ms: 0 };
  }
  log(`app core: building quicktooth-web for the browser (${label})…`);
  const t0 = Date.now();
  await rm(CORE_DIR, { recursive: true, force: true });
  try {
    await run('cargo', ['build', '--release', '--target', TARGET, '-p', 'quicktooth-web', '--quiet'],
      { cwd: WORKSPACE, windowsHide: true, maxBuffer: 1 << 26 });
  } catch (e) {
    const out = `${e.stdout ?? ''}${e.stderr ?? ''}`.split(/\r?\n/).filter((l) => /error|warning|-->/i.test(l)).slice(0, 20).join('\n');
    throw new Error(`building the Rust core for the browser failed:\n${out || e.message}`);
  }
  await mkdir(CORE_DIR, { recursive: true });
  await copyFile(WASM, path.join(CORE_DIR, 'quicktooth_web.wasm'));
  const ms = Date.now() - t0;
  await writeFile(STAMP, JSON.stringify({ kind: 'rust', label, builtAt: Date.now(), sourceMtime: src.newest, sourceFiles: src.files }, null, 1));
  log(`app core: built in ${(ms / 1000).toFixed(1)} s`);
  return { built: true, label, ms };
}
