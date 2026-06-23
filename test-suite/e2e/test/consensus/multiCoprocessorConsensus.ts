/**
 * E1: Multi-Coprocessor Consensus Tests (Single-Anvil Foundation)
 *
 * These tests verify consensus behavior across 3 coprocessor instances
 * sharing a single Anvil chain with isolated databases.
 *
 * Prerequisites:
 *   - Stack running via: ./fhevm-cli up --scenario three-of-three
 *   - Environment variables set (see helpers.ts for DATABASE_URL conventions)
 *
 * Run:
 *   ./fhevm-cli test --grep "Multi-Coprocessor Consensus"
 */
import { expect } from 'chai';
import hre, { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { deployEncryptedERC20Fixture } from '../encryptedERC20/EncryptedERC20.fixture';
import { ignoreWatchdogCiphertextHandle } from '../consensusWatchdog';

import {
  waitForConsensus,
  waitForGwListenerBlock,
  getSubmissions,
  queryDigests,
  findConsensusDigestRow,
  waitForConsensusDigestRows,
  waitForBranchCiphertexts,
  injectDigestRow,
  tamperDigestRows,
  restoreDigestRows,
  restoreArtificialDigestRows,
  dockerStop,
  dockerStart,
  dockerRestart,
  dockerPause,
  dockerUnpause,
  sleep,
  scrapeMetricValues,
  metricIncreased,
  waitForMetricIncrease,
  latestDriftRevertSignal,
  waitForDriftRevertSignalAtOrAfter,
  queryHandleBlockNumber,
  deleteDigestRowByBranchContext,
  getCoprocessorDbUrls,
  getCoprocessorMetricsUrls,
  scrapeRequiredMetric,
  handleToHex,
  containerName,
  toSafeNumber,
} from './helpers';

// ---------------------------------------------------------------------------
// Configuration from environment
// ---------------------------------------------------------------------------

const GATEWAY_RPC_URL = process.env.GATEWAY_RPC_URL || '';
const CIPHERTEXT_COMMITS_ADDRESS = process.env.CIPHERTEXT_COMMITS_ADDRESS || '';
const COPROCESSOR_COUNT = parseInt(process.env.COPROCESSOR_COUNT || process.env.NUM_COPROCESSORS || '3', 10);
const CONSENSUS_THRESHOLD = parseInt(
  process.env.CONSENSUS_THRESHOLD || process.env.COPROCESSOR_THRESHOLD || String(COPROCESSOR_COUNT),
  10,
);
const HOST_CHAIN_ID = toSafeNumber(
  process.env.CHAIN_ID_HOST || process.env.CHAIN_ID || '12345',
  'HOST_CHAIN_ID',
);
const CONSENSUS_TIMEOUT_DETECTION_MS = parseInt(
  process.env.CONSENSUS_TIMEOUT_DETECTION_MS || '360000',
  10,
);
const MISSING_SUBMISSION_DETECTION_MS = parseInt(
  process.env.MISSING_SUBMISSION_DETECTION_MS || '90000',
  10,
);
const RESTORE_STALE_ARTIFICIAL_DIGEST_ROWS = process.env.RESTORE_STALE_ARTIFICIAL_DIGEST_ROWS === '1';

function requireEnv(): void {
  if (!GATEWAY_RPC_URL) throw new Error('GATEWAY_RPC_URL not set');
  if (!CIPHERTEXT_COMMITS_ADDRESS) throw new Error('CIPHERTEXT_COMMITS_ADDRESS not set');
}

describe('Multi-Coprocessor Consensus (E1)', function () {
  this.timeout(600_000); // 10 minutes per test

  let dbUrls: string[];
  let metricsUrls: string[];

  before(async function () {
    requireEnv();
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    dbUrls = getCoprocessorDbUrls(COPROCESSOR_COUNT);
    metricsUrls = getCoprocessorMetricsUrls(COPROCESSOR_COUNT);
    const cleanup = await restoreArtificialDigestRows(dbUrls, RESTORE_STALE_ARTIFICIAL_DIGEST_ROWS);
    if (!RESTORE_STALE_ARTIFICIAL_DIGEST_ROWS) {
      expect(
        cleanup.repaired,
        'repairable stale artificial digest rows found; rerun with RESTORE_STALE_ARTIFICIAL_DIGEST_ROWS=1 to repair them',
      ).to.eq(0);
      expect(
        cleanup.unrepaired,
        'unrepairable stale artificial digest rows found',
      ).to.eq(0);
    } else if (cleanup.repaired > 0 || cleanup.unrepaired > 0) {
      console.warn(
        `[consensus] repaired ${cleanup.repaired} stale artificial digest row(s); ${cleanup.unrepaired} unrepaired`,
      );
      expect(cleanup.unrepaired, 'stale artificial digest rows must be repairable from peer DBs').to.eq(0);
    }
  });

  // -------------------------------------------------------------------------
  // C1: Normal consensus — all 3 agree
  // -------------------------------------------------------------------------

  describe('C1: Normal consensus', function () {
    it('should reach 3/3 consensus on an encrypted operation', async function () {
      // Deploy contract and mint — this triggers FHE computation.
      const contract = await deployEncryptedERC20Fixture();
      const tx = await contract.mint(1000);
      const receipt = await tx.wait();
      expect(receipt?.status).to.eq(1);

      // The mint produces a ciphertext handle. The consensus watchdog
      // (running as a Mocha root hook) automatically verifies that all
      // coprocessors submit matching digests and consensus is reached.
      //
      // Explicit assertion: query each coprocessor's DB and verify
      // all 3 store the same digest for the balance handle.
      const balanceHandle = handleToHex(await contract.balanceOf(this.signers.alice));

      // Wait for consensus on the gateway.
      const consensus = await waitForConsensus(
        GATEWAY_RPC_URL,
        CIPHERTEXT_COMMITS_ADDRESS,
        balanceHandle,
      );
      expect(consensus, 'consensus event must be emitted').to.not.be.null;
      expect(consensus!.senders.length).to.eq(CONSENSUS_THRESHOLD,
        `${CONSENSUS_THRESHOLD}/${COPROCESSOR_COUNT} senders must appear in consensus`);

      // Cross-database digest comparison.
      const allDigests = await waitForConsensusDigestRows(
        dbUrls,
        balanceHandle,
        consensus!.ciphertextDigest,
        consensus!.snsCiphertextDigest,
      );
      // Each DB should have at least one row for this handle.
      for (let i = 0; i < COPROCESSOR_COUNT; i++) {
        expect(allDigests[i].length, `coprocessor ${i} must have digest row`).to.be.gte(1);
      }
      // All DBs must contain the row that matches the emitted consensus digest.
      for (let i = 0; i < COPROCESSOR_COUNT; i++) {
        const row = findConsensusDigestRow(
          allDigests[i],
          consensus!.ciphertextDigest,
          consensus!.snsCiphertextDigest,
        );
        expect(row, `coprocessor ${i} must have the consensus digest row`).to.not.be.undefined;
      }
    });
  });

  // -------------------------------------------------------------------------
  // C4a: Consensus timeout — one coprocessor offline
  // -------------------------------------------------------------------------

  describe('C4a: Consensus timeout', function () {
    it('should detect consensus timeout when one coprocessor is offline', async function () {
      if (CONSENSUS_THRESHOLD < COPROCESSOR_COUNT) {
        this.skip(); // This 3/3 timeout scenario is covered separately from 2/3 missing-submission checks.
        return;
      }

      // Stop coprocessor 2's compute and submit pipeline.
      const worker = containerName(2, 'tfhe-worker');
      const sender = containerName(2, 'transaction-sender');
      const timeoutCountBefore = await scrapeMetricValues(
        metricsUrls,
        'coprocessor_gw_listener_consensus_timeout_counter',
      );
      await dockerStop(worker, sender);

      // Deploy outside try so it's accessible in finally for catch-up.
      const contract = await deployEncryptedERC20Fixture();

      try {
        const tx = await contract.mint(500);
        await tx.wait();

        const balanceHandle = handleToHex(await contract.balanceOf(this.signers.alice));

        // Wait a short time for coprocessors 0 and 1 to submit.
        await sleep(30_000);

        // Should have 2 submissions but no consensus (need 3/3).
        const submissions = await getSubmissions(
          GATEWAY_RPC_URL,
          CIPHERTEXT_COMMITS_ADDRESS,
          balanceHandle,
        );
        expect(submissions.length).to.be.gte(2, 'at least 2 submissions expected');

        const consensus = await waitForConsensus(
          GATEWAY_RPC_URL,
          CIPHERTEXT_COMMITS_ADDRESS,
          balanceHandle,
          5_000, // short timeout — we expect no consensus
        );
        expect(consensus, 'consensus must NOT be reached with only 2/3').to.be.null;

        // Check that at least one gw-listener detected the no-consensus case.
        const timeoutCountAfter = await waitForMetricIncrease(
          metricsUrls,
          'coprocessor_gw_listener_consensus_timeout_counter',
          timeoutCountBefore,
          CONSENSUS_TIMEOUT_DETECTION_MS,
        );
        expect(
          metricIncreased(timeoutCountBefore, timeoutCountAfter),
          'CONSENSUS_TIMEOUT_COUNTER must increment during this scenario',
        ).to.be.true;
      } finally {
        // Restore coprocessor 2 and verify catch-up consensus.
        await dockerStart(worker, sender);
        await sleep(30_000);

        const balanceHandle = handleToHex(await contract.balanceOf(this.signers.alice));
        const consensusAfterCatchUp = await waitForConsensus(
          GATEWAY_RPC_URL,
          CIPHERTEXT_COMMITS_ADDRESS,
          balanceHandle,
          60_000,
        );
        expect(consensusAfterCatchUp, 'consensus must be reached after catch-up').to.not.be.null;
      }
    });
  });

  // -------------------------------------------------------------------------
  // C4b: Missing submission after partial-quorum consensus
  // NOTE: Requires the two-of-three scenario (threshold=2).
  //       Run separately: ./fhevm-cli up --scenario two-of-three
  // -------------------------------------------------------------------------

  describe('C4b: Missing submission after consensus', function () {
    it('should detect missing submission when consensus is reached without all senders', async function () {
      // This test only applies under a 2/3 threshold. Skip if threshold is 3.
      if (CONSENSUS_THRESHOLD >= COPROCESSOR_COUNT) {
        this.skip(); // Skip in 3/3 mode; run under two-of-three scenario
        return;
      }

      const worker = containerName(2, 'tfhe-worker');
      const sender = containerName(2, 'transaction-sender');
      const missingCountBefore = await scrapeMetricValues(
        metricsUrls,
        'coprocessor_gw_listener_missing_submission_counter',
      );
      await dockerStop(worker, sender);

      try {
        const contract = await deployEncryptedERC20Fixture();
        const tx = await contract.mint(750);
        await tx.wait();

        const balanceHandle = handleToHex(await contract.balanceOf(this.signers.alice));

        // With threshold=2, coprocessors 0 and 1 reach consensus.
        const consensus = await waitForConsensus(
          GATEWAY_RPC_URL,
          CIPHERTEXT_COMMITS_ADDRESS,
          balanceHandle,
        );
        expect(consensus, 'consensus must be reached at 2/3 threshold').to.not.be.null;
        expect(consensus!.senders.length).to.eq(CONSENSUS_THRESHOLD,
          `${CONSENSUS_THRESHOLD}/${COPROCESSOR_COUNT} senders in consensus event`);

        const missingCountAfter = await waitForMetricIncrease(
          metricsUrls,
          'coprocessor_gw_listener_missing_submission_counter',
          missingCountBefore,
          MISSING_SUBMISSION_DETECTION_MS,
        );
        expect(
          metricIncreased(missingCountBefore, missingCountAfter),
          'MISSING_SUBMISSION_COUNTER must increment',
        ).to.be.true;
      } finally {
        await dockerStart(worker, sender);
        await sleep(15_000);
      }
    });
  });

  // -------------------------------------------------------------------------
  // C5: Block-scoped M2 consensus
  // -------------------------------------------------------------------------

  describe('C5: Block-scoped execution consensus', function () {
    it('should reach consensus with M2-shaped block: prior-block boundary + two independent txns', async function () {
      // Block-scoped execution is the only runtime path.
      //
      // Block shape that exercises block-scoped materialization:
      //   Block N:   mint(1000) → handle H (balance ciphertext)
      //   Block N+1: transfer(Alice→Bob, 100) AND transfer(Alice→Carol, 200)
      //              Both consume H as a boundary input from block N.
      //              Two independent transactions → exercises cross-partition dedup.

      const contract = await deployEncryptedERC20Fixture();

      // Block N: mint — creates balance handle H.
      const mintTx = await contract.mint(1000);
      await mintTx.wait();

      // Wait for block N to be processed by all 3 coprocessors.
      const balanceHandleAlice = handleToHex(await contract.balanceOf(this.signers.alice));
      const mintConsensus = await waitForConsensus(
        GATEWAY_RPC_URL,
        CIPHERTEXT_COMMITS_ADDRESS,
        balanceHandleAlice,
        120_000,
      );
      expect(mintConsensus, 'consensus on Alice initial balance').to.not.be.null;
      await waitForConsensusDigestRows(
        dbUrls,
        balanceHandleAlice,
        mintConsensus!.ciphertextDigest,
        mintConsensus!.snsCiphertextDigest,
        120_000,
      );
      const boundaryReady = await waitForBranchCiphertexts(dbUrls, balanceHandleAlice, 120_000);
      expect(
        boundaryReady.every(Boolean),
        'Alice balance ciphertext must be materialized in every coprocessor DB',
      ).to.eq(true);

      // Block N+1: two independent transfers consuming H.
      // Transfer 1: Alice → Bob (100)
      const contractAddress = await contract.getAddress();
      const encrypted1 = await this.instances.alice.encryptUint64({
        value: 100,
        contractAddress,
        userAddress: this.signers.alice.address,
      });
      await ethers.provider.send('evm_setIntervalMining', [0]);
      await ethers.provider.send('evm_setAutomine', [false]);

      let tx1;
      let tx2;
      try {
        tx1 = await contract['transfer(address,bytes32,bytes)'](
          this.signers.bob.address,
          encrypted1.handles[0],
          encrypted1.inputProof,
          { gasLimit: 6_000_000 },
        );
        // Transfer 2: Alice → Carol (200) — independent transaction.
        const encrypted2 = await this.instances.alice.encryptUint64({
          value: 200,
          contractAddress,
          userAddress: this.signers.alice.address,
        });
        tx2 = await contract['transfer(address,bytes32,bytes)'](
          this.signers.carol.address,
          encrypted2.handles[0],
          encrypted2.inputProof,
          { gasLimit: 6_000_000 },
        );
        await ethers.provider.send('evm_mine');
      } finally {
        await ethers.provider.send('evm_setAutomine', [true]);
        await ethers.provider.send('evm_setIntervalMining', [1]);
      }

      const [receipt1, receipt2] = await Promise.all([tx1!.wait(), tx2!.wait()]);
      expect(receipt1?.blockNumber).to.eq(receipt2?.blockNumber,
        'both transfers must land in the same block for the C5 block-scoped scenario');

      // Wait for consensus on both output handles.
      const balanceHandleBob = handleToHex(await contract.balanceOf(this.signers.bob));
      const balanceHandleCarol = handleToHex(await contract.balanceOf(this.signers.carol));

      const [consensusBob, consensusCarol] = await Promise.all([
        waitForConsensus(GATEWAY_RPC_URL, CIPHERTEXT_COMMITS_ADDRESS, balanceHandleBob),
        waitForConsensus(GATEWAY_RPC_URL, CIPHERTEXT_COMMITS_ADDRESS, balanceHandleCarol),
      ]);

      expect(consensusBob, 'consensus on Bob balance').to.not.be.null;
      expect(consensusBob!.senders.length).to.eq(CONSENSUS_THRESHOLD);
      expect(consensusCarol, 'consensus on Carol balance').to.not.be.null;
      expect(consensusCarol!.senders.length).to.eq(CONSENSUS_THRESHOLD);

      // All 3 DBs must agree on both outputs.
      const digestsBob = await waitForConsensusDigestRows(
        dbUrls,
        balanceHandleBob,
        consensusBob!.ciphertextDigest,
        consensusBob!.snsCiphertextDigest,
      );
      const digestsCarol = await waitForConsensusDigestRows(
        dbUrls,
        balanceHandleCarol,
        consensusCarol!.ciphertextDigest,
        consensusCarol!.snsCiphertextDigest,
      );
      for (let i = 0; i < COPROCESSOR_COUNT; i++) {
        expect(findConsensusDigestRow(
          digestsBob[i],
          consensusBob!.ciphertextDigest,
          consensusBob!.snsCiphertextDigest,
        ), `coprocessor ${i} Bob digest must include consensus row`).to.not.be.undefined;
        expect(findConsensusDigestRow(
          digestsCarol[i],
          consensusCarol!.ciphertextDigest,
          consensusCarol!.snsCiphertextDigest,
        ), `coprocessor ${i} Carol digest must include consensus row`).to.not.be.undefined;
      }
    });
  });

  // -------------------------------------------------------------------------
  // C6: gw-listener restart during in-flight consensus
  // -------------------------------------------------------------------------

  describe('C6: gw-listener restart in-flight', function () {
    it('should preserve correct behavior after gw-listener restart', async function () {
      const contract = await deployEncryptedERC20Fixture();
      const tx = await contract.mint(2000);
      await tx.wait();

      const balanceHandle = handleToHex(await contract.balanceOf(this.signers.alice));

      // Wait briefly for some submissions to appear.
      await sleep(10_000);

      // Restart coprocessor 0's gw-listener mid-consensus.
      const gwListener = containerName(0, 'gw-listener');
      await dockerRestart(gwListener);

      // Wait for replay and consensus completion.
      const consensus = await waitForConsensus(
        GATEWAY_RPC_URL,
        CIPHERTEXT_COMMITS_ADDRESS,
        balanceHandle,
      );
      expect(consensus, 'consensus must be reached after gw-listener restart').to.not.be.null;

      // The consensus watchdog root hook will also verify no
      // false drift/stall alerts after the test completes.
    });
  });

  // -------------------------------------------------------------------------
  // C2a: Consensus-layer divergence (DB injection)
  // -------------------------------------------------------------------------

  describe('C2a: Consensus-layer divergence', function () {
    it('should detect divergence when one coprocessor has a tampered digest', async function () {
      if (CONSENSUS_THRESHOLD < COPROCESSOR_COUNT) {
        this.skip(); // With partial quorum, the two matching submissions are expected to reach consensus.
        return;
      }

      // This test intentionally injects divergence and explicitly ignores
      // only the handle that is expected to diverge.
      // Pause coprocessor 2's transaction-sender so we can inject
      // before it submits.
      const sender2 = containerName(2, 'transaction-sender');
      let originalDigestRows: Awaited<ReturnType<typeof queryDigests>> = [];
      await dockerPause(sender2);

      try {
        const contract = await deployEncryptedERC20Fixture();
        const tx = await contract.mint(3000);
        await tx.wait();

        const balanceHandle = handleToHex(await contract.balanceOf(this.signers.alice));

        // Wait for coprocessor 2 to compute (but not submit).
        await sleep(15_000);

        // Tamper coprocessor 2's digest.
        originalDigestRows = await queryDigests(dbUrls[2], balanceHandle);
        expect(originalDigestRows.length, 'coprocessor 2 must have digest rows before tamper').to.be.greaterThan(0);
        expect(
          originalDigestRows.every((row) => row.ciphertext && row.ciphertext128),
          'coprocessor 2 digest rows must be materialized before tamper',
        ).to.eq(true);
        await tamperDigestRows(
          dbUrls[2],
          originalDigestRows,
          Buffer.alloc(32, 0xFF), // fake ciphertext digest
          Buffer.alloc(32, 0xEE), // fake ciphertext128 digest
        );

        // Unpause — coprocessor 2 submits the tampered digest.
        await dockerUnpause(sender2);

        // Wait for all submissions.
        await sleep(30_000);

        // Consensus should NOT be reached (2 agree, 1 diverges).
        const consensus = await waitForConsensus(
          GATEWAY_RPC_URL,
          CIPHERTEXT_COMMITS_ADDRESS,
          balanceHandle,
          5_000,
        );
        expect(consensus, 'consensus must NOT be reached with tampered digest').to.be.null;

        // Verify submissions show divergence.
        const submissions = await getSubmissions(
          GATEWAY_RPC_URL,
          CIPHERTEXT_COMMITS_ADDRESS,
          balanceHandle,
        );
        expect(submissions.length).to.be.gte(3, 'all 3 must have submitted');

        const digests = new Set(submissions.map((s) => s.ciphertextDigest));
        expect(digests.size).to.be.gte(2, 'at least 2 distinct digests expected (divergence)');
        ignoreWatchdogCiphertextHandle(balanceHandle);

        // Check DRIFT_DETECTED_COUNTER on coprocessor 0's gw-listener.
        const driftCount = await scrapeRequiredMetric(
          metricsUrls[0],
          'coprocessor_gw_listener_drift_detected_counter',
        );
        expect(driftCount).to.be.gte(1, 'DRIFT_DETECTED_COUNTER must increment on divergence');
      } finally {
        try {
          await restoreDigestRows(dbUrls[2], originalDigestRows);
        } finally {
          // Ensure sender is running for subsequent tests even if DB restore fails.
          try { await dockerUnpause(sender2); } catch { /* may already be unpaused */ }
        }
      }
    });
  });

  // -------------------------------------------------------------------------
  // C7a: Multi-row local-consensus — one row matches
  // -------------------------------------------------------------------------

  describe('C7: Multi-row local-consensus (M4 drift detector)', function () {
    // C7a and C7b test multi-row drift detection, which requires the M3
    // schema migration that adds producer_block_hash to the primary key,
    // enabling multiple digest rows per handle. Check for the column at
    // runtime and skip when the schema doesn't support it.
    let schemaSupportsMultiRow: boolean;

    before(async function () {
      const { Pool } = require('pg');
      const pool = new Pool({ connectionString: dbUrls[0], max: 1 });
      try {
        const res = await pool.query(
          "SELECT 1 FROM information_schema.columns WHERE table_name='ciphertext_digest_branch' AND column_name='producer_block_hash'",
        );
        schemaSupportsMultiRow = res.rows.length > 0;
      } finally {
        await pool.end();
      }
    });

    it('C7a: should not report drift when one local row matches consensus', async function () {
      if (!schemaSupportsMultiRow) {
        this.skip(); // Schema lacks producer_block_hash; multi-row not possible
        return;
      }

      // Snapshot the drift metric BEFORE the test.
      const driftBefore = await scrapeRequiredMetric(
        metricsUrls[0],
        'coprocessor_gw_listener_drift_detected_counter',
      );

      // Pause coprocessor 0's gw-listener before consensus is processed.
      const gwListener0 = containerName(0, 'gw-listener');
      const injectedProducerBlockHash = '0x' + 'AB'.repeat(32);
      let balanceHandle: string | null = null;
      await dockerPause(gwListener0);

      try {
        const contract = await deployEncryptedERC20Fixture();
        const tx = await contract.mint(4000);
        await tx.wait();

        balanceHandle = handleToHex(await contract.balanceOf(this.signers.alice));

        const consensus = await waitForConsensus(
          GATEWAY_RPC_URL,
          CIPHERTEXT_COMMITS_ADDRESS,
          balanceHandle,
          120_000,
        );
        expect(consensus, 'consensus must be emitted while gw-listener is paused').to.not.be.null;

        // Inject an extra ciphertext_digest row simulating fork residue.
        await injectDigestRow(
          dbUrls[0],
          balanceHandle,
          injectedProducerBlockHash,
          Buffer.alloc(32, 0x11),
          Buffer.alloc(32, 0x22),
        );

        const rows = await queryDigests(dbUrls[0], balanceHandle);
        expect(rows.length).to.be.gte(2, 'coprocessor 0 must have at least 2 digest rows');

        // Unpause — gw-listener processes consensus with 2 local rows.
        // One matches → no drift expected.
        await dockerUnpause(gwListener0);
        const processedBlock = await waitForGwListenerBlock(
          dbUrls[0],
          consensus!.blockNumber,
          60_000,
        );
        expect(
          processedBlock,
          'coprocessor 0 gw-listener must process the consensus event block before asserting drift',
        ).to.not.be.null;

        // Assert: drift metric did NOT increment.
        const driftAfter = await scrapeRequiredMetric(
          metricsUrls[0],
          'coprocessor_gw_listener_drift_detected_counter',
        );
        expect(driftAfter).to.eq(driftBefore,
          'DRIFT_DETECTED_COUNTER must not increment when one local row matches consensus');
      } finally {
        try {
          if (balanceHandle) {
            await deleteDigestRowByBranchContext(dbUrls[0], balanceHandle, injectedProducerBlockHash);
          }
        } finally {
          try { await dockerUnpause(gwListener0); } catch { /* may already be unpaused */ }
        }
      }
    });

    it('C7b: should have tampered local digest diverge from consensus', async function () {
      // This test intentionally tampers a local digest and explicitly ignores
      // only the handle that is expected to diverge.
      //
      // Strategy: pause gw-listener 0, let consensus reach on gateway,
      // tamper the local digest in coprocessor 0's DB, then verify that
      // the tampered digest does NOT match the consensus digest.

      const gwListener0 = containerName(0, 'gw-listener');
      const signalBefore = await latestDriftRevertSignal(dbUrls[0]);
      let originalDigestRows: Awaited<ReturnType<typeof queryDigests>> = [];
      if (signalBefore && ['pending', 'reverting'].includes(signalBefore.status)) {
        throw new Error(
          `C7b requires no in-flight drift signal, found ${signalBefore.status} signal ${signalBefore.id}`,
        );
      }
      await dockerPause(gwListener0);

      try {
        const contract = await deployEncryptedERC20Fixture();
        const tx = await contract.mint(5000);
        await tx.wait();

        const balanceHandle = handleToHex(await contract.balanceOf(this.signers.alice));

        // Wait for consensus to be emitted on the gateway.
        const consensus = await waitForConsensus(
          GATEWAY_RPC_URL,
          CIPHERTEXT_COMMITS_ADDRESS,
          balanceHandle,
          120_000,
        );
        expect(consensus, 'consensus must be emitted while gw-listener is paused').to.not.be.null;
        ignoreWatchdogCiphertextHandle(balanceHandle);

        // Tamper coprocessor 0's existing digest row.
        originalDigestRows = await queryDigests(dbUrls[0], balanceHandle);
        expect(originalDigestRows.length, 'coprocessor 0 must have digest rows before tamper').to.be.greaterThan(0);
        expect(
          originalDigestRows.every((row) => row.ciphertext && row.ciphertext128),
          'coprocessor 0 digest rows must be materialized before tamper',
        ).to.eq(true);
        const tamperedCt = Buffer.alloc(32, 0xAA);
        const tamperedCt128 = Buffer.alloc(32, 0xBB);
        await tamperDigestRows(
          dbUrls[0],
          originalDigestRows,
          tamperedCt,
          tamperedCt128,
        );

        // Verify tamper took effect.
        const rows = await queryDigests(dbUrls[0], balanceHandle);
        expect(rows.length).to.be.gte(1, 'coprocessor 0 must have a digest row');
        expect(rows[0].ciphertext!.equals(tamperedCt), 'digest must be tampered').to.be.true;

        // The consensus digest is from the legitimate computation.
        // The tampered local digest should NOT match.
        expect(
          rows[0].ciphertext!.toString('hex'),
          'tampered ciphertext digest must differ from consensus',
        ).to.not.eq(consensus!.ciphertextDigest.replace('0x', ''));

        const expectedBlockNumber = await queryHandleBlockNumber(
          dbUrls[0],
          balanceHandle,
          HOST_CHAIN_ID,
        );
        if (expectedBlockNumber === null) {
          throw new Error('tampered handle must have a branch computation block number');
        }

        // Unpause so the drift detector can run.
        await dockerUnpause(gwListener0);
        const minimumSignalId = (signalBefore?.id ?? 0) + 1;
        const signalAfter = await waitForDriftRevertSignalAtOrAfter(
          dbUrls[0],
          minimumSignalId,
          60_000,
          (signal) =>
            signal.host_chain_id === HOST_CHAIN_ID &&
            signal.offending_host_block_number <= expectedBlockNumber,
        );
        expect(signalAfter, 'drift revert signal must be recorded when local digest does not match consensus')
          .to.not.be.null;
        expect(signalAfter!.status).to.be.oneOf(['pending', 'reverting', 'done']);
      } finally {
        try {
          await restoreDigestRows(dbUrls[0], originalDigestRows);
        } finally {
          try { await dockerUnpause(gwListener0); } catch { /* may already be unpaused */ }
        }
      }
    });
  });
});
