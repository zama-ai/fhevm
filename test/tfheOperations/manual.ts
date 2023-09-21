import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { TFHEManualTestSuite } from '../../types/contracts/tests/TFHEManualTestSuite';
import { createInstances } from '../instance';
import { getSigners } from '../signers';

async function deployTfheManualTestFixture(): Promise<TFHEManualTestSuite> {
  const signers = await ethers.getSigners();
  const admin = signers[0];

  const contractFactory = await ethers.getContractFactory('TFHEManualTestSuite');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('TFHE manual operations', function () {
  before(async function () {
    this.signers = await getSigners();

    const contract = await deployTfheManualTestFixture();
    this.contractAddress = await contract.getAddress();
    this.contract = contract;
    const instances = await createInstances(this.contractAddress, ethers, this.signers);
    this.instances = instances;
  });

  it('Cmux works returning if false', async function () {
    const res = await this.contract.test_cmux(
      this.instances.alice.encrypt8(0),
      this.instances.alice.encrypt32(3),
      this.instances.alice.encrypt32(4),
    );
    expect(res).to.equal(4);
  });

  it('Cmux works returning if true', async function () {
    const res = await this.contract.test_cmux(
      this.instances.alice.encrypt8(1),
      this.instances.alice.encrypt32(3),
      this.instances.alice.encrypt32(4),
    );
    expect(res).to.equal(3);
  });
});
