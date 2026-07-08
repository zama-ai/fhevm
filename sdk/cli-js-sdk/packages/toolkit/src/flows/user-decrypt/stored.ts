import type { Hex } from "viem";

import { createWalletContext, type ClientOptions } from "../../config";
import { decryptUserValues } from "../../fhevm/user-decrypt";
import type { ProgressReporter } from "../../shared/progress";
import type { FheTestHandle, FheValueType, UserDecryptResult } from "../../types";
import { readStoredFheTestHandles } from "../handles";
import { describeHandle } from "../progress";

/**
 * Options for decrypting FHETest handles stored in wallet/type slots.
 *
 * The flow reads the FHETest handles stored for the signer wallet and each
 * selected type, defaulting to the bool slot when no `types` are given.
 */
export type StoredUserDecryptOptions = ClientOptions &
  Readonly<{
    types?: readonly FheValueType[];
    contractAddress?: Hex;
    privateKey?: Hex;
    mnemonic?: string;
    durationDays: number;
    includeValidationArtifact?: boolean;
    onProgress?: ProgressReporter;
  }>;

/** Decrypts FHETest handles stored for the signer wallet and selected types. */
export const storedUserDecrypt = async (
  options: StoredUserDecryptOptions,
): Promise<UserDecryptResult & { handles: readonly FheTestHandle[] }> => {
  options.onProgress?.("Loading wallet and creating clients");
  const { account, contractAddress, publicClient, ...context } =
    createWalletContext(options);

  const storedHandles = await readStoredFheTestHandles(
    { contractAddress, publicClient },
    {
      account: account.address,
      types: options.types ?? [],
      describeStoredHandle: (type, handle) =>
        `Using stored ${type} handle: ${describeHandle(handle)}`,
      onProgress: options.onProgress,
    },
  );
  const decrypted = await decryptUserValues(
    { ...context, contractAddress, publicClient },
    {
      encryptedValues: storedHandles.map((handle) => handle.handle),
      signer: account,
      ownerAddress: account.address,
      durationDays: options.durationDays,
      network: options.network,
      includeValidationArtifact: options.includeValidationArtifact,
      onProgress: options.onProgress,
    },
  );

  return {
    ...decrypted,
    handles: storedHandles,
    validationArtifact: decrypted.validationArtifact
      ? {
          ...decrypted.validationArtifact,
          expectedClearValues: storedHandles
            .filter((handle) => handle.clearText !== undefined)
            .map((handle) => ({
              type: handle.type,
              value: handle.clearText as string,
            })),
        }
      : undefined,
  };
};
