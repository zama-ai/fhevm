import { createTypedValueArray } from "@fhevm/sdk/base"
import type { Abi, AbiFunction, Account, Hex, PublicClient, WalletClient } from "viem"

import RelayerSDKTest from "../assets/RelayerSDKTest.json"
import { ensureArray } from "./array"
import { randomAddress, randomUint8, randomUint128 } from "./random"
import type { BufferWriter } from "./types"

export const relayerSDKTestAbi = RelayerSDKTest.abi as Abi

export const DECRYPT_TYPES = ["bool", "uint8", "uint128", "address", "mixed"] as const

export type DecryptType = (typeof DECRYPT_TYPES)[number]

export const DEFAULT_DECRYPT_TYPE: DecryptType = "bool"

export const DECRYPT_TYPE_LABELS: Record<DecryptType, string> = {
    bool: "boolean",
    uint8: "uint8",
    uint128: "uint128",
    address: "address",
    mixed: "mixed",
}

const DECRYPT_FUNCTION_NAMES: Record<DecryptType, AbiFunction["name"]> = {
    bool: "makePubliclyDecryptableExternalEbool",
    uint8: "makePubliclyDecryptableExternalEuint8",
    uint128: "makePubliclyDecryptableExternalEuint128",
    address: "makePubliclyDecryptableExternalEaddress",
    mixed: "makePubliclyDecryptableExternalMixed",
}

export const isDecryptType = (value: unknown): value is DecryptType =>
    typeof value === "string" && DECRYPT_TYPES.includes(value as DecryptType)

export const getPublicDecryptAbiFunction = (type: DecryptType): AbiFunction => {
    const fnName = DECRYPT_FUNCTION_NAMES[type]
    const fn = relayerSDKTestAbi.find(
        (entry): entry is AbiFunction => entry.type === "function" && entry.name === fnName
    )

    if (!fn) {
        throw new Error(`ABI function ${fnName} not found in RelayerSDKTest`)
    }

    return fn
}

const USER_DECRYPT_FUNCTION_NAMES: Record<DecryptType, AbiFunction["name"]> = {
    bool: "allowSenderExternalEbool",
    uint8: "allowSenderExternalEuint8",
    uint128: "allowSenderExternalEuint128",
    address: "allowSenderExternalEaddress",
    mixed: "allowSenderExternalMixed",
}

export const getUserDecryptAbiFunction = (type: DecryptType): AbiFunction => {
    const fnName = USER_DECRYPT_FUNCTION_NAMES[type]
    const fn = relayerSDKTestAbi.find(
        (entry): entry is AbiFunction => entry.type === "function" && entry.name === fnName
    )

    if (!fn) {
        throw new Error(`ABI function ${fnName} not found in RelayerSDKTest`)
    }

    return fn
}

export const bufferWriters: Record<DecryptType, BufferWriter> = {
    bool: () => {
        const boolSampleValue = randomUint8() % 2 === 0
        return {
            values: createTypedValueArray([
                {
                    type: "bool",
                    value: boolSampleValue,
                },
            ]),
            valuesSummary: [
                {
                    label: `addBool`,
                    value: boolSampleValue.toString(),
                    type: "bool",
                },
            ],
        }
    },
    uint8: () => {
        const uint8SampleValue = randomUint8()
        return {
            values: createTypedValueArray([
                {
                    type: "bool",
                    value: uint8SampleValue,
                },
            ]),
            valuesSummary: [
                {
                    label: `add8`,
                    value: uint8SampleValue.toString(),
                    type: "uint8",
                },
            ],
        }
    },
    uint128: () => {
        const uint128SampleValue = randomUint128()
        return {
            values: createTypedValueArray([
                {
                    type: "uint128",
                    value: uint128SampleValue,
                },
            ]),
            valuesSummary: [
                {
                    label: `add128`,
                    value: uint128SampleValue.toString(),
                    type: "uint128",
                },
            ],
        }
    },
    address: () => {
        const addressSampleValue = randomAddress()
        return {
            values: createTypedValueArray([
                {
                    type: "address",
                    value: addressSampleValue,
                },
            ]),
            valuesSummary: [
                {
                    label: `addAddress`,
                    value: addressSampleValue,
                    type: "address",
                },
            ],
        }
    },
    mixed: () => {
        const boolSampleValue = randomUint8() % 2 === 0
        const uint8SampleValue = randomUint8()
        const uint128SampleValue = randomUint128()
        const addressSampleValue = randomAddress()
        return {
            values: createTypedValueArray([
                {
                    type: "bool",
                    value: boolSampleValue,
                },
                {
                    type: "uint8",
                    value: uint8SampleValue,
                },
                {
                    type: "uint128",
                    value: uint128SampleValue,
                },
                {
                    type: "address",
                    value: addressSampleValue,
                },
            ]),
            valuesSummary: [
                {
                    label: `addBool`,
                    value: boolSampleValue.toString(),
                    type: "bool",
                },
                {
                    label: `add8`,
                    value: uint8SampleValue.toString(),
                    type: "uint8",
                },
                {
                    label: `add128`,
                    value: uint128SampleValue.toString(),
                    type: "uint128",
                },
                {
                    label: `addAddress`,
                    value: addressSampleValue,
                    type: "address",
                },
            ],
        }
    },
}

export async function callMakeUserDecryptable(parameters: {
    readonly decryptType: DecryptType
    readonly walletClient: PublicClient & WalletClient
    readonly account: Account
    readonly ciphertexts: Hex | readonly Hex[]
    readonly inputProof: Hex
    readonly contractAddress: Hex
}) {
    const { decryptType, ciphertexts, account, walletClient, contractAddress, inputProof } = parameters
    const abiFunction = getUserDecryptAbiFunction(decryptType)
    const ciphertextArgs = ensureArray(ciphertexts).flat()
    const { result, request } = await walletClient.simulateContract({
        account,
        address: contractAddress,
        abi: relayerSDKTestAbi,
        functionName: abiFunction.name,
        args: ciphertextArgs.concat(inputProof),
    })

    const hash = await walletClient.writeContract(request)

    // Wait for the tx to succeed
    await walletClient.waitForTransactionReceipt({ hash })

    const xValues: Hex[] = ensureArray<Hex>(result as Hex | Hex[])

    return xValues
}

export async function callMakePubliclyDecryptable(parameters: {
    readonly decryptType: DecryptType
    readonly walletClient: PublicClient & WalletClient
    readonly account: Account
    readonly ciphertexts: Hex | readonly Hex[]
    readonly inputProof: Hex
    readonly contractAddress: Hex
}) {
    const { decryptType, ciphertexts, account, walletClient, contractAddress, inputProof } = parameters
    const abiFunction = getPublicDecryptAbiFunction(decryptType)
    const ciphertextArgs = ensureArray(ciphertexts).flat()
    const { result, request } = await walletClient.simulateContract({
        account,
        address: contractAddress,
        abi: relayerSDKTestAbi,
        functionName: abiFunction.name,
        args: ciphertextArgs.concat(inputProof),
    })

    const hash = await walletClient.writeContract(request)

    // Wait for the tx to succeed
    await walletClient.waitForTransactionReceipt({ hash })

    const xValues: Hex[] = ensureArray<Hex>(result as Hex | Hex[])

    return xValues
}
