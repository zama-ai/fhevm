import { FhevmType } from '@fhevm/hardhat-plugin-v3';
import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/types';
import { network } from 'hardhat';

import type { ConfidentialVestingWallet, TestConfidentialVestingWallet } from '../../types/ethers-contracts/index.ts';
import { accountFor } from '../utils/signers.ts';

const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

type Hex = `0x${string}`;

export async function deployConfidentialVestingWalletFixture(
  account: HardhatEthersSigner,
  beneficiaryAddress: string,
  startTimestamp: bigint,
  duration: bigint,
): Promise<TestConfidentialVestingWallet> {
  const contractFactory = await ethers.getContractFactory('TestConfidentialVestingWallet');
  const contract = await contractFactory.connect(account).deploy(beneficiaryAddress, startTimestamp, duration);
  await contract.waitForDeployment();
  return contract;
}

export async function userDecryptReleased(
  account: HardhatEthersSigner,
  tokenAddress: Hex,
  vestingWallet: ConfidentialVestingWallet,
  vestingWalletAddress: Hex,
): Promise<bigint> {
  const releasedHandled = (await vestingWallet.released(tokenAddress)) as Hex;
  return fhevm.userDecryptEuint(FhevmType.euint64, releasedHandled, vestingWalletAddress, accountFor(account));
}
