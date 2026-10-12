// Assembling the request, against the shape the Connector parses.
//
// The permit comes from the shared canon (`solana/test-fixtures/permit/permit_v1.json`), and the body
// it yields is pinned field for field here; `decryption/connectorVectors.test.ts` holds the same body
// to the Connector's own vectors.
//
// The rest of the file covers what no vector can express, because a vector is a request that exists:
// an empty list, a request over the bit budget, a handle of a type nobody sized, a handle of another
// host chain. And one property that matters more than any of them — the builder never signs.

import type { SolanaUserDecryptHandleEntry, SolanaUserDecryptRequestFailure } from './index.js';
import type { SolanaPermitFields, SolanaSignedPermit } from '../permit/index.js';
import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import {
  MAX_DECRYPTION_REQUEST_BITS,
  decryptionRequestBitsOfHandle,
} from '../../core/handle/decryptionRequestBudget.js';
import {
  MAX_SOLANA_DECRYPT_HANDLES,
  SOLANA_SRFC38_ATTESTATION_TYPE,
  SolanaUserDecryptRequestError,
  buildSolanaUserDecryptRequest,
} from './index.js';
import { decodeSolanaPermitFields } from '../permit/index.js';
import { bytesToHex, hexToBytes } from '../../core/base/bytes.js';

/* eslint-disable @typescript-eslint/naming-convention -- the fixtures' own field names are snake_case */

interface PermitCanon {
  readonly transport_keys: Readonly<Record<string, string>>;
  readonly vectors: readonly {
    readonly name: string;
    readonly permit: {
      readonly user_address: string;
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

const canon = JSON.parse(
  readFileSync(new URL('../../../../../solana/test-fixtures/permit/permit_v1.json', import.meta.url), 'utf8'),
) as PermitCanon;

const canonRecord = canon.vectors.find((vector) => vector.name === 'reference-permit-two-scopes');
if (canonRecord === undefined) {
  throw new Error('the permit canon carries no record named reference-permit-two-scopes');
}
const transportKeyHex = canon.transport_keys[canonRecord.permit.transport_key];
if (transportKeyHex === undefined) {
  throw new Error(`the permit canon has no transport key named ${canonRecord.permit.transport_key}`);
}

const PERMIT_CHAIN_ID = BigInt(canonRecord.permit.chain_id);

const permitFields = (): SolanaPermitFields =>
  decodeSolanaPermitFields({
    userAddress: hexToBytes(`0x${canonRecord.permit.user_address}`),
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
  ownerAddress: hexToBytes(`0x${canonRecord.permit.user_address}`),
  encryptedStore: new Uint8Array(32).fill(0xea),
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

describe('the request body', () => {
  // A direct entry and a delegated one, so both owner keys travel.
  it('is the Connector DTO: the permit fields, the entries in order, and the signature', () => {
    const direct = entryFor(handleOf(EBOOL_TYPE_ID));
    const delegated = entryFor(handleOf(EUINT256_TYPE_ID), { ownerAddress: new Uint8Array(32).fill(0xde) });
    const body = buildSolanaUserDecryptRequest({ signedPermit: signedPermit(), entries: [direct, delegated] });

    expect(body).toStrictEqual({
      attestationType: 'solana-srfc38-user-decrypt-v1',
      payload: {
        handles: [direct, delegated].map((entry) => ({
          handle: bytesToHex(entry.handle),
          ownerAddress: bytesToHex(entry.ownerAddress),
          encryptedStore: bytesToHex(entry.encryptedStore),
        })),
        userAddress: `0x${canonRecord.permit.user_address}`,
        publicKey: `0x${transportKeyHex}`,
        allowedScopes: canonRecord.permit.allowed_scopes.map((scope) => `0x${scope}`),
        // JSON numbers: the DTO's u64 fields refuse strings.
        requestValidity: {
          startTimestamp: Number(canonRecord.permit.start_timestamp),
          durationSeconds: Number(canonRecord.permit.duration_seconds),
        },
        verifyingProgramId: `0x${canonRecord.permit.verifying_program_id}`,
        extraData: `0x${canonRecord.permit.extra_data}`,
      },
      signature: `0x${canonRecord.signature}`,
    });
    expect(body.attestationType).toBe(SOLANA_SRFC38_ATTESTATION_TYPE);
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

    expect(body.payload.handles.map((entry) => entry.handle)).toEqual([
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
        entryFor(handle, { encryptedStore: firstAccount }),
        entryFor(handle, { encryptedStore: secondAccount }),
      ],
    });

    expect(body.payload.handles.map((entry) => entry.encryptedStore)).toEqual([
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
          entries: [entryFor(handle), entryFor(handle, { ownerAddress: new Uint8Array(31) })],
        }),
      ),
    ).toEqual({ reason: 'entry-field-width', index: 1, field: 'ownerAddress' });

    expect(
      failureOf(() =>
        buildSolanaUserDecryptRequest({
          signedPermit: permit,
          entries: [entryFor(handle, { encryptedStore: new Uint8Array(33) })],
        }),
      ),
    ).toEqual({ reason: 'entry-field-width', index: 0, field: 'encryptedStore' });
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
      entries: entriesOfType(EBOOL_TYPE_ID, MAX_SOLANA_DECRYPT_HANDLES),
    });
    expect(body.payload.handles).toHaveLength(MAX_SOLANA_DECRYPT_HANDLES);
  });

  it('refuses one handle past the cap, well under the bit budget', () => {
    const count = MAX_SOLANA_DECRYPT_HANDLES + 1;
    expect((count + 1) * EBOOL_BITS).toBeLessThan(MAX_DECRYPTION_REQUEST_BITS);
    expect(
      failureOf(() =>
        buildSolanaUserDecryptRequest({ signedPermit: signedPermit(), entries: entriesOfType(EBOOL_TYPE_ID, count) }),
      ),
    ).toEqual({ reason: 'too-many-handles', count, max: MAX_SOLANA_DECRYPT_HANDLES });
  });

  // The cap is the Gateway's number, mirrored by hand; a mirror nobody checks is worth less than no
  // mirror at all.
  it('matches the cap the Gateway declares', () => {
    const source = readFileSync(
      new URL('../../../../../gateway-contracts/contracts/Decryption.sol', import.meta.url),
      'utf8',
    );
    const declared = /MAX_SOLANA_DECRYPT_HANDLES = (\d+);/.exec(source)?.[1];
    expect(declared, 'Decryption.sol declares MAX_SOLANA_DECRYPT_HANDLES').toBeDefined();
    expect(MAX_SOLANA_DECRYPT_HANDLES).toBe(Number(declared));
  });
});

describe('the bit budget', () => {
  // The boundary pair, in the widest sized type — the one type whose bits can reach the budget
  // inside the handle-count cap: exactly full, then one handle more.
  it('admits a request of exactly the budget', () => {
    const full = MAX_DECRYPTION_REQUEST_BITS / EUINT256_BITS;
    expect(full).toBeLessThanOrEqual(MAX_SOLANA_DECRYPT_HANDLES);
    const body = buildSolanaUserDecryptRequest({
      signedPermit: signedPermit(),
      entries: entriesOfType(EUINT256_TYPE_ID, full),
    });
    expect(body.payload.handles).toHaveLength(full);
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
    const foreign = handleOf(EBOOL_TYPE_ID, 0x0100_0000_0000_0001n);
    expect(
      failureOf(() =>
        buildSolanaUserDecryptRequest({
          signedPermit: signedPermit(),
          entries: [entryFor(handleOf(EBOOL_TYPE_ID)), entryFor(foreign)],
        }),
      ),
    ).toEqual({ reason: 'foreign-host-chain', index: 1, chainId: 0x0100_0000_0000_0001n });
  });
});
