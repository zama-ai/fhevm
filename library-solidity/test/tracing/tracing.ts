import { ethers, network } from 'hardhat';
import { awaitCoprocessor } from '../coprocessorUtils';
import { getSigners, initSigners } from '../signers';
import type { Signers } from '../signers';
import type { TracingSubCalls } from '../../typechain-types';

describe('Tracing', function () {
  let contract: TracingSubCalls;

  before(async function () {
    if (network.name !== 'hardhat') {
      this.skip(); // marks every test in this describe as pending
    }
    await initSigners(1);
    const signers: Signers = await getSigners();
    const contractFactory = await ethers.getContractFactory('TracingSubCalls');
    contract = await contractFactory.connect(signers.alice).deploy();
  });

  it('test tracing for mocked', async function () {
    if (network.name === 'hardhat') {
      const tx = await contract.subCalls();
      await tx.wait();
      await awaitCoprocessor();
    }
  });
});
