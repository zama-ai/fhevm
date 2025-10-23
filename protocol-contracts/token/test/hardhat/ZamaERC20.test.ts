import { SignerWithAddress } from '@nomiclabs/hardhat-ethers/signers'
import { expect } from 'chai'
import { BigNumber, Contract, ContractFactory } from 'ethers'
import { ethers } from 'hardhat'

describe('ZamaERC20 - Unit Test', () => {
    // Declaration of variables to be used in the test suite
    let zamaERC20Factory: ContractFactory
    let deployer: SignerWithAddress
    let owner: SignerWithAddress
    let admin: SignerWithAddress
    let alice: SignerWithAddress
    let bob: SignerWithAddress
    let charlie: SignerWithAddress
    let zamaERC20: Contract

    const DEFAULT_ADMIN_ROLE = ethers.constants.HashZero
    const MINTER_ROLE = ethers.utils.keccak256(ethers.utils.toUtf8Bytes('MINTER_ROLE'))
    const MINTING_PAUSER_ROLE = ethers.utils.keccak256(ethers.utils.toUtf8Bytes('MINTING_PAUSER_ROLE'))

    // Before hook for setup that runs once before all tests in the block
    before(async () => {
        zamaERC20Factory = await ethers.getContractFactory('ZamaERC20')

        // Fetching the first three signers (accounts) from Hardhat's local Ethereum network
        const signers = await ethers.getSigners()

        ;[deployer, owner, admin, alice, bob, charlie] = signers
    })

    // beforeEach hook for setup that runs before each test in the block
    beforeEach(async () => {
        // The INITIAL_SUPPLY_RECEIVER and INITIAL_ADMIN can be different from the deployer.
        zamaERC20 = await zamaERC20Factory.connect(deployer).deploy('ZAMAERC20', 'ZAMA', owner.address, admin.address)
    })

    describe('Initialization', () => {
        it('should properly be initialized', async () => {
            // Check that owner has the initial supply
            const expectedTokenAmount = ethers.utils.parseEther('11000000000')
            const ownerBalance = await zamaERC20.balanceOf(owner.address)

            expect(ownerBalance).eql(expectedTokenAmount)

            // Check that admin has DEFAULT_ADMIN_ROLE
            const hasAdminRole = await zamaERC20.hasRole(DEFAULT_ADMIN_ROLE, admin.address)
            expect(hasAdminRole).to.be.true

            // Check that DEFAULT_ADMIN_ROLE is the admin role of the MINTER_ROLE & MINTING_PAUSER_ROLE
            const minterRoleAdmin = await zamaERC20.getRoleAdmin(MINTER_ROLE)
            const mintingPauserRoleAdmin = await zamaERC20.getRoleAdmin(MINTING_PAUSER_ROLE)
            expect(minterRoleAdmin).to.eq(DEFAULT_ADMIN_ROLE)
            expect(mintingPauserRoleAdmin).to.eq(DEFAULT_ADMIN_ROLE)

            // Check that admin does not have MINTER_ROLE & MINTING_PAUSER_ROLE
            const hasMinterRole = await zamaERC20.hasRole(MINTER_ROLE, admin.address)
            const hasMintingPauserRole = await zamaERC20.hasRole(MINTING_PAUSER_ROLE, admin.address)
            expect(hasMinterRole).to.be.false
            expect(hasMintingPauserRole).to.be.false

            // Check that contract is unpaused
            const isPaused = await zamaERC20.paused()
            expect(isPaused).to.be.false
        })
    })

    describe('AccessControl', () => {
        it('should let admin grant MINTER_ROLE to alice', async () => {
            await expect(zamaERC20.connect(admin).grantRole(MINTER_ROLE, alice.address))
                .to.emit(zamaERC20, 'RoleGranted')
                .withArgs(MINTER_ROLE, alice.address, admin.address)
            const hasMinterRoleAlice = await zamaERC20.hasRole(MINTER_ROLE, alice.address)
            expect(hasMinterRoleAlice).to.be.true
        })

        it('should let admin grant MINTING_PAUSER_ROLE to bob', async () => {
            await expect(zamaERC20.connect(admin).grantRole(MINTING_PAUSER_ROLE, bob.address))
                .to.emit(zamaERC20, 'RoleGranted')
                .withArgs(MINTING_PAUSER_ROLE, bob.address, admin.address)
            const hasMintingPauserRoleBob = await zamaERC20.hasRole(MINTING_PAUSER_ROLE, bob.address)
            expect(hasMintingPauserRoleBob).to.be.true
        })

        it('should not let bob grant minter role to alice', async () => {
            await expect(zamaERC20.connect(bob).grantRole(MINTER_ROLE, alice.address)).to.be.revertedWithCustomError(
                zamaERC20,
                'AccessControlUnauthorizedAccount'
            )
            const hasMinterRoleAlice = await zamaERC20.hasRole(MINTER_ROLE, alice.address)
            expect(hasMinterRoleAlice).to.be.false
        })

        it('should not let bob grant minting pauser role to alice', async () => {
            await expect(
                zamaERC20.connect(bob).grantRole(MINTING_PAUSER_ROLE, alice.address)
            ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
        })
    })

    describe('Mint', () => {
        const TOKENS_TO_MINT = ethers.utils.parseEther('10')
        beforeEach(async () => {
            // Grant alice the MINTER_ROLE
            // Grant bob the MINTING_PAUSER_ROLE
            // charlie has no roles
            await zamaERC20.connect(admin).grantRole(MINTER_ROLE, alice.address)
            await zamaERC20.connect(admin).grantRole(MINTING_PAUSER_ROLE, bob.address)

            // The contract is unpaused by default
            const isPaused = await zamaERC20.paused()
            expect(isPaused).to.be.false
        })

        it('should let PAUSING_MINTER_ROLE pause the minting', async () => {
            await zamaERC20.connect(bob).pauseMinting()

            const isPaused = await zamaERC20.paused()
            expect(isPaused).to.be.true
        })

        it('should not let DEFAULT_ADMIN_ROLE pause the minting', async () => {
            await expect(zamaERC20.connect(admin).pauseMinting()).to.be.revertedWithCustomError(
                zamaERC20,
                'AccessControlUnauthorizedAccount'
            )
        })

        it('should not let MINTER_ROLE pause the minting', async () => {
            await expect(zamaERC20.connect(alice).pauseMinting()).to.be.revertedWithCustomError(
                zamaERC20,
                'AccessControlUnauthorizedAccount'
            )
        })

        it('should not let generic user pause the minting', async () => {
            await expect(zamaERC20.connect(charlie).pauseMinting()).to.be.revertedWithCustomError(
                zamaERC20,
                'AccessControlUnauthorizedAccount'
            )
        })

        it('should let DEFAULT_ADMIN_ROLE unpause the minting', async () => {
            await zamaERC20.connect(bob).pauseMinting()
            let isPaused = await zamaERC20.paused()
            expect(isPaused).to.be.true

            await zamaERC20.connect(admin).unpauseMinting()

            isPaused = await zamaERC20.paused()
            expect(isPaused).to.be.false
        })

        it('should not let MINTER_ROLE unpause the minting', async () => {
            await expect(zamaERC20.connect(alice).unpauseMinting()).to.be.revertedWithCustomError(
                zamaERC20,
                'AccessControlUnauthorizedAccount'
            )
        })

        it('should not let PAUSING_MINTER_ROLE unpause the minting', async () => {
            await expect(zamaERC20.connect(bob).unpauseMinting()).to.be.revertedWithCustomError(
                zamaERC20,
                'AccessControlUnauthorizedAccount'
            )
        })

        it('should not let generic user unpause the minting', async () => {
            await expect(zamaERC20.connect(charlie).unpauseMinting()).to.be.revertedWithCustomError(
                zamaERC20,
                'AccessControlUnauthorizedAccount'
            )
        })

        it('should let MINTER_ROLE mint $ZAMA while unpaused', async () => {
            const aliceBalanceBefore: BigNumber = await zamaERC20.balanceOf(alice.address)

            await zamaERC20.connect(alice).mint(alice.address, TOKENS_TO_MINT)

            const aliceBalanceAfter: BigNumber = await zamaERC20.balanceOf(alice.address)

            expect(aliceBalanceAfter.sub(aliceBalanceBefore)).eql(TOKENS_TO_MINT)
        })

        it('should not let DEFAULT_ADMIN_ROLE mint $ZAMA while unpaused', async () => {
            await expect(zamaERC20.connect(admin).mint(admin.address, TOKENS_TO_MINT)).to.be.revertedWithCustomError(
                zamaERC20,
                'AccessControlUnauthorizedAccount'
            )
        })

        it('should not let PAUSING_MINTER_ROLE mint $ZAMA while unpaused', async () => {
            await expect(zamaERC20.connect(bob).mint(bob.address, TOKENS_TO_MINT)).to.be.revertedWithCustomError(
                zamaERC20,
                'AccessControlUnauthorizedAccount'
            )
        })

        it('should not let generic address mint $ZAMA while unpaused', async () => {
            await expect(
                zamaERC20.connect(charlie).mint(charlie.address, TOKENS_TO_MINT)
            ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
        })

        it('should not let MINTER_ROLE mint $ZAMA while paused', async () => {
            await zamaERC20.connect(bob).pauseMinting()
            const isPaused = await zamaERC20.paused()
            expect(isPaused).to.be.true

            await expect(zamaERC20.connect(alice).mint(alice.address, TOKENS_TO_MINT)).to.be.revertedWithCustomError(
                zamaERC20,
                'EnforcedPause'
            )
        })

        it('should not let DEFAULT_ADMIN_ROLE mint $ZAMA while paused', async () => {
            await zamaERC20.connect(bob).pauseMinting()
            const isPaused = await zamaERC20.paused()
            expect(isPaused).to.be.true

            await expect(zamaERC20.connect(admin).mint(admin.address, TOKENS_TO_MINT)).to.be.revertedWithCustomError(
                zamaERC20,
                'AccessControlUnauthorizedAccount'
            )
        })

        it('should not let PAUSING_MINTER_ROLE mint $ZAMA while paused', async () => {
            await zamaERC20.connect(bob).pauseMinting()
            const isPaused = await zamaERC20.paused()
            expect(isPaused).to.be.true

            await expect(zamaERC20.connect(bob).mint(bob.address, TOKENS_TO_MINT)).to.be.revertedWithCustomError(
                zamaERC20,
                'AccessControlUnauthorizedAccount'
            )
        })

        it('should not let generic address mint $ZAMA while paused', async () => {
            await zamaERC20.connect(bob).pauseMinting()
            const isPaused = await zamaERC20.paused()
            expect(isPaused).to.be.true

            await expect(
                zamaERC20.connect(charlie).mint(charlie.address, TOKENS_TO_MINT)
            ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
        })
    })

    describe('Burn', () => {
        describe('burn', () => {})
        describe('burnFrom', () => {})
    })

    describe('Transfer', () => {
        describe('Transfer - ERC20', () => {
            describe('transfer', () => {})
            describe('transferFrom', () => {})
        })
        describe('Transfer - ERC20Permit', () => {})
        describe('Transfer - ERC1363', () => {
            describe('transferAndCall', () => {})
            describe('transferFromAndCall', () => {})
            describe('approveAndCall', () => {})
        })
    })

    describe('AssetRecoverer', () => {
        describe('recoverEther', () => {})
        describe('recoverERC20', () => {})
        describe('recoverERC721', () => {})
        describe('recoverERC1155', () => {})
    })
})
