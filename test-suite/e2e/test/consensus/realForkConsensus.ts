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
 *
 * Run:
 *   ./fhevm-cli test real-fork-consensus
 */
import { expect } from 'chai';
import { ethers } from 'hardhat';
import { Pool } from 'pg';

import { ignoreWatchdogCiphertextHandle } from '../consensusWatchdog';
import { deployEncryptedERC20Fixture } from '../encryptedERC20/EncryptedERC20.fixture';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';
import {
  advancePastFinality,
  defaultForkConfig,
  getCanonicalProvider,
  getForkProvider,
  getSignerForProvider,
  setForkMining,
  syncAnvilState,
  verifyForkDivergence,
} from './forkHelper';
import {
  containerName,
  dockerRestart,
  dockerStart,
  dockerStop,
  findConsensusDigestRow,
  getCoprocessorDbUrls,
  getSubmissions,
  handleToHex,
  sleep,
  waitForConsensus,
  waitForConsensusDigestRows,
} from './helpers';

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

const GATEWAY_RPC_URL = process.env.GATEWAY_RPC_URL || '';
const CIPHERTEXT_COMMITS_ADDRESS = process.env.CIPHERTEXT_COMMITS_ADDRESS || '';
const COPROCESSOR_COUNT = 3;
const FINALITY_LAG = parseInt(process.env.FINALITY_LAG || '5', 10);
const RFC11_SETTLEMENT_LAG = parseInt(process.env.RFC11_SETTLEMENT_LAG || '8', 10);

function requireEnv(): void {
  if (!GATEWAY_RPC_URL) throw new Error('GATEWAY_RPC_URL not set');
  if (!CIPHERTEXT_COMMITS_ADDRESS) throw new Error('CIPHERTEXT_COMMITS_ADDRESS not set');
}

interface DivergentForkWork {
  contractAddress: string;
  balanceHandle: string;
  forkBalanceHandle: string;
  canonicalBlockHash: string;
  canonicalBlockNumber: number;
  forkBlockHash: string;
  forkBlockNumber: number;
}

async function createDivergentForkWork(
  label: string,
  aliceAddress: string,
  canonicalMintAmount: number,
  forkMintAmount: number,
): Promise<DivergentForkWork> {
  expect(forkMintAmount, `[${label}] same-handle fork work must use the same cleartext on both branches`).to.eq(
    canonicalMintAmount,
  );
  const forkConfig = defaultForkConfig();
  const canonicalProvider = getCanonicalProvider(forkConfig);
  const forkProvider = getForkProvider(forkConfig);

  const contract = await deployEncryptedERC20Fixture();
  const contractAddress = await contract.getAddress();

  console.log(`[${label}] Syncing fork Anvil state after contract deployment...`);
  await syncAnvilState(forkConfig.canonicalRpcUrl, forkConfig.forkRpcUrl);

  const canonicalSigner = getSignerForProvider(canonicalProvider, 0);
  const forkSigner = getSignerForProvider(forkProvider, 0);
  const signerAddress = await canonicalSigner.getAddress();
  const contractOnFork = new ethers.Contract(contractAddress, contract.interface, forkSigner);

  const mintData = contract.interface.encodeFunctionData('mint', [canonicalMintAmount]);
  const nonce = await canonicalProvider.getTransactionCount(signerAddress);
  const { chainId } = await canonicalProvider.getNetwork();
  const gasLimit =
    (await canonicalProvider.estimateGas({
      from: signerAddress,
      to: contractAddress,
      data: mintData,
    })) * 2n;
  const signedMintTx = await canonicalSigner.signTransaction({
    to: contractAddress,
    data: mintData,
    nonce,
    chainId,
    gasLimit,
    type: 2,
    maxFeePerGas: ethers.parseUnits('100', 'gwei'),
    maxPriorityFeePerGas: ethers.parseUnits('1', 'gwei'),
  });

  const latestCanonicalBlock = await canonicalProvider.getBlock('latest');
  expect(latestCanonicalBlock, `[${label}] canonical latest block`).to.not.be.null;
  await forkProvider.send('evm_setNextBlockTimestamp', [latestCanonicalBlock!.timestamp + 2]);

  const [canonicalTx, forkTx] = await Promise.all([
    canonicalProvider.broadcastTransaction(signedMintTx),
    forkProvider.broadcastTransaction(signedMintTx),
  ]);
  const [canonicalReceipt, forkReceipt] = await Promise.all([canonicalTx.wait(), forkTx.wait()]);
  expect(canonicalReceipt, `[${label}] canonical mint receipt`).to.not.be.null;
  expect(canonicalReceipt!.blockHash, `[${label}] canonical mint block hash`).to.not.be.null;
  expect(forkReceipt, `[${label}] fork mint receipt`).to.not.be.null;
  expect(forkReceipt!.blockHash, `[${label}] fork mint block hash`).to.not.be.null;

  const balanceHandle = handleToHex(await contract.balanceOf(aliceAddress));
  const forkBalanceHandle = handleToHex(await contractOnFork.balanceOf(aliceAddress));

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
    contractAddress,
    balanceHandle,
    forkBalanceHandle,
    canonicalBlockHash: canonicalReceipt!.blockHash!,
    canonicalBlockNumber: canonicalReceipt!.blockNumber,
    forkBlockHash: forkReceipt!.blockHash!,
    forkBlockNumber: forkReceipt!.blockNumber,
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
    submissions = await getSubmissions(GATEWAY_RPC_URL, CIPHERTEXT_COMMITS_ADDRESS, balanceHandle);
  }
  expect(submissions.length).to.eq(COPROCESSOR_COUNT, `[${label}] all ${COPROCESSOR_COUNT} coprocessors must submit`);

  const digests = new Set(submissions.map((s) => s.ciphertextDigest));
  expect(digests.size).to.be.gte(2, `[${label}] at least 2 distinct digests expected (fork divergence)`);

  return submissions;
}

async function waitForSubmissionCount(
  label: string,
  balanceHandle: string,
  expectedCount: number,
  timeoutMs: number = 180_000,
): Promise<Awaited<ReturnType<typeof getSubmissions>>> {
  let submissions: Awaited<ReturnType<typeof getSubmissions>> = [];
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline && submissions.length < expectedCount) {
    await sleep(5_000);
    submissions = await getSubmissions(GATEWAY_RPC_URL, CIPHERTEXT_COMMITS_ADDRESS, balanceHandle);
  }
  expect(submissions.length).to.eq(
    expectedCount,
    `[${label}] expected ${expectedCount} submission(s) for ${balanceHandle}`,
  );
  return submissions;
}

async function waitForForkBranchSubmissions(label: string, forkWork: DivergentForkWork): Promise<void> {
  try {
    if (forkWork.balanceHandle === forkWork.forkBalanceHandle) {
      await waitForDivergentSubmissions(label, forkWork.balanceHandle);
      return;
    }

    const canonicalSubmissions = waitForSubmissionCount(`${label} canonical branch`, forkWork.balanceHandle, 2);
    const forkSubmissions = waitForSubmissionCount(`${label} fork branch`, forkWork.forkBalanceHandle, 1).then(
      async (submissions) => {
        // Submission proves the fork block is finalized. Freeze it immediately
        // so it cannot become settled before the recovery phase replaces it.
        await setForkMining(false);
        return submissions;
      },
    );
    await Promise.all([canonicalSubmissions, forkSubmissions]);
  } finally {
    await setForkMining(false);
  }
}

async function expectNoConsensusForForkWork(label: string, forkWork: DivergentForkWork): Promise<void> {
  const handles = new Set([forkWork.balanceHandle, forkWork.forkBalanceHandle]);
  for (const handle of handles) {
    const consensus = await waitForConsensus(GATEWAY_RPC_URL, CIPHERTEXT_COMMITS_ADDRESS, handle, 5_000);
    expect(consensus, `${label} handle ${handle} must not reach consensus`).to.be.null;
    ignoreWatchdogCiphertextHandle(handle);
  }
}

async function querySettledHeight(databaseUrl: string): Promise<number> {
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const result = await pool.query(
      'SELECT COALESCE(MAX(settled_height), -1)::int AS settled_height FROM coprocessor_settlement',
    );
    return result.rows[0]?.settled_height ?? -1;
  } finally {
    await pool.end();
  }
}

async function waitForSettledHeight(
  label: string,
  databaseUrls: string[],
  targetHeight: number,
  timeoutMs: number = 180_000,
): Promise<number[]> {
  const deadline = Date.now() + timeoutMs;
  let settledHeights: number[] = [];
  while (Date.now() < deadline) {
    settledHeights = await Promise.all(databaseUrls.map((url) => querySettledHeight(url)));
    if (settledHeights.every((height) => height >= targetHeight)) {
      return settledHeights;
    }
    await sleep(2_000);
  }
  throw new Error(
    `[${label}] timed out waiting for settlement >= ${targetHeight}; current heights: ${settledHeights.join(', ')}`,
  );
}

async function queryBlockStatus(
  databaseUrl: string,
  blockHash: Buffer,
  blockNumber: number,
): Promise<string | undefined> {
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const result = await pool.query(
      `SELECT block_status
         FROM host_chain_blocks_valid
        WHERE block_hash = $1
          AND block_number = $2
        LIMIT 1`,
      [blockHash, blockNumber],
    );
    return result.rows[0]?.block_status;
  } finally {
    await pool.end();
  }
}

async function waitForFinalizedBlock(
  label: string,
  databaseUrls: string[],
  blockHash: Buffer,
  blockNumber: number,
  timeoutMs: number = 180_000,
): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  let statuses: Array<string | undefined> = [];
  while (Date.now() < deadline) {
    statuses = await Promise.all(databaseUrls.map((url) => queryBlockStatus(url, blockHash, blockNumber)));
    if (statuses.every((status) => status === 'finalized')) return;
    await sleep(2_000);
  }
  throw new Error(
    `[${label}] timed out waiting for canonical block ${blockNumber} (${blockHash.toString('hex')}) to be finalized; ` +
      `current statuses: ${statuses.map((status) => status ?? 'missing').join(', ')}`,
  );
}

async function waitForOrphanedBlock(
  label: string,
  databaseUrl: string,
  blockHash: Buffer,
  timeoutMs: number = 60_000,
): Promise<{ chain_id: string; block_hash: Buffer }> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    const pool = new Pool({ connectionString: databaseUrl, max: 1 });
    try {
      const result = await pool.query(
        "SELECT chain_id, block_hash FROM host_chain_blocks_valid WHERE block_status = 'orphaned' AND block_hash = $1 LIMIT 1",
        [blockHash],
      );
      if (result.rows[0]) return result.rows[0];
    } finally {
      await pool.end();
    }
    await sleep(2_000);
  }
  throw new Error(`[${label}] timed out waiting for orphaned block ${blockHash.toString('hex')}`);
}

async function countRowsReferencingProducer(
  pool: Pool,
  table: string,
  producerHash: Buffer,
  chainId?: string,
): Promise<number> {
  const hasHostChainId = new Set([
    'computations',
    'computations_branch',
    'ciphertext_digest',
    'ciphertext_digest_branch',
    'allowed_handles',
    'allowed_handles_branch',
  ]).has(table);
  const result = hasHostChainId
    ? await pool.query(
        `SELECT count(*)::int AS cnt FROM ${table} WHERE host_chain_id = $1 AND producer_block_hash = $2`,
        [chainId, producerHash],
      )
    : await pool.query(`SELECT count(*)::int AS cnt FROM ${table} WHERE producer_block_hash = $1`, [producerHash]);
  return result.rows[0].cnt;
}

async function assertNoRowsReferencingProducer(
  label: string,
  databaseUrl: string,
  chainId: string,
  producerHash: Buffer,
): Promise<void> {
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    for (const table of [
      'computations',
      'computations_branch',
      'ciphertext_digest',
      'ciphertext_digest_branch',
      'ciphertexts',
      'ciphertexts_branch',
      'ciphertexts128',
      'ciphertexts128_branch',
      'allowed_handles',
      'allowed_handles_branch',
    ]) {
      expect(
        await countRowsReferencingProducer(pool, table, producerHash, chainId),
        `[${label}] ${table} must not retain orphaned producer rows`,
      ).to.eq(0);
    }
  } finally {
    await pool.end();
  }
}

async function waitForNoRowsReferencingProducer(
  label: string,
  databaseUrl: string,
  chainId: string,
  producerHash: Buffer,
  timeoutMs: number = 60_000,
): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  let lastError: unknown;
  while (Date.now() < deadline) {
    try {
      await assertNoRowsReferencingProducer(label, databaseUrl, chainId, producerHash);
      return;
    } catch (err) {
      lastError = err;
      await sleep(2_000);
    }
  }
  if (lastError instanceof Error) throw lastError;
  throw new Error(`[${label}] timed out waiting for orphaned producer cleanup`);
}

async function queryDigestForBranch(
  databaseUrl: string,
  handle: string,
  producerHash: Buffer,
): Promise<{ ciphertext: Buffer | null; ciphertext128: Buffer | null } | undefined> {
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const result = await pool.query(
      `SELECT ciphertext, ciphertext128
         FROM ciphertext_digest_branch
        WHERE handle = $1
          AND producer_block_hash = $2
        ORDER BY created_at ASC
        LIMIT 1`,
      [Buffer.from(handle.replace('0x', ''), 'hex'), producerHash],
    );
    return result.rows[0];
  } finally {
    await pool.end();
  }
}

describe('Real-Fork Consensus (E3)', function () {
  this.timeout(600_000); // 10 minutes

  let dbUrls: string[];
  before(async function () {
    requireEnv();
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    dbUrls = getCoprocessorDbUrls(COPROCESSOR_COUNT);
  });

  // -------------------------------------------------------------------------
  // C2b: Full-fork equivocation
  // -------------------------------------------------------------------------

  describe('C2b: Full-fork equivocation', function () {
    it('should detect divergence when coprocessors compute on different fork branches', async function () {
      const forkWork = await createDivergentForkWork('C2b', this.signers.alice.address, 1000, 1000);
      await waitForForkBranchSubmissions('C2b', forkWork);
      await expectNoConsensusForForkWork('C2b', forkWork);
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
      const divergentWork = await createDivergentForkWork('C3', this.signers.alice.address, 3000, 3000);
      await waitForForkBranchSubmissions('C3', divergentWork);
      await expectNoConsensusForForkWork('C3 setup', divergentWork);
      const c3ForkBlockHash = Buffer.from(divergentWork.forkBlockHash.replace('0x', ''), 'hex');
      const c3CanonicalBlockHash = Buffer.from(divergentWork.canonicalBlockHash.replace('0x', ''), 'hex');

      // Step 1: Advance canonical Anvil past finality.
      console.log(`[C3] Advancing canonical Anvil past finality (${FINALITY_LAG} blocks)...`);
      await advancePastFinality(FINALITY_LAG, forkConfig);
      await waitForFinalizedBlock(
        'C3 canonical coprocessors',
        dbUrls.slice(0, 2),
        c3CanonicalBlockHash,
        divergentWork.canonicalBlockNumber,
      );

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
      await waitForFinalizedBlock(
        'C3 recovered coprocessor',
        [dbUrls[2]],
        c3CanonicalBlockHash,
        divergentWork.canonicalBlockNumber,
      );

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
        expect(compResult.rows[0].cnt).to.eq(0, 'C3 orphaned computations must be cleaned up on coprocessor 2');

        const compBranchResult = await pool2Check.query(
          'SELECT count(*)::int AS cnt FROM computations_branch WHERE host_chain_id = $1 AND producer_block_hash = ANY($2::bytea[])',
          [orphanChainId, orphanHashList],
        );
        expect(compBranchResult.rows[0].cnt).to.eq(
          0,
          'C3 orphaned computations_branch rows must be cleaned up on coprocessor 2',
        );

        // Check ciphertext_digest.
        const digestResult = await pool2Check.query(
          'SELECT count(*)::int AS cnt FROM ciphertext_digest WHERE host_chain_id = $1 AND producer_block_hash = ANY($2::bytea[])',
          [orphanChainId, orphanHashList],
        );
        expect(digestResult.rows[0].cnt).to.eq(
          0,
          'C3 orphaned ciphertext_digest rows must be cleaned up on coprocessor 2',
        );

        const digestBranchResult = await pool2Check.query(
          'SELECT count(*)::int AS cnt FROM ciphertext_digest_branch WHERE host_chain_id = $1 AND producer_block_hash = ANY($2::bytea[])',
          [orphanChainId, orphanHashList],
        );
        expect(digestBranchResult.rows[0].cnt).to.eq(
          0,
          'C3 orphaned ciphertext_digest_branch rows must be cleaned up on coprocessor 2',
        );

        // Check ciphertexts.
        const ctResult = await pool2Check.query(
          'SELECT count(*)::int AS cnt FROM ciphertexts WHERE producer_block_hash = ANY($1::bytea[])',
          [orphanHashList],
        );
        expect(ctResult.rows[0].cnt).to.eq(0, 'C3 orphaned ciphertexts must be cleaned up on coprocessor 2');

        const ctBranchResult = await pool2Check.query(
          'SELECT count(*)::int AS cnt FROM ciphertexts_branch WHERE producer_block_hash = ANY($1::bytea[])',
          [orphanHashList],
        );
        expect(ctBranchResult.rows[0].cnt).to.eq(
          0,
          'C3 orphaned ciphertexts_branch rows must be cleaned up on coprocessor 2',
        );

        const ct128Result = await pool2Check.query(
          'SELECT count(*)::int AS cnt FROM ciphertexts128 WHERE producer_block_hash = ANY($1::bytea[])',
          [orphanHashList],
        );
        expect(ct128Result.rows[0].cnt).to.eq(0, 'C3 orphaned ciphertexts128 rows must be cleaned up on coprocessor 2');

        const ct128BranchResult = await pool2Check.query(
          'SELECT count(*)::int AS cnt FROM ciphertexts128_branch WHERE producer_block_hash = ANY($1::bytea[])',
          [orphanHashList],
        );
        expect(ct128BranchResult.rows[0].cnt).to.eq(
          0,
          'C3 orphaned ciphertexts128_branch rows must be cleaned up on coprocessor 2',
        );

        // Check allowed_handles.
        const ahResult = await pool2Check.query(
          'SELECT count(*)::int AS cnt FROM allowed_handles WHERE host_chain_id = $1 AND producer_block_hash = ANY($2::bytea[])',
          [orphanChainId, orphanHashList],
        );
        expect(ahResult.rows[0].cnt).to.eq(0, 'C3 orphaned allowed_handles must be cleaned up on coprocessor 2');

        const ahBranchResult = await pool2Check.query(
          'SELECT count(*)::int AS cnt FROM allowed_handles_branch WHERE host_chain_id = $1 AND producer_block_hash = ANY($2::bytea[])',
          [orphanChainId, orphanHashList],
        );
        expect(ahBranchResult.rows[0].cnt).to.eq(
          0,
          'C3 orphaned allowed_handles_branch rows must be cleaned up on coprocessor 2',
        );
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

      const balanceHandle = handleToHex(await contract.balanceOf(this.signers.alice));

      // Step 6: Wait for all 3 to compute and submit.
      const consensus = await waitForConsensus(
        GATEWAY_RPC_URL,
        CIPHERTEXT_COMMITS_ADDRESS,
        balanceHandle,
        120_000, // 2 minutes — recovery may take time
      );

      expect(consensus, 'consensus must be restored after recovery').to.not.be.null;
      expect(consensus!.senders.length).to.eq(COPROCESSOR_COUNT, 'all 3 coprocessors must agree after recovery');

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
          findConsensusDigestRow(allDigests[i], consensus!.ciphertextDigest, consensus!.snsCiphertextDigest),
          `coprocessor ${i} digest must include consensus row after recovery`,
        ).to.not.be.undefined;
      }
    });
  });

  // -------------------------------------------------------------------------
  // C3b: RFC-011 settled reorg barrier
  // -------------------------------------------------------------------------

  describe('C3b: RFC-011 settled reorg barrier', function () {
    it('should keep settlement monotonic while cleaning a forked branch and restoring consensus', async function () {
      const forkConfig = defaultForkConfig();
      const forkProvider = getForkProvider(forkConfig);
      let forkMiningPaused = false;

      const divergentWork = await createDivergentForkWork('C3b', this.signers.alice.address, 5000, 5000);
      await waitForForkBranchSubmissions('C3b', divergentWork);
      await expectNoConsensusForForkWork('C3b setup', divergentWork);

      const canonicalBlockHash = Buffer.from(divergentWork.canonicalBlockHash.replace('0x', ''), 'hex');
      const forkBlockHash = Buffer.from(divergentWork.forkBlockHash.replace('0x', ''), 'hex');
      const targetSettlementHeight = Math.max(divergentWork.canonicalBlockNumber, divergentWork.forkBlockNumber);

      const canonicalDigestBefore = await queryDigestForBranch(
        dbUrls[0],
        divergentWork.balanceHandle,
        canonicalBlockHash,
      );
      expect(canonicalDigestBefore, 'C3b canonical digest row must exist before settlement').to.not.be.undefined;
      expect(canonicalDigestBefore!.ciphertext, 'C3b canonical ciphertext digest must be materialized').to.not.be.null;
      expect(canonicalDigestBefore!.ciphertext128, 'C3b canonical sns digest must be materialized').to.not.be.null;

      try {
        await forkProvider.send('evm_setIntervalMining', [0]);
        await forkProvider.send('evm_setAutomine', [false]);
        forkMiningPaused = true;

        console.log(
          `[C3b] Advancing canonical Anvil to RFC-011 settlement boundary for block ${targetSettlementHeight}...`,
        );
        await advancePastFinality(Math.max(FINALITY_LAG, RFC11_SETTLEMENT_LAG), forkConfig);

        const canonicalSettledBeforeRecovery = await waitForSettledHeight(
          'C3b canonical settlement before recovery',
          dbUrls.slice(0, 2),
          targetSettlementHeight,
        );
        console.log(`[C3b] Canonical coprocessors settled at heights ${canonicalSettledBeforeRecovery.join(', ')}`);

        console.log('[C3b] Resyncing fork Anvil to canonical chain state...');
        await syncAnvilState(forkConfig.canonicalRpcUrl, forkConfig.forkRpcUrl);
        await forkProvider.send('evm_setAutomine', [true]);
        await forkProvider.send('evm_setIntervalMining', [1]);
        forkMiningPaused = false;

        const listener2 = containerName(2, 'host-listener');
        const poller2 = containerName(2, 'host-listener-poller');
        const worker2 = containerName(2, 'tfhe-worker');
        const sender2 = containerName(2, 'transaction-sender');
        await dockerRestart(listener2, poller2, worker2, sender2);

        const c3bOrphan = await waitForOrphanedBlock('C3b', dbUrls[2], forkBlockHash);
        await waitForNoRowsReferencingProducer('C3b', dbUrls[2], c3bOrphan.chain_id, forkBlockHash);

        const settledAfterRecovery = await waitForSettledHeight(
          'C3b settlement after recovery',
          dbUrls,
          targetSettlementHeight,
        );
        expect(settledAfterRecovery[0]).to.be.gte(
          canonicalSettledBeforeRecovery[0],
          'coprocessor 0 settlement must not regress during fork recovery',
        );
        expect(settledAfterRecovery[1]).to.be.gte(
          canonicalSettledBeforeRecovery[1],
          'coprocessor 1 settlement must not regress during fork recovery',
        );

        const canonicalDigestAfter = await queryDigestForBranch(
          dbUrls[0],
          divergentWork.balanceHandle,
          canonicalBlockHash,
        );
        expect(canonicalDigestAfter, 'C3b canonical digest row must survive recovery').to.not.be.undefined;
        expect(canonicalDigestAfter!.ciphertext, 'C3b canonical ciphertext digest must remain materialized').to.not.be
          .null;
        expect(canonicalDigestAfter!.ciphertext128, 'C3b canonical sns digest must remain materialized').to.not.be.null;
        expect(canonicalDigestAfter!.ciphertext!.equals(canonicalDigestBefore!.ciphertext!)).to.eq(
          true,
          'settled canonical ciphertext digest must not change during fork recovery',
        );
        expect(canonicalDigestAfter!.ciphertext128!.equals(canonicalDigestBefore!.ciphertext128!)).to.eq(
          true,
          'settled canonical sns digest must not change during fork recovery',
        );

        const recoveredConsensus = await waitForConsensus(
          GATEWAY_RPC_URL,
          CIPHERTEXT_COMMITS_ADDRESS,
          divergentWork.balanceHandle,
          120_000,
        );
        expect(recoveredConsensus, 'pre-reorg canonical ciphertext must reach consensus after recovery').to.not.be.null;
        expect(recoveredConsensus!.senders.length).to.eq(
          COPROCESSOR_COUNT,
          'all 3 coprocessors must agree on the pre-reorg canonical ciphertext after recovery',
        );
        expect(
          Buffer.from(recoveredConsensus!.ciphertextDigest.replace('0x', ''), 'hex').equals(
            canonicalDigestBefore!.ciphertext!,
          ),
          'recovered consensus must use the pre-reorg canonical ciphertext digest',
        ).to.eq(true);
        expect(
          Buffer.from(recoveredConsensus!.snsCiphertextDigest.replace('0x', ''), 'hex').equals(
            canonicalDigestBefore!.ciphertext128!,
          ),
          'recovered consensus must use the pre-reorg canonical sns digest',
        ).to.eq(true);

        const recoveredBalance = await this.instances.alice.userDecryptSingleHandle({
          handle: divergentWork.balanceHandle,
          contractAddress: divergentWork.contractAddress,
          signer: this.signers.alice,
        });
        expect(recoveredBalance, 'pre-reorg canonical ciphertext must decrypt after recovery').to.eq(5000n);

        const contract = await deployEncryptedERC20Fixture();
        const tx = await contract.mint(7777);
        await tx.wait();
        await syncAnvilState(forkConfig.canonicalRpcUrl, forkConfig.forkRpcUrl);

        const balanceHandle = handleToHex(await contract.balanceOf(this.signers.alice));
        const consensus = await waitForConsensus(GATEWAY_RPC_URL, CIPHERTEXT_COMMITS_ADDRESS, balanceHandle, 120_000);

        expect(consensus, 'consensus must be restored after settled reorg recovery').to.not.be.null;
        expect(consensus!.senders.length).to.eq(
          COPROCESSOR_COUNT,
          'all 3 coprocessors must agree after settled reorg recovery',
        );
      } finally {
        if (forkMiningPaused) {
          await forkProvider.send('evm_setAutomine', [true]);
          await forkProvider.send('evm_setIntervalMining', [1]);
        }
      }
    });
  });
});
