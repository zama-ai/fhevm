import { network } from 'hardhat';

import type { Rand } from '../../types/ethers-contracts/index.ts';
import { getSigners } from '../utils/signers.ts';

const connection = await network.getOrCreate();
const { ethers } = connection;

export async function deployRandFixture(): Promise<Rand> {
  const signers = await getSigners(connection);

  const contractFactory = await ethers.getContractFactory('Rand');
  const contract = await contractFactory.connect(signers.alice).deploy();
  await contract.waitForDeployment();

  return contract;
}
