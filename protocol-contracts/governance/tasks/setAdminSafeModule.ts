import { task, types } from 'hardhat/config'
import { HardhatRuntimeEnvironment } from 'hardhat/types'
import { createLogger } from '@layerzerolabs/io-devtools'
const logger = createLogger()

task('task:setAdminSafeModule', 'Sets GovernanceOAppReceiver.adminSafeModule')
    .addParam('module', 'AdminModule address to authorize', undefined, types.string)
    .addOptionalParam('receiver', 'GovernanceOAppReceiver address (defaults to deployment)', undefined, types.string)
    .setAction(async ({ module, receiver }: { module: string, receiver: string }, hre: HardhatRuntimeEnvironment) => {
        const { deployments, ethers, getNamedAccounts } = hre

        const moduleAddress = ethers.utils.getAddress(module)

        const receiverDeploymentAddress = receiver
            ? receiver
            : (await deployments.get('GovernanceOAppReceiver')).address
        const receiverAddress = ethers.utils.getAddress(receiverDeploymentAddress)

        const { deployer } = await getNamedAccounts()
        const signer = await ethers.getSigner(deployer)
        const governanceReceiver = await ethers.getContractAt('GovernanceOAppReceiver', receiverAddress, signer)

        const currentModule: string = await governanceReceiver.adminSafeModule()
        if (ethers.utils.getAddress(currentModule) === moduleAddress) {
            logger.info(`adminSafeModule already set to ${moduleAddress} on ${receiverAddress}`)
            return
        }

        logger.info(`Setting adminSafeModule on ${receiverAddress} to ${moduleAddress}...`)
        const tx = await governanceReceiver.setAdminSafeModule(moduleAddress)
        const receipt = await tx.wait()
        logger.info(`✔ adminSafeModule updated in tx ${receipt?.hash ?? tx.hash}`)
    })
