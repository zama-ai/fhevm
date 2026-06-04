import type { Hex } from "viem";

import { createClientContext, type ClientOptions } from "../../config";
import { readFheTestHandle } from "../../fhe-test/handles";
import { readPublicValues } from "../../fhevm/public-decrypt";
import type { ProgressReporter } from "../../shared/progress";
import type {
  FheTestHandle,
  FheValueType,
  PublicDecryptResult,
} from "../../types";
import { describeHandle } from "../progress";
import { resolveAccountAddress } from "./account";

const DEFAULT_CACHED_TYPES: readonly FheValueType[] = ["bool"];

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
  const handles = options.handles ?? [];
  const types = options.types ?? [];

  if (handles.length > 0 && types.length > 0) {
    throw new Error("Use either --handle or --type for cached decrypt, not both.");
  }

  options.onProgress?.("Creating FHEVM client");
  const { contractAddress, fhevm, publicClient } = createClientContext(options);

  if (handles.length > 0) {
    options.onProgress?.(`Using ${handles.length.toString()} provided handle(s)`);
    options.onProgress?.(`Provided handle(s): ${handles.join(", ")}`);
    return readPublicValues(fhevm, handles, options.onProgress);
  }

  const account = resolveAccountAddress(options);
  const selectedTypes = types.length > 0 ? types : DEFAULT_CACHED_TYPES;
  const storedHandles: FheTestHandle[] = [];
  for (const type of selectedTypes) {
    const handle = await readFheTestHandle({
      publicClient,
      contractAddress,
      account,
      type,
      onProgress: options.onProgress,
    });
    options.onProgress?.(
      `Using stored public ${type} handle: ${describeHandle(handle)}`,
    );
    storedHandles.push(handle);
  }
  const decrypted = await readPublicValues(
    fhevm,
    storedHandles.map((handle) => handle.handle),
    options.onProgress,
  );
  return { ...decrypted, handles: storedHandles };
};
