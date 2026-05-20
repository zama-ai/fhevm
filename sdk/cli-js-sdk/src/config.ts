import { sepolia } from "@fhevm/sdk/chains";
import {
  createFhevmClient,
  hasFhevmRuntimeConfig,
  setFhevmRuntimeConfig,
} from "@fhevm/sdk/viem";
import type { FhevmChain } from "@fhevm/sdk/chains";
import { createPublicClient, createWalletClient, http, type Hex } from "viem";
import {
  mnemonicToAccount,
  privateKeyToAccount,
  type Account,
} from "viem/accounts";
import { sepolia as viemSepolia } from "viem/chains";

import type { NetworkName } from "./types";

export const DEFAULT_NETWORK: NetworkName = "testnet";
export const DEFAULT_TESTNET_RPC_URL =
  "https://ethereum-sepolia-rpc.publicnode.com";
export const TESTNET_RELAYER_SDK_TEST_CONTRACT: Hex =
  "0x587CefedEA1dD8b937254184B30625a819B447d5";

const locateSdkWasmFile = (file: string): URL => {
  if (file.startsWith("tfhe") || file.startsWith("startWorkers")) {
    return new URL(
      `../node_modules/@fhevm/sdk/wasm/tfhe/${file}`,
      import.meta.url,
    );
  }
  if (file.startsWith("kms_lib")) {
    return new URL(
      `../node_modules/@fhevm/sdk/wasm/tkms/${file}`,
      import.meta.url,
    );
  }
  return new URL(`../node_modules/@fhevm/sdk/wasm/${file}`, import.meta.url);
};

export type ClientOptions = Readonly<{
  network: NetworkName;
  relayerUrl?: string;
  rpcUrl?: string;
}>;

const withRelayerUrl = (chain: FhevmChain, relayerUrl?: string): FhevmChain => {
  if (!relayerUrl) return chain;

  return {
    ...chain,
    fhevm: {
      ...chain.fhevm,
      relayerUrl: normalizeRelayerUrl(relayerUrl),
    },
  };
};

export const normalizeRelayerUrl = (value: string): string => {
  const withProtocol = /^https?:\/\//i.test(value) ? value : `http://${value}`;
  return withProtocol.replace(/\/+$/, "").replace(/\/v[12]$/i, "");
};

export const resolveChain = (options: ClientOptions): FhevmChain => {
  if (options.network !== "testnet") {
    throw new Error(
      `Unsupported network "${options.network}". Only "testnet" is supported for now.`,
    );
  }
  return withRelayerUrl(sepolia, options.relayerUrl);
};

export const resolveRpcUrl = (rpcUrl?: string): string =>
  rpcUrl ?? process.env.SEPOLIA_RPC_URL ?? DEFAULT_TESTNET_RPC_URL;

export const createClients = (options: ClientOptions) => {
  if (!hasFhevmRuntimeConfig()) {
    setFhevmRuntimeConfig({
      locateFile: locateSdkWasmFile,
      singleThread: true,
    });
  }

  const chain = resolveChain(options);
  const rpcUrl = resolveRpcUrl(options.rpcUrl);
  const transport = http(rpcUrl);
  const publicClient = createPublicClient({
    chain: viemSepolia,
    transport,
  });
  const fhevm = createFhevmClient({ chain, publicClient });

  return { chain, fhevm, publicClient, rpcUrl, transport };
};

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

export const createWallet = (
  options: ClientOptions & { privateKey?: Hex; mnemonic?: string },
) => {
  const account = loadAccount(options.privateKey, options.mnemonic);
  const { transport, publicClient, fhevm, chain, rpcUrl } =
    createClients(options);
  const walletClient = createWalletClient({
    account,
    chain: viemSepolia,
    transport,
  });

  return { account, chain, fhevm, publicClient, rpcUrl, walletClient };
};
