import {
  assertIsSendableTransaction,
  assertIsTransactionWithBlockhashLifetime,
  createTransactionMessage,
  createTransactionPlanner,
  extendClient,
  fillTransactionMessageProvisoryResourceLimits,
  getSignatureFromTransaction,
  pipe,
  sendAndConfirmTransactionFactory,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLoadedAccountsDataSizeLimit,
  type ClientWithPayer,
  type ClientWithRpc,
  type ClientWithRpcSubscriptions,
  type ClientWithTransactionSending,
  type Signature,
  type SolanaRpcApi,
  type SolanaRpcSubscriptionsApi,
  type Transaction,
} from '@solana/kit';
import { transactionPlanner } from '@solana/kit-plugin-instruction-plan';
import { rpcTransactionPlanSigningExecutor, type RpcSignContext } from '@solana/kit-plugin-rpc';

/** Agave's maximum, which is also the limit a version 0 transaction gets when it sets none. */
export const LOADED_ACCOUNTS_DATA_SIZE_LIMIT = 64 * 1024 * 1024;

/** A sent transaction's context: the signing context, with the fee payer's signature always present. */
export type FinalizedSendContext = RpcSignContext & { readonly signature: Signature };

/**
 * Plans every transaction as version 1 and signs it with kit-plugin-rpc's signing executor, which sets
 * the compute limit from a simulation. Install `payer(...)` and `rpcConnection(...)` first: the planner
 * reads `client.payer`, and the blockhash and simulation take the RPC's default commitment.
 *
 * The loaded-data limit is set up front, so the executor keeps it. Left to the executor, it would be
 * the exact simulated size, and a shared store that grows before the transaction lands would make it
 * fail on chain, with the fee charged. The limit does not change the fee.
 *
 * The executor signs partially, for a wallet that adds its signature later. Callers journal the
 * signature before sending, so `signTransaction` rejects a transaction a signer left unsigned.
 */
export function v1TransactionSigning() {
  return <T extends ClientWithPayer & ClientWithRpc<SolanaRpcApi>>(client: T) => {
    const signing = pipe(
      client,
      transactionPlanner(
        createTransactionPlanner({
          createTransactionMessage: () =>
            pipe(
              createTransactionMessage({ version: 1 }),
              (message) => setTransactionMessageFeePayerSigner(client.payer, message),
              (message) => setTransactionMessageLoadedAccountsDataSizeLimit(LOADED_ACCOUNTS_DATA_SIZE_LIMIT, message),
              fillTransactionMessageProvisoryResourceLimits,
            ),
        }),
      ),
      rpcTransactionPlanSigningExecutor(),
    );
    return extendClient(signing, {
      signTransaction: (async (input, config) => {
        const result = await signing.signTransaction(input, config);
        assertIsSendableTransaction(result.context.transaction);
        return result;
      }) as typeof signing.signTransaction,
    });
  };
}

/**
 * Signs through `v1TransactionSigning` and waits for each send at finalized (DD-070).
 * kit-plugin-rpc's own sending executor always waits at confirmed. Take the RPC from
 * `createFinalizedRpc`.
 */
export function finalizedTransactionSending() {
  return <
    T extends ClientWithPayer &
      ClientWithRpc<SolanaRpcApi> &
      ClientWithRpcSubscriptions<SolanaRpcSubscriptionsApi>,
  >(
    client: T,
  ) => {
    const signing = v1TransactionSigning()(client);
    const sendAndConfirm = sendAndConfirmTransactionFactory({
      rpc: client.rpc,
      rpcSubscriptions: client.rpcSubscriptions,
    });
    // The signing step already simulated the transaction to estimate its limits.
    const sendSignedTransaction = async (transaction: Transaction, abortSignal?: AbortSignal): Promise<void> => {
      assertIsSendableTransaction(transaction);
      assertIsTransactionWithBlockhashLifetime(transaction);
      await sendAndConfirm(transaction, { abortSignal, commitment: 'finalized', skipPreflight: true });
    };
    const sendTransaction: ClientWithTransactionSending<FinalizedSendContext>['sendTransaction'] = async (
      input,
      config,
    ) => {
      const result = await signing.signTransaction(input, config);
      await sendSignedTransaction(result.context.transaction, config?.abortSignal);
      return { ...result, context: { ...result.context, signature: getSignatureFromTransaction(result.context.transaction) } };
    };
    return extendClient(signing, { sendSignedTransaction, sendTransaction });
  };
}
