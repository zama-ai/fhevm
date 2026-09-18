import { emitAssertions } from './assertionEvidence';
/**
 * RFC-020's crash-retry clause, and what it takes to actually establish it.
 *
 *   "a crash or retry re-executes the whole transaction batch; determinism
 *    makes duplicate execution byte-identical and first-write-wins makes it
 *    harmless."
 *
 * Every other consensus suite here measures one execution per operator. This
 * one measures an operator that executed the same work twice, because its
 * worker was interrupted partway through and came back to work it had already
 * started.
 *
 * The previous version of this suite could pass without any of that happening.
 * Its host half watched `COUNT(*) FROM computations WHERE is_completed = false`
 * and fired at the first non-zero reading -- a count that includes fixture
 * setup and unrelated traffic -- so the kill could land before the victim had
 * touched anything this file later compared. Comparing the outputs afterwards
 * then established ordinary execution after a restart, which is a liveness
 * check.
 *
 * What makes the claim checkable is that acquisition is visible in the schema.
 * `tfhe-worker` takes a dependence chain by setting `worker_id`,
 * `status = 'processing'` and a lease (`dependence_chain.rs`,
 * `acquire_lock`), and `worker_id` is a fresh UUID per PROCESS
 * (`daemon_cli.rs`: "If not provided, a random UUID will be generated"). So:
 *
 *   * "the victim had acquired the target work" is `status = 'processing'` with
 *     a non-null `worker_id` on the chain that owns the target transaction's
 *     computations, while those computations are still incomplete;
 *   * "the same work was retried by a different claimant" is that same chain
 *     acquired again under a DIFFERENT `worker_id` -- which after a crash is
 *     necessarily the restarted process, reached through the `expired_lock`
 *     arm of the acquisition query.
 *
 * This half publishes the target identifiers, waits for the orchestrator to
 * acknowledge that it interrupted the victim while it held them, and then
 * asserts the clause. The orchestration lives in
 * `run-crash-retry-consensus.sh`, because interrupting a process needs host
 * access this container should not have.
 *
 * Environment:
 *   RUN_CRASH_RETRY_CONSENSUS=1   opt in
 *   COPROCESSOR_COUNT             fleet size, default 3
 *   CRASH_VICTIM_OPERATOR         which operator the runner interrupts
 *   CRASH_RETRY_HANDLES           how many handles to mint
 *   CRASH_BOUNDARY                before-commit | after-commit | expired-lease
 */
import { expect } from 'chai';
import { INTERRUPTED_WORKER_LOCKS_SQL } from './faultEvidence';

import { type CrashTarget, publishHandshake, waitForFaultAcknowledgement } from './handshake';
import {
  requireQuorumConfiguration,
  getCoprocessorDbUrls,
  queryCanonicalOutputs,
  queryTransactionCompletion,
  readGatewayMembership,
  waitForDatabaseReadiness,
} from './helpers';
import {
  type ProbeContract,
  assertOperatorsAgree,
  assertQuorumOutcome,
  deployProbe,
  operatorSet,
} from './probe';
import { assertRunValidity } from './validity';

const ENABLE = process.env.RUN_CRASH_RETRY_CONSENSUS === '1';
const COPROCESSOR_COUNT = Number.parseInt(process.env.COPROCESSOR_COUNT ?? '3', 10);
const VICTIM = Number.parseInt(process.env.CRASH_VICTIM_OPERATOR ?? '1', 10);
const HANDLE_COUNT = Number.parseInt(process.env.CRASH_RETRY_HANDLES ?? '2', 10);
const BOUNDARY = (process.env.CRASH_BOUNDARY ?? 'before-commit') as CrashTarget['boundary'];
const GATEWAY_RPC_URL = process.env.GATEWAY_RPC_URL ?? '';
const GATEWAY_CONFIG_ADDRESS = process.env.GATEWAY_CONFIG_ADDRESS ?? '';
const CIPHERTEXT_COMMITS_ADDRESS = process.env.CIPHERTEXT_COMMITS_ADDRESS ?? '';
const PROBE_GAS_LIMIT = 10_000_000;

/** Printed only when the case's own assertions have all run. */
const MARKER = '[crash-retry] CASE COMPLETE';

interface StrandedChain {
  chain: string;
  dependencyCount: number;
}

/**
 * Chains left in the state the repair path itself calls stranded: the gate is
 * closed, nobody owns the chain, and no unprocessed producer still names it a
 * dependent -- so nothing left in the system will ever decrement it.
 */
async function strandedChains(databaseUrl: string): Promise<StrandedChain[]> {
  const { Pool } = await import('pg');
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const result = await pool.query<{ chain: string; dependency_count: number }>(
      `SELECT encode(child.dependence_chain_id, 'hex') AS chain, child.dependency_count
         FROM dependence_chain AS child
        WHERE child.dependency_count > 0
          AND child.worker_id IS NULL
          AND NOT EXISTS (
                SELECT 1 FROM dependence_chain AS producer
                 WHERE producer.dependents @> ARRAY[child.dependence_chain_id]
                   AND producer.status <> 'processed')`,
    );
    return result.rows.map((row) => ({ chain: row.chain, dependencyCount: row.dependency_count }));
  } finally {
    await pool.end();
  }
}

/** Chains still owned by the process the host proved it interrupted. */
async function orphanedLocks(databaseUrl: string, interruptedWorkerId: string): Promise<string[]> {
  const { Pool } = await import('pg');
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const result = await pool.query<{ chain: string }>(
      INTERRUPTED_WORKER_LOCKS_SQL, [interruptedWorkerId],
    );
    return result.rows.map((row) => row.chain);
  } finally {
    await pool.end();
  }
}

describe('Crash-retry byte identity (RFC-020)', function () {
  this.timeout(40 * 60_000);

  let databaseUrls: string[] = [];
  let contract: ProbeContract;
  let contractAddress: string;

  before(async function () {
    if (!ENABLE) this.skip();
    requireQuorumConfiguration('crash-retry', GATEWAY_RPC_URL, GATEWAY_CONFIG_ADDRESS, CIPHERTEXT_COMMITS_ADDRESS);
    if (VICTIM < 0 || VICTIM >= COPROCESSOR_COUNT) {
      throw new Error(`CRASH_VICTIM_OPERATOR ${VICTIM} is outside the topology`);
    }
    databaseUrls = getCoprocessorDbUrls(COPROCESSOR_COUNT);
    await waitForDatabaseReadiness(databaseUrls);

    // The deferred gate is skipped here on purpose: this suite runs while the
    // orchestrator is about to interrupt a worker mid-batch, so a scheduler
    // with work in flight is the point rather than a fault.
    console.info(
      `[crash-retry] validity gates: ${await assertRunValidity({
        databaseUrls,
        rpcUrl: process.env.RPC_URL,
        checkDeferred: false,
      })}`,
    );

    const { getSigners, initSigners } = await import('../signers');
    await initSigners(2);
    const signers = await getSigners();
    const deployment = await deployProbe(signers.alice);
    contract = deployment.contract;
    contractAddress = deployment.address;
  });

  it('recovers the interrupted work itself, byte-identically, and leaves nothing stranded', async function () {
    const operators = operatorSet(COPROCESSOR_COUNT);
    const uninterrupted = operators.filter((operator) => operator !== VICTIM);
    expect(uninterrupted.length, 'this case needs at least one operator that is never interrupted').to.be.greaterThan(
      0,
    );

    // Two independent producers plus one dependent are sufficient here. The
    // host holds the selected boundary deterministically; extra identical
    // transactions add runtime without another recovery assertion.
    const handles: string[] = [];
    const transactionHashes: string[] = [];
    for (let index = 0; index < HANDLE_COUNT; index += 1) {
      const sent = await contract.combineFromStorage({ gasLimit: PROBE_GAS_LIMIT });
      const receipt = (await (sent as unknown as { wait(): Promise<{ hash: string } | null> }).wait()) as {
        hash: string;
      } | null;
      if (!receipt) throw new Error('a probe transaction produced no receipt');
      transactionHashes.push(receipt.hash.toLowerCase());
      handles.push((await contract.combined()).toLowerCase());
    }
    // A later block creates a dependent whose gate must survive the producer's crash.
    const childTx = await contract.consumeCombined({ gasLimit: PROBE_GAS_LIMIT });
    const childReceipt = await childTx.wait() as { hash: string };
    transactionHashes.push(childReceipt.hash.toLowerCase());
    handles.push((await contract.consumed()).toLowerCase());

    // Handles alias when two transactions have identical sourcing, so the
    // handle list can be shorter than the transaction list. Both are published:
    // the runner aims at transactions, this suite compares handles.
    const distinctHandles = [...new Set(handles)];
    console.info(
      `[crash-retry] minted ${transactionHashes.length} transaction(s) producing ` +
        `${distinctHandles.length} distinct handle(s); victim is operator ${VICTIM}, boundary ${BOUNDARY}`,
    );

    const target: CrashTarget = {
      victimOperator: VICTIM,
      transactionHashes,
      handles: distinctHandles,
      contractAddress,
      boundary: BOUNDARY,
      publishedAt: new Date().toISOString(),
    };
    publishHandshake('crash-target', target);

    // The orchestrator now has to interrupt the victim while it holds this
    // work. Until it says it did, this case has nothing to assert: a run where
    // the fault never landed is invalid, not passed.
    const acknowledgement = await waitForFaultAcknowledgement('crash-fault', 15 * 60_000);
    expect(
      acknowledgement.applied,
      `the orchestrator could not interrupt operator ${VICTIM} while it held the target work: ` +
        `${acknowledgement.detail}. Without that, this case measures ordinary execution`,
    ).to.eq(true);
    console.info(
      `[crash-retry] fault evidence: ${acknowledgement.detail}\n` +
        `  process before: ${acknowledgement.processBefore}\n` +
        `  process after:  ${acknowledgement.processAfter}\n` +
        `  chain:          ${acknowledgement.dependenceChainId ?? 'n/a'} ` +
        `held by ${acknowledgement.workerIdBefore ?? 'n/a'}`,
    );

    expect(acknowledgement.workerIdBefore).to.be.a('string').and.not.empty;
    expect(acknowledgement.workerIdAfter).to.be.a('string').and.not.empty;
    expect(acknowledgement.workerIdAfter).not.to.eq(acknowledgement.workerIdBefore);
    // Re-read the committed acquisition audit rather than trusting a process
    // restart or an already-processed status as evidence of a different claimant.
    const { Pool } = await import('pg');
    const claimPool = new Pool({ connectionString: databaseUrls[VICTIM], max: 1 });
    const claims = await claimPool.query(
      `SELECT worker_id::text FROM public.consensus_test_claims
       WHERE dependence_chain_id = decode($1, 'hex') AND worker_id = $2::uuid
         AND claimed_at >= $3::timestamptz`,
      [acknowledgement.dependenceChainId, acknowledgement.workerIdAfter, acknowledgement.faultObservedAt],
    ).finally(() => claimPool.end());
    expect(claims.rows.length, 'the same chain must have a committed claim by the replacement worker').to.be.greaterThan(0);

    // --- the clause itself ------------------------------------------------
    // Determinism: the victim's second execution of the SAME work has to agree
    // with operators that executed it once.
    for (const handle of distinctHandles) {
      const report = await assertOperatorsAgree(databaseUrls, operators, handle, { timeoutMs: 12 * 60_000 });
      expect(report.operators, 'the victim must be in the comparison').to.include(VICTIM);
    }
    console.info(
      `[crash-retry] all ${distinctHandles.length} handle(s) agree fleet-wide, victim included, ` +
        'after the interruption',
    );

    // Completion: a retry that gave up would leave incomplete or errored rows
    // behind, and byte agreement over the handles that did finish would hide
    // it. Scoped to the target transactions rather than to the whole database.
    for (const operator of operators) {
      const databaseUrl = databaseUrls[operator];
      const rowsForScope = await queryCanonicalOutputs(databaseUrl, distinctHandles);
      const scopes = [
        ...new Map(
          rowsForScope.map((row) => [
            `0x${row.transactionId.toString('hex')}:${row.hostChainId}:${row.blockNumber}`,
            { transactionId: row.transactionId, hostChainId: row.hostChainId, blockNumber: row.blockNumber },
          ]),
        ).values(),
      ];
      const completions = await queryTransactionCompletion(databaseUrl, scopes);
      expect(completions.length, `operator ${operator} reported no producing transactions`).to.be.greaterThan(0);
      for (const completion of completions) {
        expect(
          completion.errorCount,
          `operator ${operator} left errored computations in transaction ` +
            `0x${completion.transactionId.toString('hex')} after the interruption`,
        ).to.eq(0);
        expect(
          completion.completedCount,
          `operator ${operator} left transaction 0x${completion.transactionId.toString('hex')} ` +
            'incompletely executed after the interruption',
        ).to.eq(completion.totalCount);
      }
    }

    // First-write-wins is meant to be silent. Two executions of the same work
    // must leave one storage row, not a duplicate and not an error surfaced to
    // the worker. `assertOperatorsAgree` already asserts storage-row
    // uniqueness through the comparator; this states it per handle so a
    // failure names the handle.
    const { queryStorageRowCount } = await import('./comparator');
    for (const operator of operators) {
      for (const handle of distinctHandles) {
        expect(
          await queryStorageRowCount(databaseUrls[operator], handle),
          `operator ${operator} holds more than one storage row for ${handle}; duplicate execution must ` +
            'leave exactly one',
        ).to.eq(1);
      }
    }
    console.info('[crash-retry] first-write-wins held: one storage row per handle on every operator');

    // Nothing may be left stranded or still owned by the interrupted process.
    // Production can reclaim a lease as soon as it expires; a five-minute
    // age filter would hide locks abandoned by the very crash under test.
    const stranded = await strandedChains(databaseUrls[VICTIM]);
    expect(
      stranded.map((entry) => `${entry.chain}(count ${entry.dependencyCount})`),
      `operator ${VICTIM} has chain(s) whose gate can never be decremented after the interruption`,
    ).to.have.length(0);
    const orphaned = await orphanedLocks(databaseUrls[VICTIM], acknowledgement.workerIdBefore!);
    expect(
      orphaned,
      `operator ${VICTIM} still has chain(s) owned by interrupted worker ${acknowledgement.workerIdBefore}`,
    ).to.have.length(0);

    // And the fleet still commits: the gateway's own membership and threshold,
    // read from the running gateway rather than from the scenario file.
    const membership = await readGatewayMembership(GATEWAY_RPC_URL, GATEWAY_CONFIG_ADDRESS);
    const outcome = await assertQuorumOutcome({
      mode: 'required',
      gatewayRpcUrl: GATEWAY_RPC_URL,
      ciphertextCommitsAddress: CIPHERTEXT_COMMITS_ADDRESS,
      handle: distinctHandles[0],
      authorizedSenders: membership.txSenders,
      threshold: membership.threshold,
      label: 'crash-retry',
    });
    console.info(`[crash-retry] ${outcome.detail}`);

    const receiptCases = [BOUNDARY === 'before-commit' ? 'CR-01-INTERRUPT-BEFORE-COMMIT' : BOUNDARY === 'after-commit' ? 'CR-02-INTERRUPT-AFTER-COMMIT' : 'CR-03-EXPIRED-LEASE-RECLAIM', process.env.CONSENSUS_ASSERTION_CASE_ID].filter((value): value is string => Boolean(value));
    for (const caseId of receiptCases) emitAssertions(caseId, ['precondition', 'liveness', 'bytes', 'safety', 'quorum'], 'Exact boundary acknowledgement and new-worker claim were verified; original transactions completed without errors, matched fleet bytes, retained unique storage and left no abandoned locks/gates.');
    if (BOUNDARY === 'before-commit') emitAssertions('REG-03-SUPERVISED-DAEMON-RECOVERY', ['liveness'], 'The supervisor replacement committed a new claim and completed the same identified pending work.');
    console.info(MARKER);
  });
});
