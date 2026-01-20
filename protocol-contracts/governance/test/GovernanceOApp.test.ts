import { SignerWithAddress } from '@nomiclabs/hardhat-ethers/signers'
import { expect } from 'chai'
import { Contract, ContractFactory } from 'ethers'
import { deployments, ethers } from 'hardhat'
import { execTransaction } from './utils/execTransaction'

import { Options } from '@layerzerolabs/lz-v2-utilities'
import { EndpointId } from '@layerzerolabs/lz-definitions'

describe('GovernanceOApp Test', function () {
    const eidA = EndpointId.SEPOLIA_V2_TESTNET
    const eidB = EndpointId.ZAMA_V2_TESTNET
    let GovernanceOAppSender: ContractFactory
    let GovernanceOAppReceiver: ContractFactory
    let EndpointV2Mock: ContractFactory
    let AdminModuleMock: ContractFactory
    let GatewayConfigMock: ContractFactory
    let SafeL2: ContractFactory
    let SafeProxyFactory: ContractFactory
    let owner: SignerWithAddress
    let endpointOwner: SignerWithAddress
    let governanceOAppSender: Contract
    let governanceOAppReceiver: Contract
    let mockEndpointV2A: Contract
    let mockEndpointV2B: Contract
    let gatewayConfigMock: Contract
    let safeProxy: Contract
    let adminModuleMock: Contract

    before(async function () {
        GovernanceOAppSender = await ethers.getContractFactory('GovernanceOAppSender')
        GovernanceOAppReceiver = await ethers.getContractFactory('GovernanceOAppReceiver')

        const signers = await ethers.getSigners()

        ;[owner, endpointOwner] = signers

        const EndpointV2MockArtifact = await deployments.getArtifact('EndpointV2Mock')
        EndpointV2Mock = new ContractFactory(EndpointV2MockArtifact.abi, EndpointV2MockArtifact.bytecode, endpointOwner)

        SafeL2 = await ethers.getContractFactory('SafeL2')
        SafeProxyFactory = await ethers.getContractFactory('SafeProxyFactory')
        AdminModuleMock = await ethers.getContractFactory('AdminModuleMock')
        GatewayConfigMock = await ethers.getContractFactory('GatewayConfigMock')
    })

    beforeEach(async function () {
        mockEndpointV2A = await EndpointV2Mock.deploy(eidA)
        mockEndpointV2B = await EndpointV2Mock.deploy(eidB)

        governanceOAppSender = await GovernanceOAppSender.deploy(mockEndpointV2A.address, owner.address)
        governanceOAppReceiver = await GovernanceOAppReceiver.deploy(mockEndpointV2B.address, owner.address)

        await mockEndpointV2A.setDestLzEndpoint(governanceOAppReceiver.address, mockEndpointV2B.address)
        await mockEndpointV2B.setDestLzEndpoint(governanceOAppSender.address, mockEndpointV2A.address)

        await governanceOAppSender
            .connect(owner)
            .setPeer(eidB, ethers.utils.zeroPad(governanceOAppReceiver.address, 32))
        await governanceOAppReceiver
            .connect(owner)
            .setPeer(eidA, ethers.utils.zeroPad(governanceOAppSender.address, 32))

        const safeSingleton = await SafeL2.deploy()
        const safeProxyFactory = await SafeProxyFactory.deploy()
        const safeData = safeSingleton.interface.encodeFunctionData('setup', [
            [owner.address],
            1n,
            ethers.constants.AddressZero,
            '0x',
            ethers.constants.AddressZero,
            ethers.constants.AddressZero,
            0,
            ethers.constants.AddressZero,
        ])
        const safeProxyAddress = await safeProxyFactory.callStatic.createProxyWithNonce(
            safeSingleton.address,
            safeData,
            0n
        )
        await safeProxyFactory.createProxyWithNonce(safeSingleton.address, safeData, 0n)
        safeProxy = await ethers.getContractAt('SafeL2', safeProxyAddress)
        gatewayConfigMock = await GatewayConfigMock.deploy(safeProxy.address)
        adminModuleMock = await AdminModuleMock.deploy(governanceOAppReceiver.address, safeProxy.address)

        await governanceOAppReceiver.setAdminSafeModule(adminModuleMock.address)

        const enableModuleData = safeProxy.interface.encodeFunctionData('enableModule', [adminModuleMock.address])
        await execTransaction([owner], safeProxy, safeProxy.address, 0n, enableModuleData, 0)
    })

    it('should send a remote proposal with function signature on source chain and execute it on destination chain', async function () {
        expect(BigInt(await gatewayConfigMock.value())).to.equal(0n)
        const options = Options.newOptions().addExecutorLzReceiveOption(80000, 0).toHex().toString()

        const quotedFee = await governanceOAppSender.quoteSendCrossChainTransaction(
            [gatewayConfigMock.address],
            [0n],
            ['setValue(uint256)'],
            [ethers.utils.defaultAbiCoder.encode(['uint256'], [42n])],
            [0n],
            options
        )

        await governanceOAppSender.sendRemoteProposal(
            [gatewayConfigMock.address],
            [0n],
            ['setValue(uint256)'],
            [ethers.utils.defaultAbiCoder.encode(['uint256'], [42n])],
            [0n],
            options,
            { value: quotedFee }
        )

        expect(BigInt(await gatewayConfigMock.value())).to.equal(42n)
    })

    it('should send a remote proposal without function signature on source chain and execute it on destination chain', async function () {
        expect(BigInt(await gatewayConfigMock.value())).to.equal(0n)
        const options = Options.newOptions().addExecutorLzReceiveOption(80000, 0).toHex().toString()

        const calldata = gatewayConfigMock.interface.encodeFunctionData('setValue', [19n])

        const quotedFee = await governanceOAppSender.quoteSendCrossChainTransaction(
            [gatewayConfigMock.address],
            [0n],
            [''],
            [calldata],
            [0n],
            options
        )

        await governanceOAppSender.sendRemoteProposal(
            [gatewayConfigMock.address],
            [0n],
            [''],
            [calldata],
            [0n],
            options,
            { value: quotedFee }
        )

        expect(BigInt(await gatewayConfigMock.value())).to.equal(19n)
    })

    it('should send an expensive remote proposal', async function () {
        expect(BigInt(await gatewayConfigMock.value())).to.equal(0n)
        const options = Options.newOptions().addExecutorLzReceiveOption(1400000, 0).toHex().toString() // use a high gas value for the expensive update

        const quotedFee = await governanceOAppSender.quoteSendCrossChainTransaction(
            [gatewayConfigMock.address],
            [0n],
            ['expensiveUpdate(uint256)'],
            [ethers.utils.defaultAbiCoder.encode(['uint256'], [999n])],
            [0n],
            options
        )

        await governanceOAppSender.sendRemoteProposal(
            [gatewayConfigMock.address],
            [0n],
            ['expensiveUpdate(uint256)'],
            [ethers.utils.defaultAbiCoder.encode(['uint256'], [999n])],
            [0n],
            options,
            { value: quotedFee }
        )

        expect(BigInt(await gatewayConfigMock.value())).to.equal(999n)
    })

    it('should send an expensive remote proposal - contract is prefunded', async function () {
        expect(BigInt(await gatewayConfigMock.value())).to.equal(0n)
        const options = Options.newOptions().addExecutorLzReceiveOption(1400000, 0).toHex().toString() // use a high gas value for the expensive update

        const quotedFee = await governanceOAppSender.quoteSendCrossChainTransaction(
            [gatewayConfigMock.address],
            [0n],
            ['expensiveUpdate(uint256)'],
            [ethers.utils.defaultAbiCoder.encode(['uint256'], [999n])],
            [0n],
            options
        )

        await owner.sendTransaction({
            to: governanceOAppSender.address,
            value: quotedFee,
        }) // send funds to the governanceOAppSender before sending proposal

        await governanceOAppSender.sendRemoteProposal(
            [gatewayConfigMock.address],
            [0n],
            ['expensiveUpdate(uint256)'],
            [ethers.utils.defaultAbiCoder.encode(['uint256'], [999n])],
            [0n],
            options,
            { value: 0n } // the governanceOAppSender contract has been already funded
        )

        expect(BigInt(await gatewayConfigMock.value())).to.equal(999n)
    })

    it('should send a batch of several remote proposals', async function () {
        const gatewayConfigMockBis = await GatewayConfigMock.deploy(safeProxy.address) // deploy a second instance of GatewayConfig to test batching

        expect(BigInt(await gatewayConfigMock.value())).to.equal(0n)
        expect(BigInt(await gatewayConfigMockBis.value())).to.equal(0n)

        const options = Options.newOptions().addExecutorLzReceiveOption(120000, 0).toHex().toString()

        const quotedFee = await governanceOAppSender.quoteSendCrossChainTransaction(
            [gatewayConfigMock.address, gatewayConfigMockBis.address],
            [0n, 0n],
            ['setValue(uint256)', 'setValue(uint256)'],
            [
                ethers.utils.defaultAbiCoder.encode(['uint256'], [1n]),
                ethers.utils.defaultAbiCoder.encode(['uint256'], [2n]),
            ],
            [0n, 0n],
            options
        )

        await governanceOAppSender.sendRemoteProposal(
            [gatewayConfigMock.address, gatewayConfigMockBis.address],
            [0n, 0n],
            ['setValue(uint256)', 'setValue(uint256)'],
            [
                ethers.utils.defaultAbiCoder.encode(['uint256'], [1n]),
                ethers.utils.defaultAbiCoder.encode(['uint256'], [2n]),
            ],
            [0n, 0n],
            options,
            { value: quotedFee }
        )

        expect(BigInt(await gatewayConfigMock.value())).to.equal(1n)
        expect(BigInt(await gatewayConfigMockBis.value())).to.equal(2n)
    })

    it('owner can wihdraw ETH from prefunded GovernanceOAppSender', async function () {
        await owner.sendTransaction({
            to: governanceOAppSender.address,
            value: ethers.utils.parseEther('1'),
        })

        const balanceOwnerBefore = await ethers.provider.getBalance(owner.address)
        const balanceGovSenderBefore = await ethers.provider.getBalance(governanceOAppSender.address)
        expect(balanceGovSenderBefore.toBigInt()).to.equal(ethers.utils.parseEther('1').toBigInt())

        await governanceOAppSender.withdrawETH(ethers.utils.parseEther('1'), owner.address)

        const balanceOwnerAfter = await ethers.provider.getBalance(owner.address)
        const balanceGovSenderAfter = await ethers.provider.getBalance(governanceOAppSender.address)
        expect(balanceGovSenderAfter.toBigInt()).to.equal(0n)
        const received = balanceOwnerAfter.sub(balanceOwnerBefore).toBigInt()
        const expected = ethers.utils.parseEther('1').toBigInt()
        const tolerance = ethers.utils.parseEther('0.0001').toBigInt() // account gas used for the tx
        const diff = received > expected ? received - expected : expected - received
        expect(diff <= tolerance).to.equal(true)
    })

    it('should not send recovered funds from GovernanceOAppSender to null address', async function () {
        await owner.sendTransaction({
            to: governanceOAppSender.address,
            value: ethers.utils.parseEther('1'),
        })
        const tx = governanceOAppSender
            .connect(owner)
            .withdrawETH(ethers.utils.parseEther('1'), ethers.constants.AddressZero)
        try {
            await tx
            expect.fail('withdrawETH should have reverted with InvalidNullRecipient')
        } catch (err: any) {
            const data = err.data
            const selector = data.slice(0, 10)
            const expected = governanceOAppSender.interface.getSighash('InvalidNullRecipient()')
            expect(selector).to.equal(expected)
        }
    })

    it('owner can wihdraw ETH from prefunded GovernanceOAppReceiver', async function () {
        await ethers.provider.send('hardhat_setBalance', [
            // on a real network, funds could be sent to GovernanceOAppReceiver contract via the payable lzReceive method
            governanceOAppReceiver.address,
            '0xde0b6b3a7640000', // 1 ETH in hex
        ])

        const balanceOwnerBefore = await ethers.provider.getBalance(owner.address)
        const balanceGovReceiverBefore = await ethers.provider.getBalance(governanceOAppReceiver.address)
        expect(balanceGovReceiverBefore.toBigInt()).to.equal(ethers.utils.parseEther('1').toBigInt())

        await governanceOAppReceiver.withdrawETH(ethers.utils.parseEther('1'), owner.address)

        const balanceOwnerAfter = await ethers.provider.getBalance(owner.address)
        const balanceGovReceiverAfter = await ethers.provider.getBalance(governanceOAppReceiver.address)
        expect(balanceGovReceiverAfter.toBigInt()).to.equal(0n)
        const received = balanceOwnerAfter.sub(balanceOwnerBefore).toBigInt()
        const expected = ethers.utils.parseEther('1').toBigInt()
        const tolerance = ethers.utils.parseEther('0.0001').toBigInt() // account gas used for the tx
        const diff = received > expected ? received - expected : expected - received
        expect(diff <= tolerance).to.equal(true)
    })

    it('should not send recovered funds from GovernanceOAppReceiver to null address', async function () {
        await ethers.provider.send('hardhat_setBalance', [
            // on a real network, funds could be sent to GovernanceOAppReceiver contract via the payable lzReceive method
            governanceOAppReceiver.address,
            '0xde0b6b3a7640000', // 1 ETH in hex
        ])
        const tx = governanceOAppReceiver
            .connect(owner)
            .withdrawETH(ethers.utils.parseEther('1'), ethers.constants.AddressZero)
        try {
            await tx
            expect.fail('withdrawETH should have reverted with InvalidNullRecipient')
        } catch (err: any) {
            const data = err.data
            const selector = data.slice(0, 10)
            const expected = governanceOAppReceiver.interface.getSighash('InvalidNullRecipient()')
            expect(selector).to.equal(expected)
        }
    })

    it('should send a payable remote proposal', async function () {
        expect(BigInt(await gatewayConfigMock.value())).to.equal(0n)
        await owner.sendTransaction({
            to: safeProxy.address,
            value: ethers.utils.parseEther('1'),
        }) // send funds to the safeProxy before sending proposal
        const balanceSafeProxyBefore = await ethers.provider.getBalance(safeProxy.address)
        expect(balanceSafeProxyBefore.toBigInt()).to.equal(ethers.utils.parseEther('1').toBigInt())

        const options = Options.newOptions().addExecutorLzReceiveOption(80000, 0).toHex().toString() // use a high gas value for the expensive update

        const quotedFee = await governanceOAppSender.quoteSendCrossChainTransaction(
            [gatewayConfigMock.address],
            [ethers.utils.parseEther('1')],
            ['payableUpdate(uint256)'],
            [ethers.utils.defaultAbiCoder.encode(['uint256'], [777n])],
            [0n],
            options
        )
        await governanceOAppSender.sendRemoteProposal(
            [gatewayConfigMock.address],
            [ethers.utils.parseEther('1')],
            ['payableUpdate(uint256)'],
            [ethers.utils.defaultAbiCoder.encode(['uint256'], [777n])],
            [0n],
            options,
            { value: quotedFee }
        )
        const balanceSafeProxyAfter = await ethers.provider.getBalance(safeProxy.address)
        expect(balanceSafeProxyAfter.toBigInt()).to.equal(0n)

        expect(BigInt(await gatewayConfigMock.value())).to.equal(777n)
    })

    it('should change Safe owners and threshold via remote proposal', async function () {
        const balance = ethers.utils.hexStripZeros(ethers.utils.parseEther('100').toHexString())
        await ethers.provider.send('hardhat_setBalance', [governanceOAppSender.address, balance]) // prefund the governanceOAppSender contract

        const signers = await ethers.getSigners()
        const newOwner = signers[2] // Use third signer as the new owner to add

        // Verify initial state: Safe has 1 owner (owner) with threshold 1
        const initialOwners = await safeProxy.getOwners()
        const initialThreshold = await safeProxy.getThreshold()
        expect(initialOwners.length).to.equal(1)
        expect(initialOwners[0]).to.equal(owner.address)
        expect(BigInt(initialThreshold)).to.equal(1n)

        const options = Options.newOptions().addExecutorLzReceiveOption(150000, 0).toHex().toString()

        await governanceOAppSender.sendRemoteProposal(
            [safeProxy.address],
            [0n],
            ['addOwnerWithThreshold(address,uint256)'],
            [ethers.utils.defaultAbiCoder.encode(['address', 'uint256'], [newOwner.address, 2n])],
            [0n],
            options
        )

        // Verify the new owner was added and threshold was updated
        const ownersAfterAdd = await safeProxy.getOwners()
        const thresholdAfterAdd = await safeProxy.getThreshold()
        expect(ownersAfterAdd.length).to.equal(2)
        expect(ownersAfterAdd).to.include(owner.address)
        expect(ownersAfterAdd).to.include(newOwner.address)
        expect(BigInt(thresholdAfterAdd)).to.equal(2n)

        // Step 2: Change the threshold back to 1
        await governanceOAppSender.sendRemoteProposal(
            [safeProxy.address],
            [0n],
            ['changeThreshold(uint256)'],
            [ethers.utils.defaultAbiCoder.encode(['uint256'], [1n])],
            [0n],
            options
        )

        // Verify the threshold was changed
        const thresholdAfterChange = await safeProxy.getThreshold()
        expect(BigInt(thresholdAfterChange)).to.equal(1n)

        // Step 3: Swap an owner (replace owner with a new address)
        const anotherNewOwner = signers[3]
        // swapOwner(address prevOwner, address oldOwner, address newOwner)
        // In the Safe linked list, for owner at index 0, prevOwner is the SENTINEL_OWNERS (0x1)
        const SENTINEL_OWNERS = '0x0000000000000000000000000000000000000001'

        await governanceOAppSender.sendRemoteProposal(
            [safeProxy.address],
            [0n],
            ['swapOwner(address,address,address)'],
            [
                ethers.utils.defaultAbiCoder.encode(
                    ['address', 'address', 'address'],
                    [SENTINEL_OWNERS, newOwner.address, anotherNewOwner.address]
                ),
            ],
            [0n], // Operation.Call
            options
        )

        // Verify the owner was swapped
        const ownersAfterSwap = await safeProxy.getOwners()
        expect(ownersAfterSwap.length).to.equal(2)
        expect(ownersAfterSwap).to.include(owner.address)
        expect(ownersAfterSwap).to.include(anotherNewOwner.address)
        expect(ownersAfterSwap).to.not.include(newOwner.address)

        // Step 4: Remove an owner (remove anotherNewOwner, keeping only original owner)
        // removeOwner(address prevOwner, address owner, uint256 _threshold)
        // Since anotherNewOwner is at the head (after SENTINEL), prevOwner is SENTINEL_OWNERS
        await governanceOAppSender.sendRemoteProposal(
            [safeProxy.address],
            [0n],
            ['removeOwner(address,address,uint256)'],
            [
                ethers.utils.defaultAbiCoder.encode(
                    ['address', 'address', 'uint256'],
                    [SENTINEL_OWNERS, anotherNewOwner.address, 1n]
                ),
            ],
            [0n], // Operation.Call
            options
        )

        // Verify the owner was removed
        const finalOwners = await safeProxy.getOwners()
        const finalThreshold = await safeProxy.getThreshold()
        expect(finalOwners.length).to.equal(1)
        expect(finalOwners[0]).to.equal(owner.address)
        expect(BigInt(finalThreshold)).to.equal(1n)
    })

    it('should batch multiple Safe owner changes in a single remote proposal', async function () {
        const balance = ethers.utils.hexStripZeros(ethers.utils.parseEther('100').toHexString())
        await ethers.provider.send('hardhat_setBalance', [governanceOAppSender.address, balance]) // prefund the governanceOAppSender contract

        const signers = await ethers.getSigners()
        const newOwner1 = signers[2]
        const newOwner2 = signers[3]

        // Verify initial state
        const initialOwners = await safeProxy.getOwners()
        expect(initialOwners.length).to.equal(1)
        expect(initialOwners[0]).to.equal(owner.address)

        const options = Options.newOptions().addExecutorLzReceiveOption(250000, 0).toHex().toString()

        // Batch: Add two owners in a single proposal
        // First add newOwner1 with threshold 1, then add newOwner2 with threshold 2
        await governanceOAppSender.sendRemoteProposal(
            [safeProxy.address, safeProxy.address],
            [0n, 0n],
            ['addOwnerWithThreshold(address,uint256)', 'addOwnerWithThreshold(address,uint256)'],
            [
                ethers.utils.defaultAbiCoder.encode(['address', 'uint256'], [newOwner1.address, 1n]),
                ethers.utils.defaultAbiCoder.encode(['address', 'uint256'], [newOwner2.address, 2n]),
            ],
            [0n, 0n], // Both are Operation.Call
            options
        )

        // Verify both owners were added and threshold is 2
        const finalOwners = await safeProxy.getOwners()
        const finalThreshold = await safeProxy.getThreshold()
        expect(finalOwners.length).to.equal(3)
        expect(finalOwners).to.include(owner.address)
        expect(finalOwners).to.include(newOwner1.address)
        expect(finalOwners).to.include(newOwner2.address)
        expect(BigInt(finalThreshold)).to.equal(2n)
    })
})
