import { SignerWithAddress } from '@nomiclabs/hardhat-ethers/signers'
import { expect } from 'chai'
import { Contract, ContractFactory } from 'ethers'
import { deployments, ethers } from 'hardhat'
import { execTransaction } from './utils/execTransaction.ts'

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

    it('owner can wihdraw ETH from prefunded governanceOAppSender', async function () {
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
})
