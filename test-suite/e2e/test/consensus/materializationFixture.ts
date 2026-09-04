import type { Signer, TransactionReceipt, TransactionResponse } from 'ethers';
import { ethers } from 'hardhat';

import type { SdkInstance } from '../sdk/types';
import { waitForPendingTransactions, waitForTransactionReceipt } from '../utils';
import { FIXTURE_HANDLE_LABELS, type FixtureHandleLabel, type FixtureHandles } from './materializationFixtureModel';

export { decryptMaterializationFixture, type FixturePlaintexts } from './materializationFixtureDecrypt';

type ContractTxOverrides = { readonly gasLimit: bigint | number };

/** Minimal surface intentionally avoids generated TypeChain artifacts. */
export interface MaterializationFixtureContract {
  getAddress(): Promise<string>;
  waitForDeployment(): Promise<unknown>;
  stageInputA(input: Uint8Array, proof: Uint8Array, overrides: ContractTxOverrides): Promise<TransactionResponse>;
  deriveFromAAndB(input: Uint8Array, proof: Uint8Array, overrides: ContractTxOverrides): Promise<TransactionResponse>;
  runIndependent(input: Uint8Array, proof: Uint8Array, overrides: ContractTxOverrides): Promise<TransactionResponse>;
  consumeFanout(overrides: ContractTxOverrides): Promise<TransactionResponse>;
  stageZero(): Promise<string>;
  inputA(): Promise<string>;
  inputB(): Promise<string>;
  trivialOne(): Promise<string>;
  inputAIsZero(): Promise<string>;
  selected(): Promise<string>;
  sum(): Promise<string>;
  difference(): Promise<string>;
  independentInput(): Promise<string>;
  independentBias(): Promise<string>;
  independent(): Promise<string>;
  terminal(): Promise<string>;
}

export interface MaterializationFixtureRun {
  readonly contract: MaterializationFixtureContract;
  readonly contractAddress: string;
  readonly handles: FixtureHandles;
  readonly sameBlockNumber: number;
  readonly sameBlockHash: string;
  readonly terminalBlockNumber: number;
  readonly terminalBlockHash: string;
}

const FIXTURE_GAS_LIMIT = 10_000_000;

function requireSuccessfulReceipt(receipt: TransactionReceipt | null, description: string): TransactionReceipt {
  if (!receipt || receipt.status !== 1) {
    throw new Error(`${description} failed to mine successfully`);
  }
  return receipt;
}

function requireBlockHash(receipt: TransactionReceipt, description: string): string {
  if (!receipt.blockHash) throw new Error(`${description} has no block hash`);
  return receipt.blockHash;
}

async function collectHandles(contract: MaterializationFixtureContract): Promise<FixtureHandles> {
  const values = await Promise.all([
    contract.stageZero(),
    contract.inputA(),
    contract.inputB(),
    contract.trivialOne(),
    contract.inputAIsZero(),
    contract.selected(),
    contract.sum(),
    contract.difference(),
    contract.independentInput(),
    contract.independentBias(),
    contract.independent(),
    contract.terminal(),
  ]);
  const handles = Object.fromEntries(
    FIXTURE_HANDLE_LABELS.map((label, index) => [label, values[index]]),
  ) as FixtureHandles;
  for (const label of FIXTURE_HANDLE_LABELS) {
    if (handles[label] === ethers.ZeroHash) {
      throw new Error(`fixture did not materialize ${label}`);
    }
  }
  const handleOwners = new Map<string, FixtureHandleLabel>();
  for (const label of FIXTURE_HANDLE_LABELS) {
    const handle = handles[label].toLowerCase();
    const previous = handleOwners.get(handle);
    if (previous) {
      throw new Error(
        `fixture output handles must be unique for the ciphertext/digest/Gateway oracle: ${previous} and ${label} are ${handle}`,
      );
    }
    handleOwners.set(handle, label);
  }
  return handles;
}

/** Deploys the deterministic contract graph used by CPU and GPU consensus runs. */
export async function deployMaterializationFixture(owner: Signer): Promise<{
  readonly contract: MaterializationFixtureContract;
  readonly contractAddress: string;
}> {
  const factory = await ethers.getContractFactory('MaterializationFixture');
  const contract = (await factory.connect(owner).deploy()) as unknown as MaterializationFixtureContract;
  await contract.waitForDeployment();
  return { contract, contractAddress: await contract.getAddress() };
}

/**
 * Runs the fixture graph with a deterministic L1 block shape.
 *
 * The three transactions in the first block are intentionally sent from one
 * signer in nonce order.  `deriveFromAAndB` reads the state written by
 * `stageInputA`; this is a real cross-transaction dependency in one L1 block,
 * not a synthetic batch in one EVM call.  We restore Anvil's normal one-second
 * interval in `finally` because leaving the chain frozen starves later E2E
 * listener finality checks.
 */
export async function runMaterializationFixture(parameters: {
  readonly contract: MaterializationFixtureContract;
  readonly contractAddress: string;
  readonly owner: Signer & { readonly address: string };
  readonly instance: SdkInstance;
}): Promise<MaterializationFixtureRun> {
  const { contract, contractAddress, owner, instance } = parameters;
  const [encryptedA, encryptedB, encryptedIndependent] = await Promise.all([
    instance.encryptUint64({ value: 0n, contractAddress, userAddress: owner.address }),
    instance.encryptUint64({ value: 9n, contractAddress, userAddress: owner.address }),
    instance.encryptUint64({ value: 23n, contractAddress, userAddress: owner.address }),
  ]);

  let firstBlockReceipts: readonly TransactionReceipt[];
  await ethers.provider.send('evm_setIntervalMining', [0]);
  await ethers.provider.send('evm_setAutomine', [false]);
  try {
    // Explicit gas avoids estimating later transactions against the pre-block
    // state where `inputA` has not yet been committed.  It does not change the
    // graph or its consensus boundary.
    const stageTx = await contract.stageInputA(encryptedA.handles[0], encryptedA.inputProof, {
      gasLimit: FIXTURE_GAS_LIMIT,
    });
    const deriveTx = await contract.deriveFromAAndB(encryptedB.handles[0], encryptedB.inputProof, {
      gasLimit: FIXTURE_GAS_LIMIT,
    });
    const independentTx = await contract.runIndependent(
      encryptedIndependent.handles[0],
      encryptedIndependent.inputProof,
      { gasLimit: FIXTURE_GAS_LIMIT },
    );

    await waitForPendingTransactions([stageTx.hash, deriveTx.hash, independentTx.hash]);
    await ethers.provider.send('evm_mine');
    firstBlockReceipts = await Promise.all(
      [stageTx, deriveTx, independentTx].map(async (transaction) =>
        requireSuccessfulReceipt(await waitForTransactionReceipt(transaction.hash), 'same-block fixture transaction'),
      ),
    );
  } finally {
    await ethers.provider.send('evm_setAutomine', [true]);
    await ethers.provider.send('evm_setIntervalMining', [1]);
  }

  const sameBlockHash = requireBlockHash(firstBlockReceipts![0], 'stageInputA');
  const sameBlockNumber = firstBlockReceipts![0].blockNumber;
  for (const receipt of firstBlockReceipts!) {
    if (
      receipt.blockNumber !== sameBlockNumber ||
      requireBlockHash(receipt, 'same-block fixture transaction') !== sameBlockHash
    ) {
      throw new Error('materialization fixture transactions did not land in exactly one L1 block');
    }
  }

  const terminalTx = await contract.consumeFanout({ gasLimit: FIXTURE_GAS_LIMIT });
  const terminalReceipt = requireSuccessfulReceipt(
    await waitForTransactionReceipt(terminalTx.hash),
    'next-block terminal fixture transaction',
  );
  const terminalBlockHash = requireBlockHash(terminalReceipt, 'consumeFanout');
  if (terminalReceipt.blockNumber <= sameBlockNumber || terminalBlockHash === sameBlockHash) {
    throw new Error('terminal fixture transaction must consume persisted outputs in a later L1 block');
  }

  return {
    contract,
    contractAddress,
    handles: await collectHandles(contract),
    sameBlockNumber,
    sameBlockHash,
    terminalBlockNumber: terminalReceipt.blockNumber,
    terminalBlockHash,
  };
}
