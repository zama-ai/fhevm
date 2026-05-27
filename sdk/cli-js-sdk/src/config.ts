import { defineFhevmChain, sepolia } from "@fhevm/sdk/chains";
import {
  createFhevmClient,
  hasFhevmRuntimeConfig,
  setFhevmRuntimeConfig,
} from "@fhevm/sdk/viem";
import type { FhevmChain } from "@fhevm/sdk/chains";
import {
  createPublicClient,
  createWalletClient,
  http,
  type Chain,
  type Hex,
} from "viem";
import {
  mnemonicToAccount,
  privateKeyToAccount,
  type Account,
} from "viem/accounts";
import { mainnet as viemMainnet, sepolia as viemSepolia } from "viem/chains";

import type { NetworkName } from "./types";

export const DEFAULT_NETWORK: NetworkName = "testnet";
export const DEFAULT_TESTNET_RPC_URL =
  "https://ethereum-sepolia-rpc.publicnode.com";
export const DEFAULT_DEVNET_RPC_URL = DEFAULT_TESTNET_RPC_URL;

const devnet = defineFhevmChain({
  id: 11_155_111,
  fhevm: {
    contracts: {
      acl: {
        address: "0xBCA6F8De823a399Dc431930FD5EE550Bf1C0013e",
      },
      inputVerifier: {
        address: "0x6B32f47E39B0F8bE8bEAD5B8990F62E3e28ac08d",
      },
      kmsVerifier: {
        address: "0x3F3819BeBE4bD0EFEf8078Df6f9B574ADa80CCA4",
      },
    },
    relayerUrl: "https://relayer.dev.zama.cloud",
    gateway: {
      id: 10_900,
      contracts: {
        decryption: {
          address: "0xA4dc265D54D25D41565c60d36097E8955B03decD",
        },
        inputVerification: {
          address: "0xf091D9B4C2da7ecd11858cDD1F4515a8a767D755",
        },
      },
    },
  },
});

type NetworkConfig = Readonly<{
  fhevmChain: FhevmChain;
  hostChain: Chain;
  defaultRpcUrl: string;
  envRpcUrl: string;
  fheTestAddress: Hex;
}>;

const NETWORK_CONFIGS = {
  testnet: {
    fhevmChain: sepolia,
    hostChain: viemSepolia,
    defaultRpcUrl: DEFAULT_TESTNET_RPC_URL,
    envRpcUrl: "SEPOLIA_RPC_URL",
    fheTestAddress: "0x94B9d3aF050687D1F76251aD7D09a1F216a19845",
  },
  devnet: {
    fhevmChain: devnet,
    hostChain: viemSepolia,
    defaultRpcUrl: DEFAULT_DEVNET_RPC_URL,
    envRpcUrl: "DEVNET_RPC_URL",
    fheTestAddress: "0xD26bB032e2F06A5382902559c4EbBB82C35C6dDF",
  },
} as const satisfies Record<NetworkName, NetworkConfig>;

const _futureMainnetHostChain: Chain = viemMainnet;

export type ClientOptions = Readonly<{
  network: NetworkName;
  relayerUrl?: string;
  rpcUrl?: string;
  contractAddress?: Hex;
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

export const resolveNetworkConfig = (network: NetworkName): NetworkConfig =>
  NETWORK_CONFIGS[network];

export const resolveChain = (options: ClientOptions): FhevmChain =>
  withRelayerUrl(resolveNetworkConfig(options.network).fhevmChain, options.relayerUrl);

export const resolveContractAddress = (options: ClientOptions): Hex =>
  options.contractAddress ?? resolveNetworkConfig(options.network).fheTestAddress;

export const resolveRpcUrl = (options: ClientOptions): string => {
  const config = resolveNetworkConfig(options.network);
  return rpcUrlFromOptions(options.rpcUrl, config);
};

const rpcUrlFromOptions = (
  rpcUrl: string | undefined,
  config: NetworkConfig,
): string => rpcUrl ?? process.env[config.envRpcUrl] ?? config.defaultRpcUrl;

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
    chain: resolveNetworkConfig(options.network).hostChain,
    transport,
  });

  return { account, chain, fhevm, publicClient, rpcUrl, walletClient };
};
