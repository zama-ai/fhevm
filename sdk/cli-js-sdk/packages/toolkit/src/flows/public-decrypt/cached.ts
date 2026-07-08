import type { Hex } from "viem";

import { createClientContext, type ClientOptions } from "../../config";
import { readPublicValues } from "../../fhevm/public-decrypt";
import type { ProgressReporter } from "../../shared/progress";
import type {
  FheTestHandle,
  FheValueType,
  PublicDecryptResult,
} from "../../types";
import { describeHandle } from "../progress";
import {
  readCachedFheTestHandles,
  reportProvidedHandles,
  resolveCachedDecryptSelection,
} from "../handles";
import { resolveAccountAddress } from "./account";

/**
 * Public-decrypt options for existing ciphertext handles.
 *
 * `handles` and `types` are mutually exclusive modes. Direct handles are
 * decrypted as-is. Without direct handles, the flow reads FHETest handles for
 * `account` and each selected type; `account` may be inferred from wallet
 * credentials by the CLI layer.
 */
export type PublicDecryptOptions = ClientOptions &
  Readonly<{
    types?: readonly FheValueType[];
    contractAddress?: Hex;
    account?: Hex;
    handles?: readonly Hex[];
    privateKey?: Hex;
    mnemonic?: string;
    onProgress?: ProgressReporter;
  }>;

/** Reads existing public handles and requests relayer-backed public decryption. */
export const publicDecrypt = async (
  options: PublicDecryptOptions,
): Promise<PublicDecryptResult & { handles?: readonly FheTestHandle[] }> => {
  const { handles, types } = resolveCachedDecryptSelection(options);

  options.onProgress?.("Creating FHEVM client");
  const { contractAddress, fhevm, publicClient } = createClientContext(options);

  if (handles.length > 0) {
    reportProvidedHandles(handles, options.onProgress);
    return readPublicValues(fhevm, handles, options.onProgress);
  }

  const account = resolveAccountAddress(options);
  const storedHandles = await readCachedFheTestHandles(
    { contractAddress, publicClient },
    {
      account,
      types,
      describeStoredHandle: (type, handle) =>
        `Using stored public ${type} handle: ${describeHandle(handle)}`,
      onProgress: options.onProgress,
    },
  );
  const decrypted = await readPublicValues(
    fhevm,
    storedHandles.map((handle) => handle.handle),
    options.onProgress,
  );
  return { ...decrypted, handles: storedHandles };
};
