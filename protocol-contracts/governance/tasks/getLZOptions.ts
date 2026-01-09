import { Options } from '@layerzerolabs/lz-v2-utilities'
import { task, types } from 'hardhat/config'

// eg use: `npx hardhat task:getLZOptions --gas-limit 300000`
task('task:getLZOptions')
    .addParam('gasLimit', 'Gas limit for the LZ receive option', undefined, types.int)
    .addOptionalParam('nativeValue', 'Native value to send with the LZ message', 0, types.int)
    .setAction(async function (taskArgs) {
        const options = Options.newOptions()
            .addExecutorLzReceiveOption(taskArgs.gasLimit, taskArgs.nativeValue)
            .toHex()
            .toString()
        console.log('Options bytes for sendRemoteProposal:')
        console.log(options)
    })
