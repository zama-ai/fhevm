import { safeFetchMetadataFromSeeds } from '@metaplex-foundation/mpl-token-metadata'
import { fromWeb3JsPublicKey, toWeb3JsPublicKey } from '@metaplex-foundation/umi-web3js-adapters'
import { TOKEN_2022_PROGRAM_ID, getTokenMetadata } from '@solana/spl-token'
import { Connection, PublicKey } from '@solana/web3.js'

import { OmniPoint } from '@layerzerolabs/devtools'
import { createConnectedContractFactory } from '@layerzerolabs/devtools-evm-hardhat'
import { createSolanaConnectionFactory, createSolanaSignerFactory } from '@layerzerolabs/devtools-solana'
import { createLogger } from '@layerzerolabs/io-devtools'
import { ChainType, EndpointId, endpointIdToChainType, endpointIdToNetwork } from '@layerzerolabs/lz-definitions'
import { UlnProgram } from '@layerzerolabs/lz-solana-sdk-v2'
import { toWeb3Connection } from '@layerzerolabs/lz-solana-sdk-v2/umi'
import { Options } from '@layerzerolabs/lz-v2-utilities'
import { IOApp } from '@layerzerolabs/ua-devtools'
import { createOAppFactory } from '@layerzerolabs/ua-devtools-evm'
import { createOFTFactory } from '@layerzerolabs/ua-devtools-solana'

import { createAptosOAppFactory } from '../aptos'

import type { Umi, PublicKey as UmiPublicKey } from '@metaplex-foundation/umi'

export { createSolanaConnectionFactory }

const logger = createLogger()

type DeploymentMetadata = {
    blockExplorers?: Array<{ url?: string }>
    [key: string]: unknown
}

export const deploymentMetadataUrl = 'https://metadata.layerzero-api.com/v1/metadata/deployments'

export enum MSG_TYPE {
    SEND = 1,
    SEND_AND_CALL = 2,
}

/**
 * Given a srcEid and on-chain tx hash, return
 * `https://…blockExplorers[0].url/tx/<txHash>`, or undefined.
 */

export async function getBlockExplorerLink(srcEid: number, txHash: string): Promise<string | undefined> {
    const network = endpointIdToNetwork(srcEid) // e.g. "animechain-mainnet"
    const res = await fetch(deploymentMetadataUrl)
    if (!res.ok) return
    const all = (await res.json()) as Record<string, DeploymentMetadata>
    const meta = all[network]
    const explorer = meta?.blockExplorers?.[0]?.url
    if (explorer) {
        // many explorers use `/tx/<hash>`
        return `${explorer.replace(/\/+$/, '')}/tx/${txHash}`
    }
    return
}

export const createSdkFactory = (
    userAccount: PublicKey,
    programId: PublicKey,
    connectionFactory = createSolanaConnectionFactory()
) => {
    // To create a EVM/Solana SDK factory we need to merge the EVM and the Solana factories into one
    const evmSdkFactory = createOAppFactory(createConnectedContractFactory())
    const aptosSdkFactory = createAptosOAppFactory()
    const solanaSdkFactory = createOFTFactory(
        // The first parameter to createOFTFactory is a user account factory
        //
        // This is a function that receives an OmniPoint ({ eid, address } object)
        // and returns a user account to be used with that SDK.
        //
        // For our purposes this will always be the user account coming from the secret key passed in
        () => userAccount,
        // The second parameter is a program ID factory
        //
        // This is a function that receives an OmniPoint ({ eid, address } object)
        // and returns a program ID to be used with that SDK.
        //
        // Since we only have one OFT deployed, this will always be the program ID passed as a CLI parameter.
        //
        // In situations where we might have multiple configs with OFTs using multiple program IDs,
        // this function needs to decide which one to use.
        () => programId,
        // Last but not least the SDK will require a connection
        connectionFactory
    )

    // the return value is an SDK factory that receives an OmniPoint and returns an SDK
    return async (point: OmniPoint): Promise<IOApp> => {
        if (endpointIdToChainType(point.eid) === ChainType.SOLANA) {
            return solanaSdkFactory(point)
        } else if (endpointIdToChainType(point.eid) === ChainType.EVM) {
            return evmSdkFactory(point)
        } else if (
            endpointIdToChainType(point.eid) === ChainType.APTOS ||
            endpointIdToChainType(point.eid) === ChainType.INITIA
        ) {
            return aptosSdkFactory(point)
        } else {
            logger.error(`Unsupported chain type for EID ${point.eid}`)
            throw new Error(`Unsupported chain type for EID ${point.eid}`)
        }
    }
}

export { createSolanaSignerFactory }

export function uint8ArrayToHex(uint8Array: Uint8Array, prefix = false): string {
    const hexString = Buffer.from(uint8Array).toString('hex')
    return prefix ? `0x${hexString}` : hexString
}

function formatBigIntForDisplay(n: bigint) {
    return n.toLocaleString().replace(/,/g, '_')
}

export function isEmptyOptionsEvm(optionsHex?: string): boolean {
    return !optionsHex || optionsHex === '0x' || optionsHex === '0x0003' // 0x0003 is an empty options type 3
}

export function isEmptyOptionsSolana(optionsBytes?: Uint8Array): boolean {
    if (!optionsBytes) {
        return true // Treat undefined or null as empty options
    }
    return Buffer.from(optionsBytes).toString('hex') === '0003' // empty options type 3 without 0x prefix
}

export function decodeLzReceiveOptions(hex: string): string {
    try {
        // Handle empty/undefined values first
        if (!hex || hex === '0x') return 'No options set'
        const options = Options.fromOptions(hex)
        const lzReceiveOpt = options.decodeExecutorLzReceiveOption()
        return lzReceiveOpt
            ? `gas: ${formatBigIntForDisplay(lzReceiveOpt.gas)} , value: ${formatBigIntForDisplay(lzReceiveOpt.value)} wei`
            : 'No executor options'
    } catch (e) {
        return `Invalid options (${hex.slice(0, 12)}...)`
    }
}

export async function getSolanaUlnConfigPDAs(
    remote: EndpointId,
    connection: Connection,
    ulnAddress: PublicKey,
    oftStore: PublicKey
) {
    const uln = new UlnProgram.Uln(new PublicKey(ulnAddress))
    const sendConfig = uln.getSendConfigState(connection, new PublicKey(oftStore), remote)

    const receiveConfig = uln.getReceiveConfigState(connection, new PublicKey(oftStore), remote)

    return await Promise.all([sendConfig, receiveConfig])
}

export enum SolanaTokenProgramType {
    SPL = 'SPL',
    Token2022 = 'Token2022',
}

type TokenMetadata = {
    updateAuthority?: UmiPublicKey
    isMutable?: boolean
    name?: string
    symbol?: string
}

export async function getSolanaTokenMetadata(
    umi: Umi,
    mint: UmiPublicKey,
    tokenProgramType: SolanaTokenProgramType
): Promise<TokenMetadata> {
    let response
    switch (tokenProgramType) {
        case SolanaTokenProgramType.SPL:
            response = await safeFetchMetadataFromSeeds(umi, { mint })
            return {
                updateAuthority: response?.updateAuthority,
                isMutable: response?.isMutable,
                name: response?.name,
                symbol: response?.symbol,
            }
        case SolanaTokenProgramType.Token2022:
            response = await getTokenMetadata(
                toWeb3Connection(umi.rpc),
                toWeb3JsPublicKey(mint),
                'confirmed',
                TOKEN_2022_PROGRAM_ID
            )
            return {
                updateAuthority: response?.updateAuthority ? fromWeb3JsPublicKey(response?.updateAuthority) : undefined,
                // If the update authority is renounced via setting the update authority to the System Program address
                // note: https://docs.rs/spl-token-metadata-interface/0.8.0/spl_token_metadata_interface/state/struct.TokenMetadata.html
                isMutable: response?.updateAuthority != PublicKey.default,
                name: response?.name,
                symbol: response?.symbol,
            }
        default:
            throw new Error(`Unsupported token program type: ${tokenProgramType}`)
    }
}

export { createLogger, DebugLogger, KnownErrors, KnownOutputs, KnownWarnings } from '@layerzerolabs/io-devtools'
