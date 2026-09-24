import { assertGpuExecutionDiversity } from './gpuExecutionEvidence';
import { emitAssertions } from './assertionEvidence';
/**
 * CPU/GPU homogeneous byte-consensus gate for transaction-boundary
 * ciphertext materialization.  Run this file once in a CPU topology and once
 * in a GPU topology.  It intentionally never combines their database
 * snapshots: FFT backends may produce different ciphertext bytes, while
 * user-decrypted plaintexts remain the cross-backend oracle.
 */
import { expect } from 'chai';

import { assertCanaryFiresWith, assertRawByteCanaryFiresWith } from './canary';
import {
  assertExecutedSchedulingDiffers,
  executedScheduling,
  readAllSchedulingCounters,
} from './schedulingEvidence';
import { assertCiphertext128Format, assertRunValidity } from './validity';
import {
  assertConsensusEventBindings,
  assertGatewayTopology,
  assertDeviceSplit,
  assertHeterogeneousScheduling,
  attestationEvidenceFromCanonicalOutput,
  getCoprocessorDbUrls,
  waitForConsensus,
  waitForConsensusDatabaseReports,
  waitForDatabaseReadiness,
  waitForKmsNamespaceAttestationReadiness,
} from './helpers';
import { assertFixtureTransactionShape } from './materializationProvenance';
import {
  FIXTURE_EXPECTED_PLAINTEXTS,
  FIXTURE_HANDLE_LABELS,
  FIXTURE_PRODUCED_OUTPUT_LABELS,
  type FixtureHandleLabel,
} from './materializationFixtureModel';

const ENABLE_MATERIALIZATION_CONSENSUS = process.env.RUN_MATERIALIZATION_CONSENSUS === '1';
const COPROCESSOR_COUNT = Number.parseInt(process.env.COPROCESSOR_COUNT ?? '3', 10);
const CONSENSUS_THRESHOLD = Number.parseInt(
  process.env.CONSENSUS_THRESHOLD ?? process.env.COPROCESSOR_THRESHOLD ?? String(COPROCESSOR_COUNT),
  10,
);
const GATEWAY_RPC_URL = process.env.GATEWAY_RPC_URL ?? '';
const GATEWAY_CONFIG_ADDRESS = process.env.GATEWAY_CONFIG_ADDRESS ?? '';
const CIPHERTEXT_COMMITS_ADDRESS = process.env.CIPHERTEXT_COMMITS_ADDRESS ?? '';
// Scheduling is an axis the protocol claims byte-consensus is independent of,
// so a deliberately heterogeneous fleet is a stronger run of this same gate --
// not a different execution class. Set by the topology launcher's `test-env`.
const SCHEDULING_CLASSES = process.env.CONSENSUS_SCHEDULING_CLASSES ?? '';
const EXPECT_HETEROGENEOUS_SCHEDULING = process.env.EXPECT_HETEROGENEOUS_SCHEDULING === '1';
const EXPECT_DEVICE_SPLIT = process.env.EXPECT_DEVICE_SPLIT === '1';

function required(value: string, name: string): string {
  if (!value) throw new Error(`${name} must be set for the materialization consensus gate`);
  return value;
}

/**
 * Fire a burst of independent single-transaction chains.
 *
 * Independent so they can be batched at all: a window batches CHAINS, and a
 * dependent graph gives it nothing to choose between. Sent without awaiting
 * inclusion, so they queue together and are ingested as one backlog rather
 * than as a trickle the narrowest window handles just as well as the widest.
 */
/** Wait for every identified transaction, even when one execution batch takes
 * longer than the metrics sampling interval. Silence is not evidence of drain. */
async function waitForSchedulingDrain(
  operators: number[],
  transactionHashes: string[],
): Promise<Awaited<ReturnType<typeof readAllSchedulingCounters>>> {
  const { Pool } = await import('pg');
  const databases = getCoprocessorDbUrls(COPROCESSOR_COUNT);
  const pools = operators.map((operator) => new Pool({ connectionString: databases[operator], max: 1 }));
  const hashes = transactionHashes.map((hash) => Buffer.from(hash.slice(2), 'hex'));
  const deadline = Date.now() + 10 * 60_000;
  try {
    for (;;) {
      const reports = await Promise.all(pools.map((pool) => pool.query<{
        transactions: string; pending: string; errors: string;
      }>(`SELECT count(DISTINCT transaction_id) AS transactions,
           count(*) FILTER (WHERE NOT is_completed) AS pending,
           count(*) FILTER (WHERE is_error) AS errors
         FROM computations WHERE transaction_id=ANY($1::bytea[])`, [hashes])));
      if (reports.some((report) => Number(report.rows[0].errors) > 0)) throw new Error('scheduling backlog contains failed computations');
      if (reports.every((report) => Number(report.rows[0].transactions) === hashes.length && Number(report.rows[0].pending) === 0)) {
        // The histogram observation follows the commit in the same worker cycle.
        await new Promise((resolve) => setTimeout(resolve, 1_000));
        return await readAllSchedulingCounters(operators);
      }
      if (Date.now() >= deadline) throw new Error('identified scheduling backlog did not drain on every operator');
      await new Promise((resolve) => setTimeout(resolve, 1_000));
    }
  } finally {
    await Promise.all(pools.map((pool) => pool.end()));
  }
}

async function mintSchedulingBacklog(owner: unknown): Promise<string[]> {
  const { deployProbe } = await import('./probe');
  const { ethers } = await import('hardhat');
  const deployment = await deployProbe(owner);
  const contract = deployment.contract as unknown as {
    combineFromStorage(overrides: Record<string, unknown>): Promise<{ hash: string; wait(): Promise<unknown> }>;
  };
  const address = await (owner as { getAddress(): Promise<string> }).getAddress();
  const base = await ethers.provider.getTransactionCount(address, 'pending');

  const BURST = 24;
  const sent: Promise<{ hash: string; wait(): Promise<unknown> }>[] = [];
  for (let index = 0; index < BURST; index += 1) {
    sent.push(contract.combineFromStorage({ gasLimit: 10_000_000, nonce: base + index }));
  }
  const transactions = await Promise.all(sent);
  await Promise.all(transactions.map((transaction) => transaction.wait()));
  console.info(
    `[materialization-consensus] scheduling backlog: ${BURST} independent transaction(s) queued together, ` +
      'so a wide window has something to batch',
  );
  return transactions.map((transaction) => transaction.hash);
}

describe('Materialization byte consensus', function () {
  this.timeout(15 * 60_000);

  let databaseUrls: string[];
  const formatHandles = new Set<string>();
  // Hoisted: the after-hook format gate needs the backend class the before
  // hook resolved, and re-reading the environment there would let a run whose
  // env changed mid-flight judge itself against the wrong backend.
  let execution: { softwareRevision: string; backendClass: string; hardwareClass: string };
  // Scheduling counters read before the workload; the after-hook compares.
  let schedulingBefore: Awaited<ReturnType<typeof readAllSchedulingCounters>> | undefined;
  let schedulingBacklogPending = false;
  let schedulingBacklogTransactions = 0;
  let authorizedSenders = new Set<string>();
  let completedBodies = 0;
  let schedulingDrained: Awaited<ReturnType<typeof readAllSchedulingCounters>> | undefined;

  before(async function () {
    if (!ENABLE_MATERIALIZATION_CONSENSUS) {
      this.skip();
    }
    if (COPROCESSOR_COUNT !== 3) {
      throw new Error('the homogeneous gate is the strict three-coprocessor gate; use a separate suite for 2-of-3');
    }
    if (CONSENSUS_THRESHOLD !== COPROCESSOR_COUNT) {
      throw new Error('the homogeneous gate requires unanimous 3-of-3 consensus; use a separate suite for 2-of-3');
    }

    // These values are deliberately explicit in the run environment.  The
    // topology launcher pins every node to the same image/backend/hardware
    // class; recording it here makes byte equality auditable and prevents a
    // CPU/GPU comparison from being mislabeled as a consensus failure.
    execution = {
      softwareRevision: required(process.env.CONSENSUS_SOFTWARE_REVISION ?? '', 'CONSENSUS_SOFTWARE_REVISION'),
      backendClass: required(process.env.CONSENSUS_BACKEND_CLASS ?? '', 'CONSENSUS_BACKEND_CLASS'),
      hardwareClass: required(process.env.CONSENSUS_HARDWARE_CLASS ?? '', 'CONSENSUS_HARDWARE_CLASS'),
    };
    required(GATEWAY_RPC_URL, 'GATEWAY_RPC_URL');
    required(GATEWAY_CONFIG_ADDRESS, 'GATEWAY_CONFIG_ADDRESS');
    required(CIPHERTEXT_COMMITS_ADDRESS, 'CIPHERTEXT_COMMITS_ADDRESS');
    const membership = await assertGatewayTopology(GATEWAY_RPC_URL, GATEWAY_CONFIG_ADDRESS, COPROCESSOR_COUNT, CONSENSUS_THRESHOLD);
    authorizedSenders = new Set(membership.txSenders);
    console.info(`[materialization-consensus] execution class ${JSON.stringify(execution)}`);

    // The execution class above is what must be identical. Scheduling is what
    // may differ: RFC 020 makes result bytes a function of on-chain data
    // alone, independent of how a worker windows, batches and places the work.
    // When a run claims to exercise that independence, prove the fleet really
    // was heterogeneous before spending ten minutes concluding it agrees --
    // otherwise a mistyped override yields a green that asserts nothing.
    if (EXPECT_HETEROGENEOUS_SCHEDULING) {
      const classes = assertHeterogeneousScheduling(SCHEDULING_CLASSES, COPROCESSOR_COUNT);
      for (const [index, description] of [...classes].sort((a, b) => a[0] - b[0]))
        console.info(`[materialization-consensus] operator ${index} scheduling ${description}`);
      // Distinct CONFIGURATION is a precondition, not the claim. The counters
      // are read here and again in the after-hook, and the delta has to show
      // the operators executed materially different batches -- otherwise the
      // workload never drove the difference and the run reports agreement
      // across strategies it did not exercise.
      schedulingBefore = await readAllSchedulingCounters([...classes.keys()].sort((a, b) => a - b));
      emitAssertions(execution.backendClass === 'cpu' ? 'SCH-06-CPU-DIVERSITY' : 'SCH-01-HETEROGENEOUS', ['precondition'], 'Resolved scheduling classes were distinct and every operator exposed readable starting counters.');

      // A backlog dense enough for the configured windows to MATTER.
      //
      // Sparse arrivals can produce identical batches under different window
      // settings. Hold a complete independent backlog before allowing workers
      // to acquire it so the configured capacity can affect actual execution.
      schedulingBacklogPending = true;
      console.info(
        `[materialization-consensus] scheduling counters at start: ${[...schedulingBefore.values()]
          .map((entry) => `${entry.operator}:${entry.transactions}/${entry.batches}`)
          .join(' ')}`,
      );
    } else if (SCHEDULING_CLASSES) {
      console.info(`[materialization-consensus] scheduling classes ${SCHEDULING_CLASSES}`);
    }

    // Device placement is the independence no run varied until this host had two
    // GPUs. Assert the split happened rather than trusting the override: an
    // unset or mistyped GPU_CONSENSUS_DEVICE_<index> leaves every operator on
    // card 0, and the run would then report device independence it never tested.
    if (EXPECT_DEVICE_SPLIT) {
      const devices = assertDeviceSplit(SCHEDULING_CLASSES, COPROCESSOR_COUNT);
      console.info(
        `[materialization-consensus] device split across CUDA devices ` +
          `${[...new Set(devices.values())].sort().join(', ')} (hardware class ${execution.hardwareClass})`,
      );
    }

    // Keep the opt-in suite import-safe on a developer machine that has not
    // installed/configured the SDK.  The ordinary E2E runtime is loaded only
    // after the explicit gate flag above has enabled this test.
    const [{ createInstances }, { getSigners, initSigners }] = await Promise.all([
      import('../instance'),
      import('../signers'),
    ]);
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);

    // Measure committed batch sizes across the complete identified backlog.
    if (schedulingBacklogPending) {
      schedulingBefore = await readAllSchedulingCounters([...(schedulingBefore ?? new Map()).keys()]);
      const transactionHashes = await mintSchedulingBacklog(this.signers.alice);
      schedulingBacklogTransactions = transactionHashes.length;
      schedulingDrained = await waitForSchedulingDrain([...schedulingBefore.keys()], transactionHashes);
      schedulingBacklogPending = false;
    }
    databaseUrls = getCoprocessorDbUrls(COPROCESSOR_COUNT);
    await waitForDatabaseReadiness(databaseUrls);

    // The thorough oracle is also the most expensive; ten minutes spent on a
    // stack that was never provisioned is the worst kind of green.
    console.info(
      `[materialization-consensus] validity gates: ${await assertRunValidity({
        databaseUrls,
        rpcUrl: process.env.RPC_URL,
      })}`,
    );
  });

  // The format gate needs rows to judge, so it runs once the suite has
  // materialized something rather than in `before` on an empty stack.
  // Failing here still fails the suite, which is what it is for: a GPU run
  // whose rows say CPU had a worker of the wrong backend on a queue.
  after(async function () {
    if (!ENABLE_MATERIALIZATION_CONSENSUS) return;
    if (completedBodies !== 2) throw new Error('materialization bodies did not both complete; no scheduling byte receipt can be issued');

    // Executed scheduling evidence, before the format gate: a heterogeneous run
    // that agreed while every operator executed the same batch has not
    // established scheduling independence, and that has to fail the suite
    // rather than be noted in a log line.
    if (EXPECT_HETEROGENEOUS_SCHEDULING) {
      if (!schedulingBefore) {
        throw new Error('the heterogeneous run never read its starting scheduling counters');
      }
      // The bracketed pair when the backlog produced one: that window is where
      // the configured difference is expressed at all.
      const after = schedulingDrained ?? (await readAllSchedulingCounters([...schedulingBefore.keys()]));
      const executed = executedScheduling(schedulingBefore, after, SCHEDULING_CLASSES);
      let summary = assertExecutedSchedulingDiffers(executed, schedulingBacklogTransactions);
      if (execution.backendClass !== 'cpu') {
        summary += `; GPU permits: ${assertGpuExecutionDiversity(schedulingBefore, after, SCHEDULING_CLASSES)}`;
      }
      console.info(`[materialization-consensus] executed scheduling: ${summary}`);
      emitAssertions(execution.backendClass === 'cpu' ? 'SCH-06-CPU-DIVERSITY' : 'SCH-01-HETEROGENEOUS', ['evidence'], summary);
    }

    const formats = await assertCiphertext128Format(databaseUrls, execution.backendClass, [...formatHandles]);
    const values = [...new Set([...formats.values()].flat())];
    if (EXPECT_HETEROGENEOUS_SCHEDULING) emitAssertions(execution.backendClass === 'cpu' ? 'SCH-06-CPU-DIVERSITY' : 'SCH-01-HETEROGENEOUS', ['bytes'], 'Materialized outputs passed fleet byte/digest comparison and backend format validation.');
    console.info(
        `[materialization-consensus] ciphertext128_format ${values.join(',')} identical on all ` +
          `${formats.size} operator(s), matching ${execution.backendClass}.`,
    );
  });

  it('converges on same-block cross-transaction boundaries and intra-transaction fan-out', async function () {
    const { decryptMaterializationFixture, deployMaterializationFixture, runMaterializationFixture } =
      await import('./materializationFixture');
    const deployment = await deployMaterializationFixture(this.signers.alice);
    const run = await runMaterializationFixture({
      ...deployment,
      owner: this.signers.alice,
      instance: this.instances.alice,
    });

    // `waitForConsensusDatabaseReports` fails closed on a missing/duplicate
    // canonical row; any raw ciphertext -> Keccak digest mismatch; or type,
    // operation, transaction, or block-provenance difference.  It runs before
    // awaiting the Gateway event so its TFHE/SNS evidence is captured while
    // available locally.  CPU and GPU invoke this test in separate runs and
    // are compared below only through plaintexts.
    const reports = await waitForConsensusDatabaseReports(
      databaseUrls,
      FIXTURE_PRODUCED_OUTPUT_LABELS.map((label) => run.handles[label]),
      { timeoutMs: 10 * 60_000 },
    );
    for (const label of FIXTURE_PRODUCED_OUTPUT_LABELS) formatHandles.add(run.handles[label]);
    assertFixtureTransactionShape(reports, run);
    emitAssertions('MAT-01-BOUNDARY-FANOUT', ['bytes', 'digest', 'provenance', 'liveness'], 'Every produced handle has canonical byte/digest/provenance agreement and every identified transaction has exact successful completion counts.');

    // The canary this suite class owes, aimed at the comparator these
    // assertions actually rest on. `waitForConsensusDatabaseReports` is the
    // gate's oracle: it fails closed on a missing or duplicate canonical row,
    // a ciphertext-to-digest mismatch, or a provenance difference. Poison one
    // operator's digest and it must reject the fleet -- with a short timeout,
    // since here a rejection is the expected outcome rather than something to
    // wait ten minutes for.
    const canaryHandle = run.handles[FIXTURE_PRODUCED_OUTPUT_LABELS[0]];
    await assertCanaryFiresWith(
      databaseUrls[databaseUrls.length - 1],
      canaryHandle,
      'materialization-consensus',
      async (phase) => {
        await waitForConsensusDatabaseReports(
          databaseUrls,
          FIXTURE_PRODUCED_OUTPUT_LABELS.map((label) => run.handles[label]),
          { timeoutMs: phase === 'poisoned' ? 45_000 : 6 * 60_000 },
        );
      },
    );

    await assertRawByteCanaryFiresWith(
      databaseUrls[databaseUrls.length - 1], canaryHandle, 'materialization-consensus',
      async phase => {
        await waitForConsensusDatabaseReports(databaseUrls, FIXTURE_PRODUCED_OUTPUT_LABELS.map(label => run.handles[label]),
          { timeoutMs: phase === 'poisoned' ? 45_000 : 6 * 60_000 });
      },
    );

    // Every produced output is publishable in this fixture.  This includes
    // materialized TrivialEncrypt values: they have a producing transaction
    // and must not evade the byte/digest/provenance oracle.  Only
    // VerifyInput-only handles stay plaintext-only.  Waiting for each quorum
    // catches a bad intermediate result instead of accepting a correct
    // terminal value that happened to mask it.
    const consensuses = await Promise.all(
      FIXTURE_PRODUCED_OUTPUT_LABELS.map((label) =>
        waitForConsensus(GATEWAY_RPC_URL, CIPHERTEXT_COMMITS_ADDRESS, run.handles[label]),
      ),
    );
    for (let index = 0; index < consensuses.length; index += 1) {
      const label = FIXTURE_PRODUCED_OUTPUT_LABELS[index];
      const consensus = consensuses[index];
      expect(consensus, `on-chain quorum must form for ${label}`).to.not.be.null;
      const senders = consensus!.senders.map((sender) => sender.toLowerCase());
      expect(senders.every(sender => authorizedSenders.has(sender)), `${label} senders must belong to the observed gateway membership`).to.eq(true);
      expect(senders, `${label} consensus must contain the configured quorum`).to.have.length(CONSENSUS_THRESHOLD);
      expect(new Set(senders).size, `${label} consensus must contain unique submitters`).to.eq(CONSENSUS_THRESHOLD);
    }
    assertConsensusEventBindings(reports, consensuses as Exclude<(typeof consensuses)[number], null>[]);
    emitAssertions('MAT-01-BOUNDARY-FANOUT', ['quorum'], 'Every produced handle has exactly one bound consensus event with the required distinct senders.');

    const terminalOutput = reports[0].outputs.find(
      (output) => `0x${output.handle.toString('hex')}`.toLowerCase() === run.handles.terminal.toLowerCase(),
    );
    if (!terminalOutput)
      throw new Error('canonical database report is missing the terminal output attestation evidence');

    // Gateway consensus proves every SNS submission reached the chain, but it
    // does not prove that an attestation HEAD from the KMS worker namespace is
    // already routable.  Gate user requests on the terminal output in every
    // registered bucket so a transient object/routing gap cannot consume all
    // retry attempts for the whole plaintext oracle at once.
    await waitForKmsNamespaceAttestationReadiness({
      gatewayRpcUrl: GATEWAY_RPC_URL,
      gatewayConfigAddress: GATEWAY_CONFIG_ADDRESS,
      evidence: attestationEvidenceFromCanonicalOutput(terminalOutput),
      expectedCoprocessorCount: COPROCESSOR_COUNT,
    });

    const plaintexts = await decryptMaterializationFixture(
      this.instances.alice,
      this.signers.alice,
      run.contractAddress,
      run.handles,
    );
    for (const label of FIXTURE_HANDLE_LABELS) {
      expect(plaintexts[label as FixtureHandleLabel], `plaintext mismatch for ${label}`).to.eq(
        FIXTURE_EXPECTED_PLAINTEXTS[label],
      );
    }
    emitAssertions('MAT-03-PLAINTEXT-ORACLE', ['correctness'], 'Every fixture handle decrypted to its expected arithmetic result.');
    // The plaintext oracle is a case in its own right (MAT-03): byte agreement
    // across operators is agreement on the right value only if the value
    // decrypts to the expected arithmetic result, and it is the only oracle
    // shared across backends. Announced explicitly so a run that agreed on
    // bytes but never reached the decryption is recorded as NOT_RUN rather than
    // folded into the byte case's green.
    console.info(
      `[materialization-consensus] plaintext oracle: ${FIXTURE_HANDLE_LABELS.length} label(s) decrypt to ` +
        'the expected values',
    );
    completedBodies += 1;
  });

  it('converges on same-sourcing aliases and pins mixed sourcing to distinct handles', async function () {
    const { ALIAS_FIXTURE_EXPECTED_PLAINTEXTS, deployAliasFixture, runAliasSameBlock } = await import('./aliasFixture');
    const deployment = await deployAliasFixture(this.signers.alice);
    const run = await runAliasSameBlock(deployment.contract);

    // Under the minted-in-transaction discriminant, sourcing is part of the
    // handle. The two storage combines alias each other (identical boundary
    // sourcing), and `combineLocal`'s trivial encrypts alias
    // `produceInputs`' outputs — but its add, consuming operands minted in
    // its own transaction, folds zero boundary bits and must mint a handle
    // DISTINCT from `combined`: representation-mixing aliases can no longer
    // collide, which is exactly what makes the surviving collisions
    // byte-safe.
    expect(run.handles.combinedSecond.toLowerCase(), 'same-sourcing adds must alias').to.eq(
      run.handles.combined.toLowerCase(),
    );
    expect(run.handles.combinedLocal.toLowerCase(), 'mixed sourcing must mint a distinct handle').to.not.eq(
      run.handles.combined.toLowerCase(),
    );

    const expectedHandles = [run.handles.inputB, run.handles.inputC, run.handles.combined, run.handles.combinedLocal];
    const reports = await waitForConsensusDatabaseReports(databaseUrls, expectedHandles, {
      timeoutMs: 10 * 60_000,
      expectedProducers: {
        [run.handles.inputB.toLowerCase()]: 2,
        [run.handles.inputC.toLowerCase()]: 2,
        [run.handles.combined.toLowerCase()]: 2,
        [run.handles.combinedLocal.toLowerCase()]: 1,
      },
    });

    for (const handle of expectedHandles) formatHandles.add(handle);

    // The four producing transactions must be completely executed with the
    // exact per-transaction row counts: 2 trivial encrypts; 1 add reading
    // storage; 1 aliased add reading storage; 2 trivial encrypts + 1 add
    // recomputed locally.
    const expectedTotals = new Map<string, number>([
      [run.produceTxHash.toLowerCase(), 2],
      [run.storageTxHash.toLowerCase(), 1],
      [run.storageAgainTxHash.toLowerCase(), 1],
      [run.localTxHash.toLowerCase(), 3],
    ]);
    for (const report of reports) {
      expect(report.transactions, `${report.databaseUrl} must report all four producing transactions`).to.have.length(
        expectedTotals.size,
      );
      for (const transaction of report.transactions) {
        const id = `0x${transaction.transactionId.toString('hex')}`.toLowerCase();
        const expectedTotal = expectedTotals.get(id);
        expect(expectedTotal, `${report.databaseUrl} reported unexpected producing transaction ${id}`).to.not.be
          .undefined;
        expect(transaction.totalCount, `${id} row count`).to.eq(expectedTotal);
        expect(transaction.completedCount, `${id} completion`).to.eq(expectedTotal);
        expect(transaction.errorCount, `${id} errors`).to.eq(0);
        expect(transaction.blockNumber, `${id} block height`).to.eq(run.blockNumber);
      }
    }

    const consensus = await waitForConsensus(GATEWAY_RPC_URL, CIPHERTEXT_COMMITS_ADDRESS, run.handles.combined);
    expect(consensus, 'on-chain quorum must form for the aliased output').to.not.be.null;
    expect(new Set(consensus!.senders.map((sender) => sender.toLowerCase())).size).to.eq(CONSENSUS_THRESHOLD);
    // Only the aliased output's quorum event is awaited here; the full
    // per-output coverage is the first test's responsibility.
    assertConsensusEventBindings(reports, [consensus!], { expectComplete: false });

    const combinedOutput = reports[0].outputs.find(
      (output) => `0x${output.handle.toString('hex')}`.toLowerCase() === run.handles.combined.toLowerCase(),
    );
    await waitForKmsNamespaceAttestationReadiness({
      gatewayRpcUrl: GATEWAY_RPC_URL,
      gatewayConfigAddress: GATEWAY_CONFIG_ADDRESS,
      evidence: attestationEvidenceFromCanonicalOutput(combinedOutput!),
      expectedCoprocessorCount: COPROCESSOR_COUNT,
    });
    const plaintext = await this.instances.alice.userDecryptSingleHandle({
      handle: run.handles.combined,
      contractAddress: deployment.contractAddress,
      signer: this.signers.alice,
    });
    expect(plaintext, 'aliased output plaintext').to.eq(ALIAS_FIXTURE_EXPECTED_PLAINTEXTS.combined);
    emitAssertions('MAT-02-ALIAS-SOURCING', ['bytes', 'safety'], 'Aliased handles have one canonical value/storage row, exact completed producer counts, and mixed sourcing produced distinct handles.');
    completedBodies += 1;
  });
});
