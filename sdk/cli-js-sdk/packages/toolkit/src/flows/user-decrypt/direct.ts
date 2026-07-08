import type { Hex } from "viem";

import { createWalletContext, type ClientOptions } from "../../config";
import { decryptUserValues } from "../../fhevm/user-decrypt";
import type { ProgressReporter } from "../../shared/progress";
import type { UserDecryptResult } from "../../types";
import { reportProvidedHandles } from "../handles";

/**
 * User-decrypt options for existing private handles.
 *
 * The supplied `handles` are decrypted as-is. `contractAddress` is the ACL
 * pairing contract for the handles and defaults to the FHETest contract.
 */
export type UserDecryptOptions = ClientOptions &
  Readonly<{
    handles: readonly Hex[];
    contractAddress?: Hex;
    privateKey?: Hex;
    mnemonic?: string;
    durationDays: number;
    includeValidationArtifact?: boolean;
    onProgress?: ProgressReporter;
  }>;

/** Decrypts private handles owned by the signing wallet. */
export const userDecrypt = async (
  options: UserDecryptOptions,
): Promise<UserDecryptResult> => {
  options.onProgress?.("Loading wallet and creating clients");
  const { account, contractAddress, publicClient, ...context } =
    createWalletContext(options);

  reportProvidedHandles(options.handles, options.onProgress);
  return decryptUserValues(
    { ...context, contractAddress, publicClient },
    {
      encryptedValues: options.handles,
      signer: account,
      ownerAddress: account.address,
      durationDays: options.durationDays,
      network: options.network,
      includeValidationArtifact: options.includeValidationArtifact,
      onProgress: options.onProgress,
    },
  );
};
