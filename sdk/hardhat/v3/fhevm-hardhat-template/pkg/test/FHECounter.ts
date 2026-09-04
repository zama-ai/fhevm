import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { FhevmType } from '@fhevm/hardhat-plugin-v3';
import { network } from 'hardhat';
import { zeroHash } from 'viem';

void describe('FHECounter', async function () {
  const connection = await network.create();
  const { fhevm, viem } = connection;
  const [alice] = await viem.getWalletClients();

  void it('has an uninitialized encrypted count after deployment', async function () {
    const counter = await viem.deployContract('FHECounter');

    assert.equal(await counter.read.getCount(), zeroHash);
  });

  void it('increments the counter by one', async function () {
    const counter = await viem.deployContract('FHECounter');
    const encryptedOne = await fhevm.createEncryptedInput(counter.address, alice.account.address).add32(1).encrypt();

    await counter.write.increment([encryptedOne.handles[0], encryptedOne.inputProof]);

    const clearCount = await fhevm.userDecryptEuint(
      FhevmType.euint32,
      await counter.read.getCount(),
      counter.address,
      alice,
    );
    assert.equal(clearCount, 1n);
  });

  void it('decrements the counter back to zero', async function () {
    const counter = await viem.deployContract('FHECounter');
    const encryptedOne = await fhevm.createEncryptedInput(counter.address, alice.account.address).add32(1).encrypt();

    await counter.write.increment([encryptedOne.handles[0], encryptedOne.inputProof]);
    await counter.write.decrement([encryptedOne.handles[0], encryptedOne.inputProof]);

    const clearCount = await fhevm.userDecryptEuint(
      FhevmType.euint32,
      await counter.read.getCount(),
      counter.address,
      alice,
    );
    assert.equal(clearCount, 0n);
  });
});
