import type { Signer } from 'ethers';
import * as hre from 'hardhat';

import type { TestConfidentialVestingWalletCliff } from '../../../typechain-types';

export async function deployConfidentialVestingWalletCliffFixture(
  account: Signer,
  beneficiaryAddress: string,
  startTimestamp: bigint,
  duration: bigint,
  cliffSeconds: bigint,
): Promise<TestConfidentialVestingWalletCliff> {
  const contractFactory = await hre.ethers.getContractFactory('TestConfidentialVestingWalletCliff');
  const contract = await contractFactory
    .connect(account)
    .deploy(beneficiaryAddress, startTimestamp, duration, cliffSeconds);
  await contract.waitForDeployment();
  return contract;
}
