// D3b guards: the zero handle, a bad contract address and a wallet client without an account fail by
// name before any permit is signed; a local account signs a permit the stack accepts, and the decrypt
// then fails on the ACL (nothing allowed this user), which proves the signing path. The success path
// needs a contract that calls `FHE.allow`, so it lives in the e2e counter test.
//
// Tests import the BUILT payload (pkg/_esm); see connection.test.ts.

import assert from 'node:assert/strict';
import test from 'node:test';

import { createHardhatRuntimeEnvironment } from 'hardhat/hre';
import { HardhatPluginError } from 'hardhat/plugins';
import { createWalletClient, custom } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';

import plugin, { FhevmType, timestampNow } from '#esm/index.js';

const ZERO_HANDLE = `0x${'0'.repeat(64)}` as const;
const CONTRACT = '0x1111111111111111111111111111111111111111';
// hardhat's account #0
const ALICE = privateKeyToAccount('0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80');

function pluginError(fragment: string): (e: unknown) => boolean {
  return (e: unknown) => e instanceof HardhatPluginError && e.message.includes(fragment);
}

void test('user decryption guards fail by name before any permit is signed', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  try {
    const { fhevm } = connection;
    await assert.rejects(
      fhevm.userDecryptEuint(FhevmType.euint32, ZERO_HANDLE, CONTRACT, ALICE),
      pluginError('not initialized'),
    );
    const { externalEuint } = await fhevm.encryptUint(FhevmType.euint32, 7, CONTRACT, ALICE.address);
    await assert.rejects(fhevm.userDecryptEbool(externalEuint, '0xnope', ALICE), pluginError("'contractAddress'"));
    const noAccount = createWalletClient({ transport: custom(connection.provider) });
    await assert.rejects(
      fhevm.userDecryptEaddress(externalEuint, CONTRACT, noAccount),
      pluginError('carries no account'),
    );
    await assert.rejects(
      fhevm.userDecryptEuint(FhevmType.euint32, externalEuint, CONTRACT, ALICE, { delegatorAddress: '0xbad' }),
      pluginError("'delegatorAddress'"),
    );
  } finally {
    await connection.close();
  }
});

void test('a local account signs the permit; the ACL then refuses a handle nobody allowed', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.create();
  try {
    const { fhevm } = connection;
    const { externalEuint } = await fhevm.encryptUint(FhevmType.euint32, 7, CONTRACT, ALICE.address);
    const validity = { startTimestamp: timestampNow(), durationDays: 1 };
    await assert.rejects(
      fhevm.userDecryptEuint(FhevmType.euint32, externalEuint, CONTRACT, ALICE, { validity }),
      (e: unknown) => !(e instanceof HardhatPluginError),
    );
  } finally {
    await connection.close();
  }
});

void test('timestampNow is in seconds', () => {
  const now = timestampNow();
  assert.ok(Math.abs(now - Date.now() / 1000) < 2);
});
