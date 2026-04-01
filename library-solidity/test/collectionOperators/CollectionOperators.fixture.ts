import { ethers } from 'hardhat';

import type { FHEVMGasProfileSuite } from '../../typechain-types/examples/tests/FHEVMGasProfileSuite';
import { getSigners } from '../signers';

export async function deployFHEVMGasProfileSuiteFixture(): Promise<FHEVMGasProfileSuite> {
  const signers = await getSigners();

  const contractFactory = await ethers.getContractFactory('FHEVMGasProfileSuite');
  const contract = await contractFactory.connect(signers.alice).deploy();
  await contract.waitForDeployment();

  return contract as unknown as FHEVMGasProfileSuite;
}
