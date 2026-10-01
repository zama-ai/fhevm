// The leaf-proof wire of the coprocessors' leaf record, `POST /v1/solana/leaf-proofs`
// (`coprocessor/fhevm-engine/host-listener/openapi/solana_leaf_proofs.json`), pinned by
// `solana/test-fixtures/leaf-proofs/leaf_proofs_v1.json` as the listener and the KMS Connector are.
// The cleartext stack's server reads its queries and writes its answers here, from
// `createSolanaLeafRecord`, which the cleartext decrypt clients read directly.
import { getAddressDecoder, type Address } from '@solana/kit';
import { bytesToHexNo0x, hexToBytes } from '../../core/base/bytes.js';

////////////////////////////////////////////////////////////////////////////////

export const SOLANA_LEAF_PROOFS_PATH = '/v1/solana/leaf-proofs';

/** How many queries the leaf record answers in one read. */
export const SOLANA_MAX_LEAVES_PER_READ = 64;

/** A leaf the record is asked for: `key` allowed on `handle`, or `handle` made public without one. */
export type SolanaLeafQuery = {
  readonly encryptedStore: Address;
  readonly handle: Uint8Array;
  readonly key?: Address;
};

/** What the record says about one query. No answer is trusted: a proof is verified against the chain. */
export type SolanaLeafProofOutcome =
  /** `leafCount` is how many leaves the record had sealed when it built the proof. */
  | {
      readonly status: 'found';
      readonly leafIndex: bigint;
      readonly leafCount: bigint;
      readonly siblings: readonly Uint8Array[];
    }
  /** The record knows the store and has no such leaf in the `leafCount` leaves it has sealed. */
  | { readonly status: 'notFound'; readonly leafCount: bigint }
  /** The record has never seen the store. */
  | { readonly status: 'unknownAccount' }
  /** The record's history of the store contradicts the chain. */
  | { readonly status: 'historyIncomplete' };

/** One read of the leaf record: an outcome per query, in query order. */
export type SolanaLeafProofReader = (queries: readonly SolanaLeafQuery[]) => Promise<readonly SolanaLeafProofOutcome[]>;

////////////////////////////////////////////////////////////////////////////////

// As the listener parses it: `0x` optional.
const hex32 = (field: string, value: unknown): Uint8Array => {
  const digits = typeof value === 'string' && value.startsWith('0x') ? value.slice(2) : value;
  if (typeof digits !== 'string' || !/^[0-9a-fA-F]{64}$/.test(digits)) {
    throw new Error(`${field}: expected 32 bytes as hex`);
  }
  return hexToBytes(`0x${digits}`);
};

export function decodeSolanaLeafQuery(value: unknown): SolanaLeafQuery {
  const { encryptedStore, handle, kind, key } = (value ?? {}) as Record<string, unknown>;
  const query = {
    encryptedStore: getAddressDecoder().decode(hex32('encryptedStore', encryptedStore)),
    handle: hex32('handle', handle),
  };
  if (kind === 'public' && (key === undefined || key === null)) return query;
  if (kind === 'allowed') return { ...query, key: getAddressDecoder().decode(hex32('key', key)) };
  throw new Error('kind: expected public without a key, or allowed with one');
}

export function encodeSolanaLeafProofOutcome(outcome: SolanaLeafProofOutcome): Record<string, unknown> {
  switch (outcome.status) {
    case 'found':
      return {
        status: 'found',
        leafIndex: Number(outcome.leafIndex),
        leafCount: Number(outcome.leafCount),
        siblings: outcome.siblings.map((sibling) => bytesToHexNo0x(sibling)),
      };
    case 'notFound':
      return { status: 'notFound', leafCount: Number(outcome.leafCount) };
    case 'unknownAccount':
    case 'historyIncomplete':
      return { status: outcome.status };
  }
}
