import type { Hex } from "viem";

import type { ClientContext } from "../config";
import type { ProgressReporter } from "../shared/progress";
import type { EncryptValue } from "../types";

type FhevmClient = ClientContext["fhevm"];

export type EncryptedInput = Readonly<{
  encryptedValues: readonly Hex[];
  inputProof: Hex;
}>;

export const encryptValues = async (
  fhevm: FhevmClient,
  options: {
    contractAddress: Hex;
    userAddress: Hex;
    values: readonly EncryptValue[];
    onProgress?: ProgressReporter;
    progressLabel?: string;
  },
): Promise<EncryptedInput> => {
  options.onProgress?.(
    options.progressLabel ??
      `Encrypting ${options.values.length.toString()} value(s)`,
  );
  const encrypted = await fhevm.encryptValues({
    contractAddress: options.contractAddress,
    userAddress: options.userAddress,
    values: options.values,
  });

  return {
    encryptedValues: encrypted.encryptedValues as readonly Hex[],
    inputProof: encrypted.inputProof as Hex,
  };
};
