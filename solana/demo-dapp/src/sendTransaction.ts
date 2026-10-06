import {
  appendTransactionMessageInstructions,
  assertIsFullySignedTransaction,
  assertIsTransactionWithBlockhashLifetime,
  assertIsTransactionWithinSizeLimit,
  compileTransaction,
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

import { createFinalizedRpc } from '@fhevm/solana-zama-host';
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
  const rpc = createFinalizedRpc(config.rpcUrl);
  const rpcSubscriptions = createSolanaRpcSubscriptions(config.wsUrl);
  const sendAndConfirm = sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions });
  const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
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
  await sendAndConfirm(transaction, { commitment: "finalized", skipPreflight: true });
  return getSignatureFromTransaction(transaction);
};
