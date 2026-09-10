import type { Address, FetchAccountConfig, Rpc, SolanaRpcApi } from '@solana/kit';

import {
  fetchSolanaEncryptedState,
  type SolanaEncryptedState,
} from '@sdk-src/solana/encryptedState.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from './internal/generated/confidentialToken/programAddress.js';
import { fetchBatch, type Batch } from './internal/generated/confidentialBatcher/accounts/batch.js';
import { fetchBatcher, type Batcher } from './internal/generated/confidentialBatcher/accounts/batcher.js';
import { fetchJoinRecord, type JoinRecord } from './internal/generated/confidentialBatcher/accounts/joinRecord.js';
import { deriveBatchAddresses, type BatchAddresses, type VaultDemoRoots } from './derive.js';

/** The batcher config's decoded on-chain state (generated decoder). */
export type BatcherState = Batcher;
/** A batch's decoded on-chain state (generated decoder). */
export type BatchState = Batch;
/** A `(batch, user)` join record's decoded on-chain state (generated decoder). */
export type JoinRecordState = JoinRecord;

type SolanaRpc = Rpc<SolanaRpcApi>;

/** Reads a batcher config via the generated `Batcher` decoder. */
export async function getBatcher(rpc: SolanaRpc, batcher: Address, config?: FetchAccountConfig): Promise<BatcherState> {
  const account = await fetchBatcher(rpc, batcher, config);
  return account.data;
}

/**
 * Reads a `(batch, user)` join record via the generated `JoinRecord` decoder — derive the address
 * with `deriveJoinRecordAddress`. Throws if the record does not exist (the user never joined the
 * batch). `config` is the standard fetch passthrough, e.g. `{ commitment: 'confirmed' }` to observe
 * a claim before finalization.
 */
export async function getJoinRecord(
  rpc: SolanaRpc,
  joinRecord: Address,
  config?: FetchAccountConfig,
): Promise<JoinRecordState> {
  const account = await fetchJoinRecord(rpc, joinRecord, config);
  return account.data;
}

/**
 * Resolves the batcher's most-recently-opened batch: its zero-based index, its derived addresses,
 * and its decoded state. `Batcher.nextBatchIndex` is the index the *next* `open_batch` will use, so
 * the current batch is one before it. Throws if no batch has been opened yet (index 0).
 */
export async function getCurrentBatch(
  rpc: SolanaRpc,
  roots: VaultDemoRoots,
  config?: FetchAccountConfig,
): Promise<{ index: bigint; addresses: BatchAddresses; state: BatchState }> {
  const batcher = await getBatcher(rpc, roots.batcher, config);
  if (batcher.nextBatchIndex === 0n) {
    throw new Error(`batcher ${roots.batcher} has opened no batches yet (nextBatchIndex is 0)`);
  }
  const index = batcher.nextBatchIndex - 1n;
  return getBatchByIndex(rpc, roots, index, config);
}

/** Reads one pinned historical or current batch by its index. */
export async function getBatchByIndex(
  rpc: SolanaRpc,
  roots: VaultDemoRoots,
  index: bigint,
  config?: FetchAccountConfig,
): Promise<{ index: bigint; addresses: BatchAddresses; state: BatchState }> {
  const addresses = await deriveBatchAddresses(roots, index);
  const account = await fetchBatch(rpc, addresses.batch, config);
  return { index, addresses, state: account.data };
}

/** Reads the host-owned slot dictionary and shared MMR through the SDK decoder. */
export function getEncryptedState(
  rpc: SolanaRpc,
  address: Address,
  config?: FetchAccountConfig,
): Promise<SolanaEncryptedState> {
  return fetchSolanaEncryptedState(rpc, address, config, ZAMA_HOST_PROGRAM_ADDRESS);
}
