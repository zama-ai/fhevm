import { task, types } from 'hardhat/config'
import { HardhatRuntimeEnvironment } from 'hardhat/types'

import { TransferOwnershipProps, transferOwnershipAction } from '../utils/transferOwnership'

task('zama:oftadapter:transferOwnership', 'Transfer ownership of ZamaOFTAdapter')
    .addParam('address', 'New owner address', undefined, types.string)
    .addOptionalParam(
        'fromDeployment',
        'Fetch the address of the ZamaOFTAdapter contract from the existing deployments for the selected network.',
        false,
        types.boolean
    )
    .addOptionalParam(
        'contractAddress',
        'Address of the ZamaOFTAdapter contract to interact with. It not set, it fallback on ZAMAOFTADAPTER_CONTRACT_ADDRESS env variable.',
        undefined,
        types.string
    )
    .setAction(
        async (
            { address, fromDeployment, contractAddress }: TransferOwnershipProps,
            hre: HardhatRuntimeEnvironment
        ) => {
            await transferOwnershipAction('ZamaOFTAdapter', hre, address, fromDeployment, contractAddress)
        }
    )
