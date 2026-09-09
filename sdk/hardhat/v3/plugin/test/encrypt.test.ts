// D1: encryption over the cleartext client of an in-process development connection — the batched
// builder and the three singular helpers, plus the guards that fail by name before any RPC.
//
// Tests import the BUILT payload (pkg/_esm); see connection.test.ts.

import assert from 'node:assert/strict';
import test from 'node:test';

import { createHardhatRuntimeEnvironment } from 'hardhat/hre';
import { HardhatPluginError } from 'hardhat/plugins';
import { isHex, size } from 'viem';

import plugin, { FhevmType } from '#esm/index.js';

const CONTRACT = '0x1111111111111111111111111111111111111111';
const USER = '0x2222222222222222222222222222222222222222';

function isBytes32(value: unknown): boolean {
  return typeof value === 'string' && isHex(value) && size(value) === 32;
}

function pluginError(fragment: string): (e: unknown) => boolean {
  return (e: unknown) => e instanceof HardhatPluginError && e.message.includes(fragment);
}

void test('createEncryptedInput batches several values under one input proof', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  try {
    const input = connection.fhevm.createEncryptedInput(CONTRACT, USER);
    assert.equal(input.contractAddress, CONTRACT);
    assert.equal(input.userAddress, USER);
    const { handles, inputProof } = await input.addBool(true).add32(123).add64(1n).addAddress(USER).encrypt();
    assert.equal(handles.length, 4);
    for (const handle of handles) assert.ok(isBytes32(handle), `handle ${handle} is bytes32`);
    assert.ok(isHex(inputProof) && size(inputProof) > 0, 'one non-empty input proof');
  } finally {
    await connection.close();
  }
});

void test('the singular helpers return one handle each, keyed by type', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  try {
    const { fhevm } = connection;
    const uint = await fhevm.encryptUint(FhevmType.euint32, 42, CONTRACT, USER);
    assert.ok(isBytes32(uint.externalEuint));
    assert.ok(isHex(uint.inputProof));
    const bool = await fhevm.encryptBool(false, CONTRACT, USER);
    assert.ok(isBytes32(bool.externalEbool));
    const address = await fhevm.encryptAddress(USER, CONTRACT, USER);
    assert.ok(isBytes32(address.externalEaddress));
  } finally {
    await connection.close();
  }
});

void test('encryption guards fail by name before any RPC', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  try {
    const { fhevm } = connection;
    assert.throws(() => fhevm.createEncryptedInput('0xnot-an-address', USER), pluginError("'contractAddress'"));
    assert.throws(() => fhevm.createEncryptedInput(CONTRACT, '0x'), pluginError("'userAddress'"));
    await assert.rejects(fhevm.createEncryptedInput(CONTRACT, USER).encrypt(), pluginError('nothing to encrypt'));
    await assert.rejects(
      fhevm.encryptUint(FhevmType.ebool as unknown as FhevmType.euint8, 1, CONTRACT, USER),
      pluginError('not a valid FhevmTypeEuint'),
    );
    await assert.rejects(fhevm.encryptUint(FhevmType.euint4, 1, CONTRACT, USER), /euint4 is not supported/);
  } finally {
    await connection.close();
  }
});

void test('fhevm.client is the live SDK client on a development connection', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  try {
    assert.equal(connection.fhevm.client.chain.id, 31337);
  } finally {
    await connection.close();
  }
});
