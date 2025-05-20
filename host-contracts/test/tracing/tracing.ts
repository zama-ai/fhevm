import { ethers, network } from 'hardhat';

import { awaitCoprocessor } from '../coprocessorUtils';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

describe('Tracing', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    const contractFactory = await ethers.getContractFactory('TracingSubCalls');

    this.contract = await contractFactory.connect(this.signers.alice).deploy();
    this.contractAddress = await this.contract.getAddress();
    this.instances = await createInstances(this.signers);
  });

  it('test tracing for mocked', async function () {
    if (network.name === 'hardhat') {
      const tx = await this.contract.subCalls();
      await tx.wait();
      await awaitCoprocessor();
    }
  });
});
