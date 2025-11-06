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

    describe('Multiple initial receivers', () => {
        it('should mint the correct amount for each initial receivers', async () => {
            const _ALICE_AMOUNT = 5_000_000_000n
            const _BOB_AMOUNT = 4_000_000_000n
            const _CHARLIE_AMOUNT = 2_000_000_000n

            const ALICE_AMOUNT = ethers.utils.parseEther(_ALICE_AMOUNT.toString())
            const BOB_AMOUNT = ethers.utils.parseEther(_BOB_AMOUNT.toString())
            const CHARLIE_AMOUNT = ethers.utils.parseEther(_CHARLIE_AMOUNT.toString())

            zamaERC20 = await zamaERC20Factory
                .connect(deployer)
                .deploy(
                    'ZAMAERC20',
                    'ZAMA',
                    [alice.address, bob.address, charlie.address],
                    [ALICE_AMOUNT, BOB_AMOUNT, CHARLIE_AMOUNT],
                    admin.address
                )

            expect(await zamaERC20.balanceOf(alice.address)).to.eq(ALICE_AMOUNT)
            expect(await zamaERC20.balanceOf(bob.address)).to.eq(BOB_AMOUNT)
            expect(await zamaERC20.balanceOf(charlie.address)).to.eq(CHARLIE_AMOUNT)

            expect(await zamaERC20.totalSupply()).to.eq(ALICE_AMOUNT.add(BOB_AMOUNT).add(CHARLIE_AMOUNT))
        })

        it('should revert when receivers address and amount length differ', async () => {
            const _AMOUNT = 5_000_000_000n
            const AMOUNT = ethers.utils.parseEther(_AMOUNT.toString())

            await expect(
                zamaERC20Factory
                    .connect(deployer)
                    .deploy('ZAMAERC20', 'ZAMA', [alice.address, bob.address, charlie.address], [AMOUNT], admin.address)
            ).to.be.revertedWithCustomError({ interface: zamaERC20Factory.interface }, 'AmountsReceiversLengthMismatch')
        })
    })

    describe('Single initial receiver', () => {
        const _INITIAL_MINT_AMOUNT = 11_000_000_000n
        const INITIAL_MINT_AMOUNT = ethers.utils.parseEther(_INITIAL_MINT_AMOUNT.toString())

        // beforeEach hook for setup that runs before each test in the block
        beforeEach(async () => {
            // The INITIAL_RECEIVER_0 and INITIAL_ADMIN can be different from the deployer.
            zamaERC20 = await zamaERC20Factory
                .connect(deployer)
                .deploy('ZAMAERC20', 'ZAMA', [owner.address], [INITIAL_MINT_AMOUNT], admin.address)
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
                const ownerBalance = await zamaERC20.balanceOf(owner.address)
                expect(ownerBalance).eql(INITIAL_MINT_AMOUNT)

                // Total Supply
                expect(await zamaERC20.totalSupply()).to.eq(INITIAL_MINT_AMOUNT)

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
                await expect(
                    zamaERC20.connect(bob).grantRole(MINTER_ROLE, alice.address)
                ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
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
                expect(await zamaERC20.hasRole(MINTER_ROLE, alice.address)).to.be.true
                expect(await zamaERC20.hasRole(MINTING_PAUSER_ROLE, bob.address)).to.be.true

                // The contract is unpaused by default
                const isPaused = await zamaERC20.paused()
                expect(isPaused).to.be.false
            })

            describe('Minting AccessControl', () => {
                it('should let MINTING_PAUSER_ROLE pause the minting', async () => {
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

                it('should not let MINTING_PAUSER_ROLE unpause the minting', async () => {
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
            })

            describe('Unpaused', () => {
                it('should let MINTER_ROLE mint $ZAMA while unpaused', async () => {
                    const aliceBalanceBefore = await zamaERC20.balanceOf(alice.address)
                    const totalSupplyBefore = await zamaERC20.totalSupply()

                    await zamaERC20.connect(alice).mint(alice.address, TOKENS_TO_MINT)

                    const aliceBalanceAfter: BigNumber = await zamaERC20.balanceOf(alice.address)
                    const totalSupplyAfter = await zamaERC20.totalSupply()

                    expect(aliceBalanceAfter).to.eql(aliceBalanceBefore.add(TOKENS_TO_MINT))
                    expect(totalSupplyAfter).to.eq(totalSupplyBefore.add(TOKENS_TO_MINT))
                })

                it('should not let DEFAULT_ADMIN_ROLE mint $ZAMA while unpaused', async () => {
                    await expect(
                        zamaERC20.connect(admin).mint(admin.address, TOKENS_TO_MINT)
                    ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                })

                it('should not let MINTING_PAUSER_ROLE mint $ZAMA while unpaused', async () => {
                    await expect(
                        zamaERC20.connect(bob).mint(bob.address, TOKENS_TO_MINT)
                    ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                })

                it('should not let generic address mint $ZAMA while unpaused', async () => {
                    await expect(
                        zamaERC20.connect(charlie).mint(charlie.address, TOKENS_TO_MINT)
                    ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                })
            })

            describe('Paused', () => {
                it('should not let MINTER_ROLE mint $ZAMA while paused', async () => {
                    await zamaERC20.connect(bob).pauseMinting()
                    const isPaused = await zamaERC20.paused()
                    expect(isPaused).to.be.true

                    await expect(
                        zamaERC20.connect(alice).mint(alice.address, TOKENS_TO_MINT)
                    ).to.be.revertedWithCustomError(zamaERC20, 'EnforcedPause')
                })

                it('should not let DEFAULT_ADMIN_ROLE mint $ZAMA while paused', async () => {
                    await zamaERC20.connect(bob).pauseMinting()
                    const isPaused = await zamaERC20.paused()
                    expect(isPaused).to.be.true

                    await expect(
                        zamaERC20.connect(admin).mint(admin.address, TOKENS_TO_MINT)
                    ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                })

                it('should not let MINTING_PAUSER_ROLE mint $ZAMA while paused', async () => {
                    await zamaERC20.connect(bob).pauseMinting()
                    const isPaused = await zamaERC20.paused()
                    expect(isPaused).to.be.true

                    await expect(
                        zamaERC20.connect(bob).mint(bob.address, TOKENS_TO_MINT)
                    ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
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
        })

        describe('Burn', () => {
            beforeEach(async () => {
                // Grant alice the MINTER_ROLE
                // Grant bob the MINTING_PAUSER_ROLE
                // charlie has no roles
                await zamaERC20.connect(admin).grantRole(MINTER_ROLE, admin.address)
                await zamaERC20.connect(admin).grantRole(MINTING_PAUSER_ROLE, admin.address)
                expect(await zamaERC20.hasRole(MINTER_ROLE, admin.address)).to.be.true
                expect(await zamaERC20.hasRole(MINTING_PAUSER_ROLE, admin.address)).to.be.true

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
            const TRANSFER_AMOUNT = ethers.utils.parseEther('10')

            beforeEach(async () => {
                // Grant alice the MINTER_ROLE
                // Grant bob the MINTING_PAUSER_ROLE
                // charlie has no roles
                await zamaERC20.connect(admin).grantRole(MINTER_ROLE, admin.address)
                await zamaERC20.connect(admin).grantRole(MINTING_PAUSER_ROLE, admin.address)
                expect(await zamaERC20.hasRole(MINTER_ROLE, admin.address)).to.be.true
                expect(await zamaERC20.hasRole(MINTING_PAUSER_ROLE, admin.address)).to.be.true

                // The contract is unpaused by default
                const isPaused = await zamaERC20.paused()
                expect(isPaused).to.be.false
            })

            describe('Transfer - ERC20', () => {
                describe('transfer', () => {
                    it('should fail when transfering to address(0)', async () => {
                        await expect(
                            zamaERC20.connect(alice).transfer(ethers.constants.AddressZero, TRANSFER_AMOUNT)
                        ).to.be.revertedWithCustomError(zamaERC20, 'ERC20InvalidReceiver')
                    })

                    it('should fail when transfering more token than available', async () => {
                        // ERC20InsufficientBalance
                        expect(await zamaERC20.balanceOf(alice.address)).to.be.lt(TRANSFER_AMOUNT)
                        await expect(
                            zamaERC20.connect(alice).transfer(bob.address, TRANSFER_AMOUNT)
                        ).to.be.revertedWithCustomError(zamaERC20, 'ERC20InsufficientBalance')
                    })

                    it('should let owner transfer to itself', async () => {
                        const balanceBefore = await zamaERC20.balanceOf(owner.address)

                        await expect(zamaERC20.connect(owner).transfer(owner.address, TRANSFER_AMOUNT)).to.not.be
                            .reverted

                        const balanceAfter = await zamaERC20.balanceOf(owner.address)

                        expect(balanceAfter).to.eql(balanceBefore)
                    })

                    it('should let owner transfer token to alice', async () => {
                        const balanceOwnerBefore = await zamaERC20.balanceOf(owner.address)
                        const balanceAliceBefore = await zamaERC20.balanceOf(alice.address)

                        await expect(zamaERC20.connect(owner).transfer(alice.address, TRANSFER_AMOUNT)).to.not.be
                            .reverted

                        const balanceOwnerAfter = await zamaERC20.balanceOf(owner.address)
                        const balanceAliceAfter = await zamaERC20.balanceOf(alice.address)

                        expect(balanceOwnerAfter).to.eql(balanceOwnerBefore.sub(TRANSFER_AMOUNT))
                        expect(balanceAliceAfter).to.eql(balanceAliceBefore.add(TRANSFER_AMOUNT))
                    })

                    it('should let owner transfer 0 token to alice', async () => {
                        const balanceOwnerBefore = await zamaERC20.balanceOf(owner.address)
                        const balanceAliceBefore = await zamaERC20.balanceOf(alice.address)

                        await expect(zamaERC20.connect(owner).transfer(alice.address, 0)).to.not.be.reverted

                        const balanceOwnerAfter = await zamaERC20.balanceOf(owner.address)
                        const balanceAliceAfter = await zamaERC20.balanceOf(alice.address)

                        expect(balanceOwnerAfter).to.eql(balanceOwnerBefore)
                        expect(balanceAliceAfter).to.eql(balanceAliceBefore)
                    })
                })

                describe('transferFrom', () => {
                    const MINT_AMOUNT = ethers.utils.parseEther('10')
                    const SMALL_MINT_AMOUNT = ethers.utils.parseEther('1')
                    const TRANSFER_AMOUNT = ethers.utils.parseEther('2')
                    const ALLOWANCE = ethers.utils.parseEther('2')
                    const SMALL_ALLOWANCE = ethers.utils.parseEther('1')

                    beforeEach(async () => {
                        await zamaERC20.connect(admin).mint(alice.address, MINT_AMOUNT)
                        await zamaERC20.connect(admin).mint(bob.address, SMALL_MINT_AMOUNT)
                    })

                    it('should fail when transfering with no allowance', async () => {
                        await expect(
                            zamaERC20.transferFrom(alice.address, bob.address, TRANSFER_AMOUNT)
                        ).to.be.revertedWithCustomError(zamaERC20, 'ERC20InsufficientAllowance')
                    })

                    it('should fail when transfering more than available allowance', async () => {
                        await zamaERC20.connect(alice).approve(bob.address, SMALL_ALLOWANCE)
                        await expect(
                            zamaERC20.connect(bob).transferFrom(alice.address, charlie.address, TRANSFER_AMOUNT)
                        ).to.be.revertedWithCustomError(zamaERC20, 'ERC20InsufficientAllowance')
                    })

                    it('should fail when transfering more than available token', async () => {
                        await zamaERC20.connect(alice).approve(bob.address, ethers.constants.MaxUint256)
                        await expect(
                            zamaERC20.connect(bob).transferFrom(alice.address, charlie.address, MINT_AMOUNT.mul(2))
                        ).to.be.revertedWithCustomError(zamaERC20, 'ERC20InsufficientBalance')
                    })

                    it('should let alice transfer from owner to bob within available allowance', async () => {
                        const allowanceBefore = await zamaERC20.allowance(alice.address, bob.address)
                        await zamaERC20.connect(alice).approve(bob.address, ALLOWANCE)
                        expect(await zamaERC20.allowance(alice.address, bob.address)).to.eql(
                            allowanceBefore.add(ALLOWANCE)
                        )

                        await zamaERC20.connect(bob).transferFrom(alice.address, charlie.address, TRANSFER_AMOUNT)

                        expect(await zamaERC20.allowance(alice.address, bob.address)).to.eql(allowanceBefore)
                    })

                    it('should let alice transfer from owner to bob with max allowance', async () => {
                        await zamaERC20.connect(alice).approve(bob.address, ethers.constants.MaxUint256)
                        expect(await zamaERC20.allowance(alice.address, bob.address)).to.eql(
                            ethers.constants.MaxUint256
                        )

                        await zamaERC20.connect(bob).transferFrom(alice.address, charlie.address, TRANSFER_AMOUNT)

                        expect(await zamaERC20.allowance(alice.address, bob.address)).to.eql(
                            ethers.constants.MaxUint256
                        )
                    })
                })
            })

            describe('Transfer - ERC20Permit', () => {})

            describe('Transfer - ERC1363', () => {
                describe('transferAndCall', () => {})
                describe('transferFromAndCall', () => {})
                describe('approveAndCall', () => {})
            })
        })

        describe('AssetRecoverer', () => {
            const SEND_AMOUNT = ethers.utils.parseEther('1')

            describe('recoverEther', () => {
                // Send Ether to the ZamaERC20 contract to simulate lost asset
                beforeEach(async () => {
                    // Deploy SelfDestructableMock to send ether to ZamaERC20 without `receive()` or `fallback()`
                    const SelfDestructableMockFactory = await ethers.getContractFactory('SelfDestructableMock')
                    await SelfDestructableMockFactory.deploy(zamaERC20.address, { value: SEND_AMOUNT })
                    expect(await ethers.provider.getBalance(zamaERC20.address)).to.eql(SEND_AMOUNT)
                })

                describe('Unpaused', () => {
                    beforeEach(async () => {
                        // Grant privileged roles to alice & bob
                        await zamaERC20.connect(admin).grantRole(MINTER_ROLE, alice.address)
                        await zamaERC20.connect(admin).grantRole(MINTING_PAUSER_ROLE, bob.address)
                        expect(await zamaERC20.hasRole(MINTER_ROLE, alice.address)).to.be.true
                        expect(await zamaERC20.hasRole(MINTING_PAUSER_ROLE, bob.address)).to.be.true
                    })

                    it('should let DEFAULT_ADMIN_ROLE recover ether from contract while unpaused', async () => {
                        const etherContractBalanceBefore = await ethers.provider.getBalance(zamaERC20.address)
                        const etherAliceBalanceBefore = await ethers.provider.getBalance(alice.address)

                        await zamaERC20.connect(admin).recoverEther(SEND_AMOUNT, alice.address)

                        const etherContractBalanceAfter = await ethers.provider.getBalance(zamaERC20.address)
                        const etherAliceBalanceAfter = await ethers.provider.getBalance(alice.address)

                        expect(etherContractBalanceAfter).to.eq(etherContractBalanceBefore.sub(SEND_AMOUNT))
                        expect(etherAliceBalanceAfter).to.eq(etherAliceBalanceBefore.add(SEND_AMOUNT))
                    })

                    it('should fail when trying to recover more than available balance', async () => {
                        await expect(
                            zamaERC20.connect(admin).recoverEther(SEND_AMOUNT.mul(2), alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'FailedToSendEther')
                    })

                    it('should not let MINTER_ROLE recover ether from contract while unpaused', async () => {
                        await expect(
                            zamaERC20.connect(alice).recoverEther(SEND_AMOUNT, alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                    })

                    it('should not let MINTING_PAUSER_ROLE recover ether from contract while unpaused', async () => {
                        await expect(
                            zamaERC20.connect(bob).recoverEther(SEND_AMOUNT, alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                    })

                    it('should not let generic address recover ether from contract while unpaused', async () => {
                        await expect(
                            zamaERC20.connect(charlie).recoverEther(SEND_AMOUNT, alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                    })
                })

                describe('Paused', () => {
                    before(async () => {
                        // Grant privileged roles to alice & bob
                        await zamaERC20.connect(admin).grantRole(MINTER_ROLE, alice.address)
                        await zamaERC20.connect(admin).grantRole(MINTING_PAUSER_ROLE, bob.address)
                        await zamaERC20.connect(bob).pauseMinting()
                    })

                    it('should let DEFAULT_ADMIN_ROLE recover ether from contract while paused', async () => {
                        const etherContractBalanceBefore = await ethers.provider.getBalance(zamaERC20.address)
                        const etherAliceBalanceBefore = await ethers.provider.getBalance(alice.address)

                        await zamaERC20.connect(admin).recoverEther(SEND_AMOUNT, alice.address)

                        const etherContractBalanceAfter = await ethers.provider.getBalance(zamaERC20.address)
                        const etherAliceBalanceAfter = await ethers.provider.getBalance(alice.address)

                        expect(etherContractBalanceAfter).to.eq(etherContractBalanceBefore.sub(SEND_AMOUNT))
                        expect(etherAliceBalanceAfter).to.eq(etherAliceBalanceBefore.add(SEND_AMOUNT))
                    })

                    it('should not send recovered ETH to null address', async () => {
                        await expect(
                            zamaERC20.connect(admin).recoverEther(SEND_AMOUNT, ethers.constants.AddressZero)
                        ).to.be.revertedWithCustomError(zamaERC20, 'InvalidNullRecipient')
                    })

                    it('should fail when trying to recover more than available balance', async () => {
                        await expect(
                            zamaERC20.connect(admin).recoverEther(SEND_AMOUNT.mul(2), alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'FailedToSendEther')
                    })

                    it('should not let MINTER_ROLE recover ether from contract while paused', async () => {
                        await expect(
                            zamaERC20.connect(alice).recoverEther(SEND_AMOUNT, alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                    })

                    it('should not let MINTING_PAUSER_ROLE recover ether from contract while paused', async () => {
                        await expect(
                            zamaERC20.connect(bob).recoverEther(SEND_AMOUNT, alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                    })

                    it('should not let generic address recover ether from contract while paused', async () => {
                        await expect(
                            zamaERC20.connect(charlie).recoverEther(SEND_AMOUNT, alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                    })
                })
            })

            describe('recoverERC20', () => {
                // Send ERC20Mock to the ZamaERC20 contract to simulate lost asset
                describe('Unpaused', () => {
                    beforeEach(async () => {
                        // Grant privileged roles to alice & bob
                        await zamaERC20.connect(admin).grantRole(MINTER_ROLE, alice.address)
                        await zamaERC20.connect(admin).grantRole(MINTING_PAUSER_ROLE, bob.address)
                        expect(await zamaERC20.hasRole(MINTER_ROLE, alice.address)).to.be.true
                        expect(await zamaERC20.hasRole(MINTING_PAUSER_ROLE, bob.address)).to.be.true

                        await zamaERC20.connect(alice).mint(zamaERC20.address, SEND_AMOUNT)
                        expect(await zamaERC20.balanceOf(zamaERC20.address)).to.eq(SEND_AMOUNT)
                    })

                    it('should let DEFAULT_ADMIN_ROLE recover available $ZAMA from contract while unpaused', async () => {
                        const mockERC20ContractBalanceBefore = await zamaERC20.balanceOf(zamaERC20.address)
                        const mockERC20AliceBalanceBefore = await zamaERC20.balanceOf(alice.address)

                        await zamaERC20.connect(admin).recoverERC20(zamaERC20.address, SEND_AMOUNT, alice.address)

                        const mockERC20ContractBalanceAfter = await zamaERC20.balanceOf(zamaERC20.address)
                        const mockERC20AliceBalanceAfter = await zamaERC20.balanceOf(alice.address)

                        expect(mockERC20ContractBalanceAfter).to.eq(mockERC20ContractBalanceBefore.sub(SEND_AMOUNT))
                        expect(mockERC20AliceBalanceAfter).to.eq(mockERC20AliceBalanceBefore.add(SEND_AMOUNT))
                    })

                    it('should not send recovered ERC20 to null address', async () => {
                        await expect(
                            zamaERC20
                                .connect(admin)
                                .recoverERC20(zamaERC20.address, SEND_AMOUNT, ethers.constants.AddressZero)
                        ).to.be.revertedWithCustomError(zamaERC20, 'InvalidNullRecipient')
                    })

                    it('should let DEFAULT_ADMIN_ROLE recover 0 $ZAMA from contract while unpaused', async () => {
                        const mockERC20ContractBalanceBefore = await zamaERC20.balanceOf(zamaERC20.address)
                        const mockERC20AliceBalanceBefore = await zamaERC20.balanceOf(alice.address)

                        await zamaERC20.connect(admin).recoverERC20(zamaERC20.address, 0, alice.address)

                        const mockERC20ContractBalanceAfter = await zamaERC20.balanceOf(zamaERC20.address)
                        const mockERC20AliceBalanceAfter = await zamaERC20.balanceOf(alice.address)

                        expect(mockERC20ContractBalanceAfter).to.eq(mockERC20ContractBalanceBefore)
                        expect(mockERC20AliceBalanceAfter).to.eq(mockERC20AliceBalanceBefore)
                    })

                    it('should fail when trying to recover more $ZAMA than available', async () => {
                        await expect(
                            zamaERC20.connect(admin).recoverERC20(zamaERC20.address, SEND_AMOUNT.mul(2), alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'ERC20InsufficientBalance')
                    })

                    it('should not let MINTER_ROLE recover $ZAMA from contract while unpaused', async () => {
                        await expect(
                            zamaERC20.connect(alice).recoverERC20(zamaERC20.address, SEND_AMOUNT, alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                    })

                    it('should not let MINTING_PAUSER_ROLE recover $ZAMA from contract while unpaused', async () => {
                        await expect(
                            zamaERC20.connect(alice).recoverERC20(zamaERC20.address, SEND_AMOUNT, alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                    })

                    it('should not let generic address recover $ZAMA from contract while unpaused', async () => {
                        await expect(
                            zamaERC20.connect(alice).recoverERC20(zamaERC20.address, SEND_AMOUNT, alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                    })
                })

                describe('Paused', () => {
                    beforeEach(async () => {
                        // Grant privileged roles to alice & bob
                        await zamaERC20.connect(admin).grantRole(MINTER_ROLE, alice.address)
                        await zamaERC20.connect(admin).grantRole(MINTING_PAUSER_ROLE, bob.address)
                        expect(await zamaERC20.hasRole(MINTER_ROLE, alice.address)).to.be.true
                        expect(await zamaERC20.hasRole(MINTING_PAUSER_ROLE, bob.address)).to.be.true

                        await zamaERC20.connect(alice).mint(zamaERC20.address, SEND_AMOUNT)
                        expect(await zamaERC20.balanceOf(zamaERC20.address)).to.eq(SEND_AMOUNT)

                        await zamaERC20.connect(bob).pauseMinting()
                    })

                    it('should let DEFAULT_ADMIN_ROLE recover available $ZAMA from contract while paused', async () => {
                        const mockERC20ContractBalanceBefore = await zamaERC20.balanceOf(zamaERC20.address)
                        const mockERC20AliceBalanceBefore = await zamaERC20.balanceOf(alice.address)

                        await zamaERC20.connect(admin).recoverERC20(zamaERC20.address, SEND_AMOUNT, alice.address)

                        const mockERC20ContractBalanceAfter = await zamaERC20.balanceOf(zamaERC20.address)
                        const mockERC20AliceBalanceAfter = await zamaERC20.balanceOf(alice.address)

                        expect(mockERC20ContractBalanceAfter).to.eq(mockERC20ContractBalanceBefore.sub(SEND_AMOUNT))
                        expect(mockERC20AliceBalanceAfter).to.eq(mockERC20AliceBalanceBefore.add(SEND_AMOUNT))
                    })

                    it('should let DEFAULT_ADMIN_ROLE recover 0 $ZAMA from contract while paused', async () => {
                        const mockERC20ContractBalanceBefore = await zamaERC20.balanceOf(zamaERC20.address)
                        const mockERC20AliceBalanceBefore = await zamaERC20.balanceOf(alice.address)

                        await zamaERC20.connect(admin).recoverERC20(zamaERC20.address, 0, alice.address)

                        const mockERC20ContractBalanceAfter = await zamaERC20.balanceOf(zamaERC20.address)
                        const mockERC20AliceBalanceAfter = await zamaERC20.balanceOf(alice.address)

                        expect(mockERC20ContractBalanceAfter).to.eq(mockERC20ContractBalanceBefore)
                        expect(mockERC20AliceBalanceAfter).to.eq(mockERC20AliceBalanceBefore)
                    })

                    it('should fail when trying to recover more $ZAMA than available', async () => {
                        await expect(
                            zamaERC20.connect(admin).recoverERC20(zamaERC20.address, SEND_AMOUNT.mul(2), alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'ERC20InsufficientBalance')
                    })

                    it('should not let MINTER_ROLE recover $ZAMA from contract while paused', async () => {
                        await expect(
                            zamaERC20.connect(alice).recoverERC20(zamaERC20.address, SEND_AMOUNT, alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                    })

                    it('should not let MINTING_PAUSER_ROLE recover $ZAMA from contract while paused', async () => {
                        await expect(
                            zamaERC20.connect(alice).recoverERC20(zamaERC20.address, SEND_AMOUNT, alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                    })

                    it('should not let generic address recover $ZAMA from contract while paused', async () => {
                        await expect(
                            zamaERC20.connect(alice).recoverERC20(zamaERC20.address, SEND_AMOUNT, alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                    })
                })
            })

            describe('recoverERC721', () => {
                let ERC721MockFactory: ContractFactory
                let ERC721Mock: Contract

                const TOKEN_ID = 1

                describe('Unpaused', () => {
                    beforeEach(async () => {
                        // Grant privileged roles to alice & bob
                        await zamaERC20.connect(admin).grantRole(MINTER_ROLE, alice.address)
                        await zamaERC20.connect(admin).grantRole(MINTING_PAUSER_ROLE, bob.address)
                        expect(await zamaERC20.hasRole(MINTER_ROLE, alice.address)).to.be.true
                        expect(await zamaERC20.hasRole(MINTING_PAUSER_ROLE, bob.address)).to.be.true

                        // Deploy ERC721Mock
                        ERC721MockFactory = await ethers.getContractFactory('ERC721Mock')
                        ERC721Mock = await ERC721MockFactory.deploy('ERC721Mock', 'MOCK')

                        // Send ERC721Mock to the ZamaERC20 contract to simulate lost asset
                        await ERC721Mock.mint(zamaERC20.address, TOKEN_ID)
                        expect(await ERC721Mock.ownerOf(TOKEN_ID)).to.eq(zamaERC20.address)
                    })

                    it('should let DEFAULT_ADMIN_ROLE recover ERC721Mock from contract while unpaused', async () => {
                        await zamaERC20.connect(admin).recoverERC721(ERC721Mock.address, TOKEN_ID, alice.address)
                        expect(await ERC721Mock.ownerOf(TOKEN_ID)).to.eq(alice.address)
                    })

                    it('should not let MINTER_ROLE recover ERC721Mock from contract while unpaused', async () => {
                        await expect(
                            zamaERC20.connect(alice).recoverERC721(ERC721Mock.address, TOKEN_ID, alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                    })

                    it('should not let MINTING_PAUSER_ROLE recover ERC721Mock from contract while unpaused', async () => {
                        await expect(
                            zamaERC20.connect(bob).recoverERC721(ERC721Mock.address, TOKEN_ID, alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                    })

                    it('should not let generic address recover ERC721Mock from contract while unpaused', async () => {
                        await expect(
                            zamaERC20.connect(charlie).recoverERC721(ERC721Mock.address, TOKEN_ID, alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                    })
                })

                describe('Paused', () => {
                    beforeEach(async () => {
                        // Grant privileged roles to alice & bob
                        await zamaERC20.connect(admin).grantRole(MINTER_ROLE, alice.address)
                        await zamaERC20.connect(admin).grantRole(MINTING_PAUSER_ROLE, bob.address)
                        expect(await zamaERC20.hasRole(MINTER_ROLE, alice.address)).to.be.true
                        expect(await zamaERC20.hasRole(MINTING_PAUSER_ROLE, bob.address)).to.be.true

                        // Deploy ERC721Mock
                        ERC721MockFactory = await ethers.getContractFactory('ERC721Mock')
                        ERC721Mock = await ERC721MockFactory.deploy('ERC721Mock', 'MOCK')

                        // Send ERC721Mock to the ZamaERC20 contract to simulate lost asset
                        await ERC721Mock.mint(zamaERC20.address, TOKEN_ID)
                        expect(await ERC721Mock.ownerOf(TOKEN_ID)).to.eq(zamaERC20.address)

                        // Pause the ZamaERC20 contract
                        await zamaERC20.connect(bob).pauseMinting()
                        expect(await zamaERC20.paused()).to.be.true
                    })

                    it('should let DEFAULT_ADMIN_ROLE recover ERC721Mock from contract while paused', async () => {
                        await zamaERC20.connect(admin).recoverERC721(ERC721Mock.address, TOKEN_ID, alice.address)
                        expect(await ERC721Mock.ownerOf(TOKEN_ID)).to.eq(alice.address)
                    })

                    it('should not send recovered ERC721 to null address', async () => {
                        await expect(
                            zamaERC20
                                .connect(admin)
                                .recoverERC721(ERC721Mock.address, TOKEN_ID, ethers.constants.AddressZero)
                        ).to.be.revertedWithCustomError(zamaERC20, 'InvalidNullRecipient')
                    })

                    it('should not let MINTER_ROLE recover ERC721Mock from contract while paused', async () => {
                        await expect(
                            zamaERC20.connect(alice).recoverERC721(ERC721Mock.address, TOKEN_ID, alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                    })

                    it('should not let MINTING_PAUSER_ROLE recover ERC721Mock from contract while paused', async () => {
                        await expect(
                            zamaERC20.connect(bob).recoverERC721(ERC721Mock.address, TOKEN_ID, alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                    })

                    it('should not let generic address recover ERC721Mock from contract while paused', async () => {
                        await expect(
                            zamaERC20.connect(charlie).recoverERC721(ERC721Mock.address, TOKEN_ID, alice.address)
                        ).to.be.revertedWithCustomError(zamaERC20, 'AccessControlUnauthorizedAccount')
                    })
                })
            })
        })
    })
})
