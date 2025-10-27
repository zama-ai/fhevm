import { task, types } from 'hardhat/config'

import { logExplorerLink } from '../utils'

import { resolveOftAdapterContext } from './helpers'

task('zama:oftadapter:transferOwnership', 'Transfer ownership of ZamaOFTAdapter')
    .addParam('address', 'New owner address', undefined, types.string)
    .setAction(async ({ address }, hre) => {
        if (!hre.ethers.utils.isAddress(address)) {
            throw new Error(`The provided owner address is not a valid EVM address: ${address}`)
        }

        const { signer, oftAdapter, deploymentAddress } = await resolveOftAdapterContext(hre)

        console.log(
            `Transferring ownership to ${address} on ZamaOFTAdapter ${deploymentAddress} using signer ${signer.address}`
        )

        const tx = await oftAdapter.transferOwnership(address)
        console.log(`Transaction submitted: ${tx.hash}`)

        const receipt = await tx.wait()
        console.log(`Ownership transferred in block ${receipt.blockNumber} (gas used: ${receipt.gasUsed.toString()})`)

        await logExplorerLink(hre, tx.hash)
    })
