import { SignerWithAddress } from '@nomiclabs/hardhat-ethers/signers'
import { expect } from 'chai'
import { Contract, ContractFactory } from 'ethers'
import { deployments, ethers } from 'hardhat'

import { Options } from '@layerzerolabs/lz-v2-utilities'

/// TODO: update tests
describe('MyOApp Test', function () {
    // Constant representing a mock Endpoint ID for testing purposes
    const eidA = 1
    const eidB = 2
    // Declaration of variables to be used in the test suite
    let MyOApp: ContractFactory
    let EndpointV2Mock: ContractFactory
    let ownerA: SignerWithAddress
    let ownerB: SignerWithAddress
    let endpointOwner: SignerWithAddress
    let myOAppA: Contract
    let myOAppB: Contract
    let mockEndpointV2A: Contract
    let mockEndpointV2B: Contract

    // Before hook for setup that runs once before all tests in the block
    before(async function () {
        // Contract factory for our tested contract
        MyOApp = await ethers.getContractFactory('MyOApp')

        // Fetching the first three signers (accounts) from Hardhat's local Ethereum network
        const signers = await ethers.getSigners()

        ;[ownerA, ownerB, endpointOwner] = signers

        // The EndpointV2Mock contract comes from @layerzerolabs/test-devtools-evm-hardhat package
        // and its artifacts are connected as external artifacts to this project
        //
        // Unfortunately, hardhat itself does not yet provide a way of connecting external artifacts,
        // so we rely on hardhat-deploy to create a ContractFactory for EndpointV2Mock
        //
        // See https://github.com/NomicFoundation/hardhat/issues/1040
        const EndpointV2MockArtifact = await deployments.getArtifact('EndpointV2Mock')
        EndpointV2Mock = new ContractFactory(EndpointV2MockArtifact.abi, EndpointV2MockArtifact.bytecode, endpointOwner)
    })

    // beforeEach hook for setup that runs before each test in the block
    beforeEach(async function () {
        // Deploying a mock LZ EndpointV2 with the given Endpoint ID
        mockEndpointV2A = await EndpointV2Mock.deploy(eidA)
        mockEndpointV2B = await EndpointV2Mock.deploy(eidB)

        // Deploying two instances of MyOApp contract and linking them to the mock LZEndpoint
        myOAppA = await MyOApp.deploy(mockEndpointV2A.address, ownerA.address)
        myOAppB = await MyOApp.deploy(mockEndpointV2B.address, ownerB.address)

        // Setting destination endpoints in the LZEndpoint mock for each MyOApp instance
        await mockEndpointV2A.setDestLzEndpoint(myOAppB.address, mockEndpointV2B.address)
        await mockEndpointV2B.setDestLzEndpoint(myOAppA.address, mockEndpointV2A.address)

        // Setting each MyOApp instance as a peer of the other
        await myOAppA.connect(ownerA).setPeer(eidB, ethers.utils.zeroPad(myOAppB.address, 32))
        await myOAppB.connect(ownerB).setPeer(eidA, ethers.utils.zeroPad(myOAppA.address, 32))
    })

    // A test case to verify message sending functionality
    it('should send a string message to each destination OApp', async function () {
        // Assert initial state of lastMessage in both MyOApp instances
        expect(await myOAppA.lastMessage()).to.equal('')
        expect(await myOAppB.lastMessage()).to.equal('')

        const options = Options.newOptions().addExecutorLzReceiveOption(200000, 0).toHex().toString()
        const message = 'Test message.'

        // Define native fee and quote for the message send operation
        let nativeFee = 0
        const messagingFee = await myOAppA.quoteSendString(eidB, message, options, false)
        nativeFee = messagingFee.nativeFee

        // Execute sendString operation from myOAppA
        await myOAppA.sendString(eidB, message, options, { value: nativeFee.toString() })

        // Assert the resulting state of lastMessage in both MyOApp instances
        expect(await myOAppA.lastMessage()).to.equal('')
        expect(await myOAppB.lastMessage()).to.equal('Test message.')
    })
})
