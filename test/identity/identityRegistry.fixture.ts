import { ethers } from 'hardhat';

import type { IdentityRegistry } from '../../types';
import { getSigners } from '../signers';

export async function deployIdentityRegistryFixture(): Promise<IdentityRegistry> {
  const signers = await getSigners();
  const contractFactory = await ethers.getContractFactory('IdentityRegistry');
  const contract = await contractFactory.connect(signers.alice).deploy();
  await contract.waitForDeployment();

  return contract;
}
