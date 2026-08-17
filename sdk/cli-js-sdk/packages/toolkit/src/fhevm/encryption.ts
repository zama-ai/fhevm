import type { Hex } from "viem";

import type { ClientContext } from "../config";
import type { ProgressReporter } from "../shared/progress";
import type { EncryptValue } from "../types";

type FhevmClient = ClientContext["fhevm"];

/** Normalized SDK encryption response used by flow code and JSON output. */
export type EncryptedInput = Readonly<{
  encryptedValues: readonly Hex[];
  inputProof: Hex;
}>;

/**
 * Encrypts typed clear values with the FHEVM SDK and normalizes SDK output.
 *
 * Raw SDK values are cast at this adapter boundary so flow code can treat
 * handles and proofs as viem `Hex` values.
 */
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
  // Await client readiness so the auth-carrying encrypt init prefetches the FHE
  // encryption key first. The SDK's createZkProof key fetch omits auth, and the
  // global key cache is first-write-wins; without this, the unauthenticated
  // fetch wins and `v2/keyurl` 403s on relayers that require an API key.
  await fhevm.ready;
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
