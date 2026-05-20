import type { FhevmChain } from "@fhevm/sdk/chains";
import type { Hex } from "viem";

import { relayerSdkTestAbi } from "./abi";
import {
  createClients,
  createWallet,
  TESTNET_RELAYER_SDK_TEST_CONTRACT,
  type ClientOptions,
} from "./config";
import { resolveCachedHandles } from "./handles";
import type {
  DecryptType,
  EncryptValue,
  InputProofResult,
  PublicDecryptResult,
} from "./types";
import { createFreshDecryptValues, createInputProofValues } from "./values";

type FhevmClientLike = ReturnType<typeof createClients>["fhevm"];
export type ProgressReporter = (message: string) => void;

export type RequestInputProofOptions = ClientOptions &
  Readonly<{
    contractAddress?: Hex;
    userAddress?: Hex;
    values?: readonly EncryptValue[];
    onProgress?: ProgressReporter;
  }>;

export const requestInputProof = async (
  options: RequestInputProofOptions,
): Promise<InputProofResult> => {
  options.onProgress?.("Creating FHEVM client");
  const { fhevm } = createClients(options);
  const contractAddress =
    options.contractAddress ?? "0x0000000000000000000000000000000000000001";
  const userAddress =
    options.userAddress ?? "0x0000000000000000000000000000000000000002";
  const values = options.values ?? createInputProofValues();

  options.onProgress?.(
    `Encrypting ${values.length.toString()} value(s) and requesting verified input proof`,
  );
  const encrypted = await fhevm.encryptValues({
    contractAddress,
    userAddress,
    values,
  });

  options.onProgress?.("Input proof received");
  return {
    contractAddress,
    userAddress,
    values,
    encryptedValues: encrypted.encryptedValues as readonly Hex[],
    inputProof: encrypted.inputProof as Hex,
  };
};

export type PublicDecryptOptions = ClientOptions &
  Readonly<{
    decryptType: DecryptType;
    handles?: readonly Hex[];
    onProgress?: ProgressReporter;
  }>;

export const publicDecrypt = async (
  options: PublicDecryptOptions,
): Promise<PublicDecryptResult> => {
  options.onProgress?.("Creating FHEVM client");
  const { fhevm } = createClients(options);
  const encryptedValues = resolveCachedHandles(
    options.decryptType,
    options.handles,
  );
  return readPublicValues(fhevm, encryptedValues, options.onProgress);
};

export type FreshPublicDecryptOptions = ClientOptions &
  Readonly<{
    decryptType: DecryptType;
    contractAddress?: Hex;
    privateKey?: Hex;
    mnemonic?: string;
    onProgress?: ProgressReporter;
  }>;

const functionNameByType: Record<
  DecryptType,
  (typeof relayerSdkTestAbi)[number]["name"]
> = {
  bool: "makePubliclyDecryptableExternalEbool",
  uint8: "makePubliclyDecryptableExternalEuint8",
  uint128: "makePubliclyDecryptableExternalEuint128",
  address: "makePubliclyDecryptableExternalEaddress",
  mixed: "makePubliclyDecryptableExternalMixed",
};

export const freshPublicDecrypt = async (
  options: FreshPublicDecryptOptions,
): Promise<
  PublicDecryptResult & {
    transactionHash: Hex;
    inputProof: Hex;
    inputValues: readonly EncryptValue[];
  }
> => {
  options.onProgress?.("Loading wallet and creating clients");
  const { account, fhevm, publicClient, walletClient } = createWallet(options);
  const contractAddress =
    options.contractAddress ?? TESTNET_RELAYER_SDK_TEST_CONTRACT;
  const values = createFreshDecryptValues(options.decryptType);

  options.onProgress?.(
    `Encrypting ${values.length.toString()} ${options.decryptType} value(s)`,
  );
  const encrypted = await fhevm.encryptValues({
    contractAddress,
    userAddress: account.address,
    values,
  });

  options.onProgress?.("Simulating make-publicly-decryptable transaction");
  const args = [
    ...encrypted.encryptedValues,
    encrypted.inputProof,
  ] as readonly Hex[];
  const { request, result } = await publicClient.simulateContract({
    account,
    address: contractAddress,
    abi: relayerSdkTestAbi,
    functionName: functionNameByType[options.decryptType],
    args: args as unknown as
      | readonly [Hex, Hex]
      | readonly [Hex, Hex, Hex, Hex, Hex],
  });
  options.onProgress?.("Sending transaction");
  const transactionHash = await walletClient.writeContract(request);
  options.onProgress?.(`Waiting for transaction receipt: ${transactionHash}`);
  await publicClient.waitForTransactionReceipt({ hash: transactionHash });

  const handles = (Array.isArray(result) ? result : [result]) as readonly Hex[];
  const decrypted = await readPublicValues(fhevm, handles, options.onProgress);

  return {
    ...decrypted,
    transactionHash,
    inputProof: encrypted.inputProof as Hex,
    inputValues: values,
  };
};

const readPublicValues = async (
  fhevm: FhevmClientLike,
  encryptedValues: readonly Hex[],
  onProgress?: ProgressReporter,
): Promise<PublicDecryptResult> => {
  onProgress?.(
    `Requesting public decryption for ${encryptedValues.length.toString()} handle(s)`,
  );
  const result = await fhevm.readPublicValuesWithSignatures({
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

export const describeNetwork = (
  chain: FhevmChain,
): Readonly<{ chainId: number; relayerUrl: string }> => ({
  chainId: chain.id,
  relayerUrl: chain.fhevm.relayerUrl,
});
