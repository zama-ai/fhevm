import type { Hex } from "viem";
import {
  mnemonicToAccount,
  privateKeyToAccount,
  type Account,
} from "viem/accounts";

export const loadAccount = (privateKey?: Hex, mnemonic?: string): Account => {
  return loadNamedAccount({
    privateKey,
    mnemonic,
    privateKeyEnv: "PRIVATE_KEY",
    mnemonicEnv: "MNEMONIC",
    label: "wallet",
  });
};

export const loadNamedAccount = (options: {
  privateKey?: Hex;
  mnemonic?: string;
  privateKeyEnv: string;
  mnemonicEnv: string;
  label: string;
}): Account => {
  const resolvedMnemonic = options.mnemonic ?? process.env[options.mnemonicEnv];
  const resolvedPrivateKey =
    options.privateKey ?? (process.env[options.privateKeyEnv] as Hex | undefined);

  if (resolvedMnemonic) return mnemonicToAccount(resolvedMnemonic);
  if (resolvedPrivateKey) return privateKeyToAccount(resolvedPrivateKey);

  throw new Error(
    `Provide ${options.label} private key, ${options.label} mnemonic, ${options.privateKeyEnv}, or ${options.mnemonicEnv}.`,
  );
};
