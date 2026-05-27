import type { Hex } from "viem";
import type { Account } from "viem/accounts";

import {
  createWalletContext,
  createWalletContextForAccount,
  loadNamedAccount,
  type ClientOptions,
  type WalletContext,
} from "../config";
import {
  ensureUserDecryptionDelegation,
  type DelegationStatus,
} from "../acl/delegation";
import { readFheTestHandle } from "../fhe-test/handles";
import {
  getSetEncryptedFunctionName,
  simulateSetEncryptedValue,
} from "../fhe-test/writes";
import { encryptValues } from "../fhevm/encryption";
import { decryptUserValues } from "../fhevm/user-decrypt";
import type { ProgressReporter } from "../shared/progress";
import { sendAndWait } from "../shared/transactions";
import type {
  EncryptValue,
  FheClearValue,
  FheTestHandle,
  FheValueType,
  UserDecryptResult,
} from "../types";
import { createFreshDecryptValues } from "../values";

const DELEGATOR_PRIVATE_KEY_ENV = "DELEGATOR_PRIVATE_KEY";
const DELEGATOR_MNEMONIC_ENV = "DELEGATOR_MNEMONIC";

type DelegatedUserDecryptBaseOptions = ClientOptions &
  Readonly<{
    type: FheValueType;
    contractAddress?: Hex;
    delegatorAddress?: Hex;
    delegatorPrivateKey?: Hex;
    delegatorMnemonic?: string;
    privateKey?: Hex;
    mnemonic?: string;
    durationDays: number;
    delegationDurationDays: number;
    onProgress?: ProgressReporter;
  }>;

export type DelegatedUserDecryptOptions = DelegatedUserDecryptBaseOptions &
  Readonly<{
    handles?: readonly Hex[];
  }>;

export type FreshDelegatedUserDecryptOptions = DelegatedUserDecryptBaseOptions &
  Readonly<{
    value?: FheClearValue;
  }>;

export type DelegatedUserDecryptResult = UserDecryptResult &
  Readonly<{
    delegatorAddress: Hex;
    delegateAddress: Hex;
    delegation: DelegationStatus;
  }>;

export type FreshDelegatedUserDecryptResult = DelegatedUserDecryptResult &
  Readonly<{
    transactionHash: Hex;
    inputProof: Hex;
    inputValues: readonly EncryptValue[];
    handle: FheTestHandle;
  }>;

export const delegatedUserDecrypt = async (
  options: DelegatedUserDecryptOptions,
): Promise<DelegatedUserDecryptResult & { handles?: readonly FheTestHandle[] }> => {
  options.onProgress?.("Loading delegate wallet and creating clients");
  const delegateContext = createWalletContext(options);
  const delegatorAccount = loadOptionalDelegatorAccount(options);
  const delegatorAddress = resolveDelegatorAddress(options, delegatorAccount);
  const delegatorContext = delegatorAccount
    ? createWalletContextForAccount(options, delegatorAccount)
    : undefined;
  validateDelegatedActors(delegatorAddress, delegateContext.account.address);

  const delegation = await ensureUserDecryptionDelegation(delegateContext, {
    delegatorContext,
    delegatorAddress,
    delegateAddress: delegateContext.account.address,
    durationDays: options.delegationDurationDays,
    onProgress: options.onProgress,
  });

  if (options.handles && options.handles.length > 0) {
    options.onProgress?.(
      `Using ${options.handles.length.toString()} provided handle(s)`,
    );
    const decrypted = await decryptDelegatedHandles(delegateContext, {
      encryptedValues: options.handles,
      delegatorAddress,
      durationDays: options.durationDays,
      onProgress: options.onProgress,
    });
    return {
      ...decrypted,
      delegatorAddress,
      delegateAddress: delegateContext.account.address,
      delegation,
    };
  }

  const handle = await readFheTestHandle({
    publicClient: delegateContext.publicClient,
    contractAddress: delegateContext.contractAddress,
    account: delegatorAddress,
    type: options.type,
    onProgress: options.onProgress,
  });
  const decrypted = await decryptDelegatedHandles(delegateContext, {
    encryptedValues: [handle.handle],
    delegatorAddress,
    durationDays: options.durationDays,
    onProgress: options.onProgress,
  });

  return {
    ...decrypted,
    handles: [handle],
    delegatorAddress,
    delegateAddress: delegateContext.account.address,
    delegation,
  };
};

export const freshDelegatedUserDecrypt = async (
  options: FreshDelegatedUserDecryptOptions,
): Promise<FreshDelegatedUserDecryptResult> => {
  options.onProgress?.("Loading delegate and delegator wallets");
  const delegateContext = createWalletContext(options);
  const delegatorAccount = loadRequiredDelegatorAccount(options);
  const delegatorContext = createWalletContextForAccount(options, delegatorAccount);
  const delegatorAddress = resolveDelegatorAddress(options, delegatorAccount);
  validateDelegatedActors(delegatorAddress, delegateContext.account.address);

  const values =
    options.value === undefined
      ? createFreshDecryptValues(options.type)
      : [{ type: options.type, value: options.value }];
  const value = values[0];
  if (!value) throw new Error("No value to encrypt.");

  const encrypted = await encryptValues(delegatorContext.fhevm, {
    contractAddress: delegatorContext.contractAddress,
    userAddress: delegatorAddress,
    values,
    onProgress: options.onProgress,
    progressLabel: `Encrypting ${options.type} value for delegator`,
  });
  const encryptedValue = encrypted.encryptedValues[0];
  if (!encryptedValue) throw new Error("FHEVM SDK did not return a handle.");

  options.onProgress?.(
    `Simulating FHETest.${getSetEncryptedFunctionName(options.type)}`,
  );
  const request = await simulateSetEncryptedValue(
    {
      account: delegatorContext.account,
      contractAddress: delegatorContext.contractAddress,
      publicClient: delegatorContext.publicClient,
    },
    {
      encryptedValue,
      inputProof: encrypted.inputProof,
      value,
      makePublic: false,
    },
  );
  const transactionHash = await sendAndWait({
    walletClient: delegatorContext.walletClient,
    publicClient: delegatorContext.publicClient,
    request,
    onProgress: options.onProgress,
  });

  const delegation = await ensureUserDecryptionDelegation(delegateContext, {
    delegatorContext,
    delegatorAddress,
    delegateAddress: delegateContext.account.address,
    durationDays: options.delegationDurationDays,
    onProgress: options.onProgress,
  });

  const handle = await readFheTestHandle({
    publicClient: delegateContext.publicClient,
    contractAddress: delegateContext.contractAddress,
    account: delegatorAddress,
    type: options.type,
    onProgress: options.onProgress,
  });
  const decrypted = await decryptDelegatedHandles(delegateContext, {
    encryptedValues: [handle.handle],
    delegatorAddress,
    durationDays: options.durationDays,
    onProgress: options.onProgress,
  });

  return {
    ...decrypted,
    transactionHash,
    inputProof: encrypted.inputProof,
    inputValues: values,
    handle,
    delegatorAddress,
    delegateAddress: delegateContext.account.address,
    delegation,
  };
};

const decryptDelegatedHandles = (
  delegateContext: WalletContext,
  options: {
    encryptedValues: readonly Hex[];
    delegatorAddress: Hex;
    durationDays: number;
    onProgress?: ProgressReporter;
  },
): Promise<UserDecryptResult> =>
  decryptUserValues(delegateContext, {
    encryptedValues: options.encryptedValues,
    signer: delegateContext.account,
    ownerAddress: options.delegatorAddress,
    durationDays: options.durationDays,
    onProgress: options.onProgress,
  });

const loadOptionalDelegatorAccount = (
  options: DelegatedUserDecryptBaseOptions,
): Account | undefined => {
  if (
    !options.delegatorPrivateKey &&
    !options.delegatorMnemonic &&
    !process.env[DELEGATOR_PRIVATE_KEY_ENV] &&
    !process.env[DELEGATOR_MNEMONIC_ENV]
  ) {
    return undefined;
  }

  return loadRequiredDelegatorAccount(options);
};

const loadRequiredDelegatorAccount = (
  options: DelegatedUserDecryptBaseOptions,
): Account =>
  loadNamedAccount({
    privateKey: options.delegatorPrivateKey,
    mnemonic: options.delegatorMnemonic,
    privateKeyEnv: DELEGATOR_PRIVATE_KEY_ENV,
    mnemonicEnv: DELEGATOR_MNEMONIC_ENV,
    label: "delegator",
  });

const resolveDelegatorAddress = (
  options: DelegatedUserDecryptBaseOptions,
  account?: Account,
): Hex => {
  if (options.delegatorAddress && account) {
    if (
      options.delegatorAddress.toLowerCase() !== account.address.toLowerCase()
    ) {
      throw new Error(
        `Delegator address ${options.delegatorAddress} does not match delegator credentials ${account.address}.`,
      );
    }
  }

  if (options.delegatorAddress) return options.delegatorAddress;
  if (account) return account.address;

  throw new Error(
    "Provide --delegator or delegator credentials for delegated user decrypt.",
  );
};

const validateDelegatedActors = (
  delegatorAddress: Hex,
  delegateAddress: Hex,
): void => {
  if (delegatorAddress.toLowerCase() === delegateAddress.toLowerCase()) {
    throw new Error(
      "Delegator and delegate must be different. Use user-decrypt for self decryption.",
    );
  }
};
