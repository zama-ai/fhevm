import { ethers } from 'hardhat';

import type { Rand } from '../../typechain-types';
import { getSigners } from '../signers';

export async function deployRandFixture(): Promise<Rand> {
  const signers = await getSigners();

  const contractFactory = await ethers.getContractFactory('Rand');
  const contract = await contractFactory.connect(signers.alice).deploy();
  await contract.waitForDeployment();

  return contract;
}
