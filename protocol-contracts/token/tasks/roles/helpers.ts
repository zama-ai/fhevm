import { Contract } from 'ethers'
import { HardhatRuntimeEnvironment } from 'hardhat/types'

import { getBlockExplorerLink } from '../utils'

export type RoleKey = 'MINTER_ROLE' | 'PAUSING_MINTER_ROLE' | 'DEFAULT_ADMIN_ROLE'

export const ROLE_ACCESSORS: Record<RoleKey, string> = {
    MINTER_ROLE: 'MINTER_ROLE',
    PAUSING_MINTER_ROLE: 'MINTING_PAUSER_ROLE',
    DEFAULT_ADMIN_ROLE: 'DEFAULT_ADMIN_ROLE',
}

export const ROLE_TASK_SUFFIX: Record<RoleKey, string> = {
    MINTER_ROLE: 'minter_role',
    PAUSING_MINTER_ROLE: 'pausing_minter_role',
    DEFAULT_ADMIN_ROLE: 'default_admin_role',
}

export interface ZamaRoleContext {
    signer: Awaited<ReturnType<HardhatRuntimeEnvironment['ethers']['getSigners']>>[number]
    zamaErc20: Contract
    deploymentAddress: string
    networkName: string
}

export async function resolveZamaRoleContext(hre: HardhatRuntimeEnvironment): Promise<ZamaRoleContext> {
    const { ethers, deployments, network } = hre

    const signers = await ethers.getSigners()
    const signer = signers[0]
    if (!signer) {
        throw new Error('No signer available to execute the transaction. Configure accounts for this network.')
    }

    const zamaDeployment = await deployments.get('ZamaERC20').catch(() => {
        throw new Error(
            `Unable to find ZamaERC20 deployment for network "${network.name}". Make sure to select the right network, and deploy the contract before running this task.`
        )
    })

    const zamaErc20 = await ethers.getContractAt('ZamaERC20', zamaDeployment.address, signer)

    return {
        signer,
        zamaErc20,
        deploymentAddress: zamaDeployment.address,
        networkName: network.name,
    }
}

export async function resolveRoleValue(zamaErc20: Contract, role: RoleKey): Promise<string> {
    const accessor = ROLE_ACCESSORS[role]
    const getter = zamaErc20[accessor]
    if (typeof getter !== 'function') {
        throw new Error(`Role accessor ${accessor}() not available on ZamaERC20 contract`)
    }
    return getter()
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
