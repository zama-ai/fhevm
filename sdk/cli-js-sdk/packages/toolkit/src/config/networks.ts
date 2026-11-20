import {
  defineFhevmChain,
  mainnet,
  polygonAmoy as sdkPolygonAmoy,
  sepolia,
} from "@fhevm/sdk/chains";
import {
  mainnet as viemMainnet,
  polygon as viemPolygon,
  polygonAmoy,
  sepolia as viemSepolia,
} from "viem/chains";

import { DEFAULT_NETWORK } from "../types";
import type { NetworkName } from "../types";
import type { NetworkConfig } from "./types";

export const DEFAULT_MAINNET_RPC_URL = "https://eth.drpc.org";
export const DEFAULT_SEPOLIA_RPC_URL = "https://sepolia.drpc.org";
export const DEFAULT_POLYGON_RPC_URL = "https://polygon.drpc.org";
export const DEFAULT_POLYGON_AMOY_RPC_URL = "https://polygon-amoy.drpc.org";

const devnet = defineFhevmChain({
  id: 11_155_111,
  fhevm: {
    contracts: {
      acl: {
        address: "0x6da02d370A0EAa21A67f7f2180E0348e8Be57A2A",
      },
      inputVerifier: {
        address: "0x6A8bf4dBdA66f47af2C988606BA33eF4C4B9CF43",
      },
      kmsVerifier: {
        address: "0xf75BCED1A3f04f693dFD52f6210BDFf43CD232a1",
      },
      protocolConfig: {
        address: "0xf9759981BFFaAab05265a73e32136CEd6b1358dc",
      },
    },
    relayerUrl: "https://relayer.dev.zama.cloud",
    gateway: {
      id: 10_900,
      contracts: {
        decryption: {
          address: "0xd86A08489887Fae000de21fa064D89C75b5CC62B",
        },
        inputVerification: {
          address: "0xcC3355b1849F8a73F8e4d51cF3Be9C1753fD4B5B",
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

const polygon = defineFhevmChain({
  id: 137,
  fhevm: {
    contracts: {
      acl: {
        address: "0x6737F17e31cf26a1b62fb0362acC5a16CB156F49",
      },
      inputVerifier: {
        address: "0xf40BD204B035522EaAc8E5afAdc55113Acac96ca",
      },
      kmsVerifier: {
        address: "0x14e609595474874Dd6b6128376E336EfADfdBE37",
      },
      protocolConfig: {
        address: "0x17f62Ab3A1Ea519703cD597410147A30Fa1a7f1e",
      },
    },
    relayerUrl: "https://relayer.mainnet.zama.org",
    gateway: {
      id: 261_131,
      contracts: {
        decryption: {
          address: "0x0f6024a97684f7d90ddb0fAAD79cB15F2C888D24",
        },
        inputVerification: {
          address: "0xcB1bB072f38bdAF0F328CdEf1Fc6eDa1DF029287",
        },
      },
    },
  },
});

const NETWORK_CONFIGS = {
  "testnet": {
    fhevmChain: sepolia,
    hostChain: viemSepolia,
    defaultRpcUrl: DEFAULT_SEPOLIA_RPC_URL,
    envRpcUrl: "SEPOLIA_RPC_URL",
    fheTestAddress: "0x94B9d3aF050687D1F76251aD7D09a1F216a19845",
  },
  "testnet-amoy": {
    fhevmChain: sdkPolygonAmoy,
    hostChain: polygonAmoy,
    defaultRpcUrl: DEFAULT_POLYGON_AMOY_RPC_URL,
    envRpcUrl: "POLYGON_AMOY_RPC_URL",
    fheTestAddress: "0xa66bCEd74D1Df0736d0eb8E52371b1b1AAA1F0F0",
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
  },
  "polygon": {
    fhevmChain: polygon,
    hostChain: viemPolygon,
    defaultRpcUrl: DEFAULT_POLYGON_RPC_URL,
    envRpcUrl: "POLYGON_RPC_URL",
    fheTestAddress: "0xFb10eda9e9b4f3f7dd928B6F32fBB94E2a20451d",
  },
} as const satisfies Record<NetworkName, NetworkConfig>;

export const resolveNetworkConfig = (network: NetworkName): NetworkConfig =>
  NETWORK_CONFIGS[network];
