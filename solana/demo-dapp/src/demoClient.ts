import {
  createClient,
  createSolanaRpcSubscriptions,
  type SolanaRpcApi,
  type SolanaRpcSubscriptionsApi,
  type TransactionSigner,
} from '@solana/kit';
import { rpcConnection, rpcSubscriptionsConnection } from '@solana/kit-plugin-rpc';
import { payer } from '@solana/kit-plugin-signer';
import { transientStoreTransactions } from '@fhevm/sdk/solana';
import { createFinalizedRpc } from '@fhevm/solana-zama-host';
import { finalizedTransactionSending } from '@fhevm/solana-zama-host/client';

import type { DemoConfig } from './demoConfig';

/** The Kit client that signs and sends every demo transaction for `feePayer`, as version 1 at finalized. */
export const createDemoClient = (config: Pick<DemoConfig, 'rpcUrl' | 'wsUrl'>, feePayer: TransactionSigner) =>
  createClient()
    .use(payer(feePayer))
    .use(rpcConnection<SolanaRpcApi>(createFinalizedRpc(config.rpcUrl)))
    .use(rpcSubscriptionsConnection<SolanaRpcSubscriptionsApi>(createSolanaRpcSubscriptions(config.wsUrl)))
    .use(finalizedTransactionSending())
    .use(transientStoreTransactions());

export type DemoClient = ReturnType<typeof createDemoClient>;
