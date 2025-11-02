import { SignerWithAddress } from '@nomiclabs/hardhat-ethers/signers'
import { expect } from 'chai'
import { Contract, ContractFactory } from 'ethers'
import { deployments, ethers } from 'hardhat'
import { execTransaction } from './utils/execTransaction.ts'

import { Options } from '@layerzerolabs/lz-v2-utilities'

describe('GovernanceOApp Test', function () {
    const eidA = 40161
    const eidB = 40424
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

    it('should send a remote proposal on source chain and execute it on destination chain', async function () {
        expect(BigInt(await gatewayConfigMock.value())).to.equal(0n)
        const options = Options.newOptions().addExecutorLzReceiveOption(200000, 0).toHex().toString()

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
})
