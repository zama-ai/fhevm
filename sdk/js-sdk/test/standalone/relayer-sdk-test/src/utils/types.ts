import type { TypedValue } from "@fhevm/sdk/types"
import type {
    createFhevmBaseClient,
    createFhevmClient,
    createFhevmDecryptClient,
    createFhevmEncryptClient,
} from "@fhevm/sdk/viem"

//import type { Hex } from "viem"

export interface InputSummary {
    contractAddress: string
    userAddress: string
    values: { label: string; value: string; type: string }[]
}

export interface CiphertextSummary {
    readonly handles: readonly string[]
    readonly inputProof: string
}

export interface VerifyInputSummary {
    results: string[]
}

export interface PublicDecryptSummary {
    clearValues: Record<string, string>
    abiEncodedClearValues: string
    decryptionProof: string
}

export interface BufferWriterResult {
    values: TypedValue[]
    valuesSummary: { label: string; value: string; type: string }[]
}

export type BufferWriter = () => BufferWriterResult

export type BaseClient = ReturnType<typeof createFhevmBaseClient>
export type EncryptClient = ReturnType<typeof createFhevmEncryptClient>
export type DecryptClient = ReturnType<typeof createFhevmDecryptClient>
export type FullClient = ReturnType<typeof createFhevmClient>
