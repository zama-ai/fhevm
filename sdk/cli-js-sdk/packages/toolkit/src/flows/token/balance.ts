import type { Hex } from "viem";

import { createClientContext, loadAccount, type ClientOptions } from "../../config";
import type { ProgressReporter } from "../../shared/progress";
import {
  readConfidentialBalance,
  readTokenMetadata,
  type TokenMetadata,
} from "../../token/reads";

/**
 * Options for reading an ERC-7984 confidential balance.
 *
 * `contractAddress` is required. `account` defaults to the loaded wallet
 * address; when it is supplied the read runs without wallet credentials.
 */
export type BalanceOfTokenOptions = ClientOptions &
  Readonly<{
    contractAddress: Hex;
    account?: Hex;
    privateKey?: Hex;
    mnemonic?: string;
    onProgress?: ProgressReporter;
  }>;

/** Result of a confidential balance read, including the balance handle. */
export type BalanceOfTokenResult = Readonly<{
  network: ClientOptions["network"];
  contractAddress: Hex;
  account: Hex;
  balanceHandle: Hex;
  tokenMetadata: TokenMetadata;
}>;

/** Reads the confidential balance handle for an account. */
export const balanceOfToken = async (
  options: BalanceOfTokenOptions,
): Promise<BalanceOfTokenResult> => {
  if (!options.contractAddress) {
    throw new Error("token balance requires --contract-address.");
  }

  options.onProgress?.("Creating clients");
  const { publicClient, contractAddress } = createClientContext(options);
  const account =
    options.account ?? loadAccount(options.privateKey, options.mnemonic).address;

  options.onProgress?.(`Reading confidential balance for ${account}`);
  const balanceHandle = await readConfidentialBalance(
    { publicClient, contractAddress },
    account,
  );
  options.onProgress?.(`Confidential balance handle: ${balanceHandle}`);

  options.onProgress?.("Reading token metadata");
  const tokenMetadata = await readTokenMetadata({ publicClient, contractAddress });

  return {
    network: options.network,
    contractAddress,
    account,
    balanceHandle,
    tokenMetadata,
  };
};
