import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { PACKAGE_ROOT } from './createPackageTarball.ts';

// Fast preliminary gate — NO builds. The v12→v13 upgrade e2e needs the sibling cleartext-v12
// package present AND installed (so it can be built + packed into a consumer fixture). Checking
// this up front lets us skip the whole flow instantly, instead of running the slow v13 build and a
// doomed v12 forge build before discovering v12 is unusable. The library and every other test are
// unaffected by the skip.
const V12_PACKAGE_ROOT = resolve(PACKAGE_ROOT, '..', 'host-contracts-cleartext-v12');
if (!existsSync(join(V12_PACKAGE_ROOT, 'node_modules'))) {
  console.log(
    '[upgrade-e2e] host-contracts-cleartext-v12 not available (missing, or deps not installed) — ' +
      'skipping upgrade e2e. To run it: cd ../host-contracts-cleartext-v12 && npm ci',
  );
  process.exit(0);
}

function run(command: string, args: readonly string[]): void {
  const result = spawnSync(command, args, { cwd: PACKAGE_ROOT, encoding: 'utf8', stdio: 'inherit' });
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

// v12 looks available — run the full upgrade-e2e flow.
run('npm', ['run', 'clean:tarball-consumer']);
run('npm', ['run', 'build:templates']);
run('npm', ['run', 'build']);
run('npm', ['run', 'prepare:tarball-consumer']);
run('node', ['internal/prepareV12Consumer.ts']);

// prepareV12Consumer skips (without throwing) if it still can't build v12; guard the typecheck +
// vitest on the fixture so a late skip stays graceful rather than failing on an unresolved import.
const TEST_TS = join(PACKAGE_ROOT, 'test', 'ts');
const V12_FIXTURE = join(
  TEST_TS,
  'node_modules',
  '@fhevm',
  'host-contracts-cleartext-v12',
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
