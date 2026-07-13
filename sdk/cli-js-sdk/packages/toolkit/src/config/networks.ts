import {
  defineFhevmChain,
  mainnet,
  sepolia,
} from "@fhevm/sdk/chains";
import {
  mainnet as viemMainnet,
  polygonAmoy,
  sepolia as viemSepolia,
} from "viem/chains";

import { DEFAULT_NETWORK } from "../types";
import type { NetworkName } from "../types";
import type { NetworkConfig } from "./types";

export const DEFAULT_MAINNET_RPC_URL = "https://eth.drpc.org";
export const DEFAULT_SEPOLIA_RPC_URL = "https://sepolia.drpc.org";
export const DEFAULT_POLYGON_AMOY_RPC_URL =
  "https://rpc-amoy.polygon.technology";

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
      protocolConfig: {
        address: "0x51f9AFBc89Ea792e1a21a12AB802ab58D4dbee83",
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

const devnetAmoy = defineFhevmChain({
  id: 80_002,
  fhevm: {
    contracts: {
      acl: {
        address: "0x21d5fcabee8260b8aC18A2f0cEe6869AE08cc44b",
      },
      inputVerifier: {
        address: "0x371B9661c6DCd849E2779d532CA74d75A171dfa9",
      },
      kmsVerifier: {
        address: "0x2D7Ae863BF7537402AB6025bEbB4668dd9F9F4b6",
      },
      protocolConfig: {
        address: "0x4CcF009Aba90D04f52b31fc7aDdE240578aFe10F",
      },
    },
    relayerUrl: "https://relayer.dev.zama.cloud",
    gateway: devnet.fhevm.gateway,
  },
});

const NETWORK_CONFIGS = {
  "testnet": {
    fhevmChain: sepolia,
    hostChain: viemSepolia,
    defaultRpcUrl: DEFAULT_SEPOLIA_RPC_URL,
    envRpcUrl: "SEPOLIA_RPC_URL",
    fheTestAddress: "0x94B9d3aF050687D1F76251aD7D09a1F216a19845",
    runtime: {
      moduleVersions: { tfhe: "1.5.3" },
    },
  },
  "devnet": {
    fhevmChain: devnet,
    hostChain: viemSepolia,
    defaultRpcUrl: DEFAULT_SEPOLIA_RPC_URL,
    envRpcUrl: "SEPOLIA_RPC_URL",
    fheTestAddress: "0xf56a7990E63a63eC75aD9Aa07De8cB6bF7baa805",
  },
  "devnet-amoy": {
    fhevmChain: devnetAmoy,
    hostChain: polygonAmoy,
    defaultRpcUrl: DEFAULT_POLYGON_AMOY_RPC_URL,
    envRpcUrl: "POLYGON_AMOY_RPC_URL",
    fheTestAddress: "0x7553CB9124f974Ee475E5cE45482F90d5B6076BC",
  },
  "mainnet": {
    fhevmChain: mainnet,
    hostChain: viemMainnet,
    defaultRpcUrl: DEFAULT_MAINNET_RPC_URL,
    envRpcUrl: "MAINNET_RPC_URL",
    fheTestAddress: "0xba4d707745689eD409d4Afac8722224f5FD78C63",
    runtime: {
      moduleVersions: { tfhe: "1.5.3" },
    },
  },
} as const satisfies Record<NetworkName, NetworkConfig>;

export const resolveNetworkConfig = (network: NetworkName): NetworkConfig =>
  NETWORK_CONFIGS[network];
