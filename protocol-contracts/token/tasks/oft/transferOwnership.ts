import { task, types } from 'hardhat/config'
import { HardhatRuntimeEnvironment } from 'hardhat/types'

import { TransferOwnershipProps, transferOwnershipAction } from '../utils/transferOwnership'

task('zama:oft:transferOwnership', 'Transfer ownership of ZamaOFT')
    .addParam('address', 'New owner address', undefined, types.string)
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
    .setAction(
        async (
            { address, fromDeployment, contractAddress }: TransferOwnershipProps,
            hre: HardhatRuntimeEnvironment
        ) => {
            await transferOwnershipAction('ZamaOFT', hre, address, fromDeployment, contractAddress)
        }
    )
