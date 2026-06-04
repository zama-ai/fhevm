import type { Hex } from "viem";

import { createWalletContext, type ClientOptions } from "../../config";
import { readFheTestHandle } from "../../fhe-test/handles";
import {
  getSetEncryptedFunctionName,
  simulateSetEncryptedValue,
} from "../../fhe-test/writes";
import { encryptValues } from "../../fhevm/encryption";
import { decryptUserValues } from "../../fhevm/user-decrypt";
import type { ProgressReporter } from "../../shared/progress";
import { sendAndWait } from "../../shared/transactions";
import type {
  EncryptValue,
  FheClearValue,
  FheTestHandle,
  FheValueType,
  UserDecryptResult,
} from "../../types";
import { createFreshDecryptValues } from "../../values";

export type FreshUserDecryptOptions = ClientOptions &
  Readonly<{
    type: FheValueType;
    contractAddress?: Hex;
    value?: FheClearValue;
    privateKey?: Hex;
    mnemonic?: string;
    durationDays: number;
    onProgress?: ProgressReporter;
  }>;

export const freshUserDecrypt = async (
  options: FreshUserDecryptOptions,
): Promise<
  UserDecryptResult & {
    transactionHash: Hex;
    inputProof: Hex;
    inputValues: readonly EncryptValue[];
    handle: FheTestHandle;
  }
> => {
  options.onProgress?.("Loading wallet and creating clients");
  const { account, contractAddress, fhevm, publicClient, walletClient, ...context } =
    createWalletContext(options);
  const values =
    options.value === undefined
      ? createFreshDecryptValues(options.type)
      : [{ type: options.type, value: options.value }];
  const value = values[0];
  if (!value) throw new Error("No value to encrypt.");

  const encrypted = await encryptValues(fhevm, {
    contractAddress,
    userAddress: account.address,
    values,
    onProgress: options.onProgress,
    progressLabel: `Encrypting ${options.type} value`,
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
      makePublic: false,
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
  const decrypted = await decryptUserValues(
    { ...context, contractAddress, publicClient },
    {
      encryptedValues: [handle.handle],
      signer: account,
      ownerAddress: account.address,
      durationDays: options.durationDays,
      onProgress: options.onProgress,
    },
  );

  return {
    ...decrypted,
    transactionHash,
    inputProof: encrypted.inputProof,
    inputValues: values,
    handle,
  };
};
