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
    const list = await this.identifiedErc20.identifiers();
    await list.reduce(async (p, identifier) => {
      return p.then(async () => {
        const txs = await Promise.all([
          this.identity.connect(this.signers.alice).grantAccess(this.contractAddress, identifier),
          this.identity.connect(this.signers.bob).grantAccess(this.contractAddress, identifier),
          this.identity.connect(this.signers.carol).grantAccess(this.contractAddress, identifier),
          this.identity.connect(this.signers.dave).grantAccess(this.contractAddress, identifier),
        ]);

        await Promise.all(txs.map((tx) => tx.wait()));
      });
    }, Promise.resolve());

    const txIssuer = await this.identity.connect(this.signers.bob).grantAccess(this.contractAddress, 'issuer');
    await txIssuer.wait();

    const amount20k = this.instances.alice.encrypt32(20000);
    const amount10k = this.instances.alice.encrypt32(10000);

    const encryptedAmount = this.instances.alice.encrypt32(100000);
    const transaction = await this.identifiedErc20.mint(encryptedAmount);
    await transaction.wait();

    const txT1 = await this.identifiedErc20['transfer(address,bytes)'](this.signers.carol, amount20k);
    const txT2 = await this.identifiedErc20['transfer(address,bytes)'](this.signers.dave, amount10k);
    await Promise.all([txT1.wait(), txT2.wait()]);

    // // Call the method
    // const token = this.instances.bob.getTokenSignature(this.contractAddress) || {
    //   signature: '',
    //   publicKey: '',
    // };

    // const encryptedBalance = await this.identifiedErc20
    //   .connect(this.signers.bob)
    //   ['balanceOf(address,bytes32,bytes)'](this.signers.carol, token.publicKey, token.signature);

    // // Decrypt the balance
    // const balance = this.instances.bob.decrypt(this.contractAddress, encryptedBalance);
    // expect(balance).to.equal(20000);

    // const daveBalance = this.identifiedErc20
    //   .connect(this.signers.bob)
    //   ['balanceOf(address,bytes32,bytes)'](this.signers.dave.address, token.publicKey, token.signature);

    // expect(daveBalance).to.throw;

    // const carolToken = this.instances.carol.getTokenSignature(this.contractAddress) || {
    //   signature: '',
    //   publicKey: '',
    // };

    // // It must throw since Carol is not owner of the identity contract
    // const bobBalance = this.identifiedErc20
    //   .connect(this.signers.carol)
    //   ['balanceOf(address,bytes32,bytes)'](this.signers.bob.address, carolToken.publicKey, carolToken.signature);

    // expect(bobBalance).to.throw;
  });

  it('should prevent transfers', async function () {
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
    const list = await this.identifiedErc20.identifiers();
    await list.reduce(async (p, identifier) => {
      return p.then(async () => {
        const txs = await Promise.all([
          this.identity.connect(this.signers.alice).grantAccess(this.contractAddress, identifier),
          this.identity.connect(this.signers.bob).grantAccess(this.contractAddress, identifier),
          this.identity.connect(this.signers.carol).grantAccess(this.contractAddress, identifier),
          this.identity.connect(this.signers.dave).grantAccess(this.contractAddress, identifier),
        ]);

        await Promise.all(txs.map((tx) => tx.wait()));
      });
    }, Promise.resolve());

    const encryptedAmount = this.instances.alice.encrypt32(100000);
    const transaction = await this.identifiedErc20.mint(encryptedAmount);
    await transaction.wait();

    const amount20k = this.instances.alice.encrypt32(20000);
    const amount10k = this.instances.alice.encrypt32(10000);
    const amount3k = this.instances.alice.encrypt32(3000);
    const txT1 = await this.identifiedErc20['transfer(address,bytes)'](this.signers.carol, amount20k);
    // Transmit 20000 tokens to dave is possible since alice is admin
    const txT2 = await this.identifiedErc20['transfer(address,bytes)'](this.signers.dave, amount10k);
    await Promise.all([txT1.wait(), txT2.wait()]);

    const carolToken = this.instances.carol.getTokenSignature(this.contractAddress) || {
      signature: '',
      publicKey: '',
    };

    const daveToken = this.instances.dave.getTokenSignature(this.contractAddress) || {
      signature: '',
      publicKey: '',
    };

    // Check that it's not possible to transfer from country 1 to country 2 an amount above 10k
    const transfer1 = await this.identifiedErc20
      .connect(this.signers.carol)
      ['transfer(address,bytes)'](this.signers.dave, amount20k);
    await transfer1.wait();

    const encryptedBalance = await this.identifiedErc20
      .connect(this.signers.carol)
      ['balanceOf(bytes32,bytes)'](carolToken.publicKey, carolToken.signature);
    const balance = this.instances.carol.decrypt(this.contractAddress, encryptedBalance);
    expect(balance).to.be.equal(20000); // The amount didn't move

    // Check that it's not possible to transfer from country 2 to country 1
    const transfer2 = await this.identifiedErc20
      .connect(this.signers.dave)
      ['transfer(address,bytes)'](this.signers.carol, amount3k);
    await transfer2.wait();

    const encryptedBalance2 = await this.identifiedErc20
      .connect(this.signers.dave)
      ['balanceOf(bytes32,bytes)'](daveToken.publicKey, daveToken.signature);
    const balance2 = this.instances.dave.decrypt(this.contractAddress, encryptedBalance2);
    expect(balance2).to.be.equal(10000); // The amount didn't move

    // Check that it's possible to transfer from country 1 to country 2 an amount below 10k
    const transfer3 = await this.identifiedErc20
      .connect(this.signers.carol)
      ['transfer(address,bytes)'](this.signers.dave, amount3k);
    await transfer3.wait();

    const encryptedBalance3 = await this.identifiedErc20
      .connect(this.signers.carol)
      ['balanceOf(bytes32,bytes)'](carolToken.publicKey, carolToken.signature);
    const balance3 = this.instances.carol.decrypt(this.contractAddress, encryptedBalance3);
    expect(balance3).to.be.equal(17000); // The amount moved
  });
});
