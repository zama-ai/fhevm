import type { Hex } from "viem";

import { createWalletContext, type ClientOptions } from "../../config";
import { readFheTestHandle } from "../../fhe-test/handles";
import {
  type FheTestOperation,
  getFheTestOperationFunctionName,
  getFheTestOperationType,
} from "../../types";
import { simulateFheTestOperation } from "../../fhe-test/writes";
import { encryptValues } from "../../fhevm/encryption";
import type { ProgressReporter } from "../../shared/progress";
import { sendAndWait } from "../../shared/transactions";
import type {
  EncryptValue,
  FheClearValue,
  FheTestHandle,
} from "../../types";
import { createRandomValue } from "../../values";
import { describeHandle, describeValue } from "../progress";

export type { FheTestOperation };

/**
 * Options for FHETest operator demos.
 *
 * Each operation has a fixed encrypted type. The command encrypts one operand,
 * applies the operation to the existing stored handle, and stores the result.
 */
export type RunFheTestOperationOptions = ClientOptions &
  Readonly<{
    operation: FheTestOperation;
    contractAddress?: Hex;
    value?: FheClearValue;
    makePublic?: boolean;
    privateKey?: Hex;
    mnemonic?: string;
    onProgress?: ProgressReporter;
  }>;

/** Result for a FHETest operation write, including before/after handles. */
export type RunFheTestOperationResult = Readonly<{
  operation: FheTestOperation;
  functionName: string;
  type: EncryptValue["type"];
  contractAddress: Hex;
  account: Hex;
  transactionHash: Hex;
  inputValues: readonly EncryptValue[];
  inputProof: Hex;
  previousHandle: FheTestHandle;
  handle: FheTestHandle;
  makePublic: boolean;
}>;

/** Runs one explicit FHETest operator demo against the wallet's stored handle. */
export const runFheTestOperation = async (
  options: RunFheTestOperationOptions,
): Promise<RunFheTestOperationResult> => {
  options.onProgress?.("Loading wallet and creating clients");
  const { account, contractAddress, fhevm, publicClient, walletClient } =
    createWalletContext(options);
  const type = getFheTestOperationType(options.operation);
  const value: EncryptValue =
    options.value === undefined
      ? createRandomValue(type)
      : { type, value: options.value };
  const makePublic = options.makePublic ?? false;
  options.onProgress?.(`Operation input: ${describeValue(value)}`);

  const previousHandle = await readFheTestHandle({
    publicClient,
    contractAddress,
    account: account.address,
    type,
    onProgress: options.onProgress,
  });
  options.onProgress?.(
    `Previous ${type} handle: ${describeHandle(previousHandle)}`,
  );

  const encrypted = await encryptValues(fhevm, {
    contractAddress,
    userAddress: account.address,
    values: [value],
    onProgress: options.onProgress,
    progressLabel: `Encrypting ${type} operation input`,
  });
  const encryptedValue = encrypted.encryptedValues[0];
  if (!encryptedValue) throw new Error("FHEVM SDK did not return a handle.");
  options.onProgress?.(`Encrypted operation input handle: ${encryptedValue}`);

  const functionName = getFheTestOperationFunctionName(options.operation);
  options.onProgress?.(`Simulating FHETest.${functionName}`);
  const request = await simulateFheTestOperation(
    { account, contractAddress, publicClient },
    {
      operation: options.operation,
      encryptedValue,
      inputProof: encrypted.inputProof,
      value,
      makePublic,
    },
  );
  const transactionHash = await sendAndWait({
    walletClient,
    publicClient,
    request,
    onProgress: options.onProgress,
  });

  const handle = await readFheTestHandle({
    publicClient,
    contractAddress,
    account: account.address,
    type,
    onProgress: options.onProgress,
  });
  options.onProgress?.(`Updated ${type} handle: ${describeHandle(handle)}`);

  return {
    operation: options.operation,
    functionName,
    type,
    contractAddress,
    account: account.address,
    transactionHash,
    inputValues: [value],
    inputProof: encrypted.inputProof,
    previousHandle,
    handle,
    makePublic,
  };
};
