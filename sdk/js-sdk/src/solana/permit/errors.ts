// Rejection reasons for the Solana user-decrypt permit.
//
// One member per distinct rule violation, mirroring `zama-solana-permit`'s `PermitError`, so a
// normative vector can assert that a bad permit failed for the reason it was built to fail for
// rather than merely failing somehow. The union is the contract the vector runner asserts against;
// the message text is for humans and nothing depends on its wording.

import { PERMIT_IDENTITY_LEN } from './types.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Which identity field a width violation was found in.
 *
 * The KMS context and epoch ids are absent on purpose: they live inside the routing field, whose
 * length is checked as a whole, so their widths cannot be wrong independently.
 */
export type SolanaPermitIdentityField =
  | { readonly field: 'userPubkey' }
  | { readonly field: 'verifyingProgramId' }
  | { readonly field: 'aclDomainKey'; readonly index: number };

/** A permit field the canon signs as a u64. */
export type SolanaPermitU64Field = 'startTimestamp' | 'durationSeconds' | 'chainId';

/**
 * Why a permit was rejected.
 *
 * Eleven members mirror the Rust enum one-to-one. The last two have no counterpart there and cannot
 * have one: they are the JavaScript-only failures of a u64 field arriving as a `number`, or
 * arriving as a bigint or string that is not a u64 at all. Rust's wire fields are already `u64`, so
 * both are unrepresentable on that side.
 */
export type SolanaPermitRejection =
  | { readonly code: 'IdentityWidth'; readonly field: SolanaPermitIdentityField; readonly length: number }
  | { readonly code: 'TooManyAclDomainKeys'; readonly count: number }
  | { readonly code: 'AclDomainKeysNotAscending'; readonly index: number }
  | { readonly code: 'DuplicateAclDomainKey'; readonly index: number }
  | { readonly code: 'DurationOutOfRange'; readonly durationSeconds: bigint }
  | { readonly code: 'StartTimestampOutOfRange'; readonly startTimestamp: bigint }
  | { readonly code: 'TransportKeyLength'; readonly length: number }
  | { readonly code: 'UnknownKmsRoutingVersion'; readonly version: number | undefined }
  | { readonly code: 'KmsRoutingLength'; readonly version: number; readonly length: number }
  | { readonly code: 'SignatureMismatch' }
  | { readonly code: 'UnusableUserPubkey' }
  | { readonly code: 'LossyNumericInput'; readonly field: SolanaPermitU64Field }
  | { readonly code: 'NumericFieldNotU64'; readonly field: SolanaPermitU64Field; readonly value: string };

/** The `code` discriminant of {@link SolanaPermitRejection}. */
export type SolanaPermitRejectionCode = SolanaPermitRejection['code'];

////////////////////////////////////////////////////////////////////////////////

/**
 * A permit rejected by the typed-form rules, the routing parse, or the signature check.
 *
 * The reason travels as the structured {@link SolanaPermitRejection}, not as parseable text: a
 * caller (and a vector runner) reads `rejection.code`.
 */
export class SolanaPermitError extends Error {
  readonly rejection: SolanaPermitRejection;

  constructor(rejection: SolanaPermitRejection) {
    super(describeRejection(rejection));
    this.name = 'SolanaPermitError';
    this.rejection = rejection;
  }
}

/**
 * Renders a rejection as a human-readable sentence.
 *
 * Exhaustive by construction — there is no fallback arm, so a new rejection member has to be given
 * its own wording here instead of inheriting a generic one.
 *
 * @param rejection - The structured reason.
 */
function describeRejection(rejection: SolanaPermitRejection): string {
  switch (rejection.code) {
    case 'IdentityWidth':
      return `identity ${describeIdentityField(rejection.field)} is ${rejection.length} bytes, expected ${PERMIT_IDENTITY_LEN}`;
    case 'TooManyAclDomainKeys':
      return `${rejection.count} ACL domain keys exceeds the permitted maximum`;
    case 'AclDomainKeysNotAscending':
      return `ACL domain key at index ${rejection.index} is not above its predecessor in byte order`;
    case 'DuplicateAclDomainKey':
      return `ACL domain key at index ${rejection.index} repeats an earlier key`;
    case 'DurationOutOfRange':
      return `a validity window of ${rejection.durationSeconds} seconds is outside the permitted range`;
    case 'StartTimestampOutOfRange':
      return `a start timestamp of ${rejection.startTimestamp} is past the representable range`;
    case 'TransportKeyLength':
      return `the transport key is ${rejection.length} bytes, which is not the accepted length`;
    case 'UnknownKmsRoutingVersion':
      return rejection.version === undefined
        ? 'the KMS routing field is empty and carries no version byte'
        : `KMS routing version 0x${rejection.version.toString(16).padStart(2, '0')} is not known`;
    case 'KmsRoutingLength':
      return `KMS routing version 0x${rejection.version.toString(16).padStart(2, '0')} is ${rejection.length} bytes, which its version does not admit`;
    case 'SignatureMismatch':
      return 'the signature does not verify over the locally reconstructed envelope';
    case 'UnusableUserPubkey':
      return 'the user pubkey is not a usable Ed25519 verifying key';
    case 'LossyNumericInput':
      return `${rejection.field} was supplied as a number; pass a bigint or a decimal string, which cannot lose precision`;
    case 'NumericFieldNotU64':
      return `${rejection.field} is "${rejection.value}", which is not an unsigned 64-bit integer in plain decimal`;
  }
}

/**
 * Names an identity field for a message.
 *
 * @param field - The field a width violation was found in.
 */
function describeIdentityField(field: SolanaPermitIdentityField): string {
  switch (field.field) {
    case 'userPubkey':
      return 'userPubkey';
    case 'verifyingProgramId':
      return 'verifyingProgramId';
    case 'aclDomainKey':
      return `allowedAclDomainKeys[${field.index}]`;
  }
}
