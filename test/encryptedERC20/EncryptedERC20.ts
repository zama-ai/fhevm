import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances, decrypt64 } from '../instance';
import { getSigners, initSigners } from '../signers';
import { deployEncryptedERC20Fixture } from './EncryptedERC20.fixture';

describe('EncryptedERC20', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const contract = await deployEncryptedERC20Fixture();
    this.contractAddress = await contract.getAddress();
    this.erc20 = contract;
    this.instances = await createInstances(this.contractAddress, ethers, this.signers);
  });

  it('should mint the contract', async function () {
    const transaction = await this.erc20.mint(1000);
    await transaction.wait();
    // Call the method
    const token = this.instances.alice.getPublicKey(this.contractAddress) || {
      signature: '',
      publicKey: '',
    };

    const balanceHandle = await this.erc20.balanceOf(this.signers.alice);
    const balance = await decrypt64(balanceHandle);
    expect(balance).to.equal(1000);

    const totalSupply = await this.erc20.totalSupply();
    expect(totalSupply).to.equal(1000);
  });

  it('should transfer tokens between two users', async function () {
    const transaction = await this.erc20.mint(10000);
    await transaction.wait();

    const encryptedTransferAmount = this.instances.alice.encrypt64(1337);
    const tx = await this.erc20['transfer(address,bytes)'](this.signers.bob.address, encryptedTransferAmount);
    await tx.wait();

    // Decrypt Alice's balance
    const balanceHandleAlice = await this.erc20.balanceOf(this.signers.alice);
    const balanceAlice = await decrypt64(balanceHandleAlice);
    expect(balanceAlice).to.equal(10000 - 1337);

    // Decrypt Bob's balance
    const balanceHandleBob = await this.erc20.balanceOf(this.signers.bob);
    const balanceBob = await decrypt64(balanceHandleBob);
    expect(balanceBob).to.equal(1337);
  });

  it('should not transfer tokens between two users', async function () {
    const transaction = await this.erc20.mint(1000);
    await transaction.wait();

    const encryptedTransferAmount = this.instances.alice.encrypt64(1337);
    const tx = await this.erc20['transfer(address,bytes)'](this.signers.bob.address, encryptedTransferAmount);
    await tx.wait();

    // Decrypt Alice's balance
    const balanceHandleAlice = await this.erc20.balanceOf(this.signers.alice);
    const balanceAlice = await decrypt64(balanceHandleAlice);
    expect(balanceAlice).to.equal(1000);

    // Decrypt Bob's balance
    const balanceHandleBob = await this.erc20.balanceOf(this.signers.bob);
    const balanceBob = await decrypt64(balanceHandleBob);
    expect(balanceBob).to.equal(0);
  });

  it('should be able to transferFrom only if allowance is sufficient', async function () {
    const transaction = await this.erc20.mint(10000);
    await transaction.wait();

    const encryptedAllowanceAmount = this.instances.alice.encrypt64(1337);
    const tx = await this.erc20['approve(address,bytes)'](this.signers.bob.address, encryptedAllowanceAmount);
    await tx.wait();

    const bobErc20 = this.erc20.connect(this.signers.bob);
    const encryptedTransferAmount = this.instances.bob.encrypt64(1338); // above allowance so next tx should actually not send any token
    const tx2 = await bobErc20['transferFrom(address,address,bytes)'](
      this.signers.alice.address,
      this.signers.bob.address,
      encryptedTransferAmount,
    );
    await tx2.wait();

    // Decrypt Alice's balance
    const balanceHandleAlice = await this.erc20.balanceOf(this.signers.alice);
    const balanceAlice = await decrypt64(balanceHandleAlice);
    expect(balanceAlice).to.equal(10000); // check that transfer did not happen, as expected

    // Decrypt Bob's balance
    const balanceHandleBob = await this.erc20.balanceOf(this.signers.bob);
    const balanceBob = await decrypt64(balanceHandleBob);
    expect(balanceBob).to.equal(0); // check that transfer did not happen, as expected

    const encryptedTransferAmount2 = this.instances.bob.encrypt64(1337); // below allowance so next tx should send token
    const tx3 = await bobErc20['transferFrom(address,address,bytes)'](
      this.signers.alice.address,
      this.signers.bob.address,
      encryptedTransferAmount2,
    );
    await tx3.wait();

    // Decrypt Alice's balance
    const balanceHandleAlice2 = await this.erc20.balanceOf(this.signers.alice);
    const balanceAlice2 = await decrypt64(balanceHandleAlice2);
    expect(balanceAlice2).to.equal(10000 - 1337); // check that transfer did happen this time

    // Decrypt Bob's balance
    const balanceHandleBob2 = await this.erc20.balanceOf(this.signers.bob);
    const balanceBob2 = await decrypt64(balanceHandleBob2);
    expect(balanceBob2).to.equal(1337); // check that transfer did happen this time
  });
});
