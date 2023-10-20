import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { deployIdentityFixture } from './Identity.fixture';

describe('Identity', function () {
  before(async function () {
    await initSigners(3);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const contract = await deployIdentityFixture();
    this.contractAddress = await contract.getAddress();
    this.identity = contract;
    this.instances = await createInstances(this.contractAddress, ethers, this.signers);
  });

  it('should add identifier', async function () {
    const encryptedIssuer = this.instances.alice.encrypt8(0);
    const country1 = this.instances.alice.encrypt8(1);
    const tx1 = await this.identity.addDid(this.signers.bob, country1, encryptedIssuer);
    await tx1.wait();

    const encryptedBirth = this.instances.alice.encrypt32(495873907);
    const transaction = await this.identity.setIdentifier(this.signers.bob.address, 'birthdate', encryptedBirth);
    await transaction.wait();

    const allowed = await this.identity
      .connect(this.signers.bob)
      .givePermission('birthdate', this.signers.carol.address);
    await allowed.wait();

    // Carol use this token to access information
    const token = this.instances.carol.getTokenSignature(this.contractAddress) || {
      signature: '',
      publicKey: '',
    };

    const encryptedBirthdate = await this.identity
      .connect(this.signers.carol)
      .reencryptIdentifier(this.signers.bob.address, 'birthdate', token.publicKey, token.signature);
    const birthdate = this.instances.carol.decrypt(this.contractAddress, encryptedBirthdate);

    expect(birthdate).to.be.equal(495873907);
  });

  it('should remove kyc', async function () {
    const encryptedIssuer = this.instances.alice.encrypt8(0);
    const country1 = this.instances.alice.encrypt8(1);

    const tx1 = await this.identity.addDid(this.signers.bob, country1, encryptedIssuer);
    await tx1.wait();

    const encryptedBirth = this.instances.alice.encrypt32(495873907);
    const tx2 = await this.identity.setIdentifier(this.signers.bob, 'birthdate', encryptedBirth);
    await tx2.wait();

    const tx3 = await this.identity.removeDid(this.signers.bob);
    await tx3.wait();
  });
});
