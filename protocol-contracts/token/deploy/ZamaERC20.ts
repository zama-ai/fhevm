import assert from 'assert'

import { ethers } from 'hardhat'
import { type DeployFunction } from 'hardhat-deploy/types'

import { getRequiredEnvVar } from '../tasks/utils/loadVariables'

const contractName = 'ZamaERC20'

function removeUnderscores(s: string): string {
    return s.replace(/_/g, '')
}

const deploy: DeployFunction = async (hre) => {
    const { getNamedAccounts, deployments } = hre

    const { deploy } = deployments
    const { deployer } = await getNamedAccounts()

    assert(deployer, 'Missing named deployer account')

    console.log(`Network: ${hre.network.name}`)
    console.log(`Deployer: ${deployer}`)

    // Token configuration
    const tokenName = 'Zama'
    const tokenSymbol = 'ZAMA'

    const numReceivers = parseInt(getRequiredEnvVar('NUM_INITIAL_RECEIVERS'))
    // Parse the intial receivers and initial amounts
    const receivers = []
    const amounts = []
    for (let idx = 0; idx < numReceivers; idx++) {
        receivers.push(getRequiredEnvVar(`INITIAL_RECEIVER_${idx}`))
        amounts.push(ethers.utils.parseEther(removeUnderscores(getRequiredEnvVar(`INITIAL_AMOUNT_${idx}`))))
    }

    const initialAdmin = getRequiredEnvVar('INITIAL_ADMIN')

    if (hre.network.name === 'ethereum-testnet' || hre.network.name === 'ethereum-mainnet') {
        const { address } = await deploy(contractName, {
            from: deployer,
            args: [tokenName, tokenSymbol, receivers, amounts, initialAdmin],
            log: true,
            skipIfAlreadyDeployed: false,
        })

        console.log(`Deployed contract: ${contractName}, network: ${hre.network.name}, address: ${address}`)
        console.log(`Token: ${tokenName} (${tokenSymbol})`)
        console.log(`Initial Admin: ${initialAdmin}`)

        const [signer] = await hre.ethers.getSigners()
        const zamaToken = await hre.ethers.getContractAt(contractName, address, signer)

        for (let idx = 0; idx < numReceivers; idx++) {
            const balance = await zamaToken.balanceOf(getRequiredEnvVar(`INITIAL_RECEIVER_${idx}`))
            console.log(`Minted ${hre.ethers.utils.formatEther(balance)} ${tokenSymbol} tokens to ${receivers[idx]}`)
        }
    }
}

deploy.tags = [contractName]

export default deploy
