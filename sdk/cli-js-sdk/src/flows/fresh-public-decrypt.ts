import type { Hex } from "viem";

import { createWalletContext, type ClientOptions } from "../config";
import { readFheTestHandle } from "../fhe-test/handles";
import {
  getSetEncryptedFunctionName,
  simulateSetEncryptedValue,
} from "../fhe-test/writes";
import { encryptValues } from "../fhevm/encryption";
import { readPublicValues } from "../fhevm/public-decrypt";
import type { ProgressReporter } from "../shared/progress";
import { sendAndWait } from "../shared/transactions";
import type {
  EncryptValue,
  FheClearValue,
  FheTestHandle,
  FheValueType,
  PublicDecryptResult,
} from "../types";
import { createFreshDecryptValues } from "../values";

export type FreshPublicDecryptOptions = ClientOptions &
  Readonly<{
    type: FheValueType;
    contractAddress?: Hex;
    value?: FheClearValue;
    privateKey?: Hex;
    mnemonic?: string;
    onProgress?: ProgressReporter;
  }>;

export const freshPublicDecrypt = async (
  options: FreshPublicDecryptOptions,
): Promise<
  PublicDecryptResult & {
    transactionHash: Hex;
    inputProof: Hex;
    inputValues: readonly EncryptValue[];
    handle: FheTestHandle;
  }
> => {
  options.onProgress?.("Loading wallet and creating clients");
  const { account, contractAddress, fhevm, publicClient, walletClient } =
    createWalletContext(options);
  const values =
    options.value === undefined
      ? createFreshDecryptValues(options.type)
      : [{ type: options.type, value: options.value }];
  const value = values[0];
  if (!value) throw new Error("No value to encrypt.");

  options.onProgress?.(`Encrypting ${options.type} value`);
  const encrypted = await encryptValues(fhevm, {
    contractAddress,
    userAddress: account.address,
    values,
  });

  const encryptedValue = encrypted.encryptedValues[0];
  if (!encryptedValue) throw new Error("FHEVM SDK did not return a handle.");

  options.onProgress?.(
    `Simulating FHETest.${getSetEncryptedFunctionName(options.type)}`,
  );
  const request = await simulateSetEncryptedValue(
    { account, contractAddress, publicClient },
    {
      encryptedValue,
      inputProof: encrypted.inputProof,
      value,
      makePublic: true,
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
    type: options.type,
    onProgress: options.onProgress,
  });
  const decrypted = await readPublicValues(
    fhevm,
    [handle.handle],
    options.onProgress,
  );

  return {
    ...decrypted,
    transactionHash,
    inputProof: encrypted.inputProof,
    inputValues: values,
    handle,
  };
};
