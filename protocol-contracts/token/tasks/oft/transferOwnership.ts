import { task, types } from 'hardhat/config'

import { resolveContext } from '../utils/contractContext'
import { logExplorerLink } from '../utils/lz'

task('zama:oft:transferOwnership', 'Transfer ownership of ZamaOFT')
    .addParam('address', 'New owner address', undefined, types.string)
    .addOptionalParam(
        'contractAddress',
        'Address of the ZamaOFT contract to interact with. It not set, it fallback on ZAMAOFT_CONTRACT_ADDRESS env variable.',
        undefined,
        types.string
    )
    .setAction(async ({ address, contractAddress }, hre) => {
        if (!hre.ethers.utils.isAddress(address)) {
            throw new Error(`The provided owner address is not a valid EVM address: ${address}`)
        }

        const { signer, contract, deploymentAddress } = await resolveContext('ZamaOFT', hre, contractAddress)

        if ((await contract.owner()) !== signer.address) {
            throw new Error(
                `The deployer account ${signer.address} is not the owner of the ZamaOFT contract ${deploymentAddress}`
            )
        }

        console.log(
            `Transferring ownership to ${address} on ZamaOFT ${deploymentAddress} using signer ${signer.address}`
        )

        const tx = await contract.transferOwnership(address)
        console.log(`Transaction submitted: ${tx.hash}`)

        const receipt = await tx.wait()
        console.log(`Ownership transferred in block ${receipt.blockNumber} (gas used: ${receipt.gasUsed.toString()})`)

        await logExplorerLink(hre, tx.hash)
    })
