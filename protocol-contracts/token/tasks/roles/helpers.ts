import { Contract } from 'ethers'
import { HardhatRuntimeEnvironment } from 'hardhat/types'

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
