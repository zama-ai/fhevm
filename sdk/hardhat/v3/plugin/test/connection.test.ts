// The per-connection contract: every connection carries its OWN fhevm object, attached by the
// network hooks — the hardhat 3 replacement for v2's process-wide `hre.fhevm` singleton.
//
// Tests import the BUILT payload (pkg/_esm), not the sources: the plugin's lazy imports use `.js`
// specifiers a source-run cannot resolve, and the built form is what a consumer loads anyway. The
// Makefile orders compile before test.

import assert from 'node:assert/strict';
import test from 'node:test';

import { createHardhatRuntimeEnvironment } from 'hardhat/hre';

import plugin from '../pkg/_esm/index.js';

void test('every connection carries a frozen fhevm object of its own', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });

  const first = await hre.network.create();
  const second = await hre.network.create();
  try {
    assert.notEqual(first.fhevm, undefined, 'newConnection must attach connection.fhevm');
    assert.notEqual(second.fhevm, undefined);
    assert.notEqual(first.fhevm, second.fhevm, 'connections must not share fhevm state');
    assert.ok(Object.isFrozen(first.fhevm), 'the fhevm object is frozen');
    // The in-process EDR chain is a cleartext development node.
    assert.equal(first.fhevm.network.kind, 'hardhat');
    assert.equal(first.fhevm.isCleartext, true);
    assert.equal(first.fhevm.isDevelopment, true);
    // eslint-disable-next-line @typescript-eslint/no-deprecated -- the alias is the thing under test
    assert.equal(first.fhevm.isMock, first.fhevm.isCleartext);
  } finally {
    await first.close();
    await second.close();
  }
});

void test('closing a connection is clean with the fhevm hooks installed', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  assert.notEqual(connection.fhevm, undefined);
  await connection.close();
});

// Decided: connection-only, the pattern hardhat 3's own plugins follow. A regression here would
// mean someone re-introduced a process-wide fhevm object on the HRE.
void test('the HRE carries no fhevm property; the connection is the only home', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  assert.equal('fhevm' in hre, false, 'hre.fhevm must not exist');

  const connection = await hre.network.create();
  try {
    assert.equal('fhevm' in connection, true);
    assert.equal('fhevm' in hre, false, 'creating a connection must not attach fhevm to the HRE');
  } finally {
    await connection.close();
  }
});
