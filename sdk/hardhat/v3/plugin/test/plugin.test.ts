// Proves the plugin loads into a programmatic hardhat 3 environment with its tasks, and that the whole
// cluster resolves ONE hardhat instance (the topology the cluster exists to guarantee).
//
// Skips itself, loudly, until the cluster has been installed once online: hardhat@3 is not in the
// offline cache this skeleton was built under. Delete the skip once `npm --prefix hardhat/v3 install`
// has run — a permanently skipped test is worse than none.

import assert from 'node:assert/strict';
import { createRequire } from 'node:module';
import test from 'node:test';

const require = createRequire(import.meta.url);

function hardhatInstalled(): boolean {
  try {
    require.resolve('hardhat/package.json');
    return true;
  } catch {
    return false;
  }
}

void test('the fhevm tasks are registered on a programmatic hardhat 3 environment', async (t) => {
  if (!hardhatInstalled()) {
    t.skip('hardhat@3 is not installed yet — run `npm --prefix hardhat/v3 install` (online) first');
    return;
  }

  const { createHardhatRuntimeEnvironment } = await import('hardhat/hre');
  const { default: fhevmPlugin } = await import('#esm/index.js');
  const hre = await createHardhatRuntimeEnvironment({ plugins: [fhevmPlugin] });

  assert.notEqual(hre.tasks.getTask(['fhevm']), undefined, 'the fhevm scope root');
  assert.notEqual(hre.tasks.getTask(['fhevm', 'public-decrypt']), undefined, 'fhevm public-decrypt');
  assert.notEqual(hre.tasks.getTask(['fhevm', 'user-decrypt']), undefined, 'fhevm user-decrypt');
  assert.notEqual(
    hre.tasks.getTask(['fhevm', 'check-fhevm-compatibility']),
    undefined,
    'fhevm check-fhevm-compatibility',
  );
});

void test('the cluster resolves exactly one hardhat instance from every member', (t) => {
  if (!hardhatInstalled()) {
    t.skip('hardhat@3 is not installed yet — run `npm --prefix hardhat/v3 install` (online) first');
    return;
  }

  const fromOwner = require.resolve('hardhat/package.json');
  const fromPkg = createRequire(new URL('#esm/index.js', import.meta.url)).resolve('hardhat/package.json');
  assert.equal(fromOwner, fromPkg, 'owner and payload must resolve the SAME hardhat directory');
});
