import { emitAssertions } from './assertionEvidence';
/**
 * The degraded-cluster cases: what a fleet with a missing operator may and may
 * not commit.
 *
 * Split by what the test container can do. Stopping services needs host
 * process control, so `run-degraded-consensus.sh` applies the faults and this
 * file makes the assertions between the steps. The two halves are sequenced
 * through phases rather than through sleeps, and they carry state across
 * phases through the handshake, because the load-bearing property is that the
 * SAME work recovers -- not that fresh work succeeds afterwards.
 *
 * What changed here and why.
 *
 * The quorum expectation was a boolean called `expectQuorum`, defaulted to 0.
 * `0` meant "do not check", so C4a passed `0` and its comment claimed it
 * asserted that a unanimous topology must NOT reach quorum. It asserted
 * nothing of the kind. Worse, the same `0` was used on a 2-of-3 topology in CI,
 * where quorum SHOULD form -- so the case could not tell the two topologies
 * apart. Quorum is now a three-valued mode (`required`, `forbidden`,
 * `not_checked`), and the threshold it is judged against is read from the
 * RUNNING gateway rather than from the scenario file.
 *
 * C4b then minted a NEW handle to show the returning operator worked. That is a
 * liveness check: backlog convergence is about the handles that were produced
 * while the operator was away. Those handles are published by the outage phase
 * and re-read by the recovery phase.
 *
 * Phases (DEGRADED_PHASE):
 *   agreement    DEG-01: the fleet agrees and reaches its configured quorum
 *   canary       MAT-04: a poisoned digest is rejected, with classification
 *   same-block   DEG-02: one boundary consumed by two same-block transactions
 *   outage       DEG-03/DEG-04: work minted while an operator is fully offline
 *   recovery     DEG-05: that same work converges when the operator returns
 *   gw-arm       DEG-06: an identified gateway event left pending ingestion
 *   gw-verify    DEG-06: that exact event ingested after the restart
 */
import { expect } from 'chai';
import { assertGatewayWatermarkStopped, requireGatewayWatermark } from './faultEvidence';

import { assertCanaryFires, tamperDigest } from './canary';
import { rememberMiningState, restoreMiningState } from './abortRecovery';
import {type PendingGatewayEvent, recordPendingGatewayEvent} from './gatewayRecovery';
import { publishHandshake, readHandshake } from './handshake';
import {
  type GatewayMembership,
  getCoprocessorDbUrls,
  assertGatewayTopology,
  waitForConsensus,
  waitForDatabaseReadiness,
} from './helpers';
import {
  type ProbeContract,
  assertOperatorsAgree,
  assertQuorumOutcome,
  deployProbe,
  mintProbeHandle,
  operatorSet,
} from './probe';
import { queryStorageRowCount, collectOperatorEvidence } from './comparator';
import { assertNoQuorumWithSurvivorSubmissions } from './quorumObservation';
import { assertRunValidity, withDeadline } from './validity';

const ENABLE = process.env.RUN_DEGRADED_CONSENSUS === '1';
const COPROCESSOR_COUNT = Number.parseInt(process.env.COPROCESSOR_COUNT ?? '3', 10);
const PHASE = process.env.DEGRADED_PHASE ?? 'agreement';
const VICTIM = Number.parseInt(process.env.DEGRADED_VICTIM_OPERATOR ?? '2', 10);
const GATEWAY_RPC_URL = process.env.GATEWAY_RPC_URL ?? '';
const GATEWAY_CONFIG_ADDRESS = process.env.GATEWAY_CONFIG_ADDRESS ?? '';
const CIPHERTEXT_COMMITS_ADDRESS = process.env.CIPHERTEXT_COMMITS_ADDRESS ?? '';
const HOST_RPC_URL = process.env.RPC_URL ?? '';
const PROBE_GAS_LIMIT = 10_000_000;
const MARKER = (phase: string) => `[degraded/${phase}] CASE COMPLETE`;

let databaseUrls: string[] = [];
let membership: GatewayMembership;

const required = (value: string, name: string) => {
  if (!value) throw new Error(`${name} must be set for the degraded-cluster gate`);
  return value;
};

async function withPool<T>(databaseUrl: string, fn: (pool: import('pg').Pool) => Promise<T>): Promise<T> {
  const { Pool } = await import('pg');
  const pool = new Pool({ connectionString: databaseUrl, max: 1, statement_timeout: 30_000 } as never);
  try {
    return await withDeadline(fn(pool), 60_000, 'degraded-suite query');
  } finally {
    await pool.end().catch(() => undefined);
  }
}

/** One operator's gateway-ingestion watermark. */
async function gwWatermark(databaseUrl: string): Promise<number | null> {
  return withPool(databaseUrl, async (pool) => {
    const result = await pool.query<{ last_block_num: string | null }>(
      'SELECT last_block_num::text FROM gw_listener_last_block WHERE dummy_id = TRUE',
    );
    const value = result.rows[0]?.last_block_num;
    return value === undefined || value === null ? null : Number.parseInt(value, 10);
  });
}

/** Whether an operator holds any completed row for a handle. Used for absence. */
async function holdsHandle(databaseUrl: string, handle: string): Promise<boolean> {
  return withPool(databaseUrl, async (pool) => {
    const result = await pool.query<{ present: boolean }>(
      "SELECT EXISTS(SELECT 1 FROM computations WHERE output_handle = decode($1, 'hex')) AS present",
      [handle.replace(/^0x/, '')],
    );
    return result.rows[0].present;
  });
}

interface OutageRecord {
  victim: number;
  handles: string[];
  /** The gateway threshold as the running gateway reported it. */
  threshold: number;
  operators: number;
  quorumExpectation: 'required' | 'forbidden';
  contractAddress: string;
}

describe('Degraded-cluster consensus', function () {
  this.timeout(40 * 60_000);

  let contract: ProbeContract;
  let contractAddress: string;

  before(async function () {
    if (!ENABLE) this.skip();
    required(GATEWAY_RPC_URL, 'GATEWAY_RPC_URL');
    required(GATEWAY_CONFIG_ADDRESS, 'GATEWAY_CONFIG_ADDRESS');
    required(CIPHERTEXT_COMMITS_ADDRESS, 'CIPHERTEXT_COMMITS_ADDRESS');
    databaseUrls = getCoprocessorDbUrls(COPROCESSOR_COUNT);
    await waitForDatabaseReadiness(databaseUrls);

    // The gateway's own membership and threshold. Everything below judges
    // quorum against these rather than against the scenario file: the two
    // disagree exactly when a stack was generated from a different scenario
    // than the runner believes, which is the case worth catching.
    membership = await assertGatewayTopology(GATEWAY_RPC_URL, GATEWAY_CONFIG_ADDRESS, COPROCESSOR_COUNT, Number(process.env.CONSENSUS_THRESHOLD));
    console.info(
      `[degraded] gateway reports ${membership.txSenders.length} authorized coprocessor(s), ` +
        `threshold ${membership.threshold}`,
    );

    // The outage phases deliberately hold an operator down, so gating it would
    // report the injected fault as an invalid run.
    const gated = PHASE === 'outage' ? operatorSet(COPROCESSOR_COUNT, [VICTIM]) : operatorSet(COPROCESSOR_COUNT);
    console.info(
      `[degraded] validity gates: ${await assertRunValidity({
        databaseUrls,
        rpcUrl: HOST_RPC_URL || undefined,
        operators: gated,
        // The outage phase stops a worker on purpose; its scheduler gauge is
        // unreachable and its absence is the point.
        checkDeferred: PHASE !== 'outage',
      })}`,
    );

    if (PHASE === 'recovery' || PHASE === 'gw-verify') {
      // These phases reason about work an earlier phase minted, so they must
      // NOT deploy a fresh fixture.
      return;
    }
    const { getSigners, initSigners } = await import('../signers');
    await initSigners(2);
    const signers = await getSigners();
    const deployment = await deployProbe(signers.alice);
    contract = deployment.contract;
    contractAddress = deployment.address;
  });

  // ---------------------------------------------------------------- DEG-01
  it('DEG-01: the fleet agrees and reaches the gateway\'s configured quorum', async function () {
    if (PHASE !== 'agreement') this.skip();
    const operators = operatorSet(COPROCESSOR_COUNT);
    const handle = await mintProbeHandle(contract);
    const report = await assertOperatorsAgree(databaseUrls, operators, handle);
    console.info(`[degraded/agreement] agreed on ${handle}; compared ${report.compared.join(', ')}`);

    const outcome = await assertQuorumOutcome({
      mode: 'required',
      gatewayRpcUrl: GATEWAY_RPC_URL,
      ciphertextCommitsAddress: CIPHERTEXT_COMMITS_ADDRESS,
      handle,
      authorizedSenders: membership.txSenders,
      threshold: membership.threshold,
      label: 'DEG-01',
    });
    console.info(`[degraded/agreement] ${outcome.detail}`);
    emitAssertions('DEG-01-AGREEMENT-QUORUM', ['bytes', 'quorum'], 'Fleet canonical bytes/digests agree and distinct authorized members formed the configured quorum.');
    console.info(MARKER('agreement'));
  });

  // ---------------------------------------------------------------- MAT-04
  it('MAT-04: a poisoned digest is rejected by the comparator, as a digest mismatch', async function () {
    if (PHASE !== 'canary') this.skip();
    const operators = operatorSet(COPROCESSOR_COUNT);
    const handle = await mintProbeHandle(contract);
    // Agreement, and publication, before poisoning: the canary tampers with an
    // already-committed value so it can never damage a gateway commitment.
    await assertOperatorsAgree(databaseUrls, operators, handle);
    await waitForConsensus(GATEWAY_RPC_URL, CIPHERTEXT_COMMITS_ADDRESS, handle, 5 * 60_000);

    const outcome = await assertCanaryFires(databaseUrls, operators, handle, 'MAT-04');
    expect(outcome.kind, 'the canary must be caught as a compute-digest mismatch').to.eq('compute-digest');
    emitAssertions('MAT-04-CANARY-COMPUTE-DIGEST', ['safety'], 'Clean comparison passed, poison was classified as compute-digest mismatch, and restored comparison passed.');
    console.info(MARKER('canary'));
  });

  // ---------------------------------------------------------------- DEG-02
  it('DEG-02: a prior-block boundary consumed by two same-block transactions converges', async function () {
    if (PHASE !== 'same-block') this.skip();
    const operators = operatorSet(COPROCESSOR_COUNT);
    const { JsonRpcProvider } = await import('ethers');
    const provider = new JsonRpcProvider(required(HOST_RPC_URL, 'RPC_URL'));

    // Same-block inclusion is CONTROLLED, not hoped for. Two sequential sends
    // to an interval miner land in whichever blocks the miner happens to
    // produce, and the case's whole subject is two transactions in ONE block.
    await rememberMiningState(provider, HOST_RPC_URL);
    await provider.send('evm_setIntervalMining', [0]);
    await provider.send('evm_setAutomine', [false]);
    try {
      const first = await contract.combineFromStorage({ gasLimit: PROBE_GAS_LIMIT });
      const second = await contract.combineFromStorage({ gasLimit: PROBE_GAS_LIMIT });
      await provider.send('evm_mine', []);
      const receipts = (await Promise.all([
        (first as unknown as { wait(): Promise<{ blockNumber: number; hash: string } | null> }).wait(),
        (second as unknown as { wait(): Promise<{ blockNumber: number; hash: string } | null> }).wait(),
      ])) as ({ blockNumber: number; hash: string } | null)[];
      expect(receipts[0], 'the first transaction produced no receipt').to.not.be.null;
      expect(receipts[1], 'the second transaction produced no receipt').to.not.be.null;
      expect(
        receipts[1]!.blockNumber,
        'both transactions must be included in ONE block; two sequential sends to an interval miner ' +
          'are not proof of same-block execution',
      ).to.eq(receipts[0]!.blockNumber);
      console.info(
        `[degraded/same-block] both transactions in block ${receipts[0]!.blockNumber}: ` +
          `${receipts[0]!.hash}, ${receipts[1]!.hash}`,
      );

      const handle = (await contract.combined()).toLowerCase();
      // Identical sourcing, so the two transactions ALIAS: one handle, one
      // value, two producing transactions. The comparator asserts one value and
      // storage-row uniqueness, and compares the normalized producing set
      // across operators rather than an arbitrary first row.
      const report = await assertOperatorsAgree(databaseUrls, operators, handle);
      expect(
        report.provenance.length,
        `the aliased handle must be attributed to both producing transactions, got ${report.provenance.join(', ')}`,
      ).to.eq(2);
      for (const operator of operators) {
        expect(
          await queryStorageRowCount(databaseUrls[operator], handle),
          `operator ${operator} must hold exactly one storage row for the aliased handle`,
        ).to.eq(1);
      }
      console.info(`[degraded/same-block] converged on ${handle} from ${report.provenance.join(' + ')}`);
      emitAssertions('DEG-02-SAME-BLOCK-PAIR', ['precondition', 'bytes', 'provenance'], 'Both identified receipts share one block; every operator has one aliased value and the same two producing transactions.');
      console.info(MARKER('same-block'));
    } finally {
      await restoreMiningState(provider, HOST_RPC_URL);
      provider.destroy();
    }
  });

  // ------------------------------------------------------- DEG-03 / DEG-04
  it('DEG-03/DEG-04: work minted while one operator is offline', async function () {
    if (PHASE !== 'outage') this.skip();
    const survivors = operatorSet(COPROCESSOR_COUNT, [VICTIM]);
    expect(survivors.length, 'a degraded case needs at least two survivors to compare').to.be.greaterThanOrEqual(2);

    // The expectation follows the gateway's own threshold, not the scenario
    // name: with a threshold the survivors can meet, quorum MUST form for work
    // submitted during the outage; with a unanimous threshold it must NOT.
    const quorumExpectation = survivors.length >= membership.threshold ? 'required' : 'forbidden';
    console.info(
      `[degraded/outage] ${survivors.length} survivor(s) against threshold ${membership.threshold}: ` +
        `quorum ${quorumExpectation}`,
    );

    // The victim must really be offline before the workload is minted, and
    // that is judged on its DATA rather than on the runner's say-so: an
    // operator whose services are down cannot ingest the block the workload
    // lands in.
    const victimWatermarkBefore = requireGatewayWatermark(await gwWatermark(databaseUrls[VICTIM]), VICTIM);

    const handles: string[] = [];
    for (let index = 0; index < 3; index += 1) handles.push(await mintProbeHandle(contract));
    const distinct = [...new Set(handles)];
    console.info(`[degraded/outage] minted ${distinct.length} handle(s) while operator ${VICTIM} was offline`);

    // Survivors agree on it.
    for (const handle of distinct) {
      await assertOperatorsAgree(databaseUrls, survivors, handle, { timeoutMs: 8 * 60_000 });
    }
    console.info('[degraded/outage] survivors agree on every handle minted during the outage');

    // The victim holds none of it, which is the independent confirmation that
    // it really was offline for this work rather than merely reported so.
    for (const handle of distinct) {
      expect(
        await holdsHandle(databaseUrls[VICTIM], handle),
        `operator ${VICTIM} holds ${handle}, so it was not offline while the workload was minted and this ` +
          'case is not measuring a degraded cluster',
      ).to.eq(false);
    }
    {
      const after = await gwWatermark(databaseUrls[VICTIM]);
      assertGatewayWatermarkStopped(victimWatermarkBefore, after, VICTIM);
      console.info(`[degraded/outage] victim gateway watermark ${victimWatermarkBefore} -> ${after ?? 'unreadable'}`);
    }

    if (quorumExpectation === 'forbidden') {
      // The fault is fully verified, so the distinct authorized submissions
      // observed for this fresh handle must come from the healthy survivors.
      const handle = distinct[0];
      const evidence = await collectOperatorEvidence(databaseUrls[survivors[0]], survivors[0], handle);
      const observed = await assertNoQuorumWithSurvivorSubmissions({
        gatewayRpcUrl: GATEWAY_RPC_URL, ciphertextCommitsAddress: CIPHERTEXT_COMMITS_ADDRESS,
        handle, authorizedSenders: membership.txSenders, survivorCount: survivors.length,
        expected: { keyId: BigInt(`0x${evidence.keyId.toString('hex')}`).toString(),
          ciphertextDigest: `0x${evidence.computeDigest!.toString('hex')}`,
          snsCiphertextDigest: `0x${evidence.snsDigest!.toString('hex')}` },
      });
      console.info(`[degraded/outage] ${handle}: no quorum formed after healthy survivor submissions; ` +
        `gateway advanced ${observed.firstBlock} -> ${observed.lastBlock} across ${observed.samples} observations inside the window`);
    } else {
      for (const handle of distinct) {
        const outcome = await assertQuorumOutcome({ mode: 'required', gatewayRpcUrl: GATEWAY_RPC_URL,
          ciphertextCommitsAddress: CIPHERTEXT_COMMITS_ADDRESS, handle,
          authorizedSenders: membership.txSenders, threshold: membership.threshold,
          timeoutMs: 6 * 60_000, label: 'DEG-03' });
        console.info(`[degraded/outage] ${handle}: ${outcome.detail}`);
      }
    }

    const record: OutageRecord = {
      victim: VICTIM,
      handles: distinct,
      threshold: membership.threshold,
      operators: COPROCESSOR_COUNT,
      quorumExpectation,
      contractAddress,
    };
    emitAssertions(quorumExpectation === 'forbidden' ? 'DEG-04-UNANIMOUS-NO-QUORUM' : 'DEG-03-MAJORITY-AVAILABLE', ['precondition', 'bytes', 'quorum', 'safety'], quorumExpectation === 'forbidden' ? 'Victim stayed offline; survivors agreed, submitted, and advancing successful gateway observations showed no quorum throughout the window.' : 'Victim stayed offline while survivors agreed and formed the required authorized quorum without its output.');
    publishHandshake('degraded-outage', record);
    console.info(MARKER('outage'));
  });

  // ---------------------------------------------------------------- DEG-05
  it('DEG-05: the returning operator converges on the backlog it missed', async function () {
    if (PHASE !== 'recovery') this.skip();
    const record = readHandshake<OutageRecord>('degraded-outage').payload;
    expect(record.handles.length, 'the outage phase must have published the handles it minted').to.be.greaterThan(0);
    expect(
      record.victim,
      'the recovery phase must be measuring the operator the outage phase took down',
    ).to.eq(VICTIM);
    console.info(
      `[degraded/recovery] converging operator ${record.victim} on ${record.handles.length} handle(s) ` +
        'minted during its outage — the SAME handles, not a fresh fixture',
    );

    const operators = operatorSet(COPROCESSOR_COUNT);
    for (const handle of record.handles) {
      // The returned operator has to produce these rows itself and agree on
      // them. This is the assertion a fresh mint cannot make.
      const report = await assertOperatorsAgree(databaseUrls, operators, handle, { timeoutMs: 12 * 60_000 });
      expect(report.operators, 'the returned operator must be in the comparison').to.include(record.victim);
    }
    console.info('[degraded/recovery] the returned operator agrees on every backlog handle');

    // And those same handles reach quorum now that the fleet is whole. Where
    // the outage already produced quorum this re-confirms it; where it could
    // not, this is the liveness half of the case.
    for (const handle of record.handles) {
      const outcome = await assertQuorumOutcome({
        mode: 'required',
        gatewayRpcUrl: GATEWAY_RPC_URL,
        ciphertextCommitsAddress: CIPHERTEXT_COMMITS_ADDRESS,
        handle,
        authorizedSenders: membership.txSenders,
        threshold: membership.threshold,
        timeoutMs: 8 * 60_000,
        label: 'DEG-05',
      });
      console.info(`[degraded/recovery] ${handle}: ${outcome.detail}`);
    }
    emitAssertions('DEG-05-BACKLOG-CONVERGENCE', ['liveness', 'bytes', 'quorum', 'safety'], 'The returning victim agreed on every original outage handle, and those same handles reached quorum.');
    console.info(MARKER('recovery'));
  });

  // ---------------------------------------------------------------- DEG-06
  it('DEG-06 (arm): an identified gateway event is left pending ingestion', async function () {
    if (PHASE !== 'gw-arm') this.skip();
    const operators = operatorSet(COPROCESSOR_COUNT);

    // Mint and let the gateway commit it. The gw-listeners are stalled by the
    // runner at this point, so the commitment forms -- the transaction senders
    // submit to the gateway directly -- but nothing ingests it.
    const handle = await mintProbeHandle(contract);
    await assertOperatorsAgree(databaseUrls, operators, handle, { timeoutMs: 8 * 60_000 });
    const consensus = await waitForConsensus(GATEWAY_RPC_URL, CIPHERTEXT_COMMITS_ADDRESS, handle, 8 * 60_000);
    expect(consensus, 'the workload must reach a gateway commitment for there to be an event to ingest').to.not.be
      .null;
    const eventBlock = consensus!.blockNumber;
    console.info(`[degraded/gw-arm] identified gateway event for ${handle} in gateway block ${eventBlock}`);

    // The stage: every operator's gateway watermark is behind that block, so
    // the event is genuinely pending rather than already applied.
    const pending: number[] = [];
    for (const operator of operators) {
      const watermark = await gwWatermark(databaseUrls[operator]);
      console.info(`[degraded/gw-arm] operator ${operator} watermark ${watermark ?? 'none'} (event ${eventBlock})`);
      if (watermark === null || watermark < eventBlock) pending.push(operator);
    }
    expect(
      pending,
      'no operator was left with the event pending, so restarting the listeners cannot demonstrate ' +
        'recovery of an in-flight event. The runner must stall the gateway listeners before this phase',
    ).to.have.length(operators.length);

    const {JsonRpcProvider, id} = await import('ethers');
    const provider = new JsonRpcProvider(GATEWAY_RPC_URL);
    let event;
    try {
      const logs = await provider.getLogs({address:CIPHERTEXT_COMMITS_ADDRESS, fromBlock:eventBlock, toBlock:eventBlock,
        topics:[id('AddCiphertextMaterialConsensus(bytes32,uint256,bytes32,bytes32,address[])'),handle]});
      expect(logs, 'one exact gateway receipt/log must identify the pending event').to.have.length(1);
      event = logs[0];
    } finally {provider.destroy();}
    const record = await recordPendingGatewayEvent(databaseUrls, {handle, eventBlock,
      eventBlockHash:event.blockHash, eventTxHash:event.transactionHash, eventLogIndex:event.index});
    // All originals are durable before the first write. The central canary
    // helper waits for EACH operator's own publication under its row lock.
    for (const {operator,digest} of record.originals) {
      const original = await tamperDigest(databaseUrls[operator], handle);
      expect(original.toString('hex'), 'the published original changed during arming').to.eq(digest);
    }
    emitAssertions('DEG-06-GW-LISTENER-INFLIGHT', ['precondition', 'quorum'], 'The exact consensus event was identified while target gateway listeners remained behind its block.');
    console.info(MARKER('gw-arm'));
  });

  it('DEG-06 (verify): the identified event was compared by replacements and originals are restored', async function () {
    if (PHASE !== 'gw-verify') this.skip();
    const record = readHandshake<PendingGatewayEvent>('degraded-gw-event').payload;
    expect(record.restored, 'host must verify the exact-event warnings and restore the poison first').to.eq(true);
    const operators = operatorSet(COPROCESSOR_COUNT);

    const deadline = Date.now() + 8 * 60_000;
    const reached = new Map<number, number | null>();
    for (;;) {
      let allPast = true;
      for (const operator of operators) {
        const watermark = await gwWatermark(databaseUrls[operator]);
        reached.set(operator, watermark);
        if (watermark === null || watermark < record.eventBlock) allPast = false;
      }
      if (allPast) break;
      if (Date.now() >= deadline) {
        throw new Error(
          `after the gateway listeners restarted, operator watermarks are ` +
            `${[...reached].map(([operator, value]) => `${operator}:${value ?? 'none'}`).join(', ')} and the ` +
            `identified event sits in gateway block ${record.eventBlock}; the in-flight event was not ingested`,
        );
      }
      await new Promise((resolve) => setTimeout(resolve, 5_000));
    }
    console.info(
      `[degraded/gw-verify] every operator ingested past the identified event ` +
        `(${[...reached].map(([operator, value]) => `${operator}:${value}`).join(', ')})`,
    );

    // And the workload itself is unchanged and still agreed.
    await assertOperatorsAgree(databaseUrls, operators, record.handle, { timeoutMs: 5 * 60_000 });
    for (const operator of operators) {
      expect(
        await queryStorageRowCount(databaseUrls[operator], record.handle),
        `operator ${operator} holds a duplicate storage row for ${record.handle} after the restart`,
      ).to.eq(1);
    }
    emitAssertions('DEG-06-GW-LISTENER-INFLIGHT', ['liveness', 'safety'], 'Replacement listeners ingested beyond the exact pending event and original output agreement/storage uniqueness survived restoration.');
    console.info(MARKER('gw-verify'));
  });
});
