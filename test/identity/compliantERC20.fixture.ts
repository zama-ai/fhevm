import { ethers } from 'hardhat';

import type { CompliantERC20 } from '../../types';
import { getSigners } from '../signers';

export async function deployCompliantERC20Fixture(
  identityAddress: string,
  erc20RulesAddress: string,
): Promise<CompliantERC20> {
  const signers = await getSigners();

  const contractFactory = await ethers.getContractFactory('CompliantERC20');
  const contract = await contractFactory
    .connect(signers.alice)
    .deploy(identityAddress, erc20RulesAddress, 'CompliantToken', 'CTOK');
  await contract.waitForDeployment();

  return contract;
}
