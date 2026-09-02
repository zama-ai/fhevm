// Proves the hello-world plugin loads into a programmatic hardhat 3 environment, and that the whole
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

void test('the hello task is registered on a programmatic hardhat 3 environment', async (t) => {
  if (!hardhatInstalled()) {
    t.skip('hardhat@3 is not installed yet — run `npm --prefix hardhat/v3 install` (online) first');
    return;
  }

  const { createHardhatRuntimeEnvironment } = await import('hardhat/hre');
  const { default: fhevmPlugin } = await import('../pkg/_esm/index.js');
  const hre = await createHardhatRuntimeEnvironment({ plugins: [fhevmPlugin] });

  const helloTask = hre.tasks.getTask('hello');
  assert.notEqual(helloTask, undefined, "the plugin must register the 'hello' task");
});

void test('the cluster resolves exactly one hardhat instance from every member', (t) => {
  if (!hardhatInstalled()) {
    t.skip('hardhat@3 is not installed yet — run `npm --prefix hardhat/v3 install` (online) first');
    return;
  }

  const fromOwner = require.resolve('hardhat/package.json');
  const fromPkg = createRequire(new URL('../pkg/_esm/index.js', import.meta.url)).resolve('hardhat/package.json');
  assert.equal(fromOwner, fromPkg, 'owner and payload must resolve the SAME hardhat directory');
});
