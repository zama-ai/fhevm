// Builds and installs the sibling v12 package into the test/ts fixture, for the v12->v13 upgrade e2e.

import { execFileSync, spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, rmSync } from 'node:fs';
import { join } from 'node:path';
import {
  PACKAGE_ROOT_ABS_PATH,
  PREVIOUS_GENERATION_DIR_ABS_PATH,
  PREVIOUS_GENERATION_FIXTURE_ALIAS,
} from './constants.ts';
import { run } from './utils.ts';

////////////////////////////////////////////////////////////////////////////////

// The sibling cleartext-v12 package. The e2e upgrade test deploys a fresh v12 stack via this package,
// then upgrades it to v13 via the local (v13) package's `updateV12ToV13`.
const V12_PACKAGE_ROOT = PREVIOUS_GENERATION_DIR_ABS_PATH;

/**
 * The previous generation's PAYLOAD, which is what gets packed — not its harness root.
 *
 * Every generation now uses the rule 9 split: the harness manifest at the package root is
 * `private: true` and carries the devDependencies, while the publishable manifest sits in `pkg/`.
 * Packing the root therefore produces the wrong tarball (or none), and the fixture the e2e imports by
 * the published name would not exist. `createPackageTarball.ts` packs `PKG_DIR_ABS_PATH` for exactly
 * this reason; this is the same rule applied to the sibling.
 */
const V12_PAYLOAD_DIR = join(V12_PACKAGE_ROOT, 'pkg');
const V12_TARBALL_DIR = join(PACKAGE_ROOT_ABS_PATH, 'tarball');
const V12_CONSUMER_PACKAGE_DIR = join(
  PACKAGE_ROOT_ABS_PATH,
  'test',
  'ts',
  'node_modules',
  ...PREVIOUS_GENERATION_FIXTURE_ALIAS.split('/'),
);

////////////////////////////////////////////////////////////////////////////////

function _packV12(): string {
  // No pre-clean: every generation packs under the same name, differing only by version, so a
  // prefix sweep here would delete v13's tarball from this shared directory. The exact filename
  // comes from `npm pack --json` below, which makes stale files inert; `clean:tarball-consumer`
  // removes the directory wholesale.
  const result = spawnSync('npm', ['pack', '--json', '--pack-destination', V12_TARBALL_DIR], {
    cwd: V12_PAYLOAD_DIR,
    encoding: 'utf8',
    stdio: 'pipe',
  });
  if (result.status !== 0) {
    throw new Error(`npm pack (v12) failed\n${result.stdout}${result.stderr}`);
  }
  const parsed: unknown = JSON.parse(result.stdout);
  const first: unknown = Array.isArray(parsed) ? parsed[0] : undefined;
  if (typeof first !== 'object' || first === null || typeof (first as Record<string, unknown>).filename !== 'string') {
    throw new Error(`Unexpected npm pack output: ${result.stdout}`);
  }
  return join(V12_TARBALL_DIR, (first as { filename: string }).filename);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Builds the sibling v12 package, packs it, and installs it into the test/ts fixture.
 *
 * Never throws: the upgrade e2e is optional, so an unavailable v12 is reported and skipped.
 * The logging stays inside because it is the skip diagnostics, interleaved with the control flow
 * that decides to skip.
 */
export function prepareV12Consumer(): void {
  // The upgrade-e2e test is optional: it needs the sibling cleartext-v12 package to deploy the
  // "before" stack. When that package isn't checked out, skip fixture prep — the library and every
  // other test build/run without it, and the upgrade e2e self-skips (see internal/runUpgradeE2e.ts).
  if (!existsSync(V12_PACKAGE_ROOT)) {
    console.log(
      `[v12-consumer] sibling host-contracts-cleartext/v12 not found at ${V12_PACKAGE_ROOT} — skipping upgrade-e2e fixture.`,
    );
    return;
  }

  // Preparing the fixture requires the sibling package to be fully set up (its deps installed so
  // forge + tsc can build it). If anything fails — deps not installed, forge/tsc error — treat the
  // fixture as unavailable and skip: the upgrade e2e is optional and self-skips (runUpgradeE2e.ts).
  try {
    // Build the v12 package (contracts → templates → TS) so its tarball ships a ready-to-import `ts/`.
    run('npm', ['run', 'build:templates'], V12_PACKAGE_ROOT);
    run('npm', ['run', 'build'], V12_PACKAGE_ROOT);

    const tarballPath = _packV12();
    rmSync(V12_CONSUMER_PACKAGE_DIR, { recursive: true, force: true });
    mkdirSync(V12_CONSUMER_PACKAGE_DIR, { recursive: true });
    execFileSync('tar', ['-xzf', tarballPath, '--strip-components', '1', '-C', V12_CONSUMER_PACKAGE_DIR], {
      encoding: 'utf8',
      stdio: 'pipe',
    });
    console.log(`[v12-consumer] installed v12 fixture from ${tarballPath}`);
  } catch (error) {
    console.warn(
      `[v12-consumer] could not prepare the host-contracts-cleartext/v12 fixture — skipping upgrade-e2e. ` +
        `If you need the v12→v13 upgrade test, install + build the sibling package first ` +
        `(cd ../v12 && npm ci). Reason: ${error instanceof Error ? error.message : String(error)}`,
    );
  }
}
