import { bytesToHex, hexToBytes } from '../core/base/bytes.js';
import { fetchWithRetry } from '../core/base/fetch.js';

/**
 * Encrypted-value-ACL MMR inclusion proof for a historical or public confidential-balance decrypt,
 * shaped to pass straight as the `encryptedValue` parameter of the Solana {@link userDecrypt} action.
 */
export type SolanaDecryptProof = {
  /** 32-byte lineage identity (`acl_nonce_key`). */
  readonly aclValueKey: Uint8Array;
  readonly proof: {
    /** The lineage `leaf_count` the proof was built against — the staleness marker. */
    readonly proofSlot: bigint;
    /** Mode-prefixed (`0x01` historical / `0x02` public) Borsh proof blob. */
    readonly mmrProof: Uint8Array;
  };
};

/** `POST /build_proof` response from the rotation-leaf indexer. */
type BuildProofResponse = {
  readonly mmr_proof_bytes: string;
  readonly leaf_count: number;
  readonly verified: boolean;
};

/**
 * Fetches an MMR inclusion proof from the rotation-leaf indexer's `POST /build_proof` for a
 * historical or public confidential-balance decrypt.
 *
 * `valueKey` is the 32-byte lineage identity (`acl_nonce_key`); `leafIndex` selects the historical
 * (rotated-away) leaf or the public-decrypt leaf within that lineage. The returned `proofSlot` is
 * the lineage `leaf_count` the proof was built against.
 *
 * **Rebuild-on-stale.** The proof is verified by the KMS against the LIVE on-chain lineage peaks,
 * not pinned to `proofSlot`. A proof stays valid across rotations until its mountain MERGES; once it
 * does, the KMS reports the decrypt recoverable-stale. Recovery is: call this again (the lineage has
 * advanced, so it returns a fresh proof with a higher `proofSlot`) and resubmit {@link userDecrypt}.
 * Because the relayer's dedup hash includes the proof, a rebuilt proof is a new, re-processed
 * request — not a dedup hit on the dead one.
 */
export async function fetchSolanaDecryptProof(args: {
  readonly indexerUrl: string;
  readonly valueKey: Uint8Array;
  readonly leafIndex: bigint;
}): Promise<SolanaDecryptProof> {
  const { indexerUrl, valueKey, leafIndex } = args;
  if (valueKey.length !== 32) {
    throw new Error(`valueKey must be 32 bytes, got ${valueKey.length}`);
  }
  const url = `${indexerUrl.replace(/\/+$/, '')}/build_proof`;
  const response = await fetchWithRetry({
    url,
    init: {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      // parse_key strips an optional `0x`; leaf_index is a u64 (safe-integer leaf counts).
      body: JSON.stringify({ value_key: bytesToHex(valueKey), leaf_index: Number(leafIndex) }),
    },
  });
  if (!response.ok) {
    throw new Error(`indexer /build_proof failed (${response.status}): ${await response.text().catch(() => '')}`);
  }
  const body = (await response.json()) as BuildProofResponse;
  // The indexer returns 200 with `verified = false` when it could not cross-check the proof against
  // the live on-chain account (RPC unavailable). Signing such a proof would push an unverified
  // request into the KMS and fail there with a far less diagnosable error, so reject it here.
  if (!body.verified) {
    throw new Error(
      'indexer /build_proof returned an UNVERIFIED proof (on-chain cross-check unavailable); refusing to sign it',
    );
  }
  return {
    aclValueKey: valueKey,
    proof: {
      proofSlot: BigInt(body.leaf_count),
      mmrProof: hexToBytes(body.mmr_proof_bytes),
    },
  };
}
