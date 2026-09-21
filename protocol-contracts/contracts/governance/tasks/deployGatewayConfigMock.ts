import { task, types } from 'hardhat/config'
import { HardhatRuntimeEnvironment } from 'hardhat/types'

import { createLogger } from '@layerzerolabs/io-devtools'

const logger = createLogger()

// npx hardhat task:deployGatewayConfigMock --owner 0x... --network gateway-mainnet
task('task:deployGatewayConfigMock', 'Deploys the GatewayConfigMock contract')
    .addParam('owner', 'Initial owner address', undefined, types.string)
    .setAction(async ({ owner }: { owner: string }, hre: HardhatRuntimeEnvironment) => {
        const { ethers, getNamedAccounts } = hre
        const { deployer } = await getNamedAccounts()

        const initialOwner = ethers.utils.getAddress(owner)
        const signer = await ethers.getSigner(deployer)

        logger.info(`Deploying GatewayConfigMock on ${hre.network.name}`)
        logger.info(`Deployer: ${deployer}`)
        logger.info(`Initial owner: ${initialOwner}`)

        const factory = await ethers.getContractFactory('GatewayConfigMock', signer)
        const gatewayConfigMock = await factory.deploy(initialOwner)
        await gatewayConfigMock.deployed()

        logger.info(`✔ GatewayConfigMock deployed at: ${gatewayConfigMock.address}`)
    })
