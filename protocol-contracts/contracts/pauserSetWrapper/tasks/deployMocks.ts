import { task } from 'hardhat/config'
import { DeploymentsExtension } from 'hardhat-deploy/types'
import { Network } from 'hardhat/types'

async function deployMockContract(
    contractName: string,
    deployer: string,
    args: unknown[],
    deployments: DeploymentsExtension,
    network: Network
) {
    console.log(`Deploying ${contractName}...`)
    const { address } = await deployments.deploy(contractName, {
        from: deployer,
        args: args,
        log: true,
        skipIfAlreadyDeployed: false,
    })
    console.log(`Deployed contract: ${contractName}, network: ${network.name}, address: ${address}`)
}

// Deploy the mock contracts needed for testing the PauserSetWrapper contract
// Example usage:
// npx hardhat task:deployMocks
task('task:deployMocks').setAction(async function (_, { getNamedAccounts, deployments, network }) {
    const { deployer } = await getNamedAccounts()

    const tokenMockName = 'TokenMock'
    await deployMockContract(tokenMockName, deployer, [], deployments, network)

    const pauserSetMockName = 'PauserSetMock'
    await deployMockContract(pauserSetMockName, deployer, [], deployments, network)
})
