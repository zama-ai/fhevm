import { SignerWithAddress } from '@nomiclabs/hardhat-ethers/signers'
import { expect } from 'chai'
import { Contract, ContractFactory } from 'ethers'
import { ethers, run } from 'hardhat'

describe('ZamaERC20 Role task suite', function () {
    let zamaERC20Factory: ContractFactory
    let zamaERC20: Contract
    let deployer: SignerWithAddress
    let minter: SignerWithAddress
    let pausing: SignerWithAddress
    let minterRole: string
    let pausingRole: string
    let defaultAdminRole: string

    before(async () => {
        ;[deployer, minter, pausing] = await ethers.getSigners()

        zamaERC20Factory = await ethers.getContractFactory('ZamaERC20')
    })

    beforeEach(async () => {
        const _INITIAL_MINT_AMOUNT = 11_000_000_000n
        const INITIAL_MINT_AMOUNT = ethers.utils.parseEther(_INITIAL_MINT_AMOUNT.toString())
        zamaERC20 = await zamaERC20Factory
            .connect(deployer)
            .deploy('ZAMAERC20', 'ZAMA', [deployer.address], [INITIAL_MINT_AMOUNT], deployer.address)

        minterRole = await zamaERC20.MINTER_ROLE()
        pausingRole = await zamaERC20.MINTING_PAUSER_ROLE()
        defaultAdminRole = await zamaERC20.DEFAULT_ADMIN_ROLE()
    })

    describe('MINTER_ROLE', async () => {
        it('grants and revokes MINTER_ROLE via tasks', async () => {
            expect(await zamaERC20.hasRole(minterRole, minter.address)).to.be.false

            await run('zama:erc20:grant:minter_role', { address: minter.address, contractAddress: zamaERC20.address })
            expect(await zamaERC20.hasRole(minterRole, minter.address)).to.be.true

            await run('zama:erc20:revoke:minter_role', { address: minter.address, contractAddress: zamaERC20.address })
            expect(await zamaERC20.hasRole(minterRole, minter.address)).to.be.false
        })

        it('renounces MINTER_ROLE for the connected signer via task', async () => {
            expect(await zamaERC20.hasRole(minterRole, deployer.address)).to.be.false

            await run('zama:erc20:grant:minter_role', { address: deployer.address, contractAddress: zamaERC20.address })
            expect(await zamaERC20.hasRole(minterRole, deployer.address)).to.be.true

            await run('zama:erc20:renounce:minter_role', { contractAddress: zamaERC20.address })
            expect(await zamaERC20.hasRole(minterRole, deployer.address)).to.be.false
        })

        it('should fail if grantee already has the MINTER_ROLE', async () => {
            await run('zama:erc20:grant:minter_role', {
                address: minter.address,
                contractAddress: zamaERC20.address,
            })

            expect(await zamaERC20.hasRole(minterRole, minter.address)).to.be.true

            await expect(
                run('zama:erc20:grant:minter_role', {
                    address: minter.address,
                    contractAddress: zamaERC20.address,
                })
            ).to.be.rejected
        })

        it('should fail if revokee does not have the MINTER_ROLE', async () => {
            expect(await zamaERC20.hasRole(minterRole, minter.address)).to.be.false

            await expect(
                run('zama:erc20:revoke:minter_role', {
                    address: minter.address,
                    contractAddress: zamaERC20.address,
                })
            ).to.be.rejected
        })

        it('should fail if signer tries to renounce to a MINTER_ROLE it does not have', async () => {
            expect(await zamaERC20.hasRole(minterRole, minter.address)).to.be.false

            await expect(
                run('zama:erc20:renounce:minter_role', {
                    contractAddress: zamaERC20.address,
                })
            ).to.be.rejected
        })

        it('should fail to grant MINTER_ROLE if signer is not the role admin', async () => {
            // The resolveContext() function takes the first element from the array returned by `ethers.getSigners()`
            // Thus, to run the task with the deployer/signer not being the owner, we renounce to the admin role of the contract roles (DEFAULT_ADMIN_ROLE).
            const roleAdmin = await zamaERC20.getRoleAdmin(minterRole)
            expect(await zamaERC20.hasRole(roleAdmin, deployer.address)).to.be.true
            await run('zama:erc20:renounce:default_admin_role', { contractAddress: zamaERC20.address })

            expect(await zamaERC20.hasRole(roleAdmin, deployer.address)).to.be.false

            await expect(
                run('zama:erc20:grant:minter_role', {
                    address: deployer.address,
                    contractAddress: zamaERC20.address,
                })
            ).to.be.rejected
        })

        it('should fail to revoke MINTER_ROLE if signer is not the role admin', async () => {
            // The resolveContext() function takes the first element from the array returned by `ethers.getSigners()`
            // Thus, to run the task with the deployer/signer not being the owner, we renounce to the admin role of the contract roles (DEFAULT_ADMIN_ROLE).
            const roleAdmin = await zamaERC20.getRoleAdmin(minterRole)
            expect(await zamaERC20.hasRole(roleAdmin, deployer.address)).to.be.true
            await run('zama:erc20:grant:minter_role', {
                address: minter.address,
                contractAddress: zamaERC20.address,
            })
            expect(await zamaERC20.hasRole(minterRole, minter.address)).to.be.true
            await run('zama:erc20:renounce:default_admin_role', { contractAddress: zamaERC20.address })

            expect(await zamaERC20.hasRole(roleAdmin, deployer.address)).to.be.false

            await expect(
                run('zama:erc20:revoke:minter_role', {
                    address: deployer.address,
                    contractAddress: zamaERC20.address,
                })
            ).to.be.rejected
        })

        it('should fail to renounce MINTER_ROLE if signer does not have the role', async () => {
            expect(await zamaERC20.hasRole(minterRole, deployer.address)).to.be.false

            await expect(
                run('zama:erc20:renounce:minter_role', {
                    contractAddress: zamaERC20.address,
                })
            ).to.be.rejected
        })
    })

    describe('MINTING_PAUSER_ROLE', async () => {
        it('grants and revokes MINTING_PAUSER_ROLE via tasks', async () => {
            expect(await zamaERC20.hasRole(pausingRole, pausing.address)).to.be.false

            await run('zama:erc20:grant:minting_pauser_role', {
                address: pausing.address,
                contractAddress: zamaERC20.address,
            })
            expect(await zamaERC20.hasRole(pausingRole, pausing.address)).to.be.true

            await run('zama:erc20:revoke:minting_pauser_role', {
                address: pausing.address,
                contractAddress: zamaERC20.address,
            })
            expect(await zamaERC20.hasRole(pausingRole, pausing.address)).to.be.false
        })

        it('renounces MINTING_PAUSER_ROLE for the connected signer via task', async () => {
            expect(await zamaERC20.hasRole(pausingRole, deployer.address)).to.be.false

            await run('zama:erc20:grant:minting_pauser_role', {
                address: deployer.address,
                contractAddress: zamaERC20.address,
            })
            expect(await zamaERC20.hasRole(pausingRole, deployer.address)).to.be.true

            await run('zama:erc20:renounce:minting_pauser_role', { contractAddress: zamaERC20.address })
            expect(await zamaERC20.hasRole(pausingRole, deployer.address)).to.be.false
        })

        it('should fail if grantee already has the MINTING_PAUSER_ROLE', async () => {
            await run('zama:erc20:grant:minting_pauser_role', {
                address: pausing.address,
                contractAddress: zamaERC20.address,
            })

            expect(await zamaERC20.hasRole(pausingRole, pausing.address)).to.be.true

            await expect(
                run('zama:erc20:grant:minting_pauser_role', {
                    address: pausing.address,
                    contractAddress: zamaERC20.address,
                })
            ).to.be.rejected
        })

        it('should fail if revokee does not have the MINTING_PAUSER_ROLE', async () => {
            expect(await zamaERC20.hasRole(pausingRole, pausing.address)).to.be.false

            await expect(
                run('zama:erc20:revoke:minting_pauser_role', {
                    address: pausing.address,
                    contractAddress: zamaERC20.address,
                })
            ).to.be.rejected
        })

        it('should fail if signer tries to renounce to a MINTING_PAUSER_ROLE it does not have', async () => {
            expect(await zamaERC20.hasRole(pausingRole, pausing.address)).to.be.false

            await expect(
                run('zama:erc20:renounce:minting_pauser_role', {
                    contractAddress: zamaERC20.address,
                })
            ).to.be.rejected
        })

        it('should fail to grant MINTING_PAUSER_ROLE if signer is not the role admin', async () => {
            // The resolveContext() function takes the first element from the array returned by `ethers.getSigners()`
            // Thus, to run the task with the deployer/signer not being the owner, we renounce to the admin role of the contract roles (DEFAULT_ADMIN_ROLE).
            const roleAdmin = await zamaERC20.getRoleAdmin(pausingRole)
            expect(await zamaERC20.hasRole(roleAdmin, deployer.address)).to.be.true
            await run('zama:erc20:renounce:default_admin_role', { contractAddress: zamaERC20.address })

            expect(await zamaERC20.hasRole(roleAdmin, deployer.address)).to.be.false

            await expect(
                run('zama:erc20:grant:minting_pauser_role', {
                    address: deployer.address,
                    contractAddress: zamaERC20.address,
                })
            ).to.be.rejected
        })

        it('should fail to revoke MINTING_PAUSER_ROLE if signer is not the role admin', async () => {
            // The resolveContext() function takes the first element from the array returned by `ethers.getSigners()`
            // Thus, to run the task with the deployer/signer not being the owner, we renounce to the admin role of the contract roles (DEFAULT_ADMIN_ROLE).
            const roleAdmin = await zamaERC20.getRoleAdmin(pausingRole)
            expect(await zamaERC20.hasRole(roleAdmin, deployer.address)).to.be.true
            await run('zama:erc20:renounce:default_admin_role', { contractAddress: zamaERC20.address })

            expect(await zamaERC20.hasRole(roleAdmin, deployer.address)).to.be.false

            await expect(
                run('zama:erc20:revoke:minting_pauser_role', {
                    address: deployer.address,
                    contractAddress: zamaERC20.address,
                })
            ).to.be.rejected
        })

        it('should fail to renounce MINTING_PAUSER_ROLE if signer does not have the role', async () => {
            expect(await zamaERC20.hasRole(pausingRole, deployer.address)).to.be.false

            await expect(
                run('zama:erc20:renounce:minting_pauser_role', {
                    contractAddress: zamaERC20.address,
                })
            ).to.be.rejected
        })
    })

    describe('DEFAULT_ADMIN_ROLE', async () => {
        it('grants and revokes DEFAULT_ADMIN_ROLE via tasks', async () => {
            expect(await zamaERC20.hasRole(defaultAdminRole, minter.address)).to.be.false

            await run('zama:erc20:grant:default_admin_role', {
                address: minter.address,
                contractAddress: zamaERC20.address,
            })
            expect(await zamaERC20.hasRole(defaultAdminRole, minter.address)).to.be.true

            await run('zama:erc20:revoke:default_admin_role', {
                address: minter.address,
                contractAddress: zamaERC20.address,
            })
            expect(await zamaERC20.hasRole(defaultAdminRole, minter.address)).to.be.false
        })

        it('renounces DEFAULT_ADMIN_ROLE for the connected signer via task', async () => {
            expect(await zamaERC20.hasRole(defaultAdminRole, deployer.address)).to.be.true

            await run('zama:erc20:renounce:default_admin_role', { contractAddress: zamaERC20.address })
            expect(await zamaERC20.hasRole(defaultAdminRole, deployer.address)).to.be.false
        })

        it('should fail if grantee already has the DEFAULT_ADMIN_ROLE', async () => {
            expect(await zamaERC20.hasRole(defaultAdminRole, deployer.address)).to.be.true

            await expect(
                run('zama:erc20:grant:default_admin_role', {
                    address: deployer.address,
                    contractAddress: zamaERC20.address,
                })
            ).to.be.rejected
        })

        it('should fail if revokee does not have the DEFAULT_ADMIN_ROLE', async () => {
            expect(await zamaERC20.hasRole(defaultAdminRole, minter.address)).to.be.false

            await expect(
                run('zama:erc20:revoke:default_admin_role', {
                    address: minter.address,
                    contractAddress: zamaERC20.address,
                })
            ).to.be.rejected
        })

        it('should fail if signer tries to renounce to a DEFAULT_ADMIN_ROLE it does not have', async () => {
            // The resolveContext() function takes the first element from the array returned by `ethers.getSigners()`
            // Thus, to run the task with the deployer/signer not being the owner, we renounce to the admin role of the contract roles (DEFAULT_ADMIN_ROLE).
            await run('zama:erc20:renounce:default_admin_role', { contractAddress: zamaERC20.address })
            expect(await zamaERC20.hasRole(defaultAdminRole, pausing.address)).to.be.false

            await expect(
                run('zama:erc20:renounce:default_admin_role', {
                    contractAddress: zamaERC20.address,
                })
            ).to.be.rejected
        })

        it('should fail to grant DEFAULT_ADMIN_ROLE if signer is not the role admin', async () => {
            // The resolveContext() function takes the first element from the array returned by `ethers.getSigners()`
            // Thus, to run the task with the deployer/signer not being the owner, we renounce to the admin role of the contract roles (DEFAULT_ADMIN_ROLE).
            const roleAdmin = await zamaERC20.getRoleAdmin(defaultAdminRole)
            expect(await zamaERC20.hasRole(roleAdmin, deployer.address)).to.be.true
            await run('zama:erc20:renounce:default_admin_role', { contractAddress: zamaERC20.address })

            expect(await zamaERC20.hasRole(roleAdmin, deployer.address)).to.be.false

            await expect(
                run('zama:erc20:grant:default_admin_role', {
                    address: deployer.address,
                    contractAddress: zamaERC20.address,
                })
            ).to.be.rejected
        })

        it('should fail to revoke DEFAULT_ADMIN_ROLE if signer is not the role admin', async () => {
            // The resolveContext() function takes the first element from the array returned by `ethers.getSigners()`
            // Thus, to run the task with the deployer/signer not being the owner, we renounce to the admin role of the contract roles (DEFAULT_ADMIN_ROLE).
            const roleAdmin = await zamaERC20.getRoleAdmin(defaultAdminRole)
            expect(await zamaERC20.hasRole(roleAdmin, deployer.address)).to.be.true
            await run('zama:erc20:renounce:default_admin_role', { contractAddress: zamaERC20.address })

            expect(await zamaERC20.hasRole(roleAdmin, deployer.address)).to.be.false

            await expect(
                run('zama:erc20:revoke:default_admin_role', {
                    address: deployer.address,
                    contractAddress: zamaERC20.address,
                })
            ).to.be.rejected
        })

        it('should fail to renounce DEFAULT_ADMIN_ROLE if signer does not have the role', async () => {
            expect(await zamaERC20.hasRole(defaultAdminRole, deployer.address)).to.be.true
            await run('zama:erc20:renounce:default_admin_role', { contractAddress: zamaERC20.address })

            expect(await zamaERC20.hasRole(defaultAdminRole, deployer.address)).to.be.false

            await expect(
                run('zama:erc20:renounce:default_admin_role', {
                    contractAddress: zamaERC20.address,
                })
            ).to.be.rejected
        })
    })
})
