import { defineFhevmChain, sepolia } from "@fhevm/sdk/chains";
import { mainnet as viemMainnet, sepolia as viemSepolia } from "viem/chains";
import type { Chain } from "viem";

import type { NetworkName } from "../types";
import type { NetworkConfig } from "./types";

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

export const resolveNetworkConfig = (network: NetworkName): NetworkConfig =>
  NETWORK_CONFIGS[network];
