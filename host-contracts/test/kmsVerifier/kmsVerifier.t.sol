// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {KMSVerifier} from "@fhevm-host-contracts/contracts/KMSVerifier.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {KMSGeneration} from "@fhevm-host-contracts/contracts/KMSGeneration.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {KmsNode} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {KMS_CONTEXT_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {ACLOwnable} from "../../contracts/shared/ACLOwnable.sol";

contract KMSVerifierTest is HostContractsDeployerTestUtils {
    KMSVerifier internal kmsVerifier;
    ProtocolConfig internal protocolConfig;
    KMSGeneration internal kmsGeneration;

    address internal constant verifyingContractSource = address(10000);
    address internal constant owner = address(456);

    /// @dev Signer variables.
    uint256 internal constant privateKeySigner0 = 0x100;
    uint256 internal constant privateKeySigner1 = 0x200;
    uint256 internal constant privateKeySigner2 = 0x300;
    uint256 internal constant privateKeySigner3 = 0x400;

    address internal signer0;
    address internal signer1;
    address internal signer2;
    address internal signer3;

    address internal proxy;

    function setUp() public {
        signer0 = vm.addr(privateKeySigner0);
        signer1 = vm.addr(privateKeySigner1);
        signer2 = vm.addr(privateKeySigner2);
        signer3 = vm.addr(privateKeySigner3);

        _deployACL(owner);
        (protocolConfig, ) = _deployProtocolConfig(owner, _makeKmsNodes(3), _defaultThresholds());
        (kmsGeneration, ) = _deployKMSGeneration(owner);

        (kmsVerifier, ) = _deployKMSVerifier(owner, verifyingContractSource, uint64(block.chainid));
        proxy = address(kmsVerifier);
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    function _computeDigest(
        bytes32[] memory handlesList,
        bytes memory decryptedResult,
        bytes memory extraData
    ) internal view returns (bytes32) {
        bytes32 structHash = keccak256(
            abi.encode(
                kmsVerifier.DECRYPTION_RESULT_TYPEHASH(),
                keccak256(abi.encodePacked(handlesList)),
                keccak256(decryptedResult),
                keccak256(abi.encodePacked(extraData))
            )
        );

        return MessageHashUtils.toTypedDataHash(_computeDomainSeparator(), structHash);
    }

    function _computeDomainSeparator() internal view returns (bytes32) {
        (, string memory name, string memory version, uint256 chainId, address verifyingContract, , ) = kmsVerifier
            .eip712Domain();

        return
            keccak256(
                abi.encode(
                    keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
                    keccak256(bytes(name)),
                    keccak256(bytes(version)),
                    chainId,
                    verifyingContract
                )
            );
    }

    function _generateMockHandlesList(uint256 numberHandles) internal pure returns (bytes32[] memory handlesList) {
        assert(numberHandles < 250);
        handlesList = new bytes32[](numberHandles);
        for (uint256 i = 0; i < numberHandles; i++) {
            handlesList[i] = bytes32(uint256(i + 1));
        }
    }

    function _mockDecryptedResult() internal pure returns (bytes memory) {
        return abi.encodePacked(keccak256("test"));
    }

    function _buildSingleSignerProof(
        uint256 signerKey,
        bytes memory extraData
    ) internal view returns (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) {
        handlesList = _generateMockHandlesList(3);
        decryptedResult = _mockDecryptedResult();
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);
        bytes memory signature = _computeSignature(signerKey, digest);
        proof = abi.encodePacked(uint8(1), signature, extraData);
    }

    function _makeSingleSignerList(address signer) internal pure returns (address[] memory signers) {
        signers = new address[](1);
        signers[0] = signer;
    }

    function _setupHistoricalAndCurrentContexts() internal returns (uint256 historicalCtx, uint256 currentCtx) {
        historicalCtx = kmsVerifier.getCurrentKmsContextId();

        address[] memory newSigners = _makeSingleSignerList(signer3);
        vm.prank(owner);
        protocolConfig.defineNewKmsContext(_makeKmsNodesFromSigners(newSigners), _defaultThresholds());
        currentCtx = protocolConfig.getCurrentKmsContextId();
    }

    function _rotateToThresholdTwoContext() internal {
        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();
        thresholds.publicDecryption = 2;

        vm.prank(owner);
        protocolConfig.defineNewKmsContext(_makeKmsNodes(3), thresholds);
        assertEq(protocolConfig.getPublicDecryptionThreshold(), 2);
    }

    function _assertCurrentContextDelegatesToProtocolConfig() internal view {
        uint256 currentCtx = protocolConfig.getCurrentKmsContextId();
        assertEq(kmsVerifier.getCurrentKmsContextId(), currentCtx);
        assertEq(kmsVerifier.getKmsSigners(), protocolConfig.getKmsSignersForContext(currentCtx));
    }

    // -----------------------------------------------------------------------
    // Tests
    // -----------------------------------------------------------------------

    function test_PostProxyUpgradeCheck() public {
        assertEq(kmsVerifier.getVersion(), "KMSVerifier v0.3.0");
        _assertCurrentContextDelegatesToProtocolConfig();
    }

    function test_DelegationGetKmsSigners() public {
        uint256 currentCtx = protocolConfig.getCurrentKmsContextId();
        assertEq(kmsVerifier.getKmsSigners(), protocolConfig.getKmsSignersForContext(currentCtx));
    }

    function test_DelegationIsSigner() public {
        assertEq(kmsVerifier.isSigner(signer0), protocolConfig.isKmsSigner(signer0));
        assertEq(kmsVerifier.isSigner(address(0xdead)), protocolConfig.isKmsSigner(address(0xdead)));
    }

    function test_DelegationGetCurrentKmsContextId() public {
        assertEq(kmsVerifier.getCurrentKmsContextId(), protocolConfig.getCurrentKmsContextId());
    }

    function test_ProtocolConfigStateChangeReflectedInVerifier() public {
        uint256 currentCtx = protocolConfig.getCurrentKmsContextId();
        assertEq(kmsVerifier.getCurrentKmsContextId(), currentCtx);

        address[] memory nextSigners = _makeSingleSignerList(signer3);
        vm.prank(owner);
        protocolConfig.defineNewKmsContext(_makeKmsNodesFromSigners(nextSigners), _defaultThresholds());

        uint256 nextCtx = protocolConfig.getCurrentKmsContextId();
        assertEq(kmsVerifier.getCurrentKmsContextId(), nextCtx);
        assertEq(kmsVerifier.getKmsSigners(), protocolConfig.getKmsSignersForContext(nextCtx));
        assertTrue(kmsVerifier.isSigner(signer3));
        assertFalse(kmsVerifier.isSigner(signer0));
    }

    function test_GetSignersForKmsContextRevertsForInvalid() public {
        (uint256 ctx1, ) = _setupHistoricalAndCurrentContexts();

        vm.prank(owner);
        protocolConfig.destroyKmsContext(ctx1);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, ctx1));
        kmsVerifier.getSignersForKmsContext(ctx1);

        uint256 nonExistent = KMS_CONTEXT_COUNTER_BASE + 999;
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, nonExistent));
        kmsVerifier.getSignersForKmsContext(nonExistent);
    }

    function test_VerifyDecryptionEIP712KMSSignaturesWork() public {
        bytes32[] memory handlesList = _generateMockHandlesList(3);

        bytes memory decryptedResult = _mockDecryptedResult();
        bytes memory extraData = abi.encodePacked(uint8(0));
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);

        bytes[] memory signatures = new bytes[](2);
        signatures[0] = _computeSignature(privateKeySigner1, digest);
        signatures[1] = _computeSignature(privateKeySigner2, digest);

        bytes memory decryptionProof = abi.encodePacked(
            uint8(signatures.length),
            signatures[0],
            signatures[1],
            extraData
        );

        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof));
    }

    function test_VerificationSucceedsWithLastPositionSigner() public {
        bytes memory extraData = abi.encodePacked(uint8(0));
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner2,
            extraData
        );
        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof));
    }

    function test_GetSignersForKmsContextReturnsCorrectHistoricalSigners() public {
        uint256 historicalCtx = kmsVerifier.getCurrentKmsContextId();

        address[] memory nextSigners = _makeSingleSignerList(signer3);
        vm.prank(owner);
        protocolConfig.defineNewKmsContext(_makeKmsNodesFromSigners(nextSigners), _defaultThresholds());

        address[] memory historicalSigners = kmsVerifier.getSignersForKmsContext(historicalCtx);
        assertEq(historicalSigners.length, 3);
        assertEq(historicalSigners[0], signer0);
        assertEq(historicalSigners[1], signer1);
        assertEq(historicalSigners[2], signer2);
    }

    function test_VerifyDecryptionEIP712KMSSignaturesFailAsExpectedIfDigestIsInvalid() public {
        bytes32[] memory handlesList = _generateMockHandlesList(3);

        bytes memory decryptedResult = _mockDecryptedResult();
        bytes memory extraData = abi.encodePacked(uint8(0));
        bytes[] memory signatures = new bytes[](2);

        bytes32 invalidDigest = bytes32("420");
        signatures[0] = _computeSignature(privateKeySigner1, invalidDigest);
        signatures[1] = _computeSignature(privateKeySigner2, invalidDigest);

        bytes memory decryptionProof = abi.encodePacked(
            uint8(signatures.length),
            signatures[0],
            signatures[1],
            extraData
        );

        vm.expectPartialRevert(KMSVerifier.KMSInvalidSigner.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    function test_VerifyDecryptionEIP712KMSSignaturesFailAsExpectedIfNoSignerAdded() public {
        // Rotate to a single-signer context (only signer0)
        vm.prank(owner);
        protocolConfig.defineNewKmsContext(_makeKmsNodes(1), _defaultThresholds());

        bytes32[] memory handlesList = _generateMockHandlesList(3);

        bytes memory decryptedResult = _mockDecryptedResult();
        bytes memory extraData = abi.encodePacked(uint8(0));
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);

        bytes[] memory signatures = new bytes[](2);
        signatures[0] = _computeSignature(privateKeySigner1, digest);
        signatures[1] = _computeSignature(privateKeySigner2, digest);

        bytes memory decryptionProof = abi.encodePacked(
            uint8(signatures.length),
            signatures[0],
            signatures[1],
            extraData
        );

        vm.expectPartialRevert(KMSVerifier.KMSInvalidSigner.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    function test_VerifyDecryptionEIP712KMSSignaturesFailAsExpectedIfNoSignatureProvided() public {
        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = _mockDecryptedResult();
        bytes memory decryptionProof = abi.encodePacked(uint8(0), bytes1(0x00));

        vm.expectPartialRevert(KMSVerifier.KMSZeroSignature.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    function test_VerifyDecryptionEIP712KMSSignaturesFailAsExpectedIfNumberOfSignaturesIsInferiorToThreshold() public {
        _rotateToThresholdTwoContext();

        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = _mockDecryptedResult();
        bytes memory extraData = abi.encodePacked(uint8(0));
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);

        bytes[] memory signatures = new bytes[](1);
        signatures[0] = _computeSignature(privateKeySigner1, digest);

        bytes memory decryptionProof = abi.encodePacked(uint8(signatures.length), signatures[0], extraData);

        vm.expectRevert(abi.encodeWithSelector(KMSVerifier.KMSSignatureThresholdNotReached.selector, 1));
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    function test_VerifyDecryptionEIP712KMSSignaturesFailAsExpectedIfSameSignerIsUsedTwice() public {
        _rotateToThresholdTwoContext();

        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = _mockDecryptedResult();
        bytes memory extraData = abi.encodePacked(uint8(0));
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);

        bytes[] memory signatures = new bytes[](2);
        signatures[0] = _computeSignature(privateKeySigner1, digest);
        signatures[1] = _computeSignature(privateKeySigner1, digest);

        bytes memory decryptionProof = abi.encodePacked(
            uint8(signatures.length),
            signatures[0],
            signatures[1],
            extraData
        );

        assertFalse(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof));
    }

    function test_VerifyDecryptionEIP712KMSSignaturesFailsIfEmptyDecryptionProof() public {
        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = _mockDecryptedResult();

        vm.expectRevert(KMSVerifier.EmptyDecryptionProof.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, new bytes(0));
    }

    function test_VerifyDecryptionEIP712KMSSignaturesFailsIfDeserializingDecryptionProofFail(
        uint256 randomValue
    ) public {
        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = _mockDecryptedResult();
        bytes memory decryptionProof = abi.encodePacked(uint8(3), randomValue);

        vm.expectRevert(KMSVerifier.DeserializingDecryptionProofFail.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    function test_VerificationSucceedsForOldContextWithOldSigners() public {
        (uint256 ctx1, ) = _setupHistoricalAndCurrentContexts();

        bytes memory extraData = abi.encodePacked(uint8(0x01), ctx1);
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            extraData
        );

        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof));
    }

    function test_VerificationUsesPerContextThresholdForHistoricalContext() public {
        // Rotate to a new context with 3 signers and threshold=2
        IProtocolConfig.KmsThresholds memory t2 = _defaultThresholds();
        t2.publicDecryption = 2;
        vm.prank(owner);
        protocolConfig.defineNewKmsContext(_makeKmsNodes(3), t2);
        uint256 historicalCtx = protocolConfig.getCurrentKmsContextId();
        assertEq(protocolConfig.getPublicDecryptionThreshold(), 2);

        // Rotate again to a single-signer context with threshold=1 (now current)
        vm.prank(owner);
        protocolConfig.defineNewKmsContext(
            _makeKmsNodesFromSigners(_makeSingleSignerList(signer3)),
            _defaultThresholds()
        );
        assertEq(protocolConfig.getPublicDecryptionThreshold(), 1);

        // Verify against historical context with only 1 signature — must fail (threshold=2)
        bytes memory extraData = abi.encodePacked(uint8(0x01), historicalCtx);
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            extraData
        );
        vm.expectRevert(abi.encodeWithSelector(KMSVerifier.KMSSignatureThresholdNotReached.selector, 1));
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof);

        // Verify against historical context with 2 valid signatures — must succeed
        handlesList = _generateMockHandlesList(3);
        decryptedResult = _mockDecryptedResult();
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);
        bytes[] memory signatures = new bytes[](2);
        signatures[0] = _computeSignature(privateKeySigner0, digest);
        signatures[1] = _computeSignature(privateKeySigner1, digest);
        bytes memory decryptionProof = abi.encodePacked(uint8(2), signatures[0], signatures[1], extraData);
        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof));
    }

    function test_VerificationFailsForDestroyedContext() public {
        (uint256 ctx1, ) = _setupHistoricalAndCurrentContexts();

        vm.prank(owner);
        protocolConfig.destroyKmsContext(ctx1);

        bytes memory extraData = abi.encodePacked(uint8(0x01), ctx1);
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            extraData
        );

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, ctx1));
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof);
    }

    function test_VerificationFailsWithUnsupportedExtraDataVersion() public {
        bytes memory extraData = abi.encodePacked(uint8(0x02));
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            extraData
        );

        vm.expectRevert(abi.encodeWithSelector(KMSVerifier.UnsupportedExtraDataVersion.selector, uint8(0x02)));
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof);
    }

    function test_VerificationFailsWithMalformedV1ExtraData() public {
        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = _mockDecryptedResult();
        bytes memory extraData = abi.encodePacked(uint8(0x01), uint8(0x42));
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);

        bytes memory sig = _computeSignature(privateKeySigner0, digest);
        bytes memory decryptionProof = abi.encodePacked(uint8(1), sig, extraData);

        vm.expectRevert(KMSVerifier.DeserializingExtraDataFail.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, decryptionProof);
    }

    function test_VerificationFailsForInvalidContextWithV1ExtraData() public {
        uint256 nonExistentCtx = KMS_CONTEXT_COUNTER_BASE + 999;
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            abi.encodePacked(uint8(0x01), nonExistentCtx)
        );
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, nonExistentCtx));
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof);

        (handlesList, decryptedResult, proof) = _buildSingleSignerProof(
            privateKeySigner0,
            abi.encodePacked(uint8(0x01), KMS_CONTEXT_COUNTER_BASE)
        );
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, KMS_CONTEXT_COUNTER_BASE));
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof);

        (handlesList, decryptedResult, proof) = _buildSingleSignerProof(
            privateKeySigner0,
            abi.encodePacked(uint8(0x01), uint256(0))
        );
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, uint256(0)));
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof);
    }

    function test_VerificationSucceedsWithEmptyExtraData() public {
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            new bytes(0)
        );

        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof));
    }

    function test_CrossContextSignerRejection() public {
        (uint256 ctx1, ) = _setupHistoricalAndCurrentContexts();

        bytes memory extraData = abi.encodePacked(uint8(0x01), ctx1);
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner3,
            extraData
        );

        vm.expectPartialRevert(KMSVerifier.KMSInvalidSigner.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof);
    }

    function test_VerificationSucceedsWithV1ExtraDataForCurrentContext() public {
        uint256 currentCtx = kmsVerifier.getCurrentKmsContextId();

        bytes memory extraData = abi.encodePacked(uint8(0x01), currentCtx, uint256(12345));
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            extraData
        );

        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof));
    }

    function test_V0ExtraDataWithTrailingBytesUsesCurrentContext() public {
        bytes memory extraData = abi.encodePacked(uint8(0x00), uint256(12345));
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            extraData
        );

        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof));
    }

    function upgrade(address account) external {
        UnsafeUpgrades.upgradeProxy(proxy, address(new EmptyUUPSProxy()), "", account);
    }

    function test_OnlyOwnerCanAuthorizeUpgrade(address randomAccount) public {
        vm.assume(randomAccount != owner);
        vm.expectPartialRevert(ACLOwnable.NotHostOwner.selector);
        this.upgrade(randomAccount);
    }

    function test_OnlyOwnerCanAuthorizeUpgrade() public {
        this.upgrade(owner);
    }

    function test_ReinitializeV3CannotBeCalledTwice() public {
        vm.prank(owner);
        vm.expectRevert(Initializable.InvalidInitialization.selector);
        kmsVerifier.reinitializeV3();
    }

    function test_GetContextSignersAndThresholdFromExtraData() public {
        uint256 ctx1 = kmsVerifier.getCurrentKmsContextId();

        (address[] memory v0Signers, uint256 v0Threshold) = kmsVerifier.getContextSignersAndThresholdFromExtraData(
            hex"00"
        );
        assertEq(v0Signers.length, 3);
        assertEq(v0Signers[0], signer0);
        assertEq(v0Threshold, 1);

        address[] memory nextSigners = _makeSingleSignerList(signer3);
        vm.prank(owner);
        protocolConfig.defineNewKmsContext(_makeKmsNodesFromSigners(nextSigners), _defaultThresholds());

        bytes memory v1ExtraData = abi.encodePacked(uint8(0x01), ctx1);
        (address[] memory oldCtxSigners, uint256 oldCtxThreshold) = kmsVerifier
            .getContextSignersAndThresholdFromExtraData(v1ExtraData);
        assertEq(oldCtxSigners.length, 3);
        assertEq(oldCtxSigners[0], signer0);
        assertEq(oldCtxThreshold, 1);

        vm.prank(owner);
        protocolConfig.destroyKmsContext(ctx1);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, ctx1));
        kmsVerifier.getContextSignersAndThresholdFromExtraData(v1ExtraData);
    }
}
