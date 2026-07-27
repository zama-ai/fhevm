import type { Hex } from "viem";

import {
  createClientContext,
  loadAccount,
  type ClientOptions,
} from "../../config";
import {
  getHandleType,
  getHandleTypeId,
  hasFheTestClearText,
  hasFheTestHandle,
  readFheTestClearText,
  readFheTestStoredHandle,
} from "../../fhe-test/handles";
import type { ProgressReporter } from "../../shared/progress";
import type { FheValueType } from "../../types";
import { FHE_TYPE_IDS } from "../../types";

/**
 * Options for read-only FHETest inspection.
 *
 * `handle` mode is mutually exclusive with account/type mode. In account/type
 * mode, account can be inferred from wallet credentials.
 */
export type InspectFheTestOptions = ClientOptions &
  Readonly<{
    account?: Hex;
    contractAddress?: Hex;
    handle?: Hex;
    mnemonic?: string;
    privateKey?: Hex;
    type?: FheValueType;
    onProgress?: ProgressReporter;
  }>;

/** Inspection result for a raw handle, independent of any account mapping. */
export type InspectFheTestHandleResult = Readonly<{
  mode: "handle";
  contractAddress: Hex;
  handle: Hex;
  handleTypeId: number;
  handleType?: FheValueType;
  hasClearText: boolean;
  clearText?: string;
}>;

/** Inspection result for the handle FHETest stores for one account/type pair. */
export type InspectFheTestAccountResult = Readonly<{
  mode: "account";
  contractAddress: Hex;
  account: Hex;
  type: FheValueType;
  fheTypeId: number;
  hasHandle: boolean;
  handle?: Hex;
  handleTypeId?: number;
  handleType?: FheValueType;
  hasClearText?: boolean;
  clearText?: string;
}>;

export type InspectFheTestResult =
  | InspectFheTestHandleResult
  | InspectFheTestAccountResult;

/** Performs read-only FHETest handle or account/type inspection. */
export const inspectFheTest = async (
  options: InspectFheTestOptions,
): Promise<InspectFheTestResult> => {
  if (options.handle) {
    if (options.account || options.type || options.privateKey || options.mnemonic) {
      throw new Error(
        "Use either --handle, or account/type inspection options, not both.",
      );
    }

    options.onProgress?.("Creating clients");
    const { contractAddress, publicClient } = createClientContext(options);
    options.onProgress?.("Inspecting FHETest handle");
    const hasClearText = await hasFheTestClearText({
      publicClient,
      contractAddress,
      handle: options.handle,
    });
    const clearText = hasClearText
      ? await readFheTestClearText({
          publicClient,
          contractAddress,
          handle: options.handle,
        })
      : undefined;

    return {
      mode: "handle",
      contractAddress,
      handle: options.handle,
      handleTypeId: getHandleTypeId(options.handle),
      handleType: getHandleType(options.handle),
      hasClearText,
      clearText: clearText?.toString(),
    };
  }

  if (!options.type) {
    throw new Error("Provide --type for account/type inspection.");
  }

  options.onProgress?.("Creating clients");
  const { contractAddress, publicClient } = createClientContext(options);
  const account =
    options.account ?? loadAccount(options.privateKey, options.mnemonic).address;

  options.onProgress?.(`Inspecting FHETest handle for ${account} / ${options.type}`);
  const hasHandle = await hasFheTestHandle({
    publicClient,
    contractAddress,
    account,
    type: options.type,
  });

  if (!hasHandle) {
    return {
      mode: "account",
      contractAddress,
      account,
      type: options.type,
      fheTypeId: FHE_TYPE_IDS[options.type],
      hasHandle,
    };
  }

  const handle = await readFheTestStoredHandle({
    publicClient,
    contractAddress,
    account,
    type: options.type,
  });
  const hasClearText = await hasFheTestClearText({
    publicClient,
    contractAddress,
    handle,
  });
  const clearText = hasClearText
    ? await readFheTestClearText({
        publicClient,
        contractAddress,
        handle,
      })
    : undefined;

  return {
    mode: "account",
    contractAddress,
    account,
    type: options.type,
    fheTypeId: FHE_TYPE_IDS[options.type],
    hasHandle,
    handle,
    handleTypeId: getHandleTypeId(handle),
    handleType: getHandleType(handle),
    hasClearText,
    clearText: clearText?.toString(),
  };
};
