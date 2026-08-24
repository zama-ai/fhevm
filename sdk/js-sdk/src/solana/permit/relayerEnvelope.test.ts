// The relayer envelope fixture, read from this side of the seam.
//
// `solana/test-fixtures/user-decrypt/relayer_envelope_v1.json` is consumed by two implementations:
// `relayer/tests/solana_envelope_fixture.rs` feeds these records to the endpoint's own wire type, and
// this file reads the same records with the SDK. A shared file is the point — a key renamed on one
// side and mirrored in that side's own test looks green twice and fails in production.
//
// What is pinned here is everything checkable without the request builder: the derivation from the
// permit canon, the payload's key set and the form of each value, and that the permit half the
// fixture names decodes and verifies through this SDK. The byte-for-byte comparison of a built
// request against these records belongs to the builder's own tests, which consume the same file.

import type { SolanaPermitWireFields } from './index.js';
import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import {
  PERMIT_SIGNATURE_LEN,
  buildSolanaPermitEnvelope,
  decodeSolanaPermitFields,
  verifySolanaPermitSignature,
} from './index.js';
import { hexToBytes } from '../proof.js';

/* eslint-disable @typescript-eslint/naming-convention -- the fixtures' own field names are snake_case */

interface EnvelopeFixture {
  readonly schema: string;
  readonly attestation_type: string;
  readonly permit_source: { readonly file: string; readonly record: string };
  readonly composition: string;
  readonly rejected_by_values: Readonly<Record<string, string>>;
  readonly accepted: readonly EnvelopeRecord[];
  readonly rejected: readonly EnvelopeRecord[];
}

interface EnvelopeRecord {
  readonly name: string;
  readonly comment: string;
  readonly rejected_by?: string;
  readonly payload_extra?: Readonly<Record<string, unknown>>;
  readonly handles: readonly Readonly<Record<string, unknown>>[];
}

interface PermitCanon {
  readonly transport_keys: Readonly<Record<string, string>>;
  readonly vectors: readonly {
    readonly name: string;
    readonly permit: {
      readonly user_pubkey: string;
      readonly transport_key: string;
      readonly allowed_acl_domain_keys: readonly string[];
      readonly start_timestamp: string;
      readonly duration_seconds: string;
      readonly verifying_program_id: string;
      readonly chain_id: string;
      readonly extra_data: string;
    };
    readonly signature: string;
  }[];
}

/* eslint-enable @typescript-eslint/naming-convention */

const FIXTURE_DIR = new URL('../../../../../solana/test-fixtures/', import.meta.url);
const read = <T>(relative: string): T => JSON.parse(readFileSync(new URL(relative, FIXTURE_DIR), 'utf8')) as T;

const fixture = read<EnvelopeFixture>('user-decrypt/relayer_envelope_v1.json');
const canon = read<PermitCanon>('permit/permit_v1.json');

const canonRecord = canon.vectors.find((vector) => vector.name === fixture.permit_source.record);
if (canonRecord === undefined) {
  throw new Error(`the permit canon carries no record named ${fixture.permit_source.record}`);
}
const transportKeyHex = canon.transport_keys[canonRecord.permit.transport_key];
if (transportKeyHex === undefined) {
  throw new Error(`the permit canon has no transport key named ${canonRecord.permit.transport_key}`);
}

/** The permit half of every request, derived from the canon exactly as the fixture states. */
const permitPayload: Readonly<Record<string, unknown>> = {
  userPubkey: `0x${canonRecord.permit.user_pubkey}`,
  transportKey: `0x${transportKeyHex}`,
  allowedAclDomainKeys: canonRecord.permit.allowed_acl_domain_keys.map((key) => `0x${key}`),
  requestValidity: {
    startTimestamp: canonRecord.permit.start_timestamp,
    durationSeconds: canonRecord.permit.duration_seconds,
  },
  verifyingProgramId: `0x${canonRecord.permit.verifying_program_id}`,
  chainId: canonRecord.permit.chain_id,
  extraData: `0x${canonRecord.permit.extra_data}`,
};

/** Composes one record into the request body, exactly as the fixture's `composition` states. */
const compose = (record: EnvelopeRecord): Record<string, unknown> => ({
  attestationType: fixture.attestation_type,
  attestedPayload: { ...permitPayload, handles: record.handles, ...record.payload_extra },
  signature: `0x${canonRecord.signature}`,
});

/** The keys the payload carries, and the only ones the endpoint's wire type admits. */
const PAYLOAD_KEYS = [
  'userPubkey',
  'transportKey',
  'allowedAclDomainKeys',
  'requestValidity',
  'verifyingProgramId',
  'chainId',
  'extraData',
  'handles',
] as const;

/** The keys one handle entry carries. */
const HANDLE_KEYS = ['handle', 'subject', 'encryptedValueId', 'proofLeafCount', 'accessProof'] as const;

const named = <T extends { readonly name: string }>(records: readonly T[]): ReadonlyArray<readonly [string, T]> =>
  records.map((record) => [record.name, record] as const);

////////////////////////////////////////////////////////////////////////////////

describe('the relayer envelope fixture', () => {
  it('is read under the schema it declares, and names a record of the permit canon', () => {
    expect(fixture.schema).toBe('zama-solana-user-decrypt-envelope/v1');
    expect(fixture.attestation_type).toBe('solana-srfc38-user-decrypt-v1');
    expect(fixture.permit_source.file).toBe('../permit/permit_v1.json');
    expect(fixture.accepted.length).toBeGreaterThan(0);
    expect(fixture.rejected.length).toBeGreaterThan(0);
  });

  it('carries no rejecting record without a documented layer', () => {
    const documented = Object.keys(fixture.rejected_by_values);
    for (const record of fixture.rejected) {
      expect(documented, `${record.name} names an undocumented layer`).toContain(record.rejected_by);
    }
  });
});

describe('an accepted record composed into a request', () => {
  it.each(named(fixture.accepted))('%s: carries exactly the payload keys the endpoint admits', (_name, record) => {
    const body = compose(record);
    expect(Object.keys(body).sort()).toEqual(['attestationType', 'attestedPayload', 'signature']);
    expect(Object.keys(body.attestedPayload as Record<string, unknown>).sort()).toEqual([...PAYLOAD_KEYS].sort());
  });

  // Both sides read the same records, and each does so through its own JSON layer. Which fields are
  // `0x`-hex and which are decimal strings is exactly the kind of difference that survives a review
  // and fails at runtime, so it is spelled out rather than inferred from the values that happen to
  // be there.
  it.each(named(fixture.accepted))('%s: writes hex as 0x-hex and every u64 as a decimal string', (_name, record) => {
    const payload = compose(record).attestedPayload as Record<string, unknown>;
    for (const key of ['userPubkey', 'transportKey', 'verifyingProgramId', 'extraData']) {
      expect(payload[key], key).toMatch(/^0x[0-9a-f]*$/);
    }
    expect(payload.chainId).toMatch(/^(0|[1-9][0-9]*)$/);
    const validity = payload.requestValidity as Record<string, unknown>;
    expect(validity.startTimestamp).toMatch(/^(0|[1-9][0-9]*)$/);
    expect(validity.durationSeconds).toMatch(/^(0|[1-9][0-9]*)$/);

    for (const entry of record.handles) {
      expect(Object.keys(entry).sort()).toEqual([...HANDLE_KEYS].sort());
      expect(entry.handle).toMatch(/^0x[0-9a-f]{64}$/);
      expect(entry.subject).toMatch(/^0x[0-9a-f]{64}$/);
      expect(entry.encryptedValueId).toMatch(/^0x[0-9a-f]{64}$/);
      expect(entry.proofLeafCount).toMatch(/^(0|[1-9][0-9]*)$/);
      expect(entry.accessProof).toMatch(/^0x([0-9a-f]{2})*$/);
    }
  });

  // The same property the Rust consumer asserts, computed here from the handle bytes: the chain id a
  // handle embeds is the chain the permit was signed for. Asserted on both sides because a typo in a
  // hand-written handle would otherwise read as a bug in whichever implementation looked first.
  it.each(named(fixture.accepted))('%s: names handles of the host chain the permit is signed for', (_name, record) => {
    for (const entry of record.handles) {
      const handle = hexToBytes(entry.handle as string);
      const embedded = new DataView(handle.buffer, handle.byteOffset, handle.byteLength).getBigUint64(22, false);
      expect(embedded).toBe(BigInt(canonRecord.permit.chain_id));
    }
  });
});

describe('the permit the fixture is built on', () => {
  const wire = (): SolanaPermitWireFields => ({
    userPubkey: hexToBytes(`0x${canonRecord.permit.user_pubkey}`),
    transportKey: hexToBytes(`0x${transportKeyHex}`),
    allowedAclDomainKeys: canonRecord.permit.allowed_acl_domain_keys.map((key) => hexToBytes(`0x${key}`)),
    startTimestamp: canonRecord.permit.start_timestamp,
    durationSeconds: canonRecord.permit.duration_seconds,
    verifyingProgramId: hexToBytes(`0x${canonRecord.permit.verifying_program_id}`),
    chainId: canonRecord.permit.chain_id,
    extraData: hexToBytes(`0x${canonRecord.permit.extra_data}`),
  });

  // The link between the two fixtures: the request the relayer will read carries a permit this SDK
  // considers well formed and correctly signed. Without this, the envelope fixture could drift into
  // citing a permit no implementation accepts and every shape assertion above would still pass.
  it('decodes and verifies through this implementation', () => {
    const fields = decodeSolanaPermitFields(wire());
    const signature = hexToBytes(`0x${canonRecord.signature}`);
    expect(signature).toHaveLength(PERMIT_SIGNATURE_LEN);
    expect(() => verifySolanaPermitSignature(fields, signature)).not.toThrow();
    expect(buildSolanaPermitEnvelope(fields).length).toBeGreaterThan(0);
  });
});
