// The public surface contract (D0): every member of `HardhatFhevmRuntimeEnvironment` is present on
// `connection.fhevm`, and each one that is not ported yet fails with a NAMED not-implemented error —
// so a v2 test that reaches for a missing group fails on the member name, never on a TypeError.
//
// Tests import the BUILT payload (pkg/_esm); see connection.test.ts.

import assert from 'node:assert/strict';
import test from 'node:test';

import { createHardhatRuntimeEnvironment } from 'hardhat/hre';
import { HardhatPluginError } from 'hardhat/plugins';

import plugin, { FhevmType } from '../pkg/_esm/index.js';
import type { HardhatFhevmRuntimeEnvironment } from '../pkg/_esm/index.js';

const SYNC_STUBS = [
  'typeof',
  'parseCoprocessorEvents',
  'computeTransactionHCU',
  'revertedWithCustomErrorArgs',
  'createEncryptedInput',
  'createEIP712',
  'createDelegatedUserDecryptEIP712',
] as const;

const ASYNC_STUBS = [
  'assertCoprocessorInitialized',
  'getCoprocessorConfig',
  'tryParseFhevmError',
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
] as const;

const STUB_GETTERS = ['debugger', 'client'] as const;

function isNotImplemented(member: string): (e: unknown) => boolean {
  return (e: unknown) => e instanceof HardhatPluginError && e.message.includes(`fhevm.${member} is not implemented`);
}

void test('connection.fhevm exposes the whole public surface; unported members fail by name', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  try {
    const fhevm: HardhatFhevmRuntimeEnvironment = connection.fhevm;

    for (const member of SYNC_STUBS) {
      assert.equal(typeof fhevm[member], 'function', `${member} is a method`);
      assert.throws(() => (fhevm[member] as () => unknown)(), isNotImplemented(member), member);
    }
    for (const member of ASYNC_STUBS) {
      assert.equal(typeof fhevm[member], 'function', `${member} is a method`);
      await assert.rejects((fhevm[member] as () => Promise<unknown>)(), isNotImplemented(member), member);
    }
    for (const member of STUB_GETTERS) {
      assert.throws(() => fhevm[member], isNotImplemented(member), member);
    }
  } finally {
    await connection.close();
  }
});

void test('the live members answer without touching a stub', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  try {
    assert.equal(connection.fhevm.network.chainId, 31337);
    assert.equal(connection.fhevm.isDevelopment, true);
    assert.equal(connection.fhevm.isCleartext, true);
  } finally {
    await connection.close();
  }
});

void test('FhevmType is a runtime enum carrying the on-chain FheType ids', () => {
  assert.equal(FhevmType.ebool, 0);
  assert.equal(FhevmType.euint32, 4);
  assert.equal(FhevmType.eaddress, 7);
  assert.equal(FhevmType.euint256, 8);
  assert.equal(FhevmType[4], 'euint32');
});
