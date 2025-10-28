import { expect } from 'chai'
import { Contract } from 'ethers'
import { deployments, ethers, network, run } from 'hardhat'

describe('ZamaOFT task suite', function () {
    let oft: Contract
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
            await deleteFn('EndpointStubOFT')
            await deleteFn('ZamaOFT')
        }

        const endpointDeployment = await deploy('EndpointStubOFT', {
            contract: 'EndpointStub',
            from: deployer,
            args: [],
            log: false,
            skipIfAlreadyDeployed: false,
        })
        endpointStub = await ethers.getContractAt('EndpointStub', endpointDeployment.address, deployerSigner)

        const oftDeployment = await deploy('ZamaOFT', {
            from: deployer,
            args: ['ZamaOFT', 'ZOF', endpointDeployment.address, deployer],
            log: false,
            skipIfAlreadyDeployed: false,
        })

        oft = await ethers.getContractAt('ZamaOFT', oftDeployment.address, deployerSigner)

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
        expect(await endpointStub.delegateOf(oft.address)).to.equal(deployer)

        await run('zama:oft:setDelegate', { address: other })

        expect(await endpointStub.delegateOf(oft.address)).to.equal(other)
    })

    it('transfers ownership via task', async function () {
        expect(await oft.owner()).to.equal(deployer)

        await run('zama:oft:transferOwnership', { address: other })

        expect(await oft.owner()).to.equal(other)
    })
})
