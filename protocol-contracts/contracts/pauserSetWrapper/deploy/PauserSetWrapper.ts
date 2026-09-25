import assert from 'assert'

import { type DeployFunction } from 'hardhat-deploy/types'

import { getRequiredEnvVar } from '../tasks/utils/loadVariables'

const contractName = 'PauserSetWrapper'

const deploy: DeployFunction = async (hre) => {
    const { getNamedAccounts, deployments } = hre

    const { deploy } = deployments
    const { deployer } = await getNamedAccounts()

    assert(deployer, 'Missing named deployer account')

    console.log(`Network: ${hre.network.name}`)
    console.log(`Deployer: ${deployer}`)

    const contractTarget = getRequiredEnvVar('CONTRACT_TARGET')
    const functionSignature = getRequiredEnvVar('FUNCTION_SIGNATURE')
    const pauserSet = getRequiredEnvVar('PAUSER_SET')

    const { address } = await deploy(contractName, {
        from: deployer,
        args: [contractTarget, functionSignature, pauserSet],
        log: true,
        skipIfAlreadyDeployed: false,
    })
    console.log(`✅ Deployed contract: ${contractName}, network: ${hre.network.name}, address: ${address}`)
}

deploy.tags = [contractName]

export default deploy
