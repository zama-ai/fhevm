// Builds the publishable tarball with `npm pack`, into a directory the caller chooses.

import { spawnSync } from 'node:child_process';
import { mkdirSync, readdirSync, rmSync, statSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { PACKAGE_ROOT_ABS_PATH, PKG_DIR_ABS_PATH } from './constants.ts';

////////////////////////////////////////////////////////////////////////////////

/**
 * Where the tarball goes when no directory is given: a sibling of ./pkg, not somewhere under test/.
 *
 * The tarball is a build output of the package, so anything can consume it — the TS fixture, a sibling
 * test-suite, a release check. Keeping it out of test/ is what says so: a path under one test directory
 * reads as that directory's private scratch space.
 *
 * `npm pack` itself runs against the PUBLISHED payload manifest (./pkg), not the private harness one.
 */
const TARBALL_DIR = join(PACKAGE_ROOT_ABS_PATH, 'tarball');

type NpmPackEntry = {
  filename: string;
};

////////////////////////////////////////////////////////////////////////////////

/**
 * Removes only `*.tgz`, and only when asked.
 *
 * Never the default: `outDir` may be a directory the caller named for its own reasons, and deleting
 * files there is well outside what building a tarball should do. The fixture path opts in, because it
 * wants exactly one v13 tarball to hand the consumer.
 *
 * Note that ./tarball is shared — prepareTestV12Consumer.ts packs the v12 generation alongside. That is
 * safe because `npm pack` names by package AND version, so the two never collide, but it does mean a
 * `clean` here removes the sibling generation's tarball too.
 */
function _removeExistingTarballs(outDir: string): void {
  for (const entry of readdirSync(outDir)) {
    if (entry.endsWith('.tgz')) {
      rmSync(join(outDir, entry), { force: true });
    }
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * The tarball filename out of `npm pack --json`, which emits one array entry
 * per packed package.
 */
function _parseNpmPackOutput(stdout: string): NpmPackEntry {
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

  return { filename };
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Packs `pkg/` and returns the tarball's absolute path.
 *
 * @param parameters.outDir Destination directory, created if missing. Defaults to TARBALL_DIR.
 * @param parameters.clean  Delete existing `*.tgz` in `outDir` first. Defaults to false.
 */
export function createPackageTarball(
  parameters: {
    readonly outDir?: string;
    readonly clean?: boolean;
  } = {},
): string {
  const outDir = resolve(parameters.outDir ?? TARBALL_DIR);
  mkdirSync(outDir, { recursive: true });

  if (parameters.clean === true) {
    _removeExistingTarballs(outDir);
  }

  // Its own cache, because sharing the user's makes concurrent runs contend on the same lock and fail
  // with an npm error that says nothing about packing.
  const npmCache = join(tmpdir(), 'fhevm-host-contracts-cleartext-npm-cache');
  mkdirSync(npmCache, { recursive: true });

  const result = spawnSync('npm', ['pack', '--json', '--pack-destination', outDir], {
    cwd: PKG_DIR_ABS_PATH,
    encoding: 'utf8',
    env: {
      ...process.env,
      npm_config_cache: npmCache,
    },
    stdio: 'pipe',
  });

  if (result.error !== undefined) {
    throw result.error;
  }

  if (result.status !== 0) {
    throw new Error(`npm pack failed\n${result.stdout}${result.stderr}`);
  }

  const entry = _parseNpmPackOutput(result.stdout);
  const tarballPath = join(outDir, entry.filename);

  // npm reports success even when the file did not land where expected, so confirm it rather than
  // trusting the exit code — a missing tarball otherwise surfaces much later, as a tar error in the
  // fixture step.
  try {
    statSync(tarballPath);
  } catch {
    throw new Error(`npm pack reported ${entry.filename} but ${tarballPath} does not exist`);
  }

  return tarballPath;
}
