import { task, types } from 'hardhat/config'

task('task:verifyProtocolFeesBurner')
    .addParam('protocolFeesBurnerAddress', 'address of deployed ProtocolFeesBurnerAddress', '0x00', types.string)
    .addParam(
        'zamaAddress',
        'address of ZAMA token used in the constructor of ProtocolFeesBurnerAddress',
        '0x00',
        types.string
    )
    .setAction(async function ({ protocolFeesBurnerAddress, zamaAddress }, { run }) {
        await run('verify:verify', {
            address: protocolFeesBurnerAddress,
            constructorArguments: [zamaAddress],
        })
    })
