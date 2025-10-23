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
            // Number of decimals
            expect(await zamaERC20.decimals()).to.eq(18)

            // Name
            expect(await zamaERC20.name()).to.eq('ZAMAERC20')

            // Symbol
            expect(await zamaERC20.symbol()).to.eq('ZAMA')

            // Check that owner has the initial supply
            const expectedTokenAmount = ethers.utils.parseEther('11000000000')
            const ownerBalance = await zamaERC20.balanceOf(owner.address)
            expect(ownerBalance).eql(expectedTokenAmount)

            // Total Supply
            expect(await zamaERC20.totalSupply()).to.eq(expectedTokenAmount)

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

        it('should remove privileges when role is revoked', async () => {
            await zamaERC20.connect(admin).grantRole(MINTING_PAUSER_ROLE, bob.address)
            expect(await zamaERC20.hasRole(MINTING_PAUSER_ROLE, bob.address)).to.be.true
            await expect(zamaERC20.connect(admin).revokeRole(MINTING_PAUSER_ROLE, bob.address))
                .to.emit(zamaERC20, 'RoleRevoked')
                .withArgs(MINTING_PAUSER_ROLE, bob.address, admin.address)
            expect(await zamaERC20.hasRole(MINTING_PAUSER_ROLE, bob.address)).to.be.false
            await expect(zamaERC20.connect(bob).pauseMinting()).to.be.revertedWithCustomError(
                zamaERC20,
                'AccessControlUnauthorizedAccount'
            )
        })

        it('should remove privileges when role is renounced', async () => {
            await zamaERC20.connect(admin).grantRole(MINTING_PAUSER_ROLE, bob.address)
            expect(await zamaERC20.hasRole(MINTING_PAUSER_ROLE, bob.address)).to.be.true
            await expect(zamaERC20.connect(bob).renounceRole(MINTING_PAUSER_ROLE, bob.address))
                .to.emit(zamaERC20, 'RoleRevoked')
                .withArgs(MINTING_PAUSER_ROLE, bob.address, bob.address)
            expect(await zamaERC20.hasRole(MINTING_PAUSER_ROLE, bob.address)).to.be.false
            await expect(zamaERC20.connect(bob).pauseMinting()).to.be.revertedWithCustomError(
                zamaERC20,
                'AccessControlUnauthorizedAccount'
            )
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
        beforeEach(async () => {
            // Grant alice the MINTER_ROLE
            // Grant bob the MINTING_PAUSER_ROLE
            // charlie has no roles
            await zamaERC20.connect(admin).grantRole(MINTER_ROLE, admin.address)
            await zamaERC20.connect(admin).grantRole(MINTING_PAUSER_ROLE, admin.address)

            // The contract is unpaused by default
            const isPaused = await zamaERC20.paused()
            expect(isPaused).to.be.false
        })

        describe('burn', () => {
            it('should burn token within balance', async () => {
                const TOKENS_TO_MINT = ethers.utils.parseEther('10')
                const TOKENS_TO_BURN = ethers.utils.parseEther('5')
                // Mint 10 $ZAMA to alice
                await zamaERC20.connect(admin).mint(alice.address, TOKENS_TO_MINT)
                expect(await zamaERC20.balanceOf(alice.address)).to.eq(TOKENS_TO_MINT)
                // Burn 5 tokens
                await zamaERC20.connect(alice).burn(TOKENS_TO_BURN)
                expect(await zamaERC20.balanceOf(alice.address)).to.eq(TOKENS_TO_MINT.sub(TOKENS_TO_BURN))
            })
            it('should not burn more token than available balance', async () => {
                const TOKENS_TO_MINT = ethers.utils.parseEther('10')
                const TOKENS_TO_BURN = ethers.utils.parseEther('100')
                // Mint 10 $ZAMA to alice
                await zamaERC20.connect(admin).mint(alice.address, TOKENS_TO_MINT)
                expect(await zamaERC20.balanceOf(alice.address)).to.eq(TOKENS_TO_MINT)
                // Try burning 100 $ZAMA
                await expect(zamaERC20.connect(alice).burn(TOKENS_TO_BURN)).to.be.revertedWithCustomError(
                    zamaERC20,
                    'ERC20InsufficientBalance'
                )
            })
        })
        describe('burnFrom', () => {
            const TOKENS_TO_MINT = ethers.utils.parseEther('10')
            const TOKENS_TO_BURN = ethers.utils.parseEther('2')
            const ALLOWANCE = ethers.utils.parseEther('2')
            const SMALL_ALLOWANCE = ethers.utils.parseEther('1')
            let totalSupply: BigNumber

            beforeEach(async () => {
                // Mint 10 $ZAMA to alice and bob
                await zamaERC20.connect(admin).mint(alice.address, TOKENS_TO_MINT)
                await zamaERC20.connect(admin).mint(bob.address, TOKENS_TO_MINT)
                totalSupply = await zamaERC20.totalSupply()
            })

            it('should let alice burn from its own address with allowance', async () => {
                await zamaERC20.connect(alice).approve(alice.address, ALLOWANCE)
                const allowanceBefore = await zamaERC20.allowance(alice.address, alice.address)
                const balanceBefore = await zamaERC20.balanceOf(alice.address)

                await zamaERC20.connect(alice).burnFrom(alice.address, TOKENS_TO_BURN)

                const allowanceAfter = await zamaERC20.allowance(alice.address, alice.address)
                const balanceAfter = await zamaERC20.balanceOf(alice.address)

                expect(await zamaERC20.totalSupply()).to.eq(totalSupply.sub(TOKENS_TO_BURN))
                expect(allowanceAfter).to.eq(allowanceBefore.sub(TOKENS_TO_BURN))
                expect(balanceAfter).to.eq(balanceBefore.sub(TOKENS_TO_BURN))
            })

            it('should let alice burn bob tokens with max allowance', async () => {
                await zamaERC20.connect(bob).approve(alice.address, ethers.constants.MaxUint256)
                const allowanceBefore = await zamaERC20.allowance(bob.address, alice.address)
                const balanceBefore = await zamaERC20.balanceOf(bob.address)

                await zamaERC20.connect(alice).burnFrom(bob.address, TOKENS_TO_BURN)

                const allowanceAfter = await zamaERC20.allowance(bob.address, alice.address)
                const balanceAfter = await zamaERC20.balanceOf(bob.address)

                expect(await zamaERC20.totalSupply()).to.eq(totalSupply.sub(TOKENS_TO_BURN))
                expect(allowanceAfter).to.eq(allowanceBefore)
                expect(balanceAfter).to.eq(balanceBefore.sub(TOKENS_TO_BURN))
            })

            it('should let alice burn bob tokens within allowance', async () => {
                await zamaERC20.connect(bob).approve(alice.address, ALLOWANCE)
                const allowanceBefore = await zamaERC20.allowance(bob.address, alice.address)
                const balanceBefore = await zamaERC20.balanceOf(bob.address)

                await zamaERC20.connect(alice).burnFrom(bob.address, TOKENS_TO_BURN)

                const allowanceAfter = await zamaERC20.allowance(bob.address, alice.address)
                const balanceAfter = await zamaERC20.balanceOf(bob.address)

                expect(await zamaERC20.totalSupply()).to.eq(totalSupply.sub(TOKENS_TO_BURN))
                expect(allowanceAfter).to.eq(allowanceBefore.sub(TOKENS_TO_BURN))
                expect(balanceAfter).to.eq(balanceBefore.sub(TOKENS_TO_BURN))
            })

            it('should not let alice burn from its own address without allowance', async () => {
                await expect(
                    zamaERC20.connect(alice).burnFrom(alice.address, TOKENS_TO_BURN)
                ).to.be.revertedWithCustomError(zamaERC20, 'ERC20InsufficientAllowance')
            })

            it('should not let alice burn bob tokens without allowance', async () => {
                await expect(
                    zamaERC20.connect(alice).burnFrom(bob.address, TOKENS_TO_BURN)
                ).to.be.revertedWithCustomError(zamaERC20, 'ERC20InsufficientAllowance')
            })

            it('should not let alice burn bob tokens besides bob allowance', async () => {
                // Bob allows alice to burn 1 $ZAMA
                await zamaERC20.connect(bob).approve(alice.address, SMALL_ALLOWANCE)
                // Alice tries to burn 2 $ZAMA from Bob
                await expect(
                    zamaERC20.connect(alice).burnFrom(bob.address, TOKENS_TO_BURN)
                ).to.be.revertedWithCustomError(zamaERC20, 'ERC20InsufficientAllowance')
            })

            it('should not let privileged addresses burn tokens without allowance', async () => {
                await expect(
                    zamaERC20.connect(admin).burnFrom(alice.address, TOKENS_TO_BURN)
                ).to.be.revertedWithCustomError(zamaERC20, 'ERC20InsufficientAllowance')
            })
        })
    })

    describe('Transfer', () => {
        describe('Transfer - ERC20', () => {
            describe('transfer', () => {
                it('', async () => {})
            })
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
        before(async () => {
            // Deploy ERC20Mock, ERC721Mock, ERC1155Mock
        })
        describe('recoverEther', () => {
            // Send Ether to the ZamaERC20 contract to simulate lost asset
            beforeEach(async () => {})
            it('should let DEFAULT_ADMIN_ROLE recover ether from contract', () => {})
            it('should not let PAUSER_ROLE recover ether from contract', () => {})
            it('should not let PAUSING_MINTER_ROLE recover ether from contract', () => {})
            it('should not let generic address recover ether from contract', () => {})
        })
        describe('recoverERC20', () => {
            // Send ERC20Mock to the ZamaERC20 contract to simulate lost asset
            beforeEach(async () => {})
            it('should let DEFAULT_ADMIN_ROLE recover ERC20 from contract', () => {})
            it('should not let PAUSER_ROLE recover ERC20 from contract', () => {})
            it('should not let PAUSING_MINTER_ROLE recover ERC20 from contract', () => {})
            it('should not let generic address recover ERC20 from contract', () => {})
        })
        describe('recoverERC721', () => {
            // Send ERC721Mock to the ZamaERC20 contract to simulate lost asset
            beforeEach(async () => {})
            it('should let DEFAULT_ADMIN_ROLE recover ERC721 from contract', () => {})
            it('should not let PAUSER_ROLE recover ERC721 from contract', () => {})
            it('should not let PAUSING_MINTER_ROLE recover ERC721 from contract', () => {})
            it('should not let generic address recover ERC721 from contract', () => {})
        })
        describe('recoverERC1155', () => {
            // Send ERC1155Mock to the ZamaERC20 contract to simulate lost asset
            beforeEach(async () => {})
            it('should let DEFAULT_ADMIN_ROLE recover ERC1155 from contract', () => {})
            it('should not let PAUSER_ROLE recover ERC1155 from contract', () => {})
            it('should not let PAUSING_MINTER_ROLE recover ERC1155 from contract', () => {})
            it('should not let generic address recover ERC1155 from contract', () => {})
        })
    })
})
