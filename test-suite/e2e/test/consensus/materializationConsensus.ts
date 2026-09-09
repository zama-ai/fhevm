/**
 * CPU/GPU homogeneous byte-consensus gate for transaction-boundary
 * ciphertext materialization.  Run this file once in a CPU topology and once
 * in a GPU topology.  It intentionally never combines their database
 * snapshots: FFT backends may produce different ciphertext bytes, while
 * user-decrypted plaintexts remain the cross-backend oracle.
 */
import { expect } from 'chai';

import {
  type ConsensusDatabaseReport,
  assertConsensusEventBindings,
  attestationEvidenceFromCanonicalOutput,
  getCoprocessorDbUrls,
  waitForConsensus,
  waitForConsensusDatabaseReports,
  waitForDatabaseReadiness,
  waitForKmsNamespaceAttestationReadiness,
} from './helpers';
import type { MaterializationFixtureRun } from './materializationFixture';
import {
  FIXTURE_EXPECTED_PLAINTEXTS,
  FIXTURE_HANDLE_LABELS,
  FIXTURE_PRODUCED_OUTPUT_LABELS,
  FIXTURE_TRANSACTIONS,
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

function required(value: string, name: string): string {
  if (!value) throw new Error(`${name} must be set for the materialization consensus gate`);
  return value;
}

/**
 * The transaction is the materialization boundary: every produced output of
 * one fixture transaction must carry that transaction's provenance, the four
 * fixture transactions must stay distinct, land on the expected L1 blocks,
 * and be completely executed (exact per-transaction row counts).
 */
function assertFixtureTransactionShape(
  reports: readonly ConsensusDatabaseReport[],
  run: MaterializationFixtureRun,
): void {
  for (const report of reports) {
    const outputByHandle = new Map(report.outputs.map((output) => [`0x${output.handle.toString('hex')}`, output]));
    const completionById = new Map(
      report.transactions.map((transaction) => [`0x${transaction.transactionId.toString('hex')}`, transaction]),
    );
    const seenTransactionIds = new Set<string>();

    for (const transaction of FIXTURE_TRANSACTIONS) {
      const outputs = transaction.producedLabels.map((label) => {
        const output = outputByHandle.get(run.handles[label].toLowerCase());
        expect(output, `${report.databaseUrl} is missing ${label}`).to.not.be.undefined;
        return output!;
      });
      const transactionId = outputs[0].transactionId;
      const transactionIdHex = `0x${transactionId.toString('hex')}`;
      seenTransactionIds.add(transactionIdHex);

      const expectedBlockNumber = transaction.block === 'terminal' ? run.terminalBlockNumber : run.sameBlockNumber;
      for (const output of outputs) {
        expect(
          output.transactionId.equals(transactionId),
          `${report.databaseUrl} split ${transaction.name} across producing transactions`,
        ).to.eq(true);
        expect(
          output.blockNumber,
          `${report.databaseUrl} assigned ${transaction.name} output to the wrong canonical block height`,
        ).to.eq(expectedBlockNumber);
      }

      const completion = completionById.get(transactionIdHex);
      expect(completion, `${report.databaseUrl} must report completion for ${transaction.name}`).to.not.be.undefined;
      expect(completion!.totalCount, `${transaction.name} must persist its exact computation row count`).to.eq(
        transaction.exactComputationCount,
      );
      expect(completion!.completedCount, `${transaction.name} must be completely executed at quiescence`).to.eq(
        transaction.exactComputationCount,
      );
      expect(completion!.errorCount, `${transaction.name} must have no errored computations`).to.eq(0);
      expect(completion!.blockNumber, `${transaction.name} completion has the wrong block height`).to.eq(
        expectedBlockNumber,
      );
    }

    expect(
      seenTransactionIds.size,
      'the staged, derived, independent, and terminal graphs must stay distinct transactions',
    ).to.eq(FIXTURE_TRANSACTIONS.length);
    expect(report.transactions).to.have.length(FIXTURE_TRANSACTIONS.length);
  }
}

describe('Materialization byte consensus', function () {
  this.timeout(15 * 60_000);

  let databaseUrls: string[];

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
    const execution = {
      softwareRevision: required(process.env.CONSENSUS_SOFTWARE_REVISION ?? '', 'CONSENSUS_SOFTWARE_REVISION'),
      backendClass: required(process.env.CONSENSUS_BACKEND_CLASS ?? '', 'CONSENSUS_BACKEND_CLASS'),
      hardwareClass: required(process.env.CONSENSUS_HARDWARE_CLASS ?? '', 'CONSENSUS_HARDWARE_CLASS'),
    };
    required(GATEWAY_RPC_URL, 'GATEWAY_RPC_URL');
    required(GATEWAY_CONFIG_ADDRESS, 'GATEWAY_CONFIG_ADDRESS');
    required(CIPHERTEXT_COMMITS_ADDRESS, 'CIPHERTEXT_COMMITS_ADDRESS');
    console.info(`[materialization-consensus] homogeneous execution class ${JSON.stringify(execution)}`);

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
    databaseUrls = getCoprocessorDbUrls(COPROCESSOR_COUNT);
    await waitForDatabaseReadiness(databaseUrls);
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
    assertFixtureTransactionShape(reports, run);

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
      expect(senders, `${label} consensus must contain the configured quorum`).to.have.length(CONSENSUS_THRESHOLD);
      expect(new Set(senders).size, `${label} consensus must contain unique submitters`).to.eq(CONSENSUS_THRESHOLD);
    }
    assertConsensusEventBindings(reports, consensuses as Exclude<(typeof consensuses)[number], null>[]);

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
  });
});
