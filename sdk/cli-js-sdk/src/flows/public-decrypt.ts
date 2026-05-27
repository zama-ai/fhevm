import type { Hex } from "viem";

import { createClientContext, type ClientOptions } from "../config";
import { readFheTestHandle } from "../fhe-test/handles";
import { readPublicValues } from "../fhevm/public-decrypt";
import type { ProgressReporter } from "../shared/progress";
import type {
  FheTestHandle,
  FheValueType,
  PublicDecryptResult,
} from "../types";
import { resolveAccountAddress } from "./account";

export type PublicDecryptOptions = ClientOptions &
  Readonly<{
    type: FheValueType;
    contractAddress?: Hex;
    account?: Hex;
    handles?: readonly Hex[];
    privateKey?: Hex;
    mnemonic?: string;
    onProgress?: ProgressReporter;
  }>;

export const publicDecrypt = async (
  options: PublicDecryptOptions,
): Promise<PublicDecryptResult & { handles?: readonly FheTestHandle[] }> => {
  options.onProgress?.("Creating FHEVM client");
  const { contractAddress, fhevm, publicClient } = createClientContext(options);

  if (options.handles && options.handles.length > 0) {
    options.onProgress?.(
      `Using ${options.handles.length.toString()} provided handle(s)`,
    );
    return readPublicValues(fhevm, options.handles, options.onProgress);
  }

  const account = resolveAccountAddress(options);
  const handle = await readFheTestHandle({
    publicClient,
    contractAddress,
    account,
    type: options.type,
    onProgress: options.onProgress,
  });
  const decrypted = await readPublicValues(
    fhevm,
    [handle.handle],
    options.onProgress,
  );
  return { ...decrypted, handles: [handle] };
};
