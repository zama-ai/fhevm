// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {KMSVerifier} from "@fhevm-host-contracts/contracts/KMSVerifier.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {KMSGeneration} from "@fhevm-host-contracts/contracts/KMSGeneration.sol";
import {IKMSGeneration} from "@fhevm-host-contracts/contracts/interfaces/IKMSGeneration.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {IProtocolConfigCommon} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigCommon.sol";
import {KmsNode, KmsNodeParams, PcrValues} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {KMS_CONTEXT_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {ACLOwnable} from "../../contracts/shared/ACLOwnable.sol";

contract KMSVerifierTest is HostContractsDeployerTestUtils {
    KMSVerifier internal kmsVerifier;
    ProtocolConfig internal protocolConfig;
    KMSGeneration internal kmsGeneration;

    address internal constant verifyingContractSource = address(10000);
    address internal constant owner = address(456);
    uint256 internal constant EPOCH_COUNTER_BASE = uint256(8) << 248;
    uint256 internal constant PREP_KEYGEN_COUNTER_BASE = uint256(3) << 248;
    uint256 internal constant KEY_COUNTER_BASE = uint256(4) << 248;
    uint256 internal constant CRS_COUNTER_BASE = uint256(5) << 248;
    bytes32 internal constant EIP712_DOMAIN_TYPE_HASH =
        keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");
    bytes32 internal constant EIP712_KEY_DIGEST_TYPE_HASH = keccak256("KeyDigest(uint8 keyType,bytes digest)");
    bytes32 internal constant EIP712_KEYGEN_TYPE_HASH =
        keccak256(
            "KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)"
        );
    bytes32 internal constant EIP712_CRSGEN_TYPE_HASH =
        keccak256("CrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest,bytes extraData)");

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
        (protocolConfig, ) = _deployProtocolConfig(owner, _makeKmsNodeParams(3), _defaultThresholds());
        (kmsGeneration, ) = _deployKMSGeneration(owner);

        (kmsVerifier, ) = _deployKMSVerifier(owner, verifyingContractSource, uint64(block.chainid));
        proxy = address(kmsVerifier);
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    function _defineNewKmsContextAndEpoch(
        KmsNodeParams[] memory nodes,
        IProtocolConfigCommon.KmsThresholds memory thresholds
    ) internal {
        PcrValues[] memory pcrValues = new PcrValues[](0);
        protocolConfig.defineNewKmsContextAndEpoch(nodes, thresholds, "", pcrValues);
    }

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

    function _computeProtocolConfigDomainSeparator() internal view returns (bytes32) {
        return
            keccak256(
                abi.encode(
                    EIP712_DOMAIN_TYPE_HASH,
                    keccak256(bytes("ProtocolConfig")),
                    keccak256(bytes("1")),
                    block.chainid,
                    address(protocolConfig)
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

    function _emptyIds() internal pure returns (uint256[] memory ids) {
        ids = new uint256[](0);
    }

    function _prepKeygenIdForKeyId(uint256 keyId) internal pure returns (uint256) {
        return PREP_KEYGEN_COUNTER_BASE + (keyId - KEY_COUNTER_BASE);
    }

    function _mockKeyDigests() internal pure returns (IKMSGeneration.KeyDigest[] memory) {
        IKMSGeneration.KeyDigest[] memory digests = new IKMSGeneration.KeyDigest[](1);
        digests[0] = IKMSGeneration.KeyDigest({keyType: IKMSGeneration.KeyType.Server, digest: hex"aabbccdd"});
        return digests;
    }

    function _hashKeyDigests(IKMSGeneration.KeyDigest[] memory keyDigests) internal pure returns (bytes32) {
        bytes32[] memory digestHashes = new bytes32[](keyDigests.length);
        for (uint256 i = 0; i < keyDigests.length; i++) {
            digestHashes[i] = keccak256(
                abi.encode(EIP712_KEY_DIGEST_TYPE_HASH, keyDigests[i].keyType, keccak256(keyDigests[i].digest))
            );
        }
        return keccak256(abi.encodePacked(digestHashes));
    }

    function _hashProtocolConfigKeygen(
        uint256 prepKeygenId,
        uint256 keyId,
        IKMSGeneration.KeyDigest[] memory keyDigests,
        bytes memory extraData
    ) internal view returns (bytes32) {
        bytes32 structHash = keccak256(
            abi.encode(EIP712_KEYGEN_TYPE_HASH, prepKeygenId, keyId, _hashKeyDigests(keyDigests), keccak256(extraData))
        );
        return MessageHashUtils.toTypedDataHash(_computeProtocolConfigDomainSeparator(), structHash);
    }

    function _hashProtocolConfigCrsgen(
        uint256 crsId,
        uint256 maxBitLength,
        bytes memory crsDigest,
        bytes memory extraData
    ) internal view returns (bytes32) {
        bytes32 structHash = keccak256(
            abi.encode(
                EIP712_CRSGEN_TYPE_HASH,
                crsId,
                maxBitLength,
                keccak256(abi.encodePacked(crsDigest)),
                keccak256(extraData)
            )
        );
        return MessageHashUtils.toTypedDataHash(_computeProtocolConfigDomainSeparator(), structHash);
    }

    function _confirmContextCreation(uint256 contextId, uint256 pk) internal {
        vm.prank(vm.addr(pk));
        protocolConfig.confirmKmsContextCreation(contextId);
    }

    function _confirmEpochActivation(
        uint256 contextId,
        uint256 epochId,
        uint256 pk,
        address txSender,
        uint256 keyId,
        uint256 crsId
    ) internal {
        bytes memory extraData = abi.encodePacked(uint8(0x02), contextId, epochId);
        IProtocolConfig.EpochKeyResult[] memory keys = new IProtocolConfig.EpochKeyResult[](keyId == 0 ? 0 : 1);
        if (keyId != 0) {
            IKMSGeneration.KeyDigest[] memory keyDigests = _mockKeyDigests();
            uint256 prepKeygenId = _prepKeygenIdForKeyId(keyId);
            keys[0] = IProtocolConfig.EpochKeyResult({
                prepKeygenId: prepKeygenId,
                keyId: keyId,
                keyDigests: keyDigests,
                signature: _computeSignature(pk, _hashProtocolConfigKeygen(prepKeygenId, keyId, keyDigests, extraData))
            });
        }

        IProtocolConfig.EpochCrsResult[] memory crsList = new IProtocolConfig.EpochCrsResult[](crsId == 0 ? 0 : 1);
        if (crsId != 0) {
            crsList[0] = IProtocolConfig.EpochCrsResult({
                crsId: crsId,
                maxBitLength: 4096,
                crsDigest: hex"deadbeef",
                signature: _computeSignature(pk, _hashProtocolConfigCrsgen(crsId, 4096, hex"deadbeef", extraData))
            });
        }

        vm.prank(txSender);
        protocolConfig.confirmEpochActivation(epochId, keys, crsList);
    }

    function _activatePendingSingleSignerContext(uint256 contextId, uint256 epochId, uint256 pk) internal {
        _confirmContextCreation(contextId, privateKeySigner0);
        _confirmContextCreation(contextId, privateKeySigner1);
        _confirmContextCreation(contextId, privateKeySigner2);
        if (pk != privateKeySigner0 && pk != privateKeySigner1 && pk != privateKeySigner2) {
            _confirmContextCreation(contextId, pk);
        }
        _confirmEpochActivation(contextId, epochId, pk, address(0xA1), 0, 0);
    }

    function _activatePendingThreeNodeContext(uint256 contextId, uint256 epochId) internal {
        _confirmContextCreation(contextId, privateKeySigner0);
        _confirmContextCreation(contextId, privateKeySigner1);
        _confirmContextCreation(contextId, privateKeySigner2);
        _confirmEpochActivation(contextId, epochId, privateKeySigner0, address(0xA1), 0, 0);
        _confirmEpochActivation(contextId, epochId, privateKeySigner1, address(0xA2), 0, 0);
        _confirmEpochActivation(contextId, epochId, privateKeySigner2, address(0xA3), 0, 0);
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
        _defineNewKmsContextAndEpoch(_makeKmsNodeParamsFromSigners(newSigners), _defaultThresholds());
        _activatePendingSingleSignerContext(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 2, privateKeySigner3);
        currentCtx = protocolConfig.getCurrentKmsContextId();
    }

    function _rotateToThresholdTwoContext() internal {
        IProtocolConfigCommon.KmsThresholds memory thresholds = _defaultThresholds();
        thresholds.publicDecryption = 2;

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(3), thresholds);
        _activatePendingThreeNodeContext(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 2);
        assertEq(protocolConfig.getPublicDecryptionThreshold(), 2);
    }

    function _assertCurrentContextDelegatesToProtocolConfig() internal view {
        uint256 currentCtx = protocolConfig.getCurrentKmsContextId();
        uint256 verifierCtx = kmsVerifier.getCurrentKmsContextId();
        assertEq(verifierCtx, currentCtx);
        assertEq(kmsVerifier.getKmsSigners(), protocolConfig.getKmsSignersForContext(currentCtx));
    }

    // -----------------------------------------------------------------------
    // Tests
    // -----------------------------------------------------------------------

    function test_PostProxyUpgradeCheck() public {
        assertEq(kmsVerifier.getVersion(), "KMSVerifier v0.4.0");
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
        uint256 verifierCtx = kmsVerifier.getCurrentKmsContextId();
        (uint256 configCtx, ) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(verifierCtx, configCtx);
    }

    function test_ProtocolConfigStateChangeReflectedInVerifier() public {
        uint256 currentCtx = protocolConfig.getCurrentKmsContextId();
        uint256 verifierCtx = kmsVerifier.getCurrentKmsContextId();
        assertEq(verifierCtx, currentCtx);

        address[] memory nextSigners = _makeSingleSignerList(signer3);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParamsFromSigners(nextSigners), _defaultThresholds());
        _activatePendingSingleSignerContext(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 2, privateKeySigner3);

        uint256 nextCtx = protocolConfig.getCurrentKmsContextId();
        uint256 verifierCtxAfter = kmsVerifier.getCurrentKmsContextId();
        assertEq(verifierCtxAfter, nextCtx);
        assertEq(kmsVerifier.getKmsSigners(), protocolConfig.getKmsSignersForContext(nextCtx));
        assertTrue(kmsVerifier.isSigner(signer3));
        assertFalse(kmsVerifier.isSigner(signer0));
    }

    function test_GetSignersForKmsContextRevertsForInvalid() public {
        (uint256 ctx1, ) = _setupHistoricalAndCurrentContexts();

        vm.prank(owner);
        protocolConfig.destroyKmsContext(ctx1);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigCommon.InvalidKmsContext.selector, ctx1));
        kmsVerifier.getSignersForKmsContext(ctx1);

        uint256 nonExistent = KMS_CONTEXT_COUNTER_BASE + 999;
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigCommon.InvalidKmsContext.selector, nonExistent));
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
        _defineNewKmsContextAndEpoch(_makeKmsNodeParamsFromSigners(nextSigners), _defaultThresholds());

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
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(1), _defaultThresholds());
        _activatePendingSingleSignerContext(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 2, privateKeySigner0);

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

    function testFuzz_VerifyDecryptionEIP712KMSSignaturesFailsIfDeserializingDecryptionProofFail(
        uint8 numSigners,
        uint16 rawProofLength
    ) public {
        numSigners = uint8(bound(uint256(numSigners), 1, type(uint8).max));
        uint256 proofLength = bound(uint256(rawProofLength), 1, 65 * uint256(numSigners));
        bytes32[] memory handlesList = _generateMockHandlesList(3);
        bytes memory decryptedResult = _mockDecryptedResult();
        bytes memory decryptionProof = new bytes(proofLength);
        decryptionProof[0] = bytes1(numSigners);

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
        IProtocolConfigCommon.KmsThresholds memory t2 = _defaultThresholds();
        t2.publicDecryption = 2;
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(3), t2);
        _activatePendingThreeNodeContext(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 2);
        uint256 historicalCtx = protocolConfig.getCurrentKmsContextId();
        assertEq(protocolConfig.getPublicDecryptionThreshold(), 2);

        // Rotate again to a single-signer context with threshold=1 (now current)
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(
            _makeKmsNodeParamsFromSigners(_makeSingleSignerList(signer3)),
            _defaultThresholds()
        );
        _activatePendingSingleSignerContext(KMS_CONTEXT_COUNTER_BASE + 3, EPOCH_COUNTER_BASE + 3, privateKeySigner3);
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

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigCommon.InvalidKmsContext.selector, ctx1));
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof);
    }

    function testFuzz_VerificationFailsWithUnsupportedExtraDataVersion(bytes calldata extraData) public {
        vm.assume(extraData.length != 0);
        vm.assume(uint8(extraData[0]) != 0x00);
        vm.assume(uint8(extraData[0]) != 0x01);
        vm.assume(uint8(extraData[0]) != 0x02);
        uint8 version = uint8(extraData[0]);
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            extraData
        );

        vm.expectRevert(abi.encodeWithSelector(KMSVerifier.UnsupportedExtraDataVersion.selector, version));
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof);
    }

    function testFuzz_VerificationFailsWithMalformedV1ExtraData(bytes calldata malformedSuffix) public {
        vm.assume(malformedSuffix.length != 32);
        bytes memory extraData = abi.encodePacked(uint8(0x01), malformedSuffix);
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            extraData
        );

        vm.expectRevert(KMSVerifier.DeserializingExtraDataFail.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof);
    }

    function test_VerificationFailsWithV1ExtraDataTrailingBytes() public {
        uint256 ctx1 = kmsVerifier.getCurrentKmsContextId();
        bytes memory extraData = abi.encodePacked(uint8(0x01), ctx1, uint256(12345));
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            extraData
        );

        vm.expectRevert(KMSVerifier.DeserializingExtraDataFail.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof);
    }

    function testFuzz_VerificationFailsWithMalformedV2ExtraData(bytes calldata malformedSuffix) public {
        vm.assume(malformedSuffix.length != 64);
        bytes memory extraData = abi.encodePacked(uint8(0x02), malformedSuffix);
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            extraData
        );

        vm.expectRevert(KMSVerifier.DeserializingExtraDataFail.selector);
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof);
    }

    function testFuzz_VerificationFailsForInvalidContextWithV1ExtraData(uint256 invalidCtx) public {
        uint256 activeCtx = kmsVerifier.getCurrentKmsContextId();
        vm.assume(invalidCtx != activeCtx);
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            abi.encodePacked(uint8(0x01), invalidCtx)
        );
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigCommon.InvalidKmsContext.selector, invalidCtx));
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

        bytes memory extraData = abi.encodePacked(uint8(0x01), currentCtx);
        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _buildSingleSignerProof(
            privateKeySigner0,
            extraData
        );

        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof));
    }

    function test_VerificationSucceedsWithV2ExtraDataForCurrentContext() public {
        (uint256 currentCtx, uint256 currentEpoch) = protocolConfig.getCurrentKmsContextAndEpoch();

        bytes memory extraData = abi.encodePacked(uint8(0x02), currentCtx, currentEpoch);
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

    function test_ReinitializeV4CannotBeCalledTwice() public {
        vm.prank(owner);
        vm.expectRevert(Initializable.InvalidInitialization.selector);
        kmsVerifier.reinitializeV4();
    }

    function test_GetContextSignersAndThresholdFromExtraData() public {
        (uint256 ctx1, uint256 epoch1) = protocolConfig.getCurrentKmsContextAndEpoch();

        (address[] memory v0Signers, uint256 v0Threshold) = kmsVerifier.getContextSignersAndThresholdFromExtraData(
            hex"00"
        );
        assertEq(v0Signers.length, 3);
        assertEq(v0Signers[0], signer0);
        assertEq(v0Threshold, 1);

        bytes memory v1ExtraData = abi.encodePacked(uint8(0x01), ctx1);
        (address[] memory oldCtxSigners, uint256 oldCtxThreshold) = kmsVerifier
            .getContextSignersAndThresholdFromExtraData(v1ExtraData);
        assertEq(oldCtxSigners.length, 3);
        assertEq(oldCtxSigners[0], signer0);
        assertEq(oldCtxThreshold, 1);

        bytes memory v2ExtraData = abi.encodePacked(uint8(0x02), ctx1, epoch1);
        (address[] memory oldCtxV2Signers, uint256 oldCtxV2Threshold) = kmsVerifier
            .getContextSignersAndThresholdFromExtraData(v2ExtraData);
        assertEq(oldCtxV2Signers.length, 3);
        assertEq(oldCtxV2Signers[0], signer0);
        assertEq(oldCtxV2Threshold, 1);

        address[] memory nextSigners = _makeSingleSignerList(signer3);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParamsFromSigners(nextSigners), _defaultThresholds());
        uint256 pendingCtx = ctx1 + 1;
        bytes memory pendingV1ExtraData = abi.encodePacked(uint8(0x01), pendingCtx);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigCommon.InvalidKmsContext.selector, pendingCtx));
        kmsVerifier.getContextSignersAndThresholdFromExtraData(pendingV1ExtraData);
    }
}
