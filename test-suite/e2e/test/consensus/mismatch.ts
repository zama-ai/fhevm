/**
 * One classified failure vocabulary for every consensus comparison.
 *
 * This lives in its own module so both oracles can raise it. There are two of
 * them, deliberately: `comparator.ts` compares operators against each other for
 * one handle, and `helpers.ts` verifies the digest bindings and gateway
 * commitments of a whole fixture run. A canary has to be able to say WHICH
 * comparison rejected the poisoned fleet and HOW -- a canary that accepts
 * "something threw" counts a database outage or a timeout as a successfully
 * detected divergence, which is the hollow reassurance the canary rule exists
 * to prevent.
 */

/** Which comparison failed. Callers and canaries assert on these. */
export type MismatchKind =
  | 'raw-bytes'
  | 'type-version'
  | 'compute-digest'
  | 'sns-digest'
  | 'sns-evidence-missing'
  | 'compute-digest-missing'
  | 'key-identity'
  | 'operation'
  | 'provenance'
  | 'ciphertext128-format'
  | 'value-multiplicity'
  | 'storage-row-uniqueness'
  | 'evidence-missing'
  | 'gateway-commitment';

/**
 * A classified comparison failure.
 *
 * The class matters as much as the failure: it is what lets a canary require
 * the rejection it engineered rather than any rejection at all.
 */
export class ComparisonMismatch extends Error {
  readonly kind: MismatchKind;
  readonly handle: string;
  readonly operators: number[];

  constructor(kind: MismatchKind, handle: string, operators: number[], message: string) {
    super(`consensus mismatch [${kind}] on ${handle} (operators ${operators.join(',')}): ${message}`);
    this.name = 'ComparisonMismatch';
    this.kind = kind;
    this.handle = handle;
    this.operators = operators;
  }
}
