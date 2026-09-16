import { task } from 'hardhat/config'

import { getRequiredEnvVar } from './utils/loadVariables'

// Verify the PauserSetWrapper contract at the given address.
task('task:verifyPauserSetWrapper')
    .addParam('address', 'address of the already deployed PauserSetWrapper contract that should be verified')
    .setAction(async function ({ address }: { address: string }, hre) {
        const contractTarget = getRequiredEnvVar('CONTRACT_TARGET')
        const functionSignature = getRequiredEnvVar('FUNCTION_SIGNATURE')
        const pauserSet = getRequiredEnvVar('PAUSER_SET')
        const apiKey = getRequiredEnvVar('ETHERSCAN_API_KEY')

        if (typeof hre.config.verify?.etherscan?.apiKey !== 'string') {
            console.log(
                'Verification on Ethereum requires using Etherscan API, ensuring the etherscan.apiKey field is set.'
            )
            hre.config.verify!.etherscan!.apiKey = apiKey
        }

        await hre.run('verify:verify', {
            address,
            constructorArguments: [contractTarget, functionSignature, pauserSet],
        })
    })
