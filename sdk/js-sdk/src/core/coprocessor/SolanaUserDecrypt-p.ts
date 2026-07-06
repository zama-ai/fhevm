import type { Bytes20Hex, BytesHex } from '../types/primitives.js';

import { ed25519 } from '@noble/curves/ed25519.js';
import { keccak_256 } from '@noble/hashes/sha3.js';

import { bytesToHex, concatBytes } from '../base/bytes.js';

/**
 * Solana user-decryption signer and request builder.
 *
 * The signing preimage here is the TypeScript mirror of the Rust source of truth
 * `kms-connector/crates/utils/src/types/solana_extra_data.rs`. Every KMS party's connector
 * re-derives this preimage and verifies the ed25519 signature over it, so the bytes produced
 * by {@link solanaUserDecryptSigningPreimage} MUST match the Rust byte-for-byte. The cross-impl
 * parity is locked by `SolanaUserDecrypt-p.test.ts`. The ed25519 auth fields travel as typed
 * gateway fields (RFC-021), so `extraData` carries only the KMS context or the MMR-proof tail.
 *
 * On EVM the Gateway verifies the EIP-712 publicKey-to-userAddress binding on-chain. The Solana
 * host has no such on-chain binding, so the signature here is what closes the publicKey
 * substitution attack: it commits to the re-encryption `publicKey`, the handles, the identity,
 * the nonce, the allowed ACL domain keys, and the validity window.
 */

/** Context-only `extraData` version byte (RFC-003 v0x01): version ‖ contextId(32). */
export const SOLANA_CONTEXT_EXTRA_DATA_VERSION = 0x01;

/** `extraData` version byte carrying the context id PLUS the MMR-proof tail (RFC-024): version ‖
 * contextId(32) ‖ aclValueKey(32) ‖ proofSlot(8 BE) ‖ mmrProofLen(4 BE) ‖ mmrProofBytes. Mirrors
 * `SOLANA_EXTRA_DATA_VERSION_MMR_PROOF` in `solana_extra_data.rs`. */
export const SOLANA_MMR_PROOF_EXTRA_DATA_VERSION = 0x03;

/**
 * Domain-separation tag for the signing preimage (`SOLANA_USER_DECRYPT_DOMAIN_TAG` in Rust).
 * Bumped to `v2` when the MMR-proof tail (`aclValueKey`, `proofSlot`, `mmrProofBytes`) was
 * appended to the preimage (RFC-024) — ed25519 is non-malleable, so a `v1` signature can never
 * verify against the `v2` preimage.
 */
export const SOLANA_USER_DECRYPT_DOMAIN_TAG = 'zama-solana-user-decrypt-v2';

/** V2 user-decrypt attestation discriminator the relayer/gateway route on. */
export const SOLANA_USER_DECRYPT_ATTESTATION_TYPE = 'solana-ed25519-user-decrypt-v1';

const SOLANA_PUBKEY_LEN = 32;
const HANDLE_LEN = 32;
const ED25519_SIGNATURE_LEN = 64;

/** Encodes a u32 as 4 big-endian bytes (`u32::to_be_bytes`). */
function u32BE(value: number): Uint8Array {
  const out = new Uint8Array(4);
  new DataView(out.buffer).setUint32(0, value, false);
  return out;
}

/** Encodes a u64 as 8 big-endian bytes (`u64::to_be_bytes`). */
function u64BE(value: bigint): Uint8Array {
  const out = new Uint8Array(8);
  new DataView(out.buffer).setBigUint64(0, value, false);
  return out;
}

function assertLen(name: string, bytes: Uint8Array, len: number): void {
  if (bytes.length !== len) {
    throw new Error(`${name} must be ${len} bytes, got ${bytes.length}`);
  }
}

/** Fields shared by the `extraData` blob and the signing preimage. */
export interface SolanaUserDecryptInput {
  /** The host chain id the handles belong to (`contracts_chain_id`). */
  readonly contractsChainId: bigint;
  /** The ML-KEM re-encryption public key the plaintext will be sealed to. */
  readonly publicKey: Uint8Array;
  /** The requested ciphertext handles, each 32 bytes. */
  readonly handles: readonly Uint8Array[];
  /** The user's 32-byte ed25519 identity public key. */
  readonly identity: Uint8Array;
  /** The 32-byte big-endian context id (all-zero when no explicit context). */
  readonly contextId: Uint8Array;
  /** Per-request 32-byte nonce bound into the signed preimage (not dedup-enforced; replay is bounded by the validity window, matching EVM). */
  readonly nonce: Uint8Array;
  /** The authorized ACL domain keys (the signed `allowedContracts` scope), each 32 bytes. */
  readonly allowedAclDomainKeys: readonly Uint8Array[];
  /** Validity window start (unix seconds). */
  readonly startTimestamp: bigint;
  /** Validity window duration (seconds). */
  readonly durationSeconds: bigint;
  /**
   * The lineage value key for a historical/public MMR-proof decrypt; all-zero (the default) for
   * a current-ACL request. Mirrors `acl_value_key` in `solana_extra_data.rs`.
   */
  readonly aclValueKey?: Uint8Array | undefined;
  /**
   * The full MMR-proof transport blob (1-byte mode prefix ‖ Borsh proof), committed verbatim;
   * empty (the default) for a current-ACL request. Mirrors `mmr_proof_bytes`.
   */
  readonly mmrProofBytes?: Uint8Array | undefined;
  /**
   * The lineage `leaf_count` the proof was built against (staleness marker); `0n` (the default)
   * for a current-ACL request. Mirrors `proof_slot`.
   */
  readonly proofSlot?: bigint | undefined;
}

const ZERO_ACL_VALUE_KEY = new Uint8Array(32);

/** Fills in the MMR-proof tail defaults (all-zero/empty, i.e. "no proof") for fields left unset. */
function withProofDefaults(
  input: SolanaUserDecryptInput,
): { aclValueKey: Uint8Array; mmrProofBytes: Uint8Array; proofSlot: bigint } {
  return {
    aclValueKey: input.aclValueKey ?? ZERO_ACL_VALUE_KEY,
    mmrProofBytes: input.mmrProofBytes ?? new Uint8Array(0),
    proofSlot: input.proofSlot ?? 0n,
  };
}

function assertLengths(name: string, items: readonly Uint8Array[], len: number): void {
  for (let i = 0; i < items.length; i++) {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    assertLen(`${name}[${i}]`, items[i]!, len);
  }
}

function assertCommonInput(input: SolanaUserDecryptInput): void {
  assertLen('identity', input.identity, SOLANA_PUBKEY_LEN);
  assertLen('contextId', input.contextId, 32);
  assertLen('nonce', input.nonce, SOLANA_PUBKEY_LEN);
  assertLengths('handles', input.handles, HANDLE_LEN);
  assertLengths('allowedAclDomainKeys', input.allowedAclDomainKeys, SOLANA_PUBKEY_LEN);
  if (input.aclValueKey !== undefined) {
    assertLen('aclValueKey', input.aclValueKey, 32);
  }
}

/**
 * Builds the context-only `extraData` placed on the wire (RFC-003 v0x01): `0x01 ‖ contextId(32 BE)`.
 * This is what the Solana user-decrypt request now carries — the ed25519 auth fields (identity,
 * nonce, allowed ACL domain keys) travel as typed gateway fields, not packed into `extraData`.
 */
export function buildSolanaUserDecryptContextExtraData(contextId: Uint8Array): Uint8Array {
  assertLen('contextId', contextId, 32);
  return concatBytes(new Uint8Array([SOLANA_CONTEXT_EXTRA_DATA_VERSION]), contextId);
}

/**
 * Builds the MMR-proof-tail `extraData` placed on the wire (RFC-024 v0x03):
 * `0x03 ‖ contextId(32) ‖ aclValueKey(32) ‖ proofSlot(8 BE) ‖ mmrProofLen(4 BE) ‖ mmrProofBytes`.
 * Mirrors `encode_solana_extra_data_mmr_proof` in `solana_extra_data.rs`.
 */
export function buildSolanaUserDecryptMmrProofExtraData(
  contextId: Uint8Array,
  aclValueKey: Uint8Array,
  proofSlot: bigint,
  mmrProofBytes: Uint8Array,
): Uint8Array {
  assertLen('contextId', contextId, 32);
  assertLen('aclValueKey', aclValueKey, 32);
  return concatBytes(
    new Uint8Array([SOLANA_MMR_PROOF_EXTRA_DATA_VERSION]),
    contextId,
    aclValueKey,
    u64BE(proofSlot),
    u32BE(mmrProofBytes.length),
    mmrProofBytes,
  );
}

/**
 * Builds the exact bytes the user's ed25519 key must sign, byte-identical to
 * `solana_user_decrypt_signing_preimage` in Rust:
 *
 * `TAG ‖ contracts_chain_id(8 BE) ‖ publicKey_len(4 BE) ‖ publicKey ‖ handle_count(4 BE) ‖
 * handles(32 each) ‖ identity(32) ‖ context_id(32 BE) ‖ nonce(32) ‖ domain_key_count(4 BE) ‖
 * domain_keys(32 each) ‖ start_timestamp(8 BE) ‖ duration_seconds(8 BE) ‖ acl_value_key(32) ‖
 * proof_slot(8 BE) ‖ mmr_proof_len(4 BE) ‖ mmr_proof_bytes`
 *
 * The MMR-proof tail is always appended (all-zero/empty when the request carries no proof),
 * matching the Rust builder, which never makes the tail conditional.
 */
export function solanaUserDecryptSigningPreimage(input: SolanaUserDecryptInput): Uint8Array {
  assertCommonInput(input);
  const { aclValueKey, mmrProofBytes, proofSlot } = withProofDefaults(input);

  return concatBytes(
    new TextEncoder().encode(SOLANA_USER_DECRYPT_DOMAIN_TAG),
    u64BE(input.contractsChainId),
    u32BE(input.publicKey.length),
    input.publicKey,
    u32BE(input.handles.length),
    ...input.handles,
    input.identity,
    input.contextId,
    input.nonce,
    u32BE(input.allowedAclDomainKeys.length),
    ...input.allowedAclDomainKeys,
    u64BE(input.startTimestamp),
    u64BE(input.durationSeconds),
    aclValueKey,
    u64BE(proofSlot),
    u32BE(mmrProofBytes.length),
    mmrProofBytes,
  );
}

/**
 * Derives the user address the connector keys ACL checks on from the ed25519 identity:
 * the last 20 bytes of `keccak256(identity)`, lowercase 0x-prefixed. Mirrors kms-core's
 * `solana_user_decrypt_client_id`.
 */
export function solanaUserDecryptClientId(identity: Uint8Array): Bytes20Hex {
  assertLen('identity', identity, SOLANA_PUBKEY_LEN);
  const hash = keccak_256(identity);
  return bytesToHex(hash.subarray(12)) as Bytes20Hex;
}

/** A user-decrypt request ready to POST to the relayer's Solana ed25519 endpoint (RFC-021). */
export interface SolanaUserDecryptRequest {
  readonly attestationType: typeof SOLANA_USER_DECRYPT_ATTESTATION_TYPE;
  /** The 64-byte ed25519 signature over the signing preimage, 0x-hex. */
  readonly signature: BytesHex;
  /** `extraData`, 0x-hex: v0x01 context-only when no proof is present, or v0x03 with the MMR-proof tail. */
  readonly extraData: BytesHex;
  /** The ML-KEM re-encryption public key, 0x-hex. */
  readonly publicKey: BytesHex;
  /** The requested handles, each 0x-hex 32 bytes. */
  readonly handles: readonly BytesHex[];
  /** The derived client id (keccak(identity)[12..]), lowercase 0x-hex 20 bytes. */
  readonly userAddress: Bytes20Hex;
  /** The user's 32-byte ed25519 identity public key, 0x-hex (typed gateway field). */
  readonly solanaUserIdentity: BytesHex;
  /** The per-request 32-byte nonce, 0x-hex (typed gateway field). Bound into the signed preimage; not dedup-enforced (replay bounded by the validity window, matching EVM). */
  readonly solanaNonce: BytesHex;
  /** The allowed Solana ACL domain keys, each 0x-hex 32 bytes (typed gateway field). */
  readonly solanaAllowedAclDomainKeys: readonly BytesHex[];
}

/**
 * ed25519-signs the canonical preimage with the user's Solana secret key and assembles a
 * well-formed V2 Solana user-decrypt request. `secretKey` is the 32-byte ed25519 seed; the
 * derived public key must equal `input.identity` or signing throws (a mismatch would produce a
 * signature the connector rejects).
 */
export function buildSolanaUserDecryptRequest(
  input: SolanaUserDecryptInput,
  secretKey: Uint8Array,
): SolanaUserDecryptRequest {
  assertCommonInput(input);
  if (secretKey.length !== 32) {
    throw new Error(`secretKey must be a 32-byte ed25519 seed, got ${secretKey.length}`);
  }
  const derived = ed25519.getPublicKey(secretKey);
  if (bytesToHex(derived) !== bytesToHex(input.identity)) {
    throw new Error('secretKey does not derive the provided identity public key');
  }

  const preimage = solanaUserDecryptSigningPreimage(input);
  const signature = ed25519.sign(preimage, secretKey);
  if (signature.length !== ED25519_SIGNATURE_LEN) {
    throw new Error(`unexpected ed25519 signature length: ${signature.length}`);
  }

  // The signed preimage always commits to the MMR-proof tail (all-zero/empty when absent); only
  // the `extraData` wire shape is conditional: a proof-bearing request needs the v0x03 tail on
  // the wire for the connector to decode it, while a plain current-ACL request keeps the smaller
  // v0x01 context-only blob. Presence of `mmrProofBytes` is the signal for which to emit.
  const { aclValueKey, mmrProofBytes, proofSlot } = withProofDefaults(input);
  const extraData =
    mmrProofBytes.length > 0
      ? buildSolanaUserDecryptMmrProofExtraData(input.contextId, aclValueKey, proofSlot, mmrProofBytes)
      : buildSolanaUserDecryptContextExtraData(input.contextId);

  return {
    attestationType: SOLANA_USER_DECRYPT_ATTESTATION_TYPE,
    signature: bytesToHex(signature),
    extraData: bytesToHex(extraData),
    publicKey: bytesToHex(input.publicKey),
    handles: input.handles.map((h) => bytesToHex(h)),
    userAddress: solanaUserDecryptClientId(input.identity),
    solanaUserIdentity: bytesToHex(input.identity),
    solanaNonce: bytesToHex(input.nonce),
    solanaAllowedAclDomainKeys: input.allowedAclDomainKeys.map((k) => bytesToHex(k)),
  };
}
