import { FhevmType } from '@fhevm/hardhat-plugin';
import type { Signer } from 'ethers';
import * as hre from 'hardhat';

import type { ConfidentialVestingWallet, TestConfidentialVestingWallet } from '../../../typechain-types';

export async function deployConfidentialVestingWalletFixture(
  account: Signer,
  beneficiaryAddress: string,
  startTimestamp: bigint,
  duration: bigint,
): Promise<TestConfidentialVestingWallet> {
  const contractFactory = await hre.ethers.getContractFactory('TestConfidentialVestingWallet');
  const contract = await contractFactory.connect(account).deploy(beneficiaryAddress, startTimestamp, duration);
  await contract.waitForDeployment();
  return contract;
}

export async function userDecryptReleased(
  account: Signer,
  tokenAddress: string,
  vestingWallet: ConfidentialVestingWallet,
  vestingWalletAddress: string,
): Promise<bigint> {
  const releasedHandled = await vestingWallet.released(tokenAddress);
  const releasedAmount = await hre.fhevm.userDecryptEuint(
    FhevmType.euint64,
    releasedHandled,
    vestingWalletAddress,
    account,
  );
  return releasedAmount;
}
