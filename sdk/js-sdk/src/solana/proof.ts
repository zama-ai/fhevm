import { sha256 } from '@noble/hashes/sha2.js';

/**
 * Client-side MMR verification for the Zama Solana `EncryptedValue` ACL (RFC-024).
 *
 * Every hash primitive here MUST be byte-identical to the Rust shared crate
 * (`solana/crates/zama-solana-acl`), which is itself the single source of truth run
 * identically on-chain and in the KMS connector. This module lets the SDK verify an
 * MMR inclusion proof it received (e.g. from a relayer / indexer) BEFORE asking the
 * user to sign a decrypt request, so a malformed or stale proof is caught client-side
 * instead of silently failing (or worse, being blindly trusted) downstream.
 *
 * Domain-separation prefixes and encodings are pinned 1:1 to the Rust crate:
 * - `ZAMA_MMR_LEAF_V1` / `ZAMA_MMR_NODE_V1`     — MMR leaf/internal node hashing (`mmr.rs`).
 * - `ZAMA_HIST_ACCESS_LEAF_V1`                  — historical-access leaf commitment.
 * - `ZAMA_PUBLIC_DECRYPT_LEAF_V1`               — public-decrypt leaf commitment.
 * - `zama-encrypted-value-key-v1`               — the lineage value-key derivation.
 * - `leaf_index` is encoded big-endian (8 bytes) everywhere it is hashed.
 * - Leaf commitment preimages are `(account key ‖ leaf_index ‖ handle [‖ subject])`.
 */

const LEAF_PREFIX = utf8('ZAMA_MMR_LEAF_V1');
const NODE_PREFIX = utf8('ZAMA_MMR_NODE_V1');
const HISTORICAL_ACCESS_LEAF_PREFIX = utf8('ZAMA_HIST_ACCESS_LEAF_V1');
const PUBLIC_DECRYPT_LEAF_PREFIX = utf8('ZAMA_PUBLIC_DECRYPT_LEAF_V1');
const VALUE_KEY_PREFIX = utf8('zama-encrypted-value-key-v1');

function utf8(s: string): Uint8Array {
  return new TextEncoder().encode(s);
}

function concatBytes(...parts: readonly Uint8Array[]): Uint8Array {
  const total = parts.reduce((n, p) => n + p.length, 0);
  const out = new Uint8Array(total);
  let offset = 0;
  for (const part of parts) {
    out.set(part, offset);
    offset += part.length;
  }
  return out;
}

function assertLen(bytes: Uint8Array, len: number, name: string): void {
  if (bytes.length !== len) {
    throw new Error(`${name} must be exactly ${len} bytes, got ${bytes.length}`);
  }
}

/** Big-endian 8-byte encoding of a `leaf_index`/`u64`. Matches `to_be_bytes()` in Rust. */
function u64BE(value: bigint): Uint8Array {
  if (value < 0n || value > 0xffffffffffffffffn) {
    throw new Error(`u64BE: value out of range: ${value}`);
  }
  const out = new Uint8Array(8);
  const view = new DataView(out.buffer);
  view.setBigUint64(0, value, false);
  return out;
}

/** SHA-256 of the concatenation of `parts`. Matches the Rust crate's `sha256(&[...])` helper. */
function sha256Parts(...parts: readonly Uint8Array[]): Uint8Array {
  return sha256(concatBytes(...parts));
}

export function bytesToHex(bytes: Uint8Array): string {
  return `0x${Array.from(bytes, (b) => b.toString(16).padStart(2, '0')).join('')}`;
}

export function hexToBytes(hex: string): Uint8Array {
  const clean = hex.startsWith('0x') || hex.startsWith('0X') ? hex.slice(2) : hex;
  if (clean.length % 2 !== 0) {
    throw new Error(`hexToBytes: odd-length hex string: ${hex}`);
  }
  if (!/^[0-9a-fA-F]*$/.test(clean)) {
    throw new Error(`hexToBytes: invalid hex string: ${hex}`);
  }
  const out = new Uint8Array(clean.length / 2);
  for (let i = 0; i < out.length; i++) {
    out[i] = parseInt(clean.slice(i * 2, i * 2 + 2), 16);
  }
  return out;
}

/** The lineage's PDA seed / identity. Matches `zama_solana_acl::derive_value_key`. */
export function deriveValueKey(
  aclDomainKey: Uint8Array,
  appAccount: Uint8Array,
  encryptedValueLabel: Uint8Array,
): Uint8Array {
  assertLen(aclDomainKey, 32, 'aclDomainKey');
  assertLen(appAccount, 32, 'appAccount');
  assertLen(encryptedValueLabel, 32, 'encryptedValueLabel');
  return sha256Parts(VALUE_KEY_PREFIX, aclDomainKey, appAccount, encryptedValueLabel);
}

/** Matches `zama_solana_acl::mmr::mmr_leaf_node`. */
export function mmrLeafNode(commitment: Uint8Array): Uint8Array {
  assertLen(commitment, 32, 'commitment');
  return sha256Parts(LEAF_PREFIX, commitment);
}

/** Matches `zama_solana_acl::mmr::mmr_node`. */
export function mmrNode(left: Uint8Array, right: Uint8Array): Uint8Array {
  assertLen(left, 32, 'left');
  assertLen(right, 32, 'right');
  return sha256Parts(NODE_PREFIX, left, right);
}

/**
 * Matches `zama_solana_acl::historical_access_leaf_commitment`: the preimage of a
 * `HistoricalAccessLeaf { encrypted_value_account, leaf_index, handle, subject }`.
 */
export function historicalAccessLeafCommitment(
  encryptedValueAccount: Uint8Array,
  leafIndex: bigint,
  handle: Uint8Array,
  subject: Uint8Array,
): Uint8Array {
  assertLen(encryptedValueAccount, 32, 'encryptedValueAccount');
  assertLen(handle, 32, 'handle');
  assertLen(subject, 32, 'subject');
  return sha256Parts(HISTORICAL_ACCESS_LEAF_PREFIX, encryptedValueAccount, u64BE(leafIndex), handle, subject);
}

/**
 * Matches `zama_solana_acl::public_decrypt_leaf_commitment`: the preimage of a
 * `PublicDecryptLeaf { encrypted_value_account, leaf_index, handle }`.
 */
export function publicDecryptLeafCommitment(
  encryptedValueAccount: Uint8Array,
  leafIndex: bigint,
  handle: Uint8Array,
): Uint8Array {
  assertLen(encryptedValueAccount, 32, 'encryptedValueAccount');
  assertLen(handle, 32, 'handle');
  return sha256Parts(PUBLIC_DECRYPT_LEAF_PREFIX, encryptedValueAccount, u64BE(leafIndex), handle);
}

/** An MMR inclusion proof: sibling hashes from the leaf up to its mountain's peak. */
export type MmrProof = {
  readonly leafIndex: bigint;
  readonly siblings: readonly Uint8Array[];
};

/** Upper bound on `siblings`, matching the Rust connector's decode-time cap (`mmr.rs`, u64 height). */
export const MAX_MMR_SIBLINGS = 64;

/** Transport-blob mode byte for a historical-access MMR proof. */
export const MMR_MODE_HISTORICAL = 0x01;
/** Transport-blob mode byte for a public-decrypt MMR proof. */
export const MMR_MODE_PUBLIC = 0x02;

export type MmrProofTransportBlob = {
  readonly mode: number;
  readonly proof: MmrProof;
};

function dataView(bytes: Uint8Array): DataView {
  return new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
}

function requireRemaining(bytes: Uint8Array, offset: number, needed: number, field: string): void {
  const remaining = bytes.length - offset;
  if (remaining < needed) {
    throw new Error(
      `Solana MMR-proof blob is truncated while reading ${field}: need ${needed} bytes, got ${remaining}`,
    );
  }
}

/**
 * Decodes `mode || Borsh(MmrProof)` and requires the Borsh proof to consume the full blob.
 * Borsh encodes `MmrProof` as `leaf_index: u64 LE` then `siblings: Vec<[u8;32]>`.
 */
export function decodeMmrProofTransportBlob(mmrProofBytes: Uint8Array): MmrProofTransportBlob {
  if (mmrProofBytes.length === 0) {
    throw new Error('Solana MMR-proof blob is empty (missing mode byte)');
  }
  const mode = mmrProofBytes[0]!;
  const view = dataView(mmrProofBytes);
  let offset = 1;

  requireRemaining(mmrProofBytes, offset, 8, 'leaf_index');
  const leafIndex = view.getBigUint64(offset, true);
  offset += 8;

  requireRemaining(mmrProofBytes, offset, 4, 'siblings length');
  const siblingCount = view.getUint32(offset, true);
  offset += 4;
  if (siblingCount > MAX_MMR_SIBLINGS) {
    throw new Error(`Solana MMR proof carries ${siblingCount} siblings, exceeding the cap of ${MAX_MMR_SIBLINGS}`);
  }

  const expectedLength = offset + siblingCount * 32;
  if (mmrProofBytes.length < expectedLength) {
    throw new Error(
      `Solana MMR-proof blob is truncated while reading siblings: need ${expectedLength - offset} bytes, got ${
        mmrProofBytes.length - offset
      }`,
    );
  }
  if (mmrProofBytes.length > expectedLength) {
    throw new Error(
      `Solana MMR-proof blob has ${mmrProofBytes.length - expectedLength} trailing byte(s) after the Borsh proof`,
    );
  }

  const siblings: Uint8Array[] = [];
  for (let i = 0; i < siblingCount; i++) {
    const start = offset + i * 32;
    siblings.push(mmrProofBytes.slice(start, start + 32));
  }
  return { mode, proof: { leafIndex, siblings } };
}

function popcount64(value: bigint): number {
  let count = 0;
  let v = value;
  while (v > 0n) {
    count += Number(v & 1n);
    v >>= 1n;
  }
  return count;
}

/**
 * Verifies an MMR inclusion proof for `commitment` against `peaks` (the lineage's live MMR
 * peaks) and `leafCount` (the live leaf count). Port of `zama_solana_acl::mmr::mmr_verify`,
 * line-for-line: mountains correspond to the set bits of `leafCount`, most-significant first;
 * `leafIndex` selects which mountain (and thus which peak / expected proof height) the proof
 * targets, then the sibling path is folded bottom-up exactly as the Rust version does.
 */
export function mmrVerify(
  peaks: readonly Uint8Array[],
  leafCount: bigint,
  commitment: Uint8Array,
  proof: MmrProof,
): boolean {
  if (proof.siblings.length > MAX_MMR_SIBLINGS) {
    return false;
  }
  if (proof.leafIndex >= leafCount || peaks.length !== popcount64(leafCount)) {
    return false;
  }

  let offset = 0n;
  let peakPos = 0;
  for (let height = 63; height >= 0; height--) {
    const bit = 1n << BigInt(height);
    if ((leafCount & bit) === 0n) {
      continue;
    }
    if (proof.leafIndex >= offset && proof.leafIndex < offset + bit) {
      if (proof.siblings.length !== height) {
        return false;
      }
      let node = mmrLeafNode(commitment);
      let local = proof.leafIndex - offset;
      for (const sibling of proof.siblings) {
        assertLen(sibling, 32, 'sibling');
        node = local % 2n === 0n ? mmrNode(node, sibling) : mmrNode(sibling, node);
        local >>= 1n;
      }
      return bytesEqual(node, peaks[peakPos]!);
    }
    offset += bit;
    peakPos += 1;
  }
  return false;
}

function bytesEqual(a: Uint8Array, b: Uint8Array): boolean {
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) {
    if (a[i] !== b[i]) return false;
  }
  return true;
}

/**
 * Verifies a historical-access MMR proof: `subject` held access to `handle` at some point in
 * the lineage's history, provable against the LIVE peaks. Matches
 * `zama_solana_acl::authorize_historical`'s proof-verification step (membership/handle checks
 * on the current lineage state are the caller/KMS's job, not this client-side pre-check's).
 */
export function verifyHistoricalAccessProof(
  encryptedValueAccount: Uint8Array,
  peaks: readonly Uint8Array[],
  leafCount: bigint,
  handle: Uint8Array,
  subject: Uint8Array,
  proof: MmrProof,
): boolean {
  const commitment = historicalAccessLeafCommitment(encryptedValueAccount, proof.leafIndex, handle, subject);
  return mmrVerify(peaks, leafCount, commitment, proof);
}

/**
 * Verifies a public-decrypt MMR proof: `handle` was marked publicly decryptable at some point,
 * provable against the LIVE peaks. Matches `zama_solana_acl::authorize_public`'s
 * proof-verification step.
 */
export function verifyPublicDecryptProof(
  encryptedValueAccount: Uint8Array,
  peaks: readonly Uint8Array[],
  leafCount: bigint,
  handle: Uint8Array,
  proof: MmrProof,
): boolean {
  const commitment = publicDecryptLeafCommitment(encryptedValueAccount, proof.leafIndex, handle);
  return mmrVerify(peaks, leafCount, commitment, proof);
}
