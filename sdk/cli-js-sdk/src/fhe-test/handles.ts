import type { Hex } from "viem";

import { fheTestAbi } from "./abi";
import type { ClientContext } from "../config";
import type { ProgressReporter } from "../shared/progress";
import type { FheTestHandle, FheValueType } from "../types";
import { FHE_TYPE_IDS } from "../types";

type PublicClient = ClientContext["publicClient"];

const ZERO_HANDLE =
  "0x0000000000000000000000000000000000000000000000000000000000000000";

const FHE_TYPE_BY_ID = Object.fromEntries(
  Object.entries(FHE_TYPE_IDS).map(([type, id]) => [id, type]),
) as Partial<Record<number, FheValueType>>;

/** Extracts FHETest's FHE type id from the final byte of a handle. */
export const getHandleTypeId = (handle: Hex): number =>
  Number.parseInt(handle.slice(62, 64), 16);

/** Resolves a handle's embedded type id to a known CLI value type, if any. */
export const getHandleType = (handle: Hex): FheValueType | undefined =>
  FHE_TYPE_BY_ID[getHandleTypeId(handle)];

/** Checks whether FHETest has a stored handle for an account/type pair. */
export const hasFheTestHandle = async (options: {
  publicClient: PublicClient;
  contractAddress: Hex;
  account: Hex;
  type: FheValueType;
}): Promise<boolean> =>
  (await options.publicClient.readContract({
    address: options.contractAddress,
    abi: fheTestAbi,
    functionName: "hasHandleOf",
    args: [options.account, FHE_TYPE_IDS[options.type]],
  } as never)) as boolean;

/**
 * Reads and validates the FHETest handle for an account/type pair.
 *
 * Throws when no handle exists or FHETest returns the zero handle, because both
 * cases would make downstream decrypt flows misleading.
 */
export const readFheTestHandle = async (options: {
  publicClient: PublicClient;
  contractAddress: Hex;
  account: Hex;
  type: FheValueType;
  onProgress?: ProgressReporter;
}): Promise<FheTestHandle> => {
  options.onProgress?.(
    `Reading FHETest handle for ${options.account} / ${options.type}`,
  );
  const hasHandle = await hasFheTestHandle(options);
  if (!hasHandle) {
    throw new Error(
      `No FHETest handle for account ${options.account} and type ${options.type}. Run "fhe-test init", "user-decrypt fresh", "delegated-user-decrypt fresh", "public-decrypt fresh", or pass --handle.`,
    );
  }

  const handle = await readFheTestStoredHandle(options);
  if (handle === ZERO_HANDLE) {
    throw new Error(`FHETest returned an empty ${options.type} handle.`);
  }

  const clearText = (await options.publicClient.readContract({
    address: options.contractAddress,
    abi: fheTestAbi,
    functionName: "getClearText",
    args: [handle],
  } as never)) as bigint;

  return {
    type: options.type,
    fheTypeId: FHE_TYPE_IDS[options.type],
    account: options.account,
    handle,
    clearText: clearText.toString(),
  };
};

/** Reads the raw handle mapping from FHETest without existence validation. */
export const readFheTestStoredHandle = async (options: {
  publicClient: PublicClient;
  contractAddress: Hex;
  account: Hex;
  type: FheValueType;
}): Promise<Hex> =>
  (await options.publicClient.readContract({
    address: options.contractAddress,
    abi: fheTestAbi,
    functionName: "getHandleOf",
    args: [options.account, FHE_TYPE_IDS[options.type]],
  } as never)) as Hex;

/** Checks whether FHETest has an inspectable cleartext mirror for a handle. */
export const hasFheTestClearText = async (options: {
  publicClient: PublicClient;
  contractAddress: Hex;
  handle: Hex;
}): Promise<boolean> =>
  (await options.publicClient.readContract({
    address: options.contractAddress,
    abi: fheTestAbi,
    functionName: "hasClearText",
    args: [options.handle],
  } as never)) as boolean;

/** Reads FHETest's cleartext mirror for a handle. */
export const readFheTestClearText = async (options: {
  publicClient: PublicClient;
  contractAddress: Hex;
  handle: Hex;
}): Promise<bigint> =>
  (await options.publicClient.readContract({
    address: options.contractAddress,
    abi: fheTestAbi,
    functionName: "getClearText",
    args: [options.handle],
  } as never)) as bigint;
