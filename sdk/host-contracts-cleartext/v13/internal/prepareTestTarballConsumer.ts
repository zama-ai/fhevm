// Builds the tarball and installs it into the test/ts consumer fixture.

import { execFileSync } from 'node:child_process';
import { mkdirSync, rmSync } from 'node:fs';
import { join } from 'node:path';
import { PACKAGE_ROOT_ABS_PATH } from './constants.ts';
import { createPackageTarball } from './createPackageTarball.ts';

////////////////////////////////////////////////////////////////////////////////

// <root>/test/ts
const TEST_TARBALL_CONSUMER_DIR = join(PACKAGE_ROOT_ABS_PATH, 'test', 'ts');
// <root>/test/ts/node_modules/@fhevm/host-contracts-cleartext
const TEST_TARBALL_CONSUMER_PACKAGE_DIR = join(
  TEST_TARBALL_CONSUMER_DIR,
  'node_modules',
  '@fhevm',
  'host-contracts-cleartext',
);

////////////////////////////////////////////////////////////////////////////////

function _extractPackageTarball(tarballPath: string): void {
  rmSync(TEST_TARBALL_CONSUMER_PACKAGE_DIR, { recursive: true, force: true });
  mkdirSync(TEST_TARBALL_CONSUMER_PACKAGE_DIR, { recursive: true });
  execFileSync('tar', ['-xzf', tarballPath, '--strip-components', '1', '-C', TEST_TARBALL_CONSUMER_PACKAGE_DIR], {
    encoding: 'utf8',
    stdio: 'pipe',
  });
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Packs the payload and extracts it into test/ts/node_modules, so the fixture consumes the package the
 * way a real dependent would. Returns the tarball it installed.
 */
export function prepareTarballConsumer(): string {
  // clean: this directory belongs to the fixture, and a stale tarball from an earlier version
  // would sit alongside the new one. createPackageTarball does not clean unless asked.
  const tarballPath = createPackageTarball({ clean: true });
  _extractPackageTarball(tarballPath);
  return tarballPath;
}
