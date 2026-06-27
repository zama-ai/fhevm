/**
 * E3: Real-Fork Consensus Tests (Dual-Anvil)
 *
 * These tests verify consensus behavior when coprocessors compute on
 * genuinely divergent chain histories:
 *
 * - C2b: Full-fork equivocation — different digests for the same handle
 * - C3:  Recovery after finalization — orphan cleanup restores consensus
 *
 * Prerequisites:
 *   - Stack running via: ./fhevm-cli up --scenario three-of-three-fork
 *   - Fork Anvil running: docker compose -p fhevm -f docker-compose/fork-anvil-docker-compose.yml up -d
 *
 * Run:
 *   CANONICAL_RPC_URL=http://host-node:8545 \
 *   FORK_RPC_URL=http://fork-anvil:8546 \
 *   ./fhevm-cli test --grep "Real-Fork Consensus"
 */
import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import { deployEncryptedERC20Fixture } from '../encryptedERC20/EncryptedERC20.fixture';
import { ignoreWatchdogCiphertextHandle } from '../consensusWatchdog';

import {
  waitForConsensus,
  getSubmissions,
  findConsensusDigestRow,
  waitForConsensusDigestRows,
  sleep,
  getCoprocessorDbUrls,
  getCoprocessorMetricsUrls,
  scrapeRequiredMetric,
  handleToHex,
  containerName,
  dockerStop,
  dockerStart,
  dockerRestart,
} from './helpers';

import {
  defaultForkConfig,
  syncAnvilState,
  getCanonicalProvider,
  getForkProvider,
  getSignerForProvider,
  verifyForkDivergence,
  advancePastFinality,
} from './forkHelper';

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

const GATEWAY_RPC_URL = process.env.GATEWAY_RPC_URL || '';
const CIPHERTEXT_COMMITS_ADDRESS = process.env.CIPHERTEXT_COMMITS_ADDRESS || '';
const COPROCESSOR_COUNT = 3;
const FINALITY_LAG = parseInt(process.env.FINALITY_LAG || '5', 10);

function requireEnv(): void {
  if (!GATEWAY_RPC_URL) throw new Error('GATEWAY_RPC_URL not set');
  if (!CIPHERTEXT_COMMITS_ADDRESS) throw new Error('CIPHERTEXT_COMMITS_ADDRESS not set');
}

interface DivergentForkWork {
  balanceHandle: string;
  forkBlockHash: string;
}

async function createDivergentForkWork(
  label: string,
  aliceAddress: string,
  canonicalMintAmount: number,
  forkMintAmount: number,
): Promise<DivergentForkWork> {
  const forkConfig = defaultForkConfig();
  const canonicalProvider = getCanonicalProvider(forkConfig);
  const forkProvider = getForkProvider(forkConfig);

  const contract = await deployEncryptedERC20Fixture();
  const contractAddress = await contract.getAddress();

  console.log(`[${label}] Syncing fork Anvil state after contract deployment...`);
  await syncAnvilState(forkConfig.canonicalRpcUrl, forkConfig.forkRpcUrl);

  const mintCanonical = await contract.mint(canonicalMintAmount);
  await mintCanonical.wait();

  const forkSigner = getSignerForProvider(forkProvider, 0);
  const contractOnFork = new ethers.Contract(
    contractAddress,
    contract.interface,
    forkSigner,
  );
  const mintFork = await contractOnFork.mint(forkMintAmount);
  const forkReceipt = await mintFork.wait();
  expect(forkReceipt, `[${label}] fork mint receipt`).to.not.be.null;

  const canonicalBlock = await canonicalProvider.getBlockNumber();
  let diverged = false;
  for (let b = canonicalBlock; b >= Math.max(1, canonicalBlock - 3); b--) {
    try {
      await verifyForkDivergence(b, forkConfig);
      diverged = true;
      console.log(`[${label}] Fork divergence confirmed at block ${b}`);
      break;
    } catch {
      // try earlier block
    }
  }
  expect(diverged, 'fork must have diverged: block hashes should differ').to.be.true;

  return {
    balanceHandle: handleToHex(await contract.balanceOf(aliceAddress)),
    forkBlockHash: forkReceipt!.blockHash,
  };
}

async function waitForDivergentSubmissions(
  label: string,
  balanceHandle: string,
  timeoutMs: number = 120_000,
): Promise<Awaited<ReturnType<typeof getSubmissions>>> {
  let submissions: Awaited<ReturnType<typeof getSubmissions>> = [];
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline && submissions.length < COPROCESSOR_COUNT) {
    await sleep(5_000);
    submissions = await getSubmissions(
      GATEWAY_RPC_URL,
      CIPHERTEXT_COMMITS_ADDRESS,
      balanceHandle,
    );
  }
  expect(submissions.length).to.eq(COPROCESSOR_COUNT,
    `[${label}] all ${COPROCESSOR_COUNT} coprocessors must submit`);

  const digests = new Set(submissions.map((s) => s.ciphertextDigest));
  expect(digests.size).to.be.gte(2,
    `[${label}] at least 2 distinct digests expected (fork divergence)`);

  return submissions;
}

describe('Real-Fork Consensus (E3)', function () {
  this.timeout(600_000); // 10 minutes

  let dbUrls: string[];
  let metricsUrls: string[];

  before(async function () {
    requireEnv();
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    dbUrls = getCoprocessorDbUrls(COPROCESSOR_COUNT);
    metricsUrls = getCoprocessorMetricsUrls(COPROCESSOR_COUNT);
  });

  // -------------------------------------------------------------------------
  // C2b: Full-fork equivocation
  // -------------------------------------------------------------------------

  describe('C2b: Full-fork equivocation', function () {
    it('should detect divergence when coprocessors compute on different fork branches', async function () {
      const forkWork = await createDivergentForkWork(
        'C2b',
        this.signers.alice.address,
        1000,
        2000,
      );
      const { balanceHandle } = forkWork;

      await waitForDivergentSubmissions('C2b', balanceHandle);

      // Step 7: Consensus must NOT be reached (no 3/3 match).
      const consensus = await waitForConsensus(
        GATEWAY_RPC_URL,
        CIPHERTEXT_COMMITS_ADDRESS,
        balanceHandle,
        5_000,
      );
      expect(consensus, 'consensus must NOT be reached under fork divergence').to.be.null;
      ignoreWatchdogCiphertextHandle(balanceHandle);

      // Step 8: Check DRIFT_DETECTED_COUNTER.
      const driftCount = await scrapeRequiredMetric(
        metricsUrls[0],
        'coprocessor_gw_listener_drift_detected_counter',
      );
      expect(driftCount).to.be.gte(1, 'DRIFT_DETECTED_COUNTER must increment');
    });
  });

  // -------------------------------------------------------------------------
  // C3: Recovery after finalization
  // -------------------------------------------------------------------------

  describe('C3: Recovery after finalization', function () {
    it('should restore consensus after switching coprocessor 2 to canonical chain', async function () {
      const forkConfig = defaultForkConfig();

      // Step 0: Create fork-local work inside this test so recovery does not
      // depend on C2b or any previous test having already produced orphanable rows.
      const divergentWork = await createDivergentForkWork(
        'C3',
        this.signers.alice.address,
        3000,
        4000,
      );
      await waitForDivergentSubmissions('C3', divergentWork.balanceHandle);
      const divergentConsensus = await waitForConsensus(
        GATEWAY_RPC_URL,
        CIPHERTEXT_COMMITS_ADDRESS,
        divergentWork.balanceHandle,
        5_000,
      );
      expect(divergentConsensus, 'C3 setup handle must not reach consensus before recovery').to.be.null;
      ignoreWatchdogCiphertextHandle(divergentWork.balanceHandle);
      const c3ForkBlockHash = Buffer.from(divergentWork.forkBlockHash.replace('0x', ''), 'hex');

      // Step 1: Advance canonical Anvil past finality.
      console.log(`[C3] Advancing canonical Anvil past finality (${FINALITY_LAG} blocks)...`);
      await advancePastFinality(FINALITY_LAG, forkConfig);

      // Step 2: Make the fork Anvil present the canonical chain.
      // Coprocessor 2 stays connected to fork-anvil, but now observes
      // canonical block hashes at the fork height and should detect a reorg.
      console.log('[C3] Resyncing fork Anvil to canonical chain state...');
      await syncAnvilState(forkConfig.canonicalRpcUrl, forkConfig.forkRpcUrl);

      const listener2 = containerName(2, 'host-listener');
      const poller2 = containerName(2, 'host-listener-poller');
      const worker2 = containerName(2, 'tfhe-worker');
      const sender2 = containerName(2, 'transaction-sender');

      // Restart chain-facing services to force fresh polling against the now-canonical fork Anvil.
      await dockerRestart(listener2, poller2, worker2, sender2);
      console.log('[C3] Coprocessor 2 services restarted against canonicalized fork Anvil.');

      // Step 3: Poll for orphaned blocks on coprocessor 2's DB.
      // The host-listener detects the reorg when it sees different
      // block hashes at the fork height and marks them orphaned.
      const { Pool } = await import('pg');
      let c3Orphan: { chain_id: string; block_hash: Buffer } | undefined;
      const orphanDeadline = Date.now() + 60_000;
      while (Date.now() < orphanDeadline) {
        const pool2 = new Pool({ connectionString: dbUrls[2], max: 1 });
        try {
          const result = await pool2.query(
            "SELECT chain_id, block_hash FROM host_chain_blocks_valid WHERE block_status = 'orphaned' AND block_hash = $1 LIMIT 1",
            [c3ForkBlockHash],
          );
          c3Orphan = result.rows[0];
          if (c3Orphan) break;
        } finally {
          await pool2.end();
        }
        await sleep(5_000);
      }
      console.log(`[C3] Coprocessor 2 orphaned block hash: ${divergentWork.forkBlockHash}`);
      expect(c3Orphan, 'coprocessor 2 must orphan the fork block created by C3').to.not.be.undefined;

      // Step 4: Verify orphan cleanup removed branch-B rows.
      // Wait for cleanup_orphaned_branch_state to run (triggered by finalization).
      await sleep(15_000);

      // Query coprocessor 2's DB for rows referencing orphaned block hashes.
      // After cleanup, computations/ciphertexts/allowed_handles/ciphertext_digest
      // rows for orphaned producer_block_hash values should be gone.
      const pool2Check = new Pool({ connectionString: dbUrls[2], max: 1 });
      try {
        const orphanHashList = [c3ForkBlockHash];
        const orphanChainId = c3Orphan!.chain_id;

        // Check computations: no rows should reference the C3 orphaned producer_block_hash.
        const compResult = await pool2Check.query(
          'SELECT count(*)::int AS cnt FROM computations WHERE host_chain_id = $1 AND producer_block_hash = ANY($2::bytea[])',
          [orphanChainId, orphanHashList],
        );
        expect(compResult.rows[0].cnt).to.eq(0,
          'C3 orphaned computations must be cleaned up on coprocessor 2');

        const compBranchResult = await pool2Check.query(
          'SELECT count(*)::int AS cnt FROM computations_branch WHERE host_chain_id = $1 AND producer_block_hash = ANY($2::bytea[])',
          [orphanChainId, orphanHashList],
        );
        expect(compBranchResult.rows[0].cnt).to.eq(0,
          'C3 orphaned computations_branch rows must be cleaned up on coprocessor 2');

        // Check ciphertext_digest.
        const digestResult = await pool2Check.query(
          'SELECT count(*)::int AS cnt FROM ciphertext_digest WHERE host_chain_id = $1 AND producer_block_hash = ANY($2::bytea[])',
          [orphanChainId, orphanHashList],
        );
        expect(digestResult.rows[0].cnt).to.eq(0,
          'C3 orphaned ciphertext_digest rows must be cleaned up on coprocessor 2');

        const digestBranchResult = await pool2Check.query(
          'SELECT count(*)::int AS cnt FROM ciphertext_digest_branch WHERE host_chain_id = $1 AND producer_block_hash = ANY($2::bytea[])',
          [orphanChainId, orphanHashList],
        );
        expect(digestBranchResult.rows[0].cnt).to.eq(0,
          'C3 orphaned ciphertext_digest_branch rows must be cleaned up on coprocessor 2');

        // Check ciphertexts.
        const ctResult = await pool2Check.query(
          'SELECT count(*)::int AS cnt FROM ciphertexts WHERE producer_block_hash = ANY($1::bytea[])',
          [orphanHashList],
        );
        expect(ctResult.rows[0].cnt).to.eq(0,
          'C3 orphaned ciphertexts must be cleaned up on coprocessor 2');

        const ctBranchResult = await pool2Check.query(
          'SELECT count(*)::int AS cnt FROM ciphertexts_branch WHERE producer_block_hash = ANY($1::bytea[])',
          [orphanHashList],
        );
        expect(ctBranchResult.rows[0].cnt).to.eq(0,
          'C3 orphaned ciphertexts_branch rows must be cleaned up on coprocessor 2');

        const ct128Result = await pool2Check.query(
          'SELECT count(*)::int AS cnt FROM ciphertexts128 WHERE producer_block_hash = ANY($1::bytea[])',
          [orphanHashList],
        );
        expect(ct128Result.rows[0].cnt).to.eq(0,
          'C3 orphaned ciphertexts128 rows must be cleaned up on coprocessor 2');

        const ct128BranchResult = await pool2Check.query(
          'SELECT count(*)::int AS cnt FROM ciphertexts128_branch WHERE producer_block_hash = ANY($1::bytea[])',
          [orphanHashList],
        );
        expect(ct128BranchResult.rows[0].cnt).to.eq(0,
          'C3 orphaned ciphertexts128_branch rows must be cleaned up on coprocessor 2');

        // Check allowed_handles.
        const ahResult = await pool2Check.query(
          'SELECT count(*)::int AS cnt FROM allowed_handles WHERE host_chain_id = $1 AND producer_block_hash = ANY($2::bytea[])',
          [orphanChainId, orphanHashList],
        );
        expect(ahResult.rows[0].cnt).to.eq(0,
          'C3 orphaned allowed_handles must be cleaned up on coprocessor 2');

        const ahBranchResult = await pool2Check.query(
          'SELECT count(*)::int AS cnt FROM allowed_handles_branch WHERE host_chain_id = $1 AND producer_block_hash = ANY($2::bytea[])',
          [orphanChainId, orphanHashList],
        );
        expect(ahBranchResult.rows[0].cnt).to.eq(0,
          'C3 orphaned allowed_handles_branch rows must be cleaned up on coprocessor 2');
      } finally {
        await pool2Check.end();
      }

      // Step 5: Submit a new computation on the canonical chain.
      const contract = await deployEncryptedERC20Fixture();
      const tx = await contract.mint(9999);
      await tx.wait();

      // Coprocessor 2 still reads from fork-anvil; copy the recovered canonical
      // workload there so all coprocessors observe the same post-reorg chain.
      await syncAnvilState(forkConfig.canonicalRpcUrl, forkConfig.forkRpcUrl);

      const balanceHandle = handleToHex(
        await contract.balanceOf(this.signers.alice),
      );

      // Step 6: Wait for all 3 to compute and submit.
      const consensus = await waitForConsensus(
        GATEWAY_RPC_URL,
        CIPHERTEXT_COMMITS_ADDRESS,
        balanceHandle,
        120_000, // 2 minutes — recovery may take time
      );

      expect(consensus, 'consensus must be restored after recovery').to.not.be.null;
      expect(consensus!.senders.length).to.eq(COPROCESSOR_COUNT,
        'all 3 coprocessors must agree after recovery');

      // Step 7: Verify all 3 databases agree on the new digest.
      const allDigests = await waitForConsensusDigestRows(
        dbUrls,
        balanceHandle,
        consensus!.ciphertextDigest,
        consensus!.snsCiphertextDigest,
      );
      for (let i = 0; i < COPROCESSOR_COUNT; i++) {
        expect(allDigests[i].length, `coprocessor ${i} must have digest`).to.be.gte(1);
        expect(
          findConsensusDigestRow(
            allDigests[i],
            consensus!.ciphertextDigest,
            consensus!.snsCiphertextDigest,
          ),
          `coprocessor ${i} digest must include consensus row after recovery`,
        ).to.not.be.undefined;
      }
    });
  });
});
