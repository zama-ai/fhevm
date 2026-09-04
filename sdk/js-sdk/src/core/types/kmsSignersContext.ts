import type { Kms } from './kms.js';
import type { ChecksummedAddress, Uint256BigInt, Uint8Number } from './primitives.js';

/**
 * A snapshot of the KMS signers configuration for a given context, as defined
 * in `KMSVerifier.sol`.
 *
 * ### Properties
 *
 * - `address` — The checksummed address of the `KMSVerifier` contract.
 * - `id` — A unique, immutable context identifier (`uint256`). IDs are
 *   sequential (each new context = previous ID + 1). The valid range is
 *   `[MIN_ID, currentKmsContextId]`. Two different contexts can never
 *   share the same ID.
 * - `epoch_id` — The epoch identifier linked to this context. Created together
 *   with `id` by `defineNewKmsContextAndEpoch()`. Becomes **active** only after
 *   Phase 2 of the context-switch protocol (key resharing MPC) completes and
 *   all KMS signers call `confirmEpochActivation`. Until then,
 *   `getCurrentKmsContextAndEpoch()` still returns the previous
 *   `(contextId, epochId)` pair.
 * - `signers` — The ordered list of KMS node addresses responsible for
 *   decrypting encrypted values under this context. No duplicates or null
 *   addresses are allowed (enforced on-chain).
 * - `threshold` — The minimum number of KMS node decryption shares required to
 *   produce a valid decrypted result. Must be non-zero and cannot exceed
 *   the number of signers.
 *
 * ### Context lifecycle (on-chain)
 *
 * A context ID can be in one of four states:
 *
 * - **current** — `id == currentKmsContextId`. `getSignersForContext` → signers.
 * - **valid** — In range, not destroyed, not current. `getSignersForContext` → signers.
 * - **destroyed** — In range, `destroyedContexts[id] == true`. `getSignersForContext` → `[]`.
 * - **out of range** — `id < MIN_ID` or `id > currentKmsContextId`. `getSignersForContext` → `[]`.
 *
 * `MIN_ID = KMS_CONTEXT_COUNTER_BASE + 1` where `KMS_CONTEXT_COUNTER_BASE = uint256(0x07) << 248`.
 *
 * Only one context is **current** at any time. The current context **cannot**
 * be destroyed. When a new context is created (context rotation), the previous
 * one becomes **valid** (still usable for signature verification).
 */
export type KmsSignersContext = {
  /** The checksummed address of the `KMSVerifier` contract. */
  readonly kmsVerifierAddress: ChecksummedAddress;
  /** The unique, immutable context identifier. See {@link KmsSignersContext} for valid range. */
  readonly id: Uint256BigInt;
  /** Epoch identifier linked to this context. Active only after Phase 2 key resharing completes. See {@link KmsSignersContext}. */
  readonly epochId: Uint256BigInt;
  /** Ordered list of KMS node addresses for this context. No duplicates or null addresses. */
  readonly signers: readonly ChecksummedAddress[];
  /**
   * Minimum number of KMS decryption shares required for a public decryption.
   * Non-zero, at most `signers.length`.
   */
  readonly threshold: Uint8Number;
  /**
   * Minimum number of KMS decryption shares required for a user decryption.
   * Differs from the `threshold`
   * (available in protocol v13+)
   */
  readonly mpcThreshold?: Uint8Number | undefined;
  /** Returns `true` if the given address is one of the signers for this context. */
  has(signer: string): boolean;
} & Kms;
