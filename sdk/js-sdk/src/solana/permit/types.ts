/* eslint-disable @typescript-eslint/naming-convention */

// Typed form of the Solana user-decrypt permit.
//
// The TypeScript half of a two-language canon: `solana/crates/zama-solana-permit` renders and
// verifies the same permit, and the two MUST agree byte for byte. Every constant, field order and
// bound here is that crate's, and the committed vectors
// (`solana/test-fixtures/permit/permit_v1.json`) are what proves the agreement rather than the
// resemblance of the source.
//
// The validated form carries a brand, mirroring the crate's private constructor: the only way to
// obtain `SolanaPermitFields` is `decodeSolanaPermitFields`, so no caller can hand the renderer
// fields that were never checked.

import type { Prettify } from '../../core/types/utils.js';

declare const __solanaPermitFields: unique symbol;

////////////////////////////////////////////////////////////////////////////////

/** Width of every identity the permit carries: user, program, ACL domain, KMS context and epoch. */
export const PERMIT_IDENTITY_LEN = 32;

/**
 * The single accepted transport-key length: a tfhe safe-serialized `UnifiedPublicEncKey::MlKem512`
 * container. The permit carries no variant field, so the length is what pins the variant.
 */
export const PERMIT_TRANSPORT_KEY_LEN = 869;

/** Upper bound on the signed ACL-domain list; an empty list is the permissive permit. */
export const PERMIT_MAX_ACL_DOMAIN_KEYS = 10;

/** Shortest admitted validity window. */
export const PERMIT_MIN_DURATION_SECONDS = 1n;

/** Longest admitted validity window: one year. */
export const PERMIT_MAX_DURATION_SECONDS = 31_536_000n;

/** Latest start the fixed-width timestamp rendering admits: `9999-12-31T23:59:59Z`. */
export const PERMIT_MAX_START_TIMESTAMP = 253_402_300_799n;

/** The only known KMS-routing version byte: context id followed by epoch id. */
export const PERMIT_KMS_ROUTING_VERSION = 2;

/** Length of the version-`0x02` routing field: version byte, context id, epoch id. */
export const PERMIT_KMS_ROUTING_LEN = 1 + PERMIT_IDENTITY_LEN + PERMIT_IDENTITY_LEN;

/** Length of the Ed25519 signature over the envelope. */
export const PERMIT_SIGNATURE_LEN = 64;

////////////////////////////////////////////////////////////////////////////////

/**
 * A u64 permit field on the way in: a bigint or a decimal string, never a `number`.
 *
 * A chain id or a timestamp past `Number.MAX_SAFE_INTEGER` loses precision as a `number`, and a
 * permit whose fields silently changed value renders a text the wallet's signature does not cover.
 * Excluding `number` from the type is the first line of that defence; `decodeSolanaPermitFields`
 * rejects it at runtime as well, for callers reaching this module from untyped JavaScript.
 */
export type SolanaPermitU64 = bigint | string;

/**
 * The signed KMS routing material, parsed.
 *
 * `version` is a literal rather than a `number` so that a second routing version arrives as a
 * second union member: every consumer switching on it stops compiling until it says what the new
 * version renders as, which is what keeps two versions from sharing one canonical text.
 */
export type SolanaKmsRouting = {
  readonly version: typeof PERMIT_KMS_ROUTING_VERSION;
  readonly kmsContextId: Uint8Array;
  readonly kmsEpochId: Uint8Array;
};

/**
 * The eight signed permit fields, in the form they arrive in: unvalidated and unordered.
 *
 * `extraData` is the versioned routing field verbatim — the wire form of {@link SolanaKmsRouting}.
 * It is parsed by the decoder, never by the renderer.
 */
export type SolanaPermitWireFields = Prettify<{
  readonly userPubkey: Uint8Array;
  readonly transportKey: Uint8Array;
  readonly allowedAclDomainKeys: readonly Uint8Array[];
  readonly startTimestamp: SolanaPermitU64;
  readonly durationSeconds: SolanaPermitU64;
  readonly verifyingProgramId: Uint8Array;
  readonly chainId: SolanaPermitU64;
  readonly extraData: Uint8Array;
}>;

/**
 * The eight signed permit fields, validated.
 *
 * Only {@link decodeSolanaPermitFields} produces this type — the brand cannot be satisfied from
 * outside the module — so a value of this type has already passed the typed-form rules and the
 * renderer needs no `Result` and no defensive checks of its own.
 */
export type SolanaPermitFields = {
  readonly [__solanaPermitFields]: never;
  readonly userPubkey: Uint8Array;
  readonly transportKey: Uint8Array;
  /** In signed order: strictly ascending in byte order, no duplicates; empty means permissive. */
  readonly allowedAclDomainKeys: readonly Uint8Array[];
  readonly startTimestamp: bigint;
  readonly durationSeconds: bigint;
  readonly verifyingProgramId: Uint8Array;
  readonly chainId: bigint;
  readonly kmsRouting: SolanaKmsRouting;
};

/** True for a permit whose ACL-domain list is empty — a grant over every domain. */
export function isPermissivePermit(fields: SolanaPermitFields): boolean {
  return fields.allowedAclDomainKeys.length === 0;
}
