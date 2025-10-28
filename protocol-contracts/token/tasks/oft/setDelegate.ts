import { task, types } from 'hardhat/config'

import { resolveContext } from '../utils/contractContext'
import { logExplorerLink } from '../utils/lz'

task('zama:oft:setDelegate', 'Set the delegate for ZamaOFT')
    .addParam('address', 'New delegate address', undefined, types.string)
    .addOptionalParam(
        'fromDeployment',
        'Fetch the address of the ZamaOFT contract from the existing deployments for the selected network.',
        false,
        types.boolean
    )
    .addOptionalParam(
        'contractAddress',
        'Address of the ZamaOFT contract to interact with. It not set, it fallback on ZAMAOFT_CONTRACT_ADDRESS env variable.',
        undefined,
        types.string
    )
    .setAction(async ({ address, fromDeployment, contractAddress }, hre) => {
        if (!hre.ethers.utils.isAddress(address)) {
            throw new Error(`The provided delegate address is not a valid EVM address: ${address}`)
        }

        const { signer, contract, deploymentAddress } = await resolveContext(
            'ZamaOFT',
            hre,
            fromDeployment,
            contractAddress
        )

        if ((await contract.owner()) !== signer.address) {
            throw new Error(
                `The deployer account ${signer.address} is not the owner of the ZamaOFT contract ${deploymentAddress}`
            )
        }

        console.log(`Setting delegate to ${address} on ZamaOFT ${deploymentAddress} using signer ${signer.address}`)

        const tx = await contract.setDelegate(address)
        console.log(`Transaction submitted: ${tx.hash}`)

        const receipt = await tx.wait()
        console.log(`Delegate updated in block ${receipt.blockNumber} (gas used: ${receipt.gasUsed.toString()})`)

        await logExplorerLink(hre, tx.hash)
    })
