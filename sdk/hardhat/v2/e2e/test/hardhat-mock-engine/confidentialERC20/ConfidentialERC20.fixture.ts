import { FhevmType } from '@fhevm/hardhat-plugin';
import type { Signer } from 'ethers';
import * as hre from 'hardhat';

import type { IConfidentialERC20 } from '../../../typechain-types';
import type { TestConfidentialERC20Mintable } from '../../../typechain-types';

export async function deployConfidentialERC20Fixture(
  account: Signer,
  name: string,
  symbol: string,
  ownerAddress: string,
): Promise<TestConfidentialERC20Mintable> {
  const contractFactory = await hre.ethers.getContractFactory('TestConfidentialERC20Mintable');
  const contract = await contractFactory.connect(account).deploy(name, symbol, ownerAddress);
  await contract.waitForDeployment();
  return contract;
}

export async function userDecryptAllowance(
  account: Signer,
  spender: Signer,
  token: IConfidentialERC20,
  tokenAddress: string,
): Promise<bigint> {
  const allowanceHandle = await token.allowance(account, spender);
  const allowance = await hre.fhevm.userDecryptEuint(FhevmType.euint64, allowanceHandle, tokenAddress, account);
  return allowance;
}

export async function userDecryptBalance(
  account: Signer,
  token: IConfidentialERC20,
  tokenAddress: string,
): Promise<bigint> {
  const balanceHandle = await token.balanceOf(account);
  const balance = await hre.fhevm.userDecryptEuint(FhevmType.euint64, balanceHandle, tokenAddress, account);
  return balance;
}
