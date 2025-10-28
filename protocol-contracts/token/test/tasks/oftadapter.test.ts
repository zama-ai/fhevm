import { expect } from 'chai'
import { Contract } from 'ethers'
import { deployments, ethers, network, run } from 'hardhat'

describe('ZamaOFTAdapter task suite', function () {
    let oftAdapter: Contract
    let endpointStub: Contract
    let deployer: string
    let other: string
    let snapshotId: string

    before(async function () {
        if (network.name !== 'hardhat') {
            this.skip()
        }

        const [deployerSigner, otherSigner] = await ethers.getSigners()
        deployer = deployerSigner.address
        other = otherSigner.address

        const { deploy } = deployments
        const deleteFn = (deployments as unknown as { delete?: (name: string) => Promise<void> }).delete
        if (deleteFn) {
            await deleteFn('EndpointStubAdapter')
            await deleteFn('ZamaERC20')
            await deleteFn('ZamaOFTAdapter')
        }

        const endpointDeployment = await deploy('EndpointStubAdapter', {
            contract: 'EndpointStub',
            from: deployer,
            args: [],
            log: false,
            skipIfAlreadyDeployed: false,
        })
        endpointStub = await ethers.getContractAt('EndpointStub', endpointDeployment.address, deployerSigner)

        const ercDeployment = await deploy('ZamaERC20', {
            from: deployer,
            args: ['ZAMAERC20', 'ZAMA', deployer, deployer],
            log: false,
            skipIfAlreadyDeployed: false,
        })

        const oftAdapterDeployment = await deploy('ZamaOFTAdapter', {
            from: deployer,
            args: [ercDeployment.address, endpointDeployment.address, deployer],
            log: false,
            skipIfAlreadyDeployed: false,
        })

        oftAdapter = await ethers.getContractAt('ZamaOFTAdapter', oftAdapterDeployment.address, deployerSigner)

        snapshotId = await network.provider.send('evm_snapshot', [])
    })

    beforeEach(async function () {
        if (network.name !== 'hardhat') {
            this.skip()
        }

        await network.provider.send('evm_revert', [snapshotId])
        snapshotId = await network.provider.send('evm_snapshot', [])
    })

    it('updates the delegate via task', async function () {
        expect(await endpointStub.delegateOf(oftAdapter.address)).to.equal(deployer)

        await run('zama:oftadapter:setDelegate', { address: other })

        expect(await endpointStub.delegateOf(oftAdapter.address)).to.equal(other)
    })

    it('transfers ownership via task', async function () {
        expect(await oftAdapter.owner()).to.equal(deployer)

        await run('zama:oftadapter:transferOwnership', { address: other })

        expect(await oftAdapter.owner()).to.equal(other)
    })
})
