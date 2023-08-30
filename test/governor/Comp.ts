import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners } from '../signers';
import { deployCompFixture } from './Comp.fixture';

describe('Comp', function () {
  before(async function () {
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const contract = await deployCompFixture();
    this.contractAddress = await contract.getAddress();
    this.comp = contract;
    this.instances = await createInstances(this.contractAddress, ethers, this.signers);
  });

  it('should init supply', async function () {
    const encryptedAmount = this.instances.alice.encrypt32(1000);
    const transaction = await this.comp.initSupply(encryptedAmount);
    await transaction.wait();
    // Call the method
    const token = this.instances.alice.getTokenSignature(this.contractAddress) || {
      signature: '',
      publicKey: '',
    };

    const encryptedBalance = await this.comp.balanceOf(token.publicKey, token.signature);
    // Decrypt the balance
    const balance = this.instances.alice.decrypt(this.contractAddress, encryptedBalance);
    expect(balance).to.equal(1000);

    const encryptedTotalSupply = await this.comp.getTotalSupply(token.publicKey, token.signature);
    // Decrypt the total supply
    const totalSupply = this.instances.alice.decrypt(this.contractAddress, encryptedTotalSupply);
    expect(totalSupply).to.equal(1000);
  });

  it('should transfer tokens', async function () {
    const encryptedAmount = this.instances.alice.encrypt32(1000);
    const supplyTransac = await this.comp.initSupply(encryptedAmount);

    const encryptedAmountToTransfer = this.instances.alice.encrypt32(200);
    const transferTransac = await this.comp['transfer(address,bytes)'](
      this.signers.bob.address,
      encryptedAmountToTransfer,
    );

    await supplyTransac.wait();
    await transferTransac.wait();

    const aliceToken = this.instances.alice.getTokenSignature(this.contractAddress) || {
      signature: '',
      publicKey: '',
    };
    const encryptedAliceBalance = await this.comp.balanceOf(aliceToken.publicKey, aliceToken.signature);
    // Decrypt Alice's balance
    const aliceBalance = this.instances.alice.decrypt(this.contractAddress, encryptedAliceBalance);
    expect(aliceBalance).to.equal(800);

    const bobToken = this.instances.bob.getTokenSignature(this.contractAddress) || {
      signature: '',
      publicKey: '',
    };
    const encryptedBobBalance = await this.comp
      .connect(this.signers.bob)
      .balanceOf(bobToken.publicKey, bobToken.signature);
    // Decrypt Bob's balance
    const bobBalance = this.instances.bob.decrypt(this.contractAddress, encryptedBobBalance);
    expect(bobBalance).to.equal(200);
  });
});
