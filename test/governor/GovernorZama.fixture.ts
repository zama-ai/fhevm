import { ethers } from 'hardhat';

import { Comp } from '../../types';
import type { GovernorZama } from '../../types';
import { getSigners } from '../signers';

export async function deployGovernorZamaFixture(compContract: Comp): Promise<GovernorZama> {
  const signers = await getSigners();

  const timelockFactory = await ethers.getContractFactory('Timelock');
  const timelockContract = await timelockFactory.connect(signers.alice).deploy(signers.alice.address, 60 * 60 * 24 * 2);

  await timelockContract.waitForDeployment();

  const contractFactory = await ethers.getContractFactory('GovernorZama');
  const contract = await contractFactory
    .connect(signers.alice)
    .deploy(timelockContract.getAddress(), compContract.getAddress(), signers.alice.address);
  await contract.waitForDeployment();

  return contract;
}
