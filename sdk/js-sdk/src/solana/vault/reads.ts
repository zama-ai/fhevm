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
  type Rpc,
  type SolanaRpcApi,
} from '@solana/kit';

import type { Bytes32 } from '../../core/types/primitives.js';
import { fetchBatch, type Batch } from './internal/generated/confidentialBatcher/accounts/batch.js';
import { fetchBatcher, type Batcher } from './internal/generated/confidentialBatcher/accounts/batcher.js';
import { deriveBatchAddresses, type BatchAddresses, type VaultDemoRoots } from './derive.js';

/** The batcher config's decoded on-chain state (generated decoder). */
export type BatcherState = Batcher;
/** A batch's decoded on-chain state (generated decoder). */
export type BatchState = Batch;

type SolanaRpc = Rpc<SolanaRpcApi>;

/** Reads a batcher config via the generated `Batcher` decoder. */
export async function getBatcher(rpc: SolanaRpc, batcher: Address): Promise<BatcherState> {
  const account = await fetchBatcher(rpc, batcher);
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
): Promise<{ index: bigint; addresses: BatchAddresses; state: BatchState }> {
  const batcher = await getBatcher(rpc, roots.batcher);
  if (batcher.nextBatchIndex === 0n) {
    throw new Error(`batcher ${roots.batcher} has opened no batches yet (nextBatchIndex is 0)`);
  }
  const index = batcher.nextBatchIndex - 1n;
  const addresses = await deriveBatchAddresses(roots, index);
  const account = await fetchBatch(rpc, addresses.batch);
  return { index, addresses, state: account.data };
}

/**
 * The subset of an `EncryptedValue` lineage the settle legs need: the live handle, the MMR leaf
 * count, and the live peaks that a proof is verified against.
 *
 * `EncryptedValue` is defined in the `zama-solana-acl` crate and used as an Anchor `Account` with a
 * manually-computed discriminator; it is NOT declared in any program's `#[program]`, so it never
 * appears in an Anchor IDL and cannot be a Codama-generated decoder. This declarative codec mirrors
 * the crate's `#[derive(BorshDeserialize)]` field order exactly (see
 * `solana/crates/zama-solana-acl/src/lib.rs::EncryptedValue`); the 8-byte account discriminator is
 * skipped before the borsh body. `subjects` is decoded and discarded — the settle legs never read it.
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

export async function getEncryptedValueState(
  rpc: SolanaRpc,
  address: Address,
): Promise<{ currentHandle: Bytes32; leafCount: bigint; peaks: Uint8Array[] }> {
  const account = await fetchEncodedAccount(rpc, address);
  if (!account.exists) throw new Error(`EncryptedValue account ${address} does not exist`);
  const decoded = encryptedValueBodyDecoder.decode(account.data.slice(ENCRYPTED_VALUE_DISCRIMINATOR_SIZE));
  return {
    currentHandle: new Uint8Array(decoded.currentHandle) as Bytes32,
    leafCount: decoded.leafCount,
    peaks: decoded.peaks.map((peak) => new Uint8Array(peak)),
  };
}
