import { SignerWithAddress } from '@nomiclabs/hardhat-ethers/signers'
import { expect } from 'chai'
import { Contract, ContractFactory } from 'ethers'
import { deployments, ethers } from 'hardhat'

import { Options } from '@layerzerolabs/lz-v2-utilities'

describe('Zama Token OFT Transfer', () => {
    // Constant representing a mock Endpoint ID for testing purposes
    const eidA = 1
    const eidB = 2
    // Declaration of variables to be used in the test suite
    let zamaOFTAdapterFactory: ContractFactory
    let zamaOFTFactory: ContractFactory
    let zamaERC20Factory: ContractFactory
    let endpointV2MockFactory: ContractFactory
    let deployer: SignerWithAddress
    let owner: SignerWithAddress
    let admin: SignerWithAddress
    let alice: SignerWithAddress
    let bob: SignerWithAddress
    let endpointOwner: SignerWithAddress
    let zamaERC20: Contract
    let zamaOFTAdapter: Contract
    let zamaOFT: Contract
    let mockEndpointV2A: Contract
    let mockEndpointV2B: Contract

    const MINTER_ROLE = ethers.utils.keccak256(ethers.utils.toUtf8Bytes('MINTER_ROLE'))
    const MINTING_PAUSER_ROLE = ethers.utils.keccak256(ethers.utils.toUtf8Bytes('MINTING_PAUSER_ROLE'))

    // Before hook for setup that runs once before all tests in the block
    before(async () => {
        zamaOFTAdapterFactory = await ethers.getContractFactory('ZamaOFTAdapter')

        zamaOFTFactory = await ethers.getContractFactory('ZamaOFT')

        zamaERC20Factory = await ethers.getContractFactory('ZamaERC20')

        // Fetching the first three signers (accounts) from Hardhat's local Ethereum network
        const signers = await ethers.getSigners()

        ;[deployer, owner, admin, alice, bob, endpointOwner] = signers

        // The EndpointV2Mock contract comes from @layerzerolabs/test-devtools-evm-hardhat package
        // and its artifacts are connected as external artifacts to this project
        //
        // Unfortunately, hardhat itself does not yet provide a way of connecting external artifacts,
        // so we rely on hardhat-deploy to create a ContractFactory for EndpointV2Mock
        //
        // See https://github.com/NomicFoundation/hardhat/issues/1040
        const endpointV2MockFactoryArtifact = await deployments.getArtifact('EndpointV2Mock')
        endpointV2MockFactory = new ContractFactory(
            endpointV2MockFactoryArtifact.abi,
            endpointV2MockFactoryArtifact.bytecode,
            endpointOwner
        )
    })

    // beforeEach hook for setup that runs before each test in the block
    beforeEach(async () => {
        // Deploying a mock LZEndpoint with the given Endpoint ID
        mockEndpointV2A = await endpointV2MockFactory.deploy(eidA)
        mockEndpointV2B = await endpointV2MockFactory.deploy(eidB)

        // The INITIAL_RECEIVER_0 and INITIAL_ADMIN can be different from the deployer.
        const _INITIAL_MINT_AMOUNT = 11_000_000_000n
        const INITIAL_MINT_AMOUNT = ethers.utils.parseEther(_INITIAL_MINT_AMOUNT.toString())
        zamaERC20 = await zamaERC20Factory
            .connect(deployer)
            .deploy('ZAMAERC20', 'ZAMA', [owner.address], [INITIAL_MINT_AMOUNT], admin.address)
        // Grant the admin the MINTER_ROLE & PAUSING_MINTER_ROLE
        zamaERC20.connect(admin).grantRole(MINTER_ROLE, admin.address)
        zamaERC20.connect(admin).grantRole(MINTING_PAUSER_ROLE, admin.address)

        // Deploying the ZamaOFTAdapter to be linked with ZamaERC20 and ZamaOFT on two different mockEndpoint to simulate cross-chain communication.
        // Based on the deployment scripts, the owner is the deployer.
        zamaOFTAdapter = await zamaOFTAdapterFactory
            .connect(deployer)
            .deploy(zamaERC20.address, mockEndpointV2A.address, deployer.address)
        zamaOFT = await zamaOFTFactory
            .connect(deployer)
            .deploy('ZAMAOFT', 'ZAMA', mockEndpointV2B.address, deployer.address)

        // Setting destination endpoints in the LZEndpoint mock for each MyOFT instance
        await mockEndpointV2A.setDestLzEndpoint(zamaOFT.address, mockEndpointV2B.address)
        await mockEndpointV2B.setDestLzEndpoint(zamaOFTAdapter.address, mockEndpointV2A.address)

        // Setting each OFT instance as a peer of the other in the mock LZEndpoint
        await zamaOFTAdapter.connect(deployer).setPeer(eidB, ethers.utils.zeroPad(zamaOFT.address, 32))
        await zamaOFT.connect(deployer).setPeer(eidA, ethers.utils.zeroPad(zamaOFTAdapter.address, 32))
    })

    describe('Chain A > Chain B', () => {
        // A test case to verify token transfer functionality
        it('should send token from A address (alice) to B address (bob) via OFTAdapter', async () => {
            // Minting an initial amount of $ZAMA tokens to alice's address
            const initialAmount = ethers.utils.parseEther('100')
            await zamaERC20.connect(admin).mint(alice.address, initialAmount)

            // Defining the amount of tokens to send and constructing the parameters for the send operation
            const tokensToSend = ethers.utils.parseEther('1')

            // Defining extra message execution options for the send operation
            const options = Options.newOptions().addExecutorLzReceiveOption(200000, 0).toHex().toString()

            const sendParam = [
                eidB,
                ethers.utils.zeroPad(bob.address, 32),
                tokensToSend,
                tokensToSend,
                options,
                '0x',
                '0x',
            ]

            // Fetching the native fee for the token send operation
            const [nativeFee] = await zamaOFTAdapter.quoteSend(sendParam, false)

            // Approving the native fee to be spent by the ZamaOFTAdapter contract
            await zamaERC20.connect(alice).approve(zamaOFTAdapter.address, tokensToSend)

            const totalSupplyBefore = await zamaERC20.totalSupply()

            // Executing the send operation from ZamaOFTAdapter contract
            await zamaOFTAdapter.connect(alice).send(sendParam, [nativeFee, 0], alice.address, { value: nativeFee })

            // Fetching the final token balances of ownerA and ownerB
            const finalBalanceA = await zamaERC20.balanceOf(alice.address)
            const finalBalanceAdapter = await zamaERC20.balanceOf(zamaOFTAdapter.address)
            const finalBalanceB = await zamaOFT.balanceOf(bob.address)

            // Asserting that the final balances are as expected after the send operation
            expect(finalBalanceA).eql(initialAmount.sub(tokensToSend))
            expect(finalBalanceAdapter).eql(tokensToSend)
            expect(finalBalanceB).eql(tokensToSend)

            // Assert that the total supply has not changed.
            const totalSupplyAfter = await zamaERC20.totalSupply()
            expect(totalSupplyBefore).to.eq(totalSupplyAfter)
        })
    })

    describe('Chain B > Chain A', () => {
        const initialAmount = ethers.utils.parseEther('100')
        const tokensToSend = ethers.utils.parseEther('1')
        // Transfer token from host chain A to chain B, prior to sending back tokens
        beforeEach(async () => {
            // Minting an initial amount of $ZAMA tokens to alice's address
            await zamaERC20.connect(admin).mint(alice.address, initialAmount)

            // Defining extra message execution options for the send operation
            const options = Options.newOptions().addExecutorLzReceiveOption(200000, 0).toHex().toString()

            const sendParam = [
                eidB,
                ethers.utils.zeroPad(bob.address, 32),
                tokensToSend,
                tokensToSend,
                options,
                '0x',
                '0x',
            ]

            // Fetching the native fee for the token send operation
            const [nativeFee] = await zamaOFTAdapter.quoteSend(sendParam, false)

            // Approving the native fee to be spent by the ZamaOFTAdapter contract
            await zamaERC20.connect(alice).approve(zamaOFTAdapter.address, tokensToSend)

            const totalSupplyBefore = await zamaERC20.totalSupply()

            // Executing the send operation from ZamaOFTAdapter contract
            await zamaOFTAdapter.connect(alice).send(sendParam, [nativeFee, 0], alice.address, { value: nativeFee })

            // Fetching the final token balances of ownerA and ownerB
            const finalBalanceA = await zamaERC20.balanceOf(alice.address)
            const finalBalanceAdapter = await zamaERC20.balanceOf(zamaOFTAdapter.address)
            const finalBalanceB = await zamaOFT.balanceOf(bob.address)

            // Asserting that the final balances are as expected after the send operation
            expect(finalBalanceA).eql(initialAmount.sub(tokensToSend))
            expect(finalBalanceAdapter).eql(tokensToSend)
            expect(finalBalanceB).eql(tokensToSend)

            // Assert that the total supply has not changed.
            const totalSupplyAfter = await zamaERC20.totalSupply()
            expect(totalSupplyBefore).to.eq(totalSupplyAfter)
        })

        it('should send token from B address (bob) to A address (alice) via OFT', async () => {
            // Defining extra message execution options for the send operation
            const options = Options.newOptions().addExecutorLzReceiveOption(200000, 0).toHex().toString()

            const sendParam = [
                eidA,
                ethers.utils.zeroPad(alice.address, 32),
                tokensToSend,
                tokensToSend,
                options,
                '0x',
                '0x',
            ]

            // Fetching the native fee for the token send operation
            const [nativeFee] = await zamaOFT.quoteSend(sendParam, false)

            // Executing the send operation from ZamaOFTAdapter contract
            await zamaOFT.connect(bob).send(sendParam, [nativeFee, 0], bob.address, { value: nativeFee })

            // Fetching the final token balances of alice and bob
            const finalBalanceB = await zamaOFT.balanceOf(bob.address)
            const finalBalanceOFTAdapter = await zamaERC20.balanceOf(zamaOFTAdapter.address)
            const finalBalanceA = await zamaERC20.balanceOf(alice.address)

            // Asserting that the final balances are as expected after the send operation
            expect(finalBalanceB).eql(tokensToSend.sub(tokensToSend))
            expect(finalBalanceOFTAdapter).to.eq(ethers.BigNumber.from(0))
            expect(finalBalanceA).eql(initialAmount)
        })
    })
})
