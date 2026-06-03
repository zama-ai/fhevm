import type { Hex } from "viem";

import type { ClientContext } from "../config";
import type { ProgressReporter } from "../shared/progress";
import type { PublicDecryptResult } from "../types";

type FhevmClient = ClientContext["fhevm"];

export const readPublicValues = async (
  fhevm: FhevmClient,
  encryptedValues: readonly Hex[],
  onProgress?: ProgressReporter,
): Promise<PublicDecryptResult> => {
  onProgress?.(
    `Requesting public decryption for ${encryptedValues.length.toString()} handle(s)`,
  );
  const result = await fhevm.decryptPublicValuesWithSignatures({
    encryptedValues,
  });
  onProgress?.("Public decryption received and signatures verified");

  return {
    encryptedValues,
    clearValues: result.clearValues.map((value) => ({
      type: value.type,
      value:
        typeof value.value === "bigint"
          ? value.value.toString()
          : String(value.value),
    })),
    abiEncodedCleartexts: result.checkSignaturesArgs
      .abiEncodedCleartexts as Hex,
    decryptionProof: result.checkSignaturesArgs.decryptionProof as Hex,
  };
};
