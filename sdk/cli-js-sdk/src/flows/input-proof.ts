import type { Hex } from "viem";

import { createClientContext, type ClientOptions } from "../config";
import { encryptValues } from "../fhevm/encryption";
import type { ProgressReporter } from "../shared/progress";
import type {
  EncryptValue,
  FheClearValue,
  FheValueType,
  InputProofResult,
} from "../types";
import { createRandomValue } from "../values";

export type RequestInputProofOptions = ClientOptions &
  Readonly<{
    type?: FheValueType;
    contractAddress?: Hex;
    userAddress?: Hex;
    value?: FheClearValue;
    values?: readonly EncryptValue[];
    onProgress?: ProgressReporter;
  }>;

export const requestInputProof = async (
  options: RequestInputProofOptions,
): Promise<InputProofResult> => {
  options.onProgress?.("Creating FHEVM client");
  const { contractAddress, fhevm } = createClientContext(options);
  const userAddress =
    options.userAddress ?? "0x0000000000000000000000000000000000000002";
  const valueType = options.type ?? "bool";
  const values =
    options.values ??
    (options.value === undefined
      ? [createRandomValue(valueType)]
      : [{ type: valueType, value: options.value }]);

  const encrypted = await encryptValues(fhevm, {
    contractAddress,
    userAddress,
    values,
    onProgress: options.onProgress,
    progressLabel: `Encrypting ${values.length.toString()} ${valueType} value(s) and requesting verified input proof`,
  });

  options.onProgress?.("Input proof received");
  return {
    contractAddress,
    userAddress,
    values,
    encryptedValues: encrypted.encryptedValues,
    inputProof: encrypted.inputProof,
  };
};
