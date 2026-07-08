import { ensureUserDecryptionDelegation } from "../../acl/delegation";
import {
  createWalletContext,
  createWalletContextForAccount,
} from "../../config";
import type { FheTestHandle } from "../../types";
import { readStoredFheTestHandles } from "../handles";
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

/**
 * Options for decrypting FHETest handles stored in delegator/type slots.
 *
 * The flow reads FHETest handles for the delegator and each selected type,
 * defaulting to the bool slot when no `types` are given.
 */
export type StoredDelegatedUserDecryptOptions = Omit<
  DelegatedUserDecryptBaseOptions,
  "type"
> &
  Readonly<{
    types?: readonly DelegatedValueType[];
  }>;

/** Decrypts FHETest handles stored for the delegator and selected types. */
export const storedDelegatedUserDecrypt = async (
  options: StoredDelegatedUserDecryptOptions,
): Promise<
  DelegatedUserDecryptResult & { handles: readonly FheTestHandle[] }
> => {
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

  const storedHandles = await readStoredFheTestHandles(delegateContext, {
    account: delegatorAddress,
    types: options.types ?? [],
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
