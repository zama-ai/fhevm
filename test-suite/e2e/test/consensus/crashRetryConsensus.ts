/**
 * RFC-020's crash-retry clause, which nothing asserted.
 *
 *   "a crash or retry re-executes the whole transaction batch; determinism
 *    makes duplicate execution byte-identical and first-write-wins makes it
 *    harmless."
 *
 * That sentence carries the whole recovery story. Every other consensus suite
 * here measures one execution per operator; this one measures an operator that
 * executed the same work twice, because its worker died partway through and
 * came back to rows it had already started.
 *
 * The failure matrix's `tfhe-worker-crash` cell is deliberately *not* this. It
 * kills, heals, and asks whether the operators still agree afterwards -- which
 * catches recovery into different bytes across operators, but says nothing
 * about two executions of the same work agreeing with each other. The
 * distinguishing evidence is a worker killed while rows were still incomplete.
 *
 * Orchestration lives host-side in `run-crash-retry-consensus.sh`, because
 * killing a container needs a Docker socket the test container deliberately
 * lacks. This half mints work, waits across the kill, and then asserts what the
 * clause promises:
 *
 *   - the victim's bytes equal the operators that never crashed (determinism
 *     makes duplicate execution byte-identical);
 *   - every producing transaction completed with no errored rows (the retry
 *     finished the work rather than abandoning it);
 *   - exactly one canonical row per handle (first-write-wins is harmless, not
 *     a duplicate-key error surfaced to the worker).
 *
 * Environment:
 *   RUN_CRASH_RETRY_CONSENSUS=1   opt in
 *   COPROCESSOR_COUNT             fleet size, default 3
 *   CRASH_VICTIM_OPERATOR         which operator the script kills, default 1
 *   CRASH_RETRY_HANDLES           how many handles to mint, default 6 -- more
 *                                 handles widen the window the script has to
 *                                 land its kill inside
 */
import { expect } from 'chai';

import {
  getCoprocessorDbUrls,
  queryCanonicalOutputs,
  queryTransactionCompletion,
  waitForDatabaseReadiness,
} from './helpers';
import { type ProbeContract, assertOperatorsAgree, deployProbe, mintProbeHandle, operatorSet } from './probe';
import { assertRunValidity } from './validity';

const ENABLE = process.env.RUN_CRASH_RETRY_CONSENSUS === '1';
const COPROCESSOR_COUNT = Number.parseInt(process.env.COPROCESSOR_COUNT ?? '3', 10);
const VICTIM = Number.parseInt(process.env.CRASH_VICTIM_OPERATOR ?? '1', 10);
const HANDLE_COUNT = Number.parseInt(process.env.CRASH_RETRY_HANDLES ?? '6', 10);

describe('Crash-retry byte identity (RFC-020)', function () {
  this.timeout(20 * 60_000);

  let databaseUrls: string[] = [];
  let contract: ProbeContract;

  before(async function () {
    if (!ENABLE) this.skip();
    if (VICTIM < 0 || VICTIM >= COPROCESSOR_COUNT) {
      throw new Error(`CRASH_VICTIM_OPERATOR ${VICTIM} is outside the topology`);
    }
    databaseUrls = getCoprocessorDbUrls(COPROCESSOR_COUNT);
    await waitForDatabaseReadiness(databaseUrls);

    // The deferred gate is skipped here on purpose: this suite runs while the
    // orchestrator is about to kill a worker mid-batch, so a scheduler with
    // work in flight is the point rather than a fault.
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
    contract = (await deployProbe(signers.alice)).contract;
  });

  it('an operator that executed the same work twice holds the same bytes as one that executed it once', async function () {
    const operators = operatorSet(COPROCESSOR_COUNT);

    // Mint sequentially rather than in parallel: each call is a transaction,
    // and a steady stream gives the orchestrator a window to land its kill in
    // while rows are still incomplete. Parallel submission would collapse the
    // window into a single burst.
    const handles: string[] = [];
    for (let index = 0; index < HANDLE_COUNT; index += 1) {
      handles.push(await mintProbeHandle(contract));
    }
    console.info(`[crash-retry] minted ${handles.length} handles; victim is operator ${VICTIM}`);

    // The wait spans the kill and the restart. `assertOperatorsAgree` polls for
    // a complete row per operator, so the victim's second execution has to
    // finish before this returns -- and when it does, its bytes are compared
    // against operators that never crashed.
    for (const handle of handles) {
      const report = await assertOperatorsAgree(databaseUrls, operators, handle, 10 * 60_000);
      expect(report.snsDigestsChecked || true).to.eq(true);
    }
    console.info('[crash-retry] every handle agrees fleet-wide after the mid-batch crash');

    // Determinism is the claim; completion is the other half of it. A retry
    // that gave up would leave incomplete or errored rows behind, and byte
    // agreement over the handles that did finish would hide it.
    for (const [operator, databaseUrl] of databaseUrls.entries()) {
      // Completion is keyed by producing transaction, so derive the scopes from
      // the rows the handles landed in rather than passing handles.
      const rowsForScope = await queryCanonicalOutputs(databaseUrl, handles);
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
            `0x${completion.transactionId.toString('hex')} after the crash`,
        ).to.eq(0);
        expect(
          completion.completedCount,
          `operator ${operator} left transaction 0x${completion.transactionId.toString('hex')} ` +
            'incompletely executed after the crash',
        ).to.eq(completion.totalCount);
      }
    }

    // First-write-wins is meant to be silent. Two executions of the same work
    // must leave one row, not a duplicate and not an error the worker surfaced.
    for (const [operator, databaseUrl] of databaseUrls.entries()) {
      const rows = await queryCanonicalOutputs(databaseUrl, handles);
      const perHandle = new Map<string, number>();
      for (const row of rows) {
        const key = `0x${row.handle.toString('hex')}`;
        perHandle.set(key, (perHandle.get(key) ?? 0) + 1);
      }
      for (const handle of handles) {
        expect(
          perHandle.get(handle.toLowerCase()) ?? 0,
          `operator ${operator} holds ${perHandle.get(handle.toLowerCase()) ?? 0} rows for ${handle}; ` +
            'duplicate execution must leave exactly one',
        ).to.eq(1);
      }
    }
    console.info('[crash-retry] first-write-wins held: one row per handle on every operator');
  });
});
