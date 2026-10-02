// What the cleartext client asks of a leaf record (`createSolanaLeafRecord`), and what the record
// answers: the outcomes the coprocessors' leaf-proof endpoint gives the KMS Connector.
import type { Address } from '@solana/kit';

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
