import { readFileSync } from 'node:fs';
import { getAddressDecoder } from '@solana/kit';
import { describe, expect, it } from 'vitest';
import { hexToBytes } from '../../core/base/bytes.js';
import {
  decodeSolanaLeafQuery,
  encodeSolanaLeafProofOutcome,
  SOLANA_MAX_LEAVES_PER_READ,
  type SolanaLeafProofOutcome,
} from './leafProofs.js';

type WireQuery = {
  readonly encryptedStore: string;
  readonly handle: string;
  readonly kind: string;
  readonly key?: string;
};

// The wire the host listener serves and the KMS Connector reads, pinned on both of their sides too.
const wire = JSON.parse(
  readFileSync(new URL('../../../../../solana/test-fixtures/leaf-proofs/leaf_proofs_v1.json', import.meta.url), 'utf8'),
) as {
  maxLeavesPerRequest: number;
  request: { leaves: WireQuery[] };
  proofs: [{ leafIndex: number; leafCount: number; siblings: string[] }, { leafCount: number }, unknown, unknown];
};

const bytes = (hex: string): Uint8Array => hexToBytes(`0x${hex}`);
const address = (hex: string) => getAddressDecoder().decode(bytes(hex));

describe('the leaf-proof wire', () => {
  it('reads the pinned request', () => {
    expect(wire.request.leaves.map(decodeSolanaLeafQuery)).toEqual(
      wire.request.leaves.map(({ encryptedStore, handle, key }) => ({
        encryptedStore: address(encryptedStore),
        handle: bytes(handle),
        ...(key === undefined ? {} : { key: address(key) }),
      })),
    );
    expect(SOLANA_MAX_LEAVES_PER_READ).toBe(wire.maxLeavesPerRequest);
  });

  it('writes every pinned answer', () => {
    const [found, notFound] = wire.proofs;
    const outcomes: SolanaLeafProofOutcome[] = [
      {
        status: 'found',
        leafIndex: BigInt(found.leafIndex),
        leafCount: BigInt(found.leafCount),
        siblings: found.siblings.map(bytes),
      },
      { status: 'notFound', leafCount: BigInt(notFound.leafCount) },
      { status: 'unknownAccount' },
      { status: 'historyIncomplete' },
    ];
    expect(outcomes.map(encodeSolanaLeafProofOutcome)).toEqual(wire.proofs);
  });

  it('reads the queries the listener reads: hex with or without 0x, and a null key as none', () => {
    const [allowed, publicLeaf] = wire.request.leaves;
    expect(decodeSolanaLeafQuery({ ...allowed, handle: `0x${allowed?.handle}` })).toEqual(
      decodeSolanaLeafQuery(allowed),
    );
    expect(decodeSolanaLeafQuery({ ...publicLeaf, key: null })).toEqual(decodeSolanaLeafQuery(publicLeaf));
  });

  it('rejects a query that is neither an allow with a key nor a public leaf without one', () => {
    const [allowed, publicLeaf] = wire.request.leaves;
    expect(() => decodeSolanaLeafQuery({ ...allowed, key: undefined })).toThrow(/key/);
    expect(() => decodeSolanaLeafQuery({ ...publicLeaf, key: allowed?.key })).toThrow(/kind/);
    expect(() => decodeSolanaLeafQuery({ ...publicLeaf, handle: '0x00' })).toThrow(/handle/);
  });
});
