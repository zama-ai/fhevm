import { expect } from 'chai';
import type { BytesLike, HDNodeWallet, TransactionReceipt, TransactionResponse, Wallet } from 'ethers';
import { ethers, network } from 'hardhat';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';

import type { ConfidentialAuctionBidBench } from '../../types/contracts';
import { ConfidentialAuctionBidBench__factory } from '../../types/factories/contracts/ConfidentialAuctionBidBench__factory';
import {
  aclAddress,
  gatewayChainID,
  hostChainID,
  inputVerifierContractAddress,
  kmsVerifierAddress,
  protocolConfigAddress,
  relayerApiKey,
  relayerUrl,
  verifyingContractAddressDecryption,
  verifyingContractAddressInputVerification,
} from '../instance';
import { FhevmSdk } from '../sdk/fhevm-sdk/sdk';
import type { Auth } from '../sdk/types';
import { initSigners } from '../signers';
import { withGasBuffer } from '../utils';

type PgClient = {
  connect(): Promise<void>;
  end(): Promise<void>;
  query<T extends Record<string, unknown> = Record<string, unknown>>(
    sql: string,
    params?: unknown[],
  ): Promise<{ rows: T[] }>;
};

type PgClientCtor = new (config: { connectionString: string }) => PgClient;

type PreparedBid = {
  index: number;
  bidder: HDNodeWallet | Wallet;
  bidderIndex: number;
  bidIndexForBidder: number;
  price: number;
  quantity: number;
  quantityHandle: BytesLike;
  inputProof: BytesLike;
};

type ProofCacheEntry = {
  index?: number;
  bidderPrivateKey: string;
  bidderAddress: string;
  bidderIndex: number;
  bidIndexForBidder: number;
  price: number;
  quantity: number;
  quantityHandle: string;
  inputProof: string;
};

type ProofCache = {
  version: 1;
  chainId: number;
  contractAddress: string;
  bidderCount: number;
  bidsPerBidder: number;
  minPrice: number;
  maxPrice: number;
  priceTick: number;
  quantity: number;
  entries: ProofCacheEntry[];
};

type CompletionState = {
  total: number;
  completed: number;
  pending: number;
  minCreatedAtMs: number | null;
  maxCreatedAtMs: number | null;
  minCompletedAtMs: number | null;
  maxCompletedAtMs: number | null;
};

type BidTiming = {
  index: number;
  txHash: string;
  inputHandle: string;
  submittedAtMs: number;
  submittedAtDbMs?: number;
  minedAtMs: number;
  receiptObservedAtDbMs?: number;
  hostTransactionCreatedAtMs?: number;
  hostTransactionCompletedAtMs?: number;
  proofRequestedAtMs?: number;
  proofVerifiedAtMs?: number;
  targetCompletedAtMs?: number;
  blockNumber: number;
};

type SubmittedBid = {
  preparedBid: PreparedBid;
  tx: TransactionResponse;
  submittedAtMs: number;
  submittedAtDbMs?: number;
};

type SdkConfig = {
  verifyingContractAddressDecryption: string;
  verifyingContractAddressInputVerification: string;
  kmsContractAddress: string;
  inputVerifierContractAddress: string;
  aclContractAddress: string;
  protocolConfigAddress?: string;
  relayerUrl: string;
  rpcUrl: string;
  gatewayChainId: number;
  chainId: number;
  auth?: Auth;
};

type CompletionTarget = 'computations-completed' | 'tfhe-ciphertexts' | 'pbs-completed';
type SubmissionMode = 'await-receipt' | 'burst' | 'manual-mine' | 'steady-rate';

const PRICE_TICK = 5_000;

const envInt = (name: string, fallback: number) => {
  const raw = process.env[name];
  if (!raw) return fallback;
  const value = Number(raw);
  if (!Number.isInteger(value) || value <= 0) {
    throw new Error(`${name} must be a positive integer, got ${raw}`);
  }
  return value;
};

const envNonNegativeInt = (name: string, fallback: number) => {
  const raw = process.env[name];
  if (!raw) return fallback;
  const value = Number(raw);
  if (!Number.isInteger(value) || value < 0) {
    throw new Error(`${name} must be a non-negative integer, got ${raw}`);
  }
  return value;
};

const envEnum = <T extends string>(name: string, fallback: T, values: readonly T[]) => {
  const raw = process.env[name];
  if (!raw) return fallback;
  if ((values as readonly string[]).includes(raw)) return raw as T;
  throw new Error(`${name} must be one of ${values.join(', ')}, got ${raw}`);
};

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
const without0x = (value: string) => value.replace(/^0x/i, '').toLowerCase();

const benchLog = (message: string, extra?: Record<string, unknown>) => {
  const suffix = extra ? ` ${JSON.stringify(extra)}` : '';
  console.log(`[auction-benchmark] ${message}${suffix}`);
};

async function retry<T>(attempts: number, delayMs: number, fn: (attempt: number) => Promise<T>): Promise<T> {
  let lastError: unknown;
  for (let attempt = 1; attempt <= attempts; attempt++) {
    try {
      return await fn(attempt);
    } catch (error) {
      lastError = error;
      if (attempt === attempts) break;
      await sleep(delayMs * attempt);
    }
  }
  throw lastError;
}

const percentile = (values: number[], p: number) => {
  if (values.length === 0) return null;
  const sorted = [...values].sort((a, b) => a - b);
  const index = Math.min(sorted.length - 1, Math.ceil((p / 100) * sorted.length) - 1);
  return sorted[index];
};

const summarize = (values: number[]) => ({
  count: values.length,
  minMs: values.length ? Math.min(...values) : null,
  p50Ms: percentile(values, 50),
  p90Ms: percentile(values, 90),
  p95Ms: percentile(values, 95),
  p99Ms: percentile(values, 99),
  maxMs: values.length ? Math.max(...values) : null,
});

const summarizeBlocks = (blockNumbers: number[]) => {
  const counts = new Map<number, number>();
  for (const blockNumber of blockNumbers) {
    counts.set(blockNumber, (counts.get(blockNumber) ?? 0) + 1);
  }
  const txsPerBlock = [...counts.values()];
  return {
    blockCount: counts.size,
    firstBlock: blockNumbers.length ? Math.min(...blockNumbers) : null,
    lastBlock: blockNumbers.length ? Math.max(...blockNumbers) : null,
    transactionsPerBlock: {
      count: txsPerBlock.length,
      min: txsPerBlock.length ? Math.min(...txsPerBlock) : null,
      p50: percentile(txsPerBlock, 50),
      p90: percentile(txsPerBlock, 90),
      p95: percentile(txsPerBlock, 95),
      p99: percentile(txsPerBlock, 99),
      max: txsPerBlock.length ? Math.max(...txsPerBlock) : null,
      average: txsPerBlock.length ? txsPerBlock.reduce((sum, count) => sum + count, 0) / txsPerBlock.length : null,
    },
  };
};

async function mapLimit<T, U>(items: T[], limit: number, fn: (item: T, index: number) => Promise<U>): Promise<U[]> {
  const results = new Array<U>(items.length);
  let cursor = 0;
  const workers = Array.from({ length: Math.min(limit, items.length) }, async () => {
    for (;;) {
      const index = cursor;
      cursor += 1;
      if (index >= items.length) return;
      results[index] = await fn(items[index], index);
    }
  });
  await Promise.all(workers);
  return results;
}

async function mapPreparedBidsByBidder<T>(
  prepared: PreparedBid[],
  bidderConcurrency: number,
  fn: (preparedBid: PreparedBid) => Promise<T>,
): Promise<T[]> {
  const byBidder = new Map<number, PreparedBid[]>();
  for (const preparedBid of prepared) {
    const bids = byBidder.get(preparedBid.bidderIndex);
    if (bids) {
      bids.push(preparedBid);
    } else {
      byBidder.set(preparedBid.bidderIndex, [preparedBid]);
    }
  }

  const groups = [...byBidder.values()].map((bids) =>
    [...bids].sort((left, right) => left.bidIndexForBidder - right.bidIndexForBidder),
  );
  const results = new Array<T>(prepared.length);
  await mapLimit(groups, bidderConcurrency, async (bids) => {
    for (const preparedBid of bids) {
      results[preparedBid.index] = await fn(preparedBid);
    }
  });
  return results;
}

async function setWalletBalance(addresses: string[], balanceWei: bigint, concurrency: number) {
  const balance = ethers.toBeHex(balanceWei);
  await mapLimit(addresses, concurrency, async (address) => {
    try {
      await ethers.provider.send('anvil_setBalance', [address, balance]);
    } catch {
      await ethers.provider.send('hardhat_setBalance', [address, balance]);
    }
  });
}

async function setAnvilAutomine(enabled: boolean) {
  try {
    await ethers.provider.send('evm_setAutomine', [enabled]);
  } catch {
    await ethers.provider.send('hardhat_setAutomine', [enabled]);
  }
}

async function setAnvilIntervalMining(seconds: number) {
  await ethers.provider.send('evm_setIntervalMining', [seconds]);
}

async function waitForTransactionReceipt(tx: TransactionResponse, timeoutMs: number): Promise<TransactionReceipt> {
  const deadline = Date.now() + timeoutMs;
  for (;;) {
    const receipt = await ethers.provider.getTransactionReceipt(tx.hash);
    if (receipt) return receipt;
    if (Date.now() > deadline) {
      throw new Error(`Timed out waiting for transaction receipt ${tx.hash}`);
    }
    await sleep(250);
  }
}

async function mineUntilSubmittedBidsHaveReceipts(submitted: SubmittedBid[], maxBlocks: number) {
  let minedBlocks = 0;
  for (;;) {
    await ethers.provider.send('evm_mine', []);
    minedBlocks += 1;

    const receipts = await Promise.all(submitted.map(({ tx }) => ethers.provider.getTransactionReceipt(tx.hash)));
    const missingReceipts = receipts.filter((receipt) => receipt === null);
    if (missingReceipts.length === 0) return minedBlocks;
    if (minedBlocks >= maxBlocks) {
      throw new Error(
        `Manual mining left ${missingReceipts.length}/${submitted.length} bid receipts missing after ${minedBlocks} blocks`,
      );
    }
  }
}

async function submitBidsAtSteadyRate(
  prepared: PreparedBid[],
  bidsPerBlock: number,
  bidConcurrency: number,
  sendBid: (preparedBid: PreparedBid) => Promise<SubmittedBid>,
  awaitReceipt: (
    preparedBid: PreparedBid,
    tx: TransactionResponse,
    submittedAtMs: number,
    submittedAtDbMs?: number,
  ) => Promise<BidTiming>,
): Promise<BidTiming[]> {
  const timings: BidTiming[] = [];
  for (let i = 0; i < prepared.length; i += bidsPerBlock) {
    const batch = prepared.slice(i, i + bidsPerBlock);
    const submitted = (await mapPreparedBidsByBidder(batch, bidConcurrency, sendBid)).filter(
      (bid): bid is SubmittedBid => bid !== undefined,
    );
    const receipts = await mapLimit(
      submitted,
      bidConcurrency,
      async ({ preparedBid, tx, submittedAtMs, submittedAtDbMs }) =>
        awaitReceipt(preparedBid, tx, submittedAtMs, submittedAtDbMs),
    );
    timings.push(...receipts);
    benchLog('steady-rate block submitted', {
      start: i,
      count: batch.length,
      blockNumber: receipts[0]?.blockNumber ?? null,
      total: prepared.length,
    });
  }
  return timings;
}

const loadPgClient = (): PgClientCtor => {
  // eslint-disable-next-line @typescript-eslint/no-require-imports
  const pg = require('pg') as { Client: PgClientCtor };
  return pg.Client;
};

async function createBenchmarkDbClient() {
  const connectionString =
    process.env.AUCTION_BENCH_DATABASE_URL ??
    process.env.DATABASE_URL ??
    'postgresql://postgres:postgres@db:5432/coprocessor';
  const Client = loadPgClient();
  const client = new Client({ connectionString });
  await client.connect();
  return client;
}

async function ensureBenchmarkTimingTable(client: PgClient) {
  await client.query(`
    CREATE TABLE IF NOT EXISTS benchmark_timing_events (
      run_id text NOT NULL,
      workload text NOT NULL,
      item_index integer,
      tx_hash bytea,
      input_handle bytea,
      result_handle bytea,
      event_name text NOT NULL,
      event_at timestamptz NOT NULL DEFAULT clock_timestamp(),
      extra jsonb NOT NULL DEFAULT '{}'::jsonb
    )
  `);
  await client.query(`
    CREATE INDEX IF NOT EXISTS idx_benchmark_timing_events_run_event
      ON benchmark_timing_events (run_id, event_name, event_at)
  `);
  await client.query(`
    CREATE INDEX IF NOT EXISTS idx_benchmark_timing_events_run_tx
      ON benchmark_timing_events (run_id, tx_hash)
  `);
}

async function recordBenchmarkEvent(
  client: PgClient,
  runId: string,
  workload: string,
  eventName: string,
  options: {
    itemIndex?: number;
    txHash?: string;
    inputHandle?: string;
    extra?: Record<string, unknown>;
  } = {},
): Promise<number> {
  const result = await client.query<{ event_at_ms: string }>(
    `
      INSERT INTO benchmark_timing_events (
        run_id, workload, item_index, tx_hash, input_handle, event_name, event_at, extra
      )
      VALUES (
        $1,
        $2,
        $3,
        CASE WHEN $4::text IS NULL THEN NULL ELSE decode($4::text, 'hex') END,
        CASE WHEN $5::text IS NULL THEN NULL ELSE decode($5::text, 'hex') END,
        $6,
        clock_timestamp(),
        $7::jsonb
      )
      RETURNING (EXTRACT(EPOCH FROM event_at) * 1000)::bigint::text AS event_at_ms
    `,
    [
      runId,
      workload,
      options.itemIndex ?? null,
      options.txHash ? without0x(options.txHash) : null,
      options.inputHandle ? without0x(options.inputHandle) : null,
      eventName,
      JSON.stringify(options.extra ?? {}),
    ],
  );
  return Number(result.rows[0].event_at_ms);
}

async function readBidTransactionStageTimings(
  client: PgClient,
  txHashes: string[],
  completionTarget: CompletionTarget,
): Promise<
  Map<
    string,
    {
      createdAtMs: number;
      completedAtMs: number | null;
      targetCompletedAtMs: number | null;
    }
  >
> {
  if (txHashes.length === 0) return new Map();
  const txHashParams = [...new Set(txHashes.map(without0x))];
  const targetCompletedExpression =
    completionTarget === 'tfhe-ciphertexts'
      ? `(
          SELECT (EXTRACT(EPOCH FROM max(ct.created_at)) * 1000)::bigint::text
          FROM pbs_computations_branch p
          JOIN ciphertexts_branch ct ON ct.handle = p.handle
          WHERE p.transaction_id = t.id
        )`
      : completionTarget === 'pbs-completed'
        ? `(
            SELECT (EXTRACT(EPOCH FROM max(p.completed_at)) * 1000)::bigint::text
            FROM pbs_computations_branch p
            WHERE p.transaction_id = t.id AND p.is_completed
          )`
        : `(
            SELECT (EXTRACT(EPOCH FROM max(c.completed_at)) * 1000)::bigint::text
            FROM computations_branch c
            WHERE c.transaction_id = t.id AND c.is_completed
          )`;
  const result = await client.query<{
    tx_hash: string;
    created_at_ms: string;
    completed_at_ms: string | null;
    target_completed_at_ms: string | null;
  }>(
    `
      SELECT encode(t.id, 'hex') AS tx_hash,
             (EXTRACT(EPOCH FROM t.created_at) * 1000)::bigint::text AS created_at_ms,
             (EXTRACT(EPOCH FROM t.completed_at) * 1000)::bigint::text AS completed_at_ms,
             ${targetCompletedExpression} AS target_completed_at_ms
      FROM transactions t
      WHERE encode(t.id, 'hex') = ANY($1::text[])
    `,
    [txHashParams],
  );
  return new Map(
    result.rows.map((row) => [
      row.tx_hash,
      {
        createdAtMs: Number(row.created_at_ms),
        completedAtMs: row.completed_at_ms === null ? null : Number(row.completed_at_ms),
        targetCompletedAtMs: row.target_completed_at_ms === null ? null : Number(row.target_completed_at_ms),
      },
    ]),
  );
}

async function readProofVerificationDbTimings(
  client: PgClient,
  txHashes: string[],
): Promise<Map<string, { requestedAtMs: number; verifiedAtMs: number | null }>> {
  if (txHashes.length === 0) return new Map();
  const result = await client.query<{
    tx_hash: string;
    requested_at_ms: string;
    verified_at_ms: string | null;
  }>(
    `
      SELECT encode(transaction_id, 'hex') AS tx_hash,
             (EXTRACT(EPOCH FROM min(created_at)) * 1000)::bigint::text AS requested_at_ms,
             (EXTRACT(EPOCH FROM max(verified_at)) * 1000)::bigint::text AS verified_at_ms
      FROM verify_proofs
      WHERE transaction_id IS NOT NULL
        AND encode(transaction_id, 'hex') = ANY($1::text[])
      GROUP BY transaction_id
    `,
    [[...new Set(txHashes.map(without0x))]],
  );
  return new Map(
    result.rows.map((row) => [
      row.tx_hash,
      {
        requestedAtMs: Number(row.requested_at_ms),
        verifiedAtMs: row.verified_at_ms === null ? null : Number(row.verified_at_ms),
      },
    ]),
  );
}

async function readCompletionState(
  client: PgClient,
  blockNumbers: number[],
  completionTarget: CompletionTarget,
): Promise<CompletionState> {
  const uniqueBlockNumbers = [...new Set(blockNumbers)].map((blockNumber) => String(blockNumber));
  const table = completionTarget === 'pbs-completed' ? 'pbs_computations_branch' : 'computations_branch';
  const result = await client.query<{
    total: string;
    completed: string;
    pending: string;
    min_created_at_ms: string | null;
    max_created_at_ms: string | null;
    min_completed_at_ms: string | null;
    max_completed_at_ms: string | null;
  }>(
    `
      SELECT count(*)::text AS total,
             count(*) FILTER (WHERE is_completed)::text AS completed,
             count(*) FILTER (WHERE NOT is_completed AND NOT is_error)::text AS pending,
             (EXTRACT(EPOCH FROM min(created_at)) * 1000)::bigint::text AS min_created_at_ms,
             (EXTRACT(EPOCH FROM max(created_at)) * 1000)::bigint::text AS max_created_at_ms,
             (EXTRACT(EPOCH FROM min(completed_at)) * 1000)::bigint::text AS min_completed_at_ms,
             (EXTRACT(EPOCH FROM max(completed_at)) * 1000)::bigint::text AS max_completed_at_ms
      FROM ${table}
      WHERE block_number = ANY($1::bigint[])
    `,
    [uniqueBlockNumbers],
  );
  const row = result.rows[0];
  return {
    total: Number(row.total),
    completed: Number(row.completed),
    pending: Number(row.pending),
    minCreatedAtMs: row.min_created_at_ms === null ? null : Number(row.min_created_at_ms),
    maxCreatedAtMs: row.max_created_at_ms === null ? null : Number(row.max_created_at_ms),
    minCompletedAtMs: row.min_completed_at_ms === null ? null : Number(row.min_completed_at_ms),
    maxCompletedAtMs: row.max_completed_at_ms === null ? null : Number(row.max_completed_at_ms),
  };
}

async function readTfheCiphertextState(client: PgClient, blockNumbers: number[]): Promise<CompletionState> {
  const uniqueBlockNumbers = [...new Set(blockNumbers)].map((blockNumber) => String(blockNumber));
  const result = await client.query<{
    total: string;
    completed: string;
    pending: string;
    min_created_at_ms: string | null;
    max_created_at_ms: string | null;
    min_completed_at_ms: string | null;
    max_completed_at_ms: string | null;
  }>(
    `
      WITH computed_ciphertexts AS (
        SELECT handle, min(created_at) AS created_at
        FROM ciphertexts_branch
        GROUP BY handle
      )
      SELECT count(*)::text AS total,
             count(c.handle)::text AS completed,
             (count(*) - count(c.handle))::text AS pending,
             (EXTRACT(EPOCH FROM min(p.created_at)) * 1000)::bigint::text AS min_created_at_ms,
             (EXTRACT(EPOCH FROM max(p.created_at)) * 1000)::bigint::text AS max_created_at_ms,
             (EXTRACT(EPOCH FROM min(c.created_at)) * 1000)::bigint::text AS min_completed_at_ms,
             (EXTRACT(EPOCH FROM max(c.created_at)) * 1000)::bigint::text AS max_completed_at_ms
      FROM pbs_computations_branch p
      LEFT JOIN computed_ciphertexts c ON c.handle = p.handle
      WHERE p.block_number = ANY($1::bigint[])
    `,
    [uniqueBlockNumbers],
  );
  const row = result.rows[0];
  return {
    total: Number(row.total),
    completed: Number(row.completed),
    pending: Number(row.pending),
    minCreatedAtMs: row.min_created_at_ms === null ? null : Number(row.min_created_at_ms),
    maxCreatedAtMs: row.max_created_at_ms === null ? null : Number(row.max_created_at_ms),
    minCompletedAtMs: row.min_completed_at_ms === null ? null : Number(row.min_completed_at_ms),
    maxCompletedAtMs: row.max_completed_at_ms === null ? null : Number(row.max_completed_at_ms),
  };
}

async function readProofCache(cachePath: string | undefined) {
  if (!cachePath) return undefined;
  try {
    return JSON.parse(await readFile(cachePath, 'utf8')) as ProofCache;
  } catch (error) {
    const code = (error as { code?: string }).code;
    if (code === 'ENOENT') return undefined;
    throw error;
  }
}

async function writeProofCache(cachePath: string | undefined, cache: ProofCache) {
  if (!cachePath) return;
  await mkdir(path.dirname(cachePath), { recursive: true });
  await writeFile(cachePath, `${JSON.stringify(cache, null, 2)}\n`);
}

async function usableProofCacheEntryCount(
  cache: ProofCache | undefined,
  bidderCount: number,
  bidsPerBidder: number,
  minPrice: number,
  maxPrice: number,
  quantity: number,
) {
  if (!cache) return 0;
  const chainId = await ethers.provider.getNetwork().then((network) => Number(network.chainId));
  if (
    cache.version !== 1 ||
    cache.chainId !== chainId ||
    cache.bidsPerBidder !== bidsPerBidder ||
    cache.minPrice !== minPrice ||
    cache.maxPrice !== maxPrice ||
    cache.priceTick !== PRICE_TICK ||
    cache.quantity !== quantity ||
    cache.bidderCount < bidderCount ||
    cache.entries.length <= 0
  ) {
    return 0;
  }
  if ((await ethers.provider.getCode(cache.contractAddress)) === '0x') return 0;

  const bidCount = bidderCount * bidsPerBidder;
  let contiguousEntries = 0;
  for (const [position, entry] of cache.entries.slice(0, bidCount).entries()) {
    const entryIndex = entry.index ?? position;
    if (entryIndex !== position) break;
    contiguousEntries += 1;
  }
  return contiguousEntries;
}

function proofCacheEntryFromBid(bid: PreparedBid): ProofCacheEntry {
  return {
    index: bid.index,
    bidderPrivateKey: bid.bidder.privateKey,
    bidderAddress: bid.bidder.address,
    bidderIndex: bid.bidderIndex,
    bidIndexForBidder: bid.bidIndexForBidder,
    price: bid.price,
    quantity: bid.quantity,
    quantityHandle: ethers.hexlify(bid.quantityHandle),
    inputProof: ethers.hexlify(bid.inputProof),
  };
}

function normalPrice(minPrice: number, maxPrice: number) {
  const mean = (minPrice + maxPrice) / 2;
  const stddev = (maxPrice - minPrice) / 6;
  let value = mean;
  for (;;) {
    const u1 = Math.max(Number.EPSILON, Math.random());
    const u2 = Math.random();
    const z0 = Math.sqrt(-2 * Math.log(u1)) * Math.cos(2 * Math.PI * u2);
    value = mean + z0 * stddev;
    if (value >= minPrice && value <= maxPrice) break;
  }
  const ticked = Math.round(value / PRICE_TICK) * PRICE_TICK;
  return Math.min(maxPrice, Math.max(minPrice, ticked));
}

describe('Confidential auction bid benchmark', function () {
  it('benchmark confidential auction bids', async function () {
    const bidderCount = envInt('AUCTION_BENCH_BIDDERS', 500);
    const bidsPerBidder = envInt('AUCTION_BENCH_BIDS_PER_BIDDER', 2);
    const bidCount = bidderCount * bidsPerBidder;
    const completionTarget = envEnum<CompletionTarget>('AUCTION_BENCH_COMPLETION_TARGET', 'computations-completed', [
      'computations-completed',
      'tfhe-ciphertexts',
      'pbs-completed',
    ]);
    const submissionMode = envEnum<SubmissionMode>('AUCTION_BENCH_SUBMISSION_MODE', 'manual-mine', [
      'await-receipt',
      'burst',
      'manual-mine',
      'steady-rate',
    ]);
    const bidConcurrency = envInt('AUCTION_BENCH_BID_CONCURRENCY', 1000);
    const steadyBidsPerBlock = envInt('AUCTION_BENCH_STEADY_BIDS_PER_BLOCK', bidConcurrency);
    const proofConcurrency = envInt('AUCTION_BENCH_PROOF_CONCURRENCY', 10);
    const fundConcurrency = envInt('AUCTION_BENCH_FUND_CONCURRENCY', 200);
    const pollIntervalMs = envInt('AUCTION_BENCH_POLL_INTERVAL_MS', 1000);
    const timeoutMs = envInt('AUCTION_BENCH_TIMEOUT_MS', 60 * 60 * 1000);
    const proofRetries = envInt('AUCTION_BENCH_PROOF_RETRIES', 5);
    const proofRetryDelayMs = envInt('AUCTION_BENCH_PROOF_RETRY_DELAY_MS', 1000);
    const manualMineMaxBlocks = envInt('AUCTION_BENCH_MANUAL_MINE_MAX_BLOCKS', 20);
    const manualMineIntervalSeconds = envInt('AUCTION_BENCH_MANUAL_MINE_INTERVAL_SECONDS', 3600);
    const manualMineRestoreIntervalSeconds = envNonNegativeInt('AUCTION_BENCH_MANUAL_MINE_RESTORE_INTERVAL_SECONDS', 0);
    const minPrice = envInt('AUCTION_BENCH_MIN_PRICE', 10_000);
    const maxPrice = envInt('AUCTION_BENCH_MAX_PRICE', 200_000);
    const quantity = envInt('AUCTION_BENCH_QUANTITY', 1);
    const maxCumulativeBidQuantity = envInt(
      'AUCTION_BENCH_MAX_CUMULATIVE_QUANTITY',
      Math.max(quantity * bidsPerBidder, 1),
    );
    const initialPaymentBalance = envInt('AUCTION_BENCH_INITIAL_PAYMENT_BALANCE', maxPrice * quantity * bidsPerBidder);
    const walletCount = envInt('AUCTION_BENCH_WALLET_COUNT', 32);
    const proofCachePath = process.env.AUCTION_BENCH_PROOF_CACHE;
    const sdkAuth = relayerApiKey ? { __type: 'ApiKeyHeader' as const, value: relayerApiKey } : undefined;
    const sdkConfig: SdkConfig = {
      verifyingContractAddressDecryption,
      verifyingContractAddressInputVerification: verifyingContractAddressInputVerification!,
      kmsContractAddress: kmsVerifierAddress,
      inputVerifierContractAddress: inputVerifierContractAddress!,
      aclContractAddress: aclAddress,
      protocolConfigAddress,
      relayerUrl,
      rpcUrl: (network.config as { url: string }).url,
      gatewayChainId: gatewayChainID!,
      chainId: hostChainID!,
      ...(sdkAuth ? { auth: sdkAuth } : {}),
    };

    this.timeout(timeoutMs + 10 * 60 * 1000);

    benchLog('starting', {
      bidderCount,
      bidsPerBidder,
      bidCount,
      completionTarget,
      submissionMode,
      bidConcurrency,
      steadyBidsPerBlock,
      proofConcurrency,
      minPrice,
      maxPrice,
      quantity,
      proofCachePath: proofCachePath ?? null,
    });

    await initSigners(1);
    const proofCache = await readProofCache(proofCachePath);
    const cachedEntryCount = Math.min(
      await usableProofCacheEntryCount(proofCache, bidderCount, bidsPerBidder, minPrice, maxPrice, quantity),
      bidCount,
    );
    const useProofCache = cachedEntryCount === bidCount;
    const extendProofCache = cachedEntryCount > 0 && cachedEntryCount < bidCount;
    benchLog('proof cache checked', {
      reused: useProofCache,
      extendable: extendProofCache,
      reusableEntries: cachedEntryCount,
      requestedEntries: bidCount,
      entries: proofCache?.entries.length ?? 0,
    });

    const [deployer] = await ethers.getSigners();
    const auction = (
      cachedEntryCount > 0
        ? ConfidentialAuctionBidBench__factory.connect(proofCache!.contractAddress, ethers.provider)
        : await new ConfidentialAuctionBidBench__factory(deployer).deploy(
            maxPrice,
            maxCumulativeBidQuantity,
            initialPaymentBalance,
            walletCount,
          )
    ) as ConfidentialAuctionBidBench;
    if (cachedEntryCount === 0) await auction.waitForDeployment();
    const contractAddress = await auction.getAddress();
    benchLog(cachedEntryCount > 0 ? 'attached contract' : 'deployed contract', {
      contractAddress,
      proofCacheReused: useProofCache,
    });

    const cachedEntries = proofCache?.entries.slice(0, cachedEntryCount) ?? [];
    const cachedBidderPrivateKeys = new Map<number, string>();
    for (const entry of cachedEntries) {
      cachedBidderPrivateKeys.set(entry.bidderIndex, entry.bidderPrivateKey);
    }
    const bidders: (HDNodeWallet | Wallet)[] = Array.from({ length: bidderCount }, (_, bidderIndex) => {
      const cachedPrivateKey = cachedBidderPrivateKeys.get(bidderIndex);
      return cachedPrivateKey
        ? withGasBuffer(new ethers.Wallet(cachedPrivateKey, ethers.provider))
        : withGasBuffer(ethers.Wallet.createRandom().connect(ethers.provider));
    });

    await setWalletBalance(
      bidders.map((wallet) => wallet.address),
      ethers.parseEther('1'),
      fundConcurrency,
    );
    benchLog('funded bidders', { bidders: bidders.length, fundConcurrency });

    const fhe = await FhevmSdk.create(sdkConfig);
    const bidSeeds = Array.from({ length: bidCount }, (_, index) => {
      const bidderIndex = Math.floor(index / bidsPerBidder);
      return {
        index,
        bidderIndex,
        bidIndexForBidder: index % bidsPerBidder,
        price: normalPrice(minPrice, maxPrice),
      };
    });

    let generatedProofs = 0;
    let checkpointedEntries = cachedEntries.length;
    const generatedEntriesByIndex = new Map<number, ProofCacheEntry>();
    const writeProofCacheCheckpoint = async (force: boolean) => {
      if (!proofCachePath) return;
      const entries = [...cachedEntries];
      for (let index = cachedEntryCount; index < bidCount; index += 1) {
        const entry = generatedEntriesByIndex.get(index);
        if (!entry) break;
        entries.push(entry);
      }
      if (!force && entries.length === checkpointedEntries) return;
      checkpointedEntries = entries.length;
      const chainId = await ethers.provider.getNetwork().then((network) => Number(network.chainId));
      await writeProofCache(proofCachePath, {
        version: 1,
        chainId,
        contractAddress,
        bidderCount,
        bidsPerBidder,
        minPrice,
        maxPrice,
        priceTick: PRICE_TICK,
        quantity,
        entries,
      });
      benchLog(force ? 'wrote proof cache' : 'wrote partial proof cache', {
        proofCachePath,
        entries: entries.length,
      });
    };
    // Proof generation is parallel. Serialize cache checkpoints so concurrent
    // workers never race while opening and truncating the same cache file.
    let proofCacheCheckpoint = Promise.resolve();
    const writeProofCacheCheckpointSerialized = (force: boolean) => {
      proofCacheCheckpoint = proofCacheCheckpoint.then(() => writeProofCacheCheckpoint(force));
      return proofCacheCheckpoint;
    };
    const prepared: PreparedBid[] = useProofCache
      ? cachedEntries.map((entry, index) => ({
          index: entry.index ?? index,
          bidder: bidders[entry.bidderIndex],
          bidderIndex: entry.bidderIndex,
          bidIndexForBidder: entry.bidIndexForBidder,
          price: entry.price,
          quantity: entry.quantity,
          quantityHandle: entry.quantityHandle,
          inputProof: entry.inputProof,
        }))
      : [
          ...cachedEntries.map((entry, index) => ({
            index: entry.index ?? index,
            bidder: bidders[entry.bidderIndex],
            bidderIndex: entry.bidderIndex,
            bidIndexForBidder: entry.bidIndexForBidder,
            price: entry.price,
            quantity: entry.quantity,
            quantityHandle: entry.quantityHandle,
            inputProof: entry.inputProof,
          })),
          ...(await mapLimit(bidSeeds.slice(cachedEntryCount), proofConcurrency, async (seed): Promise<PreparedBid> => {
            const bidder = bidders[seed.bidderIndex];
            const encryptedQuantity = await retry(proofRetries, proofRetryDelayMs, () =>
              fhe.encryptUint64({
                value: quantity,
                contractAddress,
                userAddress: bidder.address,
              }),
            );
            const bid = {
              index: seed.index,
              bidder,
              bidderIndex: seed.bidderIndex,
              bidIndexForBidder: seed.bidIndexForBidder,
              price: seed.price,
              quantity,
              quantityHandle: ethers.hexlify(encryptedQuantity.handles[0]),
              inputProof: ethers.hexlify(encryptedQuantity.inputProof),
            };
            generatedEntriesByIndex.set(seed.index, proofCacheEntryFromBid(bid));
            generatedProofs += 1;
            if (generatedProofs % 25 === 0 || generatedProofs === bidCount - cachedEntryCount) {
              benchLog('generated input proofs', {
                generated: generatedProofs,
                cached: cachedEntryCount,
                total: bidCount,
              });
              await writeProofCacheCheckpointSerialized(false);
            }
            return bid;
          })),
        ];
    benchLog(useProofCache ? 'loaded prepared bids' : 'generated proofs', {
      prepared: prepared.length,
      cached: cachedEntryCount,
      generated: prepared.length - cachedEntryCount,
    });

    if (!useProofCache && proofCachePath) {
      for (const bid of prepared) generatedEntriesByIndex.set(bid.index, proofCacheEntryFromBid(bid));
      await writeProofCacheCheckpointSerialized(true);
    }

    const benchmarkRunId = `auction-${new Date().toISOString().replace(/[:.]/g, '-')}-${process.pid}`;
    const startedAtMs = Date.now();
    const db = await createBenchmarkDbClient();
    await ensureBenchmarkTimingTable(db);
    const startedAtDbMs = await recordBenchmarkEvent(db, benchmarkRunId, 'auction', 'run_started');

    const sendBid = async (preparedBid: PreparedBid) => {
      const contractFromBidder = auction.connect(preparedBid.bidder);
      const submittedAtMs = Date.now();
      const tx = await contractFromBidder.submitEncryptedBid(
        preparedBid.price,
        preparedBid.quantityHandle,
        preparedBid.inputProof,
      );
      const submittedAtDbMs = await recordBenchmarkEvent(db, benchmarkRunId, 'auction', 'tx_submitted', {
        itemIndex: preparedBid.index,
        txHash: tx.hash,
        inputHandle: ethers.hexlify(preparedBid.quantityHandle),
      });
      return { preparedBid, tx, submittedAtMs, submittedAtDbMs };
    };

    const awaitReceipt = async (
      preparedBid: PreparedBid,
      tx: TransactionResponse,
      submittedAtMs: number,
      submittedAtDbMs?: number,
    ): Promise<BidTiming> => {
      const receipt = await waitForTransactionReceipt(tx, timeoutMs);
      const minedAtMs = Date.now();
      expect(receipt.status).to.equal(1);
      expect(receipt.blockNumber).to.not.equal(null);
      const receiptObservedAtDbMs = await recordBenchmarkEvent(db, benchmarkRunId, 'auction', 'receipt_observed', {
        itemIndex: preparedBid.index,
        txHash: tx.hash,
        inputHandle: ethers.hexlify(preparedBid.quantityHandle),
        extra: { blockNumber: receipt.blockNumber },
      });
      return {
        index: preparedBid.index,
        txHash: tx.hash,
        inputHandle: ethers.hexlify(preparedBid.quantityHandle),
        submittedAtMs,
        submittedAtDbMs,
        minedAtMs,
        receiptObservedAtDbMs,
        blockNumber: receipt.blockNumber,
      };
    };

    let timings: BidTiming[];
    let automineDisabled = false;
    let intervalMiningChanged = false;
    let manualMineBlocks = 0;
    try {
      if (submissionMode === 'manual-mine') {
        if (manualMineIntervalSeconds > 0) {
          await setAnvilIntervalMining(manualMineIntervalSeconds);
          intervalMiningChanged = true;
        }
        await setAnvilAutomine(false);
        automineDisabled = true;
      } else if (submissionMode === 'steady-rate') {
        await setAnvilIntervalMining(1);
        intervalMiningChanged = true;
        await setAnvilAutomine(false);
        automineDisabled = true;
      }

      const submitted =
        submissionMode === 'await-receipt' || submissionMode === 'steady-rate'
          ? []
          : await mapPreparedBidsByBidder(prepared, bidConcurrency, sendBid);
      benchLog('submitted bids', {
        submitted: submissionMode === 'await-receipt' || submissionMode === 'steady-rate' ? 0 : submitted.length,
        submissionMode,
      });

      if (submissionMode === 'manual-mine') {
        manualMineBlocks = await mineUntilSubmittedBidsHaveReceipts(submitted, manualMineMaxBlocks);
        benchLog('manual mined bids', {
          manualMineBlocks,
          submitted: submitted.length,
        });
      }

      timings =
        submissionMode === 'steady-rate'
          ? await submitBidsAtSteadyRate(prepared, steadyBidsPerBlock, bidConcurrency, sendBid, awaitReceipt)
          : submissionMode === 'await-receipt'
            ? await mapPreparedBidsByBidder(prepared, bidConcurrency, async (preparedBid) => {
                const { tx, submittedAtMs, submittedAtDbMs } = await sendBid(preparedBid);
                return awaitReceipt(preparedBid, tx, submittedAtMs, submittedAtDbMs);
              })
            : await mapLimit(submitted, bidConcurrency, async ({ preparedBid, tx, submittedAtMs, submittedAtDbMs }) =>
                awaitReceipt(preparedBid, tx, submittedAtMs, submittedAtDbMs),
              );
    } finally {
      if (intervalMiningChanged) {
        await setAnvilIntervalMining(manualMineRestoreIntervalSeconds);
      }
      if (automineDisabled) {
        await setAnvilAutomine(true);
      }
    }

    const blockNumbers = timings.map((timing) => timing.blockNumber);
    benchLog('collected bid receipts', {
      bids: timings.length,
      blocks: summarizeBlocks(blockNumbers),
    });

    const deadline = Date.now() + timeoutMs;
    let completionState: CompletionState | null = null;
    let completionObservedAtMs: number | null = null;
    try {
      for (;;) {
        const now = Date.now();
        if (now > deadline) {
          const progress = completionState
            ? `${completionState.completed}/${completionState.total} completed, ${completionState.pending} pending`
            : 'no matching rows observed';
          throw new Error(`Timed out waiting for ${completionTarget}: ${progress}`);
        }
        completionState =
          completionTarget === 'tfhe-ciphertexts'
            ? await readTfheCiphertextState(db, blockNumbers)
            : await readCompletionState(db, blockNumbers, completionTarget);
        if (completionState.total > 0 && completionState.pending === 0) {
          completionObservedAtMs = await recordBenchmarkEvent(db, benchmarkRunId, 'auction', 'completion_observed', {
            extra: { completionTarget },
          });
          break;
        }
        benchLog('waiting for completion target', {
          completionTarget,
          completed: completionState.completed,
          total: completionState.total,
          pending: completionState.pending,
        });
        await sleep(pollIntervalMs);
      }
      const txStageTimings = await readBidTransactionStageTimings(
        db,
        timings.map((timing) => timing.txHash),
        completionTarget,
      );
      const proofVerificationDbTimings = await readProofVerificationDbTimings(
        db,
        timings.map((timing) => timing.txHash),
      );
      for (const timing of timings) {
        const stage = txStageTimings.get(without0x(timing.txHash));
        if (stage) {
          timing.hostTransactionCreatedAtMs = stage.createdAtMs;
          timing.hostTransactionCompletedAtMs = stage.completedAtMs ?? undefined;
          timing.targetCompletedAtMs = stage.targetCompletedAtMs ?? undefined;
        }
        const proofTiming = proofVerificationDbTimings.get(without0x(timing.txHash));
        if (proofTiming) {
          timing.proofRequestedAtMs = proofTiming.requestedAtMs;
          timing.proofVerifiedAtMs = proofTiming.verifiedAtMs ?? undefined;
        }
      }
    } finally {
      await db.end();
    }

    const perBidCompletionTimes = timings
      .map((timing) => timing.targetCompletedAtMs)
      .filter((value): value is number => value !== undefined);
    const completedAtMs =
      perBidCompletionTimes.length > 0
        ? Math.max(...perBidCompletionTimes)
        : (completionState?.maxCompletedAtMs ?? completionObservedAtMs ?? Date.now());
    const wallSeconds = (completedAtMs - startedAtDbMs) / 1000;
    const dbSeconds =
      completionState?.minCreatedAtMs != null && completionState.maxCompletedAtMs != null
        ? (completionState.maxCompletedAtMs - completionState.minCreatedAtMs) / 1000
        : null;
    const minedToCompleted = timings
      .filter((timing) => timing.targetCompletedAtMs !== undefined && timing.hostTransactionCreatedAtMs !== undefined)
      .map((timing) => timing.targetCompletedAtMs! - timing.hostTransactionCreatedAtMs!);
    const submittedToCompleted = timings
      .filter((timing) => timing.targetCompletedAtMs !== undefined && timing.submittedAtDbMs !== undefined)
      .map((timing) => timing.targetCompletedAtMs! - timing.submittedAtDbMs!);
    const localMinedToCompleted = timings
      .filter((timing) => timing.targetCompletedAtMs !== undefined)
      .map((timing) => timing.targetCompletedAtMs! - timing.minedAtMs);
    const localSubmittedToCompleted = timings
      .filter((timing) => timing.targetCompletedAtMs !== undefined)
      .map((timing) => timing.targetCompletedAtMs! - timing.submittedAtMs);
    const proofRequestedToVerified = timings
      .filter((timing) => timing.proofRequestedAtMs !== undefined && timing.proofVerifiedAtMs !== undefined)
      .map((timing) => timing.proofVerifiedAtMs! - timing.proofRequestedAtMs!);
    const priceCounts = new Map<number, number>();
    for (const bid of prepared) {
      priceCounts.set(bid.price, (priceCounts.get(bid.price) ?? 0) + 1);
    }
    const bidsPerPrice = [...priceCounts.values()];
    const report = {
      benchmarkRunId,
      bidCount,
      bidderCount,
      bidsPerBidder,
      completionTarget,
      submissionMode,
      manualMineBlocks,
      manualMineMaxBlocks,
      manualMineIntervalSeconds,
      priceDistribution: {
        minPrice,
        maxPrice,
        priceTick: PRICE_TICK,
        distinctPriceLevels: priceCounts.size,
        bidsPerPrice: {
          min: bidsPerPrice.length ? Math.min(...bidsPerPrice) : null,
          p50: percentile(bidsPerPrice, 50),
          p90: percentile(bidsPerPrice, 90),
          p95: percentile(bidsPerPrice, 95),
          p99: percentile(bidsPerPrice, 99),
          max: bidsPerPrice.length ? Math.max(...bidsPerPrice) : null,
        },
      },
      concurrency: {
        bidSubmission: bidConcurrency,
        steadyBidsPerBlock,
        proofGeneration: proofConcurrency,
        proofRetries,
        funding: fundConcurrency,
      },
      bidBlocks: summarizeBlocks(blockNumbers),
      proofCache: {
        path: proofCachePath ?? null,
        reused: useProofCache,
      },
      timestampCoverage: {
        submittedAtDb: timings.filter((timing) => timing.submittedAtDbMs !== undefined).length,
        receiptObservedAtDb: timings.filter((timing) => timing.receiptObservedAtDbMs !== undefined).length,
        hostTransactionCreatedAt: timings.filter((timing) => timing.hostTransactionCreatedAtMs !== undefined).length,
        hostTransactionCompletedAt: timings.filter((timing) => timing.hostTransactionCompletedAtMs !== undefined)
          .length,
        proofRequestedAt: timings.filter((timing) => timing.proofRequestedAtMs !== undefined).length,
        proofVerifiedAt: timings.filter((timing) => timing.proofVerifiedAtMs !== undefined).length,
        targetCompletedAt: timings.filter((timing) => timing.targetCompletedAtMs !== undefined).length,
      },
      completion: completionState
        ? {
            ...completionState,
            dbCreatedToCompletedSeconds: dbSeconds,
            completedItemsPerSecond: dbSeconds && dbSeconds > 0 ? completionState.completed / dbSeconds : null,
            bidEquivalentPerSecond: dbSeconds && dbSeconds > 0 ? bidCount / dbSeconds : null,
            observedAtMs: completionObservedAtMs,
          }
        : null,
      throughput: {
        bidsPerSecond: bidCount / wallSeconds,
        wallSeconds,
      },
      latency: {
        proofRequestedToVerified: summarize(proofRequestedToVerified),
        minedToCompleted: summarize(minedToCompleted),
        submittedToCompleted: summarize(submittedToCompleted),
      },
      harnessClockDiagnostics: {
        startedAtMs,
        startedAtDbMs,
        localMinedToCompleted: summarize(localMinedToCompleted),
        localSubmittedToCompleted: summarize(localSubmittedToCompleted),
      },
      timestampSources: {
        submitted: 'benchmark_timing_events.event_at for tx_submitted, recorded by Postgres clock_timestamp()',
        mined:
          'transactions.created_at for the host-chain transaction, recorded by the host-listener in Postgres after block ingestion',
        receiptObserved:
          'benchmark_timing_events.event_at for receipt_observed, recorded by Postgres clock_timestamp()',
        proofRequested: 'verify_proofs.created_at by transaction_id',
        proofVerified: 'verify_proofs.verified_at by transaction_id',
        targetCompleted:
          completionTarget === 'tfhe-ciphertexts'
            ? 'max(ciphertexts_branch.created_at) for ciphertexts joined from pbs_computations_branch by transaction_id'
            : completionTarget === 'pbs-completed'
              ? 'max(pbs_computations_branch.completed_at) grouped by transaction_id'
              : 'max(computations_branch.completed_at) grouped by transaction_id',
      },
      pollIntervalMs,
    };

    console.log(`AUCTION_BENCHMARK_REPORT ${JSON.stringify(report)}`);
    expect(completionState?.pending).to.equal(0);
  });
});
