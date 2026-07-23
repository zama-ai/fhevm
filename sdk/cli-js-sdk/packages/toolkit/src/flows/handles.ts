import type { Hex } from "viem";

import type { ClientContext, WalletContext } from "../config";
import { readFheTestHandle } from "../fhe-test/handles";
import {
  getSetEncryptedFunctionName,
  simulateSetEncryptedValue,
} from "../fhe-test/writes";
import { encryptValues } from "../fhevm/encryption";
import type { ProgressReporter } from "../shared/progress";
import { sendAndWait } from "../shared/transactions";
import type {
  EncryptValue,
  FheClearValue,
  FheTestHandle,
  FheValueType,
} from "../types";
import { createFreshDecryptValues } from "../values";
import { describeValue } from "./progress";

type HandleReadContext = Pick<ClientContext, "contractAddress" | "publicClient">;

export const DEFAULT_STORED_TYPES: readonly FheValueType[] = ["bool"];

/** Reports direct handles supplied to a decrypt family root command. */
export const reportProvidedHandles = (
  handles: readonly Hex[],
  onProgress: ProgressReporter | undefined,
): void => {
  onProgress?.(`Using ${handles.length.toString()} provided handle(s)`);
  onProgress?.(`Provided handle(s): ${handles.join(", ")}`);
};

/** Reads FHETest handles for selected stored types, defaulting to bool. */
export const readStoredFheTestHandles = async (
  context: HandleReadContext,
  options: {
    account: Hex;
    types: readonly FheValueType[];
    describeStoredHandle: (type: FheValueType, handle: FheTestHandle) => string;
    onProgress?: ProgressReporter;
  },
): Promise<readonly FheTestHandle[]> => {
  const selectedTypes =
    options.types.length > 0 ? options.types : DEFAULT_STORED_TYPES;
  const storedHandles: FheTestHandle[] = [];

  for (const type of selectedTypes) {
    const handle = await readFheTestHandle({
      publicClient: context.publicClient,
      contractAddress: context.contractAddress,
      account: options.account,
      type,
      onProgress: options.onProgress,
    });
    options.onProgress?.(options.describeStoredHandle(type, handle));
    storedHandles.push(handle);
  }

  return storedHandles;
};

/** Encrypts, stores, and rereads one FHETest handle for fresh decrypt flows. */
export const createStoredFheTestHandle = async (
  context: WalletContext,
  options: {
    type: FheValueType;
    ownerAddress: Hex;
    value?: FheClearValue;
    makePublic: boolean;
    inputProgressLabel: string;
    encryptProgressLabel: string;
    encryptedProgressLabel: string;
    storedProgressLabel: (handle: FheTestHandle) => string;
    onProgress?: ProgressReporter;
  },
): Promise<{
  transactionHash: Hex;
  inputProof: Hex;
  inputValues: readonly EncryptValue[];
  handle: FheTestHandle;
}> => {
  const values =
    options.value === undefined
      ? createFreshDecryptValues(options.type)
      : [{ type: options.type, value: options.value }];
  const value = values[0];
  if (!value) throw new Error("No value to encrypt.");
  options.onProgress?.(`${options.inputProgressLabel}: ${describeValue(value)}`);

  const encrypted = await encryptValues(context.fhevm, {
    contractAddress: context.contractAddress,
    userAddress: options.ownerAddress,
    values,
    onProgress: options.onProgress,
    progressLabel: options.encryptProgressLabel,
  });

  const encryptedValue = encrypted.encryptedValues[0];
  if (!encryptedValue) throw new Error("FHEVM SDK did not return a handle.");
  options.onProgress?.(`${options.encryptedProgressLabel}: ${encryptedValue}`);

  options.onProgress?.(
    `Simulating FHETest.${getSetEncryptedFunctionName(options.type)}`,
  );
  const request = await simulateSetEncryptedValue(
    {
      account: context.account,
      contractAddress: context.contractAddress,
      publicClient: context.publicClient,
    },
    {
      encryptedValue,
      inputProof: encrypted.inputProof,
      value,
      makePublic: options.makePublic,
    },
  );
  const transactionHash = await sendAndWait({
    walletClient: context.walletClient,
    publicClient: context.publicClient,
    request,
    onProgress: options.onProgress,
  });

  const handle = await readFheTestHandle({
    publicClient: context.publicClient,
    contractAddress: context.contractAddress,
    account: options.ownerAddress,
    type: options.type,
    onProgress: options.onProgress,
  });
  options.onProgress?.(options.storedProgressLabel(handle));

  return {
    transactionHash,
    inputProof: encrypted.inputProof,
    inputValues: values,
    handle,
  };
};
