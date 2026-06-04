import type { Hex } from "viem";

import { createWalletContext, type ClientOptions } from "../../config";
import { readFheTestHandle } from "../../fhe-test/handles";
import { simulateMakePubliclyDecryptable } from "../../fhe-test/writes";
import { readPublicValues } from "../../fhevm/public-decrypt";
import type { ProgressReporter } from "../../shared/progress";
import { sendAndWait } from "../../shared/transactions";
import type {
  FheTestHandle,
  FheValueType,
  PublicDecryptResult,
} from "../../types";
import { describeHandle } from "../progress";

/**
 * Options for converting an existing wallet-owned FHETest handle to public
 * decryptability.
 */
export type MakePublicOptions = ClientOptions &
  Readonly<{
    type: FheValueType;
    contractAddress?: Hex;
    privateKey?: Hex;
    mnemonic?: string;
    onProgress?: ProgressReporter;
  }>;

/**
 * Marks the wallet's stored FHETest handle as publicly decryptable, then
 * verifies it through the public decrypt API.
 */
export const makePublicAndDecrypt = async (
  options: MakePublicOptions,
): Promise<
  PublicDecryptResult & {
    transactionHash: Hex;
    handle: FheTestHandle;
  }
> => {
  options.onProgress?.("Loading wallet and creating clients");
  const { account, contractAddress, fhevm, publicClient, walletClient } =
    createWalletContext(options);
  const previousHandle = await readFheTestHandle({
    publicClient,
    contractAddress,
    account: account.address,
    type: options.type,
    onProgress: options.onProgress,
  });
  options.onProgress?.(
    `Existing ${options.type} handle: ${describeHandle(previousHandle)}`,
  );

  options.onProgress?.(
    `Simulating FHETest.makePubliclyDecryptable for ${options.type}`,
  );
  const request = await simulateMakePubliclyDecryptable(
    { account, contractAddress, publicClient },
    options.type,
  );

  const transactionHash = await sendAndWait({
    walletClient,
    publicClient,
    request,
    onProgress: options.onProgress,
  });
  options.onProgress?.(
    `Publicly decryptable ${options.type} handle: ${describeHandle(previousHandle)}`,
  );
  const decrypted = await readPublicValues(
    fhevm,
    [previousHandle.handle],
    options.onProgress,
  );

  return { ...decrypted, transactionHash, handle: previousHandle };
};
