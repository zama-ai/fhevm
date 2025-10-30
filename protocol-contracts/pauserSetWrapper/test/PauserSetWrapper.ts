import { loadFixture } from '@nomicfoundation/hardhat-toolbox/network-helpers'
import { expect } from 'chai'
import hre from 'hardhat'

describe('PauserSetWrapper', function () {
    // We define a fixture to reuse the same setup in every test.
    // We use loadFixture to run this setup once, snapshot that state,
    // and reset Hardhat Network to that snapshot in every test.
    async function deployPauserSetMockAndWrapper() {
        // Contracts are deployed using the first signer/account by default
        const [owner, alice, bob] = await hre.ethers.getSigners()

        const TokenMock = await hre.ethers.getContractFactory('TokenMock')
        const tokenMock = await TokenMock.deploy()

        const PauserSetMock = await hre.ethers.getContractFactory('PauserSetMock')
        const pauserSetMock = await PauserSetMock.deploy()

        const PauserSetWrapper = await hre.ethers.getContractFactory('PauserSetWrapper')
        const tokenAddress = await tokenMock.getAddress()
        const pauserSetAddress = await pauserSetMock.getAddress()
        const pauserSetWrapper = await PauserSetWrapper.deploy(tokenAddress, 'pauseMinting()', pauserSetAddress)

        const pauserSetWrapperAddress = await pauserSetWrapper.getAddress()
        const mintingPauserRole = await tokenMock.MINTING_PAUSER_ROLE()
        // token admin grants the MINTING_PAUSER_ROLE to pauserSetWrapper contract
        await tokenMock.grantRole(mintingPauserRole, pauserSetWrapperAddress)

        return { tokenMock, pauserSetMock, pauserSetWrapper, owner, alice, bob }
    }

    describe('Deployment', function () {
        it('Should set the right PAUSER_SET address', async function () {
            const { tokenMock, pauserSetMock, pauserSetWrapper } = await loadFixture(deployPauserSetMockAndWrapper)
            expect(await pauserSetWrapper.CONTRACT_TARGET()).to.equal(await tokenMock.getAddress())
            expect(await pauserSetWrapper.PAUSER_SET()).to.equal(await pauserSetMock.getAddress())
        })
    })

    describe('Execution', function () {
        it('Only pauser from PauserSet could pause minting', async function () {
            const { tokenMock, pauserSetMock, pauserSetWrapper, alice } =
                await loadFixture(deployPauserSetMockAndWrapper)

            await pauserSetMock.addPauser(alice.address) // owner adds alice as a pauser
            expect(await pauserSetMock.isPauser(alice.address)).to.be.true

            // expected to revert since owner is not a pauser
            await expect(pauserSetWrapper.callFunction('0x')).to.be.revertedWithCustomError(
                pauserSetWrapper,
                'SenderNotPauser'
            )

            expect(await tokenMock.paused()).to.be.false

            // expected to succeed since alice is indeed a pauser
            await expect(pauserSetWrapper.connect(alice).callFunction('0x')).not.be.reverted
            expect(await tokenMock.paused()).to.be.true
        })
    })
})
