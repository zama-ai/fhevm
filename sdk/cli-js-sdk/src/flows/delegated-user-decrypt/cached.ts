import type { Hex } from "viem";

import { ensureUserDecryptionDelegation } from "../../acl/delegation";
import {
  createWalletContext,
  createWalletContextForAccount,
} from "../../config";
import { readFheTestHandle } from "../../fhe-test/handles";
import type { FheTestHandle } from "../../types";
import { describeHandle } from "../progress";
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

export type DelegatedUserDecryptOptions = DelegatedUserDecryptBaseOptions &
  Readonly<{
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

  if (options.handles && options.handles.length > 0) {
    options.onProgress?.(
      `Using ${options.handles.length.toString()} provided handle(s)`,
    );
    options.onProgress?.(`Provided handle(s): ${options.handles.join(", ")}`);
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
  options.onProgress?.(
    `Using stored delegator ${options.type} handle: ${describeHandle(handle)}`,
  );
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
