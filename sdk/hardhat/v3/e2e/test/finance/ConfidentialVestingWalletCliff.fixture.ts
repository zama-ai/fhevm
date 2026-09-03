import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/types';
import { network } from 'hardhat';

import type { TestConfidentialVestingWalletCliff } from '../../types/ethers-contracts/index.ts';

const { ethers } = await network.getOrCreate();

export async function deployConfidentialVestingWalletCliffFixture(
  account: HardhatEthersSigner,
  beneficiaryAddress: string,
  startTimestamp: bigint,
  duration: bigint,
  cliffSeconds: bigint,
): Promise<TestConfidentialVestingWalletCliff> {
  const contractFactory = await ethers.getContractFactory('TestConfidentialVestingWalletCliff');
  const contract = await contractFactory
    .connect(account)
    .deploy(beneficiaryAddress, startTimestamp, duration, cliffSeconds);
  await contract.waitForDeployment();
  return contract;
}
