import { SignerWithAddress } from '@nomiclabs/hardhat-ethers/signers'
import { Contract } from 'ethers'
import { HardhatRuntimeEnvironment } from 'hardhat/types'

import { getRequiredEnvVar } from './loadVariables'

export interface ContractContext {
    signer: SignerWithAddress
    contract: Contract
    deploymentAddress: string
    networkName: string
}

export async function resolveContext(
    contractName: string,
    hre: HardhatRuntimeEnvironment,
    fromDeployment: boolean,
    contractAddress?: string
): Promise<ContractContext> {
    const { ethers, deployments, network } = hre

    const [signer] = await ethers.getSigners()
    if (!signer) {
        throw new Error('No signer available to execute the transaction. Configure accounts for this network.')
    }

    if (fromDeployment && contractAddress) {
        throw new Error(
            `You cannot fetch the contract address from deployment and specify a contract address as well. Use either one.`
        )
    }

    let contract: Contract
    let deploymentAddress: string

    if (fromDeployment) {
        const deployment = await deployments.get(contractName).catch(() => {
            throw new Error(
                `Unable to find ${contractName} deployment for network "${network.name}". Ensure you selected the correct network and deployed the contract.`
            )
        })

        deploymentAddress = deployment.address
        contract = await ethers.getContractAt(contractName, deploymentAddress, signer)
    } else {
        deploymentAddress = contractAddress ?? getRequiredEnvVar(`${contractName.toUpperCase()}_CONTRACT_ADDRESS`)
        if (!hre.ethers.utils.isAddress(deploymentAddress)) {
            throw new Error(
                `The provided deployment address of ${contractName} is not a valid EVM address: ${deploymentAddress}`
            )
        }
        contract = await ethers.getContractAt(contractName, deploymentAddress).catch(() => {
            throw new Error(
                `Unable to find ${contractName} deployment for network "${network.name}".
                Ensure you selected the correct network and deployed the contract.`
            )
        })
    }

    return {
        signer,
        contract,
        deploymentAddress,
        networkName: network.name,
    }
}
