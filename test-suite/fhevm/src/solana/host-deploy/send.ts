// HTTP-only send/confirm for the zama-host deployer. Preview-env Solana RPC is a single URL
// (Helius / public **devnet**); there is no Yellowstone and no guarantee of a WS endpoint, so this
// must not use `sendAndConfirmTransactionFactory` (that factory requires rpcSubscriptions).

import {
  appendTransactionMessageInstructions,
  assertIsTransactionWithBlockhashLifetime,
  createSolanaRpc,
  createTransactionMessage,
  getBase64EncodedWireTransaction,
  getSignatureFromTransaction,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  type Instruction,
  type Rpc,
  type SolanaRpcApi,
  type TransactionSigner,
} from "@solana/kit";

const CONFIRM_TIMEOUT_MS = 60_000;
const CONFIRM_INTERVAL_MS = 400;

export type HostDeployContext = {
  readonly rpc: Rpc<SolanaRpcApi>;
  sendTransaction(payer: TransactionSigner, instructions: readonly Instruction[]): Promise<void>;
};

const sleep = (ms: number): Promise<void> => new Promise((resolve) => setTimeout(resolve, ms));

export const createHostDeployContext = (rpcUrl: string): HostDeployContext => {
  const rpc = createSolanaRpc(rpcUrl);
  return {
    rpc,
    async sendTransaction(payer, instructions) {
      const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
      const base = setTransactionMessageFeePayerSigner(payer, createTransactionMessage({ version: 0 }));
      const withLifetime = setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, base);
      const message = appendTransactionMessageInstructions([...instructions], withLifetime);
      const signedTransaction = await signTransactionMessageWithSigners(message);
      assertIsTransactionWithBlockhashLifetime(signedTransaction);
      const signature = getSignatureFromTransaction(signedTransaction);
      await rpc
        .sendTransaction(getBase64EncodedWireTransaction(signedTransaction), { encoding: "base64" })
        .send();
      const deadline = Date.now() + CONFIRM_TIMEOUT_MS;
      for (;;) {
        const { value } = await rpc.getSignatureStatuses([signature]).send();
        const status = value[0];
        if (status?.err) {
          throw new Error(`transaction ${signature} failed: ${JSON.stringify(status.err)}`);
        }
        const level = status?.confirmationStatus;
        if (level === "confirmed" || level === "finalized") return;
        if (Date.now() >= deadline) {
          throw new Error(`transaction ${signature} did not confirm within ${CONFIRM_TIMEOUT_MS}ms`);
        }
        await sleep(CONFIRM_INTERVAL_MS);
      }
    },
  };
};
