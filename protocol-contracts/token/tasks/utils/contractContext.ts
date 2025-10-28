import { Contract } from 'ethers'
import { HardhatRuntimeEnvironment } from 'hardhat/types'

export interface ContractContext {
    signer: Awaited<ReturnType<HardhatRuntimeEnvironment['ethers']['getSigners']>>[number]
    contract: Contract
    deploymentAddress: string
    networkName: string
}

import { getRequiredEnvVar } from './loadVariables'

export async function resolveContext(contractName: string, hre: HardhatRuntimeEnvironment): Promise<ContractContext> {
    const { ethers, network } = hre

    const [signer] = await ethers.getSigners()
    if (!signer) {
        throw new Error('No signer available to execute the transaction. Configure accounts for this network.')
    }

    const contractAddress = getRequiredEnvVar(`${contractName.toUpperCase()}_CONTRACT_ADDRESS`)
    const contract = await ethers.getContractAt(contractName, contractAddress).catch(() => {
        throw new Error(
            `Unable to find ${contractName} deployment for network "${network.name}".
            Ensure you selected the correct network and deployed the contract.`
        )
    })

    return {
        signer,
        contract,
        deploymentAddress: contractAddress,
        networkName: network.name,
    }
}
