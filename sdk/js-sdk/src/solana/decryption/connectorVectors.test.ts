// The two request bodies, held to the Connector's own vectors.
//
// `kms-connector/crates/api/tests/vectors.json` is written by the Connector's DTO tests, which check
// each vector's serde round trip and decryption id. A body this SDK sends must have the same shape:
// the same keys at the same depths (in any order, as serde reads them), the same scheme strings, numbers where the DTO has numbers, and
// hex of the same width where the DTO has fixed-width bytes. The vectors' values are not ones a valid
// permit can carry (an all-zero user, an empty transport key), so the bodies here come from real
// inputs and are compared by shape; `request.test.ts` pins the user body's values.

import { readFileSync } from 'node:fs';
import { afterEach, describe, expect, it, vi } from 'vitest';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import { asBytes32Hex } from '../../core/base/bytes.js';
import { buildHandle } from '../../core/handle/FhevmHandle.js';
import { publicDecryptCertificate } from '../actions/publicDecryptCertificate.js';
import {
  PERMIT_IDENTITY_LEN,
  PERMIT_KMS_ROUTING_LEN,
  PERMIT_KMS_ROUTING_VERSION,
  PERMIT_SIGNATURE_LEN,
  PERMIT_TRANSPORT_KEY_LEN,
  decodeSolanaPermitFields,
} from '../permit/index.js';
import { buildSolanaUserDecryptRequest } from '../userDecrypt/index.js';

type Vector = { readonly name: string; readonly request: unknown };
const vectors = JSON.parse(
  readFileSync(new URL('../../../../../kms-connector/crates/api/tests/vectors.json', import.meta.url), 'utf8'),
) as { readonly solanaUserDecryption?: readonly Vector[]; readonly solanaPublicDecryption?: readonly Vector[] };

/** The vectors under `key`, which the Connector's DTO tests must have written. */
function vectorsOf(key: 'solanaUserDecryption' | 'solanaPublicDecryption'): readonly Vector[] {
  const list = vectors[key];
  if (list === undefined || list.length === 0) throw new Error(`vectors.json has no ${key} vectors`);
  return list;
}

/**
 * The shape of a JSON value: objects by their keys, arrays by their first element, numbers as
 * `number`, 32-byte hex as `bytes32`, other hex as `bytes`, and any other string as itself — the
 * scheme strings are part of the shape.
 */
function shapeOf(value: unknown): unknown {
  if (Array.isArray(value)) return value.length === 0 ? ['array'] : ['array', shapeOf(value[0])];
  if (typeof value === 'number') return 'number';
  if (typeof value === 'string') {
    if (!/^0x([0-9a-f]{2})*$/.test(value)) return value;
    return value.length === 66 ? 'bytes32' : 'bytes';
  }
  if (typeof value === 'object' && value !== null) {
    return Object.fromEntries(Object.entries(value).map(([key, field]) => [key, shapeOf(field)]));
  }
  return typeof value;
}

/**
 * Compares shapes, keys in any order, letting an empty array stand for an array of any element: the
 * Connector's empty public request carries no handle to take an element shape from.
 */
function expectSameShape(actual: unknown, expected: unknown, path = '$'): void {
  if (Array.isArray(actual) && Array.isArray(expected)) {
    expect(actual[0], path).toBe('array');
    expect(expected[0], path).toBe('array');
    if (actual.length > 1 && expected.length > 1) expectSameShape(actual[1], expected[1], `${path}[0]`);
    return;
  }
  if (typeof actual === 'object' && actual !== null && typeof expected === 'object' && expected !== null) {
    expect(Object.keys(actual).sort(), path).toEqual(Object.keys(expected).sort());
    for (const key of Object.keys(expected)) {
      expectSameShape(
        (actual as Record<string, unknown>)[key],
        (expected as Record<string, unknown>)[key],
        `${path}.${key}`,
      );
    }
    return;
  }
  expect(actual, path).toEqual(expected);
}

const CHAIN_ID = 5n;
const identity = (fill: number): Uint8Array => new Uint8Array(PERMIT_IDENTITY_LEN).fill(fill);

describe('the user-decryption body', () => {
  it.each(vectorsOf('solanaUserDecryption').map((vector) => [vector.name, vector] as const))(
    'has the shape of the Connector vector %s',
    (_name, vector) => {
      const routing = new Uint8Array(PERMIT_KMS_ROUTING_LEN);
      routing[0] = PERMIT_KMS_ROUTING_VERSION;
      const body = buildSolanaUserDecryptRequest({
        signedPermit: {
          fields: decodeSolanaPermitFields({
            userAddress: identity(0x11),
            transportKey: new Uint8Array(PERMIT_TRANSPORT_KEY_LEN),
            allowedScopes: [],
            startTimestamp: 1_767_229_380n,
            durationSeconds: 3_600n,
            verifyingProgramId: identity(0x22),
            chainId: CHAIN_ID,
            extraData: routing,
          }),
          signature: new Uint8Array(PERMIT_SIGNATURE_LEN).fill(0x77),
        },
        entries: [
          {
            handle: buildHandle({ chainId: CHAIN_ID, hash21: `0x${'a1'.repeat(21)}`, fheTypeId: 0 }).bytes32,
            ownerAddress: identity(0x11),
            encryptedStore: identity(0xea),
          },
        ],
      });
      expectSameShape(shapeOf(body), shapeOf(vector.request));
    },
  );
});

describe('the public-decryption body', () => {
  const originalFetch = global.fetch;
  afterEach(() => {
    global.fetch = originalFetch;
  });

  it.each(vectorsOf('solanaPublicDecryption').map((vector) => [vector.name, vector] as const))(
    'has the shape of the Connector vector %s',
    async (_name, vector) => {
      const contextId = new Uint8Array(32).fill(5);
      const epochId = new Uint8Array(32).fill(6);
      const fetchMock = vi.fn().mockResolvedValue(new Response(JSON.stringify({ code: 'malformed' }), { status: 400 }));
      global.fetch = fetchMock;
      const chain = {
        id: CHAIN_ID,
        fhevm: {
          relayerUrl: 'https://relayer.example.test',
          decryptionRelayerUrl: 'https://decryption.relayer.example.test',
          programs: { host: { address: asBytes32Hex(`0x${'22'.repeat(32)}`) } },
        },
      } as FhevmSolanaChain;
      await publicDecryptCertificate(
        { chain, runtime: { config: {} } as FhevmRuntime },
        {
          entries: [
            {
              handle: buildHandle({ chainId: CHAIN_ID, hash21: `0x${'b1'.repeat(21)}`, fheTypeId: 0 }).bytes32,
              encryptedStore: new Uint8Array(32).fill(0xea),
            },
          ],
          contextId,
          epochId,
        },
      ).catch(() => undefined);
      const body: unknown = JSON.parse(String((fetchMock.mock.calls[0]?.[1] as RequestInit).body));
      expectSameShape(shapeOf(body), shapeOf(vector.request));
    },
  );
});
