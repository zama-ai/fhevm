import type { Hex } from "viem";

import { createWalletContext, type ClientOptions } from "../../config";
import { decryptUserValues } from "../../fhevm/user-decrypt";
import type { ProgressReporter } from "../../shared/progress";
import type { FheTestHandle, FheValueType, UserDecryptResult } from "../../types";
import { describeHandle } from "../progress";
import {
  readCachedFheTestHandles,
  reportProvidedHandles,
  resolveCachedDecryptSelection,
} from "../handles";

/**
 * User-decrypt options for existing private handles.
 *
 * `handles` and `types` are mutually exclusive modes. Direct handles are
 * decrypted as-is. Without direct handles, the flow reads FHETest handles stored
 * for the signer wallet and each selected type.
 */
export type UserDecryptOptions = ClientOptions &
  Readonly<{
    types?: readonly FheValueType[];
    contractAddress?: Hex;
    handles?: readonly Hex[];
    privateKey?: Hex;
    mnemonic?: string;
    durationDays: number;
    includeValidationArtifact?: boolean;
    onProgress?: ProgressReporter;
  }>;

/** Decrypts private handles owned by the signing wallet. */
export const userDecrypt = async (
  options: UserDecryptOptions,
): Promise<UserDecryptResult & { handles?: readonly FheTestHandle[] }> => {
  const { handles, types } = resolveCachedDecryptSelection(options);

  options.onProgress?.("Loading wallet and creating clients");
  const { account, contractAddress, publicClient, ...context } =
    createWalletContext(options);

  if (handles.length > 0) {
    reportProvidedHandles(handles, options.onProgress);
    return decryptUserValues(
      { ...context, contractAddress, publicClient },
      {
        encryptedValues: handles,
        signer: account,
        ownerAddress: account.address,
        durationDays: options.durationDays,
        network: options.network,
        includeValidationArtifact: options.includeValidationArtifact,
        onProgress: options.onProgress,
      },
    );
  }

  const storedHandles = await readCachedFheTestHandles(
    { contractAddress, publicClient },
    {
      account: account.address,
      types,
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
