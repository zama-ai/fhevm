import type { Hex } from "viem";

import { ensureUserDecryptionDelegation } from "../../acl/delegation";
import {
  createWalletContext,
  createWalletContextForAccount,
} from "../../config";
import { readFheTestHandle } from "../../fhe-test/handles";
import {
  getSetEncryptedFunctionName,
  simulateSetEncryptedValue,
} from "../../fhe-test/writes";
import { encryptValues } from "../../fhevm/encryption";
import { sendAndWait } from "../../shared/transactions";
import type {
  EncryptValue,
  FheClearValue,
  FheTestHandle,
} from "../../types";
import { createFreshDecryptValues } from "../../values";
import {
  loadRequiredDelegatorAccount,
  resolveDelegatorAddress,
  validateDistinctDelegatedAccounts,
} from "./account";
import { decryptDelegatedHandles } from "./decrypt";
import type {
  DelegatedUserDecryptBaseOptions,
  DelegatedUserDecryptResult,
} from "./types";

export type FreshDelegatedUserDecryptOptions = DelegatedUserDecryptBaseOptions &
  Readonly<{
    value?: FheClearValue;
  }>;

export type FreshDelegatedUserDecryptResult = DelegatedUserDecryptResult &
  Readonly<{
    transactionHash: Hex;
    inputProof: Hex;
    inputValues: readonly EncryptValue[];
    handle: FheTestHandle;
  }>;

export const freshDelegatedUserDecrypt = async (
  options: FreshDelegatedUserDecryptOptions,
): Promise<FreshDelegatedUserDecryptResult> => {
  options.onProgress?.("Loading delegate and delegator wallets");
  const delegateContext = createWalletContext(options);
  const delegatorAccount = loadRequiredDelegatorAccount(options);
  const delegatorContext = createWalletContextForAccount(options, delegatorAccount);
  const delegatorAddress = resolveDelegatorAddress(options, delegatorAccount);
  validateDistinctDelegatedAccounts(
    delegatorAddress,
    delegateContext.account.address,
  );

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
