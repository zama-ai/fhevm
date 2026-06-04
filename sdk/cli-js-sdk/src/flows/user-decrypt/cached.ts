import type { Hex } from "viem";

import { createWalletContext, type ClientOptions } from "../../config";
import { readFheTestHandle } from "../../fhe-test/handles";
import { decryptUserValues } from "../../fhevm/user-decrypt";
import type { ProgressReporter } from "../../shared/progress";
import type { FheTestHandle, FheValueType, UserDecryptResult } from "../../types";

/**
 * User-decrypt options for existing private handles.
 *
 * Direct `handles` are decrypted as-is. Without direct handles, the flow reads
 * the FHETest handle stored for the signer wallet and selected `type`.
 */
export type UserDecryptOptions = ClientOptions &
  Readonly<{
    type: FheValueType;
    contractAddress?: Hex;
    handles?: readonly Hex[];
    privateKey?: Hex;
    mnemonic?: string;
    durationDays: number;
    onProgress?: ProgressReporter;
  }>;

/** Decrypts private handles owned by the signing wallet. */
export const userDecrypt = async (
  options: UserDecryptOptions,
): Promise<UserDecryptResult & { handles?: readonly FheTestHandle[] }> => {
  options.onProgress?.("Loading wallet and creating clients");
  const { account, contractAddress, publicClient, ...context } =
    createWalletContext(options);

  if (options.handles && options.handles.length > 0) {
    options.onProgress?.(
      `Using ${options.handles.length.toString()} provided handle(s)`,
    );
    return decryptUserValues(
      { ...context, contractAddress, publicClient },
      {
        encryptedValues: options.handles,
        signer: account,
        ownerAddress: account.address,
        durationDays: options.durationDays,
        onProgress: options.onProgress,
      },
    );
  }

  const handle = await readFheTestHandle({
    publicClient,
    contractAddress,
    account: account.address,
    type: options.type,
    onProgress: options.onProgress,
  });
  const decrypted = await decryptUserValues(
    { ...context, contractAddress, publicClient },
    {
      encryptedValues: [handle.handle],
      signer: account,
      ownerAddress: account.address,
      durationDays: options.durationDays,
      onProgress: options.onProgress,
    },
  );

  return { ...decrypted, handles: [handle] };
};
