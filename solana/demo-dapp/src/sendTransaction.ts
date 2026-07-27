import {
  appendTransactionMessageInstructions,
  assertIsFullySignedTransaction,
  assertIsTransactionWithBlockhashLifetime,
  assertIsTransactionWithinSizeLimit,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  createTransactionMessage,
  sendAndConfirmTransactionFactory,
  setTransactionMessageComputeUnitLimit,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  type Instruction,
  type TransactionSigner,
} from "@solana/kit";

import type { DemoConfig } from "./demoSession";

export const sendTransaction = async (
  config: DemoConfig,
  payer: TransactionSigner,
  instructions: readonly Instruction[],
  computeUnitLimit: number,
): Promise<void> => {
  const rpc = createSolanaRpc(config.rpcUrl);
  const rpcSubscriptions = createSolanaRpcSubscriptions(config.wsUrl);
  const sendAndConfirm = sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions });
  const { value: latestBlockhash } = await rpc.getLatestBlockhash({ commitment: "confirmed" }).send();
  const base = setTransactionMessageFeePayerSigner(payer, createTransactionMessage({ version: 0 }));
  const withLifetime = setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, base);
  const withComputeLimit = setTransactionMessageComputeUnitLimit(computeUnitLimit, withLifetime);
  const message = appendTransactionMessageInstructions(instructions, withComputeLimit);
  const transaction = await signTransactionMessageWithSigners(message);
  assertIsFullySignedTransaction(transaction);
  assertIsTransactionWithBlockhashLifetime(transaction);
  assertIsTransactionWithinSizeLimit(transaction);
  await sendAndConfirm(transaction, { commitment: "confirmed" });
};
