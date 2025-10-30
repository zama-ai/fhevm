import assert from 'assert'

import { type DeployFunction } from 'hardhat-deploy/types'
import { getRequiredEnvVar } from '../tasks/utils/loadVariables'

const contractName = 'ZamaERC20'

const deploy: DeployFunction = async (hre) => {
    const { getNamedAccounts, deployments } = hre

    const { deploy } = deployments
    const { deployer } = await getNamedAccounts()

    assert(deployer, 'Missing named deployer account')

    console.log(`Network: ${hre.network.name}`)
    console.log(`Deployer: ${deployer}`)

    // Token configuration
    const tokenName = 'ZAMAERC20'
    const tokenSymbol = 'ZAMA'
    const initialSupplyReceiver = getRequiredEnvVar('INITIAL_SUPPLY_RECEIVER')
    const initialAdmin = getRequiredEnvVar('INITIAL_ADMIN')

    //if (hre.network.name === 'ethereum-testnet') {
    const { address } = await deploy(contractName, {
        from: deployer,
        args: [tokenName, tokenSymbol, initialSupplyReceiver, initialAdmin],
        log: true,
        skipIfAlreadyDeployed: false,
    })

    console.log(`Deployed contract: ${contractName}, network: ${hre.network.name}, address: ${address}`)
    console.log(`Token: ${tokenName} (${tokenSymbol})`)
    console.log(`Initial Admin: ${initialAdmin}`)

    // Mint initial tokens to the deployer
    const [signer] = await hre.ethers.getSigners()
    const zamaToken = await hre.ethers.getContractAt(contractName, address, signer)

    const balance = await zamaToken.balanceOf(deployer)
    console.log(`Minted ${hre.ethers.utils.formatEther(balance)} ${tokenSymbol} tokens to ${initialSupplyReceiver}`)
    //}
}

deploy.tags = [contractName]

export default deploy
