import {
  assertIsSendableTransaction,
  assertIsTransactionWithBlockhashLifetime,
  extendClient,
  pipe,
  sendAndConfirmTransactionFactory,
  type ClientWithPayer,
  type ClientWithRpc,
  type ClientWithRpcSubscriptions,
  type ClientWithTransactionSending,
  type SolanaRpcApi,
  type SolanaRpcSubscriptionsApi,
  type Transaction,
} from '@solana/kit';
import {
  rpcTransactionPlanner,
  rpcTransactionPlanSigningExecutor,
  type RpcSignContext,
} from '@solana/kit-plugin-rpc';

/**
 * Plans every transaction as version 1 with Kit's stock planner, signs it with the stock signing
 * executor, which sets the compute and loaded-data limits from a simulation, and waits for each send
 * at finalized (DD-070). kit-plugin-rpc's own sending executor always waits at confirmed.
 *
 * Install `payer(...)` and the connections first, the RPC from `createFinalizedRpc`: the planner
 * reads `client.payer`, and the blockhash and simulation take the RPC's default commitment.
 */
export function finalizedTransactionSending() {
  return <
    T extends ClientWithPayer &
      ClientWithRpc<SolanaRpcApi> &
      ClientWithRpcSubscriptions<SolanaRpcSubscriptionsApi>,
  >(
    client: T,
  ) => {
    const signing = pipe(client, rpcTransactionPlanner({ version: 1 }), rpcTransactionPlanSigningExecutor());
    const sendAndConfirm = sendAndConfirmTransactionFactory({
      rpc: client.rpc,
      rpcSubscriptions: client.rpcSubscriptions,
    });
    // The signing step already simulated the transaction to estimate its limits.
    const sendSignedTransaction = async (transaction: Transaction): Promise<void> => {
      assertIsSendableTransaction(transaction);
      assertIsTransactionWithBlockhashLifetime(transaction);
      await sendAndConfirm(transaction, { commitment: 'finalized', skipPreflight: true });
    };
    const sendTransaction: ClientWithTransactionSending<RpcSignContext>['sendTransaction'] = async (
      input,
      config,
    ) => {
      const result = await signing.signTransaction(input, config);
      await sendSignedTransaction(result.context.transaction);
      return result;
    };
    return extendClient(signing, { sendSignedTransaction, sendTransaction });
  };
}
