import { Contract } from 'ethers'
import { HardhatRuntimeEnvironment } from 'hardhat/types'

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
