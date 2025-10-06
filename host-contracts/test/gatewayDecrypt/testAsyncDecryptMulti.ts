import { expect } from 'chai';
import { ethers } from 'hardhat';

import { awaitAllDecryptionResults, initDecryptionOracle } from '../asyncDecrypt';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

describe('TestAsyncDecryptMultiContracts', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    await initDecryptionOracle();
  });

  beforeEach(async function () {
    const contractFactoryA = await ethers.getContractFactory('TestAsyncDecryptA');
    this.contractA = await contractFactoryA.connect(this.signers.alice).deploy();
    await this.contractA.waitForDeployment();
    const contractFactoryB = await ethers.getContractFactory('TestAsyncDecryptB');
    this.contractB = await contractFactoryB.connect(this.signers.alice).deploy();
    await this.contractB.waitForDeployment();
    this.instances = await createInstances(this.signers);
  });

  it('test async decrypt euint64 via 2 contracts', async function () {
    const tx = await this.contractA.connect(this.signers.carol).requestUint64();
    await tx.wait();
    try {
      await awaitAllDecryptionResults();
    } catch {
      // we expect an error after first decryption, due to the revert in callback, so we silence it
    }
    const tx2 = await this.contractB.connect(this.signers.carol).requestUint64();
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contractA.yUint64();
    const y2 = await this.contractB.yUint64();
    expect(y).to.equal(0); // because first decryption callback reverted
    expect(y2).to.equal(373737); // this second decryption, on the other hand, should succeed
  });
});
