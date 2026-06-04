/**
 * Service orchestrator. Spawns one chain-worker per configured chain, shares
 * a single SQLite connection (sqlite3 serialises writes internally so this
 * is safe), and wires up graceful shutdown so SIGINT/SIGTERM lets the current
 * poll cycle finish before tearing down.
 *
 * Run via `pnpm mock:daemon` (see package.json) or directly with
 * `npx ts-node scripts/mock-coprocessor/index.ts`.
 */
import { runChainWorker } from './chain-worker';
import { RUNTIME, loadChainConfigs } from './config';
import { MockDb } from './db';

export async function runService(): Promise {
  const chains = loadChainConfigs();
  console.log(
    `[mock-coprocessor] starting service for ${chains.length} chain(s): ${chains
      .map((c) => `${c.name}(eid=${c.lzEid})`)
      .join(', ')}`
  );
  console.log(`[mock-coprocessor] db=${RUNTIME.dbPath}`);

  const db = await MockDb.open(RUNTIME.dbPath);
  const abortController = new AbortController();

  const shutdown = (signal: string) => {
    console.log(`[mock-coprocessor] ${signal} received — draining`);
    abortController.abort();
  };
  process.once('SIGINT', () => shutdown('SIGINT'));
  process.once('SIGTERM', () => shutdown('SIGTERM'));

  const workers = chains.map((chain) => runChainWorker(chain, db, abortController.signal));
  await Promise.all(workers).catch((err) => {
    console.error('[mock-coprocessor] worker crashed irrecoverably:', err);
    process.exitCode = 1;
  });

  await db.close();
  console.log('[mock-coprocessor] db closed, exiting');
}
