import { task, types } from 'hardhat/config'
import { HardhatRuntimeEnvironment } from 'hardhat/types'

import { SetDelegateProps, setDelegateAction } from '../utils/setDelegate'

task('zama:oftadapter:setDelegate', 'Set the delegate for ZamaOFTAdapter')
    .addParam('address', 'New delegate address', undefined, types.string)
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
        async ({ address, fromDeployment, contractAddress }: SetDelegateProps, hre: HardhatRuntimeEnvironment) => {
            await setDelegateAction('ZamaOFTAdapter', hre, address, fromDeployment, contractAddress)
        }
    )
