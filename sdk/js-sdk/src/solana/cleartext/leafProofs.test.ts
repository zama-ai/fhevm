import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import {
  decodeSolanaLeafProofOutcome,
  decodeSolanaLeafQuery,
  encodeSolanaLeafProofOutcome,
  encodeSolanaLeafQuery,
  SOLANA_MAX_LEAVES_PER_READ,
} from './leafProofs.js';

// The wire the host listener serves and the KMS Connector reads, pinned on both of their sides too.
const wire = JSON.parse(
  readFileSync(new URL('../../../../../solana/test-fixtures/leaf-proofs/leaf_proofs_v1.json', import.meta.url), 'utf8'),
) as { maxLeavesPerRequest: number; request: { leaves: unknown[] }; proofs: unknown[] };

describe('the leaf-proof wire', () => {
  it('reads and writes the pinned request', () => {
    const queries = wire.request.leaves.map(decodeSolanaLeafQuery);
    expect(queries.map(encodeSolanaLeafQuery)).toEqual(wire.request.leaves);
    expect(SOLANA_MAX_LEAVES_PER_READ).toBe(wire.maxLeavesPerRequest);
  });

  it('reads and writes every pinned answer', () => {
    const outcomes = wire.proofs.map(decodeSolanaLeafProofOutcome);
    expect(outcomes.map((outcome) => outcome.status)).toEqual([
      'found',
      'notFound',
      'unknownAccount',
      'historyIncomplete',
    ]);
    expect(outcomes.map(encodeSolanaLeafProofOutcome)).toEqual(wire.proofs);
  });

  it('reads the queries the listener reads: hex with or without 0x, and a null key as none', () => {
    const [allowed, publicLeaf] = wire.request.leaves as Record<string, string>[];
    expect(decodeSolanaLeafQuery({ ...allowed, handle: `0x${allowed?.handle}` })).toEqual(
      decodeSolanaLeafQuery(allowed),
    );
    expect(decodeSolanaLeafQuery({ ...publicLeaf, key: null })).toEqual(decodeSolanaLeafQuery(publicLeaf));
  });

  it('rejects a query that is neither an allow with a key nor a public leaf without one', () => {
    const [allowed, publicLeaf] = wire.request.leaves as Record<string, unknown>[];
    expect(() => decodeSolanaLeafQuery({ ...allowed, key: undefined })).toThrow(/key/);
    expect(() => decodeSolanaLeafQuery({ ...publicLeaf, key: allowed?.key })).toThrow(/kind/);
    expect(() => decodeSolanaLeafQuery({ ...publicLeaf, handle: '0x00' })).toThrow(/handle/);
  });

  it('rejects an answer it cannot trust the shape of', () => {
    expect(() => decodeSolanaLeafProofOutcome({ status: 'found', leafIndex: -1, leafCount: 1, siblings: [] })).toThrow(
      /leafIndex/,
    );
    expect(() => decodeSolanaLeafProofOutcome({ status: 'granted' })).toThrow(/status/);
  });
});
