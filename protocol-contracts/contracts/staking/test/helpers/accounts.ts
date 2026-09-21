import { impersonateAccount, setBalance } from '@nomicfoundation/hardhat-network-helpers';
import { ethers } from 'ethers';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

const DEFAULT_BALANCE: bigint = 10000n * ethers.WeiPerEther;

export async function impersonate(hre: HardhatRuntimeEnvironment, account: string, balance: bigint = DEFAULT_BALANCE) {
  return impersonateAccount(account)
    .then(() => setBalance(account, balance))
    .then(() => hre.ethers.getSigner(account));
}
