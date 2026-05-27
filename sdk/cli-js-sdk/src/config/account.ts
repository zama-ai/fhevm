import type { Hex } from "viem";
import {
  mnemonicToAccount,
  privateKeyToAccount,
  type Account,
} from "viem/accounts";

export const loadAccount = (privateKey?: Hex, mnemonic?: string): Account => {
  const resolvedMnemonic = mnemonic ?? process.env.MNEMONIC;
  const resolvedPrivateKey =
    privateKey ?? (process.env.PRIVATE_KEY as Hex | undefined);

  if (resolvedMnemonic) return mnemonicToAccount(resolvedMnemonic);
  if (resolvedPrivateKey) return privateKeyToAccount(resolvedPrivateKey);

  throw new Error(
    "Provide --private-key, --mnemonic, PRIVATE_KEY, or MNEMONIC.",
  );
};
