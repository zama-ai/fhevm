import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { deployCompFixture } from './Comp.fixture';

describe.skip('Comp', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const contract = await deployCompFixture();
    this.contractAddress = await contract.getAddress();
    this.comp = contract;
    this.instances = await createInstances(this.contractAddress, ethers, this.signers);
  });

  it('should transfer tokens', async function () {
    const encryptedAmountToTransfer = this.instances.alice.encrypt64(200000);
    const transferTransac = await this.comp['transfer(address,bytes)'](
      this.signers.bob.address,
      encryptedAmountToTransfer,
    );

    await transferTransac.wait();

    const aliceToken = this.instances.alice.getPublicKey(this.contractAddress) || {
      signature: '',
      publicKey: '',
    };
    const encryptedAliceBalance = await this.comp.balanceOf(aliceToken.publicKey, aliceToken.signature);
    // Decrypt Alice's balance
    const aliceBalance = this.instances.alice.decrypt(this.contractAddress, encryptedAliceBalance);
    expect(aliceBalance).to.equal(800000);

    const bobToken = this.instances.bob.getPublicKey(this.contractAddress) || {
      signature: '',
      publicKey: '',
    };
    const encryptedBobBalance = await this.comp
      .connect(this.signers.bob)
      .balanceOf(bobToken.publicKey, bobToken.signature);
    // Decrypt Bob's balance
    const bobBalance = this.instances.bob.decrypt(this.contractAddress, encryptedBobBalance);
    expect(bobBalance).to.equal(200000);
  });
});
