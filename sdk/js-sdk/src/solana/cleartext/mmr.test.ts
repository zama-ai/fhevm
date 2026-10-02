// The MMR primitives, pinned to the normative leaf vectors.
//
// `solana/test-fixtures/leaves/leaves_v1.json` is written by the Rust shared crate the host program,
// the coprocessor and the KMS connector all run (`bash solana/scripts/update-leaf-vectors.sh`), and
// read here. Every vector is one account history: its events, and the leaves, peaks and one proof
// they imply. Reproducing all of them is what makes this module the same MMR rather than one that
// happens to agree on a hand-picked case.

import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { bytesToHex, hexToBytes } from '../../core/base/bytes.js';
import {
  createRetainedMmr,
  historicalAccessLeafCommitment,
  mmrLeafNode,
  mmrNode,
  mmrVerify,
  publicDecryptLeafCommitment,
  storeLeafCommitment,
  verifyHistoricalAccessProof,
  verifyPublicDecryptProof,
  type MmrProof,
  type RetainedMmr,
  type SolanaStoreHistoryEvent,
} from './mmr.js';

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
  readonly encrypted_store_account: string;
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
  readFileSync(new URL('../../../../../solana/test-fixtures/leaves/leaves_v1.json', import.meta.url), 'utf8'),
) as LeafVectorFile;

const unhex = (hex: string): Uint8Array => hexToBytes(`0x${hex}`);
const rehex = (bytes: Uint8Array): string => bytesToHex(bytes).slice(2);

const eventsOf = (vector: LeafVector): readonly SolanaStoreHistoryEvent[] =>
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

/** A tree over `leaves`, in order. */
const treeOf = (leaves: readonly Uint8Array[]): RetainedMmr => {
  const tree = createRetainedMmr();
  leaves.forEach((leaf) => {
    tree.append(leaf);
  });
  return tree;
};

/** The vector's leaves, as the host appends them, and the tree over them. */
const rebuild = (vector: LeafVector) => {
  const leaves = eventsOf(vector).map((event, index) =>
    storeLeafCommitment(unhex(vector.encrypted_store_account), BigInt(index), event),
  );
  const tree = treeOf(leaves);
  return { leaves, tree, leafCount: BigInt(leaves.length), peaks: tree.peaks(leaves.length) };
};

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
    const rebuilt = rebuild(vector);
    expect(rebuilt.leaves.map(rehex)).toEqual(vector.leaves);
    expect(rebuilt.leafCount).toBe(BigInt(vector.leaf_count));
    expect(rebuilt.peaks.map(rehex)).toEqual(vector.peaks);
  });

  it.each(named)('%s: builds the committed proof, and the proof verifies against the peaks', (_id, vector) => {
    const account = unhex(vector.encrypted_store_account);
    const rebuilt = rebuild(vector);
    const expected = proofOf(vector);

    const built = rebuilt.tree.proof(Number(expected.leafIndex), rebuilt.leaves.length);
    expect(built.siblings.map(rehex)).toEqual(vector.proof.siblings);

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
      const rebuilt = rebuild(vector);
      for (const [index, leaf] of rebuilt.leaves.entries()) {
        const proof = rebuilt.tree.proof(index, rebuilt.leaves.length);
        expect(mmrVerify(rebuilt.peaks, rebuilt.leafCount, leaf, proof), `${vector.id}: leaf ${index}`).toBe(true);
      }
    }
  });

  it('binds the account into every leaf', () => {
    const [first, second] = file.vectors.filter((vector) => vector.events.length === 4);
    expect(first).toBeDefined();
    expect(second).toBeDefined();
    expect(first!.events).toEqual(second!.events);
    expect(first!.encrypted_store_account).not.toBe(second!.encrypted_store_account);
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

  it('builds one peak per complete mountain, oldest first', () => {
    const leaves = Array.from({ length: 5 }, (_, i) => publicDecryptLeafCommitment(account, BigInt(i), handle));
    const peaks = treeOf(leaves).peaks(5);
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
    const tree = treeOf(leaves);
    const peaks = tree.peaks(3);
    const proof = tree.proof(1, 3);

    expect(mmrVerify(peaks, 3n, leaves[1]!, proof)).toBe(true);
    expect(mmrVerify(peaks, 4n, leaves[1]!, proof)).toBe(false);
    expect(mmrVerify(peaks, 3n, leaves[0]!, { leafIndex: 5n, siblings: [] })).toBe(false);
    expect(
      mmrVerify(peaks, 3n, leaves[0]!, {
        leafIndex: 0n,
        siblings: Array.from({ length: 65 }, () => new Uint8Array(32)),
      }),
    ).toBe(false);
  });
});

describe('createRetainedMmr', () => {
  const account = new Uint8Array(32).fill(4);
  const leaves = Array.from({ length: 70 }, (_, i) =>
    publicDecryptLeafCommitment(account, BigInt(i), new Uint8Array(32).fill(i)),
  );

  // Every count up to 70 crosses mountains of heights 0 to 6, and the tree keeps growing past each
  // count it is asked about, as the leaf record's does past the account it checks.
  it('answers the peaks a tree over only the first leaves has, and proofs against them, at every past count', () => {
    const tree = treeOf(leaves);
    expect(tree.leafCount()).toBe(70);
    for (let count = 0; count <= leaves.length; count += 1) {
      const peaks = treeOf(leaves.slice(0, count)).peaks(count);
      expect(tree.peaks(count)).toEqual(peaks);
      for (let index = 0; index < count; index += 1) {
        expect(mmrVerify(peaks, BigInt(count), leaves[index]!, tree.proof(index, count))).toBe(true);
      }
    }
  });

  it('refuses a count it does not hold and a leaf outside the count', () => {
    const tree = treeOf(leaves.slice(0, 3));
    expect(() => tree.peaks(4)).toThrow(/holds 3 leaves, not 4/);
    expect(() => tree.proof(0, 4)).toThrow(/holds 3 leaves, not 4/);
    expect(() => tree.proof(2, 2)).toThrow(/leaf 2 is not among the first 2/);
  });
});
