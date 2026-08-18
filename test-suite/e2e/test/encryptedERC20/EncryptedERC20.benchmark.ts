import { expect } from 'chai';
import type { BytesLike, HDNodeWallet, TransactionReceipt, TransactionResponse, Wallet } from 'ethers';
import { ethers, network } from 'hardhat';
import { fork } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { availableParallelism } from 'node:os';
import path from 'node:path';

import type { EncryptedERC20 } from '../../types/contracts';
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
import type { Auth } from '../sdk/types';
import { initSigners } from '../signers';
import { withGasBuffer } from '../utils';
import { deployEncryptedERC20Fixture } from './EncryptedERC20.fixture';

type PgClient = {
  connect(): Promise<void>;
  end(): Promise<void>;
  query<T extends Record<string, unknown> = Record<string, unknown>>(
    sql: string,
    params?: unknown[],
  ): Promise<{ rows: T[] }>;
};

type PgClientCtor = new (config: { connectionString: string }) => PgClient;

type PreparedTransfer = {
  index: number;
  sender: HDNodeWallet | Wallet;
  recipientAddress: string;
  amountHandle: BytesLike;
  inputProof: BytesLike;
};

type ProofWorkerResult = {
  index: number;
  recipientAddress: string;
  amountHandle: string;
  inputProof: string;
  /** Gateway round-trip as the submitter observes it (proving + verification). */
  requestedAtMs?: number;
  respondedAtMs?: number;
};

type ProofWorkerSdkConfig = {
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
  numberOfThreads?: number;
};

type ProofCacheEntry = {
  senderPrivateKey: string;
  senderAddress: string;
  recipientAddress: string;
  amountHandle: string;
  inputProof: string;
};

/** Identifies the account pattern the cached entries were generated for. Entries
 * carry their sender key, so replaying them reproduces the exact sender/recipient
 * sequence — which is only correct if the run wants that same pattern. */
type TrafficSignature = {
  joinBraid: boolean;
  lagBlocks: number;
  transfersPerBlock: number;
};

type ProofCache = {
  version: 1;
  chainId: number;
  contractAddress: string;
  transferCount: number;
  transferAmount: number;
  /** Absent for the independent-transfer workload. */
  traffic?: TrafficSignature;
  entries: ProofCacheEntry[];
};

const sameTrafficSignature = (a: TrafficSignature | undefined, b: TrafficSignature | undefined) => {
  if (!a || !b) return !a && !b;
  return a.joinBraid === b.joinBraid && a.lagBlocks === b.lagBlocks && a.transfersPerBlock === b.transfersPerBlock;
};

type TransferTiming = {
  index: number;
  txHash: string;
  inputHandle: string;
  resultHandle: string;
  submittedAtMs: number;
  submittedAtDbMs?: number;
  minedAtMs: number;
  receiptObservedAtDbMs?: number;
  hostTransactionCreatedAtMs?: number;
  hostTransactionCompletedAtMs?: number;
  proofRequestedAtMs?: number;
  proofVerifiedAtMs?: number;
  computationsCompletedAtMs?: number;
  tfheCiphertextAtMs?: number;
  pbsCompletedAtMs?: number;
  snsCt128ComputedAtMs?: number;
  snsReadyAtMs?: number;
};

type BenchmarkState = {
  readyHandles: Set<string>;
  computationsCompleted: Map<string, number>;
  tfheCiphertexts: Map<string, number>;
  pbsCompleted: Map<string, number>;
  snsCt128Computed: Map<string, number>;
  verifiedInputs: Map<string, number>;
};

type TransactionDbTiming = {
  createdAtMs: number;
  completedAtMs: number | null;
};

type ProofVerificationDbTiming = {
  requestedAtMs: number;
  verifiedAtMs: number | null;
};

type TransferSubmissionMode = 'await-receipt' | 'burst' | 'fixed-rate' | 'manual-mine' | 'steady-rate';
type CompletionTarget = 'sns-ready' | 'pbs-completed' | 'tfhe-ciphertexts' | 'computations-completed';
type SubmittedTransfer = {
  preparedTransfer: PreparedTransfer;
  tx: TransactionResponse;
  submittedAtMs: number;
  submittedAtDbMs?: number;
};
type PbsCompletionState = {
  total: number;
  completed: number;
  pending: number;
  minCreatedAtMs: number | null;
  maxCreatedAtMs: number | null;
  minCompletedAtMs: number | null;
  maxCompletedAtMs: number | null;
};

function withDbStageRates(state: PbsCompletionState | null, transferCount: number) {
  if (!state) return null;
  const startAtMs = state.minCreatedAtMs ?? state.minCompletedAtMs;
  const dbCreatedToCompletedSeconds =
    startAtMs != null && state.maxCompletedAtMs != null ? (state.maxCompletedAtMs - startAtMs) / 1000 : null;
  return {
    ...state,
    dbCreatedToCompletedSeconds,
    completedItemsPerSecond:
      dbCreatedToCompletedSeconds && dbCreatedToCompletedSeconds > 0
        ? state.completed / dbCreatedToCompletedSeconds
        : null,
    transferEquivalentPerSecond:
      dbCreatedToCompletedSeconds && dbCreatedToCompletedSeconds > 0
        ? transferCount / dbCreatedToCompletedSeconds
        : null,
  };
}

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
  console.log(`[erc20-benchmark] ${message}${suffix}`);
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

const loadPgClient = (): PgClientCtor => {
  // The benchmark is the only e2e test that reads coprocessor worker state directly.
  // eslint-disable-next-line @typescript-eslint/no-require-imports
  const pg = require('pg') as { Client: PgClientCtor };
  return pg.Client;
};

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
  transferAmount: number,
  traffic?: TrafficSignature,
) {
  if (!cache) return 0;
  const chainId = await ethers.provider.getNetwork().then((network) => Number(network.chainId));
  if (
    cache.version !== 1 ||
    cache.chainId !== chainId ||
    cache.transferAmount !== transferAmount ||
    cache.transferCount <= 0 ||
    cache.entries.length <= 0 ||
    // Replaying independent-transfer entries into a traffic run (or vice versa)
    // would silently measure the wrong dependence graph.
    !sameTrafficSignature(cache.traffic, traffic)
  ) {
    return 0;
  }
  if ((await ethers.provider.getCode(cache.contractAddress)) === '0x') return 0;
  return Math.min(cache.transferCount, cache.entries.length);
}

function randomWalletWithUniqueAddress(usedAddresses: Set<string>) {
  for (;;) {
    const wallet = withGasBuffer(ethers.Wallet.createRandom().connect(ethers.provider));
    const address = wallet.address.toLowerCase();
    if (!usedAddresses.has(address)) {
      usedAddresses.add(address);
      return wallet;
    }
  }
}

function randomUniqueAddress(usedAddresses: Set<string>) {
  for (;;) {
    const address = ethers.Wallet.createRandom().address;
    const normalized = address.toLowerCase();
    if (!usedAddresses.has(normalized)) {
      usedAddresses.add(normalized);
      return address;
    }
  }
}

/**
 * Assigns (sender, recipient) account-pool slots to each transfer so the
 * resulting handle graph reproduces sustained L1 traffic rather than a fan of
 * independent transfers.
 *
 * Transfers are consumed `transfersPerBlock` at a time, so transfer `i` lands
 * in block `floor(i / transfersPerBlock)`. Blocks are dealt round-robin to
 * `lagBlocks` decks, so a deck is revisited exactly every `lagBlocks` blocks
 * and each transfer therefore reads balances written `lagBlocks` blocks
 * earlier — the dependence lag.
 *
 * Two shapes, matching the native fixtures:
 *  - linear (`joinBraid` false): each slot is a fixed sender/recipient couple,
 *    so chains advance independently and a linear extension exists.
 *  - braided (`joinBraid` true): every transfer moves value between two
 *    accounts of its own deck, and the pairing rotates by round, so each
 *    transfer joins two distinct chains and no linear extension exists.
 *    Within a block all participants are distinct, so the joins are the only
 *    source of serialisation.
 *
 * Returns pool indices; the caller owns wallet creation and funding.
 */
function buildTrafficSchedule(parameters: {
  transferCount: number;
  transfersPerBlock: number;
  lagBlocks: number;
  joinBraid: boolean;
}): { senderSlots: number[]; recipientSlots: number[]; senderPoolSize: number; recipientPoolSize: number } {
  const { transferCount, transfersPerBlock, lagBlocks, joinBraid } = parameters;
  if (transfersPerBlock < 1) throw new Error('traffic mode needs transfersPerBlock >= 1');
  if (lagBlocks < 1) throw new Error('traffic mode needs lagBlocks >= 1');

  const senderSlots: number[] = [];
  const recipientSlots: number[] = [];

  if (joinBraid) {
    // Two participants per transfer, all distinct inside a block: a deck holds
    // 2 * transfersPerBlock accounts, and there is one deck per lag position.
    const deckSize = 2 * transfersPerBlock;
    for (let i = 0; i < transferCount; i += 1) {
      const block = Math.floor(i / transfersPerBlock);
      const slot = i % transfersPerBlock;
      const deck = block % lagBlocks;
      const round = Math.floor(block / lagBlocks);
      const base = deck * deckSize;
      // Rotating by the round re-pairs the chains every visit, which is what
      // stops the braid from degenerating into independent 2-account chains.
      senderSlots.push(base + (2 * slot + round) % deckSize);
      recipientSlots.push(base + (2 * slot + 1 + round) % deckSize);
    }
    const pool = deckSize * lagBlocks;
    return { senderSlots, recipientSlots, senderPoolSize: pool, recipientPoolSize: pool };
  }

  // Linear: chain c owns one sender and one dedicated recipient for the whole
  // run, so its balance handles form a single strand.
  const chainCount = transfersPerBlock * lagBlocks;
  for (let i = 0; i < transferCount; i += 1) {
    const block = Math.floor(i / transfersPerBlock);
    const slot = i % transfersPerBlock;
    const chain = (block % lagBlocks) * transfersPerBlock + slot;
    senderSlots.push(chain);
    recipientSlots.push(chain);
  }
  return { senderSlots, recipientSlots, senderPoolSize: chainCount, recipientPoolSize: chainCount };
}

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

async function mineUntilSubmittedTransfersHaveReceipts(submitted: SubmittedTransfer[], maxBlocks: number) {
  let minedBlocks = 0;
  for (;;) {
    await ethers.provider.send('evm_mine', []);
    minedBlocks += 1;

    const receipts = await Promise.all(submitted.map(({ tx }) => ethers.provider.getTransactionReceipt(tx.hash)));
    const missingReceipts = receipts.filter((receipt) => receipt === null);
    if (missingReceipts.length === 0) {
      return minedBlocks;
    }
    if (minedBlocks >= maxBlocks) {
      throw new Error(
        `Manual mining left ${missingReceipts.length}/${submitted.length} transfer receipts missing after ${minedBlocks} blocks`,
      );
    }
  }
}

async function submitTransfersInManualBlocks(
  prepared: PreparedTransfer[],
  transfersPerBlock: number,
  transferConcurrency: number,
  sendTransfer: (preparedTransfer: PreparedTransfer) => Promise<SubmittedTransfer>,
  awaitReceipt: (
    preparedTransfer: PreparedTransfer,
    tx: TransactionResponse,
    submittedAtMs: number,
    submittedAtDbMs?: number,
  ) => Promise<TransferTiming & { blockNumber: number }>,
  onReceipts?: (receipts: (TransferTiming & { blockNumber: number })[]) => void,
): Promise<{ timings: (TransferTiming & { blockNumber: number })[]; minedBlocks: number }> {
  const timings: (TransferTiming & { blockNumber: number })[] = [];
  let minedBlocks = 0;
  for (let i = 0; i < prepared.length; i += transfersPerBlock) {
    const batch = prepared.slice(i, i + transfersPerBlock);
    const submitted = await mapLimit(batch, Math.min(transferConcurrency, transfersPerBlock), sendTransfer);
    await ethers.provider.send('evm_mine', []);
    minedBlocks += 1;
    const receipts = await Promise.all(
      submitted.map(({ preparedTransfer, tx, submittedAtMs, submittedAtDbMs }) =>
        awaitReceipt(preparedTransfer, tx, submittedAtMs, submittedAtDbMs),
      ),
    );
    timings.push(...receipts);
    onReceipts?.(receipts);
    benchLog('manual-mine block submitted', {
      start: i,
      count: batch.length,
      blockNumber: receipts[0]?.blockNumber ?? null,
      total: prepared.length,
    });
  }
  return { timings, minedBlocks };
}

/**
 * Open-loop arrivals: submits `targetTps` transfers on every wall-clock second
 * and never waits for a receipt before issuing the next tick.
 *
 * This is the difference that matters for a latency-vs-load curve. The
 * closed-loop modes (`steady-rate`, `await-receipt`) only issue new work once
 * earlier work completes, so the offered rate silently collapses to whatever
 * the system can serve and queueing delay never appears in the numbers. Here
 * the arrival rate is fixed by the clock, so when the offered load approaches
 * capacity the wait shows up in the percentiles — which is the entire point of
 * measuring at 25/50/75/100% of the known ceiling.
 *
 * `submissionDriftMs` reports how far behind schedule the harness itself fell:
 * if that grows, the client (not the coprocessor) failed to offer the rate and
 * the run does not measure the load it claims.
 */
async function submitTransfersAtFixedRate(
  prepared: PreparedTransfer[],
  targetTps: number,
  sendTransfer: (preparedTransfer: PreparedTransfer) => Promise<SubmittedTransfer>,
  awaitReceipt: (
    preparedTransfer: PreparedTransfer,
    tx: TransactionResponse,
    submittedAtMs: number,
    submittedAtDbMs?: number,
  ) => Promise<TransferTiming & { blockNumber: number }>,
  onReceipts?: (receipts: (TransferTiming & { blockNumber: number })[]) => void,
): Promise<{ timings: (TransferTiming & { blockNumber: number })[]; maxDriftMs: number; offeredSeconds: number }> {
  const startedAt = Date.now();
  const inflight: Promise<TransferTiming & { blockNumber: number }>[] = [];
  let maxDriftMs = 0;

  for (let i = 0; i < prepared.length; i += targetTps) {
    const tick = i / targetTps;
    const dueAt = startedAt + tick * 1000;
    const drift = Date.now() - dueAt;
    if (drift > maxDriftMs) maxDriftMs = drift;
    if (drift < 0) await sleep(-drift);

    for (const preparedTransfer of prepared.slice(i, i + targetTps)) {
      inflight.push(
        sendTransfer(preparedTransfer)
          .then(({ tx, submittedAtMs, submittedAtDbMs }) =>
            awaitReceipt(preparedTransfer, tx, submittedAtMs, submittedAtDbMs),
          )
          // Report each receipt the moment it lands. The pipeline poller only
          // watches handles it has been told about, so batching these to the end
          // of the run would leave every handle first observed at the same
          // instant — which reads back as latency ≈ (run end − mined), i.e. a
          // p50 of half the run and a p99 of the whole run, at every load.
          .then((timing) => {
            onReceipts?.([timing]);
            return timing;
          }),
      );
    }
    if (tick % 10 === 0) {
      benchLog('fixed-rate tick', { tick, submitted: i + targetTps, total: prepared.length, driftMs: drift });
    }
  }

  // Offered window closes when the last tick is issued, not when work drains.
  const offeredSeconds = (Date.now() - startedAt) / 1000;
  const timings = await Promise.all(inflight);
  return { timings, maxDriftMs, offeredSeconds };
}

async function submitTransfersAtSteadyRate(
  prepared: PreparedTransfer[],
  transfersPerBlock: number,
  sendTransfer: (preparedTransfer: PreparedTransfer) => Promise<SubmittedTransfer>,
  awaitReceipt: (
    preparedTransfer: PreparedTransfer,
    tx: TransactionResponse,
    submittedAtMs: number,
    submittedAtDbMs?: number,
  ) => Promise<TransferTiming & { blockNumber: number }>,
  onReceipts?: (receipts: (TransferTiming & { blockNumber: number })[]) => void,
): Promise<(TransferTiming & { blockNumber: number })[]> {
  const timings: (TransferTiming & { blockNumber: number })[] = [];
  for (let i = 0; i < prepared.length; i += transfersPerBlock) {
    const batch = prepared.slice(i, i + transfersPerBlock);
    const submitted = await Promise.all(batch.map((preparedTransfer) => sendTransfer(preparedTransfer)));
    const receipts = await Promise.all(
      submitted.map(({ preparedTransfer, tx, submittedAtMs, submittedAtDbMs }) =>
        awaitReceipt(preparedTransfer, tx, submittedAtMs, submittedAtDbMs),
      ),
    );
    timings.push(...receipts);
    onReceipts?.(receipts);
    benchLog('steady-rate block submitted', {
      start: i,
      count: batch.length,
      blockNumber: receipts[0]?.blockNumber ?? null,
      total: prepared.length,
    });
  }
  return timings;
}

async function generateProofsWithWorkers(parameters: {
  senders: (HDNodeWallet | Wallet)[];
  recipientAddresses: string[];
  workerCount: number;
  contractAddress: string;
  transferAmount: number;
  sdkConfig: ProofWorkerSdkConfig;
  workerTimeoutMs: number;
  jobRetries: number;
  retryDelayMs: number;
  /** Requests per second across all workers; omit for unpaced (fastest). */
  rate?: number;
  onProofLatency?: (latencyMs: number) => void;
}): Promise<PreparedTransfer[]> {
  const {
    senders,
    recipientAddresses,
    workerCount,
    contractAddress,
    transferAmount,
    sdkConfig,
    workerTimeoutMs,
    jobRetries,
    retryDelayMs,
    rate,
    onProofLatency,
  } = parameters;
  const workersToStart = Math.min(workerCount, senders.length);
  const paceOrigin = Date.now();
  const tasks = senders.map((sender, index) => ({
    index,
    senderAddress: sender.address,
    recipientAddress: recipientAddresses[index],
  }));

  let generatedProofs = 0;
  const workerPath = path.join(__dirname, 'erc20ProofWorker.ts');
  const tasksPerWorkerJob = 10;
  const taskBatches = Array.from(
    { length: Math.ceil(tasks.length / tasksPerWorkerJob) },
    (_, batchIndex) => tasks.slice(batchIndex * tasksPerWorkerJob, (batchIndex + 1) * tasksPerWorkerJob),
  );
  const runProofJob = (workerTasks: typeof tasks, workerIndex: number) =>
    new Promise<ProofWorkerResult[]>((resolve, reject) => {
      let settled = false;
      let child: ReturnType<typeof fork> | undefined;
      const workerTimeout = setTimeout(() => {
        settled = true;
        child?.kill('SIGTERM');
        reject(new Error(`ERC20 proof worker job ${workerIndex} timed out after ${workerTimeoutMs}ms`));
      }, workerTimeoutMs);
      child = fork(workerPath, [], {
        execArgv: ['-r', 'ts-node/register/transpile-only'],
        stdio: ['ignore', 'pipe', 'pipe', 'ipc'],
      });
      child.stdout?.on('data', (data) => {
        benchLog('proof worker stdout', {
          worker: workerIndex,
          message: data.toString().trim(),
        });
      });
      child.stderr?.on('data', (data) => {
        benchLog('proof worker stderr', {
          worker: workerIndex,
          message: data.toString().trim(),
        });
      });
      child.on('message', (message: { ok: boolean; results?: ProofWorkerResult[]; error?: string }) => {
        if (!message.ok) {
          settled = true;
          clearTimeout(workerTimeout);
          reject(new Error(message.error ?? `ERC20 proof worker job ${workerIndex} failed`));
          return;
        }
        generatedProofs += message.results?.length ?? 0;
        benchLog('generated input proofs', {
          generated: generatedProofs,
          total: senders.length,
          worker: workerIndex,
        });
        const results = message.results;
        settled = true;
        clearTimeout(workerTimeout);
        if (!results || results.length !== workerTasks.length) {
          reject(
            new Error(
              `ERC20 proof worker job ${workerIndex} returned ${results?.length ?? 0} of ${workerTasks.length} results`,
            ),
          );
          return;
        }
        resolve(results);
      });
      child.on('error', (error) => {
        settled = true;
        clearTimeout(workerTimeout);
        reject(error);
      });
      child.on('exit', (code) => {
        if (settled) return;
        clearTimeout(workerTimeout);
        reject(new Error(`ERC20 proof worker job ${workerIndex} exited with code ${code} before returning results`));
      });
      child.send({
        contractAddress,
        transferAmount,
        tasks: workerTasks,
        sdkConfig,
        // Each worker holds 1/workersToStart of the stream, so its own spacing
        // is that much wider for the aggregate to land on `rate`.
        ...(rate ? { paceMs: (1000 * workersToStart) / rate, startAtMs: paceOrigin } : {}),
      });
    });
  const results = (
    await mapLimit(taskBatches, workersToStart, (workerTasks, workerIndex) =>
      retry(jobRetries, retryDelayMs, (attempt) => {
        if (attempt > 1) {
          benchLog('retrying input proof generation', {
            worker: workerIndex,
            attempt,
            maxAttempts: jobRetries,
          });
        }
        return runProofJob(workerTasks, workerIndex);
      }),
    )
  ).flat();

  if (onProofLatency) {
    for (const result of results) {
      if (result.requestedAtMs !== undefined && result.respondedAtMs !== undefined) {
        onProofLatency(result.respondedAtMs - result.requestedAtMs);
      }
    }
  }

  return results
    .sort((a, b) => a.index - b.index)
    .map(
      (result): PreparedTransfer => ({
        index: result.index,
        sender: senders[result.index],
        recipientAddress: result.recipientAddress,
        amountHandle: result.amountHandle,
        inputProof: result.inputProof,
      }),
    );
}

async function pollPipelineProgress(
  db: PgClient,
  timings: TransferTiming[],
  expectedCount: number,
  completionTarget: CompletionTarget,
  ready: Set<string>,
  proofTimes: Map<string, number>,
  timeoutMs: number,
  pollIntervalMs: number,
  runId?: string,
  shouldStop = () => false,
) {
  const deadline = Date.now() + timeoutMs;
  const countCompleted = (field: keyof TransferTiming) =>
    timings.filter((timing) => timing[field] !== undefined).length;
  const targetCompleted = () => {
    if (completionTarget === 'sns-ready') return ready.size;
    if (completionTarget === 'pbs-completed') return countCompleted('pbsCompletedAtMs');
    if (completionTarget === 'tfhe-ciphertexts') return countCompleted('tfheCiphertextAtMs');
    return countCompleted('computationsCompletedAtMs');
  };

  while (!shouldStop() && targetCompleted() < expectedCount) {
    const now = Date.now();
    if (now > deadline) {
      throw new Error(`Timed out waiting for ${completionTarget}: ${targetCompleted()}/${expectedCount}`);
    }
    if (timings.length === 0) {
      await sleep(pollIntervalMs);
      continue;
    }

    const inputHandles = timings.map((timing) => without0x(timing.inputHandle));
    const resultHandles = timings.map((timing) => without0x(timing.resultHandle));
    const state = await readBenchmarkState(db, inputHandles, resultHandles);
    for (const [inputHandle, verifiedAtMs] of state.verifiedInputs) {
      proofTimes.set(inputHandle, verifiedAtMs);
    }
    for (const [handle, completedAtMs] of state.computationsCompleted) {
      const timing = timings.find((candidate) => without0x(candidate.resultHandle) === handle);
      if (timing) timing.computationsCompletedAtMs = completedAtMs;
    }
    for (const [handle, createdAtMs] of state.tfheCiphertexts) {
      const timing = timings.find((candidate) => without0x(candidate.resultHandle) === handle);
      if (timing) timing.tfheCiphertextAtMs = createdAtMs;
    }
    for (const [handle, completedAtMs] of state.pbsCompleted) {
      const timing = timings.find((candidate) => without0x(candidate.resultHandle) === handle);
      if (timing) timing.pbsCompletedAtMs = completedAtMs;
    }
    for (const [handle, computedAtMs] of state.snsCt128Computed) {
      const timing = timings.find((candidate) => without0x(candidate.resultHandle) === handle);
      if (timing) timing.snsCt128ComputedAtMs = computedAtMs;
    }
    for (const handle of state.readyHandles) {
      if (!ready.has(handle)) {
        ready.add(handle);
        const timing = timings.find((candidate) => without0x(candidate.resultHandle) === handle);
        if (timing) {
          timing.snsReadyAtMs = runId
            ? await recordBenchmarkEvent(db, runId, 'erc20', 'sns_ready_observed', {
                itemIndex: timing.index,
                txHash: timing.txHash,
                inputHandle: timing.inputHandle,
                resultHandle: timing.resultHandle,
              })
            : now;
        }
      }
    }
    if (targetCompleted() < expectedCount) {
      benchLog('waiting for completion target', {
        completionTarget,
        completed: targetCompleted(),
        total: expectedCount,
        observedTransfers: timings.length,
        computationsCompleted: countCompleted('computationsCompletedAtMs'),
        tfheCiphertexts: countCompleted('tfheCiphertextAtMs'),
        pbsCompleted: countCompleted('pbsCompletedAtMs'),
        snsReady: ready.size,
        proofTimestamps: proofTimes.size,
      });
      await sleep(pollIntervalMs);
    }
  }
}

async function createBenchmarkDbClient() {
  const connectionString =
    process.env.ERC20_BENCH_DATABASE_URL ??
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
    resultHandle?: string;
    extra?: Record<string, unknown>;
  } = {},
): Promise<number> {
  const result = await client.query<{ event_at_ms: string }>(
    `
      INSERT INTO benchmark_timing_events (
        run_id, workload, item_index, tx_hash, input_handle, result_handle, event_name, event_at, extra
      )
      VALUES (
        $1,
        $2,
        $3,
        CASE WHEN $4::text IS NULL THEN NULL ELSE decode($4::text, 'hex') END,
        CASE WHEN $5::text IS NULL THEN NULL ELSE decode($5::text, 'hex') END,
        CASE WHEN $6::text IS NULL THEN NULL ELSE decode($6::text, 'hex') END,
        $7,
        clock_timestamp(),
        $8::jsonb
      )
      RETURNING (EXTRACT(EPOCH FROM event_at) * 1000)::bigint::text AS event_at_ms
    `,
    [
      runId,
      workload,
      options.itemIndex ?? null,
      options.txHash ? without0x(options.txHash) : null,
      options.inputHandle ? without0x(options.inputHandle) : null,
      options.resultHandle ? without0x(options.resultHandle) : null,
      eventName,
      JSON.stringify(options.extra ?? {}),
    ],
  );
  return Number(result.rows[0].event_at_ms);
}

async function readTransactionDbTimings(
  client: PgClient,
  txHashes: string[],
): Promise<Map<string, TransactionDbTiming>> {
  if (txHashes.length === 0) return new Map();
  const result = await client.query<{
    tx_hash: string;
    created_at_ms: string;
    completed_at_ms: string | null;
  }>(
    `
      SELECT encode(id, 'hex') AS tx_hash,
             (EXTRACT(EPOCH FROM created_at) * 1000)::bigint::text AS created_at_ms,
             (EXTRACT(EPOCH FROM completed_at) * 1000)::bigint::text AS completed_at_ms
      FROM transactions
      WHERE encode(id, 'hex') = ANY($1::text[])
    `,
    [[...new Set(txHashes.map(without0x))]],
  );
  return new Map(
    result.rows.map((row) => [
      row.tx_hash,
      {
        createdAtMs: Number(row.created_at_ms),
        completedAtMs: row.completed_at_ms === null ? null : Number(row.completed_at_ms),
      },
    ]),
  );
}

async function readProofVerificationDbTimings(
  client: PgClient,
  txHashes: string[],
): Promise<Map<string, ProofVerificationDbTiming>> {
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

async function readBenchmarkState(
  client: PgClient,
  inputHandles: string[],
  resultHandles: string[],
): Promise<BenchmarkState> {
  const ciphertexts = await client.query<{ handle: string }>(
    `
      SELECT encode(handle, 'hex') AS handle
      FROM ciphertext_digest
      WHERE ciphertext128 IS NOT NULL
        AND encode(handle, 'hex') = ANY($1::text[])
    `,
    [resultHandles],
  );
  const snsCt128Computed = await client.query<{
    handle: string;
    computed_at_ms: string;
  }>(
    `
      SELECT encode(handle, 'hex') AS handle,
             (EXTRACT(EPOCH FROM completed_at) * 1000)::bigint::text AS computed_at_ms
      FROM pbs_computations
      WHERE is_completed
        AND encode(handle, 'hex') = ANY($1::text[])
    `,
    [resultHandles],
  );
  const pbsCompleted = await client.query<{
    handle: string;
    completed_at_ms: string;
  }>(
    `
      SELECT encode(handle, 'hex') AS handle,
             (EXTRACT(EPOCH FROM completed_at) * 1000)::bigint::text AS completed_at_ms
      FROM pbs_computations
      WHERE is_completed
        AND encode(handle, 'hex') = ANY($1::text[])
    `,
    [resultHandles],
  );
  const tfheCiphertexts = await client.query<{
    handle: string;
    created_at_ms: string;
  }>(
    `
      SELECT encode(handle, 'hex') AS handle,
             (EXTRACT(EPOCH FROM min(created_at)) * 1000)::bigint::text AS created_at_ms
      FROM ciphertexts
      WHERE encode(handle, 'hex') = ANY($1::text[])
      GROUP BY handle
    `,
    [resultHandles],
  );
  const computationsCompleted = await client.query<{
    handle: string;
    completed_at_ms: string;
  }>(
    `
      SELECT encode(output_handle, 'hex') AS handle,
             (EXTRACT(EPOCH FROM max(completed_at)) * 1000)::bigint::text AS completed_at_ms
      FROM computations
      WHERE is_completed
        AND encode(output_handle, 'hex') = ANY($1::text[])
      GROUP BY output_handle
    `,
    [resultHandles],
  );
  let inputs: { rows: { handle: string; verified_at_ms: string }[] };
  try {
    inputs = await client.query<{ handle: string; verified_at_ms: string }>(
      `
        SELECT encode(handle, 'hex') AS handle,
               (EXTRACT(EPOCH FROM created_at) * 1000)::bigint::text AS verified_at_ms
        FROM input_handles
        WHERE encode(handle, 'hex') = ANY($1::text[])
      `,
      [inputHandles],
    );
  } catch (error) {
    if ((error as { code?: string }).code !== '42P01') {
      throw error;
    }
    inputs = { rows: [] };
  }

  return {
    readyHandles: new Set(ciphertexts.rows.map((row) => row.handle)),
    computationsCompleted: new Map(computationsCompleted.rows.map((row) => [row.handle, Number(row.completed_at_ms)])),
    tfheCiphertexts: new Map(tfheCiphertexts.rows.map((row) => [row.handle, Number(row.created_at_ms)])),
    pbsCompleted: new Map(pbsCompleted.rows.map((row) => [row.handle, Number(row.completed_at_ms)])),
    snsCt128Computed: new Map(snsCt128Computed.rows.map((row) => [row.handle, Number(row.computed_at_ms)])),
    verifiedInputs: new Map(inputs.rows.map((row) => [row.handle, Number(row.verified_at_ms)])),
  };
}

async function readPbsCompletionState(client: PgClient, blockNumbers: number[]): Promise<PbsCompletionState> {
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
      SELECT count(*)::text AS total,
             count(*) FILTER (WHERE is_completed)::text AS completed,
             count(*) FILTER (WHERE NOT is_completed)::text AS pending,
             (EXTRACT(EPOCH FROM min(p.created_at)) * 1000)::bigint::text AS min_created_at_ms,
             (EXTRACT(EPOCH FROM max(p.created_at)) * 1000)::bigint::text AS max_created_at_ms,
             (EXTRACT(EPOCH FROM min(p.completed_at)) * 1000)::bigint::text AS min_completed_at_ms,
             (EXTRACT(EPOCH FROM max(p.completed_at)) * 1000)::bigint::text AS max_completed_at_ms
      FROM pbs_computations p
      JOIN transactions t ON t.id = p.transaction_id
      WHERE t.block_number = ANY($1::bigint[])
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

async function readTfheCiphertextState(client: PgClient, blockNumbers: number[]): Promise<PbsCompletionState> {
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
        FROM ciphertexts
        GROUP BY handle
      )
      SELECT count(*)::text AS total,
             count(c.handle)::text AS completed,
             (count(*) - count(c.handle))::text AS pending,
             (EXTRACT(EPOCH FROM min(p.created_at)) * 1000)::bigint::text AS min_created_at_ms,
             (EXTRACT(EPOCH FROM max(p.created_at)) * 1000)::bigint::text AS max_created_at_ms,
             (EXTRACT(EPOCH FROM min(c.created_at)) * 1000)::bigint::text AS min_completed_at_ms,
             (EXTRACT(EPOCH FROM max(c.created_at)) * 1000)::bigint::text AS max_completed_at_ms
      FROM pbs_computations p
      JOIN transactions t ON t.id = p.transaction_id
      LEFT JOIN computed_ciphertexts c ON c.handle = p.handle
      WHERE t.block_number = ANY($1::bigint[])
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

async function readComputationsCompletionState(client: PgClient, blockNumbers: number[]): Promise<PbsCompletionState> {
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
      SELECT count(*)::text AS total,
             count(*) FILTER (WHERE is_completed)::text AS completed,
             count(*) FILTER (WHERE NOT is_completed AND NOT is_error)::text AS pending,
             (EXTRACT(EPOCH FROM min(c.created_at)) * 1000)::bigint::text AS min_created_at_ms,
             (EXTRACT(EPOCH FROM max(c.created_at)) * 1000)::bigint::text AS max_created_at_ms,
             (EXTRACT(EPOCH FROM min(c.completed_at)) * 1000)::bigint::text AS min_completed_at_ms,
             (EXTRACT(EPOCH FROM max(c.completed_at)) * 1000)::bigint::text AS max_completed_at_ms
      FROM computations c
      JOIN transactions t ON t.id = c.transaction_id
      WHERE t.block_number = ANY($1::bigint[])
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

async function readSnsCt128ComputedState(client: PgClient, handles: string[]): Promise<PbsCompletionState> {
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
      WITH expected(handle) AS (
        SELECT decode(unnest($1::text[]), 'hex')
      )
      SELECT count(*)::text AS total,
             count(c.handle)::text AS completed,
             (count(*) - count(c.handle))::text AS pending,
             NULL::text AS min_created_at_ms,
             NULL::text AS max_created_at_ms,
             (EXTRACT(EPOCH FROM min(c.completed_at)) * 1000)::bigint::text AS min_completed_at_ms,
             (EXTRACT(EPOCH FROM max(c.completed_at)) * 1000)::bigint::text AS max_completed_at_ms
      FROM expected e
      LEFT JOIN pbs_computations c ON c.handle = e.handle AND c.is_completed
    `,
    [handles],
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

describe('EncryptedERC20 benchmark', function () {
  it('benchmark independent ERC20 transfers', async function () {
    const transferCount = envInt('ERC20_BENCH_TRANSFERS', 1000);
    const completionTarget = envEnum<CompletionTarget>('ERC20_BENCH_COMPLETION_TARGET', 'sns-ready', [
      'sns-ready',
      'pbs-completed',
      'tfhe-ciphertexts',
      'computations-completed',
    ]);
    const transferConcurrency = envInt('ERC20_BENCH_TRANSFER_CONCURRENCY', 100);
    const transferSubmissionMode = envEnum<TransferSubmissionMode>(
      'ERC20_BENCH_TRANSFER_SUBMISSION_MODE',
      'await-receipt',
      ['await-receipt', 'burst', 'fixed-rate', 'manual-mine', 'steady-rate'],
    );
    const proofConcurrency = envInt('ERC20_BENCH_PROOF_CONCURRENCY', 10);
    const proofThreads = envInt(
      'ERC20_BENCH_PROOF_THREADS',
      Math.max(1, Math.floor(availableParallelism() / proofConcurrency)),
    );
    const fundConcurrency = envInt('ERC20_BENCH_FUND_CONCURRENCY', 200);
    const mintBatchSize = envInt('ERC20_BENCH_MINT_BATCH_SIZE', 25);
    const steadyTransfersPerBlock = envInt('ERC20_BENCH_STEADY_TRANSFERS_PER_BLOCK', transferConcurrency);
    // Offered arrival rate for the open-loop 'fixed-rate' mode.
    const targetTps = envInt('ERC20_BENCH_TARGET_TPS', 10);
    const manualMineTransfersPerBlock = envNonNegativeInt('ERC20_BENCH_MANUAL_MINE_TRANSFERS_PER_BLOCK', 0);
    const manualMineMaxBlocks = envInt('ERC20_BENCH_MANUAL_MINE_MAX_BLOCKS', 20);
    const manualMineIntervalSeconds = envInt('ERC20_BENCH_MANUAL_MINE_INTERVAL_SECONDS', 1);
    const manualMineRestoreIntervalSeconds = envNonNegativeInt('ERC20_BENCH_MANUAL_MINE_RESTORE_INTERVAL_SECONDS', 1);
    const pollIntervalMs = envInt('ERC20_BENCH_POLL_INTERVAL_MS', 1000);
    const timeoutMs = envInt('ERC20_BENCH_TIMEOUT_MS', 60 * 60 * 1000);
    const transferAmount = envInt('ERC20_BENCH_TRANSFER_AMOUNT', 1);
    const initialBalance = envInt('ERC20_BENCH_INITIAL_BALANCE', 100);
    const proofRetries = envInt('ERC20_BENCH_PROOF_RETRIES', 5);
    const proofRetryDelayMs = envInt('ERC20_BENCH_PROOF_RETRY_DELAY_MS', 1000);
    const proofWorkerTimeoutMs = envInt('ERC20_BENCH_PROOF_WORKER_TIMEOUT_MS', 10 * 60 * 1000);
    const proofCachePath = process.env.ERC20_BENCH_PROOF_CACHE;
    const useEmptyCachedInputProofs = process.env.ERC20_BENCH_USE_EMPTY_CACHED_INPUT_PROOFS === '1';
    // Organic traffic: instead of one throwaway account per transfer, replay a
    // small deck of accounts so each transfer consumes balance handles produced
    // by transfers `lag` L1 blocks earlier. Mirrors the native fixtures
    // traffic_1000_200x5_10x100_lag2 / traffic_join_1000_200x5_20acct_lag2.
    const trafficMode = process.env.ERC20_BENCH_TRAFFIC_MODE === '1';
    const trafficLagBlocks = envInt('ERC20_BENCH_TRAFFIC_LAG_BLOCKS', 2);
    const trafficJoinBraid = process.env.ERC20_BENCH_TRAFFIC_JOIN_BRAID === '1';
    // Setup minting queues thousands of FHE ops; measuring before they drain
    // reports the fixture's backlog as the workload's latency tail.
    // Input proofs reach the gateway off-chain and asynchronously, so their
    // verification is a separate service path from the L1 transfer pipeline.
    // Driving both at once is the only way to see what each costs the other.
    const zkRate = envNonNegativeInt('ERC20_BENCH_ZK_RATE', 0);
    const zkWorkers = envInt('ERC20_BENCH_ZK_WORKERS', 4);
    const drainBeforeTransfers = process.env.ERC20_BENCH_NO_DRAIN_WAIT !== '1';
    const drainTimeoutMs = envInt('ERC20_BENCH_DRAIN_TIMEOUT_MS', 5 * 60 * 1000);
    // The lag is expressed in blocks, so the schedule must be cut with the same
    // per-block count the submission loop uses. Fixed-rate arrivals run against
    // 1-second blocks, so a tick's worth of transfers (targetTps) is exactly one
    // block — which lets organic traffic be measured at a controlled offered
    // load rather than only at the manual-mine cadence.
    const trafficTransfersPerBlock =
      transferSubmissionMode === 'fixed-rate'
        ? targetTps
        : manualMineTransfersPerBlock > 0
          ? manualMineTransfersPerBlock
          : steadyTransfersPerBlock;
    const trafficSignature: TrafficSignature | undefined = trafficMode
      ? { joinBraid: trafficJoinBraid, lagBlocks: trafficLagBlocks, transfersPerBlock: trafficTransfersPerBlock }
      : undefined;
    const workerSdkAuth = relayerApiKey ? { __type: 'ApiKeyHeader' as const, value: relayerApiKey } : undefined;
    const workerSdkConfig: ProofWorkerSdkConfig = {
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
      numberOfThreads: proofThreads,
      ...(workerSdkAuth ? { auth: workerSdkAuth } : {}),
    };

    this.timeout(timeoutMs + 10 * 60 * 1000);

    benchLog('starting', {
      transferCount,
      completionTarget,
      transferSubmissionMode,
      transferConcurrency,
      steadyTransfersPerBlock,
      manualMineTransfersPerBlock,
      proofConcurrency,
      proofThreads,
      proofCachePath: proofCachePath ?? null,
      useEmptyCachedInputProofs,
    });
    await initSigners(1);
    const proofCache = await readProofCache(proofCachePath);
    const cachedEntryCount = Math.min(await usableProofCacheEntryCount(proofCache, transferAmount, trafficSignature), transferCount);
    const useProofCache = cachedEntryCount === transferCount;
    const extendProofCache = cachedEntryCount > 0 && cachedEntryCount < transferCount;
    benchLog('proof cache checked', {
      reused: useProofCache,
      extendable: extendProofCache,
      reusableEntries: cachedEntryCount,
      requestedEntries: transferCount,
      entries: proofCache?.entries.length ?? 0,
    });
    const erc20 = (
      cachedEntryCount > 0
        ? await ethers
            .getContractFactory('EncryptedERC20')
            .then((factory) => factory.attach(proofCache!.contractAddress))
        : await deployEncryptedERC20Fixture()
    ) as EncryptedERC20;
    const contractAddress = await erc20.getAddress();
    benchLog(cachedEntryCount > 0 ? 'attached contract' : 'deployed contract', {
      contractAddress,
    });
    const usedAddresses = new Set<string>();
    const cachedEntries = proofCache?.entries.slice(0, cachedEntryCount) ?? [];
    const senders: (HDNodeWallet | Wallet)[] = cachedEntries.map((entry) => {
      usedAddresses.add(entry.senderAddress.toLowerCase());
      usedAddresses.add(entry.recipientAddress.toLowerCase());
      return withGasBuffer(new ethers.Wallet(entry.senderPrivateKey, ethers.provider));
    });
    const recipientAddresses = cachedEntries.map((entry) => entry.recipientAddress);
    if (trafficMode && cachedEntryCount < transferCount) {
      // A full cache already replays the deck (each entry carries its sender
      // key), and its traffic signature was checked on load; only a partial one
      // has to be rebuilt from the schedule, and mixing the two would corrupt
      // the pattern.
      if (cachedEntries.length > 0) {
        throw new Error('ERC20_BENCH_TRAFFIC_MODE cannot extend a partial proof cache; delete it or use a full one');
      }
      const schedule = buildTrafficSchedule({
        transferCount,
        transfersPerBlock: trafficTransfersPerBlock,
        lagBlocks: trafficLagBlocks,
        joinBraid: trafficJoinBraid,
      });
      const senderPool = Array.from({ length: schedule.senderPoolSize }, () =>
        randomWalletWithUniqueAddress(usedAddresses),
      );
      // Braided traffic moves value inside the deck, so recipients are drawn
      // from the very same accounts; linear traffic gives each chain its own
      // counterpart.
      const recipientPool = trafficJoinBraid
        ? senderPool.map((wallet) => wallet.address)
        : Array.from({ length: schedule.recipientPoolSize }, () => randomUniqueAddress(usedAddresses));
      senders.push(...schedule.senderSlots.map((slot) => senderPool[slot]));
      recipientAddresses.push(...schedule.recipientSlots.map((slot) => recipientPool[slot]));
      benchLog('traffic schedule', {
        joinBraid: trafficJoinBraid,
        lagBlocks: trafficLagBlocks,
        transfersPerBlock: trafficTransfersPerBlock,
        senderPool: schedule.senderPoolSize,
        recipientPool: trafficJoinBraid ? 0 : schedule.recipientPoolSize,
        distinctAccounts: new Set([...senders, ...recipientAddresses].map((a) => (typeof a === 'string' ? a : a.address).toLowerCase())).size,
      });
    } else {
      while (senders.length < transferCount) {
        senders.push(randomWalletWithUniqueAddress(usedAddresses));
        recipientAddresses.push(randomUniqueAddress(usedAddresses));
      }
      // Traffic runs replaying a full cache land here with the deck already
      // restored, and a deck is meant to repeat addresses.
      if (!trafficMode) {
        const uniqueAddresses = new Set(
          [...senders.map((wallet) => wallet.address), ...recipientAddresses].map((address) => address.toLowerCase()),
        );
        expect(uniqueAddresses.size).to.equal(transferCount * 2);
      }
    }

    // Traffic mode replays a small deck, so `senders` repeats the same wallets;
    // fund each address once instead of once per transfer.
    const fundingAddresses = [...new Set(senders.map((wallet) => wallet.address))];
    await setWalletBalance(fundingAddresses, ethers.parseEther('1'), fundConcurrency);
    benchLog('funded senders', {
      senders: senders.length,
      distinctSenders: fundingAddresses.length,
      fundConcurrency,
    });

    const cachedPrepared: PreparedTransfer[] = cachedEntries.map(
      (entry, index): PreparedTransfer => ({
        index,
        sender: senders[index],
        recipientAddress: entry.recipientAddress,
        amountHandle: entry.amountHandle,
        inputProof: entry.inputProof,
      }),
    );
    const missingPrepared =
      cachedEntryCount === transferCount
        ? []
        : await retry(proofRetries, proofRetryDelayMs, () => {
            benchLog('starting worker proof generation', {
              workers: Math.min(proofConcurrency, transferCount - cachedEntryCount),
              cached: cachedEntryCount,
              total: transferCount,
            });
            return generateProofsWithWorkers({
              senders: senders.slice(cachedEntryCount),
              recipientAddresses: recipientAddresses.slice(cachedEntryCount),
              workerCount: proofConcurrency,
              contractAddress,
              transferAmount,
              sdkConfig: workerSdkConfig,
              workerTimeoutMs: proofWorkerTimeoutMs,
              jobRetries: proofRetries,
              retryDelayMs: proofRetryDelayMs,
            }).then((generated) =>
              generated.map((transfer, index) => ({
                ...transfer,
                index: cachedEntryCount + index,
              })),
            );
          });
    const prepared = [...cachedPrepared, ...missingPrepared];
    benchLog(useProofCache ? 'loaded prepared transfers' : 'prepared transfers', {
      prepared: prepared.length,
      cached: cachedPrepared.length,
      generated: missingPrepared.length,
    });

    if (!useProofCache && proofCachePath) {
      const chainId = await ethers.provider.getNetwork().then((network) => Number(network.chainId));
      benchLog('writing proof cache', {
        proofCachePath,
        entries: prepared.length,
      });
      await writeProofCache(proofCachePath, {
        version: 1,
        chainId,
        contractAddress,
        transferCount,
        transferAmount,
        ...(trafficSignature ? { traffic: trafficSignature } : {}),
        entries: prepared.map((transfer) => ({
          senderPrivateKey: transfer.sender.privateKey,
          senderAddress: transfer.sender.address,
          recipientAddress: transfer.recipientAddress,
          amountHandle: ethers.hexlify(transfer.amountHandle),
          inputProof: ethers.hexlify(transfer.inputProof),
        })),
      });
      benchLog('wrote proof cache', { proofCachePath });
    }

    // One mint per distinct account. A deck account sends many transfers, so a
    // flat initial balance would run dry part-way through the run and the tail
    // of the workload would silently degrade into no-op transfers.
    const sendsPerAddress = new Map<string, number>();
    for (const wallet of senders) {
      sendsPerAddress.set(wallet.address, (sendsPerAddress.get(wallet.address) ?? 0) + 1);
    }
    const maxSendsPerAddress = Math.max(...sendsPerAddress.values());
    const mintAmount = trafficMode ? initialBalance + maxSendsPerAddress * transferAmount : initialBalance;
    const mintAddresses = [...sendsPerAddress.keys()];
    if (trafficMode) {
      benchLog('traffic mint sizing', { distinctAccounts: mintAddresses.length, maxSendsPerAddress, mintAmount });
    }

    for (let i = 0; i < mintAddresses.length; i += mintBatchSize) {
      const batch = mintAddresses.slice(i, i + mintBatchSize);
      benchLog('submitting mint batch', {
        start: i,
        count: batch.length,
        total: mintAddresses.length,
      });
      // Explicit limit rather than eth_estimateGas + the signer's 50% buffer:
      // a batch of FHE.add/allowThis/allow per recipient has been observed to
      // exceed the estimate by more than that buffer and abort the whole run
      // with an out-of-gas revert. Minting is setup, not measured, so over-
      // provisioning costs nothing (unused gas is refunded to the block).
      const tx = await (
        erc20 as unknown as {
          mintToMany(
            addresses: string[],
            amount: number,
            overrides?: { gasLimit: bigint },
          ): Promise<TransactionResponse>;
        }
      ).mintToMany(batch, mintAmount, { gasLimit: BigInt(500_000 + 400_000 * batch.length) });
      const receipt = await waitForTransactionReceipt(tx, timeoutMs);
      expect(receipt.status).to.equal(1);
      benchLog('mined mint batch', {
        start: i,
        count: batch.length,
        blockNumber: receipt.blockNumber,
      });
    }

    // Minting 1000 accounts queues ~6000 FHE operations, and the coprocessor is
    // still draining them when the first transfers arrive. Measured from a
    // non-empty pipeline that shows up as a startup transient — at 5 tx/s the
    // first 100 transfers averaged 6.7 s against a 2.0 s steady state, which
    // lands entirely in the run's p95/p99 and reads as a latency tail the system
    // does not actually have. Wait for setup work to clear so the run measures
    // the workload rather than its own fixture.
    let db: PgClient | undefined = await createBenchmarkDbClient();
    if (drainBeforeTransfers && db) {
      // Wait for the queue to stop shrinking rather than for it to hit zero:
      // runs that share a contract can leave permanently-stuck work behind
      // (inputs that no computation will ever produce), and insisting on zero
      // burns the whole timeout on every subsequent run.
      const drainDeadline = Date.now() + drainTimeoutMs;
      const startedDrainAt = Date.now();
      let pendingSetup = Number.MAX_SAFE_INTEGER;
      let stableRounds = 0;
      for (;;) {
        const { rows } = await db.query<{ pending: string }>(
          'SELECT count(*)::text AS pending FROM computations WHERE NOT is_completed',
        );
        const pending = Number(rows[0]?.pending ?? 0);
        stableRounds = pending < pendingSetup ? 0 : stableRounds + 1;
        pendingSetup = pending;
        if (pending === 0 || stableRounds >= 6 || Date.now() > drainDeadline) break;
        await sleep(500);
      }
      benchLog('setup drain finished', {
        pendingSetup,
        stalled: pendingSetup > 0,
        waitedMs: Date.now() - startedDrainAt,
      });
    }

    // (gateway proof load is summarised after the transfer phase)
    // Clock starts only once the pipeline is empty.
    const benchmarkRunId = `erc20-${new Date().toISOString().replace(/[:.]/g, '-')}-${process.pid}`;
    const startedAtMs = Date.now();
    await ensureBenchmarkTimingTable(db);
    const startedAtDbMs = await recordBenchmarkEvent(db, benchmarkRunId, 'erc20', 'run_started');

    const sendTransfer = async (preparedTransfer: PreparedTransfer) => {
      const contractFromSender = erc20.connect(preparedTransfer.sender);
      const submittedAtMs = Date.now();
      const tx = await contractFromSender['transfer(address,bytes32,bytes)'](
        preparedTransfer.recipientAddress,
        preparedTransfer.amountHandle,
        useProofCache && useEmptyCachedInputProofs ? '0x' : preparedTransfer.inputProof,
      );
      const submittedAtDbMs = await recordBenchmarkEvent(db!, benchmarkRunId, 'erc20', 'tx_submitted', {
        itemIndex: preparedTransfer.index,
        txHash: tx.hash,
        inputHandle: ethers.hexlify(preparedTransfer.amountHandle),
      });
      return { preparedTransfer, tx, submittedAtMs, submittedAtDbMs };
    };

    const awaitReceipt = async (
      preparedTransfer: PreparedTransfer,
      tx: TransactionResponse,
      submittedAtMs: number,
      submittedAtDbMs?: number,
    ): Promise<TransferTiming & { blockNumber: number }> => {
      const receipt = await waitForTransactionReceipt(tx, timeoutMs);
      const minedAtMs = Date.now();
      expect(receipt.status).to.equal(1);
      expect(receipt.blockNumber).to.not.equal(null);
      const resultHandle = await erc20.balanceOf(preparedTransfer.recipientAddress);
      const receiptObservedAtDbMs = await recordBenchmarkEvent(db!, benchmarkRunId, 'erc20', 'receipt_observed', {
        itemIndex: preparedTransfer.index,
        txHash: tx.hash,
        inputHandle: ethers.hexlify(preparedTransfer.amountHandle),
        resultHandle,
        extra: { blockNumber: receipt.blockNumber },
      });
      return {
        index: preparedTransfer.index,
        txHash: tx.hash,
        inputHandle: ethers.hexlify(preparedTransfer.amountHandle),
        resultHandle,
        submittedAtMs,
        submittedAtDbMs,
        minedAtMs,
        receiptObservedAtDbMs,
        blockNumber: receipt.blockNumber,
      };
    };

    let timingsWithBlocks: (TransferTiming & { blockNumber: number })[];
    const observedTimings: TransferTiming[] = [];
    const ready = new Set<string>();
    const proofTimes = new Map<string, number>();
    let pipelinePoll: Promise<void> | undefined;
    let stopPipelinePoll = false;
    // Concurrent gateway proof load. Senders are throwaway addresses: a proof
    // commits to (contract, user) and needs no funds, so this exercises the
    // verification path without perturbing the L1 balances under measurement.
    const zkLatencies: number[] = [];
    let zkPromise: Promise<PreparedTransfer[]> | undefined;
    let zkStartedAtMs = 0;
    let zkRequested = 0;
    if (zkRate > 0) {
      const expectedTransferSeconds =
        transferSubmissionMode === 'fixed-rate' ? transferCount / targetTps : 60;
      zkRequested = Math.max(1, Math.round(zkRate * expectedTransferSeconds));
      const zkSenders = Array.from({ length: zkRequested }, () => randomWalletWithUniqueAddress(usedAddresses));
      const zkRecipients = zkSenders.map(() => randomUniqueAddress(usedAddresses));
      benchLog('starting concurrent gateway proof load', {
        zkRate,
        zkRequested,
        zkWorkers,
        overSeconds: Math.round(expectedTransferSeconds),
      });
      zkStartedAtMs = Date.now();
      zkPromise = generateProofsWithWorkers({
        senders: zkSenders,
        recipientAddresses: zkRecipients,
        workerCount: zkWorkers,
        contractAddress,
        transferAmount,
        sdkConfig: workerSdkConfig,
        workerTimeoutMs: proofWorkerTimeoutMs,
        jobRetries: proofRetries,
        retryDelayMs: proofRetryDelayMs,
        rate: zkRate,
        onProofLatency: (ms) => zkLatencies.push(ms),
      }).catch((error) => {
        benchLog('gateway proof load failed', { error: String(error).slice(0, 200) });
        return [] as PreparedTransfer[];
      });
    }

    let automineDisabled = false;
    let intervalMiningChanged = false;
    let manualMineBlocks = 0;
    // Records what load was actually offered, so a run that failed to sustain
    // its target rate is visibly distinguishable from one that did.
    let fixedRateStats:
      | { targetTps: number; maxDriftMs: number; offeredSeconds: number; offeredTps: number }
      | undefined;
    try {
      pipelinePoll = pollPipelineProgress(
        db,
        observedTimings,
        transferCount,
        completionTarget,
        ready,
        proofTimes,
        timeoutMs,
        pollIntervalMs,
        benchmarkRunId,
        () => stopPipelinePoll,
      );

      if (transferSubmissionMode === 'manual-mine') {
        if (manualMineIntervalSeconds > 1) {
          await setAnvilIntervalMining(manualMineIntervalSeconds);
          intervalMiningChanged = true;
        }
        await setAnvilAutomine(false);
        automineDisabled = true;
      } else if (transferSubmissionMode === 'steady-rate' || transferSubmissionMode === 'fixed-rate') {
        await setAnvilIntervalMining(1);
        intervalMiningChanged = true;
        await setAnvilAutomine(false);
        automineDisabled = true;
      }

      const useManualMineBlocks = transferSubmissionMode === 'manual-mine' && manualMineTransfersPerBlock > 0;
      const submitted =
        transferSubmissionMode === 'await-receipt' ||
        transferSubmissionMode === 'steady-rate' ||
        transferSubmissionMode === 'fixed-rate' ||
        useManualMineBlocks
          ? []
          : await mapLimit(prepared, transferConcurrency, sendTransfer);
      benchLog('submitted transfers', {
        submitted: transferSubmissionMode === 'await-receipt' ? 0 : submitted.length,
        transferSubmissionMode,
      });

      let manualBlockTimings: (TransferTiming & { blockNumber: number })[] | undefined;
      let fixedRateTimings: (TransferTiming & { blockNumber: number })[] | undefined;
      if (useManualMineBlocks) {
        const expectedBlocks = Math.ceil(prepared.length / manualMineTransfersPerBlock);
        if (expectedBlocks > manualMineMaxBlocks) {
          throw new Error(
            `Manual-mine block size requires ${expectedBlocks} blocks, above ERC20_BENCH_MANUAL_MINE_MAX_BLOCKS=${manualMineMaxBlocks}`,
          );
        }
        const result = await submitTransfersInManualBlocks(
          prepared,
          manualMineTransfersPerBlock,
          transferConcurrency,
          sendTransfer,
          awaitReceipt,
          (receipts) => observedTimings.push(...receipts),
        );
        manualBlockTimings = result.timings;
        manualMineBlocks = result.minedBlocks;
        benchLog('manual mined transfer blocks', {
          manualMineBlocks,
          transfersPerBlock: manualMineTransfersPerBlock,
          submitted: prepared.length,
        });
      } else if (transferSubmissionMode === 'manual-mine') {
        manualMineBlocks = await mineUntilSubmittedTransfersHaveReceipts(submitted, manualMineMaxBlocks);
        benchLog('manual mined transfers', {
          manualMineBlocks,
          submitted: submitted.length,
        });
      }

      if (transferSubmissionMode === 'fixed-rate') {
        const result = await submitTransfersAtFixedRate(
          prepared,
          targetTps,
          sendTransfer,
          awaitReceipt,
          (receipts) => observedTimings.push(...receipts),
        );
        fixedRateTimings = result.timings;
        fixedRateStats = {
          targetTps,
          maxDriftMs: result.maxDriftMs,
          offeredSeconds: Number(result.offeredSeconds.toFixed(2)),
          offeredTps: Number((prepared.length / result.offeredSeconds).toFixed(2)),
        };
        benchLog('fixed-rate submission complete', fixedRateStats);
      }

      timingsWithBlocks =
        manualBlockTimings ??
        (transferSubmissionMode === 'fixed-rate'
          ? fixedRateTimings!
          : transferSubmissionMode === 'steady-rate'
          ? await submitTransfersAtSteadyRate(
              prepared,
              steadyTransfersPerBlock,
              sendTransfer,
              awaitReceipt,
              (receipts) => observedTimings.push(...receipts),
            )
          : transferSubmissionMode === 'await-receipt'
            ? await mapLimit(prepared, transferConcurrency, async (preparedTransfer) => {
                const { tx, submittedAtMs, submittedAtDbMs } = await sendTransfer(preparedTransfer);
                const timing = await awaitReceipt(preparedTransfer, tx, submittedAtMs, submittedAtDbMs);
                observedTimings.push(timing);
                return timing;
              })
            : await mapLimit(
                submitted,
                transferConcurrency,
                async ({ preparedTransfer, tx, submittedAtMs, submittedAtDbMs }) => {
                  const timing = await awaitReceipt(preparedTransfer, tx, submittedAtMs, submittedAtDbMs);
                  observedTimings.push(timing);
                  return timing;
                },
              ));

      await pipelinePoll;
    } finally {
      stopPipelinePoll = true;
      if (intervalMiningChanged) {
        await setAnvilIntervalMining(manualMineRestoreIntervalSeconds);
      }
      if (automineDisabled) {
        await setAnvilAutomine(true);
      }
    }
    const timings: TransferTiming[] = timingsWithBlocks.map(({ blockNumber: _blockNumber, ...timing }) => timing);
    const blockNumbers = timingsWithBlocks.map((timing) => timing.blockNumber);
    benchLog('collected transfer receipts', {
      transfers: timings.length,
      blocks: summarizeBlocks(blockNumbers),
    });

    const transactionDbTimings = await readTransactionDbTimings(
      db!,
      timings.map((timing) => timing.txHash),
    );
    const proofVerificationDbTimings = await readProofVerificationDbTimings(
      db!,
      timings.map((timing) => timing.txHash),
    );
    for (const timing of timings) {
      const dbTiming = transactionDbTimings.get(without0x(timing.txHash));
      if (dbTiming) {
        timing.hostTransactionCreatedAtMs = dbTiming.createdAtMs;
        timing.hostTransactionCompletedAtMs = dbTiming.completedAtMs ?? undefined;
      }
      const proofTiming = proofVerificationDbTimings.get(without0x(timing.txHash));
      if (proofTiming) {
        timing.proofRequestedAtMs = proofTiming.requestedAtMs;
        timing.proofVerifiedAtMs = proofTiming.verifiedAtMs ?? undefined;
      }
    }

    const resultHandles = timings.map((timing) => without0x(timing.resultHandle));
    let pbsCompletionState: PbsCompletionState | null = null;
    let pbsCompletionObservedAtMs: number | null = null;
    let computationsStage: PbsCompletionState | null = null;
    let pbsStage: PbsCompletionState | null = null;
    let tfheCiphertextsStage: PbsCompletionState | null = null;
    let snsCt128ComputedStage: PbsCompletionState | null = null;

    try {
      db ??= await createBenchmarkDbClient();
      computationsStage = await readComputationsCompletionState(db, blockNumbers);
      pbsStage = await readPbsCompletionState(db, blockNumbers);
      tfheCiphertextsStage = await readTfheCiphertextState(db, blockNumbers);
      snsCt128ComputedStage = await readSnsCt128ComputedState(db, resultHandles);

      if (completionTarget === 'pbs-completed') {
        pbsCompletionState = pbsStage;
        pbsCompletionObservedAtMs = Math.max(...timings.map((timing) => timing.pbsCompletedAtMs ?? 0));
      } else if (completionTarget === 'tfhe-ciphertexts') {
        pbsCompletionState = tfheCiphertextsStage;
        pbsCompletionObservedAtMs = Math.max(...timings.map((timing) => timing.tfheCiphertextAtMs ?? 0));
      } else if (completionTarget === 'computations-completed') {
        pbsCompletionState = computationsStage;
        pbsCompletionObservedAtMs = Math.max(...timings.map((timing) => timing.computationsCompletedAtMs ?? 0));
      }
    } finally {
      await db?.end();
    }

    if (!useProofCache) {
      for (const timing of timings) {
        timing.proofVerifiedAtMs ??= proofTimes.get(without0x(timing.inputHandle));
      }
    }

    const targetCompletedAt = (timing: TransferTiming) => {
      if (completionTarget === 'pbs-completed') return timing.pbsCompletedAtMs;
      if (completionTarget === 'tfhe-ciphertexts') return timing.tfheCiphertextAtMs;
      if (completionTarget === 'computations-completed') return timing.computationsCompletedAtMs;
      return timing.snsReadyAtMs;
    };
    const computeStartedAt = (timing: TransferTiming) => timing.hostTransactionCreatedAtMs;
    const submittedAt = (timing: TransferTiming) => timing.submittedAtDbMs;
    const targetCompletionTimes = timings
      .map((timing) => targetCompletedAt(timing))
      .filter((value): value is number => value !== undefined);
    const completedAtMs = targetCompletionTimes.length > 0 ? Math.max(...targetCompletionTimes) : startedAtDbMs;
    const endToEndLatencies = timings
      .filter((timing) => timing.proofVerifiedAtMs !== undefined && timing.snsReadyAtMs !== undefined)
      .map((timing) => timing.snsReadyAtMs! - timing.proofVerifiedAtMs!);
    const proofRequestedToVerifiedLatencies = timings
      .filter((timing) => timing.proofRequestedAtMs !== undefined && timing.proofVerifiedAtMs !== undefined)
      .map((timing) => timing.proofVerifiedAtMs! - timing.proofRequestedAtMs!);
    const proofVerifiedToSnsCt128ComputedLatencies = timings
      .filter((timing) => timing.proofVerifiedAtMs !== undefined && timing.snsCt128ComputedAtMs !== undefined)
      .map((timing) => timing.snsCt128ComputedAtMs! - timing.proofVerifiedAtMs!);
    const targetMinedLatencies = timings
      .filter((timing) => targetCompletedAt(timing) !== undefined && computeStartedAt(timing) !== undefined)
      .map((timing) => targetCompletedAt(timing)! - computeStartedAt(timing)!);
    const targetSubmittedLatencies = timings
      .filter((timing) => targetCompletedAt(timing) !== undefined && submittedAt(timing) !== undefined)
      .map((timing) => targetCompletedAt(timing)! - submittedAt(timing)!);
    const minedToComputationsCompletedLatencies = timings
      .filter((timing) => timing.computationsCompletedAtMs !== undefined && computeStartedAt(timing) !== undefined)
      .map((timing) => timing.computationsCompletedAtMs! - computeStartedAt(timing)!);
    const submittedToComputationsCompletedLatencies = timings
      .filter((timing) => timing.computationsCompletedAtMs !== undefined && submittedAt(timing) !== undefined)
      .map((timing) => timing.computationsCompletedAtMs! - submittedAt(timing)!);
    const minedToTfheCiphertextLatencies = timings
      .filter((timing) => timing.tfheCiphertextAtMs !== undefined && computeStartedAt(timing) !== undefined)
      .map((timing) => timing.tfheCiphertextAtMs! - computeStartedAt(timing)!);
    const submittedToTfheCiphertextLatencies = timings
      .filter((timing) => timing.tfheCiphertextAtMs !== undefined && submittedAt(timing) !== undefined)
      .map((timing) => timing.tfheCiphertextAtMs! - submittedAt(timing)!);
    const minedToPbsCompletedLatencies = timings
      .filter((timing) => timing.pbsCompletedAtMs !== undefined && computeStartedAt(timing) !== undefined)
      .map((timing) => timing.pbsCompletedAtMs! - computeStartedAt(timing)!);
    const submittedToPbsCompletedLatencies = timings
      .filter((timing) => timing.pbsCompletedAtMs !== undefined && submittedAt(timing) !== undefined)
      .map((timing) => timing.pbsCompletedAtMs! - submittedAt(timing)!);
    const minedToSnsReadyLatencies = timings
      .filter((timing) => timing.snsReadyAtMs !== undefined && computeStartedAt(timing) !== undefined)
      .map((timing) => timing.snsReadyAtMs! - computeStartedAt(timing)!);
    const submittedToSnsReadyLatencies = timings
      .filter((timing) => timing.snsReadyAtMs !== undefined && submittedAt(timing) !== undefined)
      .map((timing) => timing.snsReadyAtMs! - submittedAt(timing)!);
    const minedToSnsCt128ComputedLatencies = timings
      .filter((timing) => timing.snsCt128ComputedAtMs !== undefined && computeStartedAt(timing) !== undefined)
      .map((timing) => timing.snsCt128ComputedAtMs! - computeStartedAt(timing)!);
    const submittedToSnsCt128ComputedLatencies = timings
      .filter((timing) => timing.snsCt128ComputedAtMs !== undefined && submittedAt(timing) !== undefined)
      .map((timing) => timing.snsCt128ComputedAtMs! - submittedAt(timing)!);
    const localMinedToTargetLatencies = timings
      .filter((timing) => targetCompletedAt(timing) !== undefined)
      .map((timing) => targetCompletedAt(timing)! - timing.minedAtMs);
    const localSubmittedToTargetLatencies = timings
      .filter((timing) => targetCompletedAt(timing) !== undefined)
      .map((timing) => targetCompletedAt(timing)! - timing.submittedAtMs);
    const wallSeconds = (completedAtMs - startedAtDbMs) / 1000;
    const pbsDbSeconds =
      pbsCompletionState?.minCreatedAtMs != null && pbsCompletionState.maxCompletedAtMs != null
        ? (pbsCompletionState.maxCompletedAtMs - pbsCompletionState.minCreatedAtMs) / 1000
        : null;
    const pbsCompletion = pbsCompletionState
      ? {
          ...pbsCompletionState,
          dbCreatedToCompletedSeconds: pbsDbSeconds,
          completedItemsPerSecond:
            pbsDbSeconds && pbsDbSeconds > 0 ? pbsCompletionState.completed / pbsDbSeconds : null,
          pbsComputationsPerSecond:
            completionTarget === 'pbs-completed' && pbsDbSeconds && pbsDbSeconds > 0
              ? pbsCompletionState.completed / pbsDbSeconds
              : null,
          tfheCiphertextsPerSecond:
            completionTarget === 'tfhe-ciphertexts' && pbsDbSeconds && pbsDbSeconds > 0
              ? pbsCompletionState.completed / pbsDbSeconds
              : null,
          computationsPerSecond:
            completionTarget === 'computations-completed' && pbsDbSeconds && pbsDbSeconds > 0
              ? pbsCompletionState.completed / pbsDbSeconds
              : null,
          transferEquivalentPerSecond: pbsDbSeconds && pbsDbSeconds > 0 ? transferCount / pbsDbSeconds : null,
          observedAtMs: pbsCompletionObservedAtMs,
        }
      : null;
    let zkSummary: Record<string, unknown> | null = null;
    if (zkPromise) {
      const zkResults = await zkPromise;
      const zkSeconds = (Date.now() - zkStartedAtMs) / 1000;
      zkSummary = {
        targetRate: zkRate,
        requested: zkRequested,
        completed: zkResults.length,
        achievedRate: Number((zkResults.length / zkSeconds).toFixed(2)),
        seconds: Number(zkSeconds.toFixed(1)),
        // Client-observed gateway round-trip: client-side proving plus the
        // coprocessor verification it triggers. The server-side portion alone
        // is in verify_proofs (created_at -> verified_at).
        latency: summarize(zkLatencies),
      };
      benchLog('gateway proof load complete', zkSummary);
    }

    const report = {
      benchmarkRunId,
      gatewayProofLoad: zkSummary,
      transferCount,
      // Traffic mode replays a deck, so the wallet count is the deck, not 2/transfer.
      walletCount: trafficMode ? sendsPerAddress.size : transferCount * 2,
      traffic: trafficMode
        ? { mode: trafficJoinBraid ? 'join-braid' : 'linear', lagBlocks: trafficLagBlocks, accounts: sendsPerAddress.size }
        : null,
      completionTarget,
      concurrency: {
        transfer: transferConcurrency,
        steadyTransfersPerBlock,
        manualMineTransfersPerBlock,
        proofGeneration: proofConcurrency,
        proofRetries,
        funding: fundConcurrency,
        mintBatchSize,
      },
      transferSubmissionMode,
      // Null unless the run used open-loop fixed-rate arrivals.
      fixedRate: fixedRateStats ?? null,
      manualMineBlocks,
      manualMineMaxBlocks,
      manualMineIntervalSeconds,
      transferBlocks: summarizeBlocks(blockNumbers),
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
        computationsCompletedAt: timings.filter((timing) => timing.computationsCompletedAtMs !== undefined).length,
        tfheCiphertextAt: timings.filter((timing) => timing.tfheCiphertextAtMs !== undefined).length,
        pbsCompletedAt: timings.filter((timing) => timing.pbsCompletedAtMs !== undefined).length,
        snsCt128ComputedAt: timings.filter((timing) => timing.snsCt128ComputedAtMs !== undefined).length,
        snsReadyObservedAt: timings.filter((timing) => timing.snsReadyAtMs !== undefined).length,
      },
      throughput: {
        snsReadyTransfersPerSecond: completionTarget === 'sns-ready' ? transferCount / wallSeconds : null,
        pbsCompletedTransfersPerSecond: completionTarget === 'pbs-completed' ? transferCount / wallSeconds : null,
        tfheCiphertextTransfersPerSecond: completionTarget === 'tfhe-ciphertexts' ? transferCount / wallSeconds : null,
        computationsCompletedTransfersPerSecond:
          completionTarget === 'computations-completed' ? transferCount / wallSeconds : null,
        wallSeconds,
      },
      pbsCompletion,
      stageBreakdown: {
        computations: withDbStageRates(computationsStage, transferCount),
        pbsComputations: withDbStageRates(pbsStage, transferCount),
        tfheCiphertexts: withDbStageRates(tfheCiphertextsStage, transferCount),
        snsCt128Computed: withDbStageRates(snsCt128ComputedStage, transferCount),
      },
      latency: {
        proofRequestedToVerified: summarize(proofRequestedToVerifiedLatencies),
        proofVerifiedToSnsReady: completionTarget === 'sns-ready' ? summarize(endToEndLatencies) : summarize([]),
        proofVerifiedToSnsCt128Computed: summarize(proofVerifiedToSnsCt128ComputedLatencies),
        minedToSnsReady: summarize(minedToSnsReadyLatencies),
        submittedToSnsReady: summarize(submittedToSnsReadyLatencies),
        minedToSnsCt128Computed: summarize(minedToSnsCt128ComputedLatencies),
        submittedToSnsCt128Computed: summarize(submittedToSnsCt128ComputedLatencies),
        minedToPbsCompleted: summarize(minedToPbsCompletedLatencies),
        submittedToPbsCompleted: summarize(submittedToPbsCompletedLatencies),
        minedToTfheCiphertexts: summarize(minedToTfheCiphertextLatencies),
        submittedToTfheCiphertexts: summarize(submittedToTfheCiphertextLatencies),
        minedToComputationsCompleted: summarize(minedToComputationsCompletedLatencies),
        submittedToComputationsCompleted: summarize(submittedToComputationsCompletedLatencies),
        targetMinedToCompleted: summarize(targetMinedLatencies),
        targetSubmittedToCompleted: summarize(targetSubmittedLatencies),
      },
      harnessClockDiagnostics: {
        startedAtMs,
        startedAtDbMs,
        targetLocalMinedToCompleted: summarize(localMinedToTargetLatencies),
        targetLocalSubmittedToCompleted: summarize(localSubmittedToTargetLatencies),
      },
      proofVerificationTimestampsObserved: endToEndLatencies.length,
      missingProofVerificationTimestamps: transferCount - endToEndLatencies.length,
      proofVerificationTimestampSource: useProofCache
        ? 'not measured for cached input proofs'
        : 'verify_proofs.verified_at by transaction_id, falling back to input_handles.created_at',
      proofRequestTimestampSource: 'verify_proofs.created_at by transaction_id',
      submittedTimestampSource:
        'benchmark_timing_events.event_at for tx_submitted, recorded by Postgres clock_timestamp()',
      minedTimestampSource:
        'transactions.created_at for the host-chain transaction, recorded by the host-listener in Postgres after block ingestion',
      receiptObservedTimestampSource:
        'benchmark_timing_events.event_at for receipt_observed, recorded by Postgres clock_timestamp()',
      snsCt128ComputedTimestampSource: 'pbs_computations.completed_at for the transfer result handle',
      snsReadyTimestampSource:
        completionTarget === 'sns-ready'
          ? 'benchmark_timing_events.event_at when the harness observed ciphertext_digest.ciphertext128 non-null'
          : null,
      pollIntervalMs,
    };

    console.log(`ERC20_BENCHMARK_REPORT ${JSON.stringify(report)}`);
    if (
      completionTarget === 'pbs-completed' ||
      completionTarget === 'tfhe-ciphertexts' ||
      completionTarget === 'computations-completed'
    ) {
      expect(pbsCompletionState?.pending).to.equal(0);
    } else {
      expect(ready.size).to.equal(transferCount);
    }
  });
});
