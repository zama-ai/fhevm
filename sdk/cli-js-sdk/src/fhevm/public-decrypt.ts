import type { Hex } from "viem";

import type { ClientContext } from "../config";
import {
  describeDecryptedValues,
  type ProgressReporter,
} from "../shared/progress";
import type { PublicDecryptResult } from "../types";

type FhevmClient = ClientContext["fhevm"];

/**
 * Requests public decryption and returns proof material needed for verification.
 *
 * SDK value objects are converted to strings so stdout JSON is stable for
 * bigint-sized FHE integer types.
 */
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
  const clearValues = result.clearValues.map((value) => ({
    type: value.type,
    value:
      typeof value.value === "bigint"
        ? value.value.toString()
        : String(value.value),
  }));
  onProgress?.(`Public decrypted value(s): ${describeDecryptedValues(clearValues)}`);

  return {
    encryptedValues,
    clearValues,
    abiEncodedCleartexts: result.checkSignaturesArgs
      .abiEncodedCleartexts as Hex,
    decryptionProof: result.checkSignaturesArgs.decryptionProof as Hex,
  };
};
