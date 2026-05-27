import type { Hex } from "viem";

import { fheTestAbi } from "../abi";
import type { WalletContext } from "../config";
import type { EncryptValue, FheValueType } from "../types";
import { FHE_TYPE_IDS } from "../types";

const setEncryptedFunctionByType = {
  bool: "setEbool",
  uint8: "setEuint8",
  uint16: "setEuint16",
  uint32: "setEuint32",
  uint64: "setEuint64",
  uint128: "setEuint128",
  uint256: "setEuint256",
  address: "setEaddress",
} as const satisfies Record<FheValueType, string>;

const setClearFunctionByType = {
  bool: "setClearEbool",
  uint8: "setClearEuint8",
  uint16: "setClearEuint16",
  uint32: "setClearEuint32",
  uint64: "setClearEuint64",
  uint128: "setClearEuint128",
  uint256: "setClearEuint256",
  address: "setClearEaddress",
} as const satisfies Record<FheValueType, string>;

type WriteContext = Pick<
  WalletContext,
  "account" | "contractAddress" | "publicClient"
>;

export const simulateSetEncryptedValue = async (
  context: WriteContext,
  options: {
    encryptedValue: Hex;
    inputProof: Hex;
    value: EncryptValue;
    makePublic: boolean;
  },
): Promise<unknown> => {
  const { request } = await context.publicClient.simulateContract({
    account: context.account,
    address: context.contractAddress,
    abi: fheTestAbi,
    functionName: setEncryptedFunctionByType[options.value.type],
    args: [
      options.encryptedValue,
      options.inputProof,
      options.value.value,
      options.makePublic,
    ],
  } as never);

  return request;
};

export const simulateSetClearValue = async (
  context: WriteContext,
  options: {
    value: EncryptValue;
    makePublic: boolean;
  },
): Promise<unknown> => {
  const { request } = await context.publicClient.simulateContract({
    account: context.account,
    address: context.contractAddress,
    abi: fheTestAbi,
    functionName: setClearFunctionByType[options.value.type],
    args: [options.value.value, options.makePublic],
  } as never);

  return request;
};

export const simulateMakePubliclyDecryptable = async (
  context: WriteContext,
  type: FheValueType,
): Promise<unknown> => {
  const { request } = await context.publicClient.simulateContract({
    account: context.account,
    address: context.contractAddress,
    abi: fheTestAbi,
    functionName: "makePubliclyDecryptable",
    args: [FHE_TYPE_IDS[type]],
  } as never);

  return request;
};

export const getSetEncryptedFunctionName = (type: FheValueType): string =>
  setEncryptedFunctionByType[type];

export const getSetClearFunctionName = (type: FheValueType): string =>
  setClearFunctionByType[type];
