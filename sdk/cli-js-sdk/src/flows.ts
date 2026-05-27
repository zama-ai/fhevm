import type { FhevmChain } from "@fhevm/sdk/chains";
import type { Hex } from "viem";

import { fheTestAbi } from "./abi";
import {
  createClients,
  createWallet,
  loadAccount,
  resolveContractAddress,
  type ClientOptions,
} from "./config";
import type {
  EncryptValue,
  FheClearValue,
  FheTestHandle,
  FheValueType,
  InputProofResult,
  PublicDecryptResult,
} from "./types";
import { FHE_TYPE_IDS, FHE_VALUE_TYPES } from "./types";
import {
  createFreshDecryptValues,
  createInitValue,
  createRandomValue,
} from "./values";

type FhevmClientLike = ReturnType<typeof createClients>["fhevm"];
type PublicClientLike = ReturnType<typeof createClients>["publicClient"];
export type ProgressReporter = (message: string) => void;

const zeroHandle =
  "0x0000000000000000000000000000000000000000000000000000000000000000";

const setEncryptedFunctionByType = {
  bool: "setEbool",
  uint8: "setEuint8",
  uint16: "setEuint16",
  uint32: "setEuint32",
  uint64: "setEuint64",
  uint128: "setEuint128",
  uint256: "setEuint256",
  address: "setEaddress",
} as const satisfies Record<FheValueType, string>;

const setClearFunctionByType = {
  bool: "setClearEbool",
  uint8: "setClearEuint8",
  uint16: "setClearEuint16",
  uint32: "setClearEuint32",
  uint64: "setClearEuint64",
  uint128: "setClearEuint128",
  uint256: "setClearEuint256",
  address: "setClearEaddress",
} as const satisfies Record<FheValueType, string>;

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
  const { fhevm } = createClients(options);
  const contractAddress = resolveContractAddress(options);
  const userAddress =
    options.userAddress ?? "0x0000000000000000000000000000000000000002";
  const valueType = options.type ?? "bool";
  const values =
    options.values ??
    (options.value === undefined
      ? [createRandomValue(valueType)]
      : [{ type: valueType, value: options.value }]);

  options.onProgress?.(
    `Encrypting ${values.length.toString()} ${valueType} value(s) and requesting verified input proof`,
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
    type: FheValueType;
    contractAddress?: Hex;
    account?: Hex;
    handles?: readonly Hex[];
    privateKey?: Hex;
    mnemonic?: string;
    onProgress?: ProgressReporter;
  }>;

export const publicDecrypt = async (
  options: PublicDecryptOptions,
): Promise<PublicDecryptResult & { handles?: readonly FheTestHandle[] }> => {
  options.onProgress?.("Creating FHEVM client");
  const { fhevm, publicClient } = createClients(options);

  if (options.handles && options.handles.length > 0) {
    options.onProgress?.(
      `Using ${options.handles.length.toString()} provided handle(s)`,
    );
    return readPublicValues(fhevm, options.handles, options.onProgress);
  }

  const account = resolveAccountAddress(options);
  const contractAddress = resolveContractAddress(options);
  const handle = await readFheTestHandle({
    publicClient,
    contractAddress,
    account,
    type: options.type,
    onProgress: options.onProgress,
  });
  const decrypted = await readPublicValues(
    fhevm,
    [handle.handle],
    options.onProgress,
  );
  return { ...decrypted, handles: [handle] };
};

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
  const { account, fhevm, publicClient, walletClient } = createWallet(options);
  const contractAddress = resolveContractAddress(options);
  const values =
    options.value === undefined
      ? createFreshDecryptValues(options.type)
      : [{ type: options.type, value: options.value }];
  const value = values[0];
  if (!value) throw new Error("No value to encrypt.");

  options.onProgress?.(`Encrypting ${options.type} value`);
  const encrypted = await fhevm.encryptValues({
    contractAddress,
    userAddress: account.address,
    values,
  });

  const encryptedValue = encrypted.encryptedValues[0] as Hex | undefined;
  if (!encryptedValue) throw new Error("FHEVM SDK did not return a handle.");

  options.onProgress?.(`Simulating FHETest.${setEncryptedFunctionByType[options.type]}`);
  const { request } = await publicClient.simulateContract({
    account,
    address: contractAddress,
    abi: fheTestAbi,
    functionName: setEncryptedFunctionByType[options.type],
    args: [encryptedValue, encrypted.inputProof as Hex, value.value, true],
  } as never);

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
    inputProof: encrypted.inputProof as Hex,
    inputValues: values,
    handle,
  };
};

export type MakePublicOptions = ClientOptions &
  Readonly<{
    type: FheValueType;
    contractAddress?: Hex;
    privateKey?: Hex;
    mnemonic?: string;
    onProgress?: ProgressReporter;
  }>;

export const makePublicAndDecrypt = async (
  options: MakePublicOptions,
): Promise<
  PublicDecryptResult & {
    transactionHash: Hex;
    handle: FheTestHandle;
  }
> => {
  options.onProgress?.("Loading wallet and creating clients");
  const { account, fhevm, publicClient, walletClient } = createWallet(options);
  const contractAddress = resolveContractAddress(options);

  options.onProgress?.(`Simulating FHETest.makePubliclyDecryptable for ${options.type}`);
  const { request } = await publicClient.simulateContract({
    account,
    address: contractAddress,
    abi: fheTestAbi,
    functionName: "makePubliclyDecryptable",
    args: [FHE_TYPE_IDS[options.type]],
  } as never);

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

  return { ...decrypted, transactionHash, handle };
};

export type InitFheTestOptions = ClientOptions &
  Readonly<{
    contractAddress?: Hex;
    type?: FheValueType;
    force?: boolean;
    privateKey?: Hex;
    mnemonic?: string;
    onProgress?: ProgressReporter;
  }>;

export const initFheTest = async (
  options: InitFheTestOptions,
): Promise<{
  contractAddress: Hex;
  account: Hex;
  initialized: readonly FheTestHandle[];
  skipped: readonly FheTestHandle[];
}> => {
  options.onProgress?.("Loading wallet and creating clients");
  const { account, publicClient, walletClient } = createWallet(options);
  const contractAddress = resolveContractAddress(options);
  const types = options.type ? [options.type] : FHE_VALUE_TYPES;
  const initialized: FheTestHandle[] = [];
  const skipped: FheTestHandle[] = [];

  for (const valueType of types) {
    options.onProgress?.(`Checking existing ${valueType} handle`);
    const hasHandle = await hasFheTestHandle({
      publicClient,
      contractAddress,
      account: account.address,
      type: valueType,
    });

    if (hasHandle && !options.force) {
      skipped.push(
        await readFheTestHandle({
          publicClient,
          contractAddress,
          account: account.address,
          type: valueType,
          onProgress: options.onProgress,
        }),
      );
      continue;
    }

    const value = createInitValue(valueType);
    options.onProgress?.(`Simulating FHETest.${setClearFunctionByType[valueType]}`);
    const { request } = await publicClient.simulateContract({
      account,
      address: contractAddress,
      abi: fheTestAbi,
      functionName: setClearFunctionByType[valueType],
      args: [value.value, true],
    } as never);

    await sendAndWait({
      walletClient,
      publicClient,
      request,
      onProgress: options.onProgress,
    });
    initialized.push(
      await readFheTestHandle({
        publicClient,
        contractAddress,
        account: account.address,
        type: valueType,
        onProgress: options.onProgress,
      }),
    );
  }

  return {
    contractAddress,
    account: account.address,
    initialized,
    skipped,
  };
};

const resolveAccountAddress = (
  options: Readonly<{ account?: Hex; privateKey?: Hex; mnemonic?: string }>,
): Hex => {
  if (options.account) return options.account;
  return loadAccount(options.privateKey, options.mnemonic).address;
};

const hasFheTestHandle = async (options: {
  publicClient: PublicClientLike;
  contractAddress: Hex;
  account: Hex;
  type: FheValueType;
}): Promise<boolean> =>
  (await options.publicClient.readContract({
    address: options.contractAddress,
    abi: fheTestAbi,
    functionName: "hasHandleOf",
    args: [options.account, FHE_TYPE_IDS[options.type]],
  } as never)) as boolean;

const readFheTestHandle = async (options: {
  publicClient: PublicClientLike;
  contractAddress: Hex;
  account: Hex;
  type: FheValueType;
  onProgress?: ProgressReporter;
}): Promise<FheTestHandle> => {
  options.onProgress?.(
    `Reading FHETest handle for ${options.account} / ${options.type}`,
  );
  const hasHandle = await hasFheTestHandle(options);
  if (!hasHandle) {
    throw new Error(
      `No FHETest handle for account ${options.account} and type ${options.type}. Run "fhe-test init", "public-decrypt fresh", or pass --handle.`,
    );
  }

  const handle = (await options.publicClient.readContract({
    address: options.contractAddress,
    abi: fheTestAbi,
    functionName: "getHandleOf",
    args: [options.account, FHE_TYPE_IDS[options.type]],
  } as never)) as Hex;
  if (handle === zeroHandle) {
    throw new Error(`FHETest returned an empty ${options.type} handle.`);
  }

  const clearText = (await options.publicClient.readContract({
    address: options.contractAddress,
    abi: fheTestAbi,
    functionName: "getClearText",
    args: [handle],
  } as never)) as bigint;

  return {
    type: options.type,
    fheTypeId: FHE_TYPE_IDS[options.type],
    account: options.account,
    handle,
    clearText: clearText.toString(),
  };
};

const sendAndWait = async (options: {
  walletClient: ReturnType<typeof createWallet>["walletClient"];
  publicClient: PublicClientLike;
  request: unknown;
  onProgress?: ProgressReporter;
}): Promise<Hex> => {
  options.onProgress?.("Sending transaction");
  const transactionHash = await options.walletClient.writeContract(
    options.request as never,
  );
  options.onProgress?.(`Waiting for transaction receipt: ${transactionHash}`);
  const receipt = await options.publicClient.waitForTransactionReceipt({
    hash: transactionHash,
  });
  if (receipt.status !== "success") {
    throw new Error(`Transaction reverted: ${transactionHash}`);
  }
  return transactionHash;
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
