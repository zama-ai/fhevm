import type { FheValueType, NetworkName } from "@cli-fhevm-sdk/toolkit";

import type { FlowKind } from "../relayer/types";

/**
 * Pool formats. Pools are plain JSON on disk so other injectors (k6) can
 * consume them later; values are serialized as strings because several FHE
 * types exceed JS number range.
 */

export type PoolKind = "input-proof-payloads" | "fhe-handles";

export type PoolMeta = Readonly<{
  kind: PoolKind;
  /** Flow the pool serves; `fhe-handles` pools are flow-specific (ACL setup differs). */
  flow: FlowKind;
  network: NetworkName;
  contractChainId: number;
  contractAddress: string;
  createdAt: string;
  /** Number of items in the immutable JSONL snapshot referenced by metadata. */
  count: number;
  /** Relayer the payloads were generated against (key material origin). */
  relayerUrl?: string;
  /** fhe-handles pools: HD lane indices, or PRIVATE_KEY_LANE (-1), used for owners. */
  ownerIndices?: readonly number[];
  /** delegated pools: HD lane index, or DELEGATE_KEY_LANE (-2), of the delegate. */
  delegateIndex?: number;
  delegateAddress?: string;
  /** delegated pools: earliest recorded owner delegation expiration. */
  delegationExpiration?: string;
  /** delegated pools: expiration by decimal owner lane index. */
  delegationExpirations?: Readonly<Record<string, string>>;
}>;

/** Exact v2 metadata representation accepted from and returned by PoolStore. */
export type PersistedPoolMeta = PoolMeta &
  Readonly<{
    schemaVersion: 2;
    /** Digest-addressed immutable snapshot; `meta.json` is the commit pointer. */
    itemsFile: string;
    /** SHA-256 corruption check of the exact snapshot bytes (not authentication). */
    itemsDigest: Readonly<{ algorithm: "sha256"; value: string }>;
  }>;

/**
 * One pre-generated input-proof request. Single-use: the relayer dedups on
 * (chainId, contract, user, ciphertext, extraData) and a completed request
 * blocks identical resubmission forever.
 */
export type InputProofPoolItem = Readonly<{
  index: number;
  contractChainId: number;
  contractAddress: string;
  userAddress: string;
  /** Raw hex, NO `0x` prefix — exactly what the relayer expects. */
  ciphertextWithInputVerification: string;
  /** `0x`-prefixed. */
  extraData: string;
  /** 32-byte handles the relayer must return when it accepts the proof. */
  expectedHandles: readonly string[];
  /** Clear values that were encrypted, for the record. */
  values: readonly Readonly<{ type: FheValueType; value: string }>[];
}>;

/**
 * One stored FHETest handle with its known plaintext. Public handles are
 * reusable across user-decrypt requests (dedup includes the per-request
 * transport key); public-decrypt requests must use unique handle
 * combinations (dedup is handles + extraData only).
 */
export type FheHandlePoolItem = Readonly<{
  index: number;
  type: FheValueType;
  /** Known plaintext, stringified; booleans as "true"/"false". */
  value: string;
  /** 32-byte `0x`-prefixed FHE handle. */
  handle: string;
  /** HD derivation index, or PRIVATE_KEY_LANE (-1), of the owner account. */
  ownerIndex: number;
  ownerAddress: string;
  isPublic: boolean;
  /** 32-byte `0x`-prefixed transaction hash. */
  transactionHash: string;
}>;
