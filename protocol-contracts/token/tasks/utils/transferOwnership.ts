import { HardhatRuntimeEnvironment } from 'hardhat/types'

import { resolveContext } from './contractContext'
import { logExplorerLink } from './lz'

export interface TransferOwnershipProps {
    address: string
    fromDeployment: boolean
    contractAddress?: string
}

export const transferOwnershipAction = async (
    contractName: string,
    hre: HardhatRuntimeEnvironment,
    address: string,
    fromDeployment: boolean,
    contractAddress?: string
) => {
    if (!hre.ethers.utils.isAddress(address)) {
        throw new Error(`The provided owner address is not a valid EVM address: ${address}`)
    }

    const { signer, contract, deploymentAddress } = await resolveContext(
        contractName,
        hre,
        fromDeployment,
        contractAddress
    )

    if ((await contract.owner()) !== signer.address) {
        throw new Error(
            `The deployer account ${signer.address} is not the owner of the ${contractName} contract ${deploymentAddress}`
        )
    }

    console.log(
        `Transferring ownership to ${address} on ${contractName} ${deploymentAddress} using signer ${signer.address}`
    )

    const tx = await contract.transferOwnership(address)
    console.log(`Transaction submitted: ${tx.hash}`)

    const receipt = await tx.wait()
    console.log(`Ownership transferred in block ${receipt.blockNumber} (gas used: ${receipt.gasUsed.toString()})`)

    await logExplorerLink(hre, tx.hash)
}
