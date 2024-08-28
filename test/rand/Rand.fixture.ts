import { ethers } from 'hardhat';

import type { Rand } from '../../types';
import { getSigners } from '../signers';

export async function deployRandFixture(): Promise<Rand> {
  const signers = await getSigners();

  const contractFactory = await ethers.getContractFactory('Rand');
  const contract = await contractFactory.connect(signers.alice).deploy({
    value: ethers.parseEther('0.001'),
  });
  await contract.waitForDeployment();

  return contract;
}
