import { Contract } from 'ethers'
import { HardhatRuntimeEnvironment } from 'hardhat/types'

import { getBlockExplorerLink } from '../utils'

export interface OftAdapterContext {
    signer: Awaited<ReturnType<HardhatRuntimeEnvironment['ethers']['getSigners']>>[number]
    oftAdapter: Contract
    deploymentAddress: string
    networkName: string
}

export async function resolveOftAdapterContext(hre: HardhatRuntimeEnvironment): Promise<OftAdapterContext> {
    const { ethers, deployments, network } = hre

    const [signer] = await ethers.getSigners()
    if (!signer) {
        throw new Error('No signer available to execute the transaction. Configure accounts for this network.')
    }

    const deployment = await deployments.get('ZamaOFTAdapter').catch(() => {
        throw new Error(
            `Unable to find ZamaOFTAdapter deployment for network "${network.name}". Ensure you selected the correct network and deployed the contract.`
        )
    })

    const oftAdapter = await ethers.getContractAt('ZamaOFTAdapter', deployment.address, signer)

    return {
        signer,
        oftAdapter,
        deploymentAddress: deployment.address,
        networkName: network.name,
    }
}

export async function logExplorerLink(hre: HardhatRuntimeEnvironment, txHash: string): Promise<void> {
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
