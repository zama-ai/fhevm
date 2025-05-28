import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { initializeHCULimit } from '../paymentUtils';
import { getSigners, initSigners } from '../signers';

describe('Test HCULimit', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.HCULimit = await initializeHCULimit();
  });

  beforeEach(async function () {
    this.instances = await createInstances(this.signers);
  });

  it('tx reverts if above hcu depth limit', async function () {
    const contractFactory = await ethers.getContractFactory('HCULimitTest');
    const contract = await contractFactory.connect(this.signers.alice).deploy();
    await contract.waitForDeployment();
    await expect(contract.aboveTransactionHCULimitWithSequentialOperations()).revertedWithCustomError(
      this.HCULimit,
      'HCUTransactionDepthLimitExceeded',
    );
  });

  it('tx reverts if above hcu transaction limit', async function () {
    const contractFactory = await ethers.getContractFactory('HCULimitTest');
    const contract = await contractFactory.connect(this.signers.alice).deploy();
    await contract.waitForDeployment();
    await expect(contract.aboveTransactionHCUWithNonSequentialOperations()).revertedWithCustomError(
      this.HCULimit,
      'HCUTransactionLimitExceeded',
    );
  });
});
