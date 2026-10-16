import { FhevmType } from '@fhevm/hardhat-plugin-v3';
import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/types';
import { network } from 'hardhat';

import type { IConfidentialERC20, TestConfidentialERC20Mintable } from '../../types/ethers-contracts/index.ts';
import { accountFor } from '../utils/signers.ts';

const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

type Hex = `0x${string}`;

export async function deployConfidentialERC20Fixture(
  account: HardhatEthersSigner,
  name: string,
  symbol: string,
  ownerAddress: string,
): Promise<TestConfidentialERC20Mintable> {
  const contractFactory = await ethers.getContractFactory('TestConfidentialERC20Mintable');
  const contract = await contractFactory.connect(account).deploy(name, symbol, ownerAddress);
  await contract.waitForDeployment();
  return contract;
}

export async function userDecryptAllowance(
  account: HardhatEthersSigner,
  spender: HardhatEthersSigner,
  token: IConfidentialERC20,
  tokenAddress: Hex,
): Promise<bigint> {
  const allowanceHandle = (await token.allowance(account, spender)) as Hex;
  return fhevm.userDecryptEuint(FhevmType.euint64, allowanceHandle, tokenAddress, accountFor(account));
}

export async function userDecryptBalance(
  account: HardhatEthersSigner,
  token: IConfidentialERC20,
  tokenAddress: Hex,
): Promise<bigint> {
  const balanceHandle = (await token.balanceOf(account)) as Hex;
  return fhevm.userDecryptEuint(FhevmType.euint64, balanceHandle, tokenAddress, accountFor(account));
}
