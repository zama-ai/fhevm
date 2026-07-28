import {
  fetchEncodedAccount,
  fixDecoderSize,
  getArrayDecoder,
  getBytesDecoder,
  getStructDecoder,
  getU32Decoder,
  getU64Decoder,
  getU8Decoder,
  type Address,
  type FetchAccountConfig,
  type Rpc,
  type SolanaRpcApi,
} from '@solana/kit';

import type { Bytes32 } from '../../core/types/primitives.js';
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

/**
 * The subset of an `EncryptedValue` encrypted value account the settle phases need: the live handle, the MMR leaf
 * count, and the live peaks that a proof is verified against.
 *
 * (a) DELIBERATE, REVIEWED DEVIATION — hand-rolled, not generated. Every other account decoder in
 *     this module is Codama-generated, but `EncryptedValue` cannot be: it is defined in the
 *     `zama-solana-acl` crate and used as an Anchor `Account` with a manually-computed discriminator,
 *     and is NOT declared in any program's `#[program]`, so it never appears in an Anchor IDL and
 *     Codama has nothing to generate from. This declarative codec is therefore maintained by hand.
 *
 * (b) ASSUMED ON-CHAIN LAYOUT (borsh, after the 8-byte account discriminator), mirroring the crate's
 *     `#[derive(BorshDeserialize)]` field order in `solana/crates/zama-solana-acl/src/lib.rs`:
 *       [8-byte discriminator][aclDomainKey: 32][appAccount: 32][encryptedValueLabel: 32]
 *       [currentHandle: 32][subjects: Vec<32-byte grant>][leafCount: u64][peaks: Vec<32>][bump: u8]
 *     The discriminator is sliced off before this decoder runs; `subjects` is decoded and discarded
 *     (the settle phases never read it — only its length matters, to advance the cursor).
 *
 * (c) FRAGILITY — each `subjects` element is decoded as a bare 32-byte grant. If the crate's
 *     `EncryptedValueSubjectGrant` ever gains a field, its element size stops being 32 and THIS
 *     decoder can misalign everything after it (`leafCount`, `peaks`, `bump`). The structural
 *     checks below reject impossible MMR shapes and non-32-byte trailing capacity. Anyone who
 *     changes that struct MUST still update the decoder here in lockstep. Do not treat 32 as
 *     incidental.
 */
const encryptedValueBodyDecoder = getStructDecoder([
  ['aclDomainKey', fixDecoderSize(getBytesDecoder(), 32)],
  ['appAccount', fixDecoderSize(getBytesDecoder(), 32)],
  ['encryptedValueLabel', fixDecoderSize(getBytesDecoder(), 32)],
  ['currentHandle', fixDecoderSize(getBytesDecoder(), 32)],
  ['subjects', getArrayDecoder(fixDecoderSize(getBytesDecoder(), 32), { size: getU32Decoder() })],
  ['leafCount', getU64Decoder()],
  ['peaks', getArrayDecoder(fixDecoderSize(getBytesDecoder(), 32), { size: getU32Decoder() })],
  ['bump', getU8Decoder()],
]);

const ENCRYPTED_VALUE_DISCRIMINATOR_SIZE = 8;
const ENCRYPTED_VALUE_VECTOR_ELEMENT_SIZE = 32;

const popcount = (value: bigint): number => {
  let count = 0;
  for (let remaining = value; remaining > 0n; remaining >>= 1n) count += Number(remaining & 1n);
  return count;
};

export async function getEncryptedValueState(
  rpc: SolanaRpc,
  address: Address,
): Promise<{ currentHandle: Bytes32; leafCount: bigint; peaks: Uint8Array[] }> {
  const account = await fetchEncodedAccount(rpc, address);
  if (!account.exists) throw new Error(`EncryptedValue account ${address} does not exist`);
  const body = account.data.slice(ENCRYPTED_VALUE_DISCRIMINATOR_SIZE);
  // `EncryptedValue` realloc-grows but never shrinks, so a shorter current `subjects`/`peaks`
  // serialization may leave stale high-water capacity after the live borsh value. Each capacity
  // step is exactly one 32-byte vector element. Reject any other trailing shape, and independently
  // verify the MMR invariant so a misaligned hand-written decoder still fails loudly.
  const [decoded, offset] = encryptedValueBodyDecoder.read(body, 0);
  const trailingCapacity = body.length - offset;
  const expectedPeakCount = popcount(decoded.leafCount);
  if (
    trailingCapacity < 0 ||
    trailingCapacity % ENCRYPTED_VALUE_VECTOR_ELEMENT_SIZE !== 0 ||
    decoded.peaks.length !== expectedPeakCount
  ) {
    throw new Error(
      `EncryptedValue account ${address}: decoded ${decoded.peaks.length} MMR peaks for leaf count ` +
        `${decoded.leafCount} and consumed ${offset} of ${body.length} body bytes (after the ` +
        `${ENCRYPTED_VALUE_DISCRIMINATOR_SIZE}-byte discriminator) — the on-chain layout has drifted ` +
        `from this decoder. Re-check the crate's EncryptedValue struct and update reads.ts.`,
    );
  }
  return {
    currentHandle: new Uint8Array(decoded.currentHandle) as Bytes32,
    leafCount: decoded.leafCount,
    peaks: decoded.peaks.map((peak) => new Uint8Array(peak)),
  };
}
