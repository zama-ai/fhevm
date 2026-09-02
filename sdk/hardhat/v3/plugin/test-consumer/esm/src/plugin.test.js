// A real consumer's view: the INSTALLED payload (not the workspace source) must register its task on a
// programmatic hardhat 3 environment. Plain JavaScript on purpose — a fixture that adds a toolchain
// stops representing a consumer.

import assert from 'node:assert/strict';
import test from 'node:test';

import { createHardhatRuntimeEnvironment } from 'hardhat/hre';
import plugin from '@fhevm/hardhat-plugin';

test('the installed plugin registers the hello task', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  assert.notEqual(hre.tasks.getTask('hello'), undefined);
});
