import { SignerWithAddress } from '@nomiclabs/hardhat-ethers/signers'
import { expect } from 'chai'
import { Contract, ContractFactory } from 'ethers'
import { ethers, run } from 'hardhat'

describe('ZamaOFTAdapter task suite', function () {
    let endpointFactory: ContractFactory
    let zamaERC20Factory: ContractFactory
    let zamaOFTAdapterFactory: ContractFactory
    let endpointStub: Contract
    let zamaERC20: Contract
    let zamaOFTAdapter: Contract
    let deployer: SignerWithAddress
    let other: SignerWithAddress

    before(async () => {
        ;[deployer, other] = await ethers.getSigners()

        endpointFactory = await ethers.getContractFactory('EndpointStub')
        zamaERC20Factory = await ethers.getContractFactory('ZamaERC20')
        zamaOFTAdapterFactory = await ethers.getContractFactory('ZamaOFTAdapter')
    })

    beforeEach(async function () {
        endpointStub = await endpointFactory.deploy()
        zamaERC20 = await zamaERC20Factory
            .connect(deployer)
            .deploy('ZAMAERC20', 'ZAMA', [deployer.address], [11_000_000_000n], deployer.address)
        zamaOFTAdapter = await zamaOFTAdapterFactory
            .connect(deployer)
            .deploy(zamaERC20.address, endpointStub.address, deployer.address)
    })

    describe('setDelegate', async () => {
        it('updates the delegate via task', async function () {
            expect(await endpointStub.delegates(zamaOFTAdapter.address)).to.equal(deployer.address)

            await run('zama:oftadapter:setDelegate', {
                address: other.address,
                contractAddress: zamaOFTAdapter.address,
            })

            expect(await endpointStub.delegates(zamaOFTAdapter.address)).to.equal(other.address)
        })

        it('should fail if the provided address is not a valid EVM address', async () => {
            await expect(
                run('zama:oftadapter:setDelegate', { address: '0xabCDe', contractAddress: zamaOFTAdapter.address })
            ).to.be.rejected
        })
    })

    describe('transferOwnership', async () => {
        it('transfers ownership via task', async function () {
            expect(await zamaOFTAdapter.owner()).to.equal(deployer.address)

            await run('zama:oftadapter:transferOwnership', {
                address: other.address,
                contractAddress: zamaOFTAdapter.address,
            })

            expect(await zamaOFTAdapter.owner()).to.equal(other.address)
        })

        it('should fail if the provided address is not a valid EVM address', async () => {
            await expect(
                run('zama:oftadapter:transferOwnership', {
                    address: '0xabCDe',
                    contractAddress: zamaOFTAdapter.address,
                })
            ).to.be.rejected
        })
    })
})
