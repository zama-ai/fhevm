// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {Vm} from "forge-std/Test.sol";

import {KMSGeneration} from "@fhevm-host-contracts/contracts/KMSGeneration.sol";
import {IKMSGeneration} from "@fhevm-host-contracts/contracts/interfaces/IKMSGeneration.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {KmsNode, KmsNodeParams, PcrValues} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";
import {UUPSUpgradeableEmptyProxy} from "@fhevm-host-contracts/contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE, PREP_KEYGEN_COUNTER_BASE, KEY_COUNTER_BASE, CRS_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {protocolConfigAdd, kmsGenerationAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";
import {HostContractsDeployerTestUtils} from "@fhevm-host-contracts/fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {KMSGenerationUpgradedExample} from "@fhevm-host-contracts/examples/KMSGenerationUpgradedExample.sol";

contract KMSGenerationHarness is KMSGeneration {
    function extractContextIdFromExtraData(bytes memory extraData) external view returns (uint256 contextId) {
        return _extractContextIdFromExtraData(extraData);
    }
}

contract KMSGenerationTest is HostContractsDeployerTestUtils {
    KMSGeneration internal kmsGeneration;
    KMSGenerationHarness internal kmsGenerationHarness;

    // EIP-712 type hashes
    string internal constant EIP712_DOMAIN_NAME = "KMSGeneration";
    string internal constant EIP712_DOMAIN_VERSION = "1";
    bytes32 internal constant EIP712_PREP_KEYGEN_TYPE_HASH =
        keccak256("PrepKeygenVerification(uint256 prepKeygenId,bytes extraData)");

    address internal constant owner = address(456);

    // KMS node private keys for signing
    uint256 internal constant kmsPk0 = 0x100;
    uint256 internal constant kmsPk1 = 0x200;
    uint256 internal constant kmsPk2 = 0x300;
    uint256 internal constant kmsPk3 = 0x400;
    address internal kmsSigner0;
    address internal kmsSigner1;
    address internal kmsSigner2;
    address internal kmsSigner3;
    address internal kmsTxSender0 = address(0xA1);
    address internal kmsTxSender1 = address(0xA2);
    address internal kmsTxSender2 = address(0xA3);
    address internal kmsTxSender3 = address(0xA4);

    function setUp() public {
        kmsSigner0 = vm.addr(kmsPk0);
        kmsSigner1 = vm.addr(kmsPk1);
        kmsSigner2 = vm.addr(kmsPk2);
        kmsSigner3 = vm.addr(kmsPk3);

        // Deploy ACL
        _deployACL(owner);

        // Deploy ProtocolConfig with our test KMS nodes
        _deployProtocolConfig(owner, _makeKmsNodeParams(2), _defaultThresholds());
        protocolConfig = ProtocolConfig(protocolConfigAdd);

        // Deploy KMSGeneration
        _deployKMSGeneration(owner);
        kmsGeneration = KMSGeneration(kmsGenerationAdd);
        kmsGenerationHarness = new KMSGenerationHarness();
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    function _computeDomainSeparator(address verifier) internal view returns (bytes32) {
        return
            keccak256(
                abi.encode(
                    EIP712_DOMAIN_TYPE_HASH,
                    keccak256(bytes(EIP712_DOMAIN_NAME)),
                    keccak256(bytes(EIP712_DOMAIN_VERSION)),
                    block.chainid,
                    verifier
                )
            );
    }

    function _buildExtraData() internal view returns (bytes memory) {
        (uint256 contextId, uint256 epochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        return abi.encodePacked(uint8(0x02), contextId, epochId);
    }

    function _hashPrepKeygen(uint256 prepKeygenId, bytes memory extraData) internal view returns (bytes32) {
        return _hashPrepKeygen(kmsGenerationAdd, prepKeygenId, extraData);
    }

    function _hashPrepKeygen(
        address verifier,
        uint256 prepKeygenId,
        bytes memory extraData
    ) internal view returns (bytes32) {
        bytes32 structHash = keccak256(abi.encode(EIP712_PREP_KEYGEN_TYPE_HASH, prepKeygenId, keccak256(extraData)));
        return MessageHashUtils.toTypedDataHash(_computeDomainSeparator(verifier), structHash);
    }

    function _hashKeygen(
        uint256 prepKeygenId,
        uint256 keyId,
        IKMSGeneration.KeyDigest[] memory keyDigests,
        bytes memory extraData
    ) internal view returns (bytes32) {
        return _hashKeygen(kmsGenerationAdd, prepKeygenId, keyId, keyDigests, extraData);
    }

    function _hashKeygen(
        address verifier,
        uint256 prepKeygenId,
        uint256 keyId,
        IKMSGeneration.KeyDigest[] memory keyDigests,
        bytes memory extraData
    ) internal view returns (bytes32) {
        return _hashKeygenWithDomain(_computeDomainSeparator(verifier), prepKeygenId, keyId, keyDigests, extraData);
    }

    function _hashCrsgen(
        uint256 crsId,
        uint256 maxBitLength,
        bytes memory crsDigest,
        bytes memory extraData
    ) internal view returns (bytes32) {
        return _hashCrsgen(kmsGenerationAdd, crsId, maxBitLength, crsDigest, extraData);
    }

    function _hashCrsgen(
        address verifier,
        uint256 crsId,
        uint256 maxBitLength,
        bytes memory crsDigest,
        bytes memory extraData
    ) internal view returns (bytes32) {
        return _hashCrsgenWithDomain(_computeDomainSeparator(verifier), crsId, maxBitLength, crsDigest, extraData);
    }

    function _activatePendingTwoNodeContext(uint256 contextId, uint256 epochId, uint256 pk0, uint256 pk1) internal {
        _confirmContextCreation(contextId, kmsPk0);
        _confirmContextCreation(contextId, kmsPk1);
        if (pk0 != kmsPk0 && pk0 != kmsPk1) {
            _confirmContextCreation(contextId, pk0);
        }
        if (pk1 != kmsPk0 && pk1 != kmsPk1 && pk1 != pk0) {
            _confirmContextCreation(contextId, pk1);
        }
        _confirmEpochActivation(contextId, epochId, pk0, kmsTxSender0, 0, 0);
        _confirmEpochActivation(contextId, epochId, pk1, kmsTxSender1, 0, 0);
    }

    function _activatePendingDisjointTwoNodeContext(
        uint256 contextId,
        uint256 epochId,
        uint256 pk0,
        address txSender0,
        uint256 pk1,
        address txSender1
    ) internal {
        _confirmContextCreation(contextId, kmsPk0);
        _confirmContextCreation(contextId, kmsPk1);
        if (pk0 != kmsPk0 && pk0 != kmsPk1) {
            _confirmContextCreation(contextId, pk0);
        }
        if (pk1 != kmsPk0 && pk1 != kmsPk1 && pk1 != pk0) {
            _confirmContextCreation(contextId, pk1);
        }
        _confirmEpochActivation(contextId, epochId, pk0, txSender0, 0, 0);
        _confirmEpochActivation(contextId, epochId, pk1, txSender1, 0, 0);
    }

    function _activatePendingFourNodeContext(uint256 contextId, uint256 epochId) internal {
        _confirmContextCreation(contextId, kmsPk0);
        _confirmContextCreation(contextId, kmsPk1);
        _confirmContextCreation(contextId, kmsPk2);
        _confirmContextCreation(contextId, kmsPk3);
        _confirmEpochActivation(contextId, epochId, kmsPk0, kmsTxSender0, 0, 0);
        _confirmEpochActivation(contextId, epochId, kmsPk1, kmsTxSender1, 0, 0);
        _confirmEpochActivation(contextId, epochId, kmsPk2, kmsTxSender2, 0, 0);
        _confirmEpochActivation(contextId, epochId, kmsPk3, kmsTxSender3, 0, 0);
    }

    function _primaryStorageUrls() internal pure returns (string[] memory urls) {
        urls = new string[](1);
        urls[0] = "https://s0.example.com";
    }

    /// @dev Define a new KMS context with 4 nodes and kmsGen threshold 3.
    function _switchToMultiSignerContext() internal {
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(4);
        IProtocolConfig.KmsThresholds memory thresholds = IProtocolConfig.KmsThresholds({
            publicDecryption: 3,
            userDecryption: 3,
            kmsGen: 3,
            mpc: 3
        });
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, thresholds);
        _activatePendingFourNodeContext(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 2);
    }

    /// @dev Hash + sign + prank + call prepKeygenResponse for a single KMS node.
    function _doPrepKeygenResponse(uint256 prepKeygenId, uint256 pk, address sender) internal {
        _doPrepKeygenResponse(prepKeygenId, _buildExtraData(), pk, sender);
    }

    function _doPrepKeygenResponse(uint256 prepKeygenId, bytes memory extraData, uint256 pk, address sender) internal {
        bytes32 digest = _hashPrepKeygen(prepKeygenId, extraData);
        bytes memory sig = _computeSignature(pk, digest);
        vm.prank(sender);
        kmsGeneration.prepKeygenResponse(prepKeygenId, sig);
    }

    /// @dev Hash + sign + prank + call keygenResponse for a single KMS node (uses _mockKeyDigests).
    function _doKeygenResponse(uint256 prepKeygenId, uint256 keyId, uint256 pk, address sender) internal {
        _doKeygenResponse(prepKeygenId, keyId, _buildExtraData(), pk, sender);
    }

    function _doKeygenResponse(
        uint256 prepKeygenId,
        uint256 keyId,
        bytes memory extraData,
        uint256 pk,
        address sender
    ) internal {
        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();
        bytes32 digest = _hashKeygen(prepKeygenId, keyId, digests, extraData);
        bytes memory sig = _computeSignature(pk, digest);
        vm.prank(sender);
        kmsGeneration.keygenResponse(keyId, digests, sig);
    }

    /// @dev Hash + sign + prank + call crsgenResponse for a single KMS node.
    function _doCrsgenResponse(
        uint256 crsId,
        uint256 maxBitLength,
        bytes memory crsDigestData,
        uint256 pk,
        address sender
    ) internal {
        _doCrsgenResponse(crsId, maxBitLength, crsDigestData, _buildExtraData(), pk, sender);
    }

    function _doCrsgenResponse(
        uint256 crsId,
        uint256 maxBitLength,
        bytes memory crsDigestData,
        bytes memory extraData,
        uint256 pk,
        address sender
    ) internal {
        bytes32 digest = _hashCrsgen(crsId, maxBitLength, crsDigestData, extraData);
        bytes memory sig = _computeSignature(pk, digest);
        vm.prank(sender);
        kmsGeneration.crsgenResponse(crsId, crsDigestData, sig);
    }

    /// @dev Convenience overload using default maxBitLength=4096 and crsDigest=0xdeadbeef.
    function _doCrsgenResponse(uint256 crsId, uint256 pk, address sender) internal {
        _doCrsgenResponse(crsId, 4096, hex"deadbeef", pk, sender);
    }

    /// @dev Run a full keygen cycle: keygen() -> prepKeygenResponse -> keygenResponse for one node
    function _runFullKeygenCycle() internal returns (uint256 prepKeygenId, uint256 keyId) {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        keyId = KEY_COUNTER_BASE + 1;
        assertEq(kmsGeneration.getKeyCounter(), keyId);
        assertFalse(kmsGeneration.isRequestDone(keyId));

        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);
        assertFalse(kmsGeneration.isRequestDone(keyId));
        _doKeygenResponse(prepKeygenId, keyId, kmsPk0, kmsTxSender0);
        assertTrue(kmsGeneration.isRequestDone(keyId));
    }

    /// @dev Run a full CRS cycle
    function _runFullCrsCycle() internal returns (uint256 crsId) {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        crsId = CRS_COUNTER_BASE + 1;
        assertEq(kmsGeneration.getCrsCounter(), crsId);
        assertFalse(kmsGeneration.isRequestDone(crsId));
        _doCrsgenResponse(crsId, kmsPk0, kmsTxSender0);
        assertTrue(kmsGeneration.isRequestDone(crsId));
    }

    function _assumeNotCurrentKmsTxSender(address caller) internal view {
        vm.assume(!protocolConfig.isKmsTxSenderForContext(protocolConfig.getCurrentKmsContextId(), caller));
    }

    // -----------------------------------------------------------------------
    // Init tests
    // -----------------------------------------------------------------------

    function test_initSuccess() public view {
        assertEq(kmsGeneration.getVersion(), "KMSGeneration v0.3.0");
        assertEq(kmsGeneration.getActiveKeyId(), 0);
        assertEq(kmsGeneration.getActiveCrsId(), 0);
        assertEq(kmsGeneration.getKeyCounter(), KEY_COUNTER_BASE);
        assertEq(kmsGeneration.getCrsCounter(), CRS_COUNTER_BASE);
    }

    function test_revertDoubleInitFromEmptyProxy() public {
        vm.prank(owner);
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        kmsGeneration.initializeFromEmptyProxy();
    }

    function testFuzz_revertExtractContextIdMalformedV1ExtraData(bytes calldata malformedSuffix) public {
        vm.assume(malformedSuffix.length != 32);
        bytes memory extraData = abi.encodePacked(uint8(0x01), malformedSuffix);
        vm.expectRevert(IKMSGeneration.DeserializingExtraDataFail.selector);
        kmsGenerationHarness.extractContextIdFromExtraData(extraData);
    }

    function test_revertExtractContextIdV1ExtraDataTrailingBytes() public {
        uint256 contextId = protocolConfig.getCurrentKmsContextId();
        bytes memory extraData = abi.encodePacked(uint8(0x01), contextId, uint256(12345));
        vm.expectRevert(IKMSGeneration.DeserializingExtraDataFail.selector);
        kmsGenerationHarness.extractContextIdFromExtraData(extraData);
    }

    function testFuzz_revertExtractContextIdMalformedV2ExtraData(bytes calldata malformedSuffix) public {
        vm.assume(malformedSuffix.length != 64);
        bytes memory extraData = abi.encodePacked(uint8(0x02), malformedSuffix);
        vm.expectRevert(IKMSGeneration.DeserializingExtraDataFail.selector);
        kmsGenerationHarness.extractContextIdFromExtraData(extraData);
    }

    function testFuzz_revertExtractContextIdUnsupportedExtraDataVersion(bytes calldata extraData) public {
        vm.assume(extraData.length != 0);
        vm.assume(uint8(extraData[0]) != 0x00);
        vm.assume(uint8(extraData[0]) != 0x01);
        vm.assume(uint8(extraData[0]) != 0x02);
        uint8 version = uint8(extraData[0]);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.UnsupportedExtraDataVersion.selector, version));
        kmsGenerationHarness.extractContextIdFromExtraData(extraData);
    }

    function test_extractContextIdEmptyExtraDataUsesCurrentContext() public view {
        assertEq(kmsGenerationHarness.extractContextIdFromExtraData(""), protocolConfig.getCurrentKmsContextId());
    }

    function test_extractContextIdV1ExtraData() public view {
        uint256 contextId = protocolConfig.getCurrentKmsContextId();
        bytes memory extraData = abi.encodePacked(uint8(0x01), contextId);
        assertEq(kmsGenerationHarness.extractContextIdFromExtraData(extraData), contextId);
    }

    function test_extractContextIdV2ExtraData() public view {
        (uint256 contextId, uint256 epochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        bytes memory extraData = abi.encodePacked(uint8(0x02), contextId, epochId);
        assertEq(kmsGenerationHarness.extractContextIdFromExtraData(extraData), contextId);
    }

    // -----------------------------------------------------------------------
    // Full keygen cycle
    // -----------------------------------------------------------------------

    function test_fullKeygenCycle() public {
        (, uint256 keyId) = _runFullKeygenCycle();
        assertEq(kmsGeneration.getActiveKeyId(), keyId);
        assertEq(uint256(kmsGeneration.getKeyParamsType(keyId)), uint256(IKMSGeneration.ParamsType.Default));
    }

    function test_keygenMaterials() public {
        (, uint256 keyId) = _runFullKeygenCycle();
        (string[] memory urls, IKMSGeneration.KeyDigest[] memory digests) = kmsGeneration.getKeyMaterials(keyId);
        assertEq(urls.length, 1);
        assertEq(urls[0], "https://s0.example.com");
        assertEq(digests.length, 1);
    }

    function test_keygenConsensusTxSenders() public {
        _runFullKeygenCycle();
        uint256 keyId = KEY_COUNTER_BASE + 1;
        address[] memory txSenders = kmsGeneration.getConsensusTxSenders(keyId);
        assertEq(txSenders.length, 1);
        assertEq(txSenders[0], kmsTxSender0);
    }

    function test_completedKeyIdsExcludeAbortedAndExposePrepKeygenId() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);
        vm.prank(owner);
        kmsGeneration.abortKeygen(PREP_KEYGEN_COUNTER_BASE + 1);

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);
        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 2;
        uint256 keyId = KEY_COUNTER_BASE + 2;
        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);
        _doKeygenResponse(prepKeygenId, keyId, kmsPk0, kmsTxSender0);
        uint256[] memory completedKeyIds = kmsGeneration.getCompletedKeyIds();

        assertEq(completedKeyIds.length, 1);
        assertEq(completedKeyIds[0], keyId);
        assertEq(kmsGeneration.getKeyInfo(keyId).prepKeygenId, prepKeygenId);
    }

    function test_crsConsensusTxSenders() public {
        _runFullCrsCycle();
        uint256 crsId = CRS_COUNTER_BASE + 1;
        address[] memory txSenders = kmsGeneration.getConsensusTxSenders(crsId);
        assertEq(txSenders.length, 1);
        assertEq(txSenders[0], kmsTxSender0);
    }

    function test_completedCrsIdsExcludeAborted() public {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);
        vm.prank(owner);
        kmsGeneration.abortCrsgen(CRS_COUNTER_BASE + 1);

        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);
        uint256 crsId = CRS_COUNTER_BASE + 2;
        _doCrsgenResponse(crsId, kmsPk0, kmsTxSender0);
        uint256[] memory completedCrsIds = kmsGeneration.getCompletedCrsIds();

        assertEq(completedCrsIds.length, 1);
        assertEq(completedCrsIds[0], crsId);
    }

    // -----------------------------------------------------------------------
    // Full CRS cycle
    // -----------------------------------------------------------------------

    function test_fullCrsCycle() public {
        uint256 crsId = _runFullCrsCycle();
        assertEq(kmsGeneration.getActiveCrsId(), crsId);
        assertEq(uint256(kmsGeneration.getCrsParamsType(crsId)), uint256(IKMSGeneration.ParamsType.Default));
    }

    function test_emitKeygenLifecycleEvents() public {
        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;
        bytes memory extraData = _buildExtraData();
        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();

        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.PrepKeygenRequest(
            prepKeygenId,
            IKMSGeneration.ParamsType.Default,
            IKMSGeneration.KeygenMode.Fresh,
            0,
            extraData
        );
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        bytes32 prepDigest = _hashPrepKeygen(prepKeygenId, extraData);
        bytes memory prepSig = _computeSignature(kmsPk0, prepDigest);
        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.PrepKeygenResponse(prepKeygenId, prepSig, kmsTxSender0);
        vm.prank(kmsTxSender0);
        kmsGeneration.prepKeygenResponse(prepKeygenId, prepSig);

        bytes32 keyDigest = _hashKeygen(prepKeygenId, keyId, digests, extraData);
        bytes memory keySig = _computeSignature(kmsPk0, keyDigest);
        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.KeygenResponse(keyId, digests, keySig, kmsTxSender0);
        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.ActivateKey(keyId, _primaryStorageUrls(), digests);
        vm.prank(kmsTxSender0);
        kmsGeneration.keygenResponse(keyId, digests, keySig);
    }

    function test_emitCrsLifecycleEvents() public {
        uint256 crsId = CRS_COUNTER_BASE + 1;
        bytes memory extraData = _buildExtraData();
        bytes memory crsDigestData = hex"deadbeef";

        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.CrsgenRequest(crsId, 4096, IKMSGeneration.ParamsType.Default, extraData);
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        bytes32 digest = _hashCrsgen(crsId, 4096, crsDigestData, extraData);
        bytes memory sig = _computeSignature(kmsPk0, digest);
        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.CrsgenResponse(crsId, crsDigestData, sig, kmsTxSender0);
        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.ActivateCrs(crsId, _primaryStorageUrls(), crsDigestData);
        vm.prank(kmsTxSender0);
        kmsGeneration.crsgenResponse(crsId, crsDigestData, sig);
    }

    function test_crsMaterials() public {
        uint256 crsId = _runFullCrsCycle();
        (string[] memory urls, bytes memory crsDigest) = kmsGeneration.getCrsMaterials(crsId);
        assertEq(urls.length, 1);
        assertEq(urls[0], "https://s0.example.com");
        assertEq(crsDigest, hex"deadbeef");
    }

    // -----------------------------------------------------------------------
    // Sequential ordering
    // -----------------------------------------------------------------------

    function test_revertKeygenOngoing() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 expectedKeyId = KEY_COUNTER_BASE + 1;
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeygenOngoing.selector, expectedKeyId));
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);
    }

    function test_revertCrsgenOngoing() public {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        uint256 expectedCrsId = CRS_COUNTER_BASE + 1;
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.CrsgenOngoing.selector, expectedCrsId));
        kmsGeneration.crsgenRequest(2048, IKMSGeneration.ParamsType.Default);
    }

    function test_revertKeygenResponseBeforePrepKeygenConsensus() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        vm.prank(kmsTxSender0);
        vm.expectRevert(IKMSGeneration.KeyManagementRequestPending.selector);
        kmsGeneration.keygenResponse(KEY_COUNTER_BASE + 1, _mockKeyDigests(), hex"");
    }

    function test_pendingKeygenCompletesAfterContextRotationUsingPinnedContext() public {
        bytes memory oldExtraData = _buildExtraData();
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;

        address[] memory rotatedSigners = new address[](2);
        rotatedSigners[0] = kmsSigner1;
        rotatedSigners[1] = kmsSigner2;
        KmsNodeParams[] memory rotatedNodes = _makeKmsNodeParamsFromSigners(rotatedSigners);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(rotatedNodes, _defaultThresholds());
        _activatePendingTwoNodeContext(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 2, kmsPk1, kmsPk2);

        _doPrepKeygenResponse(prepKeygenId, oldExtraData, kmsPk0, kmsTxSender0);
        assertTrue(kmsGeneration.isRequestDone(prepKeygenId));

        _doKeygenResponse(prepKeygenId, keyId, oldExtraData, kmsPk0, kmsTxSender0);
        assertTrue(kmsGeneration.isRequestDone(keyId));
        assertEq(kmsGeneration.getActiveKeyId(), keyId);
    }

    function test_pendingKeygenCompletesWhenTxSenderRotatedOutOfLiveContext() public {
        bytes memory oldExtraData = _buildExtraData();
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;

        // Override both tx sender and signer fields: `_makeKmsNodeParams` reuses kmsTxSender0/1 by
        // default, and we need a context fully disjoint from the original committee.
        KmsNodeParams[] memory rotatedNodes = _makeKmsNodeParams(2);
        rotatedNodes[0].txSenderAddress = address(0xB1);
        rotatedNodes[0].signerAddress = kmsSigner1;
        rotatedNodes[1].txSenderAddress = address(0xB2);
        rotatedNodes[1].signerAddress = kmsSigner2;

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(rotatedNodes, _defaultThresholds());
        _activatePendingDisjointTwoNodeContext(
            KMS_CONTEXT_COUNTER_BASE + 2,
            EPOCH_COUNTER_BASE + 2,
            kmsPk1,
            address(0xB1),
            kmsPk2,
            address(0xB2)
        );

        // Sanity: kmsTxSender0 is no longer a tx sender under the live context.
        assertFalse(protocolConfig.isKmsTxSenderForContext(protocolConfig.getCurrentKmsContextId(), kmsTxSender0));

        bytes32 prepDigest = _hashPrepKeygen(prepKeygenId, oldExtraData);
        bytes memory prepSig = _computeSignature(kmsPk0, prepDigest);
        vm.prank(kmsTxSender0);
        kmsGeneration.prepKeygenResponse(prepKeygenId, prepSig);
        assertTrue(kmsGeneration.isRequestDone(prepKeygenId));
    }

    function test_keygenConsensusUsesRequestPinnedThreshold() public {
        uint256 requestContextId = protocolConfig.getCurrentKmsContextId();
        assertEq(protocolConfig.getKmsGenThresholdForContext(requestContextId), 1);

        bytes memory oldExtraData = _buildExtraData();
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;

        _switchToMultiSignerContext();
        assertEq(protocolConfig.getKmsGenThresholdForContext(protocolConfig.getCurrentKmsContextId()), 3);

        _doPrepKeygenResponse(prepKeygenId, oldExtraData, kmsPk0, kmsTxSender0);

        assertTrue(kmsGeneration.isRequestDone(prepKeygenId));
        assertEq(kmsGeneration.getConsensusTxSenders(prepKeygenId).length, 1);
    }

    function test_keygenUsesActiveContextWhenContextRotationIsPending() public {
        address[] memory rotatedSigners = new address[](2);
        rotatedSigners[0] = kmsSigner1;
        rotatedSigners[1] = kmsSigner2;
        KmsNodeParams[] memory rotatedNodes = _makeKmsNodeParamsFromSigners(rotatedSigners);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(rotatedNodes, _defaultThresholds());

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;

        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);
        _doKeygenResponse(prepKeygenId, keyId, kmsPk0, kmsTxSender0);

        assertTrue(kmsGeneration.isRequestDone(keyId));
        assertEq(kmsGeneration.getConsensusTxSenders(keyId)[0], kmsTxSender0);
    }

    function test_crsgenUsesActiveContextWhenContextRotationIsCreated() public {
        address[] memory rotatedSigners = new address[](2);
        rotatedSigners[0] = kmsSigner1;
        rotatedSigners[1] = kmsSigner2;
        KmsNodeParams[] memory rotatedNodes = _makeKmsNodeParamsFromSigners(rotatedSigners);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(rotatedNodes, _defaultThresholds());
        _confirmContextCreation(KMS_CONTEXT_COUNTER_BASE + 2, kmsPk0);
        _confirmContextCreation(KMS_CONTEXT_COUNTER_BASE + 2, kmsPk1);
        _confirmContextCreation(KMS_CONTEXT_COUNTER_BASE + 2, kmsPk2);

        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        uint256 crsId = CRS_COUNTER_BASE + 1;

        _doCrsgenResponse(crsId, 4096, hex"deadbeef", kmsPk0, kmsTxSender0);

        assertTrue(kmsGeneration.isRequestDone(crsId));
        assertEq(kmsGeneration.getConsensusTxSenders(crsId)[0], kmsTxSender0);
    }

    function test_pendingCrsgenCompletesAfterContextRotationUsingPinnedContext() public {
        bytes memory oldExtraData = _buildExtraData();
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        uint256 crsId = CRS_COUNTER_BASE + 1;

        address[] memory rotatedSigners = new address[](2);
        rotatedSigners[0] = kmsSigner1;
        rotatedSigners[1] = kmsSigner2;
        KmsNodeParams[] memory rotatedNodes = _makeKmsNodeParamsFromSigners(rotatedSigners);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(rotatedNodes, _defaultThresholds());
        _activatePendingTwoNodeContext(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 2, kmsPk1, kmsPk2);

        _doCrsgenResponse(crsId, 4096, hex"deadbeef", oldExtraData, kmsPk0, kmsTxSender0);

        assertTrue(kmsGeneration.isRequestDone(crsId));
        assertEq(kmsGeneration.getActiveCrsId(), crsId);
    }

    function test_revertPrepKeygenResponseSignedWithStaleContextExtraData() public {
        bytes memory oldExtraData = _buildExtraData();

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);
        vm.prank(owner);
        kmsGeneration.abortKeygen(PREP_KEYGEN_COUNTER_BASE + 1);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        _activatePendingTwoNodeContext(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 2, kmsPk0, kmsPk1);

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 2;
        bytes32 replayDigest = _hashPrepKeygen(prepKeygenId, oldExtraData);
        bytes memory replaySig = _computeSignature(kmsPk0, replayDigest);

        vm.prank(kmsTxSender0);
        vm.expectPartialRevert(IKMSGeneration.NotKmsSigner.selector);
        kmsGeneration.prepKeygenResponse(prepKeygenId, replaySig);
    }

    function test_revertKeygenResponseSignedWithStaleContextExtraData() public {
        bytes memory oldExtraData = _buildExtraData();

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);
        vm.prank(owner);
        kmsGeneration.abortKeygen(PREP_KEYGEN_COUNTER_BASE + 1);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        _activatePendingTwoNodeContext(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 2, kmsPk0, kmsPk1);

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 keyId = kmsGeneration.getKeyCounter();
        uint256 prepKeygenId = _prepKeygenIdForKeyId(keyId);
        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);

        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();
        bytes32 replayDigest = _hashKeygen(prepKeygenId, keyId, digests, oldExtraData);
        bytes memory replaySig = _computeSignature(kmsPk0, replayDigest);

        vm.prank(kmsTxSender0);
        vm.expectPartialRevert(IKMSGeneration.NotKmsSigner.selector);
        kmsGeneration.keygenResponse(keyId, digests, replaySig);
    }

    function test_revertCrsgenResponseSignedWithStaleContextExtraData() public {
        bytes memory oldExtraData = _buildExtraData();

        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);
        vm.prank(owner);
        kmsGeneration.abortCrsgen(CRS_COUNTER_BASE + 1);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        _activatePendingTwoNodeContext(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 2, kmsPk0, kmsPk1);

        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        uint256 crsId = CRS_COUNTER_BASE + 2;
        bytes memory crsDigestData = hex"deadbeef";
        bytes32 replayDigest = _hashCrsgen(crsId, 4096, crsDigestData, oldExtraData);
        bytes memory replaySig = _computeSignature(kmsPk0, replayDigest);

        vm.prank(kmsTxSender0);
        vm.expectPartialRevert(IKMSGeneration.NotKmsSigner.selector);
        kmsGeneration.crsgenResponse(crsId, crsDigestData, replaySig);
    }

    // -----------------------------------------------------------------------
    // Access control
    // -----------------------------------------------------------------------

    function testFuzz_revertKeygenNotOwner(address caller) public {
        vm.assume(caller != owner);
        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, caller));
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);
    }

    function testFuzz_revertCrsgenNotOwner(address caller) public {
        vm.assume(caller != owner);
        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, caller));
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);
    }

    function testFuzz_revertPrepKeygenResponseNotTxSender(address caller) public {
        _assumeNotCurrentKmsTxSender(caller);
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.NotKmsTxSender.selector, caller));
        kmsGeneration.prepKeygenResponse(PREP_KEYGEN_COUNTER_BASE + 1, hex"");
    }

    function testFuzz_revertKeygenResponseNotTxSender(address caller) public {
        _assumeNotCurrentKmsTxSender(caller);
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();
        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.NotKmsTxSender.selector, caller));
        kmsGeneration.keygenResponse(KEY_COUNTER_BASE + 1, digests, hex"");
    }

    function testFuzz_revertCrsgenResponseNotTxSender(address caller) public {
        _assumeNotCurrentKmsTxSender(caller);
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.NotKmsTxSender.selector, caller));
        kmsGeneration.crsgenResponse(CRS_COUNTER_BASE + 1, hex"deadbeef", hex"");
    }

    // -----------------------------------------------------------------------
    // Duplicate signature rejection
    // -----------------------------------------------------------------------

    function test_revertDuplicatePrepKeygenSignature() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;

        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);

        bytes memory extraData = _buildExtraData();
        bytes32 digest = _hashPrepKeygen(prepKeygenId, extraData);
        bytes memory sig = _computeSignature(kmsPk0, digest);
        vm.expectRevert(
            abi.encodeWithSelector(IKMSGeneration.KmsAlreadySignedForPrepKeygen.selector, prepKeygenId, kmsSigner0)
        );
        vm.prank(kmsTxSender0);
        kmsGeneration.prepKeygenResponse(prepKeygenId, sig);
    }

    function test_revertDuplicateKeygenSignature() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;

        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);
        _doKeygenResponse(prepKeygenId, keyId, kmsPk0, kmsTxSender0);

        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();
        bytes memory extraData = _buildExtraData();
        bytes32 keyDigest = _hashKeygen(prepKeygenId, keyId, digests, extraData);
        bytes memory keySig = _computeSignature(kmsPk0, keyDigest);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KmsAlreadySignedForKeygen.selector, keyId, kmsSigner0));
        vm.prank(kmsTxSender0);
        kmsGeneration.keygenResponse(keyId, digests, keySig);
    }

    function test_revertDuplicateCrsgenSignature() public {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        uint256 crsId = CRS_COUNTER_BASE + 1;

        _doCrsgenResponse(crsId, kmsPk0, kmsTxSender0);

        bytes memory extraData = _buildExtraData();
        bytes32 digest = _hashCrsgen(crsId, 4096, hex"deadbeef", extraData);
        bytes memory sig = _computeSignature(kmsPk0, digest);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KmsAlreadySignedForCrsgen.selector, crsId, kmsSigner0));
        vm.prank(kmsTxSender0);
        kmsGeneration.crsgenResponse(crsId, hex"deadbeef", sig);
    }

    // -----------------------------------------------------------------------
    // Invalid request ID
    // -----------------------------------------------------------------------

    function testFuzz_revertPrepKeygenNotRequested(uint256 prepKeygenId) public {
        vm.prank(kmsTxSender0);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.PrepKeygenNotRequested.selector, prepKeygenId));
        kmsGeneration.prepKeygenResponse(prepKeygenId, hex"");
    }

    function testFuzz_revertKeygenNotRequested(uint256 keyId) public {
        IKMSGeneration.KeyDigest[] memory d = _mockKeyDigests();
        vm.prank(kmsTxSender0);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeygenNotRequested.selector, keyId));
        kmsGeneration.keygenResponse(keyId, d, hex"");
    }

    function testFuzz_revertCrsgenNotRequested(uint256 crsId) public {
        vm.prank(kmsTxSender0);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.CrsgenNotRequested.selector, crsId));
        kmsGeneration.crsgenResponse(crsId, hex"deadbeef", hex"");
    }

    function test_revertKmsSignerDoesNotMatchTxSender() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;

        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);

        // Sign with kmsSigner1's key but send from kmsTxSender0 (mismatch)
        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();
        bytes memory extraData = _buildExtraData();
        bytes32 keyDigest = _hashKeygen(prepKeygenId, keyId, digests, extraData);
        bytes memory keySig = _computeSignature(kmsPk1, keyDigest);

        vm.prank(kmsTxSender0);
        vm.expectRevert(
            abi.encodeWithSelector(IKMSGeneration.KmsSignerDoesNotMatchTxSender.selector, kmsSigner1, kmsTxSender0)
        );
        kmsGeneration.keygenResponse(keyId, digests, keySig);
    }

    function test_revertNotKmsSigner() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;

        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);

        // Sign with a key that is not a registered KMS signer for the context
        uint256 unknownPk = 0x999;
        address unknownSigner = vm.addr(unknownPk);
        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();
        bytes memory extraData = _buildExtraData();
        bytes32 keyDigest = _hashKeygen(prepKeygenId, keyId, digests, extraData);
        bytes memory keySig = _computeSignature(unknownPk, keyDigest);

        vm.prank(kmsTxSender0);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.NotKmsSigner.selector, unknownSigner));
        kmsGeneration.keygenResponse(keyId, digests, keySig);
    }

    function test_revertKmsSignerDoesNotMatchTxSenderPrepKeygen() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        bytes memory extraData = _buildExtraData();
        // Sign with kmsSigner0's key but send from kmsTxSender1
        bytes32 prepDigest = _hashPrepKeygen(prepKeygenId, extraData);
        bytes memory prepSig = _computeSignature(kmsPk0, prepDigest);

        vm.prank(kmsTxSender1);
        vm.expectRevert(
            abi.encodeWithSelector(IKMSGeneration.KmsSignerDoesNotMatchTxSender.selector, kmsSigner0, kmsTxSender1)
        );
        kmsGeneration.prepKeygenResponse(prepKeygenId, prepSig);
    }

    function test_revertKmsSignerDoesNotMatchTxSenderCrsgen() public {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        uint256 crsId = CRS_COUNTER_BASE + 1;
        bytes memory extraData = _buildExtraData();
        bytes memory crsDigestData = hex"deadbeef";
        // Sign with kmsSigner0's key but send from kmsTxSender1
        bytes32 digest = _hashCrsgen(crsId, 4096, crsDigestData, extraData);
        bytes memory sig = _computeSignature(kmsPk0, digest);

        vm.prank(kmsTxSender1);
        vm.expectRevert(
            abi.encodeWithSelector(IKMSGeneration.KmsSignerDoesNotMatchTxSender.selector, kmsSigner0, kmsTxSender1)
        );
        kmsGeneration.crsgenResponse(crsId, crsDigestData, sig);
    }

    function test_revertEmptyKeyDigests() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 keyId = KEY_COUNTER_BASE + 1;
        IKMSGeneration.KeyDigest[] memory digests = new IKMSGeneration.KeyDigest[](0);

        vm.prank(kmsTxSender0);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.EmptyKeyDigests.selector, keyId));
        kmsGeneration.keygenResponse(keyId, digests, hex"");
    }

    // -----------------------------------------------------------------------
    // Abort flows
    // -----------------------------------------------------------------------

    function test_abortKeygen() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;
        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.AbortKeygen(prepKeygenId);
        vm.prank(owner);
        kmsGeneration.abortKeygen(prepKeygenId);

        assertTrue(kmsGeneration.isRequestDone(prepKeygenId));
        assertTrue(kmsGeneration.isRequestDone(keyId));
    }

    function test_abortKeygenAfterPrepConsensus() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;
        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);
        assertFalse(kmsGeneration.isRequestDone(keyId));

        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.AbortKeygen(prepKeygenId);
        vm.prank(owner);
        kmsGeneration.abortKeygen(prepKeygenId);

        assertTrue(kmsGeneration.isRequestDone(prepKeygenId));
        assertTrue(kmsGeneration.isRequestDone(keyId));

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Test, IKMSGeneration.KeygenMode.Fresh, 0);
    }

    function test_abortPendingKeygenAfterContextRotationAllowsNewKeygen() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 abortedPrepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 abortedKeyId = KEY_COUNTER_BASE + 1;

        address[] memory rotatedSigners = new address[](2);
        rotatedSigners[0] = kmsSigner1;
        rotatedSigners[1] = kmsSigner2;
        KmsNodeParams[] memory rotatedNodes = _makeKmsNodeParamsFromSigners(rotatedSigners);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(rotatedNodes, _defaultThresholds());
        _activatePendingTwoNodeContext(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 2, kmsPk1, kmsPk2);

        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.AbortKeygen(abortedPrepKeygenId);
        vm.prank(owner);
        kmsGeneration.abortKeygen(abortedPrepKeygenId);

        assertTrue(kmsGeneration.isRequestDone(abortedPrepKeygenId));
        assertTrue(kmsGeneration.isRequestDone(abortedKeyId));

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Test, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 2;
        uint256 keyId = KEY_COUNTER_BASE + 2;

        _doPrepKeygenResponse(prepKeygenId, kmsPk1, kmsTxSender0);
        _doKeygenResponse(prepKeygenId, keyId, kmsPk1, kmsTxSender0);

        assertEq(kmsGeneration.getActiveKeyId(), keyId);
        assertEq(uint256(kmsGeneration.getKeyParamsType(keyId)), uint256(IKMSGeneration.ParamsType.Test));
    }

    function test_abortCrsgen() public {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        uint256 crsId = CRS_COUNTER_BASE + 1;
        assertFalse(kmsGeneration.isRequestDone(crsId));
        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.AbortCrsgen(crsId);
        vm.prank(owner);
        kmsGeneration.abortCrsgen(crsId);

        assertTrue(kmsGeneration.isRequestDone(crsId));
    }

    function testFuzz_revertAbortKeygenInvalidId(uint256 prepKeygenId) public {
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.AbortKeygenInvalidId.selector, prepKeygenId));
        kmsGeneration.abortKeygen(prepKeygenId);
    }

    function test_revertAbortKeygenAlreadyDoneAfterCompletion() public {
        (uint256 prepKeygenId, ) = _runFullKeygenCycle();
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.AbortKeygenAlreadyDone.selector, prepKeygenId));
        kmsGeneration.abortKeygen(prepKeygenId);
    }

    function test_revertAbortKeygenAlreadyDoneAfterAbort() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);
        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;

        vm.prank(owner);
        kmsGeneration.abortKeygen(prepKeygenId);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.AbortKeygenAlreadyDone.selector, prepKeygenId));
        kmsGeneration.abortKeygen(prepKeygenId);
    }

    function testFuzz_revertAbortCrsgenInvalidId(uint256 crsId) public {
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.AbortCrsgenInvalidId.selector, crsId));
        kmsGeneration.abortCrsgen(crsId);
    }

    function test_revertAbortCrsgenAlreadyDone() public {
        uint256 crsId = _runFullCrsCycle();
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.AbortCrsgenAlreadyDone.selector, crsId));
        kmsGeneration.abortCrsgen(crsId);
    }

    function testFuzz_abortKeygenNotOwner(address caller) public {
        vm.assume(caller != owner);
        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, caller));
        kmsGeneration.abortKeygen(PREP_KEYGEN_COUNTER_BASE + 1);
    }

    function testFuzz_abortCrsgenNotOwner(address caller) public {
        vm.assume(caller != owner);
        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, caller));
        kmsGeneration.abortCrsgen(CRS_COUNTER_BASE + 1);
    }

    function test_keygenAfterAbort() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 abortedPrepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        vm.prank(owner);
        kmsGeneration.abortKeygen(abortedPrepKeygenId);

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Test, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 2;
        uint256 keyId = KEY_COUNTER_BASE + 2;

        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);
        _doKeygenResponse(prepKeygenId, keyId, kmsPk0, kmsTxSender0);

        assertEq(kmsGeneration.getActiveKeyId(), keyId);
        assertEq(uint256(kmsGeneration.getKeyParamsType(keyId)), uint256(IKMSGeneration.ParamsType.Test));
    }

    function test_revertGetAbortedKeyViews() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;
        vm.prank(owner);
        kmsGeneration.abortKeygen(prepKeygenId);

        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeyAborted.selector, keyId));
        kmsGeneration.getKeyParamsType(keyId);

        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeyAborted.selector, keyId));
        kmsGeneration.getKeyMaterials(keyId);
    }

    function test_crsgenAfterAbort() public {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        uint256 abortedCrsId = CRS_COUNTER_BASE + 1;
        vm.prank(owner);
        kmsGeneration.abortCrsgen(abortedCrsId);

        vm.prank(owner);
        kmsGeneration.crsgenRequest(8192, IKMSGeneration.ParamsType.Test);

        uint256 crsId = CRS_COUNTER_BASE + 2;

        _doCrsgenResponse(crsId, 8192, hex"cafebabe", kmsPk0, kmsTxSender0);

        assertEq(kmsGeneration.getActiveCrsId(), crsId);
        assertEq(uint256(kmsGeneration.getCrsParamsType(crsId)), uint256(IKMSGeneration.ParamsType.Test));
    }

    function test_revertGetAbortedCrsViews() public {
        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);

        uint256 crsId = CRS_COUNTER_BASE + 1;
        vm.prank(owner);
        kmsGeneration.abortCrsgen(crsId);

        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.CrsAborted.selector, crsId));
        kmsGeneration.getCrsParamsType(crsId);

        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.CrsAborted.selector, crsId));
        kmsGeneration.getCrsMaterials(crsId);
    }

    // -----------------------------------------------------------------------
    // Nonexistent ID view reverts (never started)
    // -----------------------------------------------------------------------

    function testFuzz_revertGetKeyParamsTypeNonexistent(uint256 keyId) public {
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeyNotGenerated.selector, keyId));
        kmsGeneration.getKeyParamsType(keyId);
    }

    function testFuzz_revertGetKeyMaterialsNonexistent(uint256 keyId) public {
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeyNotGenerated.selector, keyId));
        kmsGeneration.getKeyMaterials(keyId);
    }

    function testFuzz_revertGetCrsParamsTypeNonexistent(uint256 crsId) public {
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.CrsNotGenerated.selector, crsId));
        kmsGeneration.getCrsParamsType(crsId);
    }

    function testFuzz_revertGetCrsMaterialsNonexistent(uint256 crsId) public {
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.CrsNotGenerated.selector, crsId));
        kmsGeneration.getCrsMaterials(crsId);
    }

    // -----------------------------------------------------------------------
    // Second keygen cycle after first completes
    // -----------------------------------------------------------------------

    function test_secondKeygenCycleAfterFirst() public {
        _runFullKeygenCycle();

        // Start second keygen
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Test, IKMSGeneration.KeygenMode.Fresh, 0);

        uint256 prepKeygenId2 = PREP_KEYGEN_COUNTER_BASE + 2;
        uint256 keyId2 = KEY_COUNTER_BASE + 2;

        _doPrepKeygenResponse(prepKeygenId2, kmsPk0, kmsTxSender0);
        _doKeygenResponse(prepKeygenId2, keyId2, kmsPk0, kmsTxSender0);

        assertEq(kmsGeneration.getActiveKeyId(), keyId2);
        assertEq(uint256(kmsGeneration.getKeyParamsType(keyId2)), uint256(IKMSGeneration.ParamsType.Test));
    }

    // -----------------------------------------------------------------------
    // Upgrade (V3 -> V4)
    // -----------------------------------------------------------------------

    function test_upgradeToV4() public {
        // Verify initial version
        assertEq(kmsGeneration.getVersion(), "KMSGeneration v0.3.0");
        assertEq(kmsGeneration.getActiveKeyId(), 0);
        assertEq(kmsGeneration.getActiveCrsId(), 0);

        // Deploy the upgraded implementation and upgrade
        address v4Impl = address(new KMSGenerationUpgradedExample());
        vm.prank(owner);
        kmsGeneration.upgradeToAndCall(v4Impl, "");

        // Verify new version
        string memory newVersion = kmsGeneration.getVersion();
        assertEq(newVersion, "KMSGeneration v0.4.0");

        // Verify state is preserved
        assertEq(kmsGeneration.getActiveKeyId(), 0);
        assertEq(kmsGeneration.getActiveCrsId(), 0);
    }

    // -----------------------------------------------------------------------
    // Multi-signer post-consensus ignore (4 nodes, kmsGen threshold 3)
    // -----------------------------------------------------------------------

    function test_postConsensusPrepKeygenResponseIgnored() public {
        _switchToMultiSignerContext();

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);
        uint256 prepKeygenId = _prepKeygenIdForKeyId(kmsGeneration.getKeyCounter());

        // Responses 1-3 reach consensus (threshold = 3)
        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);
        _doPrepKeygenResponse(prepKeygenId, kmsPk1, kmsTxSender1);
        _doPrepKeygenResponse(prepKeygenId, kmsPk2, kmsTxSender2);

        // 4th response should be silently ignored (no KeygenRequest event, no revert)
        vm.expectEmit(false, false, false, false, address(kmsGeneration), 0);
        emit IKMSGeneration.KeygenRequest(0, 0, IKMSGeneration.KeygenMode.Fresh, 0, "");
        _doPrepKeygenResponse(prepKeygenId, kmsPk3, kmsTxSender3);
    }

    function test_postConsensusKeygenResponseIgnored() public {
        _switchToMultiSignerContext();

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);
        uint256 keyId = kmsGeneration.getKeyCounter();
        uint256 prepKeygenId = _prepKeygenIdForKeyId(keyId);

        // Complete prepKeygen consensus (3 of 4)
        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);
        _doPrepKeygenResponse(prepKeygenId, kmsPk1, kmsTxSender1);
        _doPrepKeygenResponse(prepKeygenId, kmsPk2, kmsTxSender2);

        // Keygen responses 1-3 reach consensus
        _doKeygenResponse(prepKeygenId, keyId, kmsPk0, kmsTxSender0);
        _doKeygenResponse(prepKeygenId, keyId, kmsPk1, kmsTxSender1);
        _doKeygenResponse(prepKeygenId, keyId, kmsPk2, kmsTxSender2);

        // 4th response should be silently ignored (no ActivateKey event, no revert)
        vm.expectEmit(false, false, false, false, address(kmsGeneration), 0);
        emit IKMSGeneration.ActivateKey(0, new string[](0), new IKMSGeneration.KeyDigest[](0));
        _doKeygenResponse(prepKeygenId, keyId, kmsPk3, kmsTxSender3);
    }

    function test_postConsensusCrsgenResponseIgnored() public {
        _switchToMultiSignerContext();

        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);
        uint256 crsId = kmsGeneration.getCrsCounter();

        // Responses 1-3 reach consensus
        _doCrsgenResponse(crsId, kmsPk0, kmsTxSender0);
        _doCrsgenResponse(crsId, kmsPk1, kmsTxSender1);
        _doCrsgenResponse(crsId, kmsPk2, kmsTxSender2);

        // 4th response should be silently ignored (no ActivateCrs event, no revert)
        vm.expectEmit(false, false, false, false, address(kmsGeneration), 0);
        emit IKMSGeneration.ActivateCrs(0, new string[](0), "");
        _doCrsgenResponse(crsId, kmsPk3, kmsTxSender3);
    }

    // -----------------------------------------------------------------------
    // getKeyInfo revert branches + happy path
    // -----------------------------------------------------------------------

    function testFuzz_revertGetKeyInfoNotGenerated(uint256 keyId) public {
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeyNotGenerated.selector, keyId));
        kmsGeneration.getKeyInfo(keyId);
    }

    function test_revertGetKeyInfoAborted() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);
        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;
        vm.prank(owner);
        kmsGeneration.abortKeygen(prepKeygenId);

        // isRequestDone is set on abort but consensusDigest stays zero -> KeyAborted, not KeyNotGenerated.
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeyAborted.selector, keyId));
        kmsGeneration.getKeyInfo(keyId);
    }

    function test_getKeyInfoHappyPathFields() public {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Test, IKMSGeneration.KeygenMode.Fresh, 0);
        uint256 prepKeygenId = PREP_KEYGEN_COUNTER_BASE + 1;
        uint256 keyId = KEY_COUNTER_BASE + 1;
        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);
        _doKeygenResponse(prepKeygenId, keyId, kmsPk0, kmsTxSender0);

        IKMSGeneration.KeyInfo memory info = kmsGeneration.getKeyInfo(keyId);
        assertEq(info.prepKeygenId, prepKeygenId);
        assertEq(info.keyId, keyId);
        assertEq(uint256(info.paramsType), uint256(IKMSGeneration.ParamsType.Test));
        IKMSGeneration.KeyDigest[] memory expected = _mockKeyDigests();
        assertEq(info.keyDigests.length, expected.length);
        assertEq(uint256(info.keyDigests[0].keyType), uint256(expected[0].keyType));
        assertEq(info.keyDigests[0].digest, expected[0].digest);
    }

    // -----------------------------------------------------------------------
    // completedKeyIds / completedCrsIds multi-element ordering
    // -----------------------------------------------------------------------

    function test_completedKeyIdsAccumulateInOrder() public {
        // First key.
        (, uint256 keyId1) = _runFullKeygenCycle();
        // Second key.
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Test, IKMSGeneration.KeygenMode.Fresh, 0);
        uint256 prepKeygenId2 = PREP_KEYGEN_COUNTER_BASE + 2;
        uint256 keyId2 = KEY_COUNTER_BASE + 2;
        _doPrepKeygenResponse(prepKeygenId2, kmsPk0, kmsTxSender0);
        _doKeygenResponse(prepKeygenId2, keyId2, kmsPk0, kmsTxSender0);

        uint256[] memory completed = kmsGeneration.getCompletedKeyIds();
        assertEq(completed.length, 2);
        assertEq(completed[0], keyId1);
        assertEq(completed[1], keyId2);
    }

    function test_completedCrsIdsAccumulateInOrder() public {
        uint256 crsId1 = _runFullCrsCycle();
        vm.prank(owner);
        kmsGeneration.crsgenRequest(8192, IKMSGeneration.ParamsType.Test);
        uint256 crsId2 = CRS_COUNTER_BASE + 2;
        _doCrsgenResponse(crsId2, 8192, hex"cafebabe", kmsPk0, kmsTxSender0);

        uint256[] memory completed = kmsGeneration.getCompletedCrsIds();
        assertEq(completed.length, 2);
        assertEq(completed[0], crsId1);
        assertEq(completed[1], crsId2);
    }

    function test_fullKeygenCycleMultiSigner() public {
        _switchToMultiSignerContext();

        bytes memory extraData = _buildExtraData();
        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);
        uint256 keyId = kmsGeneration.getKeyCounter();
        uint256 prepKeygenId = _prepKeygenIdForKeyId(keyId);

        // prepKeygen: first 2 responses don't trigger KeygenRequest
        _doPrepKeygenResponse(prepKeygenId, kmsPk0, kmsTxSender0);
        _doPrepKeygenResponse(prepKeygenId, kmsPk1, kmsTxSender1);

        // 3rd prepKeygen response triggers KeygenRequest event (consensus reached)
        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.KeygenRequest(prepKeygenId, keyId, IKMSGeneration.KeygenMode.Fresh, 0, extraData);
        _doPrepKeygenResponse(prepKeygenId, kmsPk2, kmsTxSender2);

        // keygen: first 2 don't trigger ActivateKey
        _doKeygenResponse(prepKeygenId, keyId, kmsPk0, kmsTxSender0);
        _doKeygenResponse(prepKeygenId, keyId, kmsPk1, kmsTxSender1);

        // 3rd keygen response triggers ActivateKey
        string[] memory urls = new string[](3);
        urls[0] = "https://s0.example.com";
        urls[1] = "https://s1.example.com";
        urls[2] = "https://s2.example.com";
        vm.expectEmit(true, true, true, true, address(kmsGeneration));
        emit IKMSGeneration.ActivateKey(keyId, urls, digests);
        _doKeygenResponse(prepKeygenId, keyId, kmsPk2, kmsTxSender2);

        assertEq(kmsGeneration.getActiveKeyId(), keyId);
    }

    // -----------------------------------------------------------------------
    // RFC-029 one-time compressed-key migration
    // -----------------------------------------------------------------------

    /// @dev Mirrors KMS Core's KeygenVerification::new_compressed digest set:
    /// [PUBLIC over the untouched public key, COMPRESSED_KEYSET over the new blob].
    function _mockCompressedKeyDigests() internal pure returns (IKMSGeneration.KeyDigest[] memory) {
        IKMSGeneration.KeyDigest[] memory digests = new IKMSGeneration.KeyDigest[](2);
        digests[0] = IKMSGeneration.KeyDigest({keyType: IKMSGeneration.KeyType.Public, digest: hex"deadbeef"});
        digests[1] = IKMSGeneration.KeyDigest({
            keyType: IKMSGeneration.KeyType.CompressedKeyset,
            digest: hex"c0ffee00"
        });
        return digests;
    }

    /// @dev Hash + sign + prank + call keygenResponse with compressed digests
    /// for a FromExisting request (single KMS node).
    function _doFromExistingKeygenResponse(
        uint256 prepKeygenId,
        uint256 migrationRequestId,
        uint256 pk,
        address sender
    ) internal {
        IKMSGeneration.KeyDigest[] memory digests = _mockCompressedKeyDigests();
        bytes32 digest = _hashKeygen(prepKeygenId, migrationRequestId, digests, _buildExtraData());
        bytes memory sig = _computeSignature(pk, digest);
        vm.prank(sender);
        kmsGeneration.keygenResponse(migrationRequestId, digests, sig);
    }

    /// @dev Run a full migration cycle for an existing key; returns the migration request pair.
    function _runFullMigrationCycle(
        uint256 keyId
    ) internal returns (uint256 migrationPrepId, uint256 migrationRequestId) {
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.FromExisting, keyId);

        migrationPrepId = kmsGeneration.getKeyCounter() - KEY_COUNTER_BASE + PREP_KEYGEN_COUNTER_BASE;
        migrationRequestId = kmsGeneration.getKeyCounter();

        _doPrepKeygenResponse(migrationPrepId, kmsPk0, kmsTxSender0);
        _doFromExistingKeygenResponse(migrationPrepId, migrationRequestId, kmsPk0, kmsTxSender0);
    }

    function test_compressedKeyMigrationFullCycleDoesNotActivate() public {
        (, uint256 keyId) = _runFullKeygenCycle();
        uint256 activeKeyIdBefore = kmsGeneration.getActiveKeyId();
        uint256 completedBefore = kmsGeneration.getCompletedKeyIds().length;

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.FromExisting, keyId);
        uint256 migrationPrepId = PREP_KEYGEN_COUNTER_BASE + 2;
        uint256 migrationRequestId = KEY_COUNTER_BASE + 2;

        // The prep consensus triggers the typed migration keygen request, not KeygenRequest.
        vm.recordLogs();
        _doPrepKeygenResponse(migrationPrepId, kmsPk0, kmsTxSender0);
        _doFromExistingKeygenResponse(migrationPrepId, migrationRequestId, kmsPk0, kmsTxSender0);
        Vm.Log[] memory logs = vm.getRecordedLogs();
        bool sawFromExistingKeygenRequest;
        for (uint256 i = 0; i < logs.length; i++) {
            assertTrue(
                logs[i].topics[0] != IKMSGeneration.ActivateKey.selector,
                "FromExisting flow must never emit ActivateKey"
            );
            if (logs[i].topics[0] == IKMSGeneration.KeygenRequest.selector) {
                (, , IKMSGeneration.KeygenMode mode, uint256 existing, ) = abi.decode(
                    logs[i].data,
                    (uint256, uint256, IKMSGeneration.KeygenMode, uint256, bytes)
                );
                assertTrue(mode == IKMSGeneration.KeygenMode.FromExisting, "mode must be FromExisting");
                assertEq(existing, keyId);
                sawFromExistingKeygenRequest = true;
            }
        }
        assertTrue(sawFromExistingKeygenRequest);

        // Publication is not activation.
        assertEq(kmsGeneration.getActiveKeyId(), activeKeyIdBefore);
        assertEq(kmsGeneration.getCompletedKeyIds().length, completedBefore);
        assertTrue(kmsGeneration.isRequestDone(migrationRequestId));

        // The compressed materials are published under the existing keyId.
        (string[] memory urls, IKMSGeneration.KeyDigest[] memory digests) = kmsGeneration.getCompressedKeyMaterials(
            keyId
        );
        assertEq(urls.length, 1);
        assertEq(digests.length, 2);
        assertEq(uint256(digests[1].keyType), uint256(IKMSGeneration.KeyType.CompressedKeyset));
        assertEq(digests[1].digest, hex"c0ffee00");

        // The original key materials are untouched.
        (, IKMSGeneration.KeyDigest[] memory originalDigests) = kmsGeneration.getKeyMaterials(keyId);
        assertEq(originalDigests[0].digest, _mockKeyDigests()[0].digest);
    }

    function test_revertFromExistingResponseWithoutCompressedDigest() public {
        (, uint256 keyId) = _runFullKeygenCycle();
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.FromExisting, keyId);
        uint256 migrationPrepId = PREP_KEYGEN_COUNTER_BASE + 2;
        uint256 migrationRequestId = KEY_COUNTER_BASE + 2;
        _doPrepKeygenResponse(migrationPrepId, kmsPk0, kmsTxSender0);

        // A FromExisting response without a CompressedKeyset digest is
        // useless to coprocessors and must be rejected up front.
        IKMSGeneration.KeyDigest[] memory digests = _mockKeyDigests();
        bytes32 digest = _hashKeygen(migrationPrepId, migrationRequestId, digests, _buildExtraData());
        bytes memory sig = _computeSignature(kmsPk0, digest);
        vm.prank(kmsTxSender0);
        vm.expectRevert(
            abi.encodeWithSelector(IKMSGeneration.MissingCompressedKeysetDigest.selector, migrationRequestId)
        );
        kmsGeneration.keygenResponse(migrationRequestId, digests, sig);
    }

    function test_revertFreshKeygenWithExistingKeyId() public {
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.InvalidExistingKeyId.selector, uint256(42)));
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 42);
    }

    function test_revertMigrationKeygenForUngeneratedOrAbortedKey() public {
        // Never requested.
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeyNotGenerated.selector, KEY_COUNTER_BASE + 1));
        kmsGeneration.keygen(
            IKMSGeneration.ParamsType.Default,
            IKMSGeneration.KeygenMode.FromExisting,
            KEY_COUNTER_BASE + 1
        );

        // Aborted key.
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);
        vm.prank(owner);
        kmsGeneration.abortKeygen(PREP_KEYGEN_COUNTER_BASE + 1);
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeyAborted.selector, KEY_COUNTER_BASE + 1));
        kmsGeneration.keygen(
            IKMSGeneration.ParamsType.Default,
            IKMSGeneration.KeygenMode.FromExisting,
            KEY_COUNTER_BASE + 1
        );
    }

    function test_revertSecondMigrationForSameKey() public {
        (, uint256 keyId) = _runFullKeygenCycle();
        _runFullMigrationCycle(keyId);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.CompressedKeyMaterialsAlreadyAdded.selector, keyId));
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.FromExisting, keyId);
    }

    function test_migrationAndKeygenDoNotInterleave() public {
        (, uint256 keyId) = _runFullKeygenCycle();

        // Migration pending blocks a new keygen (and a second migration).
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.FromExisting, keyId);
        uint256 migrationRequestId = KEY_COUNTER_BASE + 2;
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeygenOngoing.selector, migrationRequestId));
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeygenOngoing.selector, migrationRequestId));
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.FromExisting, keyId);
    }

    function test_scheduleCompressedKeyCutover() public {
        (, uint256 keyId) = _runFullKeygenCycle();

        IKMSGeneration.HostChainCutover[] memory cutovers = new IKMSGeneration.HostChainCutover[](2);
        cutovers[0] = IKMSGeneration.HostChainCutover({chainId: 1, cutoverBlock: 1000});
        cutovers[1] = IKMSGeneration.HostChainCutover({chainId: 2, cutoverBlock: 2000});

        // Scheduling requires published compressed materials.
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.CompressedKeyMaterialsNotAdded.selector, keyId));
        kmsGeneration.scheduleCompressedKeyCutover(keyId, cutovers, 500);

        _runFullMigrationCycle(keyId);

        vm.prank(owner);
        vm.expectEmit();
        emit IKMSGeneration.CompressedKeyCutoverScheduled(keyId, cutovers, 500);
        kmsGeneration.scheduleCompressedKeyCutover(keyId, cutovers, 500);

        (bool exists, IKMSGeneration.HostChainCutover[] memory stored, uint64 gatewayBlock) = kmsGeneration
            .getCompressedKeyCutoverSchedule(keyId);
        assertTrue(exists);
        assertEq(stored.length, 2);
        assertEq(stored[0].chainId, 1);
        assertEq(stored[0].cutoverBlock, 1000);
        assertEq(stored[1].chainId, 2);
        assertEq(stored[1].cutoverBlock, 2000);
        assertEq(gatewayBlock, 500);

        // Single-assignment, even for identical values.
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.CompressedKeyCutoverAlreadyScheduled.selector, keyId));
        kmsGeneration.scheduleCompressedKeyCutover(keyId, cutovers, 500);
    }

    function test_revertScheduleCutoverInvalidInputs() public {
        (, uint256 keyId) = _runFullKeygenCycle();
        _runFullMigrationCycle(keyId);

        // Empty host chain list.
        IKMSGeneration.HostChainCutover[] memory empty = new IKMSGeneration.HostChainCutover[](0);
        vm.prank(owner);
        vm.expectRevert(IKMSGeneration.EmptyHostChainCutovers.selector);
        kmsGeneration.scheduleCompressedKeyCutover(keyId, empty, 500);

        // Duplicate chain ID.
        IKMSGeneration.HostChainCutover[] memory dup = new IKMSGeneration.HostChainCutover[](2);
        dup[0] = IKMSGeneration.HostChainCutover({chainId: 1, cutoverBlock: 1000});
        dup[1] = IKMSGeneration.HostChainCutover({chainId: 1, cutoverBlock: 2000});
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.DuplicateCutoverChainId.selector, 1));
        kmsGeneration.scheduleCompressedKeyCutover(keyId, dup, 500);

        // Zero blocks.
        IKMSGeneration.HostChainCutover[] memory zero = new IKMSGeneration.HostChainCutover[](1);
        zero[0] = IKMSGeneration.HostChainCutover({chainId: 1, cutoverBlock: 0});
        vm.prank(owner);
        vm.expectRevert(IKMSGeneration.InvalidCutoverBlock.selector);
        kmsGeneration.scheduleCompressedKeyCutover(keyId, zero, 500);

        IKMSGeneration.HostChainCutover[] memory ok = new IKMSGeneration.HostChainCutover[](1);
        ok[0] = IKMSGeneration.HostChainCutover({chainId: 1, cutoverBlock: 1000});
        vm.prank(owner);
        vm.expectRevert(IKMSGeneration.InvalidCutoverBlock.selector);
        kmsGeneration.scheduleCompressedKeyCutover(keyId, ok, 0);
    }

    function testFuzz_revertMigrationEntrypointsNotOwner(address caller) public {
        vm.assume(caller != owner);
        (, uint256 keyId) = _runFullKeygenCycle();

        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, caller));
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.FromExisting, keyId);

        IKMSGeneration.HostChainCutover[] memory cutovers = new IKMSGeneration.HostChainCutover[](1);
        cutovers[0] = IKMSGeneration.HostChainCutover({chainId: 1, cutoverBlock: 1000});
        vm.prank(caller);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, caller));
        kmsGeneration.scheduleCompressedKeyCutover(keyId, cutovers, 500);
    }
    function test_genericKeyGettersRejectMigrationRequestId() public {
        (, uint256 keyId) = _runFullKeygenCycle();
        (, uint256 migrationRequestId) = _runFullMigrationCycle(keyId);

        // A migration request ID is a request handle, not a key: the
        // generic getters must revert instead of returning
        // plausible-looking empty records.
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeyNotGenerated.selector, migrationRequestId));
        kmsGeneration.getKeyMaterials(migrationRequestId);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeyNotGenerated.selector, migrationRequestId));
        kmsGeneration.getKeyInfo(migrationRequestId);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.KeyNotGenerated.selector, migrationRequestId));
        kmsGeneration.getKeyParamsType(migrationRequestId);

        // The real key stays fully readable.
        (, IKMSGeneration.KeyDigest[] memory digests) = kmsGeneration.getKeyMaterials(keyId);
        assertEq(digests.length, 1);
    }
    function test_revertMigrationKeygenForNonActiveKey() public {
        (, uint256 keyId1) = _runFullKeygenCycle();

        // Rotate: a second keygen makes keyId2 the active key.
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);
        uint256 prepKeygenId2 = PREP_KEYGEN_COUNTER_BASE + 2;
        uint256 keyId2 = KEY_COUNTER_BASE + 2;
        _doPrepKeygenResponse(prepKeygenId2, kmsPk0, kmsTxSender0);
        _doKeygenResponse(prepKeygenId2, keyId2, kmsPk0, kmsTxSender0);
        assertEq(kmsGeneration.getActiveKeyId(), keyId2);

        // RFC-029 migrates the ACTIVE key only.
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.NotActiveKey.selector, keyId1));
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.FromExisting, keyId1);
    }
    function test_revertScheduleCutoverForNoLongerActiveKey() public {
        (, uint256 keyIdA) = _runFullKeygenCycle();
        _runFullMigrationCycle(keyIdA);

        // A normal keygen activates key B after A's materials were published.
        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, IKMSGeneration.KeygenMode.Fresh, 0);
        uint256 prepKeygenIdB = PREP_KEYGEN_COUNTER_BASE + 3;
        uint256 keyIdB = KEY_COUNTER_BASE + 3;
        _doPrepKeygenResponse(prepKeygenIdB, kmsPk0, kmsTxSender0);
        _doKeygenResponse(prepKeygenIdB, keyIdB, kmsPk0, kmsTxSender0);
        assertEq(kmsGeneration.getActiveKeyId(), keyIdB);

        // Scheduling the dormant key must revert: a stale schedule could
        // steer coprocessors that load material for the live key.
        IKMSGeneration.HostChainCutover[] memory cutovers = new IKMSGeneration.HostChainCutover[](1);
        cutovers[0] = IKMSGeneration.HostChainCutover({chainId: 1, cutoverBlock: 1000});
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IKMSGeneration.NotActiveKey.selector, keyIdA));
        kmsGeneration.scheduleCompressedKeyCutover(keyIdA, cutovers, 500);
    }
}
