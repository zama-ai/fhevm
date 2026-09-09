// Assembling the request, against the shape the relayer parses.
//
// The accepted records of `solana/test-fixtures/user-decrypt/relayer_envelope_v1.json` are the same
// ones `relayer/tests/solana_envelope_fixture.rs` feeds to the endpoint's wire type. Here they are the
// expected *output*: what this builder emits must equal them field for field, which is what makes the
// seam a contract rather than two implementations that happen to agree today.
//
// The rest of the file covers what no fixture record can express, because a record is a request that
// exists: an empty list, a request over the bit budget, a handle of a type nobody sized, a handle of
// another host chain. And one property that matters more than any of them — the builder never signs.

import type { SolanaUserDecryptHandleEntry, SolanaUserDecryptRequestFailure } from './index.js';
import type { SolanaPermitFields, SolanaSignedPermit } from '../permit/index.js';
import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import {
  MAX_DECRYPTION_REQUEST_BITS,
  decryptionRequestBitsOfHandle,
} from '../../core/handle/decryptionRequestBudget.js';
import {
  MAX_SOLANA_USER_DECRYPT_HANDLES,
  SOLANA_SRFC38_ATTESTATION_TYPE,
  SolanaUserDecryptRequestError,
  buildSolanaUserDecryptRequest,
} from './index.js';
import { decodeSolanaPermitFields } from '../permit/index.js';
import { bytesToHex, hexToBytes } from '../proof.js';

/* eslint-disable @typescript-eslint/naming-convention -- the fixtures' own field names are snake_case */

interface EnvelopeFixture {
  readonly attestation_type: string;
  readonly permit_source: { readonly record: string };
  readonly accepted: readonly {
    readonly name: string;
    readonly handles: readonly Readonly<Record<string, string>>[];
  }[];
}

interface PermitCanon {
  readonly transport_keys: Readonly<Record<string, string>>;
  readonly vectors: readonly {
    readonly name: string;
    readonly permit: {
      readonly user_pubkey: string;
      readonly transport_key: string;
      readonly allowed_scopes: readonly string[];
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

const PERMIT_CHAIN_ID = BigInt(canonRecord.permit.chain_id);

const permitFields = (): SolanaPermitFields =>
  decodeSolanaPermitFields({
    userPubkey: hexToBytes(`0x${canonRecord.permit.user_pubkey}`),
    transportKey: hexToBytes(`0x${transportKeyHex}`),
    allowedScopes: canonRecord.permit.allowed_scopes.map((scope) => hexToBytes(`0x${scope}`)),
    startTimestamp: canonRecord.permit.start_timestamp,
    durationSeconds: canonRecord.permit.duration_seconds,
    verifyingProgramId: hexToBytes(`0x${canonRecord.permit.verifying_program_id}`),
    chainId: canonRecord.permit.chain_id,
    extraData: hexToBytes(`0x${canonRecord.permit.extra_data}`),
  });

const signedPermit = (): SolanaSignedPermit => ({
  fields: permitFields(),
  signature: hexToBytes(`0x${canonRecord.signature}`),
});

/** The request body the fixture pins for one record, composed as the fixture states. */
const expectedBody = (record: (typeof fixture.accepted)[number]): unknown => ({
  attestationType: fixture.attestation_type,
  attestedPayload: {
    userPubkey: `0x${canonRecord.permit.user_pubkey}`,
    transportKey: `0x${transportKeyHex}`,
    allowedScopes: canonRecord.permit.allowed_scopes.map((scope) => `0x${scope}`),
    requestValidity: {
      startTimestamp: canonRecord.permit.start_timestamp,
      durationSeconds: canonRecord.permit.duration_seconds,
    },
    verifyingProgramId: `0x${canonRecord.permit.verifying_program_id}`,
    chainId: canonRecord.permit.chain_id,
    extraData: `0x${canonRecord.permit.extra_data}`,
    handles: record.handles,
  },
  signature: `0x${canonRecord.signature}`,
});

/** The entries a fixture record's handles stand for, in the order it lists them. */
const entriesOf = (record: (typeof fixture.accepted)[number]): readonly SolanaUserDecryptHandleEntry[] =>
  record.handles.map((entry) => ({
    handle: hexToBytes(entry.handle ?? '0x'),
    allowedKey: hexToBytes(entry.allowedKey ?? '0x'),
    encryptedValueAccount: hexToBytes(entry.encryptedValueAccount ?? '0x'),
  }));

////////////////////////////////////////////////////////////////////////////////
// Handles built here, for the cases no record can carry
////////////////////////////////////////////////////////////////////////////////

/** A handle of a given FHE type on a given host chain: chain id at bytes 22..30, type at 30. */
function handleOf(fheTypeId: number, chainId: bigint = PERMIT_CHAIN_ID, fill = 0xa1): Uint8Array {
  const handle = new Uint8Array(32).fill(fill);
  new DataView(handle.buffer, handle.byteOffset, handle.byteLength).setBigUint64(22, chainId, false);
  handle[30] = fheTypeId;
  handle[31] = 0;
  return handle;
}

const EBOOL_TYPE_ID = 0;
const EBOOL_BITS = decryptionRequestBitsOfHandle(handleOf(EBOOL_TYPE_ID)) ?? 0;

// The widest sized type. The bit budget's boundary is only reachable in a type this wide: in small
// types the handle-count cap refuses the list long before the bits add up.
const EUINT256_TYPE_ID = 8;
const EUINT256_BITS = decryptionRequestBitsOfHandle(handleOf(EUINT256_TYPE_ID)) ?? 0;

const entriesOfType = (typeId: number, count: number): readonly SolanaUserDecryptHandleEntry[] =>
  Array.from({ length: count }, (_, index) => entryFor(handleOf(typeId, PERMIT_CHAIN_ID, index % 251)));

const entryFor = (
  handle: Uint8Array,
  overrides: Partial<SolanaUserDecryptHandleEntry> = {},
): SolanaUserDecryptHandleEntry => ({
  handle,
  allowedKey: hexToBytes(`0x${canonRecord.permit.user_pubkey}`),
  encryptedValueAccount: new Uint8Array(32).fill(0xea),
  ...overrides,
});

/** The assembly failure a call produced, or a failure if it produced anything else. */
function failureOf(call: () => unknown): SolanaUserDecryptRequestFailure {
  try {
    call();
  } catch (error) {
    if (error instanceof SolanaUserDecryptRequestError) {
      return error.failure;
    }
    throw error;
  }
  throw new Error('expected the request to be refused, the call returned');
}

////////////////////////////////////////////////////////////////////////////////

describe('the request the fixture pins', () => {
  it.each(fixture.accepted.map((record) => [record.name, record] as const))(
    '%s: is what this builder emits',
    (_name, record) => {
      const body = buildSolanaUserDecryptRequest({ signedPermit: signedPermit(), entries: entriesOf(record) });
      expect(body).toEqual(expectedBody(record));
    },
  );

  it('declares the attestation type the relayer dispatches on', () => {
    expect(SOLANA_SRFC38_ATTESTATION_TYPE).toBe(fixture.attestation_type);
  });
});

describe('the builder and the wallet', () => {
  // The permit is the reusable object. If assembling a request needed a signature, every retry
  // would cost the user another wallet prompt — and each prompt is a chance to sign something else.
  it('cites the one signature the permit carries, and produces the same body twice', () => {
    const permit = signedPermit();
    const entries = [entryFor(handleOf(EBOOL_TYPE_ID))];

    const first = buildSolanaUserDecryptRequest({ signedPermit: permit, entries });
    const second = buildSolanaUserDecryptRequest({ signedPermit: permit, entries });

    expect(first.signature).toBe(bytesToHex(permit.signature));
    expect(second).toEqual(first);
  });
});

describe('the entry list', () => {
  it('is refused when it is empty', () => {
    expect(failureOf(() => buildSolanaUserDecryptRequest({ signedPermit: signedPermit(), entries: [] }))).toEqual({
      reason: 'no-handles',
    });
  });

  // Position is meaning: the response linker binds the ordered list, so a builder that sorted or
  // de-duplicated would produce a request the caller never made — and the answer would bind to that
  // request rather than to theirs.
  it('keeps duplicates and order exactly as given', () => {
    const first = handleOf(EBOOL_TYPE_ID, PERMIT_CHAIN_ID, 0xa1);
    const second = handleOf(EBOOL_TYPE_ID, PERMIT_CHAIN_ID, 0xb2);
    const body = buildSolanaUserDecryptRequest({
      signedPermit: signedPermit(),
      entries: [entryFor(first), entryFor(second), entryFor(first)],
    });

    expect(body.attestedPayload.handles.map((entry) => entry.handle)).toEqual([
      bytesToHex(first),
      bytesToHex(second),
      bytesToHex(first),
    ]);
  });

  // The wire identity is the triple: the same handle under the same key can live under two
  // encrypted value accounts, and collapsing them would answer for an account the caller never named.
  it('keeps each entry its own account, even under one (handle, key)', () => {
    const handle = handleOf(EBOOL_TYPE_ID);
    const firstAccount = new Uint8Array(32).fill(0xe1);
    const secondAccount = new Uint8Array(32).fill(0xe2);
    const body = buildSolanaUserDecryptRequest({
      signedPermit: signedPermit(),
      entries: [
        entryFor(handle, { encryptedValueAccount: firstAccount }),
        entryFor(handle, { encryptedValueAccount: secondAccount }),
      ],
    });

    expect(body.attestedPayload.handles.map((entry) => entry.encryptedValueAccount)).toEqual([
      bytesToHex(firstAccount),
      bytesToHex(secondAccount),
    ]);
  });

  it('names the entry whose key or account is not 32 bytes', () => {
    const permit = signedPermit();
    const handle = handleOf(EBOOL_TYPE_ID);

    expect(
      failureOf(() =>
        buildSolanaUserDecryptRequest({
          signedPermit: permit,
          entries: [entryFor(handle), entryFor(handle, { allowedKey: new Uint8Array(31) })],
        }),
      ),
    ).toEqual({ reason: 'entry-field-width', index: 1, field: 'allowedKey' });

    expect(
      failureOf(() =>
        buildSolanaUserDecryptRequest({
          signedPermit: permit,
          entries: [entryFor(handle, { encryptedValueAccount: new Uint8Array(33) })],
        }),
      ),
    ).toEqual({ reason: 'entry-field-width', index: 0, field: 'encryptedValueAccount' });
  });
});

describe('the handle count', () => {
  // The Gateway refuses a request past the cap before any fee, and the Connector refuses the same
  // count terminally — so a list the cap refuses can never be paid for and answered, and refusing it
  // here is what the pre-check layer is for. The cap catches what the bit budget cannot: many small
  // handles, far under the budget in bits.
  it('admits a request of exactly the cap', () => {
    const body = buildSolanaUserDecryptRequest({
      signedPermit: signedPermit(),
      entries: entriesOfType(EBOOL_TYPE_ID, MAX_SOLANA_USER_DECRYPT_HANDLES),
    });
    expect(body.attestedPayload.handles).toHaveLength(MAX_SOLANA_USER_DECRYPT_HANDLES);
  });

  it('refuses one handle past the cap, well under the bit budget', () => {
    const count = MAX_SOLANA_USER_DECRYPT_HANDLES + 1;
    expect((count + 1) * EBOOL_BITS).toBeLessThan(MAX_DECRYPTION_REQUEST_BITS);
    expect(
      failureOf(() =>
        buildSolanaUserDecryptRequest({ signedPermit: signedPermit(), entries: entriesOfType(EBOOL_TYPE_ID, count) }),
      ),
    ).toEqual({ reason: 'too-many-handles', count, max: MAX_SOLANA_USER_DECRYPT_HANDLES });
  });

  // The cap is the Gateway's number, mirrored by hand; a mirror nobody checks is worth less than no
  // mirror at all.
  it('matches the cap the Gateway declares', () => {
    const source = readFileSync(
      new URL('../../../../../gateway-contracts/contracts/Decryption.sol', import.meta.url),
      'utf8',
    );
    const declared = /MAX_SOLANA_USER_DECRYPT_HANDLES = (\d+);/.exec(source)?.[1];
    expect(declared, 'Decryption.sol declares MAX_SOLANA_USER_DECRYPT_HANDLES').toBeDefined();
    expect(MAX_SOLANA_USER_DECRYPT_HANDLES).toBe(Number(declared));
  });
});

describe('the bit budget', () => {
  // The boundary pair, in the widest sized type — the one type whose bits can reach the budget
  // inside the handle-count cap: exactly full, then one handle more.
  it('admits a request of exactly the budget', () => {
    const full = MAX_DECRYPTION_REQUEST_BITS / EUINT256_BITS;
    expect(full).toBeLessThanOrEqual(MAX_SOLANA_USER_DECRYPT_HANDLES);
    const body = buildSolanaUserDecryptRequest({
      signedPermit: signedPermit(),
      entries: entriesOfType(EUINT256_TYPE_ID, full),
    });
    expect(body.attestedPayload.handles).toHaveLength(full);
  });

  // Refused, never trimmed to fit: a trimmed request is answered as a request the caller never made,
  // and the budget is not what would catch that — the trimmed list is inside it.
  it('refuses one handle past the budget, and says by how much', () => {
    const full = MAX_DECRYPTION_REQUEST_BITS / EUINT256_BITS;
    expect(
      failureOf(() =>
        buildSolanaUserDecryptRequest({
          signedPermit: signedPermit(),
          entries: entriesOfType(EUINT256_TYPE_ID, full + 1),
        }),
      ),
    ).toEqual({
      reason: 'budget-exceeded',
      bits: MAX_DECRYPTION_REQUEST_BITS + EUINT256_BITS,
      budget: MAX_DECRYPTION_REQUEST_BITS,
    });
  });

  it('names the handle whose type has no width', () => {
    // Type 1 is the deprecated euint4: the Gateway assigns it no size and reverts on it, so the
    // pre-check must refuse it rather than count it as free.
    const unsized = handleOf(1);
    expect(decryptionRequestBitsOfHandle(unsized)).toBeUndefined();
    expect(
      failureOf(() =>
        buildSolanaUserDecryptRequest({
          signedPermit: signedPermit(),
          entries: [entryFor(handleOf(EBOOL_TYPE_ID)), entryFor(unsized)],
        }),
      ),
    ).toEqual({ reason: 'handle-without-a-width', index: 1 });
  });
});

describe('the host chain', () => {
  // A handle of another chain cannot be authorized under this permit, and the relayer would refuse it
  // on the chain id it embeds. Refusing locally is what keeps that from costing a round trip.
  it('names the handle that belongs to another host chain', () => {
    const foreign = handleOf(EBOOL_TYPE_ID, 0x8000_0000_0000_0001n);
    expect(
      failureOf(() =>
        buildSolanaUserDecryptRequest({
          signedPermit: signedPermit(),
          entries: [entryFor(handleOf(EBOOL_TYPE_ID)), entryFor(foreign)],
        }),
      ),
    ).toEqual({ reason: 'foreign-host-chain', index: 1, chainId: 0x8000_0000_0000_0001n });
  });
});
