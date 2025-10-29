import { ethers } from 'ethers'
import { task, types } from 'hardhat/config'

interface MasterArgs {
    address: string
}

task('evm:validate:address', 'Sends OFT tokens cross‐chain from EVM chains')
    .addParam('address', 'Address to validate', undefined, types.string)
    .setAction(async (args: MasterArgs) => {
        if (!ethers.utils.isAddress(args.address)) {
            throw new Error(`The provided address is not a valid EVM address: ${args.address}`)
        }

        console.log(`${args.address} is a valid EVM address.`)
    })
