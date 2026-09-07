// The packing primitives shared by `publish pack` and its `pack-tarball` alias: which payloads are
// packable (from the dev owner's publishedRelPath, never a `./pkg` guess; mirror-only refused), where
// tarballs go (declared in the manifest), and one `npm pack` run with the workspace's own npm cache.

import { spawnSync } from 'node:child_process';
import { mkdirSync, statSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';

import type { NpmManifest } from '../manifest.ts';
import { loadPackages } from './npm.ts';

/** Own npm cache rather than the user's: sharing that one makes concurrent runs contend on its lock
 * and fail with an npm error that says nothing about packing. */
const NPM_CACHE_ABS_PATH = join(tmpdir(), 'fhevm-sdk-npm-cache');

export type PackTarget = {
  readonly ownerKey: string;
  readonly payloadKey: string;
  readonly payloadDirectory: string;
};

/** Every npm-distributed payload, keyed by its dev owner — the packable universe. */
export function selectPackTargets(
  workspaceRoot: string,
  manifest: NpmManifest,
  selector?: string,
): readonly PackTarget[] {
  const packages = loadPackages(workspaceRoot, manifest);
  const byKey = new Map(packages.map((pkg) => [pkg.key, pkg]));

  const targets: PackTarget[] = [];
  for (const owner of packages) {
    if (owner.inventory.kind !== 'dev' || owner.inventory.publishedRelPath === undefined) continue;
    const payload = byKey.get(owner.inventory.publishedRelPath);
    if (payload === undefined || !(payload.inventory.distribution ?? ['npm']).includes('npm')) continue;
    targets.push({ ownerKey: owner.key, payloadKey: payload.key, payloadDirectory: payload.directory });
  }
  targets.sort((left, right) => left.ownerKey.localeCompare(right.ownerKey));

  if (selector === undefined) return targets;
  const normalized = selector === '.' || selector.startsWith('./') ? selector : `./${selector.replace(/^\//, '')}`;
  const matches = targets.filter((target) => target.ownerKey === normalized || target.payloadKey === normalized);
  if (matches.length === 0) {
    throw new Error(
      `No npm-distributed payload matches '${selector}'. Packable owners: ${targets
        .map((target) => target.ownerKey)
        .join(', ')}`,
    );
  }
  return matches;
}

/** The one directory every payload packs into, declared in the manifest so nothing guesses it. */
export function tarballsOutDir(workspaceRoot: string, manifest: NpmManifest, override?: string): string {
  if (override !== undefined) return resolve(override);
  const relPath = manifest.tarballs?.relPath;
  if (relPath === undefined) {
    throw new Error('npm-manifest.json#tarballs.relPath is required (e.g. "./tarballs"), or pass --out-dir');
  }
  return resolve(workspaceRoot, relPath);
}

/** One `npm pack` in a directory, scripts skipped (the staged copy has no node_modules); returns the tarball path. */
export function packOne(packageDir: string, outDir: string): string {
  mkdirSync(NPM_CACHE_ABS_PATH, { recursive: true });
  const result = spawnSync('npm', ['pack', '--json', '--ignore-scripts', '--pack-destination', outDir], {
    cwd: packageDir,
    encoding: 'utf8',
    env: { ...process.env, npm_config_cache: NPM_CACHE_ABS_PATH },
    stdio: 'pipe',
  });
  if (result.error !== undefined) throw result.error;
  if (result.status !== 0) {
    throw new Error(`npm pack failed in ${packageDir}\n${result.stdout}${result.stderr}`);
  }

  const tarballPath = join(outDir, parseNpmPackFilename(result.stdout));
  // npm reports success even when the file did not land where expected, so confirm it rather than
  // trusting the exit code — a missing tarball otherwise surfaces much later, in whatever consumes it.
  try {
    statSync(tarballPath);
  } catch {
    throw new Error(`npm pack reported a tarball but ${tarballPath} does not exist`);
  }
  return tarballPath;
}

/** The tarball filename out of `npm pack --json`, which emits one array entry per packed package. */
function parseNpmPackFilename(stdout: string): string {
  const parsed: unknown = JSON.parse(stdout);
  const first: unknown = Array.isArray(parsed) ? parsed[0] : undefined;
  const filename =
    typeof first === 'object' && first !== null ? (first as Record<string, unknown>).filename : undefined;
  if (typeof filename !== 'string') {
    throw new Error(`Unexpected npm pack output: ${stdout}`);
  }
  return filename;
}
