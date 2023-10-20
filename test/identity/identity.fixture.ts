import { ethers } from 'hardhat';

import type { Identity } from '../../types';
import { getSigners } from '../signers';

export async function deployIdentityFixture(): Promise<Identity> {
  const signers = await getSigners();
  const contractFactory = await ethers.getContractFactory('Identity');
  const contract = await contractFactory.connect(signers.alice).deploy();
  await contract.waitForDeployment();

  return contract;
}
