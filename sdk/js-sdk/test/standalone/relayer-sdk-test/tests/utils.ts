export type RelayerConfigName = "testnet" | "devnet" | "devnetV2" | "testnetV2"

export const decryptionTypes = [
    "bool",
    "uint8",
    "uint128",
    "address",
    "mixed",
] as const

export const RELAYER_CONFIG_NAMES = [
    "testnet",
    "devnet",
    "devnetV2",
    "testnetV2",
] as const

export type DecryptionTypes = (typeof decryptionTypes)[number]
