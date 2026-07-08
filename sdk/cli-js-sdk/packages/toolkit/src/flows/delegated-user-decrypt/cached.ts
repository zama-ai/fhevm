import type { Hex } from "viem";

import { ensureUserDecryptionDelegation } from "../../acl/delegation";
import {
  createWalletContext,
  createWalletContextForAccount,
} from "../../config";
import type { FheTestHandle } from "../../types";
import { describeHandle } from "../progress";
import {
  readCachedFheTestHandles,
  reportProvidedHandles,
  resolveCachedDecryptSelection,
} from "../handles";
import {
  loadOptionalDelegatorAccount,
  resolveDelegatorAddress,
  validateDistinctDelegatedAccounts,
} from "./account";
import { decryptDelegatedHandles } from "./decrypt";
import type {
  DelegatedUserDecryptBaseOptions,
  DelegatedUserDecryptResult,
} from "./types";

type DelegatedValueType = DelegatedUserDecryptBaseOptions["type"];

export type DelegatedUserDecryptOptions = Omit<
  DelegatedUserDecryptBaseOptions,
  "type"
> &
  Readonly<{
    types?: readonly DelegatedValueType[];
    handles?: readonly Hex[];
  }>;

/**
 * Decrypts existing handles as a delegate.
 *
 * The flow ensures the delegate has ACL permission from the delegator before it
 * signs the delegated user decryption permit.
 */
export const delegatedUserDecrypt = async (
  options: DelegatedUserDecryptOptions,
): Promise<DelegatedUserDecryptResult & { handles?: readonly FheTestHandle[] }> => {
  const { handles, types } = resolveCachedDecryptSelection(options);

  options.onProgress?.("Loading delegate wallet and creating clients");
  const delegateContext = createWalletContext(options);
  const delegatorAccount = loadOptionalDelegatorAccount(options);
  const delegatorAddress = resolveDelegatorAddress(options, delegatorAccount);
  const delegatorContext = delegatorAccount
    ? createWalletContextForAccount(options, delegatorAccount)
    : undefined;
  validateDistinctDelegatedAccounts(
    delegatorAddress,
    delegateContext.account.address,
  );
  options.onProgress?.(`Delegate address: ${delegateContext.account.address}`);
  options.onProgress?.(`Delegator address: ${delegatorAddress}`);

  const delegation = await ensureUserDecryptionDelegation(delegateContext, {
    delegatorContext,
    delegatorAddress,
    delegateAddress: delegateContext.account.address,
    durationDays: options.delegationDurationDays,
    onProgress: options.onProgress,
  });

  if (handles.length > 0) {
    reportProvidedHandles(handles, options.onProgress);
    const decrypted = await decryptDelegatedHandles(delegateContext, {
      encryptedValues: handles,
      delegatorAddress,
      durationDays: options.durationDays,
      network: options.network,
      includeValidationArtifact: options.includeValidationArtifact,
      onProgress: options.onProgress,
    });
    return {
      ...decrypted,
      delegatorAddress,
      delegateAddress: delegateContext.account.address,
      delegation,
    };
  }

  const storedHandles = await readCachedFheTestHandles(delegateContext, {
    account: delegatorAddress,
    types,
    describeStoredHandle: (type, handle) =>
      `Using stored delegator ${type} handle: ${describeHandle(handle)}`,
    onProgress: options.onProgress,
  });
  const decrypted = await decryptDelegatedHandles(delegateContext, {
    encryptedValues: storedHandles.map((handle) => handle.handle),
    delegatorAddress,
    durationDays: options.durationDays,
    network: options.network,
    includeValidationArtifact: options.includeValidationArtifact,
    onProgress: options.onProgress,
  });

  return {
    ...decrypted,
    handles: storedHandles,
    delegatorAddress,
    delegateAddress: delegateContext.account.address,
    delegation,
    validationArtifact: decrypted.validationArtifact
      ? {
          ...decrypted.validationArtifact,
          expectedClearValues: storedHandles
            .filter((handle) => handle.clearText !== undefined)
            .map((handle) => ({
              type: handle.type,
              value: handle.clearText as string,
            })),
        }
      : undefined,
  };
};
