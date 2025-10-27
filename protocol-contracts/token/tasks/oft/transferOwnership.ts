import { task, types } from 'hardhat/config'

import { logExplorerLink } from '../utils'

import { resolveOftContext } from './helpers'

task('zama:oft:transferOwnership', 'Transfer ownership of ZamaOFT')
    .addParam('address', 'New owner address', undefined, types.string)
    .setAction(async ({ address }, hre) => {
        if (!hre.ethers.utils.isAddress(address)) {
            throw new Error(`The provided owner address is not a valid EVM address: ${address}`)
        }

        const { signer, oft, deploymentAddress } = await resolveOftContext(hre)

        console.log(
            `Transferring ownership to ${address} on ZamaOFT ${deploymentAddress} using signer ${signer.address}`
        )

        const tx = await oft.transferOwnership(address)
        console.log(`Transaction submitted: ${tx.hash}`)

        const receipt = await tx.wait()
        console.log(`Ownership transferred in block ${receipt.blockNumber} (gas used: ${receipt.gasUsed.toString()})`)

        await logExplorerLink(hre, tx.hash)
    })
