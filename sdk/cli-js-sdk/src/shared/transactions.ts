import type { Hex } from "viem";

import type { createClients, createWallet } from "../config";
import type { ProgressReporter } from "./progress";

type PublicClient = ReturnType<typeof createClients>["publicClient"];
type WalletClient = ReturnType<typeof createWallet>["walletClient"];

export const sendAndWait = async (options: {
  walletClient: WalletClient;
  publicClient: PublicClient;
  request: unknown;
  onProgress?: ProgressReporter;
}): Promise<Hex> => {
  options.onProgress?.("Sending transaction");
  const transactionHash = await options.walletClient.writeContract(
    options.request as never,
  );
  options.onProgress?.(`Waiting for transaction receipt: ${transactionHash}`);
  const receipt = await options.publicClient.waitForTransactionReceipt({
    hash: transactionHash,
  });
  if (receipt.status !== "success") {
    throw new Error(`Transaction reverted: ${transactionHash}`);
  }
  return transactionHash;
};
