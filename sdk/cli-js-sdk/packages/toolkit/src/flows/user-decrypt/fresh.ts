import type { Hex } from "viem";

import { createWalletContext, type ClientOptions } from "../../config";
import { decryptUserValues } from "../../fhevm/user-decrypt";
import type { ProgressReporter } from "../../shared/progress";
import type {
  EncryptValue,
  FheClearValue,
  FheTestHandle,
  FheValueType,
  UserDecryptResult,
} from "../../types";
import { createStoredFheTestHandle } from "../handles";
import { describeHandle } from "../progress";

/**
 * Options for the fresh user-decrypt demo flow.
 *
 * The flow encrypts one value for the wallet, stores it in FHETest with
 * `makePublic=false`, then signs a user decryption permit for the stored handle.
 */
export type FreshUserDecryptOptions = ClientOptions &
  Readonly<{
    type: FheValueType;
    contractAddress?: Hex;
    value?: FheClearValue;
    privateKey?: Hex;
    mnemonic?: string;
    durationDays: number;
    includeValidationArtifact?: boolean;
    onProgress?: ProgressReporter;
  }>;

/** Creates a new private FHETest handle and decrypts it as its owner. */
export const freshUserDecrypt = async (
  options: FreshUserDecryptOptions,
): Promise<
  UserDecryptResult & {
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
      makePublic: false,
      inputProgressLabel: "Input value",
      encryptProgressLabel: `Encrypting ${options.type} value`,
      encryptedProgressLabel: "Encrypted input handle",
      storedProgressLabel: (storedHandle) =>
        `Stored ${options.type} handle: ${describeHandle(storedHandle)}`,
      onProgress: options.onProgress,
    });
  const decrypted = await decryptUserValues(
    context,
    {
      encryptedValues: [handle.handle],
      signer: context.account,
      ownerAddress: context.account.address,
      durationDays: options.durationDays,
      network: options.network,
      includeValidationArtifact: options.includeValidationArtifact,
      onProgress: options.onProgress,
    },
  );

  return {
    ...decrypted,
    transactionHash,
    inputProof,
    inputValues,
    handle,
    validationArtifact: decrypted.validationArtifact
      ? {
          ...decrypted.validationArtifact,
          expectedClearValues: inputValues.map((value) => ({
            type: value.type,
            value: String(value.value),
          })),
        }
      : undefined,
  };
};
