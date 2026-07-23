import type { Hex } from "viem";

import { fheTestAbi } from "./abi";
import type { WalletContext } from "../config";
import type { ContractWriteRequest } from "../shared/transactions";
import type { EncryptValue, FheTestOperation, FheValueType } from "../types";
import {
  FHE_TYPE_IDS,
  getFheTestOperationFunctionName,
  getFheTestOperationType,
} from "../types";

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

const simulateWriteRequest = async (
  context: WriteContext,
  options: {
    functionName: string;
    args: readonly unknown[];
  },
): Promise<ContractWriteRequest> => {
  const { request } = await context.publicClient.simulateContract({
    account: context.account,
    address: context.contractAddress,
    abi: fheTestAbi,
    functionName: options.functionName,
    args: options.args,
  } as never);

  return request as ContractWriteRequest;
};

/** Simulates an encrypted setter call and returns the viem write request. */
export const simulateSetEncryptedValue = async (
  context: WriteContext,
  options: {
    encryptedValue: Hex;
    inputProof: Hex;
    value: EncryptValue;
    makePublic: boolean;
  },
): Promise<ContractWriteRequest> => {
  return simulateWriteRequest(context, {
    functionName: setEncryptedFunctionByType[options.value.type],
    args: [
      options.encryptedValue,
      options.inputProof,
      options.value.value,
      options.makePublic,
    ],
  });
};

/** Simulates a clear setter call used by FHETest initialization. */
export const simulateSetClearValue = async (
  context: WriteContext,
  options: {
    value: EncryptValue;
    makePublic: boolean;
  },
): Promise<ContractWriteRequest> => {
  return simulateWriteRequest(context, {
    functionName: setClearFunctionByType[options.value.type],
    args: [options.value.value, options.makePublic],
  });
};

/** Simulates marking an existing stored FHETest handle publicly decryptable. */
export const simulateMakePubliclyDecryptable = async (
  context: WriteContext,
  type: FheValueType,
): Promise<ContractWriteRequest> => {
  return simulateWriteRequest(context, {
    functionName: "makePubliclyDecryptable",
    args: [FHE_TYPE_IDS[type]],
  });
};

/** Simulates FHETest's contract-level all-types initializer. */
export const simulateInitFheTest = async (
  context: WriteContext,
  force: boolean,
): Promise<ContractWriteRequest> => {
  return simulateWriteRequest(context, {
    functionName: "initFheTest",
    args: [force],
  });
};

/**
 * Simulates a supported FHETest operation and validates its operand type.
 *
 * Operation names intentionally map to concrete contract functions instead of a
 * generic `type` flag, keeping CLI completion and help aligned with FHETest.sol.
 */
export const simulateFheTestOperation = async (
  context: WriteContext,
  options: {
    operation: FheTestOperation;
    encryptedValue: Hex;
    inputProof: Hex;
    value: EncryptValue;
    makePublic: boolean;
  },
): Promise<ContractWriteRequest> => {
  const type = getFheTestOperationType(options.operation);
  if (options.value.type !== type) {
    throw new Error(
      `${options.operation} expects ${type}, received ${options.value.type}.`,
    );
  }

  return simulateWriteRequest(context, {
    functionName: getFheTestOperationFunctionName(options.operation),
    args: [
      options.encryptedValue,
      options.inputProof,
      options.value.value,
      options.makePublic,
    ],
  });
};

/** Returns the FHETest encrypted setter name for a value type. */
export const getSetEncryptedFunctionName = (type: FheValueType): string =>
  setEncryptedFunctionByType[type];

/** Returns the FHETest clear setter name for a value type. */
export const getSetClearFunctionName = (type: FheValueType): string =>
  setClearFunctionByType[type];
