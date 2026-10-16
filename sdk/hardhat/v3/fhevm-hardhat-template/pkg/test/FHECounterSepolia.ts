import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { FhevmType } from '@fhevm/hardhat-plugin-v3';
import { network } from 'hardhat';
import { getAddress } from 'viem';

void describe('FHECounter on Sepolia', async function () {
  const connection = await network.create();
  const { fhevm, viem } = connection;

  void it('increments the counter by one', { timeout: 160_000 }, async function (testContext) {
    if (fhevm.isCleartext) {
      testContext.skip('This test only runs on a public FHEVM network.');
      return;
    }

    const configuredAddress = process.env.FHECOUNTER_ADDRESS;
    if (configuredAddress === undefined) {
      throw new Error('Set FHECOUNTER_ADDRESS to the deployed contract address before running this test.');
    }

    const counterAddress = getAddress(configuredAddress);
    const [alice] = await viem.getWalletClients();
    const counter = await viem.getContractAt('FHECounter', counterAddress);
    const encryptedZero = await fhevm.createEncryptedInput(counterAddress, alice.account.address).add32(0).encrypt();

    await counter.write.increment([encryptedZero.handles[0], encryptedZero.inputProof]);
    const before = await fhevm.userDecryptEuint(
      FhevmType.euint32,
      await counter.read.getCount(),
      counterAddress,
      alice,
    );

    const encryptedOne = await fhevm.createEncryptedInput(counterAddress, alice.account.address).add32(1).encrypt();
    await counter.write.increment([encryptedOne.handles[0], encryptedOne.inputProof]);
    const after = await fhevm.userDecryptEuint(FhevmType.euint32, await counter.read.getCount(), counterAddress, alice);

    assert.equal(after - before, 1n);
  });
});
