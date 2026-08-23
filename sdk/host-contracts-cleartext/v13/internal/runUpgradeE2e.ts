// Drives the optional v12->v13 upgrade end-to-end test.

import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { join } from 'node:path';
import {
  PACKAGE_ROOT_ABS_PATH,
  PREVIOUS_GENERATION_DIR_ABS_PATH,
  PREVIOUS_GENERATION_FIXTURE_ALIAS,
} from './constants.ts';

////////////////////////////////////////////////////////////////////////////////

/**
 * Runs the v12->v13 upgrade e2e end to end, or exits 0 having explained why it is skipping.
 *
 * Calls process.exit directly rather than returning a status: every exit here is either a
 * deliberate skip or a failed child process whose code has to reach the caller unchanged.
 */
export function runUpgradeE2e(): void {
  // Fast preliminary gate — NO builds. The v12→v13 upgrade e2e needs the sibling cleartext-v12
  // package present AND installed (so it can be built + packed into a consumer fixture). Checking
  // this up front lets us skip the whole flow instantly, instead of running the slow v13 build and a
  // doomed v12 forge build before discovering v12 is unusable. The library and every other test are
  // unaffected by the skip.
  const V12_PACKAGE_ROOT = PREVIOUS_GENERATION_DIR_ABS_PATH;
  if (!existsSync(join(V12_PACKAGE_ROOT, 'node_modules'))) {
    console.log(
      '[upgrade-e2e] host-contracts-cleartext/v12 not available (missing, or deps not installed) — ' +
        'skipping upgrade e2e. To run it: cd ../v12 && npm ci',
    );
    process.exit(0);
  }

  function run(command: string, args: readonly string[]): void {
    const result = spawnSync(command, args, { cwd: PACKAGE_ROOT_ABS_PATH, encoding: 'utf8', stdio: 'inherit' });
    if (result.status !== 0) {
      process.exit(result.status ?? 1);
    }
  }

  // v12 looks available — run the full upgrade-e2e flow.
  run('npm', ['run', 'clean:tarball-consumer']);
  run('npm', ['run', 'build:templates']);
  // No prepare:tarball-consumer here: `build` ends with it, and `build` is also what deletes the fixture
  // (its first step is `clean`), so it is obliged to leave one behind.
  run('npm', ['run', 'build']);
  run('node', ['internal/cli/prepareTestV12Consumer.ts']);

  // prepareTestV12Consumer skips (without throwing) if it still can't build v12; guard the typecheck +
  // vitest on the fixture so a late skip stays graceful rather than failing on an unresolved import.
  const TEST_TS = join(PACKAGE_ROOT_ABS_PATH, 'test', 'ts');
  // The sentinel is the package's own declared types entry, so it exists exactly when the fixture is
  // importable. Built from the package name rather than spelled out, so it tracks a rename.
  const V12_FIXTURE = join(
    TEST_TS,
    'node_modules',
    ...PREVIOUS_GENERATION_FIXTURE_ALIAS.split('/'),
    'ts',
    '_types',
    'index.d.ts',
  );
  if (!existsSync(V12_FIXTURE)) {
    console.log('[upgrade-e2e] v12 fixture was not produced — skipping upgrade e2e.');
    process.exit(0);
  }

  run('tsc', ['--project', join(TEST_TS, 'tsconfig.e2e.json'), '--noEmit']);
  run('vitest', ['run', '--config', join(TEST_TS, 'vitest.e2e.config.ts')]);
}
