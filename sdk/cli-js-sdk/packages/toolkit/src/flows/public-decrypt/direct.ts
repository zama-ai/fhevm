import type { Hex } from "viem";

import { createClientContext, type ClientOptions } from "../../config";
import { readPublicValues } from "../../fhevm/public-decrypt";
import type { ProgressReporter } from "../../shared/progress";
import type { PublicDecryptResult } from "../../types";
import { reportProvidedHandles } from "../handles";

/**
 * Public-decrypt options for existing ciphertext handles.
 *
 * The supplied `handles` are decrypted as-is via the relayer. `contractAddress`
 * is the ACL pairing contract and defaults to the FHETest contract.
 */
export type PublicDecryptOptions = ClientOptions &
  Readonly<{
    handles: readonly Hex[];
    contractAddress?: Hex;
    privateKey?: Hex;
    mnemonic?: string;
    onProgress?: ProgressReporter;
  }>;

/** Requests relayer-backed public decryption for the supplied handles. */
export const publicDecrypt = async (
  options: PublicDecryptOptions,
): Promise<PublicDecryptResult> => {
  options.onProgress?.("Creating FHEVM client");
  const { fhevm } = createClientContext(options);

  reportProvidedHandles(options.handles, options.onProgress);
  return readPublicValues(fhevm, options.handles, options.onProgress);
};
