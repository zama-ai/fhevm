// The normative permit vectors, run against this implementation.
//
// `solana/test-fixtures/permit/permit_v1.json` is consumed byte-identically by the SDK, the relayer,
// the Connector, KMS Core and the KMS client. `permit_vectors.rs` beside it is the Rust half of the
// schema; this file is the TypeScript half. Nothing here generates a vector — regeneration belongs
// to the crate that owns the canon (`bash solana/scripts/update-permit-vectors.sh`), and a consumer
// able to regenerate what it checks against would be checking nothing.
//
// Rejecting records name the coarse rule that must reject them, which is what `RULE_REJECTION` below
// exists for: it maps the shared rule names onto this implementation's rejection codes, so "every
// implementation rejects for the same reason" is a check rather than a hope. The map is total in
// both directions at compile time — a rejection code with no rule name, or a rule name this file
// forgot, is a type error rather than a silently uncovered case.

import type {
  SolanaPermitFields,
  SolanaPermitRejection,
  SolanaPermitRejectionCode,
  SolanaPermitU64,
  SolanaPermitWireFields,
} from './index.js';
import { readFileSync } from 'node:fs';
import { ed25519 } from '@noble/curves/ed25519.js';
import { describe, expect, it } from 'vitest';
import {
  PERMIT_ENVELOPE_PREAMBLE,
  PERMIT_ENVELOPE_SIGNER_COUNT,
  PERMIT_ENVELOPE_VERSION,
  PERMIT_SIGNATURE_LEN,
  SolanaPermitError,
  buildSolanaPermitEnvelope,
  decodeSolanaPermitFields,
  isPermissivePermit,
  renderSolanaPermitText,
  verifySolanaPermitSignature,
} from './index.js';
import { bytesToHex, hexToBytes } from '../proof.js';

////////////////////////////////////////////////////////////////////////////////
// The committed file, and the schema it is read under
////////////////////////////////////////////////////////////////////////////////

/* eslint-disable @typescript-eslint/naming-convention -- the fixture's own field names are snake_case */

/** One record's eight fields, in transport form. Several records declare wrong widths on purpose. */
interface WirePermitRecord {
  readonly user_pubkey: string;
  readonly transport_key: string;
  readonly allowed_acl_domain_keys: readonly string[];
  readonly start_timestamp: string;
  readonly duration_seconds: string;
  readonly verifying_program_id: string;
  readonly chain_id: string;
  readonly extra_data: string;
}

/** One record. The optional fields are the ones only a rejecting or a well-formed record carries. */
interface PermitVectorRecord {
  readonly name: string;
  readonly comment: string;
  readonly result: 'valid' | 'invalid' | 'acceptable';
  readonly rule?: string;
  readonly derived_from?: string;
  readonly mutation?: string;
  readonly permit: WirePermitRecord;
  readonly kms_routing?: {
    readonly version: string;
    readonly kms_context_id: string;
    readonly kms_epoch_id: string;
  };
  readonly signature: string;
  readonly permit_text?: string;
  readonly permit_text_bytes?: string;
  readonly envelope_bytes?: string;
  readonly signed_text?: string;
}

/** The vector file. */
interface PermitVectorFile {
  readonly schema: string;
  readonly description: string;
  readonly regenerate_with: string;
  readonly deployment: { readonly chain_id_decimal: string };
  readonly transport_keys: Readonly<Record<string, string>>;
  readonly vectors: readonly PermitVectorRecord[];
}

/* eslint-enable @typescript-eslint/naming-convention */

const VECTOR_FILE = new URL('../../../../../solana/test-fixtures/permit/permit_v1.json', import.meta.url);
const PERMIT_VECTOR_SCHEMA = 'zama-solana-permit-vectors/v1';

const file = JSON.parse(readFileSync(VECTOR_FILE, 'utf8')) as PermitVectorFile;

const unhex = (hex: string): Uint8Array => hexToBytes(`0x${hex}`);
const rehex = (bytes: Uint8Array): string => bytesToHex(bytes).slice(2);

////////////////////////////////////////////////////////////////////////////////
// The rule contract
////////////////////////////////////////////////////////////////////////////////

/** Every rule name the shared schema defines, mirroring `permit_vectors.rs`'s `rule::ALL`. */
const PERMIT_VECTOR_RULES = [
  'identity-width',
  'too-many-acl-domain-keys',
  'acl-domain-keys-not-ascending',
  'duplicate-acl-domain-key',
  'duration-out-of-range',
  'start-timestamp-out-of-range',
  'transport-key-length',
  'unknown-kms-routing-version',
  'kms-routing-length',
  'signature-mismatch',
  'unusable-user-pubkey',
] as const;

type PermitVectorRule = (typeof PERMIT_VECTOR_RULES)[number];

/**
 * The rejection codes that mirror the Rust error enum, and so are the ones a shared rule can name.
 * The two excluded members are this language's own: no vector can express a `number` argument or a
 * value outside u64, because in Rust the wire fields are already `u64`.
 */
type MirroredRejectionCode = Exclude<SolanaPermitRejectionCode, 'LossyNumericInput' | 'NumericFieldNotU64'>;

const RULE_REJECTION = {
  'identity-width': 'IdentityWidth',
  'too-many-acl-domain-keys': 'TooManyAclDomainKeys',
  'acl-domain-keys-not-ascending': 'AclDomainKeysNotAscending',
  'duplicate-acl-domain-key': 'DuplicateAclDomainKey',
  'duration-out-of-range': 'DurationOutOfRange',
  'start-timestamp-out-of-range': 'StartTimestampOutOfRange',
  'transport-key-length': 'TransportKeyLength',
  'unknown-kms-routing-version': 'UnknownKmsRoutingVersion',
  'kms-routing-length': 'KmsRoutingLength',
  'signature-mismatch': 'SignatureMismatch',
  'unusable-user-pubkey': 'UnusableUserPubkey',
} as const satisfies Record<PermitVectorRule, MirroredRejectionCode>;

/**
 * Completeness in the other direction: a mirrored rejection code that no rule names collapses this
 * alias to `never`, and the assignment that uses it below stops compiling. Without it the map could
 * go stale on the side no assertion looks at, and coverage would narrow quietly.
 */
type EveryMirroredCodeIsNamedByARule =
  Exclude<MirroredRejectionCode, (typeof RULE_REJECTION)[PermitVectorRule]> extends never ? true : never;

/** The rules checked while decoding; the remaining two are decided by the signature check. */
const DECODE_TIME_RULES: readonly PermitVectorRule[] = PERMIT_VECTOR_RULES.filter(
  (rule) => rule !== 'signature-mismatch' && rule !== 'unusable-user-pubkey',
);

////////////////////////////////////////////////////////////////////////////////
// Driving one record
////////////////////////////////////////////////////////////////////////////////

/** Rebuilds a record's wire form. u64 fields go in as the decimal strings the file carries. */
function wireOf(record: PermitVectorRecord): SolanaPermitWireFields {
  const transportKey = file.transport_keys[record.permit.transport_key];
  if (transportKey === undefined) {
    throw new Error(`${record.name}: transport key ${record.permit.transport_key} is not in the file's table`);
  }
  return {
    userPubkey: unhex(record.permit.user_pubkey),
    transportKey: unhex(transportKey),
    allowedAclDomainKeys: record.permit.allowed_acl_domain_keys.map(unhex),
    startTimestamp: record.permit.start_timestamp,
    durationSeconds: record.permit.duration_seconds,
    verifyingProgramId: unhex(record.permit.verifying_program_id),
    chainId: record.permit.chain_id,
    extraData: unhex(record.permit.extra_data),
  };
}

/** The rejection a call produced, or a failure if it produced anything else. */
function rejectionFrom(call: () => unknown): SolanaPermitRejection {
  try {
    call();
  } catch (error) {
    if (error instanceof SolanaPermitError) {
      return error.rejection;
    }
    throw error;
  }
  throw new Error('expected a rejection, the call returned');
}

/**
 * Runs a rejecting record the way every implementation must: decode first, and only if it decodes,
 * check the signature. Which of the two rejects is itself asserted below — a permit rejected at the
 * wrong stage would otherwise pass on the code alone.
 */
function rejectionOfRecord(record: PermitVectorRecord): {
  rejection: SolanaPermitRejection;
  stage: 'decode' | 'verify';
} {
  let fields: SolanaPermitFields;
  try {
    fields = decodeSolanaPermitFields(wireOf(record));
  } catch (error) {
    if (error instanceof SolanaPermitError) {
      return { rejection: error.rejection, stage: 'decode' };
    }
    throw error;
  }
  return {
    rejection: rejectionFrom(() => verifySolanaPermitSignature(fields, unhex(record.signature))),
    stage: 'verify',
  };
}

const accepted = file.vectors.filter((record) => record.result !== 'invalid');
const rejected = file.vectors.filter((record) => record.result === 'invalid');
const named = (records: readonly PermitVectorRecord[]): ReadonlyArray<readonly [string, PermitVectorRecord]> =>
  records.map((record) => [record.name, record] as const);

/** Looks a record up by name, so a test that needs a particular one says which. */
function recordNamed(name: string): PermitVectorRecord {
  const record = file.vectors.find((candidate) => candidate.name === name);
  if (record === undefined) {
    throw new Error(`the vector file carries no record named ${name}`);
  }
  return record;
}

/** The record every rejecting record is derived from: two ACL domains, everything else nominal. */
const REFERENCE_RECORD = recordNamed('reference-permit-two-domains');

////////////////////////////////////////////////////////////////////////////////

describe('normative permit vectors (solana/test-fixtures/permit)', () => {
  it('reads the file under the schema it declares', () => {
    expect(file.schema).toBe(PERMIT_VECTOR_SCHEMA);
    expect(file.regenerate_with).not.toBe('');
    expect(accepted.length).toBeGreaterThan(0);
    expect(rejected.length).toBeGreaterThan(0);
  });

  it('names every rejection code this implementation mirrors', () => {
    // The assignment is the check: if a mirrored code has no rule name, its type is `never`.
    const everyMirroredCodeIsNamedByARule: EveryMirroredCodeIsNamedByARule = true;
    expect(everyMirroredCodeIsNamedByARule).toBe(true);
    expect(Object.keys(RULE_REJECTION)).toEqual([...PERMIT_VECTOR_RULES]);
  });

  it('exercises every shared rule, and names no rule the schema does not define', () => {
    const covered = new Set(rejected.map((record) => record.rule));
    for (const rule of PERMIT_VECTOR_RULES) {
      expect(covered, `no record exercises ${rule}`).toContain(rule);
    }
    for (const rule of covered) {
      expect(PERMIT_VECTOR_RULES, `a record names the unknown rule ${String(rule)}`).toContain(rule);
    }
  });
});

describe('accepted records', () => {
  it.each(named(accepted))('%s: decodes, renders and verifies as recorded', (_name, record) => {
    const fields = decodeSolanaPermitFields(wireOf(record));

    // The text is the normative artifact; the string copy is checked too, so a divergence in the
    // hex and a divergence in the readable form cannot hide behind one another.
    const text = renderSolanaPermitText(fields);
    expect(text).toBe(record.permit_text);
    expect(rehex(new TextEncoder().encode(text))).toBe(record.permit_text_bytes);

    expect(rehex(buildSolanaPermitEnvelope(fields))).toBe(record.envelope_bytes);

    const signature = unhex(record.signature);
    expect(signature).toHaveLength(PERMIT_SIGNATURE_LEN);
    expect(() => verifySolanaPermitSignature(fields, signature)).not.toThrow();
  });

  it.each(named(accepted))('%s: parses the routing field the record declares', (_name, record) => {
    const { kmsRouting } = decodeSolanaPermitFields(wireOf(record));
    expect(String(kmsRouting.version)).toBe(record.kms_routing?.version);
    expect(rehex(kmsRouting.kmsContextId)).toBe(record.kms_routing?.kms_context_id);
    expect(rehex(kmsRouting.kmsEpochId)).toBe(record.kms_routing?.kms_epoch_id);
  });

  it.each(named(accepted))('%s: reports its breadth from the signed domain list alone', (_name, record) => {
    const fields = decodeSolanaPermitFields(wireOf(record));
    expect(isPermissivePermit(fields)).toBe(record.permit.allowed_acl_domain_keys.length === 0);
  });

  it('reads a u64 field identically as a bigint and as a decimal string', () => {
    const wire = wireOf(REFERENCE_RECORD);
    const asBigints: SolanaPermitWireFields = {
      ...wire,
      startTimestamp: BigInt(wire.startTimestamp as string),
      durationSeconds: BigInt(wire.durationSeconds as string),
      chainId: BigInt(wire.chainId as string),
    };
    expect(renderSolanaPermitText(decodeSolanaPermitFields(asBigints))).toBe(
      renderSolanaPermitText(decodeSolanaPermitFields(wire)),
    );
  });
});

describe('rejecting records', () => {
  it.each(named(rejected))('%s: is rejected by the rule it names', (_name, record) => {
    const { rejection, stage } = rejectionOfRecord(record);
    const rule = record.rule as PermitVectorRule;

    expect(rejection.code).toBe(RULE_REJECTION[rule]);
    expect(stage).toBe(DECODE_TIME_RULES.includes(rule) ? 'decode' : 'verify');
  });

  it.each(named(rejected))('%s: is a single mutation of a record this implementation accepts', (_name, record) => {
    const base = file.vectors.find((candidate) => candidate.name === record.derived_from);
    expect(base, `${record.name}: base record ${String(record.derived_from)} is missing`).toBeDefined();
    expect(record.mutation).toBeDefined();
    if (base === undefined) {
      return;
    }

    // The rejection above is attributable to the mutation only if the base itself passes here.
    const fields = decodeSolanaPermitFields(wireOf(base));
    expect(() => verifySolanaPermitSignature(fields, unhex(base.signature))).not.toThrow();
  });
});

describe('records whose signature covers a text other than the canonical one', () => {
  const shownADifferentText = file.vectors.filter((record) => record.signed_text !== undefined);

  it('are present in the file', () => {
    expect(shownADifferentText.length).toBeGreaterThan(0);
  });

  // Verifying by reconstruction is what these records are for: the signature is genuine, made by
  // the record's own wallet, and is rejected anyway because the text it covers is not the text this
  // implementation renders. Checking the signature independently here is what makes the rejection
  // attributable to the text rather than to a signature that was never valid.
  it.each(named(shownADifferentText))('%s: the signature is genuine over the text the wallet saw', (_name, record) => {
    const userPubkey = unhex(record.permit.user_pubkey);
    const shown = new TextEncoder().encode(record.signed_text);
    const envelope = new Uint8Array(PERMIT_ENVELOPE_PREAMBLE.length + 2 + userPubkey.length + shown.length);
    envelope.set(PERMIT_ENVELOPE_PREAMBLE, 0);
    envelope[PERMIT_ENVELOPE_PREAMBLE.length] = PERMIT_ENVELOPE_VERSION;
    envelope[PERMIT_ENVELOPE_PREAMBLE.length + 1] = PERMIT_ENVELOPE_SIGNER_COUNT;
    envelope.set(userPubkey, PERMIT_ENVELOPE_PREAMBLE.length + 2);
    envelope.set(shown, PERMIT_ENVELOPE_PREAMBLE.length + 2 + userPubkey.length);

    expect(ed25519.verify(unhex(record.signature), envelope, userPubkey)).toBe(true);
  });
});

describe('numeric inputs no vector can express', () => {
  const reference = () => wireOf(REFERENCE_RECORD);

  it('rejects a u64 field handed over as a number', () => {
    const wire = reference();
    const lossy = { ...wire, chainId: Number(wire.chainId) as unknown as SolanaPermitU64 };
    expect(rejectionFrom(() => decodeSolanaPermitFields(lossy))).toMatchObject({
      code: 'LossyNumericInput',
      field: 'chainId',
    });
  });

  // One value, one spelling: everything below either is not a u64 or is a second way to write one.
  const notAU64: ReadonlyArray<readonly [string, SolanaPermitU64]> = [
    ['a negative bigint', -1n],
    ['a bigint past u64', 2n ** 64n],
    ['a negative decimal', '-1'],
    ['a signed decimal', '+1'],
    ['a decimal with a leading zero', '01'],
    ['a decimal with surrounding space', ' 1'],
    ['a decimal point', '1.0'],
    ['hexadecimal', '0x10'],
    ['the empty string', ''],
    ['a decimal past u64', '18446744073709551616'],
  ];

  it.each(notAU64)('rejects %s as a chain id', (_label, chainId) => {
    expect(rejectionFrom(() => decodeSolanaPermitFields({ ...reference(), chainId }))).toMatchObject({
      code: 'NumericFieldNotU64',
      field: 'chainId',
    });
  });

  it.each(notAU64)('rejects %s as a start timestamp', (_label, startTimestamp) => {
    expect(rejectionFrom(() => decodeSolanaPermitFields({ ...reference(), startTimestamp }))).toMatchObject({
      code: 'NumericFieldNotU64',
      field: 'startTimestamp',
    });
  });

  it.each(notAU64)('rejects %s as a duration', (_label, durationSeconds) => {
    expect(rejectionFrom(() => decodeSolanaPermitFields({ ...reference(), durationSeconds }))).toMatchObject({
      code: 'NumericFieldNotU64',
      field: 'durationSeconds',
    });
  });
});
