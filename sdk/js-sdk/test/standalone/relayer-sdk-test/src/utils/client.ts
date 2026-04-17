import { createTypedValueArray } from "@fhevm/sdk/base"
import type { EncryptedValue, TypedValue } from "@fhevm/sdk/types"
import {
    createFhevmBaseClient,
    createFhevmClient,
    createFhevmDecryptClient,
    createFhevmEncryptClient,
    setFhevmRuntimeConfig,
} from "@fhevm/sdk/viem"
import type { Hex } from "viem"
import { createPublicClient, http } from "viem"
import { sepolia as viemSepolia } from "viem/chains"

import type { FhevmChainConfig } from "./const"
import {
    randomAddress,
    randomUint8,
    randomUint16,
    randomUint32,
    randomUint64,
    randomUint128,
    randomUint256,
} from "./random"
import type { EncryptClient, InputSummary } from "./types"

////////////////////////////////////////////////////////////////////////////////

function setRuntimeConfig(log: (msg: string) => void) {
    setFhevmRuntimeConfig({
        logger: {
            debug: (message: string) => log(`  [debug] ${message}`),
            error: (message: string, cause: unknown) => {
                log(`  [error] ${message}`)
                if (cause !== undefined) {
                    log(`  [error] ${cause}`)
                }
            },
        },
    })
}

////////////////////////////////////////////////////////////////////////////////

export async function initBaseRuntime(config: FhevmChainConfig, log: (msg: string) => void) {
    setRuntimeConfig(log)
    await createBaseClient(config)
}

////////////////////////////////////////////////////////////////////////////////

export async function initEncryptRuntime(config: FhevmChainConfig, log: (msg: string) => void) {
    setRuntimeConfig(log)
    await createEncryptClient(config)
}

////////////////////////////////////////////////////////////////////////////////

export async function initDecryptRuntime(config: FhevmChainConfig, log: (msg: string) => void) {
    setRuntimeConfig(log)
    await createFullClient(config)
}

////////////////////////////////////////////////////////////////////////////////

export async function initFullRuntime(config: FhevmChainConfig, log: (msg: string) => void) {
    setRuntimeConfig(log)
    await createFullClient(config)
}

////////////////////////////////////////////////////////////////////////////////

export async function createFullClient(config: FhevmChainConfig) {
    const viemPublicClient = createPublicClient({
        chain: viemSepolia,
        transport: http(config.rpcUrl),
    })

    const fhevmClient = createFhevmClient({
        chain: config.chain,
        publicClient: viemPublicClient,
    })

    // init encrypt wasm and download encryption key
    await fhevmClient.ready

    return fhevmClient
}

////////////////////////////////////////////////////////////////////////////////

export async function createBaseClient(config: FhevmChainConfig) {
    const viemPublicClient = createPublicClient({
        chain: viemSepolia,
        transport: http(config.rpcUrl),
    })

    const fhevmClient = createFhevmBaseClient({
        chain: config.chain,
        publicClient: viemPublicClient,
    })

    // init is instant for a base client
    await fhevmClient.ready

    return fhevmClient
}

////////////////////////////////////////////////////////////////////////////////

export async function createEncryptClient(config: FhevmChainConfig) {
    const viemPublicClient = createPublicClient({
        chain: viemSepolia,
        transport: http(config.rpcUrl),
    })

    const fhevmClient = createFhevmEncryptClient({
        chain: config.chain,
        publicClient: viemPublicClient,
    })

    // init encrypt wasm and download encryption key
    await fhevmClient.ready

    return fhevmClient
}

////////////////////////////////////////////////////////////////////////////////

export async function createDecryptClient(config: FhevmChainConfig) {
    const viemPublicClient = createPublicClient({
        chain: viemSepolia,
        transport: http(config.rpcUrl),
    })

    const fhevmClient = createFhevmDecryptClient({
        chain: config.chain,
        publicClient: viemPublicClient,
    })

    // init encrypt wasm and download encryption key
    await fhevmClient.ready

    return fhevmClient
}

export async function encryptRandom(parameters: {
    readonly client: EncryptClient
    readonly userAddress: Hex
    readonly contractAddress: Hex
}): Promise<{
    readonly encryptedValues: readonly EncryptedValue[]
    readonly values: ReadonlyArray<TypedValue>
    readonly inputProof: Hex
    readonly inputSummary: InputSummary
}> {
    const { client, userAddress, contractAddress } = parameters

    const sampleBool = randomUint8() % 2 === 0
    const sampleUint8 = randomUint8()
    const sampleUint16 = randomUint16()
    const sampleUint32 = randomUint32()
    const sampleUint64 = randomUint64()
    const sampleUint128 = randomUint128()
    const sampleUint256 = randomUint256()
    const sampleAddress = randomAddress()

    const values = createTypedValueArray([
        {
            type: "bool",
            value: sampleBool,
        },
        {
            type: "uint8",
            value: sampleUint8,
        },
        {
            type: "uint16",
            value: sampleUint16,
        },
        {
            type: "uint32",
            value: sampleUint32,
        },
        {
            type: "uint64",
            value: sampleUint64,
        },
        {
            type: "uint128",
            value: sampleUint128,
        },
        {
            type: "uint256",
            value: sampleUint256,
        },
        {
            type: "address",
            value: sampleAddress,
        },
    ])

    const ciphertexts = await client.encryptValues({
        values,
        contractAddress,
        userAddress,
    })

    const valuesSummary = [
        {
            label: "addBool",
            value: sampleBool.toString(),
            type: "bool",
        },
        { label: "add8", value: sampleUint8.toString(), type: "uint8" },
        {
            label: "add16",
            value: sampleUint16.toString(),
            type: "uint16",
        },
        {
            label: "add32",
            value: sampleUint32.toString(),
            type: "uint32",
        },
        {
            label: "add64",
            value: sampleUint64.toString(),
            type: "uint64",
        },
        {
            label: "add128",
            value: sampleUint128.toString(),
            type: "uint128",
        },
        {
            label: "add256",
            value: sampleUint256.toString(),
            type: "uint256",
        },
        {
            label: "addAddress",
            value: sampleAddress,
            type: "address",
        },
    ]

    return {
        encryptedValues: ciphertexts.encryptedValues,
        inputProof: ciphertexts.inputProof,
        values,
        inputSummary: {
            contractAddress,
            userAddress,
            values: valuesSummary,
        },
    }
}
