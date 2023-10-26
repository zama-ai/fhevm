import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { deployERC20RulesFixture } from './ERC20Rules';
import { deployIdentifiedERC20Fixture } from './identifiedERC20.fixture';
import { deployIdentityFixture } from './identity.fixture';

describe('IdentifiedERC20', function () {
  before(async function () {
    await initSigners(4);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const erc20Rules = await deployERC20RulesFixture();
    const erc20RulesAddress = await erc20Rules.getAddress();
    this.identity = await deployIdentityFixture();
    const identityAddress = await this.identity.getAddress();
    const contract = await deployIdentifiedERC20Fixture(identityAddress, erc20RulesAddress);
    this.contractAddress = await contract.getAddress();
    this.identifiedErc20 = contract;
    this.instances = await createInstances(this.contractAddress, ethers, this.signers);
  });

  it('should allow decryption of balance for identity owner', async function () {
    // Create accounts;
    const country1 = this.instances.alice.encrypt8(1);
    const country2 = this.instances.alice.encrypt8(2);
    const notIssuer = this.instances.alice.encrypt8(0);
    const issuer = this.instances.alice.encrypt8(1);
    // Alice => owner
    // Bob => Issuer
    // Carol & Dave => User
    const tx1 = await this.identity.addDid(this.signers.alice, country1, notIssuer);
    const tx2 = await this.identity.addDid(this.signers.bob, country1, issuer);
    const tx3 = await this.identity.addDid(this.signers.carol, country1, notIssuer);
    const tx4 = await this.identity.addDid(this.signers.dave, country2, notIssuer);
    await Promise.all([tx1.wait(), tx2.wait(), tx3.wait(), tx4.wait()]);

    // Give permissions

    const txP1 = await this.identity.connect(this.signers.alice).givePermission('country', this.contractAddress);
    const txP2issuer = await this.identity.connect(this.signers.bob).givePermission('issuer', this.contractAddress);
    const txP2 = await this.identity.connect(this.signers.bob).givePermission('country', this.contractAddress);
    const txP3 = await this.identity.connect(this.signers.carol).givePermission('country', this.contractAddress);
    const txP4 = await this.identity.connect(this.signers.dave).givePermission('country', this.contractAddress);
    await Promise.all([txP2issuer.wait(), txP1.wait(), txP2.wait(), txP3.wait(), txP4.wait()]);

    const txB1 = await this.identity.connect(this.signers.alice).givePermission('blacklist', this.contractAddress);
    const txB2 = await this.identity.connect(this.signers.bob).givePermission('blacklist', this.contractAddress);
    const txB3 = await this.identity.connect(this.signers.carol).givePermission('blacklist', this.contractAddress);
    const txB4 = await this.identity.connect(this.signers.dave).givePermission('blacklist', this.contractAddress);
    await Promise.all([txB1.wait(), txB2.wait(), txB3.wait(), txB4.wait()]);

    const encryptedAmount = this.instances.alice.encrypt32(100000);
    const transaction = await this.identifiedErc20.mint(encryptedAmount);
    await transaction.wait();

    const encryptedTransferAmount = this.instances.alice.encrypt32(20000);
    const txT1 = await this.identifiedErc20['transfer(address,bytes)'](this.signers.carol, encryptedTransferAmount);
    // Transmit 20000 tokens to dave is possible since alice is admin
    const txT2 = await this.identifiedErc20['transfer(address,bytes)'](this.signers.dave, encryptedTransferAmount);
    await Promise.all([txT1.wait(), txT2.wait()]);

    // Call the method
    const token = this.instances.bob.getTokenSignature(this.contractAddress) || {
      signature: '',
      publicKey: '',
    };

    const encryptedBalance = await this.identifiedErc20
      .connect(this.signers.bob)
      ['balanceOf(address,bytes32,bytes)'](this.signers.carol, token.publicKey, token.signature);

    // Decrypt the balance
    const balance = this.instances.bob.decrypt(this.contractAddress, encryptedBalance);
    expect(balance).to.equal(20000);

    const daveBalance = this.identifiedErc20
      .connect(this.signers.bob)
      ['balanceOf(address,bytes32,bytes)'](this.signers.dave.address, token.publicKey, token.signature);

    expect(daveBalance).to.throw;

    const carolToken = this.instances.carol.getTokenSignature(this.contractAddress) || {
      signature: '',
      publicKey: '',
    };

    // It must throw since Carol is not owner of the identity contract
    const bobBalance = this.identifiedErc20
      .connect(this.signers.carol)
      ['balanceOf(address,bytes32,bytes)'](this.signers.bob.address, carolToken.publicKey, carolToken.signature);

    expect(bobBalance).to.throw;
  });

  it('should allow decryption of balance for identity owner', async function () {
    // Create accounts;
    const country1 = this.instances.alice.encrypt8(1);
    const country2 = this.instances.alice.encrypt8(2);
    const notIssuer = this.instances.alice.encrypt8(0);
    const issuer = this.instances.alice.encrypt8(1);
    // Alice => owner
    // Bob => Issuer
    // Carol & Dave => User
    const tx1 = await this.identity.addDid(this.signers.alice, country1, notIssuer);
    const tx2 = await this.identity.addDid(this.signers.bob, country1, issuer);
    const tx3 = await this.identity.addDid(this.signers.carol, country1, notIssuer);
    const tx4 = await this.identity.addDid(this.signers.dave, country2, notIssuer);
    await Promise.all([tx1.wait(), tx2.wait(), tx3.wait(), tx4.wait()]);

    // Give permissions

    const txP1 = await this.identity.connect(this.signers.alice).givePermission('country', this.contractAddress);
    const txP2issuer = await this.identity.connect(this.signers.bob).givePermission('issuer', this.contractAddress);
    const txP2 = await this.identity.connect(this.signers.bob).givePermission('country', this.contractAddress);
    const txP3 = await this.identity.connect(this.signers.carol).givePermission('country', this.contractAddress);
    const txP4 = await this.identity.connect(this.signers.dave).givePermission('country', this.contractAddress);
    await Promise.all([txP2issuer.wait(), txP1.wait(), txP2.wait(), txP3.wait(), txP4.wait()]);

    const txB1 = await this.identity.connect(this.signers.alice).givePermission('blacklist', this.contractAddress);
    const txB2 = await this.identity.connect(this.signers.bob).givePermission('blacklist', this.contractAddress);
    const txB3 = await this.identity.connect(this.signers.carol).givePermission('blacklist', this.contractAddress);
    const txB4 = await this.identity.connect(this.signers.dave).givePermission('blacklist', this.contractAddress);
    await Promise.all([txB1.wait(), txB2.wait(), txB3.wait(), txB4.wait()]);

    const encryptedAmount = this.instances.alice.encrypt32(100000);
    const transaction = await this.identifiedErc20.mint(encryptedAmount);
    await transaction.wait();

    const encryptedTransferAmount = this.instances.alice.encrypt32(20000);
    const txT1 = await this.identifiedErc20['transfer(address,bytes)'](this.signers.carol, encryptedTransferAmount);
    // Transmit 20000 tokens to dave is possible since alice is admin
    const txT2 = await this.identifiedErc20['transfer(address,bytes)'](this.signers.dave, encryptedTransferAmount);
    await Promise.all([txT1.wait(), txT2.wait()]);

    // Carol try to transfer 150000 to dave

    const transfer1 = await this.identifiedErc20
      .connect(this.signers.carol)
      ['transfer(address,bytes)'](this.signers.dave, encryptedTransferAmount);
    await transfer1.wait();

    const token = this.instances.carol.getTokenSignature(this.contractAddress) || {
      signature: '',
      publicKey: '',
    };

    const encryptedBalance = await this.identifiedErc20
      .connect(this.signers.carol)
      ['balanceOf(bytes32,bytes)'](token.publicKey, token.signature);
    const balance = this.instances.carol.decrypt(this.contractAddress, encryptedBalance);
    expect(balance).to.be.equal(20000); // The amount didn't move

    const encryptedTransferAmount2 = this.instances.alice.encrypt32(3000);
    const transfer2 = await this.identifiedErc20
      .connect(this.signers.carol)
      ['transfer(address,bytes)'](this.signers.dave, encryptedTransferAmount2);
    await transfer2.wait();

    const encryptedBalance2 = await this.identifiedErc20
      .connect(this.signers.carol)
      ['balanceOf(bytes32,bytes)'](token.publicKey, token.signature);
    const balance2 = this.instances.carol.decrypt(this.contractAddress, encryptedBalance2);
    expect(balance2).to.be.equal(17000); // The amount didn't move
  });
});
