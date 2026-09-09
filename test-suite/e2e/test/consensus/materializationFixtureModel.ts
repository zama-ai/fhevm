/**
 * Stable description of the materialization-consensus workload.
 *
 * Keep this model free of RPC, database, and SDK dependencies.  The workload
 * driver, the database adapter, and the homogeneous-byte oracle all consume
 * this one description, so adding a handle cannot silently leave one oracle
 * behind.
 */
export const FIXTURE_HANDLE_LABELS = [
  'stageZero',
  'inputA',
  'inputB',
  'trivialOne',
  'inputAIsZero',
  'selected',
  'sum',
  'difference',
  'independentInput',
  'independentBias',
  'independent',
  'terminal',
] as const;

export type FixtureHandleLabel = (typeof FIXTURE_HANDLE_LABELS)[number];

export type FixtureHandles = Readonly<Record<FixtureHandleLabel, string>>;

/**
 * The transaction is the materialization boundary, so the fixture's oracle is
 * organised by transaction: every produced output of one transaction must
 * carry the same `transaction_id` provenance, whole transactions must be
 * completely executed, and cross-transaction consumers (deriveFromAAndB
 * reading inputA; consumeFanout reading sum and difference) always read the
 * producer's persisted canonical bytes.
 */
export const FIXTURE_TRANSACTIONS = [
  {
    name: 'stage-input-a',
    block: 'same' as const,
    /** FHE.asEuint64 + FHE.add; VerifyInput is not a computation row. */
    exactComputationCount: 2,
    producedLabels: ['stageZero', 'inputA'],
    labels: ['stageZero', 'inputA'],
  },
  {
    name: 'derive-from-a-and-b',
    block: 'same' as const,
    /** TrivialEncrypt, eq, select and the select fan-out into add and sub. */
    exactComputationCount: 5,
    producedLabels: ['trivialOne', 'inputAIsZero', 'selected', 'sum', 'difference'],
    labels: ['inputB', 'trivialOne', 'inputAIsZero', 'selected', 'sum', 'difference'],
  },
  {
    name: 'run-independent',
    block: 'same' as const,
    /** FHE.asEuint64 + FHE.add; VerifyInput is an external boundary. */
    exactComputationCount: 2,
    producedLabels: ['independentBias', 'independent'],
    labels: ['independentInput', 'independentBias', 'independent'],
  },
  {
    name: 'next-block-terminal',
    block: 'terminal' as const,
    exactComputationCount: 1,
    producedLabels: ['terminal'],
    labels: ['terminal'],
  },
] as const satisfies ReadonlyArray<{
  readonly name: string;
  readonly block: 'same' | 'terminal';
  readonly exactComputationCount: number;
  readonly producedLabels: readonly FixtureHandleLabel[];
  readonly labels: readonly FixtureHandleLabel[];
}>;

/**
 * Only VerifyInput-only handles stay in the plaintext oracle.  Every
 * materialized TrivialEncrypt is a produced computation row, so it belongs in
 * the persisted-byte/digest/provenance oracle just like an add or ITE output.
 * Excluding it would hide a real same-SW/same-HW divergence.
 */
export const FIXTURE_PRODUCED_OUTPUT_LABELS = FIXTURE_TRANSACTIONS.flatMap(
  (transaction) => transaction.producedLabels,
) as readonly FixtureHandleLabel[];

/** Plaintext oracle shared by CPU and GPU runs; ciphertext bytes are not. */
export const FIXTURE_EXPECTED_PLAINTEXTS = {
  stageZero: 0n,
  inputA: 0n,
  inputB: 9n,
  trivialOne: 1n,
  inputAIsZero: true,
  selected: 9n,
  sum: 18n,
  difference: 0n,
  independentInput: 23n,
  independentBias: 7n,
  independent: 30n,
  terminal: 18n,
} as const satisfies Readonly<Record<FixtureHandleLabel, bigint | boolean>>;
