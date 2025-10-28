import { expect } from 'chai'
import { Contract } from 'ethers'
import { deployments, ethers, network, run } from 'hardhat'

describe('ZamaERC20 task suite', function () {
    let zamaErc20: Contract
    let deployer: string
    let minter: string
    let pausing: string
    let minterRole: string
    let pausingRole: string
    let defaultAdminRole: string
    let snapshotId: string

    before(async () => {
        const [deployerSigner, minterSigner, pausingSigner] = await ethers.getSigners()
        deployer = deployerSigner.address
        minter = minterSigner.address
        pausing = pausingSigner.address

        const { deploy } = deployments
        const deleteFn = (deployments as unknown as { delete?: (name: string) => Promise<void> }).delete
        if (deleteFn) {
            await deleteFn('ZamaERC20')
        }

        const deployment = await deploy('ZamaERC20', {
            from: deployer,
            args: ['ZAMAERC20', 'ZAMA', deployer, deployer],
            log: false,
            skipIfAlreadyDeployed: false,
        })

        zamaErc20 = await ethers.getContractAt('ZamaERC20', deployment.address, deployerSigner)
        minterRole = await zamaErc20.MINTER_ROLE()
        pausingRole = await zamaErc20.MINTING_PAUSER_ROLE()
        defaultAdminRole = await zamaErc20.DEFAULT_ADMIN_ROLE()

        snapshotId = await network.provider.send('evm_snapshot', [])
    })

    beforeEach(async () => {
        await network.provider.send('evm_revert', [snapshotId])
        snapshotId = await network.provider.send('evm_snapshot', [])
    })

    it('grants and revokes MINTER_ROLE via tasks', async () => {
        expect(await zamaErc20.hasRole(minterRole, minter)).to.be.false

        await run('zama:erc20:grant:minter_role', { address: minter })
        expect(await zamaErc20.hasRole(minterRole, minter)).to.be.true

        await run('zama:erc20:revoke:minter_role', { address: minter })
        expect(await zamaErc20.hasRole(minterRole, minter)).to.be.false
    })

    it('grants and revokes PAUSING_MINTER_ROLE via tasks', async () => {
        expect(await zamaErc20.hasRole(pausingRole, pausing)).to.be.false

        await run('zama:erc20:grant:pausing_minter_role', { address: pausing })
        expect(await zamaErc20.hasRole(pausingRole, pausing)).to.be.true

        await run('zama:erc20:revoke:pausing_minter_role', { address: pausing })
        expect(await zamaErc20.hasRole(pausingRole, pausing)).to.be.false
    })

    it('grants and revokes DEFAULT_ADMIN_ROLE via tasks', async () => {
        expect(await zamaErc20.hasRole(defaultAdminRole, minter)).to.be.false

        await run('zama:erc20:grant:default_admin_role', { address: minter })
        expect(await zamaErc20.hasRole(defaultAdminRole, minter)).to.be.true

        await run('zama:erc20:revoke:default_admin_role', { address: minter })
        expect(await zamaErc20.hasRole(defaultAdminRole, minter)).to.be.false
    })

    it('renounces MINTER_ROLE for the connected signer via task', async () => {
        expect(await zamaErc20.hasRole(minterRole, deployer)).to.be.false

        await run('zama:erc20:grant:minter_role', { address: deployer })
        expect(await zamaErc20.hasRole(minterRole, deployer)).to.be.true

        await run('zama:erc20:renounce:minter_role')
        expect(await zamaErc20.hasRole(minterRole, deployer)).to.be.false
    })

    it('renounces PAUSING_MINTER_ROLE for the connected signer via task', async () => {
        expect(await zamaErc20.hasRole(pausingRole, deployer)).to.be.false

        await run('zama:erc20:grant:pausing_minter_role', { address: deployer })
        expect(await zamaErc20.hasRole(pausingRole, deployer)).to.be.true

        await run('zama:erc20:renounce:pausing_minter_role')
        expect(await zamaErc20.hasRole(pausingRole, deployer)).to.be.false
    })

    it('renounces DEFAULT_ADMIN_ROLE for the connected signer via task', async () => {
        expect(await zamaErc20.hasRole(defaultAdminRole, deployer)).to.be.true

        await run('zama:erc20:renounce:default_admin_role')
        expect(await zamaErc20.hasRole(defaultAdminRole, deployer)).to.be.false
    })
})
