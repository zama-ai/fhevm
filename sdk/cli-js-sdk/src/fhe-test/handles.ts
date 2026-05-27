import type { Hex } from "viem";

import { fheTestAbi } from "./abi";
import type { ClientContext } from "../config";
import type { ProgressReporter } from "../shared/progress";
import type { FheTestHandle, FheValueType } from "../types";
import { FHE_TYPE_IDS } from "../types";

type PublicClient = ClientContext["publicClient"];

const ZERO_HANDLE =
  "0x0000000000000000000000000000000000000000000000000000000000000000";

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

  const handle = (await options.publicClient.readContract({
    address: options.contractAddress,
    abi: fheTestAbi,
    functionName: "getHandleOf",
    args: [options.account, FHE_TYPE_IDS[options.type]],
  } as never)) as Hex;
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
