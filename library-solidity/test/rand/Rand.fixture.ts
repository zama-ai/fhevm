import type { Rand } from '../../typechain-types/index.js';
import { ethers } from '../hardhat.js';
import { getSigners } from '../signers.js';

export async function deployRandFixture(): Promise<Rand> {
  const signers = await getSigners();

  const contractFactory = await ethers.getContractFactory('Rand');
  const contract = await contractFactory.connect(signers.alice).deploy();
  await contract.waitForDeployment();

  return contract;
}
