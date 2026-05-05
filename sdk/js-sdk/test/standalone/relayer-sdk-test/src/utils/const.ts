import { type FhevmChain, defineFhevmChain, sepolia } from "@fhevm/sdk/chains"
import type { Hex } from "viem"

import type { AccountKey } from "./account"

const privateKey: Hex = import.meta.env.VITE_PRIVATE_KEY as Hex
const mnemonic: string = import.meta.env.VITE_MNEMONIC as string
export const credentials: AccountKey = {
    mnemonic,
    privateKey,
}

export const devnet = /*#__PURE__*/ defineFhevmChain({
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
})

export type FhevmChainName = "testnet" | "devnet" | "devnetV2" | "testnetV2"

export const DEFAULT_FHEVM_CHAIN: FhevmChainName = "devnet"

export type FhevmChainConfig = {
    readonly chain: FhevmChain
    readonly rpcUrl: string
}

const FHEVM_CHAINS: Record<FhevmChainName, FhevmChainConfig> = {
    testnet: {
        chain: sepolia,
        rpcUrl: "https://ethereum-sepolia-rpc.publicnode.com",
    },
    devnet: {
        chain: devnet,
        rpcUrl: "https://ethereum-sepolia-rpc.publicnode.com",
    },
    devnetV2: {
        chain: devnet,
        rpcUrl: "https://ethereum-sepolia-rpc.publicnode.com",
    },
    testnetV2: {
        chain: sepolia,
        rpcUrl: "https://ethereum-sepolia-rpc.publicnode.com",
    },
}

const RELAYER_CONTRACT_ADDRESSES: Record<FhevmChainName, Hex> = {
    testnet: "0x587CefedEA1dD8b937254184B30625a819B447d5",
    testnetV2: "0x587CefedEA1dD8b937254184B30625a819B447d5",
    devnet: "0x29D14ae49A6C3d99F75B1b6c931937d1018bfDf3",
    devnetV2: "0x29D14ae49A6C3d99F75B1b6c931937d1018bfDf3",
}

const RELAYER_CONTRACT_MIRROR_ADDRESSES: Record<FhevmChainName, Hex> = {
    testnet: "0x78A0d832ECb8b3C7c7eF87E822549bd36aF0D0E6",
    testnetV2: "0x78A0d832ECb8b3C7c7eF87E822549bd36aF0D0E6",
    devnet: "0x2ec07593d8C4F7704Df9b490916D4c495bD78Fc1",
    devnetV2: "0x2ec07593d8C4F7704Df9b490916D4c495bD78Fc1",
}

export const resolveRelayerContractAddress = (config: FhevmChainName): Hex =>
    RELAYER_CONTRACT_ADDRESSES[config]

export const resolveRelayerContractMirrorAddress = (
    config: FhevmChainName
): Hex => RELAYER_CONTRACT_MIRROR_ADDRESSES[config]

export const resolveFhevmChainConfig = (
    config: FhevmChainName
): FhevmChainConfig => FHEVM_CHAINS[config]

export const normalizeRelayerConfig = (config: string): FhevmChainName =>
    config.includes("devnet")
        ? config.includes("V2")
            ? "devnetV2"
            : "devnet"
        : config.includes("V2")
          ? "testnetV2"
          : DEFAULT_FHEVM_CHAIN
