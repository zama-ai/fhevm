import type { Hex } from "viem";
import type { Account } from "viem/accounts";

import { loadNamedAccount } from "../../config";
import type { DelegatedUserDecryptBaseOptions } from "./types";

const DELEGATOR_PRIVATE_KEY_ENV = "DELEGATOR_PRIVATE_KEY";
const DELEGATOR_MNEMONIC_ENV = "DELEGATOR_MNEMONIC";

export const loadOptionalDelegatorAccount = (
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

export const loadRequiredDelegatorAccount = (
  options: DelegatedUserDecryptBaseOptions,
): Account =>
  loadNamedAccount({
    privateKey: options.delegatorPrivateKey,
    mnemonic: options.delegatorMnemonic,
    privateKeyEnv: DELEGATOR_PRIVATE_KEY_ENV,
    mnemonicEnv: DELEGATOR_MNEMONIC_ENV,
    label: "delegator",
  });

export const resolveDelegatorAddress = (
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
