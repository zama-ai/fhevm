import type { Hex } from "viem";

import { ensureUserDecryptionDelegation } from "../../acl/delegation";
import {
  createWalletContext,
  createWalletContextForAccount,
} from "../../config";
import { reportProvidedHandles } from "../handles";
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

/**
 * Options for decrypting existing handles as a delegate.
 *
 * The supplied `handles` are decrypted as-is on behalf of the delegator.
 * `contractAddress` is the ACL pairing contract and defaults to FHETest.
 */
export type DelegatedUserDecryptOptions = Omit<
  DelegatedUserDecryptBaseOptions,
  "type"
> &
  Readonly<{
    handles: readonly Hex[];
  }>;

/**
 * Decrypts existing handles as a delegate.
 *
 * The flow ensures the delegate has ACL permission from the delegator before it
 * signs the delegated user decryption permit.
 */
export const delegatedUserDecrypt = async (
  options: DelegatedUserDecryptOptions,
): Promise<DelegatedUserDecryptResult> => {
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

  reportProvidedHandles(options.handles, options.onProgress);
  const decrypted = await decryptDelegatedHandles(delegateContext, {
    encryptedValues: options.handles,
    delegatorAddress,
    durationSeconds: options.permitDurationSeconds,
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
};
