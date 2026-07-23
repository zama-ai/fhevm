import type { Hex } from "viem";
import {
  mnemonicToAccount,
  privateKeyToAccount,
  type Account,
} from "viem/accounts";

/**
 * Loads the default wallet account from explicit options, then environment.
 *
 * Explicit CLI credentials take precedence over environment fallback.
 */
export const loadAccount = (privateKey?: Hex, mnemonic?: string): Account => {
  return loadNamedAccount({
    privateKey,
    mnemonic,
    privateKeyEnv: "PRIVATE_KEY",
    mnemonicEnv: "MNEMONIC",
    label: "wallet",
  });
};

/**
 * Loads a named account using caller-provided env variable names for fallback.
 *
 * Used to keep delegate (`PRIVATE_KEY`/`MNEMONIC`) and delegator
 * (`DELEGATOR_PRIVATE_KEY`/`DELEGATOR_MNEMONIC`) credentials separate.
 */
export const loadNamedAccount = (options: {
  privateKey?: Hex;
  mnemonic?: string;
  privateKeyEnv: string;
  mnemonicEnv: string;
  label: string;
}): Account => {
  const resolvedPrivateKey = process.env[options.privateKeyEnv] as Hex | undefined;
  const resolvedMnemonic = process.env[options.mnemonicEnv];

  if (options.mnemonic) return mnemonicToAccount(options.mnemonic);
  if (options.privateKey) return privateKeyToAccount(options.privateKey);
  if (resolvedPrivateKey) return privateKeyToAccount(resolvedPrivateKey);
  if (resolvedMnemonic) return mnemonicToAccount(resolvedMnemonic);

  throw new Error(
    `Provide ${options.label} private key, ${options.label} mnemonic, ${options.privateKeyEnv}, or ${options.mnemonicEnv}.`,
  );
};
