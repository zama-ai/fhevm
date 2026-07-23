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
import { describeValues } from "./progress";

/**
 * Options for requesting an input proof without writing anything to FHETest.
 *
 * `values` wins over `value`; when neither is supplied the flow generates one
 * random value of `type` so the command can be used as a smoke test.
 */
export type RequestInputProofOptions = ClientOptions &
  Readonly<{
    type?: FheValueType;
    contractAddress?: Hex;
    userAddress?: Hex;
    value?: FheClearValue;
    values?: readonly EncryptValue[];
    onProgress?: ProgressReporter;
  }>;

/**
 * Encrypts clear values for a user/contract pair and returns the SDK input proof.
 *
 * This is the pure input-proof flow: it does not submit a transaction and does
 * not persist handles into FHETest.
 */
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
  options.onProgress?.(`Input value(s): ${describeValues(values)}`);

  const encrypted = await encryptValues(fhevm, {
    contractAddress,
    userAddress,
    values,
    onProgress: options.onProgress,
    progressLabel: `Encrypting ${values.length.toString()} ${valueType} value(s) and requesting verified input proof`,
  });
  options.onProgress?.(
    `Encrypted input handle(s): ${encrypted.encryptedValues.join(", ")}`,
  );

  options.onProgress?.("Input proof received");
  return {
    contractAddress,
    userAddress,
    values,
    encryptedValues: encrypted.encryptedValues,
    inputProof: encrypted.inputProof,
  };
};
