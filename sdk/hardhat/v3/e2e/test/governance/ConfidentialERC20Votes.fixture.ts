/* eslint-disable no-unexpected-multiline */
import { FhevmType } from '@fhevm/hardhat-plugin-v3';
import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/types';
import { network } from 'hardhat';

import type { TestConfidentialERC20Votes } from '../../types/ethers-contracts/index.ts';
import { accountFor } from '../utils/signers.ts';

const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

type Hex = `0x${string}`;

export async function deployConfidentialERC20Votes(account: HardhatEthersSigner): Promise<TestConfidentialERC20Votes> {
  const contractFactory = await ethers.getContractFactory('TestConfidentialERC20Votes');
  const contract = await contractFactory
    .connect(account)
    .deploy(
      await account.getAddress(),
      'CompoundZama',
      'CONFIDENTIAL_ERC20_VOTES',
      '1.0',
      ethers.parseUnits('10000000', 6),
    );
  await contract.waitForDeployment();
  return contract;
}

export async function transferTokensAndDelegate(
  owner: HardhatEthersSigner,
  delegator: HardhatEthersSigner,
  delegateeAddress: string,
  transferAmount: bigint,
  confidentialERC20Votes: TestConfidentialERC20Votes,
  confidentialERC20VotesAddress: Hex,
): Promise<void> {
  const input = fhevm.createEncryptedInput(confidentialERC20VotesAddress, (await owner.getAddress()) as Hex);
  input.add64(transferAmount);
  const encryptedTransferAmount = await input.encrypt();
  const [transferHandle] = encryptedTransferAmount.handles;
  if (transferHandle === undefined) throw new Error('encrypt() returned no handle');

  let tx = await confidentialERC20Votes
    .connect(owner)
    ['transfer(address,bytes32,bytes)'](
      await delegator.getAddress(),
      transferHandle,
      encryptedTransferAmount.inputProof,
    );
  await tx.wait();

  tx = await confidentialERC20Votes.connect(delegator).delegate(delegateeAddress);
  await tx.wait();
}

export async function userDecryptCurrentVotes(
  account: HardhatEthersSigner,
  confidentialERC20Votes: TestConfidentialERC20Votes,
  confidentialERC20VotesAddress: Hex,
): Promise<bigint> {
  const voteHandle = (await confidentialERC20Votes.getCurrentVotes(await account.getAddress())) as Hex;
  return fhevm.userDecryptEuint(FhevmType.euint64, voteHandle, confidentialERC20VotesAddress, accountFor(account));
}

export async function userDecryptPriorVotes(
  account: HardhatEthersSigner,
  blockNumber: number,
  confidentialERC20Votes: TestConfidentialERC20Votes,
  confidentialERC20VotesAddress: Hex,
): Promise<bigint> {
  const voteHandle = (await confidentialERC20Votes.getPriorVotes(await account.getAddress(), blockNumber)) as Hex;
  return fhevm.userDecryptEuint(FhevmType.euint64, voteHandle, confidentialERC20VotesAddress, accountFor(account));
}
