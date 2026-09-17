/**
 * The consensus probe: one small computation, then the question every test in
 * this directory ultimately asks — do all the operators hold the same bytes?
 *
 * This exists because of a rule the coverage inventory sets for the failure
 * matrix: *every cell's test ends in the consensus assertion*, not in "the
 * service came back". A restart that recovers into different bytes is the
 * failure these tests are for, and a liveness check would sail straight past
 * it. Rather than restate that assertion in every cell, each one calls this.
 *
 * It is deliberately cheap. The materialization gate is the thorough oracle
 * and takes ten minutes; a matrix of dozens of cells cannot pay that per cell,
 * so this does one add over two persisted operands and hands the result to the
 * shared comparator in `comparator.ts`.
 *
 * The comparison itself is NOT implemented here. It used to be, and it drifted:
 * `rows[0]` was compared against `rows[0]` across operators, which is a coin
 * toss for an aliased handle with several producing transactions, and a missing
 * SNS digest was excused as post-submission cleanup, which it is not.
 * `comparator.ts` owns the semantics and the failure classes; this file owns
 * the workload and the waiting.
 *
 * Fault injection is NOT done here either. Faults are injected host-side and
 * this probe is invoked between the steps.
 */
import { expect } from 'chai';

import {
  type AgreementReport,
  type ComparisonFields,
  type OperatorEvidence,
  ComparisonMismatch,
  FULL_COMPARISON,
  collectOperatorEvidence,
  compareOperatorEvidence,
} from './comparator';

export const PROBE_GAS_LIMIT = 10_000_000;

/**
 * The AliasFixture surface these suites use.
 *
 * `consumeCombined` is the only cross-BLOCK edge in the fixture, and the
 * failure and fork cases need it: a consumer gated on a producer's chain is the
 * only stage at which "a child waiting on work that then disappears" can be
 * constructed and observed.
 */
export interface ProbeContract {
  getAddress(): Promise<string>;
  waitForDeployment(): Promise<unknown>;
  produceInputs(overrides: { gasLimit: number }): Promise<{ wait(): Promise<unknown> }>;
  combineFromStorage(overrides: { gasLimit: number }): Promise<{ wait(): Promise<unknown> }>;
  combineFromStorageAgain(overrides: { gasLimit: number }): Promise<{ wait(): Promise<unknown> }>;
  combineLocal(overrides: { gasLimit: number }): Promise<{ wait(): Promise<unknown> }>;
  consumeCombined(overrides: { gasLimit: number }): Promise<{ wait(): Promise<unknown> }>;
  consumeExternal(handle: string, proof: string, overrides: { gasLimit: number }): Promise<{ wait(): Promise<unknown> }>;
  combined(): Promise<string>;
  combinedSecond(): Promise<string>;
  combinedLocal(): Promise<string>;
  consumed(): Promise<string>;
}

/**
 * Which operand shape the probe exercises.
 *
 * `boundary` is what every measurement so far used: two trivially encrypted
 * values persisted in an earlier transaction, then added as cross-transaction
 * boundaries. That is an unusual combination — trivially encrypted ciphertexts
 * are noiseless, and consuming them through the compressed boundary
 * representation is not what most traffic does. If a divergence depends on the
 * noise profile of its input, this is exactly where it would show and ordinary
 * traffic would not.
 *
 * `local` recomputes the same operands inside the consuming transaction, so
 * they are forwarded in memory and fold zero boundary bits. Same values, same
 * opcode, different provenance.
 */
export type ProbeShape = 'boundary' | 'local';

/**
 * Deploys the probe fixture and seeds its two operands.
 *
 * The operands are minted in their own transaction on purpose: the add that
 * follows then consumes them as persisted boundary values, which is the path
 * that actually crosses the materialization boundary. An add over two operands
 * minted in the same transaction would forward them in memory and never test
 * the compressed representation the operators must agree on.
 */
export async function deployProbe(owner: unknown): Promise<{ contract: ProbeContract; address: string }> {
  const { ethers } = await import('hardhat');
  const factory = await ethers.getContractFactory('AliasFixture');
  const contract = (await factory.connect(owner as never).deploy()) as unknown as ProbeContract;
  await contract.waitForDeployment();
  const address = await contract.getAddress();
  await (await contract.produceInputs({ gasLimit: PROBE_GAS_LIMIT })).wait();
  return { contract, address };
}

/** Mints one fresh handle and returns it. Repeatable: each call mints a new one. */
export async function mintProbeHandle(
  contract: ProbeContract,
  shape: ProbeShape = 'boundary',
): Promise<string> {
  if (shape === 'local') {
    await (await contract.combineLocal({ gasLimit: PROBE_GAS_LIMIT })).wait();
    return (await contract.combinedLocal()).toLowerCase();
  }
  await (await contract.combineFromStorage({ gasLimit: PROBE_GAS_LIMIT })).wait();
  return (await contract.combined()).toLowerCase();
}

/** Thrown when an operator never produced the evidence a comparison needs. */
export class EvidenceTimeout extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'EvidenceTimeout';
  }
}

export interface EvidenceWaitOptions {
  timeoutMs?: number;
  /** Wait for the durable SNS digest too. Default true; see the note below. */
  requireSnsDigest?: boolean;
  ciphertextVersion?: number;
}

/**
 * Waits for one operator to hold complete evidence for a handle.
 *
 * Absence is only meaningful after the deadline, so a probe that queried once
 * would report ordinary ingestion lag as a consensus failure — which after a
 * deliberately injected fault is exactly the wrong conclusion to jump to.
 *
 * The SNS digest is waited for rather than treated as optional. It lands after
 * the compute row, and it is durable: `transaction-sender` deletes the raw
 * squashed ciphertext from `ciphertexts128` after submission but leaves
 * `ciphertext_digest.ciphertext128` in place. So "not there yet" is a reason to
 * keep waiting and "still not there at the deadline" is missing evidence, not
 * cleanup.
 */
export async function waitForOperatorEvidence(
  databaseUrl: string,
  operator: number,
  handle: string,
  options: EvidenceWaitOptions = {},
): Promise<OperatorEvidence> {
  const timeoutMs = options.timeoutMs ?? 5 * 60_000;
  const requireSnsDigest = options.requireSnsDigest ?? true;
  const deadline = Date.now() + timeoutMs;
  let lastError: unknown;
  for (;;) {
    try {
      const evidence = await collectOperatorEvidence(databaseUrl, operator, handle, options.ciphertextVersion);
      if (!requireSnsDigest || evidence.snsDigest !== null) return evidence;
      lastError = new Error('the compute row is complete but the SNS digest has not been written yet');
    } catch (error) {
      // A classified mismatch about THIS operator's own rows (two values for
      // one handle, a duplicated storage row) is a finding, not a wait: more
      // patience cannot make it go away.
      if (error instanceof ComparisonMismatch && error.kind !== 'evidence-missing') throw error;
      lastError = error;
    }
    if (Date.now() >= deadline) {
      throw new EvidenceTimeout(
        `operator ${operator} never produced complete evidence for ${handle} within ` +
          `${Math.round(timeoutMs / 1000)}s: ${lastError instanceof Error ? lastError.message : String(lastError)}`,
      );
    }
    await new Promise((resolve) => setTimeout(resolve, 2_000));
  }
}

export interface AgreementOptions extends EvidenceWaitOptions {
  /** Which fields to compare. Defaults to everything. */
  fields?: ComparisonFields;
}

/**
 * The assertion itself: every named operator holds byte-identical ciphertext,
 * the same compute digest, the same durable SNS digest, the same key identity
 * and the same normalized provenance for the handle.
 *
 * The SNS digest is asserted rather than reported. Operators had been seen
 * agreeing on compute bytes while disagreeing on it; that turned out to be two
 * workers serving one operator queue on the test host -- a CPU container racing
 * a CUDA host worker, each writing its own backend's bytes. The harness now
 * refuses such a stack. With
 * that gone there is no reason to weaken the probe: the SNS digest is exactly
 * what a unanimous topology must agree on to reach quorum.
 *
 * It is not skipped when an operator is missing one. That was the previous
 * behaviour, justified by post-submission cleanup, and the justification was
 * wrong for this schema -- see the module note in `comparator.ts`.
 */
export async function assertOperatorsAgree(
  databaseUrls: readonly string[],
  operators: readonly number[],
  handle: string,
  options: AgreementOptions = {},
): Promise<AgreementReport> {
  const fields = options.fields ?? FULL_COMPARISON;
  const evidence = await Promise.all(
    operators.map((operator) =>
      waitForOperatorEvidence(databaseUrls[operator], operator, handle, {
        ...options,
        requireSnsDigest: options.requireSnsDigest ?? fields.snsDigest,
      }),
    ),
  );
  return compareOperatorEvidence(evidence, fields);
}

/**
 * Backwards-compatible single-row accessor for suites that only need one
 * operator's canonical value.
 */
export async function waitForOperatorRow(
  databaseUrl: string,
  handle: string,
  timeoutMs = 5 * 60_000,
  requireSnsDigest = false,
): Promise<OperatorEvidence> {
  return waitForOperatorEvidence(databaseUrl, 0, handle, { timeoutMs, requireSnsDigest });
}

/** Operator indexes 0..count-1, minus any the caller knows are deliberately down. */
export function operatorSet(count: number, excluded: number[] = []): number[] {
  return Array.from({ length: count }, (_, index) => index).filter((index) => !excluded.includes(index));
}

/**
 * Asserts a quorum outcome according to the case's declared mode.
 *
 * `not_checked` exists so a lower-layer case can say so out loud rather than
 * passing `0` into a flag named `expectQuorum` and leaving a reader to guess
 * whether absence of quorum was checked, expected, or ignored.
 */
export type QuorumMode = 'required' | 'forbidden' | 'not_checked';

export interface QuorumOutcome {
  mode: QuorumMode;
  senders: string[];
  detail: string;
}

export async function assertQuorumOutcome(options: {
  mode: QuorumMode;
  gatewayRpcUrl: string;
  ciphertextCommitsAddress: string;
  handle: string;
  /** Members authorized to submit, from the gateway itself. */
  authorizedSenders: readonly string[];
  threshold: number;
  timeoutMs?: number;
  label?: string;
}): Promise<QuorumOutcome> {
  const label = options.label ?? 'quorum';
  if (options.mode === 'not_checked') {
    return { mode: options.mode, senders: [], detail: `${label}: quorum deliberately not checked by this case` };
  }
  const { waitForConsensus } = await import('./helpers');
  const consensus = await waitForConsensus(
    options.gatewayRpcUrl,
    options.ciphertextCommitsAddress,
    options.handle,
    options.timeoutMs ?? 5 * 60_000,
  );

  if (options.mode === 'forbidden') {
    expect(
      consensus,
      `${label}: no consensus event may form for ${options.handle} while the topology cannot reach its ` +
        `threshold of ${options.threshold}`,
    ).to.be.null;
    return { mode: options.mode, senders: [], detail: `${label}: no quorum formed, as required` };
  }

  expect(consensus, `${label}: ${options.handle} must reach on-chain quorum`).to.not.be.null;
  const senders = consensus!.senders.map((sender) => sender.toLowerCase());
  const distinct = new Set(senders);
  expect(distinct.size, `${label}: quorum must come from distinct submitters`).to.eq(senders.length);

  // Distinct addresses are not enough: the claim is that the gateway's own
  // authorized members reached the configured threshold.
  const authorized = new Set(options.authorizedSenders.map((sender) => sender.toLowerCase()));
  const unauthorized = senders.filter((sender) => !authorized.has(sender));
  expect(
    unauthorized,
    `${label}: quorum includes submitter(s) the gateway does not list as coprocessors: ${unauthorized.join(', ')}`,
  ).to.have.length(0);
  expect(
    distinct.size,
    `${label}: quorum formed from ${distinct.size} authorized member(s), below the gateway's configured ` +
      `threshold of ${options.threshold}`,
  ).to.be.greaterThanOrEqual(options.threshold);

  return {
    mode: options.mode,
    senders,
    detail: `${label}: quorum from ${distinct.size} authorized member(s) (threshold ${options.threshold})`,
  };
}
