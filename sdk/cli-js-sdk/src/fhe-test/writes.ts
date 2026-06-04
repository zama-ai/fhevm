import type { Hex } from "viem";

import { fheTestAbi } from "./abi";
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

export const FHE_TEST_OPERATIONS = [
  "xor-bool",
  "add-uint8",
  "add-uint16",
  "add-uint32",
  "add-uint64",
  "add-uint128",
  "xor-uint256",
  "eq-address",
] as const;

export type FheTestOperation = (typeof FHE_TEST_OPERATIONS)[number];

const operationConfig = {
  "xor-bool": { functionName: "xorEbool", type: "bool" },
  "add-uint8": { functionName: "addEuint8", type: "uint8" },
  "add-uint16": { functionName: "addEuint16", type: "uint16" },
  "add-uint32": { functionName: "addEuint32", type: "uint32" },
  "add-uint64": { functionName: "addEuint64", type: "uint64" },
  "add-uint128": { functionName: "addEuint128", type: "uint128" },
  "xor-uint256": { functionName: "xorEuint256", type: "uint256" },
  "eq-address": { functionName: "eqEaddress", type: "address" },
} as const satisfies Record<
  FheTestOperation,
  Readonly<{ functionName: string; type: FheValueType }>
>;

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

export const simulateInitFheTest = async (
  context: WriteContext,
  force: boolean,
): Promise<unknown> => {
  const { request } = await context.publicClient.simulateContract({
    account: context.account,
    address: context.contractAddress,
    abi: fheTestAbi,
    functionName: "initFheTest",
    args: [force],
  } as never);

  return request;
};

export const simulateFheTestOperation = async (
  context: WriteContext,
  options: {
    operation: FheTestOperation;
    encryptedValue: Hex;
    inputProof: Hex;
    value: EncryptValue;
    makePublic: boolean;
  },
): Promise<unknown> => {
  const config = operationConfig[options.operation];
  if (options.value.type !== config.type) {
    throw new Error(
      `${options.operation} expects ${config.type}, received ${options.value.type}.`,
    );
  }

  const { request } = await context.publicClient.simulateContract({
    account: context.account,
    address: context.contractAddress,
    abi: fheTestAbi,
    functionName: config.functionName,
    args: [
      options.encryptedValue,
      options.inputProof,
      options.value.value,
      options.makePublic,
    ],
  } as never);

  return request;
};

export const getSetEncryptedFunctionName = (type: FheValueType): string =>
  setEncryptedFunctionByType[type];

export const getSetClearFunctionName = (type: FheValueType): string =>
  setClearFunctionByType[type];

export const getFheTestOperationType = (
  operation: FheTestOperation,
): FheValueType => operationConfig[operation].type;

export const getFheTestOperationFunctionName = (
  operation: FheTestOperation,
): string => operationConfig[operation].functionName;
