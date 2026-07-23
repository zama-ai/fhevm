import type { Hex, WriteContractParameters } from "viem";

import type { createClients, createWallet } from "../config";
import type { ProgressReporter } from "./progress";

type PublicClient = ReturnType<typeof createClients>["publicClient"];
type WalletClient = ReturnType<typeof createWallet>["walletClient"];

export type ContractWriteRequest = WriteContractParameters;

export const sendAndWait = async (options: {
  walletClient: WalletClient;
  publicClient: PublicClient;
  request: ContractWriteRequest;
  onProgress?: ProgressReporter;
}): Promise<Hex> => {
  options.onProgress?.("Sending transaction");
  const transactionHash = await options.walletClient.writeContract(options.request);
  options.onProgress?.(`Waiting for transaction receipt: ${transactionHash}`);
  const receipt = await options.publicClient.waitForTransactionReceipt({
    hash: transactionHash,
  });
  if (receipt.status !== "success") {
    throw new Error(`Transaction reverted: ${transactionHash}`);
  }
  return transactionHash;
};
