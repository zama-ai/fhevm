import type { Hex } from "viem";

import { createWalletContext, type ClientOptions } from "../../config";
import { readPublicValues } from "../../fhevm/public-decrypt";
import type { ProgressReporter } from "../../shared/progress";
import type {
  EncryptValue,
  FheClearValue,
  FheTestHandle,
  FheValueType,
  PublicDecryptResult,
} from "../../types";
import { createStoredFheTestHandle } from "../handles";
import { describeHandle } from "../progress";

/**
 * Options for the fresh public-decrypt demo flow.
 *
 * The flow encrypts one value for the wallet, writes it to FHETest with
 * `makePublic=true`, then public-decrypts the stored handle.
 */
export type FreshPublicDecryptOptions = ClientOptions &
  Readonly<{
    type: FheValueType;
    contractAddress?: Hex;
    value?: FheClearValue;
    privateKey?: Hex;
    mnemonic?: string;
    onProgress?: ProgressReporter;
  }>;

/** Creates a new public FHETest handle and decrypts it through the public API. */
export const freshPublicDecrypt = async (
  options: FreshPublicDecryptOptions,
): Promise<
  PublicDecryptResult & {
    transactionHash: Hex;
    inputProof: Hex;
    inputValues: readonly EncryptValue[];
    handle: FheTestHandle;
  }
> => {
  options.onProgress?.("Loading wallet and creating clients");
  const context = createWalletContext(options);
  const { transactionHash, inputProof, inputValues, handle } =
    await createStoredFheTestHandle(context, {
      type: options.type,
      ownerAddress: context.account.address,
      value: options.value,
      makePublic: true,
      inputProgressLabel: "Input value",
      encryptProgressLabel: `Encrypting ${options.type} value`,
      encryptedProgressLabel: "Encrypted input handle",
      storedProgressLabel: (storedHandle) =>
        `Stored public ${options.type} handle: ${describeHandle(storedHandle)}`,
      onProgress: options.onProgress,
    });
  const decrypted = await readPublicValues(
    context.fhevm,
    [handle.handle],
    options.onProgress,
  );

  return {
    ...decrypted,
    transactionHash,
    inputProof,
    inputValues,
    handle,
  };
};
