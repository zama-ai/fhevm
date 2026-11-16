import type { TracingSubCalls } from '../../typechain-types/index.js';
import { awaitCoprocessor } from '../coprocessorUtils.js';
import { ethers, isInProcessNetwork } from '../hardhat.js';
import { getSigners, initSigners } from '../signers.js';
import type { Signers } from '../signers.js';

describe('Tracing', function () {
  let contract: TracingSubCalls;

  before(async function () {
    if (!isInProcessNetwork) {
      this.skip(); // marks every test in this describe as pending
    }
    await initSigners(1);
    const signers: Signers = await getSigners();
    const contractFactory = await ethers.getContractFactory('TracingSubCalls');
    contract = await contractFactory.connect(signers.alice).deploy();
  });

  it('test tracing for mocked', async function () {
    if (isInProcessNetwork) {
      const tx = await contract.subCalls();
      await tx.wait();
      await awaitCoprocessor();
    }
  });
});
