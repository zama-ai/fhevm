// The public surface contract: every member of `HardhatFhevmRuntimeEnvironment` is live on
// `connection.fhevm` — nothing is stubbed any more — and the module exports are what v2 shipped.
//
// Tests import the BUILT payload (pkg/_esm); see connection.test.ts.

import assert from 'node:assert/strict';
import test from 'node:test';

import { createHardhatRuntimeEnvironment } from 'hardhat/hre';

import plugin, { FhevmType, getHCU, timestampNow } from '#esm/index.js';
import type { HardhatFhevmRuntimeEnvironment } from '#esm/index.js';

const METHODS = [
  'typeof',
  'parseCoprocessorEvents',
  'computeTransactionHCU',
  'assertCoprocessorInitialized',
  'getCoprocessorConfig',
  'revertedWithCustomErrorArgs',
  'tryParseFhevmError',
  'createEncryptedInput',
  'encryptUint',
  'encryptBool',
  'encryptAddress',
  'publicDecrypt',
  'publicDecryptEbool',
  'publicDecryptEuint',
  'publicDecryptEaddress',
  'userDecryptEbool',
  'userDecryptEuint',
  'userDecryptEaddress',
] as const satisfies ReadonlyArray<keyof HardhatFhevmRuntimeEnvironment>;

const DEBUGGER_METHODS = ['decryptEbool', 'decryptEuint', 'decryptEaddress'] as const;

void test('connection.fhevm exposes the whole public surface, every member live', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  try {
    const fhevm: HardhatFhevmRuntimeEnvironment = connection.fhevm;
    for (const member of METHODS) assert.equal(typeof fhevm[member], 'function', `${member} is a method`);
    for (const member of DEBUGGER_METHODS) {
      assert.equal(typeof fhevm.debugger[member], 'function', `debugger.${member} is a method`);
    }
    assert.equal(fhevm.network.chainId, 31337);
    assert.equal(fhevm.isDevelopment, true);
    assert.equal(fhevm.isCleartext, true);
    assert.equal(fhevm.client.chain.id, 31337);
  } finally {
    await connection.close();
  }
});

void test('the module exports are the v2 ones', () => {
  assert.equal(typeof getHCU, 'function');
  assert.equal(typeof timestampNow, 'function');
  assert.equal(FhevmType.ebool, 0);
  assert.equal(FhevmType.euint32, 4);
  assert.equal(FhevmType.eaddress, 7);
  assert.equal(FhevmType.euint256, 8);
  assert.equal(FhevmType[4], 'euint32');
});
