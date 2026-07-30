import type { Hex } from "viem";
import type { Account } from "viem/accounts";

import { loadNamedAccount } from "../../config";
import type { DelegatedUserDecryptBaseOptions } from "./types";

const DELEGATOR_PRIVATE_KEY_ENV = "DELEGATOR_PRIVATE_KEY";
const DELEGATOR_MNEMONIC_ENV = "DELEGATOR_MNEMONIC";

type DelegatedAccountOptions = Omit<DelegatedUserDecryptBaseOptions, "type">;

/** Loads delegator credentials only when explicitly supplied by flags or env. */
export const loadOptionalDelegatorAccount = (
  options: DelegatedAccountOptions,
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

/** Loads the encrypted data owner's credentials for delegated writes or grants. */
export const loadRequiredDelegatorAccount = (
  options: DelegatedAccountOptions,
): Account =>
  loadNamedAccount({
    privateKey: options.delegatorPrivateKey,
    mnemonic: options.delegatorMnemonic,
    privateKeyEnv: DELEGATOR_PRIVATE_KEY_ENV,
    mnemonicEnv: DELEGATOR_MNEMONIC_ENV,
    label: "delegator",
  });

/**
 * Resolves and cross-checks the encrypted data owner address.
 *
 * When both an address and credentials are present, they must describe the same
 * account to avoid granting/decrypting on behalf of the wrong owner.
 */
export const resolveDelegatorAddress = (
  options: DelegatedAccountOptions,
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

/** Rejects self-delegation; the regular user-decrypt path handles that case. */
export const validateDistinctDelegatedAccounts = (
  delegatorAddress: Hex,
  delegateAddress: Hex,
): void => {
  if (delegatorAddress.toLowerCase() === delegateAddress.toLowerCase()) {
    throw new Error(
      "Delegator and delegate must be different. Use user-decrypt for self decryption.",
    );
  }
};
