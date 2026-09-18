import { fetchMint } from '@metaplex-foundation/mpl-toolbox'
import { PublicKey as UmiPublicKey, publicKey, unwrapOption } from '@metaplex-foundation/umi'
import { fromWeb3JsPublicKey, toWeb3JsPublicKey } from '@metaplex-foundation/umi-web3js-adapters'
import { TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from '@solana/spl-token'
import { Keypair, PublicKey } from '@solana/web3.js'
import { task } from 'hardhat/config'

import { OmniPoint, denormalizePeer } from '@layerzerolabs/devtools'
import { types } from '@layerzerolabs/devtools-evm-hardhat'
import { isSquadsV4Vault } from '@layerzerolabs/devtools-solana'
import { EndpointId, getNetworkForChainId } from '@layerzerolabs/lz-definitions'
import { EndpointPDADeriver, EndpointProgram } from '@layerzerolabs/lz-solana-sdk-v2'
import { EndpointProgram as EndpointProgramUmi } from '@layerzerolabs/lz-solana-sdk-v2/umi'
import { IMetadata, defaultFetchMetadata } from '@layerzerolabs/metadata-tools'
import { OftPDA, oft } from '@layerzerolabs/oft-v2-solana-sdk'
import { EndpointV2 } from '@layerzerolabs/protocol-devtools-solana'

import { getSolanaReceiveConfig, getSolanaSendConfig } from '../common/taskHelper'
import {
    DebugLogger,
    SolanaTokenProgramType,
    createSolanaConnectionFactory,
    decodeLzReceiveOptions,
    getSolanaTokenMetadata,
    uint8ArrayToHex,
} from '../common/utils'

import { deriveConnection, getSolanaDeployment } from './index'

const DEBUG_ACTIONS = {
    OFT_STORE: 'oft-store',
    GET_ADMIN: 'admin',
    GET_DELEGATE: 'delegate',
    CHECKS: 'checks',
    GET_TOKEN: 'token',
    GET_PEERS: 'peers',
    RATE_LIMITS: 'rate-limits',
}

/**
 * Get the OFTStore account from the task arguments, the deployment file, or throw an error.
 * @param {EndpointId} eid
 * @param {string} oftStore
 */
const getOftStore = (eid: EndpointId, oftStore?: string) => publicKey(oftStore ?? getSolanaDeployment(eid).oftStore)

function getChainKeyForEid(metadata: IMetadata, eid: number): string {
    const eidStr = String(eid)
    for (const objectKey in metadata) {
        const entry = metadata[objectKey]
        if (typeof entry?.deployments !== 'undefined') {
            for (const deployment of entry.deployments) {
                if (deployment.eid === eidStr) {
                    return deployment.chainKey
                }
            }
        }
    }
    throw new Error(`Can't find chainKey for eid: "${eid}".`)
}

function formatDvnAddresses(addresses: string[], metadata?: IMetadata, chainKey?: string): string {
    const dvnMap = (chainKey && metadata ? metadata[chainKey]?.dvns : undefined) as
        | Record<string, { canonicalName?: string }>
        | undefined
    return addresses.map((addr) => dvnMap?.[addr]?.canonicalName ?? addr).join(', ')
}

function tokenProgramAddressToType(tokenProgramAddress: string | PublicKey): SolanaTokenProgramType {
    const address = typeof tokenProgramAddress === 'string' ? new PublicKey(tokenProgramAddress) : tokenProgramAddress
    const tokenProgramMap: Record<string, SolanaTokenProgramType> = {
        [TOKEN_PROGRAM_ID.toBase58()]: SolanaTokenProgramType.SPL,
        [TOKEN_2022_PROGRAM_ID.toBase58()]: SolanaTokenProgramType.Token2022,
    }
    const addressStr = address.toBase58()
    const name = tokenProgramMap[addressStr]
    if (!name) {
        throw new Error(`Invalid token program address: ${addressStr}. Expected either SPL Token or Token2022.`)
    }
    return name
}

type DebugTaskArgs = {
    eid: EndpointId
    oftStore?: string
    endpoint: string
    dstEids: EndpointId[]
    action?: string
}

task('lz:oft:solana:debug', 'Manages OFTStore and OAppRegistry information')
    .addParam(
        'eid',
        'Solana mainnet (30168) or testnet (40168).  Defaults to mainnet.',
        EndpointId.SOLANA_V2_MAINNET,
        types.eid
    )
    .addParam(
        'oftStore',
        'The OFTStore public key. Derived from deployments if not provided.',
        undefined,
        types.string,
        true
    )
    .addParam('endpoint', 'The Endpoint public key', EndpointProgram.PROGRAM_ID.toBase58(), types.string)
    .addOptionalParam('dstEids', 'Destination eids to check (comma-separated list)', [], types.csv)
    .addOptionalParam(
        'action',
        `The action to perform: ${Object.keys(DEBUG_ACTIONS).join(', ')} (defaults to all)`,
        undefined,
        types.string
    )
    .setAction(async (taskArgs: DebugTaskArgs) => {
        const { eid, oftStore: oftStoreArg, endpoint, dstEids, action } = taskArgs
        const { umi, connection } = await deriveConnection(eid, true)
        const oftStore = getOftStore(eid, oftStoreArg)
        const metadata = await defaultFetchMetadata()
        const sourceChainKey = getChainKeyForEid(metadata, eid)

        let oftStoreInfo
        try {
            oftStoreInfo = await oft.accounts.fetchOFTStore(umi, oftStore)
        } catch (e) {
            console.error(`Failed to fetch OFTStore at ${oftStore.toString()}:`, e)
            return
        }
        const nonceAccountChecksInfo: Partial<
            Record<EndpointId, { data: EndpointProgramUmi.accounts.NonceAccountData; address: UmiPublicKey }>
        > = {}

        const mintAccount = await fetchMint(umi, publicKey(oftStoreInfo.tokenMint))
        const tokenProgramType = tokenProgramAddressToType(mintAccount.header.owner)

        const epDeriver = new EndpointPDADeriver(new PublicKey(endpoint))
        const [oAppRegistry] = epDeriver.oappRegistry(toWeb3JsPublicKey(oftStore))
        const oAppRegistryInfo = await EndpointProgram.accounts.OAppRegistry.fromAccountAddress(
            connection,
            oAppRegistry
        )

        if (!oAppRegistryInfo) {
            console.warn('OAppRegistry info not found.')
            return
        }

        const oftDeriver = new OftPDA(oftStoreInfo.header.owner)

        const tokenMetadata = await getSolanaTokenMetadata(umi, publicKey(oftStoreInfo.tokenMint), tokenProgramType)

        // Note: isSquadsV4Vault only works on mainnet
        const adminIsSquadsV4Vault = await isSquadsV4Vault(eid as number, oftStoreInfo.admin)
        const delegateIsSquadsV4Vault = await isSquadsV4Vault(eid as number, oAppRegistryInfo?.delegate?.toBase58())

        const printOftStore = async () => {
            DebugLogger.header('OFT Store Information')
            DebugLogger.keyValue('OFT Program', oftStoreInfo.header.owner)
            DebugLogger.keyValue('OFT Type', oft.types.OFTType[oftStoreInfo.oftType])
            DebugLogger.keyValue('Admin', oftStoreInfo.admin)
            DebugLogger.keyValue('Token Mint', oftStoreInfo.tokenMint)
            DebugLogger.keyValue('Token Escrow', oftStoreInfo.tokenEscrow)
            DebugLogger.keyValue('Endpoint Program', oftStoreInfo.endpointProgram)
            DebugLogger.separator()
        }

        const printAdmin = async () => {
            const admin = oftStoreInfo.admin
            DebugLogger.keyValue('Admin', admin)
        }

        const printDelegate = async () => {
            const delegate = oAppRegistryInfo?.delegate?.toBase58()
            DebugLogger.header('OApp Registry Information')
            DebugLogger.keyValue('Delegate', delegate)
            DebugLogger.separator()
        }

        const printToken = async () => {
            DebugLogger.header('Token Information')
            DebugLogger.keyValue('Mint Address', oftStoreInfo.tokenMint)
            DebugLogger.keyValue('Token Name', tokenMetadata?.name ?? 'N/A')
            DebugLogger.keyValue('Token Symbol', tokenMetadata?.symbol ?? 'N/A')
            DebugLogger.keyValue('Token Program', tokenProgramType)
            DebugLogger.keyValue('Mint Authority', unwrapOption(mintAccount.mintAuthority))
            DebugLogger.keyValue(
                'Freeze Authority',
                unwrapOption(mintAccount.freezeAuthority, () => 'None')
            )
            DebugLogger.keyValue('Update Authority', tokenMetadata?.updateAuthority ?? 'N/A (no Metaplex Metadata)')
            DebugLogger.keyValue('Metadata is mutable', tokenMetadata?.isMutable ?? 'N/A')
            DebugLogger.separator()
        }

        const printChecks = async () => {
            const delegate = oAppRegistryInfo?.delegate?.toBase58()

            DebugLogger.header('Checks')
            DebugLogger.keyValue('Admin (Owner) same as Delegate', oftStoreInfo.admin === delegate)
            DebugLogger.keyValue(
                'Token Mint Authority is OFT Store',
                unwrapOption(mintAccount.mintAuthority) === oftStore
            )
            DebugLogger.keyValue('Admin is Squads V4 Vault', adminIsSquadsV4Vault)
            DebugLogger.keyValue('Delegate is Squads V4 Vault', delegateIsSquadsV4Vault)
            dstEids.map((dstEid) => {
                DebugLogger.keyHeader(`Nonce Account Checks`)
                const nonceAccountCheckInfo = nonceAccountChecksInfo[dstEid]
                if (nonceAccountCheckInfo) {
                    const definedForDstEid = !!nonceAccountCheckInfo.data
                    DebugLogger.keyValue(
                        `Defined for ${dstEid} (${getNetworkForChainId(dstEid).chainName})`,
                        definedForDstEid,
                        2
                    )
                    if (!definedForDstEid) {
                        console.warn(
                            `Expected Nonce Account to exist at ${nonceAccountCheckInfo.address.toString()} for destination ${dstEid} (${getNetworkForChainId(dstEid).chainName}).`
                        )
                    }
                }
            })
            DebugLogger.separator()
        }

        let peerConfigsCache: {
            peerConfigs: UmiPublicKey[]
            peerConfigInfos: Awaited<ReturnType<typeof oft.accounts.safeFetchAllPeerConfig>>
            endpointV2Sdk: EndpointV2
        } | null = null
        const fetchPeerConfigsAndSdk = async () => {
            if (peerConfigsCache) {
                return peerConfigsCache
            }

            const peerConfigs = dstEids.map((dstEid) => {
                const peerConfig = oftDeriver.peer(oftStore, dstEid)
                return publicKey(peerConfig)
            })
            const mockKeypair = new Keypair()
            const point: OmniPoint = {
                eid,
                address: oftStore.toString(),
            }
            const endpointV2Sdk = new EndpointV2(
                await createSolanaConnectionFactory()(eid),
                point,
                mockKeypair.publicKey // doesn't matter as we are not sending transactions
            )

            const peerConfigInfos = await oft.accounts.safeFetchAllPeerConfig(umi, peerConfigs)

            peerConfigsCache = { peerConfigs, peerConfigInfos, endpointV2Sdk }
            return peerConfigsCache
        }

        const printPeerConfigs = async () => {
            const { peerConfigs, peerConfigInfos, endpointV2Sdk } = await fetchPeerConfigsAndSdk()

            DebugLogger.header('Peer Configurations')
            for (let index = 0; index < dstEids.length; index++) {
                const dstEid = dstEids[index]
                const info = peerConfigInfos[index]
                const network = getNetworkForChainId(dstEid)
                const oAppReceiveConfig = await getSolanaReceiveConfig(endpointV2Sdk, dstEid, oftStore)
                const oAppSendConfig = await getSolanaSendConfig(endpointV2Sdk, dstEid, oftStore)
                // Show the chain info
                DebugLogger.header(`${dstEid} (${network.chainName})`)

                if (info) {
                    // nonce account
                    const nonceAccount = epDeriver.nonce(toWeb3JsPublicKey(oftStore), dstEid, info.peerAddress)[0]
                    const nonceAccountInfo = await EndpointProgramUmi.accounts.fetchNonce(
                        umi,
                        fromWeb3JsPublicKey(nonceAccount)
                    )
                    nonceAccountChecksInfo[dstEid] = {
                        data: nonceAccountInfo,
                        address: fromWeb3JsPublicKey(nonceAccount),
                    }
                    // Existing PeerConfig info
                    DebugLogger.keyValue('PeerConfig Account', peerConfigs[index].toString())
                    DebugLogger.keyValue('Peer Address', denormalizePeer(info.peerAddress, dstEid))
                    DebugLogger.keyValue('Nonce Account', nonceAccount.toString())
                    DebugLogger.keyHeader('Enforced Options')
                    DebugLogger.keyValue(
                        'Send',
                        decodeLzReceiveOptions(uint8ArrayToHex(info.enforcedOptions.send, true)),
                        2
                    )
                    DebugLogger.keyValue(
                        'SendAndCall',
                        decodeLzReceiveOptions(uint8ArrayToHex(info.enforcedOptions.sendAndCall, true)),
                        2
                    )

                    printOAppReceiveConfigs(oAppReceiveConfig, network.chainName, metadata, sourceChainKey)
                    printOAppSendConfigs(oAppSendConfig, network.chainName, metadata, sourceChainKey)
                } else {
                    // No PeerConfig account
                    console.log(`No PeerConfig account found for ${dstEid} (${network.chainName}).`)
                }

                DebugLogger.separator()
            }
        }

        const printRateLimits = async () => {
            const { peerConfigInfos } = await fetchPeerConfigsAndSdk()

            DebugLogger.header('Rate Limits')

            const sourceNetwork = getNetworkForChainId(eid)

            for (let index = 0; index < dstEids.length; index++) {
                const dstEid = dstEids[index]
                const info = peerConfigInfos[index]
                const network = getNetworkForChainId(dstEid)

                DebugLogger.header(`${dstEid} (${network.chainName})`)

                if (info) {
                    const { outboundRateLimiter, inboundRateLimiter } = info
                    printRateLimitsForPeer(
                        outboundRateLimiter,
                        inboundRateLimiter,
                        sourceNetwork.chainName,
                        network.chainName
                    )
                } else {
                    DebugLogger.keyValue('PeerConfig', 'Not found', 1)
                }

                DebugLogger.separator()
            }
        }
        if (action) {
            switch (action) {
                case DEBUG_ACTIONS.OFT_STORE:
                    await printOftStore()
                    break
                case DEBUG_ACTIONS.GET_ADMIN:
                    await printAdmin()
                    break
                case DEBUG_ACTIONS.GET_DELEGATE:
                    await printDelegate()
                    break
                case DEBUG_ACTIONS.CHECKS:
                    await printChecks()
                    break
                case DEBUG_ACTIONS.GET_TOKEN:
                    await printToken()
                    break
                case DEBUG_ACTIONS.GET_PEERS:
                    await printPeerConfigs()
                    break
                case DEBUG_ACTIONS.RATE_LIMITS:
                    await printRateLimits()
                    break
                default:
                    console.error(`Invalid action specified. Use any of ${Object.keys(DEBUG_ACTIONS)}.`)
            }
        } else {
            const tasks = [printOftStore(), printDelegate(), printToken()]
            if (dstEids.length > 0) tasks.push(printPeerConfigs())
            await Promise.all(tasks)
            // printChecks might depend on other tasks, so we don't add it to the tasks array
            await printChecks()
        }
    })

function printOAppReceiveConfigs(
    oAppReceiveConfig: Awaited<ReturnType<typeof getSolanaReceiveConfig>>,
    peerChainName: string,
    metadata?: IMetadata,
    chainKey?: string
) {
    const oAppReceiveConfigIndexesToKeys: Record<number, string> = {
        0: 'receiveLibrary',
        1: 'receiveUlnConfig',
        2: 'receiveLibraryTimeoutConfig',
    }

    if (!oAppReceiveConfig) {
        console.log('No receive configs found.')
        return
    }

    DebugLogger.keyValue(`Receive Configs (${peerChainName} to solana)`, '')
    for (let i = 0; i < oAppReceiveConfig.length; i++) {
        const item = oAppReceiveConfig[i]
        if (typeof item === 'object' && item !== null) {
            // Print each property in the object
            DebugLogger.keyValue(`${oAppReceiveConfigIndexesToKeys[i]}`, '', 2)
            for (const [propKey, propVal] of Object.entries(item)) {
                const valueDisplay =
                    (propKey === 'requiredDVNs' || propKey === 'optionalDVNs') && Array.isArray(propVal)
                        ? formatDvnAddresses(propVal as string[], metadata, chainKey)
                        : String(propVal)
                DebugLogger.keyValue(`${propKey}`, valueDisplay, 3)
            }
        } else {
            // Print a primitive (string, number, etc.)
            DebugLogger.keyValue(`${oAppReceiveConfigIndexesToKeys[i]}`, String(item), 2)
        }
    }
}

function printOAppSendConfigs(
    oAppSendConfig: Awaited<ReturnType<typeof getSolanaSendConfig>>,
    peerChainName: string,
    metadata?: IMetadata,
    chainKey?: string
) {
    const sendOappConfigIndexesToKeys: Record<number, string> = {
        0: 'sendLibrary',
        1: 'sendUlnConfig',
        2: 'sendExecutorConfig',
    }

    if (!oAppSendConfig) {
        console.log('No send configs found.')
        return
    }

    DebugLogger.keyValue(`Send Configs (solana to ${peerChainName})`, '')
    for (let i = 0; i < oAppSendConfig.length; i++) {
        const item = oAppSendConfig[i]
        if (typeof item === 'object' && item !== null) {
            DebugLogger.keyValue(`${sendOappConfigIndexesToKeys[i]}`, '', 2)
            for (const [propKey, propVal] of Object.entries(item)) {
                const valueDisplay =
                    (propKey === 'requiredDVNs' || propKey === 'optionalDVNs') && Array.isArray(propVal)
                        ? formatDvnAddresses(propVal as string[], metadata, chainKey)
                        : String(propVal)
                DebugLogger.keyValue(`${propKey}`, valueDisplay, 3)
            }
        } else {
            DebugLogger.keyValue(`${sendOappConfigIndexesToKeys[i]}`, String(item), 2)
        }
    }
}

type RateLimiter = {
    __option: 'Some' | 'None'
    value?: {
        capacity: bigint
        tokens: bigint
        refillPerSecond: bigint
        lastRefillTime: bigint
    }
}

function formatUnixSecondsToUtc(secs: bigint): string {
    const millis = Number(secs) * 1000
    const iso = new Date(millis).toISOString()
    return iso.replace('T', ' ').replace('.000Z', ' UTC')
}

function printSingleRateLimiter(label: string, limiter: RateLimiter | null | undefined) {
    DebugLogger.keyValue(label, '', 1)

    if (!limiter || limiter.__option !== 'Some' || !limiter.value) {
        DebugLogger.keyValue('status', 'None', 2)
        return
    }

    const { capacity, tokens, refillPerSecond, lastRefillTime } = limiter.value

    DebugLogger.keyValue('capacity', String(capacity), 2)
    DebugLogger.keyValue('tokens (available allowance)', String(tokens), 2)
    DebugLogger.keyValue('refillPerSecond', String(refillPerSecond), 2)
    DebugLogger.keyValue('lastRefillTime', formatUnixSecondsToUtc(lastRefillTime), 2)
}

function printRateLimitsForPeer(
    outboundRateLimiter: RateLimiter | null | undefined,
    inboundRateLimiter: RateLimiter | null | undefined,
    sourceChainName: string,
    destinationChainName: string
) {
    DebugLogger.keyValue('Rate Limits', '')
    printSingleRateLimiter(`Outbound (${sourceChainName} to ${destinationChainName})`, outboundRateLimiter)
    printSingleRateLimiter(`Inbound (${destinationChainName} to ${sourceChainName})`, inboundRateLimiter)
}
