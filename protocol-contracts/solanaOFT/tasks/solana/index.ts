import assert from 'assert'
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs'
import path from 'node:path'

import {
    fetchAddressLookupTable,
    mplToolbox,
    setComputeUnitLimit,
    setComputeUnitPrice,
} from '@metaplex-foundation/mpl-toolbox'
import {
    AddressLookupTableInput,
    EddsaInterface,
    Instruction,
    KeypairSigner,
    PublicKey,
    TransactionBuilder,
    Umi,
    createNoopSigner,
    createSignerFromKeypair,
    publicKey,
    signerIdentity,
    transactionBuilder,
} from '@metaplex-foundation/umi'
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults'
import { createWeb3JsEddsa } from '@metaplex-foundation/umi-eddsa-web3js'
import { toWeb3JsInstruction, toWeb3JsPublicKey } from '@metaplex-foundation/umi-web3js-adapters'
import { AddressLookupTableAccount, Connection } from '@solana/web3.js'
import { getSimulationComputeUnits } from '@solana-developers/helpers'
import { backOff } from 'exponential-backoff'

import { formatEid } from '@layerzerolabs/devtools'
import { getPrioritizationFees, getSolanaKeypair } from '@layerzerolabs/devtools-solana'
import { promptToContinue } from '@layerzerolabs/io-devtools'
import { EndpointId, endpointIdToNetwork } from '@layerzerolabs/lz-definitions'
import { OftPDA } from '@layerzerolabs/oft-v2-solana-sdk'

import { DebugLogger, KnownWarnings, createSolanaConnectionFactory } from '../common/utils'

export const DEFAULT_LOOKUP_TABLE_ADDRESS: Partial<Record<EndpointId, PublicKey>> = {
    [EndpointId.SOLANA_V2_MAINNET]: publicKey('AokBxha6VMLLgf97B5VYHEtqztamWmYERBmmFvjuTzJB'),
    [EndpointId.SOLANA_V2_TESTNET]: publicKey('9thqPdbR27A1yLWw2spwJLySemiGMXxPnEvfmXVk4KuK'),
}

type DeriveConnectionParams =
    | boolean
    | {
          readOnly?: boolean
          noopSigner?: PublicKey
      }
/**
 * Derive common connection and UMI objects for a given endpoint ID.
 * @param eid {EndpointId}
 */
export const deriveConnection = async (eid: EndpointId, params: DeriveConnectionParams = false) => {
    // line below is for backwards compatibility (second param was initially only readOnly, updated to an object)
    const { readOnly = false, noopSigner } = typeof params === 'object' ? params : { readOnly: params }
    const keypair = await getSolanaKeypair(readOnly)
    const connectionFactory = createSolanaConnectionFactory()
    const connection = await connectionFactory(eid)
    const umi = createUmi(connection.rpcEndpoint).use(mplToolbox())
    const umiWalletKeyPair = umi.eddsa.createKeypairFromSecretKey(keypair.secretKey)
    const umiWalletSigner = noopSigner ? createNoopSigner(noopSigner) : createSignerFromKeypair(umi, umiWalletKeyPair)
    umi.use(signerIdentity(umiWalletSigner))
    return {
        connection,
        umi,
        umiWalletKeyPair,
        umiWalletSigner,
    }
}

export const useWeb3Js = async () => {
    // note: if we are okay with exporting getSolanaKeypair, then useWeb3js can be removed
    const keypair = await getSolanaKeypair()
    return {
        web3JsKeypair: keypair,
    }
}

/**
 * Derive the keys needed for the OFT program.
 * @param programIdStr {string}
 */
export const deriveKeys = (programIdStr: string) => {
    const programId = publicKey(programIdStr)
    const eddsa: EddsaInterface = createWeb3JsEddsa()
    const oftDeriver = new OftPDA(programId)
    const lockBox = eddsa.generateKeypair()
    const escrowPK = lockBox.publicKey
    const [oftStorePda] = oftDeriver.oftStore(escrowPK)
    return {
        programId,
        lockBox,
        escrowPK,
        oftStorePda,
        eddsa,
    }
}

/**
 * Outputs the OFT accounts to a JSON file.
 * @param eid {EndpointId}
 * @param programId {string}
 * @param mint {string}
 * @param mintAuthority {string}
 * @param escrow {string}
 * @param oftStore {string}
 */
export const saveSolanaDeployment = (
    eid: EndpointId,
    programId: string,
    mint: string,
    mintAuthority: string,
    escrow: string,
    oftStore: string
) => {
    const outputDir = `./deployments/${endpointIdToNetwork(eid)}`
    if (!existsSync(outputDir)) {
        mkdirSync(outputDir, { recursive: true })
    }
    writeFileSync(
        `${outputDir}/OFT.json`,
        JSON.stringify(
            {
                programId,
                mint,
                mintAuthority,
                escrow,
                oftStore,
            },
            null,
            4
        )
    )
    console.log(`Accounts have been saved to ${outputDir}/OFT.json`)
}

/**
 * Reads the OFT deployment info from disk for the given endpoint ID.
 * @param eid {EndpointId}
 * @returns The contents of the OFT.json file as a JSON object.
 */
export const getSolanaDeployment = (
    eid: EndpointId
): {
    programId: string
    mint: string
    mintAuthority: string
    escrow: string
    oftStore: string
} => {
    if (!eid) {
        throw new Error('eid is required')
    }
    const outputDir = path.join('deployments', endpointIdToNetwork(eid))
    const filePath = path.join(outputDir, 'OFT.json') // Note: if you have multiple deployments, change this filename to refer to the desired deployment file

    if (!existsSync(filePath)) {
        DebugLogger.printWarning(KnownWarnings.SOLANA_DEPLOYMENT_NOT_FOUND)
        throw new Error(`Could not find Solana deployment file for eid ${eid} at: ${filePath}`)
    }

    const fileContents = readFileSync(filePath, 'utf-8')
    return JSON.parse(fileContents)
}

/**
 * Safely load the OFT store PDA for a given Solana endpoint.
 * Logs a warning if the deployment file is missing or malformed,
 * and returns null so consumers can decide how to proceed.
 */
export const getOftStoreAddress = (eid: EndpointId): string | null => {
    try {
        const { oftStore } = getSolanaDeployment(eid)
        if (!oftStore) {
            DebugLogger.printWarning(
                KnownWarnings.SOLANA_DEPLOYMENT_MISSING_OFT_STORE,
                `deployment file for ${endpointIdToNetwork(eid)} (eid ${eid}) missing 'oftStore' field.`
            )
            return null
        }
        return oftStore
    } catch (err: any) {
        DebugLogger.printWarning(
            KnownWarnings.ERROR_LOADING_SOLANA_DEPLOYMENT,
            `Could not load Solana deployment for ${endpointIdToNetwork(eid)} (eid ${eid}): ${err.message}`
        )
        return null
    }
}

// TODO: move below outside of solana folder since it's generic
export const getLayerZeroScanLink = (hash: string, isTestnet = false) =>
    isTestnet ? `https://testnet.layerzeroscan.com/tx/${hash}` : `https://layerzeroscan.com/tx/${hash}`

export const getExplorerTxLink = (hash: string, isTestnet = false) =>
    `https://solscan.io/tx/${hash}?cluster=${isTestnet ? 'devnet' : 'mainnet-beta'}`

const getAddressLookupTable = async (_lookupTableAddress: string | PublicKey, connection: Connection, umi: Umi) => {
    const lookupTableAddress = publicKey(_lookupTableAddress)
    const addressLookupTableInput: AddressLookupTableInput = await fetchAddressLookupTable(umi, lookupTableAddress)
    if (!addressLookupTableInput) {
        throw new Error(`No address lookup table found for ${lookupTableAddress}`)
    }
    const { value: lookupTableAccount } = await connection.getAddressLookupTable(toWeb3JsPublicKey(lookupTableAddress))
    if (!lookupTableAccount) {
        throw new Error(`No address lookup table account found for ${lookupTableAddress}`)
    }
    return { lookupTableAddress, addressLookupTableInput, lookupTableAccount }
}

export const getDefaultAddressLookupTable = async (connection: Connection, umi: Umi, fromEid: EndpointId) => {
    // Lookup Table Address and Priority Fee Calculation
    const lookupTableAddress = DEFAULT_LOOKUP_TABLE_ADDRESS[fromEid]
    assert(lookupTableAddress != null, `No lookup table found for ${formatEid(fromEid)}`)
    return getAddressLookupTable(lookupTableAddress, connection, umi)
}

export enum TransactionType {
    CreateToken = 'CreateToken',
    CreateMultisig = 'CreateMultisig',
    InitOft = 'InitOft',
    SetAuthority = 'SetAuthority',
    InitConfig = 'InitConfig',
    SendOFT = 'SendOFT',
}

const TransactionCuEstimates: Record<TransactionType, number> = {
    // for the sample values, they are: devnet, mainnet
    [TransactionType.CreateToken]: 125_000, // actual sample: (59073, 123539), 55785 (more volatile as it has CPI to Metaplex)
    [TransactionType.CreateMultisig]: 5_000, // actual sample: 3,230
    [TransactionType.InitOft]: 70_000, // actual sample: 59207, 65198 (note: this is the only transaction that createOFTAdapter does)
    [TransactionType.SetAuthority]: 8_000, // actual sample: 6424, 6472
    [TransactionType.InitConfig]: 42_000, // actual sample: 33157, 40657
    [TransactionType.SendOFT]: 230_000, // actual sample: 217,784
}

export const getComputeUnitPriceAndLimit = async (
    connection: Connection,
    ixs: Instruction[],
    wallet: KeypairSigner,
    lookupTableAccounts: AddressLookupTableAccount | AddressLookupTableAccount[],
    transactionType: TransactionType
) => {
    const { averageFeeExcludingZeros } = await getPrioritizationFees(connection)
    const priorityFee = Math.round(averageFeeExcludingZeros)
    const computeUnitPrice = BigInt(priorityFee)

    let computeUnits

    try {
        computeUnits = await backOff(
            () =>
                getSimulationComputeUnits(
                    connection,
                    ixs.map((ix) => toWeb3JsInstruction(ix)),
                    toWeb3JsPublicKey(wallet.publicKey),
                    Array.isArray(lookupTableAccounts) ? lookupTableAccounts : [lookupTableAccounts]
                ),
            {
                maxDelay: 10000,
                numOfAttempts: 3,
            }
        )
    } catch (e) {
        console.error(`Error retrieving simulations compute units from RPC:`, e)
        const continueByUsingHardcodedEstimate = await promptToContinue(
            'Failed to call simulateTransaction on the RPC. This can happen when the network is congested. Would you like to use hardcoded estimates (TransactionCuEstimates) ? This may result in slightly overpaying for the transaction.'
        )
        if (!continueByUsingHardcodedEstimate) {
            throw new Error(
                'Failed to call simulateTransaction on the RPC and user chose to not continue with hardcoded estimate.'
            )
        }
        console.log(
            `Falling back to hardcoded estimate for ${transactionType}: ${TransactionCuEstimates[transactionType]} CUs`
        )
        computeUnits = TransactionCuEstimates[transactionType]
    }

    if (!computeUnits) {
        throw new Error('Unable to compute units')
    }

    return {
        computeUnitPrice,
        computeUnits,
    }
}

export const addComputeUnitInstructions = async (
    connection: Connection,
    umi: Umi,
    eid: EndpointId,
    txBuilder: TransactionBuilder,
    umiWalletSigner: KeypairSigner,
    computeUnitPriceScaleFactor: number,
    transactionType: TransactionType,
    addressLookupTables?: (string | PublicKey)[]
) => {
    const computeUnitLimitScaleFactor = 1.1 // hardcoded to 1.1 as the estimations are not perfect and can fall slightly short of the actual CU usage on-chain

    const addressLookupTableInputs: AddressLookupTableInput[] = []
    const lookupTableAccounts: AddressLookupTableAccount[] = []

    if (addressLookupTables) {
        const lookupTableResults = await Promise.all(
            addressLookupTables.map((lookupTable) => getAddressLookupTable(lookupTable, connection, umi))
        )
        for (const { addressLookupTableInput, lookupTableAccount } of lookupTableResults) {
            addressLookupTableInputs.push(addressLookupTableInput)
            lookupTableAccounts.push(lookupTableAccount)
        }
    } else {
        const { addressLookupTableInput, lookupTableAccount } = await getDefaultAddressLookupTable(connection, umi, eid)
        addressLookupTableInputs.push(addressLookupTableInput)
        lookupTableAccounts.push(lookupTableAccount)
    }

    const { computeUnitPrice, computeUnits } = await getComputeUnitPriceAndLimit(
        connection,
        txBuilder.getInstructions(),
        umiWalletSigner,
        lookupTableAccounts,
        transactionType
    )
    // Since transaction builders are immutable, we must be careful to always assign the result of the add and prepend
    // methods to a new variable.
    const newTxBuilder = transactionBuilder()
        .add(
            setComputeUnitPrice(umi, {
                microLamports: computeUnitPrice * BigInt(Math.floor(computeUnitPriceScaleFactor)),
            })
        )
        .add(setComputeUnitLimit(umi, { units: computeUnits * computeUnitLimitScaleFactor }))
        .setAddressLookupTables(addressLookupTableInputs)
        .add(txBuilder)
    return newTxBuilder
}
