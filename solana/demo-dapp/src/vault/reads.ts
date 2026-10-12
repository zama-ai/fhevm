import {
  getBase58Decoder,
  getBase64Encoder,
  type Address,
  type Base58EncodedBytes,
  type Rpc,
  type SolanaRpcApi,
} from '@solana/kit';

import {
  type FhevmSolanaBaseClient,
  type SolanaEncryptedStore,
} from '@fhevm/sdk/solana';
import { fetchBatch, type Batch } from './internal/generated/confidentialBatcher/accounts/batch.js';
import { fetchBatcher, type Batcher } from './internal/generated/confidentialBatcher/accounts/batcher.js';
import {
  fetchAllMaybeJoinRecord,
  fetchJoinRecord,
  getJoinRecordDecoder,
  getJoinRecordSize,
  JOIN_RECORD_DISCRIMINATOR,
  type JoinRecord,
} from './internal/generated/confidentialBatcher/accounts/joinRecord.js';
import { findBatchPda } from './internal/generated/confidentialBatcher/pdas/batch.js';
import { findJoinRecordPda } from './internal/generated/confidentialBatcher/pdas/joinRecord.js';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './internal/generated/confidentialBatcher/programAddress.js';
import { deriveBatchAddresses, type BatchAddresses, type VaultDemoRoots } from './derive.js';

/** The batcher config's decoded on-chain state (generated decoder). */
export type BatcherState = Batcher;
/** A batch's decoded on-chain state (generated decoder). */
export type BatchState = Batch;
/** A `(batch, user)` join record's decoded on-chain state (generated decoder). */
export type JoinRecordState = JoinRecord;

type SolanaRpc = Rpc<SolanaRpcApi>;

/** Reads a batcher config via the generated `Batcher` decoder. */
export async function getBatcher(rpc: SolanaRpc, batcher: Address): Promise<BatcherState> {
  const account = await fetchBatcher(rpc, batcher);
  return account.data;
}

/** Unix time from which a pending batch may be dispatched. */
export function dispatchableAt(batch: BatchState, batcher: BatcherState): bigint {
  return batch.openedAt + batcher.minBatchAgeSecs;
}

/** Unix time from which a dispatched batch can no longer settle and anyone may cancel it. */
export function settleDeadline(batch: BatchState, batcher: BatcherState): bigint {
  return batch.dispatchedAt + batcher.settleDeadlineSecs;
}

/**
 * Reads a `(batch, user)` join record via the generated `JoinRecord` decoder — derive the address
 * with the generated `findJoinRecordPda`. Throws if the record does not exist (the user never joined the
 * batch).
 */
export async function getJoinRecord(
  rpc: SolanaRpc,
  joinRecord: Address,
): Promise<JoinRecordState> {
  const account = await fetchJoinRecord(rpc, joinRecord);
  return account.data;
}

/** Reads every join record of `batch` that still exists: one per participant who has not exited. */
export async function getBatchJoinRecords(rpc: SolanaRpc, batch: Address): Promise<JoinRecordState[]> {
  const memcmp = (offset: bigint, bytes: Base58EncodedBytes) => ({ memcmp: { offset, bytes, encoding: 'base58' as const } });
  const accounts = await rpc
    .getProgramAccounts(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, {
      encoding: 'base64',
      filters: [
        { dataSize: BigInt(getJoinRecordSize()) },
        memcmp(0n, getBase58Decoder().decode(JOIN_RECORD_DISCRIMINATOR) as Base58EncodedBytes),
        memcmp(8n, batch as string as Base58EncodedBytes),
      ],
    })
    .send();
  return accounts.map(({ account }) => getJoinRecordDecoder().decode(getBase64Encoder().encode(account.data[0])));
}

/**
 * Resolves the batcher's pending batch: its zero-based index, its derived addresses, and its decoded
 * state. `Batcher.nextBatchIndex` is the index the next batch will open at, so the pending batch is
 * one before it.
 */
export async function getCurrentBatch(
  rpc: SolanaRpc,
  roots: VaultDemoRoots,
): Promise<{ index: bigint; addresses: BatchAddresses; state: BatchState }> {
  const batcher = await getBatcher(rpc, roots.batcher);
  return getBatchByIndex(rpc, roots, batcher.nextBatchIndex - 1n);
}

/** Reads one pinned historical or current batch by its index. */
export async function getBatchByIndex(
  rpc: SolanaRpc,
  roots: VaultDemoRoots,
  index: bigint,
): Promise<{ index: bigint; addresses: BatchAddresses; state: BatchState }> {
  const addresses = await deriveBatchAddresses(roots, index);
  const account = await fetchBatch(rpc, addresses.batch);
  return { index, addresses, state: account.data };
}

/** How many batches, the pending one included, a wallet's join is looked for in. */
export const RECENT_BATCH_SCAN_LIMIT = 32n;

/**
 * Finds `user`'s join record in `preferred`, when it is one of the batcher's batches, and then in the
 * last `RECENT_BATCH_SCAN_LIMIT` batches, newest first. A join therefore stays findable after its
 * batch is dispatched, through settle and claim.
 */
export async function findRecentJoin(
  rpc: SolanaRpc,
  roots: VaultDemoRoots,
  user: Address,
  preferred?: { readonly batchIndex: bigint; readonly batch: Address },
): Promise<{ readonly batchIndex: bigint; readonly batch: Address; readonly joinRecord: JoinRecordState } | null> {
  const candidates: { batchIndex: bigint; batch: Address }[] = [];
  const batchAt = async (index: bigint) => (await findBatchPda({ batcher: roots.batcher, index }))[0];
  if (preferred !== undefined && (await batchAt(preferred.batchIndex)) === preferred.batch) {
    candidates.push({ batchIndex: preferred.batchIndex, batch: preferred.batch });
  }
  const pending = (await getBatcher(rpc, roots.batcher)).nextBatchIndex - 1n;
  const oldest = pending >= RECENT_BATCH_SCAN_LIMIT ? pending - RECENT_BATCH_SCAN_LIMIT + 1n : 0n;
  for (let batchIndex = pending; batchIndex >= oldest; batchIndex -= 1n) {
    const batch = await batchAt(batchIndex);
    if (batch !== candidates[0]?.batch) candidates.push({ batchIndex, batch });
  }
  const records = await fetchAllMaybeJoinRecord(
    rpc,
    await Promise.all(candidates.map(async ({ batch }) => (await findJoinRecordPda({ batch, user }))[0])),
  );
  for (const [index, record] of records.entries()) {
    if (record.exists) return { ...candidates[index]!, joinRecord: record.data };
  }
  return null;
}

/** Reads the host-owned slot dictionary and shared MMR at `finalized` through the SDK decoder. */
export function getEncryptedStore(
  client: Pick<FhevmSolanaBaseClient, 'fetchEncryptedStore'>,
  address: Address,
): Promise<SolanaEncryptedStore> {
  return client.fetchEncryptedStore(address);
}
