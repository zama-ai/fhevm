import { ethers } from 'hardhat';

import type { IdentifiedERC20 } from '../../types';
import { getSigners } from '../signers';

export async function deployIdentifiedERC20Fixture(
  identityAddress: string,
  erc20RulesAddress: string,
): Promise<IdentifiedERC20> {
  const signers = await getSigners();

  const contractFactory = await ethers.getContractFactory('IdentifiedERC20');
  const contract = await contractFactory.connect(signers.alice).deploy(identityAddress, erc20RulesAddress);
  await contract.waitForDeployment();

  return contract;
}
