import { HardhatRuntimeEnvironment } from 'hardhat/types'

import { endpointIdToNetwork } from '@layerzerolabs/lz-definitions'
import { Options } from '@layerzerolabs/lz-v2-utilities'

export const deploymentMetadataUrl = 'https://metadata.layerzero-api.com/v1/metadata/deployments'

/**
 * Given a srcEid and on-chain tx hash, return
 * `https://…blockExplorers[0].url/tx/<txHash>`, or undefined.
 */
export async function getBlockExplorerLink(srcEid: number, txHash: string): Promise<string | undefined> {
    const network = endpointIdToNetwork(srcEid) // e.g. "ethereum-mainnet"
    const res = await fetch(deploymentMetadataUrl)
    if (!res.ok) return
    const all = (await res.json()) as Record<string, { blockExplorers?: { url: string }[] }>
    const meta = all[network]
    const explorer = meta?.blockExplorers?.[0]?.url
    if (explorer) {
        // many explorers use `/tx/<hash>`
        return `${explorer.replace(/\/+$/, '')}/tx/${txHash}`
    }
    return
}

export async function logExplorerLink(hre: Pick<HardhatRuntimeEnvironment, 'network'>, txHash: string): Promise<void> {
    const { eid } = hre.network.config as { eid?: number }
    if (typeof eid !== 'number') {
        console.log('No endpoint ID configured for this network; unable to derive block explorer link.')
        return
    }

    try {
        const explorerLink = await getBlockExplorerLink(eid, txHash)
        if (explorerLink) {
            console.log(`Block explorer: ${explorerLink}`)
        } else {
            console.log('Block explorer URL unavailable for this network; check LayerZero metadata service.')
        }
    } catch (error) {
        console.log(`Failed to retrieve block explorer URL: ${error instanceof Error ? error.message : error}`)
    }
}

function formatBigIntForDisplay(n: bigint) {
    return n.toLocaleString().replace(/,/g, '_')
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

// Get LayerZero scan link
export function getLayerZeroScanLink(txHash: string, isTestnet = false): string {
    const baseUrl = isTestnet ? 'https://testnet.layerzeroscan.com' : 'https://layerzeroscan.com'
    return `${baseUrl}/tx/${txHash}`
}

export { DebugLogger, KnownErrors, KnownOutputs, KnownWarnings } from '@layerzerolabs/io-devtools'
