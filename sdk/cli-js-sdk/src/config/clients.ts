import {
  createFhevmClient,
  hasFhevmRuntimeConfig,
  setFhevmRuntimeConfig,
} from "@fhevm/sdk/viem";
import {
  createPublicClient,
  createWalletClient,
  http,
  type Hex,
} from "viem";

import { loadAccount } from "./account";
import { resolveNetworkConfig } from "./networks";
import {
  resolveChain,
  resolveContractAddress,
  resolveRpcUrl,
} from "./resolve";
import type { ClientOptions } from "./types";

export const createClients = (options: ClientOptions) => {
  if (!hasFhevmRuntimeConfig()) {
    setFhevmRuntimeConfig({ singleThread: true });
  }

  const chain = resolveChain(options);
  const networkConfig = resolveNetworkConfig(options.network);
  const rpcUrl = resolveRpcUrl(options);
  const transport = http(rpcUrl);
  const publicClient = createPublicClient({
    chain: networkConfig.hostChain,
    transport,
  });
  const fhevm = createFhevmClient({ chain, publicClient });

  return { chain, fhevm, publicClient, rpcUrl, transport };
};

export type ClientContext = ReturnType<typeof createClients> &
  Readonly<{
    contractAddress: Hex;
  }>;

export const createClientContext = (options: ClientOptions): ClientContext => ({
  ...createClients(options),
  contractAddress: resolveContractAddress(options),
});

export const createWallet = (
  options: ClientOptions & { privateKey?: Hex; mnemonic?: string },
) => {
  const account = loadAccount(options.privateKey, options.mnemonic);
  const { transport, publicClient, fhevm, chain, rpcUrl } =
    createClients(options);
  const walletClient = createWalletClient({
    account,
    chain: resolveNetworkConfig(options.network).hostChain,
    transport,
  });

  return { account, chain, fhevm, publicClient, rpcUrl, walletClient };
};

export type WalletContext = ReturnType<typeof createWallet> &
  Readonly<{
    contractAddress: Hex;
  }>;

export const createWalletContext = (
  options: ClientOptions & { privateKey?: Hex; mnemonic?: string },
): WalletContext => ({
  ...createWallet(options),
  contractAddress: resolveContractAddress(options),
});
