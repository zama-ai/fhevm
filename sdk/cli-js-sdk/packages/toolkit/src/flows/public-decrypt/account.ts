import type { Hex } from "viem";

import { loadAccount } from "../../config";

export const resolveAccountAddress = (
  options: Readonly<{ account?: Hex; privateKey?: Hex; mnemonic?: string }>,
): Hex => {
  if (options.account) return options.account;
  return loadAccount(options.privateKey, options.mnemonic).address;
};
