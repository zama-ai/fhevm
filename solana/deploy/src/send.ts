// The deployer only needs HTTP RPC; Yellowstone is configured separately for the listener.
// Poll confirmation so deployment does not also require a WebSocket subscription endpoint.
import {
  createClient,
  getSignatureFromTransaction,
  type Instruction,
  type Signature,
  type Rpc,
  type SolanaRpcApi,
  type TransactionSigner,
} from '@solana/kit';
import { rpcConnection, rpcTransactionPlanner, rpcTransactionPlanSigningExecutor } from '@solana/kit-plugin-rpc';
import { payer as feePayer } from '@solana/kit-plugin-signer';
import { createFinalizedRpc } from '@fhevm/solana-zama-host';

const CONFIRM_TIMEOUT_MS = 60_000;
const CONFIRM_INTERVAL_MS = 400;

export type HostDeployContext = {
  readonly rpc: Rpc<SolanaRpcApi>;
  beforeSubmit?: (signature: Signature, lastValidBlockHeight: bigint) => Promise<void>;
  sendTransaction(payer: TransactionSigner, instructions: readonly Instruction[]): Promise<void>;
};

const sleep = (ms: number): Promise<void> => new Promise((resolve) => setTimeout(resolve, ms));

export const createHostDeployContext = (rpcUrl: string, signal?: AbortSignal): HostDeployContext => {
  const rpc = createFinalizedRpc(rpcUrl);
  const context: HostDeployContext = {
    rpc,
    async sendTransaction(payer, instructions) {
      signal?.throwIfAborted();
      // Kit's stock planner and signer: a version 1 transaction whose compute and loaded-data limits
      // come from a simulation.
      const { context: signed } = await createClient()
        .use(feePayer(payer))
        .use(rpcConnection<SolanaRpcApi>(rpc))
        .use(rpcTransactionPlanner({ version: 1 }))
        .use(rpcTransactionPlanSigningExecutor())
        .signTransaction([...instructions], { abortSignal: signal });
      const signature = getSignatureFromTransaction(signed.transaction);
      signal?.throwIfAborted();
      await context.beforeSubmit?.(signature, signed.message.lifetimeConstraint.lastValidBlockHeight);
      // The signing step already simulated the transaction.
      await rpc.sendTransaction(signed.transactionBase64, { encoding: 'base64', skipPreflight: true }).send();
      const deadline = Date.now() + CONFIRM_TIMEOUT_MS;
      for (;;) {
        const { value } = await rpc.getSignatureStatuses([signature]).send();
        const status = value[0];
        if (status?.err) {
          throw new Error(`transaction ${signature} failed: ${JSON.stringify(status.err)}`);
        }
        const level = status?.confirmationStatus;
        if (level === 'finalized') {
          return;
        }
        if (Date.now() >= deadline) {
          throw new Error(`transaction ${signature} did not confirm within ${CONFIRM_TIMEOUT_MS}ms`);
        }
        await sleep(CONFIRM_INTERVAL_MS);
      }
    },
  };
  return context;
};
