import type { Hex } from "viem";

import { createClientContext, type ClientOptions } from "../../config";
import { readPublicValues } from "../../fhevm/public-decrypt";
import type { ProgressReporter } from "../../shared/progress";
import type {
  FheTestHandle,
  FheValueType,
  PublicDecryptResult,
} from "../../types";
import { readStoredFheTestHandles } from "../handles";
import { describeHandle } from "../progress";
import { resolveAccountAddress } from "./account";

/**
 * Options for public-decrypting FHETest handles stored in account/type slots.
 *
 * The flow reads FHETest handles for `account` and each selected type,
 * defaulting to the bool slot when no `types` are given. `account` may be
 * inferred from wallet credentials.
 */
export type StoredPublicDecryptOptions = ClientOptions &
  Readonly<{
    types?: readonly FheValueType[];
    contractAddress?: Hex;
    account?: Hex;
    privateKey?: Hex;
    mnemonic?: string;
    onProgress?: ProgressReporter;
  }>;

/** Reads FHETest handles for account/type slots and public-decrypts them. */
export const storedPublicDecrypt = async (
  options: StoredPublicDecryptOptions,
): Promise<PublicDecryptResult & { handles: readonly FheTestHandle[] }> => {
  options.onProgress?.("Creating FHEVM client");
  const { contractAddress, fhevm, publicClient } = createClientContext(options);

  const account = resolveAccountAddress(options);
  const storedHandles = await readStoredFheTestHandles(
    { contractAddress, publicClient },
    {
      account,
      types: options.types ?? [],
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
