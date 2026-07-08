import type { Hex } from "viem";

import { createWalletContext, type ClientOptions } from "../../config";
import { encryptValues } from "../../fhevm/encryption";
import type { ProgressReporter } from "../../shared/progress";
import { sendAndWait } from "../../shared/transactions";
import { readTokenMetadata, type TokenMetadata } from "../../token/reads";
import {
  simulateConfidentialTransfer,
  simulateConfidentialTransferFrom,
} from "../../token/writes";

const EUINT64_UPPER_BOUND = 1n << 64n;

/**
 * Options for an ERC-7984 confidential transfer.
 *
 * `contractAddress` is required; there is no per-network token default. When
 * `from` is set the flow uses `confidentialTransferFrom`, spending an existing
 * operator allowance instead of the loaded wallet's balance.
 */
export type TransferTokenOptions = ClientOptions &
  Readonly<{
    contractAddress: Hex;
    to: Hex;
    amount: bigint;
    from?: Hex;
    privateKey?: Hex;
    mnemonic?: string;
    onProgress?: ProgressReporter;
  }>;

/** Result of a confidential transfer, including the transferred-amount handle. */
export type TransferTokenResult = Readonly<{
  network: ClientOptions["network"];
  contractAddress: Hex;
  from: Hex;
  to: Hex;
  amount: bigint;
  encryptedAmountHandle: Hex;
  transferredHandle: Hex;
  transactionHash: Hex;
  tokenMetadata: TokenMetadata;
}>;

/** Encrypts an amount and runs an ERC-7984 confidential transfer. */
export const transferToken = async (
  options: TransferTokenOptions,
): Promise<TransferTokenResult> => {
  if (!options.contractAddress) {
    throw new Error("token transfer requires --contract-address.");
  }
  if (options.amount <= 0n || options.amount >= EUINT64_UPPER_BOUND) {
    throw new Error(
      `token transfer amount must be greater than 0 and less than 2^64, received ${options.amount.toString()}.`,
    );
  }

  options.onProgress?.("Loading wallet and creating clients");
  const { account, contractAddress, fhevm, publicClient, walletClient } =
    createWalletContext(options);
  const from = options.from ?? account.address;

  const encrypted = await encryptValues(fhevm, {
    contractAddress,
    userAddress: account.address,
    values: [{ type: "uint64", value: options.amount }],
    onProgress: options.onProgress,
    progressLabel: "Encrypting transfer amount",
  });
  const encryptedAmountHandle = encrypted.encryptedValues[0];
  if (!encryptedAmountHandle) {
    throw new Error("FHEVM SDK did not return a handle.");
  }
  options.onProgress?.(`Encrypted transfer amount handle: ${encryptedAmountHandle}`);

  const functionName = options.from
    ? "confidentialTransferFrom"
    : "confidentialTransfer";
  options.onProgress?.(`Simulating ConfidentialFungibleToken.${functionName}`);
  const simulation = options.from
    ? await simulateConfidentialTransferFrom(
        { account, contractAddress, publicClient },
        {
          from: options.from,
          to: options.to,
          encryptedAmount: encryptedAmountHandle,
          inputProof: encrypted.inputProof,
        },
      )
    : await simulateConfidentialTransfer(
        { account, contractAddress, publicClient },
        {
          to: options.to,
          encryptedAmount: encryptedAmountHandle,
          inputProof: encrypted.inputProof,
        },
      );
  options.onProgress?.(`Transferred amount handle: ${simulation.transferredHandle}`);

  const transactionHash = await sendAndWait({
    walletClient,
    publicClient,
    request: simulation.request,
    onProgress: options.onProgress,
  });

  options.onProgress?.("Reading token metadata");
  const tokenMetadata = await readTokenMetadata({ publicClient, contractAddress });

  return {
    network: options.network,
    contractAddress,
    from,
    to: options.to,
    amount: options.amount,
    encryptedAmountHandle,
    transferredHandle: simulation.transferredHandle,
    transactionHash,
    tokenMetadata,
  };
};
