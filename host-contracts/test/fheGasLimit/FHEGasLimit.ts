import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { initializeFHEGasLimit } from '../paymentUtils';
import { getSigners, initSigners } from '../signers';

describe('TestFHEGasLimit', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.fheGasLimit = await initializeFHEGasLimit();
  });

  beforeEach(async function () {
    this.instances = await createInstances(this.signers);
  });

  it('tx reverts if above hcu depth limit', async function () {
    const contractFactory = await ethers.getContractFactory('HCULimit');
    const contract = await contractFactory.connect(this.signers.alice).deploy();
    await contract.waitForDeployment();
    await expect(contract.aboveBlockFHEGasLimitWithSequentialOperations()).revertedWithCustomError(
      this.fheGasLimit,
      'HCUTransactionDepthLimitExceeded',
    );
  });

  it('tx reverts if above hcu transaction limit', async function () {
    const contractFactory = await ethers.getContractFactory('HCULimit');
    const contract = await contractFactory.connect(this.signers.alice).deploy();
    await contract.waitForDeployment();
    await expect(contract.aboveBlockFHEGasLimitWithNonSequentialOperations()).revertedWithCustomError(
      this.fheGasLimit,
      'HCUTransactionLimitExceeded',
    );
  });
});
