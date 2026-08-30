// Building and unpacking `npm pack` tarballs, for any package in the workspace.

import { execFileSync, spawnSync } from 'node:child_process';
import { mkdirSync, readdirSync, rmSync, statSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { workspaceTarballsDirAbsPath } from './paths.ts';

/** This module's own directory — the anchor for the workspace lookup below. */
const THIS_DIR = dirname(fileURLToPath(import.meta.url));

/**
 * The one directory every workspace member packs its tarball into. Members only add to it; only the
 * workspace root clears it, so no caller may sweep `*.tgz` here.
 *
 * A function, not a const: it throws outside the workspace, and this module is reachable from the
 * package root export, so computing it eagerly broke every out-of-tree consumer fixture on import.
 *
 * @throws if called from outside the sdk workspace.
 * @example
 * tarballDirAbsPath(); // '/repo/sdk/tarballs'
 */
export function tarballDirAbsPath(): string {
  return workspaceTarballsDirAbsPath(THIS_DIR);
}

/**
 * Cache `npm pack` runs against, shared by every caller. Its own rather than the user's: sharing that one
 * makes concurrent runs contend on its lock and fail with an npm error that says nothing about packing.
 */
const NPM_CACHE_ABS_PATH = join(tmpdir(), 'fhevm-sdk-npm-cache');

////////////////////////////////////////////////////////////////////////////////

/**
 * Removes only `*.tgz`, and only when asked. Never the default: `outDir` may be a directory the caller
 * named for its own reasons, and only the workspace root may clear the shared tarballs directory.
 */
function _removeExistingTarballs(outDir: string): void {
  for (const entry of readdirSync(outDir)) {
    if (entry.endsWith('.tgz')) {
      rmSync(join(outDir, entry), { force: true });
    }
  }
}

/** The tarball filename out of `npm pack --json`, which emits one array entry per packed package. */
function _parseNpmPackOutput(stdout: string): string {
  const parsed: unknown = JSON.parse(stdout);
  if (!Array.isArray(parsed)) {
    throw new Error(`Unexpected npm pack output: ${stdout}`);
  }

  const firstEntry: unknown = parsed[0];
  if (typeof firstEntry !== 'object' || firstEntry === null) {
    throw new Error(`Unexpected npm pack output: ${stdout}`);
  }

  const filename = (firstEntry as Record<string, unknown>).filename;
  if (typeof filename !== 'string') {
    throw new Error(`npm pack output does not contain a filename: ${stdout}`);
  }

  return filename;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Packs the package in `packageDir` with `npm pack` and returns the tarball's absolute path.
 *
 * @param parameters.packageDir Directory holding the package.json to pack — the PUBLISHED manifest where
 *        a package keeps its payload and its harness apart.
 * @param parameters.outDir Destination directory, created if missing. Defaults to tarballDirAbsPath().
 * @param parameters.clean Delete existing `*.tgz` in `outDir` first. Defaults to false.
 * @throws if npm fails, or reports a filename that did not land in `outDir`.
 * @example
 * createPackageTarball({ packageDir: '/repo/sdk/js-sdk' }); // '/repo/sdk/tarballs/fhevm-sdk-0.13.0.tgz'
 */
export function createPackageTarball(parameters: {
  readonly packageDir: string;
  readonly outDir?: string;
  readonly clean?: boolean;
}): string {
  const outDir = resolve(parameters.outDir ?? tarballDirAbsPath());
  mkdirSync(outDir, { recursive: true });

  if (parameters.clean === true) {
    _removeExistingTarballs(outDir);
  }

  mkdirSync(NPM_CACHE_ABS_PATH, { recursive: true });

  const result = spawnSync('npm', ['pack', '--json', '--pack-destination', outDir], {
    cwd: parameters.packageDir,
    encoding: 'utf8',
    env: {
      ...process.env,
      npm_config_cache: NPM_CACHE_ABS_PATH,
    },
    stdio: 'pipe',
  });

  if (result.error !== undefined) {
    throw result.error;
  }

  if (result.status !== 0) {
    throw new Error(`npm pack failed in ${parameters.packageDir}\n${result.stdout}${result.stderr}`);
  }

  const filename = _parseNpmPackOutput(result.stdout);
  const tarballPath = join(outDir, filename);

  // npm reports success even when the file did not land where expected, so confirm it rather than
  // trusting the exit code — a missing tarball otherwise surfaces much later, in whatever consumes it.
  try {
    statSync(tarballPath);
  } catch {
    throw new Error(`npm pack reported ${filename} but ${tarballPath} does not exist`);
  }

  return tarballPath;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Unpacks a tarball's `package/` contents into `destDir`, which is emptied first and created if missing.
 *
 * @param tarballPath Absolute path to the `.tgz`, as `npm pack` wrote it.
 * @param destDir Where the payload lands — typically a `node_modules/<name>` inside a test fixture.
 * @throws if `tar` fails.
 */
export function extractPackageTarball(tarballPath: string, destDir: string): void {
  rmSync(destDir, { recursive: true, force: true });
  mkdirSync(destDir, { recursive: true });
  execFileSync('tar', ['-xzf', tarballPath, '--strip-components', '1', '-C', destDir], {
    encoding: 'utf8',
    stdio: 'pipe',
  });
}
