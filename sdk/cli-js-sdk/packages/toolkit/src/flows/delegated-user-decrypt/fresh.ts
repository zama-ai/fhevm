import type { Hex } from "viem";

import { ensureUserDecryptionDelegation } from "../../acl/delegation";
import {
  createWalletContext,
  createWalletContextForAccount,
} from "../../config";
import type {
  EncryptValue,
  FheClearValue,
  FheTestHandle,
} from "../../types";
import { createStoredFheTestHandle } from "../handles";
import { describeHandle } from "../progress";
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

/** Result for a fresh delegated flow, including the write that created the handle. */
export type FreshDelegatedUserDecryptResult = DelegatedUserDecryptResult &
  Readonly<{
    transactionHash: Hex;
    inputProof: Hex;
    inputValues: readonly EncryptValue[];
    handle: FheTestHandle;
  }>;

/**
 * Creates a new private handle owned by the delegator, grants delegate ACL
 * access if needed, then decrypts the handle with a delegated permit.
 */
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
  options.onProgress?.(`Delegate address: ${delegateContext.account.address}`);
  options.onProgress?.(`Delegator address: ${delegatorAddress}`);

  const { transactionHash, inputProof, inputValues, handle } =
    await createStoredFheTestHandle(delegatorContext, {
      type: options.type,
      ownerAddress: delegatorAddress,
      value: options.value,
      makePublic: false,
      inputProgressLabel: "Delegator input value",
      encryptProgressLabel: `Encrypting ${options.type} value for delegator`,
      encryptedProgressLabel: "Encrypted delegator input handle",
      storedProgressLabel: (storedHandle) =>
        `Stored delegator ${options.type} handle: ${describeHandle(storedHandle)}`,
      onProgress: options.onProgress,
    });

  const delegation = await ensureUserDecryptionDelegation(delegateContext, {
    delegatorContext,
    delegatorAddress,
    delegateAddress: delegateContext.account.address,
    durationDays: options.delegationDurationDays,
    onProgress: options.onProgress,
  });

  const decrypted = await decryptDelegatedHandles(delegateContext, {
    encryptedValues: [handle.handle],
    delegatorAddress,
    durationDays: options.durationDays,
    network: options.network,
    includeValidationArtifact: options.includeValidationArtifact,
    onProgress: options.onProgress,
  });

  return {
    ...decrypted,
    transactionHash,
    inputProof,
    inputValues,
    handle,
    delegatorAddress,
    delegateAddress: delegateContext.account.address,
    delegation,
    validationArtifact: decrypted.validationArtifact
      ? {
          ...decrypted.validationArtifact,
          expectedClearValues: inputValues.map((value) => ({
            type: value.type,
            value: String(value.value),
          })),
        }
      : undefined,
  };
};
