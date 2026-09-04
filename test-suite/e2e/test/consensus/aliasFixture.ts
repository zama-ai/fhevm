import type { Signer, TransactionReceipt, TransactionResponse } from 'ethers';
import { ethers } from 'hardhat';

import { waitForPendingTransactions, waitForTransactionReceipt } from '../utils';

export type ContractTxOverrides = { readonly gasLimit: bigint | number };

/** Minimal surface intentionally avoids generated TypeChain artifacts. */
export interface AliasFixtureContract {
  getAddress(): Promise<string>;
  waitForDeployment(): Promise<unknown>;
  produceInputs(overrides: ContractTxOverrides): Promise<TransactionResponse>;
  combineFromStorage(overrides: ContractTxOverrides): Promise<TransactionResponse>;
  combineFromStorageAgain(overrides: ContractTxOverrides): Promise<TransactionResponse>;
  combineLocal(overrides: ContractTxOverrides): Promise<TransactionResponse>;
  inputB(): Promise<string>;
  inputC(): Promise<string>;
  combined(): Promise<string>;
  combinedSecond(): Promise<string>;
  combinedLocal(): Promise<string>;
}

export interface AliasFixtureHandles {
  readonly inputB: string;
  readonly inputC: string;
  readonly combined: string;
  readonly combinedSecond: string;
  readonly combinedLocal: string;
}

export const ALIAS_FIXTURE_EXPECTED_PLAINTEXTS = {
  inputB: 7n,
  inputC: 5n,
  combined: 12n,
  combinedLocal: 12n,
} as const;

export const ALIAS_FIXTURE_GAS_LIMIT = 10_000_000;

export async function deployAliasFixture(owner: Signer): Promise<{
  readonly contract: AliasFixtureContract;
  readonly contractAddress: string;
}> {
  const factory = await ethers.getContractFactory('AliasFixture');
  const contract = (await factory.connect(owner).deploy()) as unknown as AliasFixtureContract;
  await contract.waitForDeployment();
  return { contract, contractAddress: await contract.getAddress() };
}

export function requireSuccessfulReceipt(receipt: TransactionReceipt | null, description: string): TransactionReceipt {
  if (!receipt || receipt.status !== 1) {
    throw new Error(`${description} failed to mine successfully`);
  }
  return receipt;
}

export async function collectAliasHandles(contract: AliasFixtureContract): Promise<AliasFixtureHandles> {
  const [inputB, inputC, combined, combinedSecond, combinedLocal] = await Promise.all([
    contract.inputB(),
    contract.inputC(),
    contract.combined(),
    contract.combinedSecond(),
    contract.combinedLocal(),
  ]);
  for (const [label, handle] of Object.entries({ inputB, inputC, combined, combinedSecond, combinedLocal })) {
    if (handle === ethers.ZeroHash) throw new Error(`alias fixture did not materialize ${label}`);
  }
  return { inputB, inputC, combined, combinedSecond, combinedLocal };
}

export interface AliasSameBlockRun {
  readonly handles: AliasFixtureHandles;
  readonly blockNumber: number;
  readonly blockHash: string;
  readonly produceTxHash: string;
  readonly storageTxHash: string;
  readonly storageAgainTxHash: string;
  readonly localTxHash: string;
}

/**
 * Lands `produceInputs`, `combineFromStorage`, `combineFromStorageAgain`,
 * and `combineLocal` in exactly one L1 block. The two storage combines then
 * alias each other (same op, same boundary operands, same boundary bits),
 * `combineLocal`'s trivial encrypts alias `produceInputs`' outputs, and its
 * add — consuming operands minted in its own transaction — folds zero
 * boundary bits and mints a handle DISTINCT from `combined`.
 */
export async function runAliasSameBlock(contract: AliasFixtureContract): Promise<AliasSameBlockRun> {
  let receipts: readonly TransactionReceipt[];
  await ethers.provider.send('evm_setIntervalMining', [0]);
  await ethers.provider.send('evm_setAutomine', [false]);
  try {
    const produceTx = await contract.produceInputs({ gasLimit: ALIAS_FIXTURE_GAS_LIMIT });
    const storageTx = await contract.combineFromStorage({ gasLimit: ALIAS_FIXTURE_GAS_LIMIT });
    const storageAgainTx = await contract.combineFromStorageAgain({ gasLimit: ALIAS_FIXTURE_GAS_LIMIT });
    const localTx = await contract.combineLocal({ gasLimit: ALIAS_FIXTURE_GAS_LIMIT });
    await waitForPendingTransactions([produceTx.hash, storageTx.hash, storageAgainTx.hash, localTx.hash]);
    await ethers.provider.send('evm_mine');
    receipts = await Promise.all(
      [produceTx, storageTx, storageAgainTx, localTx].map(async (transaction) =>
        requireSuccessfulReceipt(await waitForTransactionReceipt(transaction.hash), 'alias fixture transaction'),
      ),
    );
  } finally {
    await ethers.provider.send('evm_setAutomine', [true]);
    await ethers.provider.send('evm_setIntervalMining', [1]);
  }

  const blockHash = receipts![0].blockHash;
  const blockNumber = receipts![0].blockNumber;
  for (const receipt of receipts!) {
    if (receipt.blockNumber !== blockNumber || receipt.blockHash !== blockHash) {
      throw new Error('alias fixture transactions did not land in exactly one L1 block');
    }
  }

  return {
    handles: await collectAliasHandles(contract),
    blockNumber,
    blockHash: blockHash!,
    produceTxHash: receipts![0].hash,
    storageTxHash: receipts![1].hash,
    storageAgainTxHash: receipts![2].hash,
    localTxHash: receipts![3].hash,
  };
}
