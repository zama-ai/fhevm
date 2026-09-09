import { keccak_256 } from '@noble/hashes/sha3.js';

/**
 * Client-side MMR primitives for the Zama Solana `EncryptedValue` ACL (RFC 035).
 *
 * Every hash primitive here MUST be byte-identical to the Rust shared crate
 * (`solana/crates/zama-solana-acl`), which is the single source of truth run identically on-chain,
 * in the coprocessor and in the KMS connector. The committed leaf vectors
 * (`solana/test-fixtures/leaves/leaves_v1.json`) are what prove the agreement.
 *
 * Two uses. A dapp that knows an encrypted value account's history rebuilds its leaf list with
 * {@link reconstructSolanaEncryptedValueAccount}, checks the peaks against the on-chain account and
 * builds the public-leaf inclusion proof `verify_public_decrypt` takes on chain. A verifier that
 * received a proof checks it with {@link verifyPublicDecryptProof} or
 * {@link verifyHistoricalAccessProof} before acting on it.
 *
 * Domain-separation prefixes and encodings are pinned 1:1 to the Rust crate:
 * - `ZAMA_MMR_LEAF_V1` / `ZAMA_MMR_NODE_V1`     — MMR leaf/internal node hashing (`mmr.rs`).
 * - `ZAMA_HIST_ACCESS_LEAF_V1`                  — historical-access leaf commitment.
 * - `ZAMA_PUBLIC_DECRYPT_LEAF_V1`               — public-decrypt leaf commitment.
 * - `leaf_index` is encoded big-endian (8 bytes) everywhere it is hashed.
 * - Leaf commitment preimages are `(account key ‖ leaf_index ‖ handle [‖ key])`.
 */

const LEAF_PREFIX = utf8('ZAMA_MMR_LEAF_V1');
const NODE_PREFIX = utf8('ZAMA_MMR_NODE_V1');
const HISTORICAL_ACCESS_LEAF_PREFIX = utf8('ZAMA_HIST_ACCESS_LEAF_V1');
const PUBLIC_DECRYPT_LEAF_PREFIX = utf8('ZAMA_PUBLIC_DECRYPT_LEAF_V1');

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
export function u64BE(value: bigint): Uint8Array {
  if (value < 0n || value > 0xffffffffffffffffn) {
    throw new Error(`u64BE: value out of range: ${value}`);
  }
  const out = new Uint8Array(8);
  const view = new DataView(out.buffer);
  view.setBigUint64(0, value, false);
  return out;
}

/** keccak256 of the concatenation of `parts`. Matches the Rust crate's `keccak256(&[...])` helper. */
function keccak256Parts(...parts: readonly Uint8Array[]): Uint8Array {
  return keccak_256(concatBytes(...parts));
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

/** Matches `zama_solana_acl::mmr::mmr_leaf_node`. */
export function mmrLeafNode(commitment: Uint8Array): Uint8Array {
  assertLen(commitment, 32, 'commitment');
  return keccak256Parts(LEAF_PREFIX, commitment);
}

/** Matches `zama_solana_acl::mmr::mmr_node`. */
export function mmrNode(left: Uint8Array, right: Uint8Array): Uint8Array {
  assertLen(left, 32, 'left');
  assertLen(right, 32, 'right');
  return keccak256Parts(NODE_PREFIX, left, right);
}

/**
 * Matches `zama_solana_acl::historical_access_leaf_commitment`: the preimage of a
 * `HistoricalAccessLeaf { encrypted_value_account, leaf_index, handle, key }` — one allow of
 * `key` on `handle`.
 */
export function historicalAccessLeafCommitment(
  encryptedValueAccount: Uint8Array,
  leafIndex: bigint,
  handle: Uint8Array,
  key: Uint8Array,
): Uint8Array {
  assertLen(encryptedValueAccount, 32, 'encryptedValueAccount');
  assertLen(handle, 32, 'handle');
  assertLen(key, 32, 'key');
  return keccak256Parts(HISTORICAL_ACCESS_LEAF_PREFIX, encryptedValueAccount, u64BE(leafIndex), handle, key);
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
  return keccak256Parts(PUBLIC_DECRYPT_LEAF_PREFIX, encryptedValueAccount, u64BE(leafIndex), handle);
}

/** An MMR inclusion proof: sibling hashes from the leaf up to its mountain's peak. */
export type MmrProof = {
  readonly leafIndex: bigint;
  readonly siblings: readonly Uint8Array[];
};

/** Upper bound on the sibling path; a 64-bit leaf count has at most 64 levels. */
export const MAX_MMR_SIBLINGS = 64;

////////////////////////////////////////////////////////////////////////////////
// Reconstruction: the leaf list an account's history implies
////////////////////////////////////////////////////////////////////////////////

/**
 * One leaf-appending operation in an encrypted value account's history, in chronological order.
 * Mirrors `zama_solana_acl::encrypted_value_account::EncryptedValueAccountEvent`.
 */
export type SolanaEncryptedValueAccountEvent =
  /** One `allow` of `key` on `handle`, sealed by the write that installed `handle`. */
  | { readonly kind: 'allowed'; readonly handle: Uint8Array; readonly key: Uint8Array }
  /** `handle` was made publicly decryptable. */
  | { readonly kind: 'markedPublic'; readonly handle: Uint8Array };

/** The full ordered leaf list of an encrypted value account plus the MMR state it implies. */
export type SolanaReconstructedEncryptedValueAccount = {
  readonly leaves: readonly Uint8Array[];
  readonly leafCount: bigint;
  readonly peaks: readonly Uint8Array[];
};

/**
 * Rebuilds the full ordered leaf list from an account's chronological events, exactly as the host
 * program appends them: one commitment per event, the leaf index bound into each from a single
 * running counter. Matches `zama_solana_acl::encrypted_value_account::reconstruct`.
 *
 * The caller cross-checks `peaks` and `leafCount` against the on-chain account before trusting a
 * proof built from this: a missed or reordered event yields a different leaf list whose peaks
 * diverge, and a proof built from it would be rejected at verify time.
 *
 * @param encryptedValueAccount - The 32-byte account address the leaves bind.
 * @param events - The account's history, oldest first.
 */
export function reconstructSolanaEncryptedValueAccount(
  encryptedValueAccount: Uint8Array,
  events: readonly SolanaEncryptedValueAccountEvent[],
): SolanaReconstructedEncryptedValueAccount {
  const leaves = events.map((event, index) => {
    const leafIndex = BigInt(index);
    switch (event.kind) {
      case 'allowed':
        return historicalAccessLeafCommitment(encryptedValueAccount, leafIndex, event.handle, event.key);
      case 'markedPublic':
        return publicDecryptLeafCommitment(encryptedValueAccount, leafIndex, event.handle);
    }
  });
  return { leaves, leafCount: BigInt(leaves.length), peaks: mmrPeaksFromLeaves(leaves) };
}

/**
 * The MMR peaks of a leaf list, oldest mountain first. Matches
 * `zama_solana_acl::mmr::mmr_peaks_from_leaves`.
 *
 * @param leaves - Leaf commitments in append order.
 */
/**
 * The inclusion proof of leaf `publicLeafIndex` in the encrypted value account `account`, rebuilt
 * from the account's chronological `history` and cross-checked against the `live` peaks and leaf
 * count. Throws when the live account disagrees with the history: a proof built from a wrong history
 * would only fail later, inside the on-chain verifier, with nothing pointing at the leaf.
 */
export function buildPublicLeafProof(
  account: Uint8Array,
  live: { readonly leafCount: bigint; readonly peaks: readonly Uint8Array[] },
  history: readonly SolanaEncryptedValueAccountEvent[],
  publicLeafIndex: bigint,
): MmrProof {
  const rebuilt = reconstructSolanaEncryptedValueAccount(account, history);
  const samePeaks =
    rebuilt.leafCount === live.leafCount &&
    rebuilt.peaks.length === live.peaks.length &&
    rebuilt.peaks.every((peak, index) => {
      const livePeak = live.peaks[index];
      return livePeak !== undefined && bytesToHex(peak) === bytesToHex(livePeak);
    });
  if (!samePeaks) {
    throw new Error(
      `encrypted value account holds ${live.leafCount} leaves that do not match the ` +
        `${rebuilt.leafCount}-leaf history expected of it`,
    );
  }
  if (history[Number(publicLeafIndex)]?.kind !== 'markedPublic') {
    throw new Error(`leaf ${publicLeafIndex} of the expected history is not a public leaf`);
  }
  const proof = mmrBuildProof(rebuilt.leaves, publicLeafIndex);
  if (proof === undefined) throw new Error(`leaf ${publicLeafIndex} is outside the ${rebuilt.leafCount}-leaf history`);
  return proof;
}

export function mmrPeaksFromLeaves(leaves: readonly Uint8Array[]): readonly Uint8Array[] {
  // The append algorithm: push a height-0 node, then merge while the two topmost mountains have
  // the same height. What remains is the peak list, oldest mountain first.
  const stack: Array<{ node: Uint8Array; height: number }> = [];
  for (const leaf of leaves) {
    let current = { node: mmrLeafNode(leaf), height: 0 };
    for (;;) {
      const top = stack.at(-1);
      if (top?.height !== current.height) {
        break;
      }
      stack.pop();
      current = { node: mmrNode(top.node, current.node), height: current.height + 1 };
    }
    stack.push(current);
  }
  return stack.map((entry) => entry.node);
}

/**
 * The inclusion proof for the leaf at `leafIndex`, or `undefined` if it is out of range. Matches
 * `zama_solana_acl::mmr::mmr_build_proof`.
 *
 * @param leaves - Leaf commitments in append order.
 * @param leafIndex - The leaf to prove.
 */
export function mmrBuildProof(leaves: readonly Uint8Array[], leafIndex: bigint): MmrProof | undefined {
  const count = BigInt(leaves.length);
  if (leafIndex < 0n || leafIndex >= count) {
    return undefined;
  }
  let offset = 0n;
  for (let height = 63; height >= 0; height--) {
    const bit = 1n << BigInt(height);
    if ((count & bit) === 0n) {
      continue;
    }
    if (leafIndex >= offset && leafIndex < offset + bit) {
      let level = leaves.slice(Number(offset), Number(offset + bit)).map(mmrLeafNode);
      let local = Number(leafIndex - offset);
      const siblings: Uint8Array[] = [];
      while (level.length > 1) {
        const sibling = level[local % 2 === 0 ? local + 1 : local - 1];
        if (sibling === undefined) {
          throw new Error('mmrBuildProof: a complete mountain has a sibling at every level');
        }
        siblings.push(sibling);
        const next: Uint8Array[] = [];
        for (let i = 0; i + 1 < level.length; i += 2) {
          const left = level[i];
          const right = level[i + 1];
          if (left === undefined || right === undefined) {
            throw new Error('mmrBuildProof: a complete mountain pairs every node');
          }
          next.push(mmrNode(left, right));
        }
        level = next;
        local = Math.floor(local / 2);
      }
      return { leafIndex, siblings };
    }
    offset += bit;
  }
  return undefined;
}

////////////////////////////////////////////////////////////////////////////////
// Verification
////////////////////////////////////////////////////////////////////////////////

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
 * Verifies that `commitment` is the leaf at `proof.leafIndex` of the MMR with these `peaks` and
 * `leafCount`. Matches `zama_solana_acl::mmr::mmr_verify`.
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
      const peak = peaks[peakPos];
      return peak !== undefined && bytesEqual(node, peak);
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

/** Matches `zama_solana_acl::authorize_historical`: one allow of `key` on `handle` is proven. */
export function verifyHistoricalAccessProof(
  encryptedValueAccount: Uint8Array,
  peaks: readonly Uint8Array[],
  leafCount: bigint,
  handle: Uint8Array,
  key: Uint8Array,
  proof: MmrProof,
): boolean {
  const commitment = historicalAccessLeafCommitment(encryptedValueAccount, proof.leafIndex, handle, key);
  return mmrVerify(peaks, leafCount, commitment, proof);
}

/** Matches `zama_solana_acl::authorize_public`: `handle` was made public, at exactly this leaf. */
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
