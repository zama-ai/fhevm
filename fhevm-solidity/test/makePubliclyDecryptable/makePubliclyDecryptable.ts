import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

describe('MakePubliclyDecryptable', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    const contractFactory = await ethers.getContractFactory('MakePubliclyDecryptable');

    this.contract = await contractFactory.connect(this.signers.alice).deploy();
    await this.contract.waitForDeployment();
    this.contractAddress = await this.contract.getAddress();
    this.instances = await createInstances(this.signers);
  });

  it('test MakePubliclyDecryptable ebool', async function () {
    const isPubliclyDecryptableBefore = await this.contract.isPubliclyDecryptableBool();
    expect(isPubliclyDecryptableBefore).to.equal(false);
    const tx = await this.contract.makePubliclyDecryptableBool();
    await tx.wait();
    const isPubliclyDecryptableAfter = await this.contract.isPubliclyDecryptableBool();
    expect(isPubliclyDecryptableAfter).to.equal(true);
  });

  it('test MakePubliclyDecryptable euint8', async function () {
    const isPubliclyDecryptableBefore = await this.contract.isPubliclyDecryptableUint8();
    expect(isPubliclyDecryptableBefore).to.equal(false);
    const tx = await this.contract.makePubliclyDecryptableUint8();
    await tx.wait();
    const isPubliclyDecryptableAfter = await this.contract.isPubliclyDecryptableUint8();
    expect(isPubliclyDecryptableAfter).to.equal(true);
  });

  it('test MakePubliclyDecryptable ebytes256', async function () {
    const isPubliclyDecryptableBefore = await this.contract.isPubliclyDecryptableBytes256();
    expect(isPubliclyDecryptableBefore).to.equal(false);
    const tx = await this.contract.makePubliclyDecryptableBytes256();
    await tx.wait();
    const isPubliclyDecryptableAfter = await this.contract.isPubliclyDecryptableBytes256();
    expect(isPubliclyDecryptableAfter).to.equal(true);
  });
});
