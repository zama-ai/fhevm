import { ethers } from 'hardhat';

import type { TFHETestSuite } from '../../types';

export async function deployTfheTestFixture(): Promise<TFHETestSuite> {
  const signers = await ethers.getSigners();
  const admin = signers[0];

  const contractFactory = await ethers.getContractFactory('TFHETestSuite');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}
