import type { Hex } from "viem";

import { createWalletContext, type ClientOptions } from "../../config";
import { readFheTestHandle } from "../../fhe-test/handles";
import { decryptUserValues } from "../../fhevm/user-decrypt";
import type { ProgressReporter } from "../../shared/progress";
import type { FheTestHandle, FheValueType, UserDecryptResult } from "../../types";
import { describeHandle } from "../progress";

const DEFAULT_CACHED_TYPES: readonly FheValueType[] = ["bool"];

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
    onProgress?: ProgressReporter;
  }>;

/** Decrypts private handles owned by the signing wallet. */
export const userDecrypt = async (
  options: UserDecryptOptions,
): Promise<UserDecryptResult & { handles?: readonly FheTestHandle[] }> => {
  const handles = options.handles ?? [];
  const types = options.types ?? [];

  if (handles.length > 0 && types.length > 0) {
    throw new Error("Use either --handle or --type for cached decrypt, not both.");
  }

  options.onProgress?.("Loading wallet and creating clients");
  const { account, contractAddress, publicClient, ...context } =
    createWalletContext(options);

  if (handles.length > 0) {
    options.onProgress?.(`Using ${handles.length.toString()} provided handle(s)`);
    options.onProgress?.(`Provided handle(s): ${handles.join(", ")}`);
    return decryptUserValues(
      { ...context, contractAddress, publicClient },
      {
        encryptedValues: handles,
        signer: account,
        ownerAddress: account.address,
        durationDays: options.durationDays,
        onProgress: options.onProgress,
      },
    );
  }

  const selectedTypes = types.length > 0 ? types : DEFAULT_CACHED_TYPES;
  const storedHandles: FheTestHandle[] = [];
  for (const type of selectedTypes) {
    const handle = await readFheTestHandle({
      publicClient,
      contractAddress,
      account: account.address,
      type,
      onProgress: options.onProgress,
    });
    options.onProgress?.(`Using stored ${type} handle: ${describeHandle(handle)}`);
    storedHandles.push(handle);
  }
  const decrypted = await decryptUserValues(
    { ...context, contractAddress, publicClient },
    {
      encryptedValues: storedHandles.map((handle) => handle.handle),
      signer: account,
      ownerAddress: account.address,
      durationDays: options.durationDays,
      onProgress: options.onProgress,
    },
  );

  return { ...decrypted, handles: storedHandles };
};
