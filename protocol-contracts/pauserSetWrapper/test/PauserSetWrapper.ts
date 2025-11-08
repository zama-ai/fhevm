import { loadFixture } from '@nomicfoundation/hardhat-toolbox/network-helpers'
import { expect } from 'chai'
import hre from 'hardhat'
import { ethers, Signer } from 'ethers'

import { TokenMock, PauserSetMock, PauserSetWrapper } from '../typechain-types'

describe('PauserSetWrapper', function () {
    // We define a fixture to reuse the same setup in every test.
    // We use loadFixture to run this setup once, snapshot that state,
    // and reset Hardhat Network to that snapshot in every test.
    async function deployPauserSetMockAndWrapper() {
        // Get the deployed PauserSetWrapper contract
        const pauserSetWrapperDeployment = await hre.deployments.get('PauserSetWrapper')
        const pauserSetWrapper = await hre.ethers.getContractAt('PauserSetWrapper', pauserSetWrapperDeployment.address)

        // Get the deployed TokenMock contract
        const tokenMockDeployment = await hre.deployments.get('TokenMock')
        const tokenMock = await hre.ethers.getContractAt('TokenMock', tokenMockDeployment.address)

        // Get the deployed PauserSetMock contract
        const pauserSetMockDeployment = await hre.deployments.get('PauserSetMock')
        const pauserSetMock = await hre.ethers.getContractAt('PauserSetMock', pauserSetMockDeployment.address)

        // Grant the mocked token's MINTING_PAUSER_ROLE to the pauserSetWrapper contract
        const mintingPauserRole = await tokenMock.MINTING_PAUSER_ROLE()
        await tokenMock.grantRole(mintingPauserRole, pauserSetWrapperDeployment.address)

        return { tokenMock, pauserSetMock, pauserSetWrapper }
    }

    let tokenMock: TokenMock
    let pauserSetMock: PauserSetMock
    let pauserSetWrapper: PauserSetWrapper
    let deployer: Signer
    let alice: Signer
    let bob: Signer

    beforeEach(async function () {
        const fixtureData = await loadFixture(deployPauserSetMockAndWrapper)
        tokenMock = fixtureData.tokenMock
        pauserSetMock = fixtureData.pauserSetMock
        pauserSetWrapper = fixtureData.pauserSetWrapper

        const namedAccounts = await hre.getNamedAccounts()
        deployer = await hre.ethers.getSigner(namedAccounts.deployer)
        alice = await hre.ethers.getSigner(namedAccounts.alice)
        bob = await hre.ethers.getSigner(namedAccounts.bob)

        const aliceAddress = await alice.getAddress()

        // Add Alice as a pauser
        // Deployer is the owner of the PauserSetMock contract
        await pauserSetMock.connect(deployer).addPauser(aliceAddress)
        expect(await pauserSetMock.isPauser(aliceAddress)).to.be.true
    })

    describe('Deployment', function () {
        // Define the pause function signature
        const functionSignatures = 'pauseMinting()'

        it('Should have correct constructor values', async function () {
            // Check the addresses
            expect(await pauserSetWrapper.CONTRACT_TARGET()).to.equal(await tokenMock.getAddress())
            expect(await pauserSetWrapper.PAUSER_SET()).to.equal(await pauserSetMock.getAddress())

            // Check the function signature
            expect(await pauserSetWrapper.FUNCTION_SIGNATURE()).to.equal(functionSignatures)

            // Check the function selector
            // Selector is the first 4 bytes of the keccak256 hash of the function signature + 1
            // bytes for `0x` prefix
            expect(await pauserSetWrapper.FUNCTION_SELECTOR()).to.equal(
                ethers.keccak256(ethers.toUtf8Bytes(functionSignatures)).slice(0, 10)
            )
        })

        it('Should not deploy new contract if target has no code', async function () {
            const deployerAddress = await deployer.getAddress()
            const pauserSetMockAddress = await pauserSetMock.getAddress()

            // Get the contract factory
            const PauserSetWrapperFactory = await hre.ethers.getContractFactory('PauserSetWrapper')

            const fakeTargetAddress = '0x0000000000000000000000000000000000000000'
            await expect(
                hre.deployments.deploy('PauserSetWrapper', {
                    from: deployerAddress,
                    args: [fakeTargetAddress, 'pauseMinting()', pauserSetMockAddress],
                    log: true,
                    skipIfAlreadyDeployed: false,
                })
            )
                .to.be.revertedWithCustomError(PauserSetWrapperFactory, 'NoCodeAtTarget')
                .withArgs(fakeTargetAddress)
        })

        it('Should not deploy new contract if pauser set has no code', async function () {
            const deployerAddress = await deployer.getAddress()
            const tokenMockAddress = await tokenMock.getAddress()

            // Get the contract factory
            const PauserSetWrapperFactory = await hre.ethers.getContractFactory('PauserSetWrapper')

            const fakePauserSetAddress = '0x0000000000000000000000000000000000000000'
            await expect(
                hre.deployments.deploy('PauserSetWrapper', {
                    from: deployerAddress,
                    args: [tokenMockAddress, 'pauseMinting()', fakePauserSetAddress],
                    log: true,
                    skipIfAlreadyDeployed: false,
                })
            )
                .to.be.revertedWithCustomError(PauserSetWrapperFactory, 'NoCodeAtPauserSet')
                .withArgs(fakePauserSetAddress)
        })
    })

    describe('After deployment', function () {
        it('Should revert because the deployer is not a pauser', async function () {
            // The `pauseMinting` function does not require any inputs
            await expect(pauserSetWrapper.connect(deployer).callFunction('0x')).to.be.revertedWithCustomError(
                pauserSetWrapper,
                'SenderNotPauser'
            )

            expect(await tokenMock.paused()).to.be.false
        })

        it('Should pause minting', async function () {
            // The `pauseMinting` function does not require any inputs
            await expect(pauserSetWrapper.connect(alice).callFunction('0x')).not.be.reverted
            expect(await tokenMock.paused()).to.be.true

            // Check that the deployer cannot mint tokens since the token is paused
            const deployerAddress = await deployer.getAddress()
            await expect(tokenMock.connect(deployer).mint(deployerAddress, 1000)).to.be.revertedWithCustomError(
                tokenMock,
                'EnforcedPause'
            )
        })

        it('Should revert because contract is already paused', async function () {
            // The `pauseMinting` function does not require any inputs
            await expect(pauserSetWrapper.connect(alice).callFunction('0x')).not.be.reverted
            expect(await tokenMock.paused()).to.be.true

            // Check that the alice cannot pause the token since the token is already paused
            await expect(pauserSetWrapper.connect(alice).callFunction('0x'))
                .to.be.revertedWithCustomError(pauserSetWrapper, 'ExecutionFailed')
                .withArgs(ethers.keccak256(ethers.toUtf8Bytes('EnforcedPause()')).slice(0, 10))
        })

        it('Should not revert even if wrong inputs are passed', async function () {
            // The `pauseMinting` function does not require any inputs
            await expect(pauserSetWrapper.connect(alice).callFunction('0x00000001')).not.be.reverted
        })
    })
})
