// Strict decoding of the wire form into typed fields.
//
// Everything checkable without live state is checked here, before any text is rendered and before
// any signature is looked at: identity widths, the ACL-domain count and ordering, the
// validity-window bounds, the transport-key length, and the version and length of the KMS routing
// field. A permit that survives this step renders totally — the renderer has no failure path.
//
// Rules are applied in the order the fields are signed in, matching
// `solana/crates/zama-solana-permit/src/validate.rs` rule for rule and order for order. A permit
// breaking several rules is rejected by whichever comes first; nothing normative rides on that
// choice — every normative vector carries exactly one violation — but the order is fixed rather
// than incidental, so a diagnostic reproduces on either side.
//
// Failure throws rather than returning a result: a rejected permit has no partially usable form,
// and the eight fields are read as a unit or not at all.
//
// Two rules here have no counterpart in the crate and cannot have one, both about how a u64
// arrives. In Rust the wire fields are already `u64`, so a `number` and a value past `u64::MAX` are
// unrepresentable — they are the deserializer's problem, never the permit's. In TypeScript they are
// this module's, and they are checked ahead of the range rules the crate does have, since a value
// that is not a u64 has no range to be in.
//
// Numeric strings are read strictly: digits only, no sign, no leading zero, no surrounding
// whitespace. That is narrower than `u64::from_str` admits, which is safe in this direction — the
// SDK produces permits, so it never has to accept a spelling some other producer emitted — and it
// keeps one value from having two spellings on the way into a signed field.

import type { SolanaPermitIdentityField, SolanaPermitU64Field } from './errors.js';
import type { SolanaKmsRouting, SolanaPermitFields, SolanaPermitU64, SolanaPermitWireFields } from './types.js';
import { SolanaPermitError } from './errors.js';
import {
  PERMIT_IDENTITY_LEN,
  PERMIT_KMS_ROUTING_LEN,
  PERMIT_KMS_ROUTING_VERSION,
  PERMIT_MAX_ACL_DOMAIN_KEYS,
  PERMIT_MAX_DURATION_SECONDS,
  PERMIT_MAX_START_TIMESTAMP,
  PERMIT_MIN_DURATION_SECONDS,
  PERMIT_TRANSPORT_KEY_LEN,
} from './types.js';

/** The largest value a u64 field may carry. */
const U64_MAX = 2n ** 64n - 1n;

/** The one spelling a decimal u64 string is admitted in: no sign, no leading zero, no whitespace. */
const STRICT_DECIMAL_U64 = /^(0|[1-9][0-9]*)$/;

/**
 * Decodes the wire form, rejecting anything that violates the typed form.
 *
 * The only producer of {@link SolanaPermitFields}: the brand cannot be satisfied from outside this
 * module, so every permit the renderer and the envelope builder see has passed these rules.
 *
 * @param wire - The eight signed fields as they arrive, unvalidated.
 * @throws SolanaPermitError - With the first rule the permit breaks, in signed-field order.
 */
export function decodeSolanaPermitFields(wire: SolanaPermitWireFields): SolanaPermitFields {
  const userPubkey = decodeIdentity(wire.userPubkey, { field: 'userPubkey' });

  // The length is the whole rule: a transport key of any other length has no typed form to land in.
  if (wire.transportKey.length !== PERMIT_TRANSPORT_KEY_LEN) {
    throw new SolanaPermitError({ code: 'TransportKeyLength', length: wire.transportKey.length });
  }
  const transportKey = Uint8Array.from(wire.transportKey);

  const allowedAclDomainKeys = decodeAclDomainKeys(wire.allowedAclDomainKeys);

  const startTimestamp = readU64(wire.startTimestamp, 'startTimestamp');
  if (startTimestamp > PERMIT_MAX_START_TIMESTAMP) {
    throw new SolanaPermitError({ code: 'StartTimestampOutOfRange', startTimestamp });
  }

  const durationSeconds = readU64(wire.durationSeconds, 'durationSeconds');
  if (durationSeconds < PERMIT_MIN_DURATION_SECONDS || durationSeconds > PERMIT_MAX_DURATION_SECONDS) {
    throw new SolanaPermitError({ code: 'DurationOutOfRange', durationSeconds });
  }

  const verifyingProgramId = decodeIdentity(wire.verifyingProgramId, { field: 'verifyingProgramId' });

  const chainId = readU64(wire.chainId, 'chainId');

  const kmsRouting = decodeKmsRouting(wire.extraData);

  // The cast is the brand's constructor: this module is the only one that may perform it, and it
  // does so only on the far side of every rule above. Byte fields are copied so the branded value
  // cannot be edited afterwards through the arrays the caller still holds.
  return {
    userPubkey,
    transportKey,
    allowedAclDomainKeys,
    startTimestamp,
    durationSeconds,
    verifyingProgramId,
    chainId,
    kmsRouting,
  } as SolanaPermitFields;
}

/**
 * Serializes parsed routing back to the signed `extraData` bytes.
 *
 * The inverse of the routing parse inside {@link decodeSolanaPermitFields}, and the mirror of the
 * crate's `KmsRouting::to_extra_data`. Callers assembling a permit from an on-chain context and
 * epoch go through here rather than concatenating the version byte themselves, so the one encoding
 * of a given routing has one producer.
 *
 * Unlike the fields it accompanies, {@link SolanaKmsRouting} carries no brand — its ids are plain
 * byte arrays — so the widths are checked here. A wrong width is reported as a routing-length
 * failure, not as an identity-width one: what a caller would have produced is a routing field of a
 * length its version does not admit, which is exactly what the decoder would then reject.
 *
 * @param routing - The routing version and its ids.
 * @throws SolanaPermitError - If the ids do not add up to the length the version admits.
 */
export function encodeSolanaKmsRouting(routing: SolanaKmsRouting): Uint8Array {
  const length = 1 + routing.kmsContextId.length + routing.kmsEpochId.length;
  if (length !== PERMIT_KMS_ROUTING_LEN) {
    throw new SolanaPermitError({ code: 'KmsRoutingLength', version: routing.version, length });
  }
  const extraData = new Uint8Array(PERMIT_KMS_ROUTING_LEN);
  extraData[0] = routing.version;
  extraData.set(routing.kmsContextId, 1);
  extraData.set(routing.kmsEpochId, 1 + PERMIT_IDENTITY_LEN);
  return extraData;
}

/**
 * Decodes one identity field, naming it if the width is wrong. Returns a copy.
 *
 * @param bytes - The claimed 32-byte identity.
 * @param field - Which identity field it is, for the rejection.
 */
function decodeIdentity(bytes: Uint8Array, field: SolanaPermitIdentityField): Uint8Array {
  if (bytes.length !== PERMIT_IDENTITY_LEN) {
    throw new SolanaPermitError({ code: 'IdentityWidth', field, length: bytes.length });
  }
  return Uint8Array.from(bytes);
}

/**
 * Decodes the ACL-domain list: widths first, by index, so a malformed entry is named by its
 * position; then the count; then the strict byte-order ascent between neighbours. Equality is
 * reported as a duplicate rather than as a failed ascent, because that is the mistake a caller
 * actually made — and strict ascent between neighbours is the whole ordering rule, since a list in
 * which every key exceeds its predecessor cannot repeat a key anywhere.
 *
 * @param keys - The claimed domain keys, in the order they would be signed.
 */
function decodeAclDomainKeys(keys: readonly Uint8Array[]): readonly Uint8Array[] {
  const decoded = keys.map((key, index) => decodeIdentity(key, { field: 'aclDomainKey', index }));
  if (decoded.length > PERMIT_MAX_ACL_DOMAIN_KEYS) {
    throw new SolanaPermitError({ code: 'TooManyAclDomainKeys', count: decoded.length });
  }
  let previous: Uint8Array | undefined;
  for (const [index, key] of decoded.entries()) {
    if (previous !== undefined) {
      const order = compareBytes(previous, key);
      if (order === 0) {
        throw new SolanaPermitError({ code: 'DuplicateAclDomainKey', index });
      }
      if (order > 0) {
        throw new SolanaPermitError({ code: 'AclDomainKeysNotAscending', index });
      }
    }
    previous = key;
  }
  return decoded;
}

/**
 * Byte-order comparison: negative when `a` sorts below `b`, zero on equality. Both inputs are
 * 32-byte identities by the time this runs, so length never decides.
 *
 * @param a - The earlier key in list order.
 * @param b - The later key in list order.
 */
function compareBytes(a: Uint8Array, b: Uint8Array): number {
  for (let index = 0; index < a.length && index < b.length; index += 1) {
    const difference = (a[index] ?? 0) - (b[index] ?? 0);
    if (difference !== 0) {
      return difference;
    }
  }
  return a.length - b.length;
}

/**
 * Parses the signed KMS routing field.
 *
 * The version byte decides the length, and the length is exact: a field long enough to *contain*
 * the routing material with room to spare would be a second encoding of the same routing, and two
 * encodings of one meaning is what makes implementations diverge. An unknown version is rejected
 * here rather than carried forward, which is what keeps rendering total — the renderer only ever
 * sees versions it can render.
 *
 * @param bytes - The signed `extraData` field verbatim.
 */
function decodeKmsRouting(bytes: Uint8Array): SolanaKmsRouting {
  const version = bytes.at(0);
  if (version === undefined) {
    throw new SolanaPermitError({ code: 'UnknownKmsRoutingVersion', version: undefined });
  }
  if (version !== PERMIT_KMS_ROUTING_VERSION) {
    throw new SolanaPermitError({ code: 'UnknownKmsRoutingVersion', version });
  }
  if (bytes.length !== PERMIT_KMS_ROUTING_LEN) {
    throw new SolanaPermitError({ code: 'KmsRoutingLength', version, length: bytes.length });
  }
  return {
    version: PERMIT_KMS_ROUTING_VERSION,
    kmsContextId: bytes.slice(1, 1 + PERMIT_IDENTITY_LEN),
    kmsEpochId: bytes.slice(1 + PERMIT_IDENTITY_LEN),
  };
}

/**
 * Reads a u64 field, in whichever of the two admitted forms it arrived.
 *
 * The `number` check comes first and is a distinct rejection: a `number` is not merely out of
 * range, it may already have silently lost the value it was meant to carry, and the caller needs
 * to hear "change the type you pass", not "change the value".
 *
 * @param value - The field as it arrived.
 * @param field - Which u64 field it is, for the rejection.
 */
function readU64(value: SolanaPermitU64, field: SolanaPermitU64Field): bigint {
  if (typeof value === 'number') {
    throw new SolanaPermitError({ code: 'LossyNumericInput', field });
  }
  if (typeof value === 'bigint') {
    if (value < 0n || value > U64_MAX) {
      throw new SolanaPermitError({ code: 'NumericFieldNotU64', field, value: value.toString() });
    }
    return value;
  }
  if (typeof value === 'string' && STRICT_DECIMAL_U64.test(value)) {
    const parsed = BigInt(value);
    if (parsed <= U64_MAX) {
      return parsed;
    }
  }
  throw new SolanaPermitError({ code: 'NumericFieldNotU64', field, value });
}
