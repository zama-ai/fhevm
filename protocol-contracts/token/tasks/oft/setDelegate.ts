import { task, types } from 'hardhat/config'

import { logExplorerLink, resolveOftContext } from './helpers'

task('zama:oft:setdelegate', 'Set the delegate for ZamaOFT')
    .addParam('address', 'New delegate address', undefined, types.string)
    .setAction(async ({ address }, hre) => {
        if (!hre.ethers.utils.isAddress(address)) {
            throw new Error(`The provided delegate address is not a valid EVM address: ${address}`)
        }

        const { signer, oft, deploymentAddress } = await resolveOftContext(hre)

        console.log(`Setting delegate to ${address} on ZamaOFT ${deploymentAddress} using signer ${signer.address}`)

        const tx = await oft.setDelegate(address)
        console.log(`Transaction submitted: ${tx.hash}`)

        const receipt = await tx.wait()
        console.log(`Delegate updated in block ${receipt.blockNumber} (gas used: ${receipt.gasUsed.toString()})`)

        await logExplorerLink(hre, tx.hash)
    })
