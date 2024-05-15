import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { deployIdentityRegistryFixture } from './identityRegistry.fixture';

describe.skip('Identity', function () {
  before(async function () {
    await initSigners(3);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const contract = await deployIdentityRegistryFixture();
    this.contractAddress = await contract.getAddress();
    this.identityRegistry = contract;
    this.instances = await createInstances(this.contractAddress, ethers, this.signers);
  });

  it('should add identifier', async function () {
    const addRegistrarTx = await this.identityRegistry.addRegistrar(this.signers.alice, 1);
    await addRegistrarTx.wait();

    const tx1 = await this.identityRegistry.addDid(this.signers.bob);
    await tx1.wait();

    const encryptedBirth = this.instances.alice.encrypt64(495873907);
    const transaction = await this.identityRegistry.setIdentifier(
      this.signers.bob.address,
      'birthdate',
      encryptedBirth,
    );
    await transaction.wait();

    const allowed = await this.identityRegistry
      .connect(this.signers.bob)
      .grantAccess(this.signers.carol.address, ['birthdate']);
    await allowed.wait();

    // Carol use this token to access information
    const token = this.instances.carol.getPublicKey(this.contractAddress) || {
      signature: '',
      publicKey: '',
    };

    const encryptedBirthdate = await this.identityRegistry
      .connect(this.signers.carol)
      .reencryptIdentifier(this.signers.bob.address, 'birthdate', token.publicKey, token.signature);
    const birthdate = this.instances.carol.decrypt(this.contractAddress, encryptedBirthdate);

    expect(birthdate).to.be.equal(495873907);
  });

  it('should remove kyc', async function () {
    const addRegistrarTx = await this.identityRegistry.addRegistrar(this.signers.alice, 1);
    await addRegistrarTx.wait();

    const tx1 = await this.identityRegistry.addDid(this.signers.bob);
    await tx1.wait();

    const encryptedBirth = this.instances.alice.encrypt64(495873907);
    const tx2 = await this.identityRegistry.setIdentifier(this.signers.bob, 'birthdate', encryptedBirth);
    await tx2.wait();

    const tx3 = await this.identityRegistry.removeDid(this.signers.bob);
    const receipt = await tx3.wait();
    expect(receipt?.status).to.be.equal(1);
  });
});
