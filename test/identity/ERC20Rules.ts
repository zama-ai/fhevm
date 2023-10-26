import { ethers } from 'hardhat';

import type { ERC20Rules } from '../../types';
import { getSigners } from '../signers';

export async function deployERC20RulesFixture(): Promise<ERC20Rules> {
  const signers = await getSigners();

  const contractFactory = await ethers.getContractFactory('ERC20Rules');
  const contract = await contractFactory.connect(signers.alice).deploy();
  await contract.waitForDeployment();

  return contract;
}
