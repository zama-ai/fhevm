import {
  appendTransactionMessageInstructions,
  assertIsFullySignedTransaction,
  assertIsTransactionWithBlockhashLifetime,
  assertIsTransactionWithinSizeLimit,
  compileTransaction,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  createTransactionMessage,
  getSignatureFromTransaction,
  sendAndConfirmTransactionFactory,
  setTransactionMessageComputeUnitLimit,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  type Instruction,
  type Signature,
  type TransactionSigner,
} from "@solana/kit";

import type { DemoConfig } from "./demoConfig";
import {
  simulateSignedTransactionLocally,
  simulateUnsignedTransactionLocally,
} from "./transactionSimulation";

export const sendTransaction = async (
  config: DemoConfig,
  payer: TransactionSigner,
  instructions: readonly Instruction[],
  computeUnitLimit: number,
): Promise<Signature> => {
  const rpc = createSolanaRpc(config.rpcUrl);
  const rpcSubscriptions = createSolanaRpcSubscriptions(config.wsUrl);
  const sendAndConfirm = sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions });
  const { value: latestBlockhash } = await rpc.getLatestBlockhash({ commitment: "confirmed" }).send();
  const base = setTransactionMessageFeePayerSigner(payer, createTransactionMessage({ version: 0 }));
  const withLifetime = setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, base);
  const withComputeLimit = setTransactionMessageComputeUnitLimit(computeUnitLimit, withLifetime);
  const message = appendTransactionMessageInstructions(instructions, withComputeLimit);
  await simulateUnsignedTransactionLocally(rpc, compileTransaction(message), "Transaction");
  const transaction = await signTransactionMessageWithSigners(message);
  assertIsFullySignedTransaction(transaction);
  assertIsTransactionWithBlockhashLifetime(transaction);
  assertIsTransactionWithinSizeLimit(transaction);
  await simulateSignedTransactionLocally(rpc, transaction, "Signed transaction");
  await sendAndConfirm(transaction, { commitment: "confirmed", skipPreflight: true });
  return getSignatureFromTransaction(transaction);
};
