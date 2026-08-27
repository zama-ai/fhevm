// Builds the tarball and installs it into the test/ts consumer fixture.

import { createPackageTarball, extractPackageTarball } from '@fhevm/sdk-common';
import { join } from 'node:path';
import { PACKAGE_ROOT_ABS_PATH, PKG_DIR_ABS_PATH } from './constants.ts';

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

/**
 * Packs the payload and extracts it into test/ts/node_modules, so the fixture consumes the package the
 * way a real dependent would. Returns the tarball it installed.
 */
export function prepareTarballConsumer(): string {
  const tarballPath = createPackageTarball({ packageDir: PKG_DIR_ABS_PATH });
  extractPackageTarball(tarballPath, TEST_TARBALL_CONSUMER_PACKAGE_DIR);
  return tarballPath;
}
