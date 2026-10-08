/** Public API surface: the demo's tests, which drive the demo client over a scripted RPC. */
import {
  getCompiledTransactionMessageDecoder,
  getTransactionEncoder,
  type Blockhash,
  type Transaction,
  type TransactionSigner,
} from '@solana/kit';
import { vi } from 'vitest';

import { createDemoClient } from './demoClient';

export const TEST_BLOCKHASH = 'EkSnNWid2cvwEVnVx9aBqawnmiCNiDgp3gUdkDPTKN1N' as Blockhash;

type Simulation = { readonly err: unknown; readonly logs?: readonly string[] };

/**
 * The demo client over a scripted JSON-RPC that answers a blockhash and a simulation per
 * transaction. Planning, estimation and signing run for real. The test file mocks Kit's
 * `sendAndConfirmTransactionFactory` to capture each send. Restore with `vi.restoreAllMocks()`.
 */
export function testDemoClient(feePayer: TransactionSigner) {
  const simulations: Simulation[] = [];
  const rpcMethods: string[] = [];
  vi.spyOn(globalThis, 'fetch').mockImplementation(async (_input: unknown, init?: RequestInit) => {
    const { id, method } = JSON.parse(String(init?.body)) as { id: number; method: string };
    rpcMethods.push(method);
    const context = { slot: 1 };
    if (method === 'getLatestBlockhash') {
      return Response.json({ jsonrpc: '2.0', id, result: { context, value: { blockhash: TEST_BLOCKHASH, lastValidBlockHeight: 1_000 } } });
    }
    if (method === 'simulateTransaction') {
      const { err, logs = [] } = simulations.shift() ?? { err: null };
      const value = { err, logs, accounts: null, returnData: null, unitsConsumed: 200_000, loadedAccountsDataSize: 100_000 };
      return Response.json({ jsonrpc: '2.0', id, result: { context, value } });
    }
    throw new Error(`unexpected RPC ${method}`);
  });
  return {
    client: createDemoClient({ rpcUrl: 'http://rpc.test', wsUrl: 'ws://rpc.test' }, feePayer),
    rpcMethods,
    /** The next simulation (Kit's resource estimate) fails with `err`. */
    failNextSimulation: (simulation: Simulation) => void simulations.push(simulation),
  };
}

/** The version, wire size and account-key count of a signed transaction, as Kit encodes it. */
export function encodedSize(transaction: Transaction) {
  const compiled = getCompiledTransactionMessageDecoder().decode(transaction.messageBytes);
  return {
    version: compiled.version,
    bytes: getTransactionEncoder().encode(transaction).length,
    addresses: compiled.staticAccounts.length,
  };
}
