/**
 * Per-chain polling worker. Live-events-only — the daemon never catches up
 * missed events across restarts.
 *
 * On start:
 *   - Read `cursor = latestBlockNumber` once and hold it in memory.
 *
 * Each poll cycle:
 *   1. Read `head = latestBlockNumber`.
 *   2. If `head > cursor`:
 *      a. eth_getLogs for FHEVMExecutor and ConfidentialBridge in
 *         `[cursor+1, min(head, cursor+1+maxBlockRange)]`.
 *      b. Parse + sort by (blockNumber, transactionIndex, logIndex) so
 *         events within a tx land in order (the FHE handler depends on
 *         operand events being applied before derived ones).
 *      c. Dispatch each log to the right handler.
 *      d. Advance `cursor`.
 *   3. Replay the cross-chain pending bridge queue.
 *   4. Sleep `pollIntervalMs` if at head; otherwise loop immediately.
 *
 * Errors:
 *   - On any RPC / handler error, log + sleep `errorBackoffMs`, then retry.
 *     We do NOT advance the cursor on failure, so re-processing is automatic
 *     and idempotent (`INSERT OR IGNORE` in the DB).
 *
 * Shutdown:
 *   - Stops cleanly when the AbortSignal is fired (SIGINT/SIGTERM from
 *     index.ts).
 */
import { Log, ethers } from 'ethers';

import { BRIDGE_EVENTS_ABI, FHEVM_EXECUTOR_EVENTS_ABI } from './abi/events';
import { ChainConfig, RUNTIME } from './config';
import type { MockDb } from './db';
import { BridgeEvent, applyBridgeEvent, pendingBridgeCount, retryPendingBridges } from './handlers/bridge';
import { ExecutorEvent, applyExecutorEvent } from './handlers/fhe-executor';

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

interface Logger {
  info(msg: string): void;
  warn(msg: string): void;
  error(msg: string, err?: unknown): void;
}

function prefixedLogger(prefix: string): Logger {
  return {
    info: (m) => console.log(`[${prefix}] ${m}`),
    warn: (m) => console.warn(`[${prefix}] WARN ${m}`),
    error: (m, err) => console.error(`[${prefix}] ERROR ${m}`, err ?? ''),
  };
}

interface ParsedLog {
  blockNumber: number;
  transactionIndex: number;
  logIndex: number;
  source: 'executor' | 'bridge';
  event: ExecutorEvent | BridgeEvent | null;
}

function parseLog(
  log: Log,
  source: 'executor' | 'bridge',
  executor: ethers.Interface,
  bridge: ethers.Interface
): ParsedLog {
  let parsed: { name: string; args: ethers.Result } | null = null;
  try {
    const iface = source === 'executor' ? executor : bridge;
    const result = iface.parseLog({ topics: log.topics as string[], data: log.data });
    if (result) parsed = { name: result.name, args: result.args };
  } catch {
    // Unknown event signature — skip silently (might be from an OZ proxy or
    // a future operator we don't model yet).
  }
  return {
    blockNumber: log.blockNumber,
    transactionIndex: log.transactionIndex,
    logIndex: log.index,
    source,
    event: parsed ? { eventName: parsed.name, args: parsed.args } : null,
  };
}

export async function runChainWorker(chain: ChainConfig, db: MockDb, abort: AbortSignal): Promise {
  const logger = prefixedLogger(`mock-coprocessor:${chain.name}`);
  const provider = new ethers.JsonRpcProvider(chain.rpcUrl);
  const executorIface = new ethers.Interface(FHEVM_EXECUTOR_EVENTS_ABI);
  const bridgeIface = new ethers.Interface(BRIDGE_EVENTS_ABI);

  logger.info(`starting: rpc=${chain.rpcUrl} executor=${chain.fhevmExecutor} bridge=${chain.confidentialBridge}`);

  // The daemon is live-only: each session starts at the current chain head and
  // never catches up missed events. The cursor lives in memory; restarts get
  // a fresh head. To verify a tx, submit it AFTER the worker has logged this
  // initial head line.
  let cursor = await provider.getBlockNumber();
  logger.info(`initialised at chain head ${cursor} — only events from block ${cursor + 1} onwards will be processed`);

  while (!abort.aborted) {
    try {
      const head = await provider.getBlockNumber();
      const fromBlock = cursor + 1;
      if (fromBlock > head) {
        // Caught up — replay pending bridge mappings then sleep.
        if (pendingBridgeCount() > 0) await retryPendingBridges(db, console);
        await sleep(RUNTIME.pollIntervalMs);
        continue;
      }
      const toBlock = Math.min(head, fromBlock + RUNTIME.maxBlockRange - 1);

      const [execLogs, bridgeLogs] = await Promise.all([
        provider.getLogs({ address: chain.fhevmExecutor, fromBlock, toBlock }),
        provider.getLogs({ address: chain.confidentialBridge, fromBlock, toBlock }),
      ]);

      const parsed: ParsedLog[] = [
        ...execLogs.map((l) => parseLog(l, 'executor', executorIface, bridgeIface)),
        ...bridgeLogs.map((l) => parseLog(l, 'bridge', executorIface, bridgeIface)),
      ];
      parsed.sort(
        (a, b) => a.blockNumber - b.blockNumber || a.transactionIndex - b.transactionIndex || a.logIndex - b.logIndex
      );

      let inserted = 0;
      let skipped = 0;
      let pending = 0;
      for (const entry of parsed) {
        if (!entry.event) {
          skipped += 1;
          continue;
        }
        try {
          if (entry.source === 'executor') {
            const out = await applyExecutorEvent(entry.event, db);
            if (out === 'inserted') inserted += 1;
          } else {
            const out = await applyBridgeEvent(entry.event, db);
            if (out === 'inserted') inserted += 1;
            else if (out === 'pending') pending += 1;
          }
        } catch (err) {
          logger.error(
            `failed to process ${entry.source} event ${entry.event.eventName} at block ${entry.blockNumber} log ${entry.logIndex}`,
            err
          );
          // Continue with siblings — one broken event shouldn't stall the cursor.
        }
      }

      cursor = toBlock;
      // Replay pending bridges every cycle so cross-chain races resolve.
      if (pendingBridgeCount() > 0) await retryPendingBridges(db, console);

      logger.info(
        `processed blocks ${fromBlock}-${toBlock} (head=${head}): inserted=${inserted} pending=${pending} skipped=${skipped}`
      );

      if (toBlock === head) {
        await sleep(RUNTIME.pollIntervalMs);
      }
      // Otherwise loop immediately to keep catching up.
    } catch (err) {
      logger.error('poll cycle failed', err);
      await sleep(RUNTIME.errorBackoffMs);
    }
  }
  logger.info('shutting down (abort signal received)');
}
