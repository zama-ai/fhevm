import assert from 'assert'

import { type DeployFunction } from 'hardhat-deploy/types'

const contractName = 'GovernanceOAppSender'

const deploy: DeployFunction = async (hre) => {
    const { getNamedAccounts, deployments } = hre

    const { deploy } = deployments
    const { deployer } = await getNamedAccounts()

    assert(deployer, 'Missing named deployer account')

    // Usage: DST_EID=<YOUR_DST_EID> npx hardhat lz:deploy --tags GovernanceOAppSender
    const dstEidRaw = process.env.DST_EID
    assert(dstEidRaw, 'Missing DST_EID environment variable (e.g. DST_EID=40424 npx hardhat lz:deploy ...)')

    const dstEid = Number(dstEidRaw)
    assert(
        Number.isInteger(dstEid) && dstEid > 0 && dstEid <= 0xffffffff,
        `Invalid DST_EID "${dstEidRaw}": must be a positive uint32 integer`
    )

    console.log(`Network: ${hre.network.name}`)
    console.log(`Deployer: ${deployer}`)
    console.log(`Destination EID: ${dstEid}`)

    const endpointV2Deployment = await hre.deployments.get('EndpointV2')

    const { address } = await deploy(contractName, {
        from: deployer,
        args: [
            endpointV2Deployment.address, // LayerZero's EndpointV2 address
            deployer, // owner
            dstEid, // destination (Zama gateway) endpoint ID
        ],
        log: true,
        skipIfAlreadyDeployed: false,
    })

    console.log(`Deployed contract: ${contractName}, network: ${hre.network.name}, address: ${address}`)
}

deploy.tags = [contractName]

export default deploy
