import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstance, createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { deployERC20RulesFixture } from './ERC20Rules';
import { deployIdentifiedERC20Fixture } from './identifiedERC20.fixture';
import { deployIdentityRegistryFixture } from './identityRegistry.fixture';

const WALLET_COUNTRY1_PK = 'e3d2a61080fc3a972e5744e59f083f243018271b3070732c1edf1eb2593ac580';

describe('IdentifiedERC20', function () {
  before(async function () {
    await initSigners(4);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const erc20Rules = await deployERC20RulesFixture();
    const erc20RulesAddress = await erc20Rules.getAddress();
    this.identityRegistry = await deployIdentityRegistryFixture();
    const identityAddress = await this.identityRegistry.getAddress();
    const contract = await deployIdentifiedERC20Fixture(identityAddress, erc20RulesAddress);
    this.contractAddress = await contract.getAddress();
    this.identifiedErc20 = contract;
    this.instances = await createInstances(this.contractAddress, ethers, this.signers);
  });

  it.only('should allow decryption of balance for identity owner', async function () {
    // Create accounts;
    const country1 = this.instances.alice.encrypt32(1);
    const country2 = this.instances.alice.encrypt32(2);
    // Alice => owner
    // Bob => Issuer
    // Carol & Dave => User
    const bobIssuerTx = await this.identityRegistry.addRegistrar(this.signers.bob, 1);
    await bobIssuerTx.wait();
    const tx1 = await this.identityRegistry.connect(this.signers.bob).addDid(this.signers.alice);
    const tx2 = await this.identityRegistry.connect(this.signers.bob).addDid(this.signers.bob);
    const tx3 = await this.identityRegistry.connect(this.signers.bob).addDid(this.signers.carol);
    const tx4 = await this.identityRegistry.connect(this.signers.bob).addDid(this.signers.dave);
    await Promise.all([tx1.wait(), tx2.wait(), tx3.wait(), tx4.wait()]);

    const tx1Identifier = await this.identityRegistry
      .connect(this.signers.bob)
      .setIdentifier(this.signers.alice, 'country', country1);
    const tx2Identifier = await this.identityRegistry
      .connect(this.signers.bob)
      .setIdentifier(this.signers.bob, 'country', country1);
    const tx3Identifier = await this.identityRegistry
      .connect(this.signers.bob)
      .setIdentifier(this.signers.carol, 'country', country1);
    const tx4Identifier = await this.identityRegistry
      .connect(this.signers.bob)
      .setIdentifier(this.signers.dave, 'country', country2);
    await Promise.all([tx1Identifier.wait(), tx2Identifier.wait(), tx3Identifier.wait(), tx4Identifier.wait()]);

    // Give permissions
    const list = await this.identifiedErc20.identifiers();
    await list.reduce(async (p, identifier) => {
      return p.then(async () => {
        const txs = await Promise.all([
          this.identityRegistry.connect(this.signers.alice).grantAccess(this.contractAddress, identifier),
          this.identityRegistry.connect(this.signers.bob).grantAccess(this.contractAddress, identifier),
          this.identityRegistry.connect(this.signers.carol).grantAccess(this.contractAddress, identifier),
          this.identityRegistry.connect(this.signers.dave).grantAccess(this.contractAddress, identifier),
        ]);

        await Promise.all(txs.map((tx) => tx.wait()));
      });
    }, Promise.resolve());

    const txIssuer = await this.identityRegistry.connect(this.signers.bob).grantAccess(this.contractAddress, 'issuer');
    await txIssuer.wait();

    const amount20k = this.instances.alice.encrypt32(20000);
    const amount10k = this.instances.alice.encrypt32(10000);

    const encryptedAmount = this.instances.alice.encrypt32(100_000);
    const transaction = await this.identifiedErc20.mint(encryptedAmount);
    await transaction.wait();

    const txT1 = await this.identifiedErc20['transfer(address,bytes)'](this.signers.carol, amount20k);
    const txT2 = await this.identifiedErc20['transfer(address,bytes)'](this.signers.dave, amount10k);
    await Promise.all([txT1.wait(), txT2.wait()]);

    const country1Admin = new ethers.Wallet(WALLET_COUNTRY1_PK).connect(ethers.provider);
    console.log(country1Admin.address);
    const country1Instance = await createInstance(this.contractAddress, country1Admin, ethers);
    const token = country1Instance.getTokenSignature(this.contractAddress) || {
      signature: '',
      publicKey: '',
    };

    const encryptedAliceBalance = await this.identifiedErc20
      .connect(country1Admin)
      .balanceOfUser(this.signers.alice, token.publicKey, token.signature);

    // // Decrypt the balance
    const aliceBalance = country1Instance.decrypt(this.contractAddress, encryptedAliceBalance);
    expect(aliceBalance).to.equal(70000);

    const encryptedCarolBalance = await this.identifiedErc20
      .connect(country1Admin)
      .balanceOfUser(this.signers.carol, token.publicKey, token.signature);

    // // Decrypt the balance
    const carolBalance = country1Instance.decrypt(this.contractAddress, encryptedCarolBalance);
    expect(carolBalance).to.equal(20000);

    const encryptedDaveBalance = this.identifiedErc20
      .connect(country1Admin)
      .balanceOfUser(this.signers.carol, token.publicKey, token.signature);

    expect(encryptedDaveBalance).to.throw;

    const carolToken = this.instances.carol.getTokenSignature(this.contractAddress) || {
      signature: '',
      publicKey: '',
    };

    // It must throw since Carol is not owner of the identity contract
    const encryptedBobBalance2 = this.identifiedErc20
      .connect(this.signers.carol)
      .balanceOfUser(this.signers.bob.address, carolToken.publicKey, carolToken.signature);

    expect(encryptedBobBalance2).to.throw;
  });

  it('should prevent transfers', async function () {
    // Create accounts;
    const country1 = this.instances.alice.encrypt32(1);
    const country2 = this.instances.alice.encrypt32(2);
    // Alice => owner
    // Bob => Issuer
    // Carol & Dave => User
    const bobIssuerTx = await this.identityRegistry.addRegistrar(this.signers.bob, 1);
    await bobIssuerTx.wait();
    const tx1 = await this.identityRegistry.connect(this.signers.bob).addDid(this.signers.alice);
    const tx2 = await this.identityRegistry.connect(this.signers.bob).addDid(this.signers.bob);
    const tx3 = await this.identityRegistry.connect(this.signers.bob).addDid(this.signers.carol);
    const tx4 = await this.identityRegistry.connect(this.signers.bob).addDid(this.signers.dave);
    await Promise.all([tx1.wait(), tx2.wait(), tx3.wait(), tx4.wait()]);

    const tx1Identifier = await this.identityRegistry
      .connect(this.signers.bob)
      .setIdentifier(this.signers.alice, 'country', country1);
    const tx2Identifier = await this.identityRegistry
      .connect(this.signers.bob)
      .setIdentifier(this.signers.bob, 'country', country1);
    const tx3Identifier = await this.identityRegistry
      .connect(this.signers.bob)
      .setIdentifier(this.signers.carol, 'country', country1);
    const tx4Identifier = await this.identityRegistry
      .connect(this.signers.bob)
      .setIdentifier(this.signers.dave, 'country', country2);
    await Promise.all([tx1Identifier.wait(), tx2Identifier.wait(), tx3Identifier.wait(), tx4Identifier.wait()]);

    // Give permissions
    const list = await this.identifiedErc20.identifiers();
    await list.reduce(async (p, identifier) => {
      return p.then(async () => {
        const txs = await Promise.all([
          this.identityRegistry.connect(this.signers.alice).grantAccess(this.contractAddress, identifier),
          this.identityRegistry.connect(this.signers.bob).grantAccess(this.contractAddress, identifier),
          this.identityRegistry.connect(this.signers.carol).grantAccess(this.contractAddress, identifier),
          this.identityRegistry.connect(this.signers.dave).grantAccess(this.contractAddress, identifier),
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
      .balanceOf(carolToken.publicKey, carolToken.signature);
    const balance = this.instances.carol.decrypt(this.contractAddress, encryptedBalance);
    expect(balance).to.be.equal(20000); // The amount didn't move

    // Check that it's not possible to transfer from country 2 to country 1
    const transfer2 = await this.identifiedErc20
      .connect(this.signers.dave)
      ['transfer(address,bytes)'](this.signers.carol, amount3k);
    await transfer2.wait();

    const encryptedBalance2 = await this.identifiedErc20
      .connect(this.signers.dave)
      .balanceOf(daveToken.publicKey, daveToken.signature);
    const balance2 = this.instances.dave.decrypt(this.contractAddress, encryptedBalance2);
    expect(balance2).to.be.equal(10000); // The amount didn't move

    // Check that it's possible to transfer from country 1 to country 2 an amount below 10k
    const transfer3 = await this.identifiedErc20
      .connect(this.signers.carol)
      ['transfer(address,bytes)'](this.signers.dave, amount3k);
    await transfer3.wait();

    const encryptedBalance3 = await this.identifiedErc20
      .connect(this.signers.carol)
      .balanceOf(carolToken.publicKey, carolToken.signature);
    const balance3 = this.instances.carol.decrypt(this.contractAddress, encryptedBalance3);
    expect(balance3).to.be.equal(17000); // The amount moved
  });
});
