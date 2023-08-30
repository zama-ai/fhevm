import { ethers } from 'hardhat';

import type { Comp } from '../../types/contracts/Governor/Comp';
import { getSigners } from '../signers';

export async function deployCompFixture(): Promise<Comp> {
  const signers = await getSigners();

  const contractFactory = await ethers.getContractFactory('Comp');
  const contract = await contractFactory.connect(signers.alice).deploy(signers.alice.address);
  await contract.waitForDeployment();

  return contract;
}
