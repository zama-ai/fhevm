import { task, types } from 'hardhat/config'
import { HardhatRuntimeEnvironment } from 'hardhat/types'

import { createLogger } from '@layerzerolabs/io-devtools'

const logger = createLogger()

// npx hardhat task:sendMockRemoteProposal --sender 0x... --target 0x... --options 0x... --network gateway-mainnet
task('task:sendMockRemoteProposal', 'Sends one mock proposal entry via GovernanceOAppSender.sendRemoteProposal')
    .addParam('sender', 'GovernanceOAppSender address', undefined, types.string)
    .addParam('target', 'Mock target address', undefined, types.string)
    .addParam('options', 'LayerZero options bytes from task:getLZOptions', undefined, types.string)
    .setAction(
        async (
            { sender, target, options }: { sender: string; target: string; options: string },
            hre: HardhatRuntimeEnvironment
        ) => {
            const { ethers, getNamedAccounts } = hre
            const { deployer } = await getNamedAccounts()

            const senderAddress = ethers.utils.getAddress(sender)
            const targetAddress = ethers.utils.getAddress(target)
            const signer = await ethers.getSigner(deployer)

            const governanceSender = await ethers.getContractAt('GovernanceOAppSender', senderAddress, signer)
            const owner = await governanceSender.owner()
            if (ethers.utils.getAddress(owner) !== ethers.utils.getAddress(deployer)) {
                throw new Error(
                    `Deployer ${deployer} is not GovernanceOAppSender owner ${owner}. Use the owner account to send the proposal.`
                )
            }

            const targets = [targetAddress]
            const values = [0]
            const functionSignatures = ['setValue(uint256)']
            const value = 42
            const datas = [ethers.utils.defaultAbiCoder.encode(['uint256'], [value])]
            const operations = [0] // Operation.Call

            logger.info(`Network: ${hre.network.name}`)
            logger.info(`Sender: ${senderAddress}`)
            logger.info(`Target: ${targetAddress}`)
            logger.info(`Options: ${options}`)
            logger.info(`Calling sendRemoteProposal with one mock entry (setValue(${value}))...`)

            const tx = await governanceSender.sendRemoteProposal(
                targets,
                values,
                functionSignatures,
                datas,
                operations,
                options
            )
            const receipt = await tx.wait()

            logger.info(`✔ Proposal sent in tx ${receipt.transactionHash}`)
        }
    )
