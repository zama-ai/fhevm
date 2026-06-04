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

type DelegatedValueType = DelegatedUserDecryptBaseOptions["type"];

const DEFAULT_CACHED_TYPES: readonly DelegatedValueType[] = ["bool"];

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
  const handles = options.handles ?? [];
  const types = options.types ?? [];
  if (handles.length > 0 && types.length > 0) {
    throw new Error("Use either --handle or --type for cached decrypt, not both.");
  }

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
    options.onProgress?.(`Using ${handles.length.toString()} provided handle(s)`);
    options.onProgress?.(`Provided handle(s): ${handles.join(", ")}`);
    const decrypted = await decryptDelegatedHandles(delegateContext, {
      encryptedValues: handles,
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

  const selectedTypes = types.length > 0 ? types : DEFAULT_CACHED_TYPES;
  const storedHandles: FheTestHandle[] = [];
  for (const type of selectedTypes) {
    const handle = await readFheTestHandle({
      publicClient: delegateContext.publicClient,
      contractAddress: delegateContext.contractAddress,
      account: delegatorAddress,
      type,
      onProgress: options.onProgress,
    });
    options.onProgress?.(
      `Using stored delegator ${type} handle: ${describeHandle(handle)}`,
    );
    storedHandles.push(handle);
  }
  const decrypted = await decryptDelegatedHandles(delegateContext, {
    encryptedValues: storedHandles.map((handle) => handle.handle),
    delegatorAddress,
    durationDays: options.durationDays,
    onProgress: options.onProgress,
  });

  return {
    ...decrypted,
    handles: storedHandles,
    delegatorAddress,
    delegateAddress: delegateContext.account.address,
    delegation,
  };
};
