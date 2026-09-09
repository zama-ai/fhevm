// The MMR primitives, pinned to the normative leaf vectors.
//
// `solana/test-fixtures/leaves/leaves_v1.json` is written by the Rust shared crate the host program,
// the coprocessor and the KMS connector all run (`bash solana/scripts/update-leaf-vectors.sh`), and
// read here. Every vector is one account history: its events, and the leaves, peaks and one proof
// they imply. Reproducing all of them is what makes this module the same MMR rather than one that
// happens to agree on a hand-picked case.

import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import {
  buildPublicLeafProof,
  bytesToHex,
  hexToBytes,
  historicalAccessLeafCommitment,
  MAX_MMR_SIBLINGS,
  mmrBuildProof,
  mmrLeafNode,
  mmrNode,
  mmrPeaksFromLeaves,
  mmrVerify,
  publicDecryptLeafCommitment,
  reconstructSolanaEncryptedValueAccount,
  verifyHistoricalAccessProof,
  verifyPublicDecryptProof,
  type MmrProof,
  type SolanaEncryptedValueAccountEvent,
} from './proof.js';

/* eslint-disable @typescript-eslint/naming-convention -- the fixture's own field names are snake_case */

interface LeafVectorFile {
  readonly schema: string;
  readonly hash: string;
  readonly prefixes: {
    readonly historical_access_leaf: string;
    readonly public_decrypt_leaf: string;
    readonly mmr_leaf_node: string;
    readonly mmr_node: string;
  };
  readonly vectors: readonly LeafVector[];
}

interface LeafVector {
  readonly id: string;
  readonly comment: string;
  readonly encrypted_value_account: string;
  readonly events: readonly (
    | { readonly kind: 'allowed'; readonly handle: string; readonly key: string }
    | { readonly kind: 'marked_public'; readonly handle: string }
  )[];
  readonly leaves: readonly string[];
  readonly leaf_count: string;
  readonly peaks: readonly string[];
  readonly proof: { readonly leaf_index: string; readonly siblings: readonly string[] };
}

/* eslint-enable @typescript-eslint/naming-convention */

const file = JSON.parse(
  readFileSync(new URL('../../../../solana/test-fixtures/leaves/leaves_v1.json', import.meta.url), 'utf8'),
) as LeafVectorFile;

const unhex = (hex: string): Uint8Array => hexToBytes(`0x${hex}`);
const rehex = (bytes: Uint8Array): string => bytesToHex(bytes).slice(2);

const eventsOf = (vector: LeafVector): readonly SolanaEncryptedValueAccountEvent[] =>
  vector.events.map((event) =>
    event.kind === 'allowed'
      ? { kind: 'allowed', handle: unhex(event.handle), key: unhex(event.key) }
      : { kind: 'markedPublic', handle: unhex(event.handle) },
  );

const proofOf = (vector: LeafVector): MmrProof => ({
  leafIndex: BigInt(vector.proof.leaf_index),
  siblings: vector.proof.siblings.map(unhex),
});

const named = file.vectors.map((vector) => [vector.id, vector] as const);

////////////////////////////////////////////////////////////////////////////////

describe('the leaf vector file', () => {
  it('is read under the schema it declares, and names the hash and prefixes this module uses', () => {
    expect(file.schema).toBe('zama-solana-leaf-vectors/v1');
    expect(file.hash).toBe('keccak256');
    expect(file.prefixes).toEqual({
      historical_access_leaf: 'ZAMA_HIST_ACCESS_LEAF_V1',
      public_decrypt_leaf: 'ZAMA_PUBLIC_DECRYPT_LEAF_V1',
      mmr_leaf_node: 'ZAMA_MMR_LEAF_V1',
      mmr_node: 'ZAMA_MMR_NODE_V1',
    });
    expect(file.vectors.length).toBeGreaterThan(0);
    // Both leaf kinds are exercised, or the public-decrypt commitment would go unpinned.
    expect(file.vectors.some((vector) => vector.events.some((event) => event.kind === 'marked_public'))).toBe(true);
  });
});

describe('reconstructing an account history', () => {
  it.each(named)('%s: reproduces every leaf, the leaf count and the peaks', (_id, vector) => {
    const rebuilt = reconstructSolanaEncryptedValueAccount(unhex(vector.encrypted_value_account), eventsOf(vector));
    expect(rebuilt.leaves.map(rehex)).toEqual(vector.leaves);
    expect(rebuilt.leafCount).toBe(BigInt(vector.leaf_count));
    expect(rebuilt.peaks.map(rehex)).toEqual(vector.peaks);
  });

  it.each(named)('%s: builds the committed proof, and the proof verifies against the peaks', (_id, vector) => {
    const account = unhex(vector.encrypted_value_account);
    const rebuilt = reconstructSolanaEncryptedValueAccount(account, eventsOf(vector));
    const expected = proofOf(vector);

    const built = mmrBuildProof(rebuilt.leaves, expected.leafIndex);
    expect(built?.siblings.map(rehex)).toEqual(vector.proof.siblings);

    const commitment = rebuilt.leaves[Number(expected.leafIndex)]!;
    expect(mmrVerify(rebuilt.peaks, rebuilt.leafCount, commitment, expected)).toBe(true);

    // The proven leaf verifies under the authorization its kind grants, and under no other.
    const event = vector.events[Number(expected.leafIndex)]!;
    const handle = unhex(event.handle);
    if (event.kind === 'allowed') {
      expect(
        verifyHistoricalAccessProof(account, rebuilt.peaks, rebuilt.leafCount, handle, unhex(event.key), expected),
      ).toBe(true);
      expect(verifyPublicDecryptProof(account, rebuilt.peaks, rebuilt.leafCount, handle, expected)).toBe(false);
    } else {
      expect(verifyPublicDecryptProof(account, rebuilt.peaks, rebuilt.leafCount, handle, expected)).toBe(true);
      expect(
        verifyHistoricalAccessProof(account, rebuilt.peaks, rebuilt.leafCount, handle, new Uint8Array(32), expected),
      ).toBe(false);
    }
  });

  it('builds a verifying proof for every leaf of every vector', () => {
    for (const vector of file.vectors) {
      const rebuilt = reconstructSolanaEncryptedValueAccount(unhex(vector.encrypted_value_account), eventsOf(vector));
      for (const [index, leaf] of rebuilt.leaves.entries()) {
        const proof = mmrBuildProof(rebuilt.leaves, BigInt(index));
        expect(proof, `${vector.id}: leaf ${index}`).toBeDefined();
        expect(mmrVerify(rebuilt.peaks, rebuilt.leafCount, leaf, proof!), `${vector.id}: leaf ${index}`).toBe(true);
      }
    }
  });

  it('has no proof for a leaf index past the list', () => {
    expect(mmrBuildProof([], 0n)).toBeUndefined();
    expect(mmrBuildProof([new Uint8Array(32)], 1n)).toBeUndefined();
    expect(mmrBuildProof([new Uint8Array(32)], -1n)).toBeUndefined();
  });

  it('binds the account into every leaf', () => {
    const [first, second] = file.vectors.filter((vector) => vector.events.length === 4);
    expect(first).toBeDefined();
    expect(second).toBeDefined();
    expect(first!.events).toEqual(second!.events);
    expect(first!.encrypted_value_account).not.toBe(second!.encrypted_value_account);
    expect(first!.leaves).not.toEqual(second!.leaves);
  });
});

describe('the primitives, one at a time', () => {
  const account = new Uint8Array(32).fill(4);
  const handle = new Uint8Array(32).fill(5);
  const key = new Uint8Array(32).fill(6);

  it('separates the two leaf kinds by domain', () => {
    expect(historicalAccessLeafCommitment(account, 0n, handle, key)).not.toEqual(
      publicDecryptLeafCommitment(account, 0n, handle),
    );
  });

  it('binds the leaf index into the commitment', () => {
    expect(publicDecryptLeafCommitment(account, 0n, handle)).not.toEqual(
      publicDecryptLeafCommitment(account, 1n, handle),
    );
  });

  it('computes peaks the same way the proof builder walks them', () => {
    const leaves = Array.from({ length: 5 }, (_, i) => publicDecryptLeafCommitment(account, BigInt(i), handle));
    const peaks = mmrPeaksFromLeaves(leaves);
    // Five leaves: mountains of height 2 and 0.
    expect(peaks).toHaveLength(2);
    expect(peaks[1]).toEqual(mmrLeafNode(leaves[4]!));
    expect(peaks[0]).toEqual(
      mmrNode(
        mmrNode(mmrLeafNode(leaves[0]!), mmrLeafNode(leaves[1]!)),
        mmrNode(mmrLeafNode(leaves[2]!), mmrLeafNode(leaves[3]!)),
      ),
    );
  });

  it('refuses a proof that names another leaf count, an index out of range, or too many siblings', () => {
    const leaves = Array.from({ length: 3 }, (_, i) => publicDecryptLeafCommitment(account, BigInt(i), handle));
    const peaks = mmrPeaksFromLeaves(leaves);
    const proof = mmrBuildProof(leaves, 1n)!;

    expect(mmrVerify(peaks, 3n, leaves[1]!, proof)).toBe(true);
    expect(mmrVerify(peaks, 4n, leaves[1]!, proof)).toBe(false);
    expect(mmrVerify(peaks, 3n, leaves[0]!, { leafIndex: 5n, siblings: [] })).toBe(false);
    expect(
      mmrVerify(peaks, 3n, leaves[0]!, {
        leafIndex: 0n,
        siblings: Array.from({ length: MAX_MMR_SIBLINGS + 1 }, () => new Uint8Array(32)),
      }),
    ).toBe(false);
  });
});

describe('hexToBytes', () => {
  it('accepts unprefixed, 0x-prefixed, and 0X-prefixed hex', () => {
    expect(hexToBytes('00aF')).toEqual(new Uint8Array([0x00, 0xaf]));
    expect(hexToBytes('0x00aF')).toEqual(new Uint8Array([0x00, 0xaf]));
    expect(hexToBytes('0X00aF')).toEqual(new Uint8Array([0x00, 0xaf]));
  });

  it('round-trips bytes through the exported hex helpers', () => {
    const bytes = new Uint8Array([0x00, 0x12, 0xab, 0xff]);
    expect(hexToBytes(bytesToHex(bytes))).toEqual(bytes);
  });

  it('rejects malformed characters instead of coercing them to zero', () => {
    expect(() => hexToBytes('0xzz')).toThrow('hexToBytes: invalid hex string: 0xzz');
    expect(() => hexToBytes('0y00')).toThrow('hexToBytes: invalid hex string: 0y00');
  });

  it('rejects odd-length hex', () => {
    expect(() => hexToBytes('0x123')).toThrow('hexToBytes: odd-length hex string: 0x123');
  });
});

describe('buildPublicLeafProof', () => {
  // The proof handed to a consume step must verify against the peaks it was cross-checked with, and a
  // live account that disagrees with the expected history must fail here, naming the leaf count, not
  // later inside the on-chain verifier.
  const account = new Uint8Array(32).fill(0x0c);
  const handle = new Uint8Array(32).fill(0x92);
  const owner = new Uint8Array(32).fill(0x11);
  // What a burn writes, then an explicit re-seal: one allow, the public leaf, the public leaf again.
  const history: readonly SolanaEncryptedValueAccountEvent[] = [
    { kind: 'allowed', handle, key: owner },
    { kind: 'markedPublic', handle },
    { kind: 'markedPublic', handle },
  ];
  const live = reconstructSolanaEncryptedValueAccount(account, history);

  it('builds a proof of the requested public leaf that verifies against the live peaks', () => {
    const proof = buildPublicLeafProof(account, live, history, 1n);
    expect(proof.leafIndex).toBe(1n);
    expect(verifyPublicDecryptProof(account, live.peaks, live.leafCount, handle, proof)).toBe(true);
  });

  it('rejects a live account whose leaves disagree with the expected history', () => {
    const shorter = reconstructSolanaEncryptedValueAccount(account, history.slice(0, 2));
    expect(() => buildPublicLeafProof(account, shorter, history, 1n)).toThrow(
      /holds 2 leaves that do not match the 3-leaf history/,
    );
  });

  it('refuses to prove a leaf that is not public', () => {
    expect(() => buildPublicLeafProof(account, live, history, 0n)).toThrow(/not a public leaf/);
  });
});
