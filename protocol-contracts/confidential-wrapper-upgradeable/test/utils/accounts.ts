// Ported from https://github.com/OpenZeppelin/openzeppelin-confidential-contracts/blob/f0914b66f9f3766915403587b1ef1432d53054d3/test/helpers/accounts.ts
// (0.3.0 version)

import { impersonateAccount, setBalance } from '@nomicfoundation/hardhat-network-helpers';
import { Addressable, Signer, ethers } from 'ethers';
import fs from 'fs';
import { fhevm } from 'hardhat';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

const DEFAULT_BALANCE: bigint = 10000n * ethers.WeiPerEther;

export async function impersonate(hre: HardhatRuntimeEnvironment, account: string, balance: bigint = DEFAULT_BALANCE) {
  return impersonateAccount(account)
    .then(() => setBalance(account, balance))
    .then(() => hre.ethers.getSigner(account));
}

export async function getAclAddress() {
  return (await fhevm.getRelayerMetadata()).ACLAddress;
}

export async function allowHandle(hre: HardhatRuntimeEnvironment, from: Signer, to: Addressable, handle: string) {
  const acl_abi = JSON.parse(
    fs.readFileSync('node_modules/@fhevm/host-contracts/artifacts/contracts/ACL.sol/ACL.json', 'utf8'),
  ).abi;
  const aclContract = await hre.ethers.getContractAt(acl_abi, await getAclAddress());

  await (aclContract as any).connect(from).allow(handle, to);
}
