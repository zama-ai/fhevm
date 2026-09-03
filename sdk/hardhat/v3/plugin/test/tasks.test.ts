// E1: the fhevm tasks parse their arguments by name and reach the connection's fhevm object. The
// success paths need a consumer contract and live in the e2e; here the guards and the ACL refusal.
//
// Tests import the BUILT payload (pkg/_esm); see connection.test.ts.

import assert from 'node:assert/strict';
import test from 'node:test';

import { createHardhatRuntimeEnvironment } from 'hardhat/hre';
import { HardhatPluginError } from 'hardhat/plugins';

import plugin, { FhevmType } from '../pkg/_esm/index.js';

const CONTRACT = '0x1111111111111111111111111111111111111111';
const pluginError = (fragment: string) => (e: unknown) =>
  e instanceof HardhatPluginError && e.message.includes(fragment);

void test('the decrypt tasks refuse bad arguments by name, and the ACL refuses an unallowed handle', async () => {
  const hre = await createHardhatRuntimeEnvironment({ plugins: [plugin] });
  const connection = await hre.network.getOrCreate();
  try {
    const publicDecrypt = hre.tasks.getTask(['fhevm', 'public-decrypt']);
    const userDecrypt = hre.tasks.getTask(['fhevm', 'user-decrypt']);
    const { externalEuint } = await connection.fhevm.encryptUint(FhevmType.euint32, 7, CONTRACT, CONTRACT);

    await assert.rejects(
      publicDecrypt.run({ type: 'euint7', handle: externalEuint }),
      pluginError("type name 'euint7'"),
    );
    await assert.rejects(publicDecrypt.run({ type: 'euint32', handle: '0x1234' }), pluginError('Invalid handle'));
    await assert.rejects(publicDecrypt.run({ type: 'euint32', handle: externalEuint }));

    await assert.rejects(
      userDecrypt.run({ type: 'euint32', handle: externalEuint, contract: '0xnope', user: 0 }),
      pluginError('Invalid contract'),
    );
    await assert.rejects(
      userDecrypt.run({ type: 'euint32', handle: externalEuint, contract: CONTRACT, user: 99 }),
      pluginError("Invalid --user '99'"),
    );
    // Account #0 signs a permit the stack accepts; the ACL then refuses the unallowed handle.
    await assert.rejects(
      userDecrypt.run({ type: 'euint32', handle: externalEuint, contract: CONTRACT, user: 0 }),
      (e: unknown) => !(e instanceof HardhatPluginError),
    );
  } finally {
    await connection.close();
  }
});
