import fs from 'node:fs/promises';
import path from 'node:path';

import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { EncryptedERC20 } from '../../types';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { deployEncryptedERC20Fixture } from './EncryptedERC20.fixture';

const STATE_FILE = process.env.CROSS_CUTOVER_STATE_FILE ?? '/tmp/cross-cutover-state.json';
const MINT_AMOUNT = 1_000_000n;
const TRANSFER_AMOUNT = 100n;

type State = {
  contractAddress: string;
  chainId: string;
  mintTransaction: string;
  mintHandle: string;
  postPromotionTransactions: string[];
  expectedAliceBalance: string;
  transferCount: number;
};

const loadState = async (): Promise<State> => {
  const raw = await fs.readFile(STATE_FILE, 'utf8');
  const state = JSON.parse(raw) as State;
  expect(state.chainId, 'cross-cutover state belongs to this host chain').to.eq(String((await ethers.provider.getNetwork()).chainId));
  return state;
};

const saveState = async (state: State): Promise<void> => {
  await fs.mkdir(path.dirname(STATE_FILE), { recursive: true });
  await fs.writeFile(STATE_FILE, JSON.stringify(state));
};

describe('EncryptedERC20 cross-cutover chain', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
  });

  it('[cross-cutover-setup] deploys ERC20 and mints Alice balance pre-cutover', async function () {
    const contract = await deployEncryptedERC20Fixture();
    const contractAddress = await contract.getAddress();
    const mintTx = await contract.mint(MINT_AMOUNT);
    const receipt = await mintTx.wait();
    expect(receipt?.status).to.eq(1);
    await saveState({
      contractAddress,
      chainId: String((await ethers.provider.getNetwork()).chainId),
      mintTransaction: mintTx.hash,
      mintHandle: await contract.balanceOf(this.signers.alice),
      postPromotionTransactions: [],
      expectedAliceBalance: MINT_AMOUNT.toString(),
      transferCount: 0,
    });
    console.log(`[cross-cutover-setup] ${contractAddress} minted=${MINT_AMOUNT}`);
  });

  it('[cross-cutover-transfer] transfers to Bob depending on pre-cutover balance', async function () {
    const state = await loadState();
    const factory = await ethers.getContractFactory('EncryptedERC20');
    const contract = factory.attach(state.contractAddress) as EncryptedERC20;

    const encAmount = await this.instances.alice.encryptUint64({
      value: TRANSFER_AMOUNT,
      contractAddress: state.contractAddress,
      userAddress: this.signers.alice.address,
    });
    const tx = await contract
      .connect(this.signers.alice)
      ['transfer(address,bytes32,bytes)'](
        this.signers.bob.address,
        encAmount.handles[0],
        encAmount.inputProof,
      );
    const receipt = await tx.wait();
    expect(receipt?.status).to.eq(1);

    if (process.env.CROSS_CUTOVER_POST_PROMOTION === '1') state.postPromotionTransactions.push(tx.hash);
    state.transferCount += 1;
    state.expectedAliceBalance = (BigInt(state.expectedAliceBalance) - TRANSFER_AMOUNT).toString();
    await saveState(state);
    console.log(
      `[cross-cutover-transfer #${state.transferCount}] expected Alice=${state.expectedAliceBalance}`,
    );
  });

  it('[cross-cutover-verify] Alice balance matches expected math after chain + cutover', async function () {
    const state = await loadState();
    const factory = await ethers.getContractFactory('EncryptedERC20');
    const contract = factory.attach(state.contractAddress) as EncryptedERC20;

    if (process.env.CROSS_CUTOVER_REQUIRE_POST_PROMOTION === '1') {
      expect(state.postPromotionTransactions.length, 'a dependent transfer must be submitted after observed promotion').to.be.greaterThan(0);
    }
    const balanceHandle = await contract.balanceOf(this.signers.alice);
    const balance = await this.instances.alice.userDecryptSingleHandle({
      handle: balanceHandle,
      contractAddress: state.contractAddress,
      signer: this.signers.alice,
    });
    expect(balance).to.equal(BigInt(state.expectedAliceBalance));
    console.log(
      `[cross-cutover-verify] Alice=${balance} matches expected after ${state.transferCount} transfer(s)`,
    );
  });
});
