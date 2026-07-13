import type { Hex } from "viem";
import type { Account } from "viem/accounts";

import { createWalletContext, type ClientOptions } from "../../config";
import { decryptUserValues } from "../../fhevm/user-decrypt";
import { encryptValues } from "../../fhevm/encryption";
import type { ProgressReporter } from "../../shared/progress";
import { sendAndWait } from "../../shared/transactions";
import { readConfidentialBalance, readTokenMetadata, type TokenMetadata } from "../../token/reads";
import {
  simulateConfidentialTransfer,
  simulateConfidentialTransferFrom,
} from "../../token/writes";

const EUINT64_UPPER_BOUND = 1n << 64n;
const DEFAULT_PERMIT_DURATION_SECONDS = 86_400;

/**
 * Options for an ERC-7984 confidential transfer.
 *
 * `contractAddress` is required; there is no per-network token default. When
 * `from` is set the flow uses `confidentialTransferFrom`, spending an existing
 * operator allowance instead of the loaded wallet's balance. `verify` decrypts
 * the sender's balance before and after the transfer; it is incompatible with
 * `from` because the operator wallet cannot decrypt the `from` account's
 * confidential balance.
 */
export type TransferTokenOptions = ClientOptions &
  Readonly<{
    contractAddress: Hex;
    to: Hex;
    amount: bigint;
    from?: Hex;
    verify?: boolean;
    /** SDK permit lifetime for optional balance verification. */
    permitDurationSeconds?: number;
    privateKey?: Hex;
    mnemonic?: string;
    onProgress?: ProgressReporter;
  }>;

/** Sender balance verification produced when `verify` is set. */
export type TransferVerification = Readonly<{
  balanceBefore: bigint;
  balanceAfter: bigint;
  deltaMatches: boolean;
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
  verification?: TransferVerification;
}>;

type DecryptContext = Readonly<{
  chain: ReturnType<typeof createWalletContext>["chain"];
  contractAddress: Hex;
  publicClient: ReturnType<typeof createWalletContext>["publicClient"];
}>;

/** User-decrypts a single handle paired with the token contract and returns it as a bigint. */
const userDecryptAmount = async (
  context: DecryptContext,
  options: {
    handle: Hex;
    signer: Account;
    durationSeconds: number;
    network: ClientOptions["network"];
    label: string;
    onProgress?: ProgressReporter;
  },
): Promise<bigint> => {
  options.onProgress?.(options.label);
  const result = await decryptUserValues(context, {
    encryptedValues: [options.handle],
    signer: options.signer,
    ownerAddress: options.signer.address,
    durationSeconds: options.durationSeconds,
    network: options.network,
    onProgress: options.onProgress,
  });
  const clear = result.clearValues[0];
  if (!clear) throw new Error("User decryption returned no value.");
  return BigInt(clear.value);
};

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
  if (options.verify && options.from) {
    throw new Error(
      "token transfer --verify cannot be combined with --from: the operator wallet cannot decrypt the --from account's confidential balance.",
    );
  }

  options.onProgress?.("Loading wallet and creating clients");
  const { account, chain, contractAddress, fhevm, publicClient, walletClient } =
    createWalletContext(options);
  const from = options.from ?? account.address;
  const permitDurationSeconds =
    options.permitDurationSeconds ?? DEFAULT_PERMIT_DURATION_SECONDS;
  const decryptContext: DecryptContext = { chain, contractAddress, publicClient };

  let balanceBefore: bigint | undefined;
  if (options.verify) {
    options.onProgress?.("Reading sender confidential balance before transfer");
    const balanceHandleBefore = await readConfidentialBalance(
      { publicClient, contractAddress },
      from,
    );
    options.onProgress?.(`Sender balance handle (before): ${balanceHandleBefore}`);
    balanceBefore = await userDecryptAmount(decryptContext, {
      handle: balanceHandleBefore,
      signer: account,
      durationSeconds: permitDurationSeconds,
      network: options.network,
      label: "Decrypting sender balance before transfer",
      onProgress: options.onProgress,
    });
    options.onProgress?.(`Sender balance before transfer: ${balanceBefore.toString()}`);
  }

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

  let verification: TransferVerification | undefined;
  if (options.verify) {
    if (balanceBefore === undefined) {
      throw new Error("Missing sender balance before transfer.");
    }
    options.onProgress?.("Reading sender confidential balance after transfer");
    const balanceHandleAfter = await readConfidentialBalance(
      { publicClient, contractAddress },
      from,
    );
    options.onProgress?.(`Sender balance handle (after): ${balanceHandleAfter}`);
    const balanceAfter = await userDecryptAmount(decryptContext, {
      handle: balanceHandleAfter,
      signer: account,
      durationSeconds: permitDurationSeconds,
      network: options.network,
      label: "Decrypting sender balance after transfer",
      onProgress: options.onProgress,
    });
    options.onProgress?.(`Sender balance after transfer: ${balanceAfter.toString()}`);
    const deltaMatches = balanceBefore - balanceAfter === options.amount;
    options.onProgress?.(
      deltaMatches
        ? "Balance delta matches the requested transfer amount"
        : "Balance delta does NOT match the requested transfer amount",
    );
    verification = {
      balanceBefore,
      balanceAfter,
      deltaMatches,
    };
  }

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
    verification,
  };
};
