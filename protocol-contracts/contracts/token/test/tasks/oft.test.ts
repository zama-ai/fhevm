import { SignerWithAddress } from '@nomiclabs/hardhat-ethers/signers'
import { expect } from 'chai'
import { Contract, ContractFactory } from 'ethers'
import { ethers, run } from 'hardhat'

describe('ZamaOFT task suite', function () {
    let endpointFactory: ContractFactory
    let zamaOFTFactory: ContractFactory
    let endpointStub: Contract
    let zamaOFT: Contract
    let deployer: SignerWithAddress
    let other: SignerWithAddress

    before(async () => {
        ;[deployer, other] = await ethers.getSigners()

        endpointFactory = await ethers.getContractFactory('EndpointStub')
        zamaOFTFactory = await ethers.getContractFactory('ZamaOFT')
    })

    beforeEach(async () => {
        endpointStub = await endpointFactory.deploy()
        zamaOFT = await zamaOFTFactory
            .connect(deployer)
            .deploy('ZAMAOFT', 'ZAMA', endpointStub.address, deployer.address)
    })

    describe('setDelegate', async () => {
        it('updates the delegate via task', async () => {
            expect(await endpointStub.delegates(zamaOFT.address)).to.eq(deployer.address)

            await run('zama:oft:setDelegate', { address: other.address, contractAddress: zamaOFT.address })

            expect(await endpointStub.delegates(zamaOFT.address)).to.eq(other.address)
        })

        it('should fail if the provided address is not a valid EVM address', async () => {
            await expect(
                run('zama:oft:setDelegate', {
                    address: '0xabCDe',
                    contractAddress: zamaOFT.address,
                })
            ).to.be.rejected
        })
    })

    describe('transferOwnership', async () => {
        it('transfers ownership via task', async () => {
            expect(await zamaOFT.owner()).to.eq(deployer.address)

            await run('zama:oft:transferOwnership', { address: other.address, contractAddress: zamaOFT.address })

            expect(await zamaOFT.owner()).to.eq(other.address)
        })

        it('should fail if the provided address is not a valid EVM address', async () => {
            await expect(
                run('zama:oft:transferOwnership', {
                    address: '0xabCDe',
                    contractAddress: zamaOFT.address,
                })
            ).to.be.rejected
        })
    })
})
