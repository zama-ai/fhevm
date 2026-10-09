// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Vm} from "forge-std/Test.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {KMSGeneration} from "@fhevm-host-contracts/contracts/KMSGeneration.sol";
import {IKMSGeneration} from "@fhevm-host-contracts/contracts/interfaces/IKMSGeneration.sol";
import {ProtocolConfigUpgradedExample} from "@fhevm-host-contracts/examples/ProtocolConfigUpgradedExample.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {IProtocolConfigBase} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigBase.sol";
import {KmsThresholds, KmsNode, KmsNodeParams, PcrValues, ChainUpgradeWindow} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {UUPSUpgradeableEmptyProxy} from "@fhevm-host-contracts/contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE, PREP_KEYGEN_COUNTER_BASE, KEY_COUNTER_BASE, CRS_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {protocolConfigAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";

contract ProtocolConfigTest is HostContractsDeployerTestUtils {
    KMSGeneration internal kmsGeneration;

    address internal constant owner = address(456);
    uint256 internal constant kmsPk0 = 0x100;
    uint256 internal constant kmsPk1 = 0x200;
    uint256 internal constant kmsPk2 = 0x300;
    uint256 internal constant kmsPk3 = 0x400;
    address internal kmsTxSender0 = address(0xA1);
    address internal kmsTxSender1 = address(0xA2);
    address internal kmsTxSender2 = address(0xA3);
    address internal kmsTxSender3 = address(0xA4);

    function _deployEmptyProtocolConfigProxy() internal {
        address emptyProxyImpl = address(new EmptyUUPSProxy());
        deployCodeTo(
            "fhevm-foundry/HostContractsDeployerTestUtils.sol:DeployableERC1967Proxy",
            abi.encode(emptyProxyImpl, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            protocolConfigAdd
        );
    }

    function _setupEmptyProxy() internal {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();
    }

    function _setupDefault() internal {
        _deployACL(owner);
        /// @dev Distinct per-field values so each getter proves it reads the correct storage slot.
        KmsThresholds memory thresholds = KmsThresholds({publicDecryption: 1, userDecryption: 2, kmsGen: 3, mpc: 4});
        (ProtocolConfig pc, ) = _deployProtocolConfig(owner, _makeKmsNodeParams(4), thresholds);
        protocolConfig = pc;
        (KMSGeneration kg, ) = _deployKMSGeneration(owner);
        kmsGeneration = kg;
    }

    function _setupDefaultWithMpcThreshold(uint256 mpcThreshold) internal {
        _deployACL(owner);
        KmsThresholds memory thresholds = KmsThresholds({
            publicDecryption: 1,
            userDecryption: 2,
            kmsGen: 3,
            mpc: mpcThreshold
        });
        (ProtocolConfig pc, ) = _deployProtocolConfig(owner, _makeKmsNodeParams(4), thresholds);
        protocolConfig = pc;
        (KMSGeneration kg, ) = _deployKMSGeneration(owner);
        kmsGeneration = kg;
    }

    function _setupEpochLifecycle() internal {
        _deployACL(owner);
        (ProtocolConfig pc, ) = _deployProtocolConfig(owner, _makeKmsNodeParams(2), _defaultThresholds());
        protocolConfig = pc;
        (KMSGeneration kg, ) = _deployKMSGeneration(owner);
        kmsGeneration = kg;
    }

    function _upgradeProxyExpectRevert(
        KmsNodeParams[] memory nodes,
        KmsThresholds memory thresholds,
        bytes memory expectedRevert
    ) internal {
        address impl = address(new ProtocolConfig());
        vm.prank(owner);
        vm.expectRevert(expectedRevert);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(ProtocolConfig.initializeFromEmptyProxy, (nodes, thresholds, "", new PcrValues[](0)))
        );
    }

    function _revertThreshold(KmsThresholds memory t, bytes memory expectedRevert) internal {
        _setupEmptyProxy();
        _upgradeProxyExpectRevert(_makeKmsNodeParams(1), t, expectedRevert);
    }

    function _computeKmsGenerationDomainSeparator() internal view returns (bytes32) {
        return
            keccak256(
                abi.encode(
                    EIP712_DOMAIN_TYPE_HASH,
                    keccak256(bytes("KMSGeneration")),
                    keccak256(bytes("1")),
                    block.chainid,
                    address(kmsGeneration)
                )
            );
    }

    function _hashKmsGenerationPrepKeygen(
        uint256 prepKeygenId,
        bytes memory extraData
    ) internal view returns (bytes32) {
        bytes32 structHash = keccak256(
            abi.encode(
                keccak256("PrepKeygenVerification(uint256 prepKeygenId,bytes extraData)"),
                prepKeygenId,
                keccak256(extraData)
            )
        );
        return MessageHashUtils.toTypedDataHash(_computeKmsGenerationDomainSeparator(), structHash);
    }

    function _hashKmsGenerationKeygen(
        uint256 prepKeygenId,
        uint256 keyId,
        IKMSGeneration.KeyDigest[] memory keyDigests,
        bytes memory extraData
    ) internal view returns (bytes32) {
        return
            _hashKeygenWithDomain(_computeKmsGenerationDomainSeparator(), prepKeygenId, keyId, keyDigests, extraData);
    }

    function _hashKmsGenerationCrsgen(
        uint256 crsId,
        uint256 maxBitLength,
        bytes memory crsDigest,
        bytes memory extraData
    ) internal view returns (bytes32) {
        return _hashCrsgenWithDomain(_computeKmsGenerationDomainSeparator(), crsId, maxBitLength, crsDigest, extraData);
    }

    function _defineNewKmsContextAndEpoch(
        KmsNodeParams[] memory nodes,
        KmsThresholds memory thresholds,
        string memory softwareVersion,
        PcrValues[] memory pcrValues
    ) internal {
        protocolConfig.defineNewKmsContextAndEpoch(nodes, thresholds, softwareVersion, pcrValues);
        nodeConfigHashes[protocolConfig.getCurrentKmsContextIdCounter()] = _nodeConfigHash(nodes, thresholds);
    }

    function _seedActiveEpochWithMaterialForTwoNodeContext() internal returns (uint256 epochId) {
        epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        (uint256 completedKeyId, uint256 completedCrsId) = _completeKmsGenerationMaterial();
        _confirmEpochActivation(KMS_CONTEXT_COUNTER_BASE + 1, epochId, kmsPk0, completedKeyId, completedCrsId);
        _confirmEpochActivation(KMS_CONTEXT_COUNTER_BASE + 1, epochId, kmsPk1, completedKeyId, completedCrsId);
    }

    function _confirmContextCreationWithTwoSigners(uint256 contextId) internal {
        _confirmContextCreation(contextId, kmsPk0, "");
        _confirmContextCreation(contextId, kmsPk1, "");
    }

    function _activatePendingContextWithOneKmsNode(uint256 contextId, uint256 epochId) internal {
        _confirmContextCreation(contextId, kmsPk0, "");
        _confirmEpochActivation(contextId, epochId, kmsPk0);
    }

    function _activatePendingContextWithTwoKmsNodes(uint256 contextId, uint256 epochId) internal {
        _confirmContextCreationWithTwoSigners(contextId);
        _confirmEpochActivation(contextId, epochId, kmsPk0);
        _confirmEpochActivation(contextId, epochId, kmsPk1);
    }

    function _completeKmsGenerationMaterial() internal returns (uint256 keyId, uint256 crsId) {
        return _completeKmsGenerationMaterial(kmsPk0, kmsTxSender0);
    }

    function _completeKmsGenerationMaterial(
        uint256 pk,
        address txSender
    ) internal returns (uint256 keyId, uint256 crsId) {
        (uint256 contextId, uint256 epochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        bytes memory extraData = abi.encodePacked(uint8(0x02), contextId, epochId);

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, 0);
        keyId = kmsGeneration.getKeyCounter();
        uint256 prepKeygenId = _prepKeygenIdForKeyId(keyId);

        vm.prank(txSender);
        kmsGeneration.prepKeygenResponse(
            prepKeygenId,
            _computeSignature(pk, _hashKmsGenerationPrepKeygen(prepKeygenId, extraData))
        );

        IKMSGeneration.KeyDigest[] memory keyDigests = _mockKeyDigests();
        vm.prank(txSender);
        kmsGeneration.keygenResponse(
            keyId,
            keyDigests,
            _computeSignature(pk, _hashKmsGenerationKeygen(prepKeygenId, keyId, keyDigests, extraData))
        );

        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);
        crsId = kmsGeneration.getCrsCounter();
        vm.prank(txSender);
        kmsGeneration.crsgenResponse(
            crsId,
            hex"deadbeef",
            _computeSignature(pk, _hashKmsGenerationCrsgen(crsId, 4096, hex"deadbeef", extraData))
        );
    }

    function _completeKmsGenerationMaterialWithTwoResponses(
        uint256 pk0,
        address txSender0,
        uint256 pk1,
        address txSender1
    ) internal returns (uint256 keyId, uint256 crsId) {
        (uint256 contextId, uint256 epochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        bytes memory extraData = abi.encodePacked(uint8(0x02), contextId, epochId);

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, 0);
        keyId = kmsGeneration.getKeyCounter();
        uint256 prepKeygenId = _prepKeygenIdForKeyId(keyId);

        vm.prank(txSender0);
        kmsGeneration.prepKeygenResponse(
            prepKeygenId,
            _computeSignature(pk0, _hashKmsGenerationPrepKeygen(prepKeygenId, extraData))
        );
        vm.prank(txSender1);
        kmsGeneration.prepKeygenResponse(
            prepKeygenId,
            _computeSignature(pk1, _hashKmsGenerationPrepKeygen(prepKeygenId, extraData))
        );

        IKMSGeneration.KeyDigest[] memory keyDigests = _mockKeyDigests();
        vm.prank(txSender0);
        kmsGeneration.keygenResponse(
            keyId,
            keyDigests,
            _computeSignature(pk0, _hashKmsGenerationKeygen(prepKeygenId, keyId, keyDigests, extraData))
        );
        vm.prank(txSender1);
        kmsGeneration.keygenResponse(
            keyId,
            keyDigests,
            _computeSignature(pk1, _hashKmsGenerationKeygen(prepKeygenId, keyId, keyDigests, extraData))
        );

        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);
        crsId = kmsGeneration.getCrsCounter();
        vm.prank(txSender0);
        kmsGeneration.crsgenResponse(
            crsId,
            hex"deadbeef",
            _computeSignature(pk0, _hashKmsGenerationCrsgen(crsId, 4096, hex"deadbeef", extraData))
        );
        vm.prank(txSender1);
        kmsGeneration.crsgenResponse(
            crsId,
            hex"deadbeef",
            _computeSignature(pk1, _hashKmsGenerationCrsgen(crsId, 4096, hex"deadbeef", extraData))
        );
    }

    function _completeKmsGenerationMaterialWithThreeResponses() internal returns (uint256 keyId, uint256 crsId) {
        (uint256 contextId, uint256 epochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        bytes memory extraData = abi.encodePacked(uint8(0x02), contextId, epochId);

        vm.prank(owner);
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default, 0);
        keyId = kmsGeneration.getKeyCounter();
        uint256 prepKeygenId = _prepKeygenIdForKeyId(keyId);

        vm.prank(kmsTxSender0);
        kmsGeneration.prepKeygenResponse(
            prepKeygenId,
            _computeSignature(kmsPk0, _hashKmsGenerationPrepKeygen(prepKeygenId, extraData))
        );
        vm.prank(kmsTxSender1);
        kmsGeneration.prepKeygenResponse(
            prepKeygenId,
            _computeSignature(kmsPk1, _hashKmsGenerationPrepKeygen(prepKeygenId, extraData))
        );
        vm.prank(kmsTxSender2);
        kmsGeneration.prepKeygenResponse(
            prepKeygenId,
            _computeSignature(kmsPk2, _hashKmsGenerationPrepKeygen(prepKeygenId, extraData))
        );
        vm.prank(kmsTxSender3);
        kmsGeneration.prepKeygenResponse(
            prepKeygenId,
            _computeSignature(kmsPk3, _hashKmsGenerationPrepKeygen(prepKeygenId, extraData))
        );

        IKMSGeneration.KeyDigest[] memory keyDigests = _mockKeyDigests();
        vm.prank(kmsTxSender0);
        kmsGeneration.keygenResponse(
            keyId,
            keyDigests,
            _computeSignature(kmsPk0, _hashKmsGenerationKeygen(prepKeygenId, keyId, keyDigests, extraData))
        );
        vm.prank(kmsTxSender1);
        kmsGeneration.keygenResponse(
            keyId,
            keyDigests,
            _computeSignature(kmsPk1, _hashKmsGenerationKeygen(prepKeygenId, keyId, keyDigests, extraData))
        );
        vm.prank(kmsTxSender2);
        kmsGeneration.keygenResponse(
            keyId,
            keyDigests,
            _computeSignature(kmsPk2, _hashKmsGenerationKeygen(prepKeygenId, keyId, keyDigests, extraData))
        );
        vm.prank(kmsTxSender3);
        kmsGeneration.keygenResponse(
            keyId,
            keyDigests,
            _computeSignature(kmsPk3, _hashKmsGenerationKeygen(prepKeygenId, keyId, keyDigests, extraData))
        );

        vm.prank(owner);
        kmsGeneration.crsgenRequest(4096, IKMSGeneration.ParamsType.Default);
        crsId = kmsGeneration.getCrsCounter();
        vm.prank(kmsTxSender0);
        kmsGeneration.crsgenResponse(
            crsId,
            hex"deadbeef",
            _computeSignature(kmsPk0, _hashKmsGenerationCrsgen(crsId, 4096, hex"deadbeef", extraData))
        );
        vm.prank(kmsTxSender1);
        kmsGeneration.crsgenResponse(
            crsId,
            hex"deadbeef",
            _computeSignature(kmsPk1, _hashKmsGenerationCrsgen(crsId, 4096, hex"deadbeef", extraData))
        );
        vm.prank(kmsTxSender2);
        kmsGeneration.crsgenResponse(
            crsId,
            hex"deadbeef",
            _computeSignature(kmsPk2, _hashKmsGenerationCrsgen(crsId, 4096, hex"deadbeef", extraData))
        );
        vm.prank(kmsTxSender3);
        kmsGeneration.crsgenResponse(
            crsId,
            hex"deadbeef",
            _computeSignature(kmsPk3, _hashKmsGenerationCrsgen(crsId, 4096, hex"deadbeef", extraData))
        );
    }

    function _seedActiveEpochWithMaterialForFourNodeContext() internal returns (uint256 epochId) {
        epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        (uint256 completedKeyId, uint256 completedCrsId) = _completeKmsGenerationMaterialWithThreeResponses();
        _confirmEpochActivation(KMS_CONTEXT_COUNTER_BASE + 1, epochId, kmsPk0, completedKeyId, completedCrsId);
        _confirmEpochActivation(KMS_CONTEXT_COUNTER_BASE + 1, epochId, kmsPk1, completedKeyId, completedCrsId);
        _confirmEpochActivation(KMS_CONTEXT_COUNTER_BASE + 1, epochId, kmsPk2, completedKeyId, completedCrsId);
        _confirmEpochActivation(KMS_CONTEXT_COUNTER_BASE + 1, epochId, kmsPk3, completedKeyId, completedCrsId);
    }

    /// @dev Asserts the liveness-guarded context view functions revert for the given context ID.
    ///      getKmsNodeForContext is existence-guarded (readable after destroy), so callers assert it separately.
    function _expectContextGuardedViewsRevert(uint256 contextId) internal {
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, contextId));
        protocolConfig.getKmsSignersForContext(contextId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, contextId));
        protocolConfig.isKmsSignerForContext(contextId, address(0xDEAD));

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, contextId));
        protocolConfig.getKmsNodesForContext(contextId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, contextId));
        protocolConfig.isKmsTxSenderForContext(contextId, address(0xDEAD));

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, contextId));
        protocolConfig.getUserDecryptionThresholdForContext(contextId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, contextId));
        protocolConfig.getPublicDecryptionThresholdForContext(contextId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, contextId));
        protocolConfig.getKmsGenThresholdForContext(contextId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, contextId));
        protocolConfig.getMpcThresholdForContext(contextId);
    }

    // -----------------------------------------------------------------------
    // Init tests
    // -----------------------------------------------------------------------

    function test_initSuccess() public {
        _setupDefault();

        // Version and current context.
        assertEq(protocolConfig.getVersion(), "ProtocolConfig v0.4.0");
        uint256 contextId = protocolConfig.getCurrentKmsContextId();
        assertEq(contextId, KMS_CONTEXT_COUNTER_BASE + 1);
        assertEq(protocolConfig.getCurrentKmsContextId(), contextId);
        (uint256 activeContextId, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, contextId);
        assertEq(activeEpochId, EPOCH_COUNTER_BASE + 1);
        assertTrue(protocolConfig.isValidKmsContext(contextId));

        // Thresholds.
        assertEq(protocolConfig.getPublicDecryptionThreshold(), 1);
        assertEq(protocolConfig.getUserDecryptionThreshold(), 2);
        assertEq(protocolConfig.getKmsGenThreshold(), 3);
        assertEq(protocolConfig.getKmsGenThresholdForContext(contextId), 3);
        assertEq(protocolConfig.getMpcThreshold(), 4);

        // Context node arrays and registered signer/tx sender mappings.
        KmsNode[] memory expectedNodes = _makeKmsNodes(4);
        KmsNode[] memory nodes = protocolConfig.getKmsNodesForContext(contextId);
        address[] memory signers = protocolConfig.getKmsSignersForContext(contextId);
        assertEq(nodes.length, expectedNodes.length);
        assertEq(signers.length, expectedNodes.length);
        for (uint256 i = 0; i < expectedNodes.length; i++) {
            assertEq(nodes[i].txSenderAddress, expectedNodes[i].txSenderAddress);
            assertEq(nodes[i].signerAddress, expectedNodes[i].signerAddress);
            assertEq(nodes[i].ipAddress, expectedNodes[i].ipAddress);
            assertEq(nodes[i].storageUrl, expectedNodes[i].storageUrl);
            assertEq(signers[i], expectedNodes[i].signerAddress);
            assertTrue(protocolConfig.isKmsSignerForContext(contextId, expectedNodes[i].signerAddress));
            assertTrue(protocolConfig.isKmsTxSenderForContext(contextId, expectedNodes[i].txSenderAddress));

            // Direct node lookup by tx sender.
            KmsNode memory node = protocolConfig.getKmsNodeForContext(contextId, expectedNodes[i].txSenderAddress);
            assertEq(node.txSenderAddress, expectedNodes[i].txSenderAddress);
            assertEq(node.signerAddress, expectedNodes[i].signerAddress);
            assertEq(node.ipAddress, expectedNodes[i].ipAddress);
            assertEq(node.storageUrl, expectedNodes[i].storageUrl);
        }
        // Negative: unregistered addresses must return false.
        assertFalse(protocolConfig.isKmsSignerForContext(contextId, address(0xDEAD)));
        assertFalse(protocolConfig.isKmsTxSenderForContext(contextId, address(0xDEAD)));
    }

    function test_storageLocationPinned() public {
        _setupDefault();
        /// @dev A pending context makes the counter differ from the latest active context ID.
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(1), _defaultThresholds());

        bytes32 slot = 0x80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c200;
        uint256 stored = uint256(vm.load(address(protocolConfig), slot));
        assertEq(stored, protocolConfig.getCurrentKmsContextIdCounter());
        assertEq(stored, KMS_CONTEXT_COUNTER_BASE + 2);
    }

    function test_canonicalStorageLocationPinned() public {
        _setupDefault();
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(1), _defaultThresholds());

        /// @dev nodeConfigHashForContext is the first field of the canonical namespace.
        bytes32 location = keccak256(abi.encode(uint256(keccak256("fhevm.storage.ProtocolConfigCanonical")) - 1)) &
            ~bytes32(uint256(0xff));
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 2;
        bytes32 slot = keccak256(abi.encode(contextId, location));
        assertEq(vm.load(address(protocolConfig), slot), nodeConfigHashes[contextId]);
    }

    // -----------------------------------------------------------------------
    // Validation error tests
    // -----------------------------------------------------------------------

    function test_revertEmptyNodes() public {
        _setupEmptyProxy();
        KmsNodeParams[] memory emptyNodes = new KmsNodeParams[](0);
        _upgradeProxyExpectRevert(
            emptyNodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfigBase.EmptyKmsNodes.selector)
        );
    }

    function test_revertNullTxSender() public {
        _setupEmptyProxy();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(1);
        nodes[0].txSenderAddress = address(0);
        _upgradeProxyExpectRevert(
            nodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfigBase.KmsNodeNullTxSender.selector)
        );
    }

    function test_revertNullSigner() public {
        _setupEmptyProxy();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(1);
        nodes[0].signerAddress = address(0);
        _upgradeProxyExpectRevert(
            nodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfigBase.KmsNodeNullSigner.selector)
        );
    }

    function test_revertDuplicateTxSender() public {
        _setupEmptyProxy();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[1].txSenderAddress = nodes[0].txSenderAddress;
        _upgradeProxyExpectRevert(
            nodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfigBase.KmsTxSenderAlreadyRegistered.selector, nodes[0].txSenderAddress)
        );
    }

    function test_revertDuplicateSigner() public {
        _setupEmptyProxy();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[1].signerAddress = nodes[0].signerAddress;
        _upgradeProxyExpectRevert(
            nodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfigBase.KmsSignerAlreadyRegistered.selector, nodes[0].signerAddress)
        );
    }

    function test_revertNullPublicDecryptionThreshold() public {
        KmsThresholds memory t = _defaultThresholds();
        t.publicDecryption = 0;
        _revertThreshold(
            t,
            abi.encodeWithSelector(IProtocolConfigBase.InvalidNullThreshold.selector, "publicDecryption")
        );
    }

    function test_revertHighPublicDecryptionThreshold() public {
        KmsThresholds memory t = _defaultThresholds();
        t.publicDecryption = 5;
        _revertThreshold(
            t,
            abi.encodeWithSelector(IProtocolConfigBase.InvalidHighThreshold.selector, "publicDecryption", 5, 1)
        );
    }

    function test_revertNullUserDecryptionThreshold() public {
        KmsThresholds memory t = _defaultThresholds();
        t.userDecryption = 0;
        _revertThreshold(
            t,
            abi.encodeWithSelector(IProtocolConfigBase.InvalidNullThreshold.selector, "userDecryption")
        );
    }

    function test_revertHighUserDecryptionThreshold() public {
        KmsThresholds memory t = _defaultThresholds();
        t.userDecryption = 5;
        _revertThreshold(
            t,
            abi.encodeWithSelector(IProtocolConfigBase.InvalidHighThreshold.selector, "userDecryption", 5, 1)
        );
    }

    function test_revertNullKmsGenThreshold() public {
        KmsThresholds memory t = _defaultThresholds();
        t.kmsGen = 0;
        _revertThreshold(t, abi.encodeWithSelector(IProtocolConfigBase.InvalidNullThreshold.selector, "kmsGen"));
    }

    function test_revertHighKmsGenThreshold() public {
        KmsThresholds memory t = _defaultThresholds();
        t.kmsGen = 5;
        _revertThreshold(t, abi.encodeWithSelector(IProtocolConfigBase.InvalidHighThreshold.selector, "kmsGen", 5, 1));
    }

    function test_revertNullMpcThreshold() public {
        KmsThresholds memory t = _defaultThresholds();
        t.mpc = 0;
        _revertThreshold(t, abi.encodeWithSelector(IProtocolConfigBase.InvalidNullThreshold.selector, "mpc"));
    }

    function test_revertHighMpcThreshold() public {
        KmsThresholds memory t = _defaultThresholds();
        t.mpc = 5;
        _revertThreshold(t, abi.encodeWithSelector(IProtocolConfigBase.InvalidHighThreshold.selector, "mpc", 5, 1));
    }

    function test_revertSignerSetExceedsProofFormatLimit() public {
        _setupEmptyProxy();
        KmsNodeParams[] memory tooManyNodes = _makeKmsNodeParams(256);
        _upgradeProxyExpectRevert(
            tooManyNodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfigBase.KmsSignerSetExceedsProofFormatLimit.selector, 256, 255)
        );
    }

    // -----------------------------------------------------------------------
    // Context lifecycle tests
    // -----------------------------------------------------------------------

    function test_defineNewKmsContextAndEpochCreatesPendingContext() public {
        _setupDefault();

        KmsNodeParams[] memory newNodeParams = _makeKmsNodeParams(1);
        PcrValues[] memory pcrValues = new PcrValues[](0);

        KmsThresholds memory thresholds = _defaultThresholds();
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.NewKmsContext(
            KMS_CONTEXT_COUNTER_BASE + 2,
            KMS_CONTEXT_COUNTER_BASE + 1,
            newNodeParams,
            thresholds,
            "",
            pcrValues
        );
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(newNodeParams, thresholds);

        assertEq(protocolConfig.getCurrentKmsContextId(), KMS_CONTEXT_COUNTER_BASE + 1);
        assertFalse(protocolConfig.isValidKmsContext(KMS_CONTEXT_COUNTER_BASE + 2));
    }

    function test_defineNewKmsContextAndEpochStoresContextAnchor() public {
        _setupDefault();

        KmsNodeParams[] memory params = _makeKmsNodeParams(2);
        KmsThresholds memory thresholds = _defaultThresholds();
        PcrValues[] memory pcrValues = new PcrValues[](1);
        pcrValues[0] = PcrValues({
            pcr0: abi.encodePacked(uint256(1)),
            pcr1: abi.encodePacked(uint256(2)),
            pcr2: abi.encodePacked(uint256(3))
        });

        uint256 expectedBlock = block.number;
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(params, thresholds, "kms-v1", pcrValues);

        (uint256 emissionBlockNumber, bytes32 contextInfoHash) = protocolConfig.getKmsContextAnchor(
            KMS_CONTEXT_COUNTER_BASE + 2
        );
        assertEq(emissionBlockNumber, expectedBlock);
        assertEq(contextInfoHash, keccak256(abi.encode(params, thresholds, "kms-v1", pcrValues)));
    }

    function test_historicalContextReadable() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        _seedActiveEpochWithMaterialForFourNodeContext();

        KmsNodeParams[] memory newNodes = _makeKmsNodeParams(1);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(newNodes, _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 newEpochId = EPOCH_COUNTER_BASE + 3;
        _activatePendingContextWithOneKmsNode(newContextId, newEpochId);

        uint256 currentId = protocolConfig.getCurrentKmsContextId();
        assertTrue(currentId != firstContextId);
        assertTrue(protocolConfig.isValidKmsContext(firstContextId));
        address[] memory oldSigners = protocolConfig.getKmsSignersForContext(firstContextId);
        assertEq(oldSigners.length, 4);
    }

    function test_destroyContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        (uint256 firstEmissionBlockNumber, bytes32 firstContextInfoHash) = protocolConfig.getKmsContextAnchor(
            firstContextId
        );
        _seedActiveEpochWithMaterialForFourNodeContext();

        KmsNodeParams[] memory newNodes = _makeKmsNodeParams(1);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(newNodes, _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _activatePendingContextWithOneKmsNode(newContextId, EPOCH_COUNTER_BASE + 3);

        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextDestroyed(firstContextId);
        vm.prank(owner);
        protocolConfig.destroyKmsContext(firstContextId);
        assertFalse(protocolConfig.isValidKmsContext(firstContextId));
        (uint256 destroyedEmissionBlockNumber, bytes32 destroyedContextInfoHash) = protocolConfig.getKmsContextAnchor(
            firstContextId
        );
        assertEq(destroyedEmissionBlockNumber, firstEmissionBlockNumber);
        assertEq(destroyedContextInfoHash, firstContextInfoHash);
    }

    function test_revertDestroyCurrentContext() public {
        _setupDefault();
        uint256 currentId = protocolConfig.getCurrentKmsContextId();
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.LatestActiveKmsContextCannotBeDestroyed.selector, currentId)
        );
        protocolConfig.destroyKmsContext(currentId);
    }

    function testFuzz_revertDestroyInvalidContext(uint256 invalidContextId) public {
        _setupDefault();
        vm.assume(invalidContextId != protocolConfig.getCurrentKmsContextId());
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, invalidContextId));
        protocolConfig.destroyKmsContext(invalidContextId);
    }

    function test_revertDestroyAlreadyDestroyedContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        _seedActiveEpochWithMaterialForFourNodeContext();

        KmsNodeParams[] memory newNodes = _makeKmsNodeParams(1);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(newNodes, _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _activatePendingContextWithOneKmsNode(newContextId, EPOCH_COUNTER_BASE + 3);

        vm.prank(owner);
        protocolConfig.destroyKmsContext(firstContextId);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, firstContextId));
        protocolConfig.destroyKmsContext(firstContextId);
    }

    function test_defineNewEpochForCurrentKmsContextDoesNotActivateImmediately() public {
        _setupEpochLifecycle();
        (uint256 contextId, uint256 epochId) = protocolConfig.getCurrentKmsContextAndEpoch();

        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        (uint256 currentContextId, uint256 currentEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(currentContextId, contextId);
        assertEq(currentEpochId, epochId);
    }

    function test_revertDefineNewEpochForCurrentKmsContextNotOwner() public {
        _setupEpochLifecycle();
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.defineNewEpochForCurrentKmsContext();
    }

    function test_fullSameSetResharingFlow() public {
        _setupEpochLifecycle();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;

        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        (uint256 completedKeyId, uint256 completedCrsId) = _completeKmsGenerationMaterial();

        _confirmEpochActivation(KMS_CONTEXT_COUNTER_BASE + 1, epochId, kmsPk0, completedKeyId, completedCrsId);
        (, uint256 activeEpochBeforeSecondConfirmation) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochBeforeSecondConfirmation, EPOCH_COUNTER_BASE + 1);

        _confirmEpochActivation(KMS_CONTEXT_COUNTER_BASE + 1, epochId, kmsPk1, completedKeyId, completedCrsId);

        (uint256 contextId, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(contextId, KMS_CONTEXT_COUNTER_BASE + 1);
        assertEq(activeEpochId, epochId);
    }

    function test_isValidEpochForContext_trueOnFreshDeploy() public {
        _setupEpochLifecycle();
        assertTrue(protocolConfig.isValidEpochForContext(KMS_CONTEXT_COUNTER_BASE + 1, EPOCH_COUNTER_BASE + 1));
    }

    function test_isValidEpochForContext_falseForWrongContextId() public {
        _setupEpochLifecycle();
        // Active epoch exists, but paired with the wrong context.
        assertFalse(protocolConfig.isValidEpochForContext(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 1));
    }

    function test_isValidEpochForContext_falseForUnknownEpoch() public {
        _setupEpochLifecycle();
        assertFalse(protocolConfig.isValidEpochForContext(KMS_CONTEXT_COUNTER_BASE + 1, 0));
        assertFalse(protocolConfig.isValidEpochForContext(KMS_CONTEXT_COUNTER_BASE + 1, EPOCH_COUNTER_BASE + 999));
    }

    function test_isValidEpochForContext_falseForPendingSameSetEpoch() public {
        _setupEpochLifecycle();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 pendingEpochId = EPOCH_COUNTER_BASE + 2;

        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        assertFalse(protocolConfig.isValidEpochForContext(contextId, pendingEpochId));
        // Previous active epoch still passes, the new one is Pending until activated.
        assertTrue(protocolConfig.isValidEpochForContext(contextId, EPOCH_COUNTER_BASE + 1));
    }

    function test_isValidEpochForContext_trueAfterSameSetActivation() public {
        _setupEpochLifecycle();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 newEpochId = _seedActiveEpochWithMaterialForTwoNodeContext();

        assertTrue(protocolConfig.isValidEpochForContext(contextId, newEpochId));
    }

    function test_isValidEpochForContext_falseForPendingEpochUnderPendingContext() public {
        _setupEpochLifecycle();
        uint256 pendingContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 pendingEpochId = EPOCH_COUNTER_BASE + 2;

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());

        assertFalse(protocolConfig.isValidEpochForContext(pendingContextId, pendingEpochId));
    }

    function test_isValidEpochForContext_trueAfterContextSwitchActivation() public {
        _setupEpochLifecycle();
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 newEpochId = EPOCH_COUNTER_BASE + 2;

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        _activatePendingContextWithTwoKmsNodes(newContextId, newEpochId);

        assertTrue(protocolConfig.isValidEpochForContext(newContextId, newEpochId));
    }

    function test_isValidEpochForContext_oldPairStillTrueAfterContextSwitch() public {
        _setupEpochLifecycle();
        uint256 oldContextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 oldEpochId = EPOCH_COUNTER_BASE + 1;
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 newEpochId = EPOCH_COUNTER_BASE + 2;

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        _activatePendingContextWithTwoKmsNodes(newContextId, newEpochId);

        assertTrue(protocolConfig.isValidEpochForContext(oldContextId, oldEpochId));
    }

    function test_isValidEpochForContext_falseAfterContextDestroyed() public {
        _setupEpochLifecycle();
        uint256 oldContextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 oldEpochId = EPOCH_COUNTER_BASE + 1;
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 newEpochId = EPOCH_COUNTER_BASE + 2;

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        _activatePendingContextWithTwoKmsNodes(newContextId, newEpochId);

        // The old context's epoch stays Active after rotation, so the pair is still valid
        assertTrue(protocolConfig.isValidEpochForContext(oldContextId, oldEpochId));

        // but destroying the old context must invalidate its epoch too.
        vm.prank(owner);
        protocolConfig.destroyKmsContext(oldContextId);

        assertFalse(protocolConfig.isValidEpochForContext(oldContextId, oldEpochId));
    }

    function test_destroyEpoch() public {
        _setupEpochLifecycle();
        uint256 oldContextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 oldEpochId = EPOCH_COUNTER_BASE + 1;
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 newEpochId = EPOCH_COUNTER_BASE + 2;

        // Switch to a newer context+epoch so the old epoch is a superseded (non-current) Active epoch.
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        _activatePendingContextWithTwoKmsNodes(newContextId, newEpochId);
        assertTrue(protocolConfig.isValidEpochForContext(oldContextId, oldEpochId));

        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsEpochDestroyed(oldEpochId);
        vm.prank(owner);
        protocolConfig.destroyKmsEpoch(oldEpochId);

        assertFalse(protocolConfig.isValidEpochForContext(oldContextId, oldEpochId));
    }

    function test_revertDestroyCurrentEpoch() public {
        _setupEpochLifecycle();
        (, uint256 currentEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.LatestActiveKmsEpochCannotBeDestroyed.selector, currentEpochId)
        );
        protocolConfig.destroyKmsEpoch(currentEpochId);
    }

    function testFuzz_revertDestroyInvalidKmsEpoch(uint256 invalidEpochId) public {
        _setupEpochLifecycle();
        (, uint256 currentEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        vm.assume(invalidEpochId != currentEpochId);
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, invalidEpochId));
        protocolConfig.destroyKmsEpoch(invalidEpochId);
    }

    function test_destroyPendingEpochOfActiveContext() public {
        _setupEpochLifecycle();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 pendingEpochId = EPOCH_COUNTER_BASE + 2;
        (, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();

        // A Pending epoch under the Active context (same-set rotation) is abortable.
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsEpochDestroyed(pendingEpochId);
        vm.prank(owner);
        protocolConfig.destroyKmsEpoch(pendingEpochId);

        assertFalse(protocolConfig.isValidEpochForContext(contextId, pendingEpochId));
        (, uint256 epochAfter) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(epochAfter, activeEpochId);

        // Governance can re-trigger the rotation and complete it.
        uint256 retriedEpochId = EPOCH_COUNTER_BASE + 3;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        _confirmEpochActivation(contextId, retriedEpochId, kmsPk0);
        _confirmEpochActivation(contextId, retriedEpochId, kmsPk1);
        (, uint256 epochFinal) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(epochFinal, retriedEpochId);
    }

    function test_destroyPendingEpochAfterDivergentVotes() public {
        _setupEpochLifecycle();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 pendingEpochId = EPOCH_COUNTER_BASE + 2;

        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        (uint256 completedKeyId, uint256 completedCrsId) = _completeKmsGenerationMaterial();

        // Divergent votes: the two signers attest to different key ids, so no result reaches quorum.
        _confirmEpochActivation(contextId, pendingEpochId, kmsPk0, completedKeyId, completedCrsId);
        _confirmEpochActivation(contextId, pendingEpochId, kmsPk1, completedKeyId + 1, completedCrsId);

        // Confirmations are one-shot per signer, so the split vote can never converge: even coming back
        // with the other signer's result is rejected.
        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _buildEpochResults(contextId, pendingEpochId, kmsPk0, completedKeyId + 1, completedCrsId);
        bytes memory signature = _signEpochActivation(contextId, pendingEpochId, kmsPk0, keys, crsList, "");
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.EpochActivationAlreadyConfirmed.selector,
                vm.addr(kmsPk0),
                pendingEpochId
            )
        );
        protocolConfig.confirmEpochActivation(pendingEpochId, keys, crsList, signature, "");
        (, uint256 epochBefore) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(epochBefore, EPOCH_COUNTER_BASE + 1);

        // Governance aborts the stuck epoch and re-runs the rotation to completion.
        vm.prank(owner);
        protocolConfig.destroyKmsEpoch(pendingEpochId);

        uint256 retriedEpochId = EPOCH_COUNTER_BASE + 3;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        _confirmEpochActivation(contextId, retriedEpochId, kmsPk0, completedKeyId, completedCrsId);
        _confirmEpochActivation(contextId, retriedEpochId, kmsPk1, completedKeyId, completedCrsId);
        (, uint256 epochFinal) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(epochFinal, retriedEpochId);
    }

    function test_revertDestroyPendingEpochOfPendingContext() public {
        _setupEpochLifecycle();
        uint256 pendingEpochId = EPOCH_COUNTER_BASE + 2;

        // A pending context switch is settled by destroyKmsContext, not destroyKmsEpoch.
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, pendingEpochId));
        protocolConfig.destroyKmsEpoch(pendingEpochId);
    }

    function test_destroyPendingContextClearsPairedEpoch() public {
        _setupEpochLifecycle();
        uint256 pendingContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 pendingEpochId = EPOCH_COUNTER_BASE + 2;
        (uint256 activeContextId, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();

        // A pending context switch is never stuck: destroying the pending context settles the
        // whole pair, clearing the paired pending epoch with it.
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());

        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextDestroyed(pendingContextId);
        vm.prank(owner);
        protocolConfig.destroyKmsContext(pendingContextId);

        assertFalse(protocolConfig.isValidKmsContext(pendingContextId));
        assertFalse(protocolConfig.isValidEpochForContext(pendingContextId, pendingEpochId));
        (uint256 contextAfter, uint256 epochAfter) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(contextAfter, activeContextId);
        assertEq(epochAfter, activeEpochId);
    }

    function test_revertDestroyEpochNotOwner() public {
        _setupEpochLifecycle();
        (, uint256 currentEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.destroyKmsEpoch(currentEpochId);
    }

    function test_revertDestroyAlreadyDestroyedEpoch() public {
        _setupEpochLifecycle();
        uint256 oldEpochId = EPOCH_COUNTER_BASE + 1;
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 newEpochId = EPOCH_COUNTER_BASE + 2;

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        _activatePendingContextWithTwoKmsNodes(newContextId, newEpochId);

        vm.prank(owner);
        protocolConfig.destroyKmsEpoch(oldEpochId);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, oldEpochId));
        protocolConfig.destroyKmsEpoch(oldEpochId);
    }

    function test_defineNewKmsContextAndEpochDoesNotActivateImmediately() public {
        _setupEpochLifecycle();
        (uint256 oldContextId, uint256 oldEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[0].signerAddress = vm.addr(kmsPk2);
        nodes[1].signerAddress = vm.addr(kmsPk3);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());

        assertEq(protocolConfig.getCurrentKmsContextId(), oldContextId);
        (uint256 currentContextId, uint256 currentEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(currentContextId, oldContextId);
        assertEq(currentEpochId, oldEpochId);
        assertFalse(protocolConfig.isValidKmsContext(KMS_CONTEXT_COUNTER_BASE + 2));
        assertTrue(protocolConfig.isKmsTxSenderForContext(KMS_CONTEXT_COUNTER_BASE + 2, kmsTxSender0));
        assertEq(protocolConfig.getKmsGenThresholdForContext(KMS_CONTEXT_COUNTER_BASE + 2), 1);
    }

    /// @dev Previous-side quorum boundary, both sides: with n = 4 previous nodes and t = 2, the
    ///      target is n - t = 2. All-new + n - t - 1 previous confirmations stay Pending, the
    ///      (n - t)-th completes creation (NewKmsEpoch), and any later one hits the Created state.
    function test_confirmKmsContextCreationUsesNewSignersAndOldQuorum() public {
        _setupDefaultWithMpcThreshold(2);
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[0].txSenderAddress = address(0xC1);
        nodes[0].signerAddress = vm.addr(0xB2);
        nodes[1].txSenderAddress = address(0xC2);
        nodes[1].signerAddress = vm.addr(0xB3);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        _confirmContextCreation(newContextId, 0xB2, "");
        _confirmContextCreation(newContextId, 0xB3, "");
        _confirmContextCreation(newContextId, kmsPk0, "");
        assertFalse(protocolConfig.isValidKmsContext(newContextId));

        // Full-args assertion (indexed kmsContextId/epochId + data) on the quorum-completing event.
        bytes memory signature = _signContextCreation(newContextId, kmsPk1, "");
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.NewKmsEpoch(
            newContextId,
            EPOCH_COUNTER_BASE + 2,
            KMS_CONTEXT_COUNTER_BASE + 1,
            EPOCH_COUNTER_BASE + 1,
            block.number - 1
        );
        protocolConfig.confirmKmsContextCreation(newContextId, signature, "");

        // The context left Pending on the (n - t)-th previous confirmation; a further one is rejected.
        signature = _signContextCreation(newContextId, kmsPk2, "");
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.KmsContextNotPending.selector, newContextId));
        protocolConfig.confirmKmsContextCreation(newContextId, signature, "");
    }

    function test_confirmKmsContextCreationRequiresAllNewSigners() public {
        _setupDefaultWithMpcThreshold(3);
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[0].txSenderAddress = address(0xC1);
        nodes[0].signerAddress = vm.addr(0xB2);
        nodes[1].txSenderAddress = address(0xC2);
        nodes[1].signerAddress = vm.addr(0xB3);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        _confirmContextCreation(newContextId, 0xB2, "");
        _confirmContextCreation(newContextId, kmsPk0, "");
        _confirmContextCreation(newContextId, kmsPk1, "");
        assertFalse(protocolConfig.isValidKmsContext(newContextId));

        // The quorum-completing confirmation (all new signers present) must emit NewKmsEpoch with the
        // pending epoch's full indexed args. Before this last confirmation no such event was emitted.
        bytes memory signature = _signContextCreation(newContextId, 0xB3, "");
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.NewKmsEpoch(
            newContextId,
            EPOCH_COUNTER_BASE + 2,
            KMS_CONTEXT_COUNTER_BASE + 1,
            EPOCH_COUNTER_BASE + 1,
            block.number - 1
        );
        protocolConfig.confirmKmsContextCreation(newContextId, signature, "");
    }

    function test_revertConfirmEpochActivationBeforeCreateKmsContext() public {
        _setupEpochLifecycle();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[0].signerAddress = vm.addr(kmsPk2);
        nodes[1].signerAddress = vm.addr(kmsPk3);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());

        // The switch's epoch is only created once creation is confirmed, so activating the
        // would-be epoch ID before that reverts as an unknown epoch.
        uint256 newEpochId = EPOCH_COUNTER_BASE + 2;
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, newEpochId));
        IProtocolConfig.EpochKeyResult[] memory keys = new IProtocolConfig.EpochKeyResult[](0);
        IProtocolConfig.EpochCrsResult[] memory crsList = new IProtocolConfig.EpochCrsResult[](0);
        protocolConfig.confirmEpochActivation(newEpochId, keys, crsList, "", "");
    }

    function test_destroyCreatedContext() public {
        _setupEpochLifecycle();

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        uint256 createdContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _confirmContextCreationWithTwoSigners(createdContextId);

        vm.prank(owner);
        protocolConfig.destroyKmsContext(createdContextId);
        assertFalse(protocolConfig.isValidKmsContext(createdContextId));
    }

    function test_revertDefineSecondContextSwitchWhileSwitchPending() public {
        _setupEpochLifecycle();

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsLifecycleOperationInFlight.selector,
                KMS_CONTEXT_COUNTER_BASE + 2,
                EPOCH_COUNTER_BASE + 1
            )
        );
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
    }

    function test_revertDefineSecondContextSwitchWhileSwitchCreated() public {
        _setupEpochLifecycle();

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        _confirmContextCreationWithTwoSigners(KMS_CONTEXT_COUNTER_BASE + 2);

        // The Created context and the epoch created at its confirmation are both still settling.
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsLifecycleOperationInFlight.selector,
                KMS_CONTEXT_COUNTER_BASE + 2,
                EPOCH_COUNTER_BASE + 2
            )
        );
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
    }

    function test_revertDefineNewEpochWhileContextSwitchPending() public {
        _setupEpochLifecycle();

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsLifecycleOperationInFlight.selector,
                KMS_CONTEXT_COUNTER_BASE + 2,
                EPOCH_COUNTER_BASE + 1
            )
        );
        protocolConfig.defineNewEpochForCurrentKmsContext();
    }

    function test_revertDefineContextSwitchWhileReshareEpochPending() public {
        _setupEpochLifecycle();

        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsLifecycleOperationInFlight.selector,
                KMS_CONTEXT_COUNTER_BASE + 1,
                EPOCH_COUNTER_BASE + 2
            )
        );
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
    }

    function test_revertDefineSecondReshareEpochWhileReshareEpochPending() public {
        _setupEpochLifecycle();

        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsLifecycleOperationInFlight.selector,
                KMS_CONTEXT_COUNTER_BASE + 1,
                EPOCH_COUNTER_BASE + 2
            )
        );
        protocolConfig.defineNewEpochForCurrentKmsContext();
    }

    function test_defineNewContextSwitchAfterDestroyingPendingSwitch() public {
        _setupEpochLifecycle();

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());

        // Destroying the pending switch must reopen the gate: destroyed entries are None,
        // not in flight.
        vm.prank(owner);
        protocolConfig.destroyKmsContext(KMS_CONTEXT_COUNTER_BASE + 2);

        // The active pair is untouched: no epoch existed for the destroyed switch yet.
        (uint256 activeContextId, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, KMS_CONTEXT_COUNTER_BASE + 1);
        assertEq(activeEpochId, EPOCH_COUNTER_BASE + 1);
        assertTrue(protocolConfig.isValidEpochForContext(KMS_CONTEXT_COUNTER_BASE + 1, EPOCH_COUNTER_BASE + 1));

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        (uint256 emissionBlockNumber, ) = protocolConfig.getKmsContextAnchor(KMS_CONTEXT_COUNTER_BASE + 3);
        assertEq(emissionBlockNumber, block.number);
    }

    function test_defineNewContextSwitchAfterDestroyingCreatedSwitch() public {
        _setupEpochLifecycle();

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        _confirmContextCreationWithTwoSigners(KMS_CONTEXT_COUNTER_BASE + 2);

        // Destroying the created switch clears the epoch created at its confirmation and reopens the gate.
        vm.prank(owner);
        protocolConfig.destroyKmsContext(KMS_CONTEXT_COUNTER_BASE + 2);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        (uint256 emissionBlockNumber, ) = protocolConfig.getKmsContextAnchor(KMS_CONTEXT_COUNTER_BASE + 3);
        assertEq(emissionBlockNumber, block.number);
    }

    function test_fullContextSwitchFlow() public {
        _setupEpochLifecycle();
        _seedActiveEpochWithMaterialForTwoNodeContext();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[0].txSenderAddress = address(0xC1);
        nodes[0].signerAddress = vm.addr(kmsPk2);
        nodes[1].txSenderAddress = address(0xC2);
        nodes[1].signerAddress = vm.addr(kmsPk3);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());

        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 newEpochId = EPOCH_COUNTER_BASE + 3;
        _confirmContextCreationWithTwoSigners(newContextId);
        (uint256 contextBeforeCreation, uint256 epochBeforeCreation) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(contextBeforeCreation, KMS_CONTEXT_COUNTER_BASE + 1);
        assertEq(epochBeforeCreation, EPOCH_COUNTER_BASE + 2);
        _confirmContextCreation(newContextId, kmsPk2, "");
        _confirmContextCreation(newContextId, kmsPk3, "");
        (uint256 contextBeforeActivation, uint256 epochBeforeActivation) = protocolConfig
            .getCurrentKmsContextAndEpoch();
        assertEq(contextBeforeActivation, KMS_CONTEXT_COUNTER_BASE + 1);
        assertEq(epochBeforeActivation, EPOCH_COUNTER_BASE + 2);

        (uint256 keyId, uint256 crsId) = _completeKmsGenerationMaterialWithTwoResponses(
            kmsPk0,
            kmsTxSender0,
            kmsPk1,
            kmsTxSender1
        );
        _confirmEpochActivation(newContextId, newEpochId, kmsPk2, keyId, crsId);
        assertEq(protocolConfig.getCurrentKmsContextId(), KMS_CONTEXT_COUNTER_BASE + 1);

        _confirmEpochActivation(newContextId, newEpochId, kmsPk3, keyId, crsId);

        (uint256 activeContextId, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, newContextId);
        assertEq(activeEpochId, newEpochId);
    }

    function test_confirmKmsContextCreationEmitsMaterialBlockNumber() public {
        _setupEpochLifecycle();

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        _confirmContextCreation(newContextId, kmsPk0, "");

        vm.recordLogs();
        _confirmContextCreation(newContextId, kmsPk1, "");
        Vm.Log[] memory logs = vm.getRecordedLogs();

        uint256 createLogIndex = type(uint256).max;
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].topics[0] == IProtocolConfig.NewKmsEpoch.selector) {
                createLogIndex = i;
                break;
            }
        }
        assertTrue(createLogIndex != type(uint256).max);
        assertEq(uint256(logs[createLogIndex].topics[1]), newContextId);
        assertEq(uint256(logs[createLogIndex].topics[2]), EPOCH_COUNTER_BASE + 2);

        (uint256 previousContextId, uint256 previousEpochId, uint256 materialBlockNumber) = abi.decode(
            logs[createLogIndex].data,
            (uint256, uint256, uint256)
        );

        // Context switch: previousContextId is the outgoing (still active) context.
        assertEq(previousContextId, KMS_CONTEXT_COUNTER_BASE + 1);
        assertEq(previousEpochId, EPOCH_COUNTER_BASE + 1);
        assertEq(materialBlockNumber, block.number - 1);
    }

    function test_defineNewEpochForCurrentKmsContextEmitsMaterialBlockNumber() public {
        _setupEpochLifecycle();
        uint256 materialEpochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        (uint256 completedKeyId, uint256 completedCrsId) = _completeKmsGenerationMaterial();

        uint256[] memory keyIds = new uint256[](1);
        keyIds[0] = completedKeyId;
        uint256[] memory crsIds = new uint256[](1);
        crsIds[0] = completedCrsId;

        _confirmEpochActivation(KMS_CONTEXT_COUNTER_BASE + 1, materialEpochId, kmsPk0, keyIds[0], crsIds[0]);
        _confirmEpochActivation(KMS_CONTEXT_COUNTER_BASE + 1, materialEpochId, kmsPk1, keyIds[0], crsIds[0]);

        vm.recordLogs();
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        Vm.Log[] memory logs = vm.getRecordedLogs();

        assertEq(logs[0].topics[0], IProtocolConfig.NewKmsEpoch.selector);
        assertEq(uint256(logs[0].topics[1]), KMS_CONTEXT_COUNTER_BASE + 1);
        assertEq(uint256(logs[0].topics[2]), EPOCH_COUNTER_BASE + 3);

        (uint256 previousContextId, uint256 previousEpochId, uint256 materialBlockNumber) = abi.decode(
            logs[0].data,
            (uint256, uint256, uint256)
        );

        // Same-set resharing: previousContextId equals the current context.
        assertEq(previousContextId, KMS_CONTEXT_COUNTER_BASE + 1);
        assertEq(previousEpochId, materialEpochId);
        assertEq(materialBlockNumber, block.number - 1);
    }

    /// @dev Regression: the first transition points Connectors at an already-observable material block.
    function test_newEpochPointsToHistoricalMaterialBlock() public {
        _setupEpochLifecycle();
        _completeKmsGenerationMaterial();

        vm.recordLogs();
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        Vm.Log[] memory logs = vm.getRecordedLogs();

        uint256 logIndex = type(uint256).max;
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].topics[0] == IProtocolConfig.NewKmsEpoch.selector) {
                logIndex = i;
                break;
            }
        }
        assertTrue(logIndex != type(uint256).max);

        (, , uint256 materialBlockNumber) = abi.decode(logs[logIndex].data, (uint256, uint256, uint256));
        assertEq(materialBlockNumber, block.number - 1);
    }

    function test_revertConfirmKmsContextCreationUnauthorizedAndReplay() public {
        _setupEpochLifecycle();
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        // A signer in neither committee is rejected.
        bytes memory signature = _signContextCreation(newContextId, 0x999, "");
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsContextCreationUnauthorized.selector,
                vm.addr(0x999),
                newContextId
            )
        );
        protocolConfig.confirmKmsContextCreation(newContextId, signature, "");

        // A signer confirms once, whatever the extraData of the second signature.
        _confirmContextCreation(newContextId, kmsPk0, "");
        signature = _signContextCreation(newContextId, kmsPk0, hex"01");
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsContextCreationAlreadyConfirmed.selector,
                vm.addr(kmsPk0),
                newContextId
            )
        );
        protocolConfig.confirmKmsContextCreation(newContextId, signature, hex"01");
    }

    function test_structuredConfirmEpochActivationDivergentDigestsAccumulateSeparately() public {
        _setupEpochLifecycle();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        (uint256 completedKeyId, uint256 completedCrsId) = _completeKmsGenerationMaterial();

        (, uint256 activeEpochBefore) = protocolConfig.getCurrentKmsContextAndEpoch();

        _confirmEpochActivation(KMS_CONTEXT_COUNTER_BASE + 1, epochId, kmsPk0, completedKeyId, completedCrsId);

        bytes memory extraData = abi.encodePacked(uint8(0x02), KMS_CONTEXT_COUNTER_BASE + 1, epochId);
        IKMSGeneration.KeyDigest[] memory keyDigests = _mockKeyDigests();
        keyDigests[0].digest = hex"01020304";
        IProtocolConfig.EpochKeyResult[] memory keys = new IProtocolConfig.EpochKeyResult[](1);
        keys[0] = IProtocolConfig.EpochKeyResult({
            prepKeygenId: PREP_KEYGEN_COUNTER_BASE + 1,
            keyId: completedKeyId,
            keyDigests: keyDigests,
            signature: _computeSignature(
                kmsPk1,
                _hashProtocolConfigKeygen(PREP_KEYGEN_COUNTER_BASE + 1, completedKeyId, keyDigests, extraData)
            )
        });
        IProtocolConfig.EpochCrsResult[] memory crsList = new IProtocolConfig.EpochCrsResult[](1);
        crsList[0] = IProtocolConfig.EpochCrsResult({
            crsId: completedCrsId,
            maxBitLength: 4096,
            crsDigest: hex"deadbeef",
            signature: _computeSignature(
                kmsPk1,
                _hashProtocolConfigCrsgen(completedCrsId, 4096, hex"deadbeef", extraData)
            )
        });

        protocolConfig.confirmEpochActivation(
            epochId,
            keys,
            crsList,
            _signEpochActivation(KMS_CONTEXT_COUNTER_BASE + 1, epochId, kmsPk1, keys, crsList, ""),
            ""
        );

        (, uint256 activeEpochAfter) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochAfter, activeEpochBefore);
    }

    function test_revertConfirmEpochActivationUnauthorizedAndReplay() public {
        _setupEpochLifecycle();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        (uint256 completedKeyId, uint256 completedCrsId) = _completeKmsGenerationMaterial();

        // A signer outside the epoch's committee is rejected, even when all its signatures agree.
        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _buildEpochResults(contextId, epochId, 0x999, completedKeyId, completedCrsId);
        bytes memory signature = _signEpochActivation(contextId, epochId, 0x999, keys, crsList, "");
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.EpochActivationUnauthorized.selector, vm.addr(0x999), epochId)
        );
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, signature, "");

        _confirmEpochActivation(contextId, epochId, kmsPk0, completedKeyId, completedCrsId);

        // Replay with the same material the first confirm used. The signer check passes, so the call reaches
        // the already-confirmed check. An empty array would revert earlier with EmptyEpochActivationAttestation.
        (keys, crsList) = _buildEpochResults(contextId, epochId, kmsPk0, completedKeyId, completedCrsId);
        signature = _signEpochActivation(contextId, epochId, kmsPk0, keys, crsList, "");
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.EpochActivationAlreadyConfirmed.selector, vm.addr(kmsPk0), epochId)
        );
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, signature, "");
    }

    function test_revertConfirmEpochActivationEmptyPayload() public {
        _setupEpochLifecycle();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        // A live pending epoch passes the state check, so the empty payload reaches the dedicated revert.
        IProtocolConfig.EpochKeyResult[] memory keys = new IProtocolConfig.EpochKeyResult[](0);
        IProtocolConfig.EpochCrsResult[] memory crsList = new IProtocolConfig.EpochCrsResult[](0);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.EmptyEpochActivationAttestation.selector, epochId));
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, "", "");
    }

    function test_revertConfirmEpochActivationKeysOnlyPayload() public {
        _setupEpochLifecycle();
        (uint256 completedKeyId, ) = _completeKmsGenerationMaterial();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        bytes memory extraData = abi.encodePacked(uint8(0x02), KMS_CONTEXT_COUNTER_BASE + 1, epochId);
        IKMSGeneration.KeyDigest[] memory keyDigests = _mockKeyDigests();
        IProtocolConfig.EpochKeyResult[] memory keys = new IProtocolConfig.EpochKeyResult[](1);
        keys[0] = IProtocolConfig.EpochKeyResult({
            prepKeygenId: PREP_KEYGEN_COUNTER_BASE + 1,
            keyId: completedKeyId,
            keyDigests: keyDigests,
            signature: _computeSignature(
                kmsPk0,
                _hashProtocolConfigKeygen(PREP_KEYGEN_COUNTER_BASE + 1, completedKeyId, keyDigests, extraData)
            )
        });
        IProtocolConfig.EpochCrsResult[] memory crsList = new IProtocolConfig.EpochCrsResult[](0);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.EmptyEpochActivationAttestation.selector, epochId));
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, "", "");
    }

    function test_revertConfirmEpochActivationCrsOnlyPayload() public {
        _setupEpochLifecycle();
        (, uint256 completedCrsId) = _completeKmsGenerationMaterial();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        bytes memory extraData = abi.encodePacked(uint8(0x02), KMS_CONTEXT_COUNTER_BASE + 1, epochId);
        IProtocolConfig.EpochKeyResult[] memory keys = new IProtocolConfig.EpochKeyResult[](0);
        IProtocolConfig.EpochCrsResult[] memory crsList = new IProtocolConfig.EpochCrsResult[](1);
        crsList[0] = IProtocolConfig.EpochCrsResult({
            crsId: completedCrsId,
            maxBitLength: 4096,
            crsDigest: hex"deadbeef",
            signature: _computeSignature(
                kmsPk0,
                _hashProtocolConfigCrsgen(completedCrsId, 4096, hex"deadbeef", extraData)
            )
        });

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.EmptyEpochActivationAttestation.selector, epochId));
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, "", "");
    }

    function test_confirmEpochActivationAcceptsActiveEpochMaterial() public {
        _setupEpochLifecycle();
        (uint256 completedKeyId, uint256 completedCrsId) = _completeKmsGenerationMaterial();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        uint256[] memory keyIds = new uint256[](1);
        keyIds[0] = completedKeyId;
        uint256[] memory crsIds = new uint256[](1);
        crsIds[0] = completedCrsId;

        _confirmEpochActivation(KMS_CONTEXT_COUNTER_BASE + 1, epochId, kmsPk0, keyIds[0], crsIds[0]);
    }

    /// @dev The aggregate EpochActivationConfirmation must come from the signer of the key and CRS results.
    function test_revertStructuredConfirmEpochActivationSignerMismatch() public {
        _setupEpochLifecycle();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        (uint256 completedKeyId, uint256 completedCrsId) = _completeKmsGenerationMaterial();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _buildEpochResults(contextId, epochId, kmsPk1, completedKeyId, completedCrsId);
        bytes memory signature = _signEpochActivation(contextId, epochId, kmsPk0, keys, crsList, "");

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.EpochResultSignerMismatch.selector, vm.addr(kmsPk1), vm.addr(kmsPk0))
        );
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, signature, "");
    }

    function test_activateEpochEventCarriesMaterialIds() public {
        _setupEpochLifecycle();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        (uint256 completedKeyId, uint256 completedCrsId) = _completeKmsGenerationMaterial();

        _confirmEpochActivation(KMS_CONTEXT_COUNTER_BASE + 1, epochId, kmsPk0, completedKeyId, completedCrsId);

        vm.recordLogs();
        _confirmEpochActivation(KMS_CONTEXT_COUNTER_BASE + 1, epochId, kmsPk1, completedKeyId, completedCrsId);
        Vm.Log[] memory logs = vm.getRecordedLogs();

        uint256 activateLogIndex = type(uint256).max;
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].topics[0] == IProtocolConfig.ActivateEpoch.selector) {
                activateLogIndex = i;
                break;
            }
        }
        assertTrue(activateLogIndex != type(uint256).max);
        assertEq(uint256(logs[activateLogIndex].topics[1]), KMS_CONTEXT_COUNTER_BASE + 1);
        assertEq(uint256(logs[activateLogIndex].topics[2]), epochId);

        (uint256 eventKeyId, uint256 eventCrsId) = this._decodeActivateEpochEventData(logs[activateLogIndex].data);
        assertEq(eventKeyId, completedKeyId);
        assertEq(eventCrsId, completedCrsId);
    }

    /// @dev `external` (called via `this.`) with a single `calldata` argument so the heavy
    ///      multi-array `abi.decode` runs in its own minimal stack frame; this keeps the
    ///      legacy (non-IR) codegen below the EVM stack limit.
    function _decodeActivateEpochEventData(bytes calldata data) external returns (uint256 keyId, uint256 crsId) {
        (
            IProtocolConfig.EpochKeyResult[] memory eventKeys,
            IProtocolConfig.EpochCrsResult[] memory eventCrsList,
            string[] memory urls
        ) = abi.decode(data, (IProtocolConfig.EpochKeyResult[], IProtocolConfig.EpochCrsResult[], string[]));
        assertEq(eventKeys.length, 1);
        assertEq(eventCrsList.length, 1);
        // URLs must match the activated context's nodes in insertion order.
        assertEq(urls.length, 2);
        assertEq(urls[0], "https://s0.example.com");
        assertEq(urls[1], "https://s1.example.com");
        keyId = eventKeys[0].keyId;
        crsId = eventCrsList[0].crsId;
    }

    function test_epochIdsUseTaggedCounterAndIncrementGlobally() public {
        _setupEpochLifecycle();

        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        (uint256 completedKeyId, uint256 completedCrsId) = _completeKmsGenerationMaterial();
        _confirmEpochActivation(
            KMS_CONTEXT_COUNTER_BASE + 1,
            EPOCH_COUNTER_BASE + 2,
            kmsPk0,
            completedKeyId,
            completedCrsId
        );
        _confirmEpochActivation(
            KMS_CONTEXT_COUNTER_BASE + 1,
            EPOCH_COUNTER_BASE + 2,
            kmsPk1,
            completedKeyId,
            completedCrsId
        );

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());

        (, uint256 activeEpochBeforeContextActivation) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochBeforeContextActivation, EPOCH_COUNTER_BASE + 2);
        _confirmContextCreationWithTwoSigners(KMS_CONTEXT_COUNTER_BASE + 2);
        (uint256 nextKeyId, uint256 nextCrsId) = _completeKmsGenerationMaterial();
        _confirmEpochActivation(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 3, kmsPk0, nextKeyId, nextCrsId);
        _confirmEpochActivation(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 3, kmsPk1, nextKeyId, nextCrsId);
        (, uint256 finalActiveEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(finalActiveEpochId, EPOCH_COUNTER_BASE + 3);
    }

    function test_emptyProxyInitializer_emitsNewKmsContext() public {
        _setupEmptyProxy();

        address impl = address(new ProtocolConfig());
        KmsNodeParams[] memory nodeParams = _makeKmsNodeParams(2);
        PcrValues[] memory pcrValues = new PcrValues[](0);
        KmsThresholds memory thresholds = _defaultThresholds();

        vm.expectEmit(true, true, false, true, protocolConfigAdd);
        emit IProtocolConfig.NewKmsContext(
            KMS_CONTEXT_COUNTER_BASE + 1,
            KMS_CONTEXT_COUNTER_BASE,
            nodeParams,
            thresholds,
            "",
            pcrValues
        );

        vm.prank(owner);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(ProtocolConfig.initializeFromEmptyProxy, (nodeParams, thresholds, "", pcrValues))
        );
    }

    // -----------------------------------------------------------------------
    // View-function guards (invalid & destroyed contexts)
    // -----------------------------------------------------------------------

    function testFuzz_revertViewFunctionsForInvalidContext(uint256 invalidContextId) public {
        _setupDefault();
        vm.assume(invalidContextId != protocolConfig.getCurrentKmsContextId());
        _expectContextGuardedViewsRevert(invalidContextId);
        // A never-created context does not exist, so the node lookup reverts too.
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, invalidContextId));
        protocolConfig.getKmsNodeForContext(invalidContextId, address(0xDEAD));
    }

    function test_revertViewFunctionsForDestroyedContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        _seedActiveEpochWithMaterialForFourNodeContext();

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(1), _defaultThresholds());
        _activatePendingContextWithOneKmsNode(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 3);

        vm.prank(owner);
        protocolConfig.destroyKmsContext(firstContextId);

        _expectContextGuardedViewsRevert(firstContextId);
    }

    /// @dev Key/CRS material keeps pointing at the context it was generated under, so its nodes must
    ///      stay readable once that context is destroyed.
    function test_getKmsNodeForContextReadableAfterDestroy() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        _seedActiveEpochWithMaterialForFourNodeContext();

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(1), _defaultThresholds());
        _activatePendingContextWithOneKmsNode(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 3);

        vm.prank(owner);
        protocolConfig.destroyKmsContext(firstContextId);

        KmsNode memory expectedNode = _makeKmsNodes(4)[0];
        KmsNode memory node = protocolConfig.getKmsNodeForContext(firstContextId, expectedNode.txSenderAddress);
        assertEq(node.txSenderAddress, expectedNode.txSenderAddress);
        assertEq(node.signerAddress, expectedNode.signerAddress);
        assertEq(node.storageUrl, expectedNode.storageUrl);
    }

    // -----------------------------------------------------------------------
    // Threshold getters after context rotation
    // -----------------------------------------------------------------------

    function test_getUserDecryptionThresholdForContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        // _setupDefault uses userDecryption = 2
        assertEq(protocolConfig.getUserDecryptionThresholdForContext(firstContextId), 2);
        _seedActiveEpochWithMaterialForFourNodeContext();

        // Rotate to a new context with userDecryption = 1
        KmsThresholds memory newThresholds = KmsThresholds({publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 1});
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), newThresholds);
        uint256 secondContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _activatePendingContextWithTwoKmsNodes(secondContextId, EPOCH_COUNTER_BASE + 3);

        // New context returns its own threshold
        assertEq(protocolConfig.getUserDecryptionThresholdForContext(secondContextId), 1);
        // Old context still returns the original threshold
        assertEq(protocolConfig.getUserDecryptionThresholdForContext(firstContextId), 2);
    }

    function test_getKmsGenThresholdForContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        assertEq(protocolConfig.getKmsGenThresholdForContext(firstContextId), 3);
        _seedActiveEpochWithMaterialForFourNodeContext();

        KmsThresholds memory newThresholds = KmsThresholds({publicDecryption: 1, userDecryption: 1, kmsGen: 2, mpc: 1});
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), newThresholds);
        uint256 secondContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _activatePendingContextWithTwoKmsNodes(secondContextId, EPOCH_COUNTER_BASE + 3);

        assertEq(protocolConfig.getKmsGenThresholdForContext(secondContextId), 2);
        assertEq(protocolConfig.getKmsGenThresholdForContext(firstContextId), 3);

        uint256 invalidId = KMS_CONTEXT_COUNTER_BASE + 999;
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, invalidId));
        protocolConfig.getKmsGenThresholdForContext(invalidId);
    }

    function test_getMpcThresholdForContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        assertEq(protocolConfig.getMpcThresholdForContext(firstContextId), 4);
        _seedActiveEpochWithMaterialForFourNodeContext();

        KmsThresholds memory newThresholds = KmsThresholds({publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 2});
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), newThresholds);
        uint256 secondContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _activatePendingContextWithTwoKmsNodes(secondContextId, EPOCH_COUNTER_BASE + 3);

        assertEq(protocolConfig.getMpcThresholdForContext(secondContextId), 2);
        assertEq(protocolConfig.getMpcThresholdForContext(firstContextId), 4);

        uint256 invalidId = KMS_CONTEXT_COUNTER_BASE + 999;
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, invalidId));
        protocolConfig.getMpcThresholdForContext(invalidId);
    }

    function test_getPublicDecryptionThresholdForContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        assertEq(protocolConfig.getPublicDecryptionThresholdForContext(firstContextId), 1);
        _seedActiveEpochWithMaterialForFourNodeContext();

        KmsThresholds memory newThresholds = KmsThresholds({publicDecryption: 2, userDecryption: 1, kmsGen: 2, mpc: 1});
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), newThresholds);
        uint256 secondContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _activatePendingContextWithTwoKmsNodes(secondContextId, EPOCH_COUNTER_BASE + 3);

        assertEq(protocolConfig.getPublicDecryptionThresholdForContext(secondContextId), 2);
        assertEq(protocolConfig.getPublicDecryptionThresholdForContext(firstContextId), 1);

        uint256 invalidId = KMS_CONTEXT_COUNTER_BASE + 999;
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, invalidId));
        protocolConfig.getPublicDecryptionThresholdForContext(invalidId);
    }

    function test_thresholdsAfterContextRotation() public {
        _setupDefault();
        _seedActiveEpochWithMaterialForFourNodeContext();
        // Initial context uses thresholds {1, 2, 3, 4}.
        // Define a new context with different thresholds.
        KmsThresholds memory newThresholds = KmsThresholds({publicDecryption: 2, userDecryption: 1, kmsGen: 2, mpc: 1});

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), newThresholds);
        _activatePendingContextWithTwoKmsNodes(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 3);

        assertEq(protocolConfig.getPublicDecryptionThreshold(), 2);
        assertEq(protocolConfig.getUserDecryptionThreshold(), 1);
        assertEq(protocolConfig.getKmsGenThreshold(), 2);
        assertEq(protocolConfig.getMpcThreshold(), 1);
    }

    // -----------------------------------------------------------------------
    // Re-initialization protection
    // -----------------------------------------------------------------------

    function test_revertDoubleInit() public {
        _setupDefault();

        // onlyFromEmptyProxy fires first (version is 3, not 1) before reinitializer.
        vm.prank(owner);
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        protocolConfig.initializeFromEmptyProxy(_makeKmsNodeParams(1), _defaultThresholds(), "", new PcrValues[](0));
    }

    // -----------------------------------------------------------------------
    // Access control
    // -----------------------------------------------------------------------

    function test_revertDefineNewKmsContextAndEpochNotOwner() public {
        _setupDefault();
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(1), _defaultThresholds());
    }

    function test_revertDestroyContextNotOwner() public {
        _setupDefault();
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.destroyKmsContext(KMS_CONTEXT_COUNTER_BASE + 1);
    }

    function test_revertUpgradeNotOwner() public {
        _setupDefault();

        address newImpl = address(new ProtocolConfigUpgradedExample());
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.upgradeToAndCall(newImpl, "");
    }

    function test_upgradeSuccess() public {
        _setupDefault();

        address newImpl = address(new ProtocolConfigUpgradedExample());
        vm.prank(owner);
        protocolConfig.upgradeToAndCall(newImpl, "");

        assertEq(protocolConfig.getVersion(), "ProtocolConfig v0.5.0");
        // State preserved across upgrade.
        assertTrue(protocolConfig.isValidKmsContext(protocolConfig.getCurrentKmsContextId()));
    }

    // -----------------------------------------------------------------------
    // reinitializeV4 upgrade path
    // -----------------------------------------------------------------------

    /// @dev Calls reinitializeV4() on a proxy pinned at the initialized version the v0.3.0
    ///      implementation left behind (4), as the upgrade tooling does. The pre-store check proves
    ///      the hardcoded slot is where OZ Initializable writes.
    function test_reinitializeV4SucceedsOnUpgradePath() public {
        _setupEmptyProxy();
        address impl = address(new ProtocolConfig());
        vm.prank(owner);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(impl, "");

        bytes32 initializableStorage = 0xf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00;
        assertEq(uint256(vm.load(protocolConfigAdd, initializableStorage)), 1);
        vm.store(protocolConfigAdd, initializableStorage, bytes32(uint256(4)));

        ProtocolConfig(protocolConfigAdd).reinitializeV4();
        assertEq(uint256(vm.load(protocolConfigAdd, initializableStorage)), 5);

        vm.expectRevert(Initializable.InvalidInitialization.selector);
        ProtocolConfig(protocolConfigAdd).reinitializeV4();
    }

    // -----------------------------------------------------------------------
    // confirmEpochActivation negative branches
    // -----------------------------------------------------------------------

    /// @dev Destroying a Created context clears its pending epoch, so a later confirmation for that
    ///      epoch reverts on the epoch state guard.
    function test_revertConfirmEpochActivationForDestroyedContext() public {
        _setupEpochLifecycle();
        _seedActiveEpochWithMaterialForTwoNodeContext();

        // Switch to a second context and activate it so the first becomes destroyable.
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[0].signerAddress = vm.addr(kmsPk2);
        nodes[1].signerAddress = vm.addr(kmsPk3);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());
        uint256 secondContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 secondEpochId = EPOCH_COUNTER_BASE + 3;
        _confirmContextCreationWithTwoSigners(secondContextId);
        _confirmContextCreation(secondContextId, kmsPk2, "");
        _confirmContextCreation(secondContextId, kmsPk3, "");
        (uint256 keyId, uint256 crsId) = _completeKmsGenerationMaterialWithTwoResponses(
            kmsPk0,
            kmsTxSender0,
            kmsPk1,
            kmsTxSender1
        );
        _confirmEpochActivation(secondContextId, secondEpochId, kmsPk2, keyId, crsId);
        _confirmEpochActivation(secondContextId, secondEpochId, kmsPk3, keyId, crsId);

        // Open a third context switch, confirm its creation (Created), then destroy that context.
        // Its pending epoch is cleared on destruction, so a later confirmation hits the lifecycle guards.
        KmsNodeParams[] memory thirdNodes = _makeKmsNodeParams(2);
        thirdNodes[0].signerAddress = vm.addr(kmsPk0);
        thirdNodes[1].signerAddress = vm.addr(kmsPk1);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(thirdNodes, _defaultThresholds());
        uint256 thirdContextId = KMS_CONTEXT_COUNTER_BASE + 3;
        uint256 thirdEpochId = EPOCH_COUNTER_BASE + 4;

        _confirmContextCreation(thirdContextId, kmsPk2, "");
        _confirmContextCreation(thirdContextId, kmsPk0, "");
        _confirmContextCreation(thirdContextId, kmsPk1, "");

        vm.prank(owner);
        protocolConfig.destroyKmsContext(thirdContextId);

        // The pending epoch was cleared with the context, so confirming activation now reverts on the
        // epoch state guard (InvalidKmsEpoch) — the context is no longer live for it.
        IProtocolConfig.EpochKeyResult[] memory keys = new IProtocolConfig.EpochKeyResult[](0);
        IProtocolConfig.EpochCrsResult[] memory crsList = new IProtocolConfig.EpochCrsResult[](0);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, thirdEpochId));
        protocolConfig.confirmEpochActivation(thirdEpochId, keys, crsList, "", "");
    }

    /// @dev Partial quorum (one of two new signers confirming context creation) must not advance the
    ///      live context id.
    function test_partialContextCreationQuorumDoesNotAdvance() public {
        _setupEpochLifecycle();
        uint256 activeContextIdBefore = protocolConfig.getCurrentKmsContextId();

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        // Only one signer confirms: quorum not reached.
        _confirmContextCreation(newContextId, kmsPk0, "");

        assertEq(protocolConfig.getCurrentKmsContextId(), activeContextIdBefore);
        assertFalse(protocolConfig.isValidKmsContext(newContextId));
    }

    /// @dev n=3 epoch-activation split: two signers agree on one digest, a third diverges. Neither
    ///      group reaches the full-quorum (3) threshold, so the epoch never activates. Distinct from the
    ///      n=2 divergence test, which can never have an agreeing majority.
    function test_divergentDigestSplitThreeSignersDoesNotActivate() public {
        // Deploy a 3-node context directly as the active context (signers kmsPk0/1/2, kmsGen threshold 1).
        _deployACL(owner);
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(3);
        KmsThresholds memory thresholds = KmsThresholds({publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 1});
        (ProtocolConfig pc, ) = _deployProtocolConfig(owner, nodes, thresholds);
        protocolConfig = pc;
        (KMSGeneration kg, ) = _deployKMSGeneration(owner);
        kmsGeneration = kg;

        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 epochId = EPOCH_COUNTER_BASE + 2;

        // Same-set resharing epoch under the active 3-node context.
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        (uint256 keyId, uint256 crsId) = _completeKmsGenerationMaterial();
        (, uint256 activeEpochBefore) = protocolConfig.getCurrentKmsContextAndEpoch();

        // Two signers agree on the matching digest.
        _confirmEpochActivation(contextId, epochId, kmsPk0, keyId, crsId);
        _confirmEpochActivation(contextId, epochId, kmsPk1, keyId, crsId);

        // Third signer diverges (different key digest) — accumulates under a separate hash.
        bytes memory extraData = abi.encodePacked(uint8(0x02), contextId, epochId);
        IKMSGeneration.KeyDigest[] memory keyDigests = _mockKeyDigests();
        keyDigests[0].digest = hex"99887766";
        uint256 prepKeygenId = _prepKeygenIdForKeyId(keyId);
        IProtocolConfig.EpochKeyResult[] memory keys = new IProtocolConfig.EpochKeyResult[](1);
        keys[0] = IProtocolConfig.EpochKeyResult({
            prepKeygenId: prepKeygenId,
            keyId: keyId,
            keyDigests: keyDigests,
            signature: _computeSignature(kmsPk2, _hashProtocolConfigKeygen(prepKeygenId, keyId, keyDigests, extraData))
        });
        IProtocolConfig.EpochCrsResult[] memory crsList = new IProtocolConfig.EpochCrsResult[](1);
        crsList[0] = IProtocolConfig.EpochCrsResult({
            crsId: crsId,
            maxBitLength: 4096,
            crsDigest: hex"deadbeef",
            signature: _computeSignature(kmsPk2, _hashProtocolConfigCrsgen(crsId, 4096, hex"deadbeef", extraData))
        });
        protocolConfig.confirmEpochActivation(
            epochId,
            keys,
            crsList,
            _signEpochActivation(contextId, epochId, kmsPk2, keys, crsList, ""),
            ""
        );

        // No digest group reached the full quorum of 3 — epoch unchanged.
        (uint256 activeContextAfter, uint256 activeEpochAfter) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochAfter, activeEpochBefore);
        assertEq(activeContextAfter, contextId);
    }

    // -----------------------------------------------------------------------
    // Per-confirmation events and malformed-signature negative
    // -----------------------------------------------------------------------

    /// @dev Asserts KmsContextCreationConfirmation carries the recovered signer, the signature and extraData.
    function test_kmsContextCreationConfirmationEvent() public {
        _setupEpochLifecycle();
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        bytes memory extraData = hex"01ab";
        bytes memory signature = _signContextCreation(newContextId, kmsPk1, extraData);
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextCreationConfirmation(newContextId, vm.addr(kmsPk1), signature, extraData);
        protocolConfig.confirmKmsContextCreation(newContextId, signature, extraData);
    }

    /// @dev A signer in both committees confirms once and counts toward both sides. New committee
    ///      {signer0, signer2}, previous quorum 1: signer2 then signer0 complete the creation.
    function test_confirmKmsContextCreationSharedSignerCountsInBothSets() public {
        _setupEpochLifecycle();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[1].signerAddress = vm.addr(kmsPk2);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        _confirmContextCreation(newContextId, kmsPk2, "");

        bytes memory signature = _signContextCreation(newContextId, kmsPk0, "");
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.NewKmsEpoch(
            newContextId,
            EPOCH_COUNTER_BASE + 2,
            KMS_CONTEXT_COUNTER_BASE + 1,
            EPOCH_COUNTER_BASE + 1,
            block.number - 1
        );
        protocolConfig.confirmKmsContextCreation(newContextId, signature, "");
    }

    /// @dev Asserts EpochActivationConfirmation carries the signer, epochMaterialHash, signature and extraData.
    function test_epochActivationConfirmationEvent() public {
        _setupEpochLifecycle();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        (uint256 completedKeyId, uint256 completedCrsId) = _completeKmsGenerationMaterial();

        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _buildEpochResults(contextId, epochId, kmsPk0, completedKeyId, completedCrsId);
        bytes memory extraData = hex"01ab";
        bytes memory signature = _signEpochActivation(contextId, epochId, kmsPk0, keys, crsList, extraData);
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.EpochActivationConfirmation(
            epochId,
            vm.addr(kmsPk0),
            _epochMaterialHash(keys, crsList),
            signature,
            extraData
        );
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, signature, extraData);
    }

    /// @dev Distinct from the signer-mismatch test: a malformed (too-short) signature must hit
    ///      ECDSA.recover's length validation, with real key material in the payload.
    function test_revertConfirmEpochActivationMalformedSignature() public {
        _setupEpochLifecycle();
        (uint256 completedKeyId, uint256 completedCrsId) = _completeKmsGenerationMaterial();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        IProtocolConfig.EpochKeyResult[] memory keys = new IProtocolConfig.EpochKeyResult[](1);
        IKMSGeneration.KeyDigest[] memory keyDigests = _mockKeyDigests();
        keys[0] = IProtocolConfig.EpochKeyResult({
            prepKeygenId: _prepKeygenIdForKeyId(completedKeyId),
            keyId: completedKeyId,
            keyDigests: keyDigests,
            // 65 bytes is the only valid ECDSA length; a 10-byte blob is rejected by ECDSA.recover.
            signature: hex"00112233445566778899"
        });
        // The CRS entry only keeps the payload past the non-empty check, so the keys loop reaches ECDSA.recover.
        bytes memory extraData = abi.encodePacked(uint8(0x02), KMS_CONTEXT_COUNTER_BASE + 1, epochId);
        IProtocolConfig.EpochCrsResult[] memory crsList = new IProtocolConfig.EpochCrsResult[](1);
        crsList[0] = IProtocolConfig.EpochCrsResult({
            crsId: completedCrsId,
            maxBitLength: 4096,
            crsDigest: hex"deadbeef",
            signature: _computeSignature(
                kmsPk0,
                _hashProtocolConfigCrsgen(completedCrsId, 4096, hex"deadbeef", extraData)
            )
        });

        vm.expectRevert();
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, "", "");
    }

    // -----------------------------------------------------------------------
    // EIP-712 context creation confirmations
    // -----------------------------------------------------------------------

    /// @dev The recovered signer counts, not the caller: an unrelated account submits both signatures.
    function test_confirmKmsContextCreationAcceptsAnySender() public {
        _setupEpochLifecycle();
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        bytes memory signature = _signContextCreation(newContextId, kmsPk0, "");
        vm.prank(address(0x999));
        protocolConfig.confirmKmsContextCreation(newContextId, signature, "");

        signature = _signContextCreation(newContextId, kmsPk1, "");
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.NewKmsEpoch(
            newContextId,
            EPOCH_COUNTER_BASE + 2,
            KMS_CONTEXT_COUNTER_BASE + 1,
            EPOCH_COUNTER_BASE + 1,
            block.number - 1
        );
        vm.prank(address(0x999));
        protocolConfig.confirmKmsContextCreation(newContextId, signature, "");
    }

    /// @dev OZ ECDSA rejects the high-s twin of a valid signature and the 64-byte compact form.
    function test_revertConfirmKmsContextCreationMalleableSignature() public {
        _setupEpochLifecycle();
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(
            kmsPk0,
            _hashContextCreation(KMS_CONTEXT_COUNTER_BASE + 1, newContextId, nodeConfigHashes[newContextId], "")
        );
        // secp256k1 group order.
        uint256 n = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141;
        bytes32 highS = bytes32(n - uint256(s));
        vm.expectRevert(abi.encodeWithSelector(ECDSA.ECDSAInvalidSignatureS.selector, highS));
        protocolConfig.confirmKmsContextCreation(
            newContextId,
            abi.encodePacked(r, highS, v == 27 ? uint8(28) : uint8(27)),
            ""
        );

        vm.expectRevert(abi.encodeWithSelector(ECDSA.ECDSAInvalidSignatureLength.selector, 64));
        protocolConfig.confirmKmsContextCreation(newContextId, abi.encodePacked(r, s), "");
    }

    /// @dev nodeConfigHash binds the stored nodes and thresholds: a signature over other thresholds
    ///      recovers an unrelated address, and the same signer over the defined ones is accepted.
    function test_revertConfirmKmsContextCreationOverOtherNodeConfig() public {
        _setupEpochLifecycle();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        KmsThresholds memory thresholds = _defaultThresholds();
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, thresholds);
        uint256 previousContextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        KmsThresholds memory otherThresholds = KmsThresholds({
            publicDecryption: 1,
            userDecryption: 1,
            kmsGen: 1,
            mpc: 2
        });
        bytes memory signature = _computeSignature(
            kmsPk0,
            _hashContextCreation(previousContextId, newContextId, _nodeConfigHash(nodes, otherThresholds), "")
        );
        vm.expectPartialRevert(IProtocolConfig.KmsContextCreationUnauthorized.selector);
        protocolConfig.confirmKmsContextCreation(newContextId, signature, "");

        signature = _computeSignature(
            kmsPk0,
            _hashContextCreation(previousContextId, newContextId, _nodeConfigHash(nodes, thresholds), "")
        );
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextCreationConfirmation(newContextId, vm.addr(kmsPk0), signature, "");
        protocolConfig.confirmKmsContextCreation(newContextId, signature, "");
    }

    /// @dev Confirmations count per digest: two signers with different extraData never reach the
    ///      all-new-signers quorum, so no epoch is created for the switch.
    function test_confirmKmsContextCreationDivergentExtraDataDoesNotComplete() public {
        _setupEpochLifecycle();
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        _confirmContextCreation(newContextId, kmsPk0, "");
        _confirmContextCreation(newContextId, kmsPk1, hex"01");

        // The latest-issued epoch is still the genesis one: the switch never got its epoch.
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsLifecycleOperationInFlight.selector,
                newContextId,
                EPOCH_COUNTER_BASE + 1
            )
        );
        protocolConfig.defineNewEpochForCurrentKmsContext();
    }

    /// @dev Both quorums must hold on one digest: the previous quorum on extraData X and new-committee
    ///      unanimity on extraData Y do not complete the switch.
    function test_confirmKmsContextCreationQuorumsOnDifferentExtraDataDoNotComplete() public {
        _setupEpochLifecycle();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[0].signerAddress = vm.addr(kmsPk2);
        nodes[1].signerAddress = vm.addr(kmsPk3);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        // The previous quorum is max(n - t, 1) = 1 for the genesis committee.
        _confirmContextCreation(newContextId, kmsPk0, hex"01");
        _confirmContextCreation(newContextId, kmsPk2, hex"02");
        _confirmContextCreation(newContextId, kmsPk3, hex"02");

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsLifecycleOperationInFlight.selector,
                newContextId,
                EPOCH_COUNTER_BASE + 1
            )
        );
        protocolConfig.defineNewEpochForCurrentKmsContext();
    }

    /// @dev previousContextId is the latest active context, not `newContextId - 1`: a destroyed switch
    ///      leaves a gap in the ids.
    function test_confirmKmsContextCreationPreviousContextIdSkipsDestroyedContext() public {
        _setupEpochLifecycle();
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        vm.prank(owner);
        protocolConfig.destroyKmsContext(KMS_CONTEXT_COUNTER_BASE + 2);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 3;

        bytes memory signature = _computeSignature(
            kmsPk0,
            _hashContextCreation(KMS_CONTEXT_COUNTER_BASE + 2, newContextId, nodeConfigHashes[newContextId], "")
        );
        vm.expectPartialRevert(IProtocolConfig.KmsContextCreationUnauthorized.selector);
        protocolConfig.confirmKmsContextCreation(newContextId, signature, "");

        signature = _computeSignature(
            kmsPk0,
            _hashContextCreation(KMS_CONTEXT_COUNTER_BASE + 1, newContextId, nodeConfigHashes[newContextId], "")
        );
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextCreationConfirmation(newContextId, vm.addr(kmsPk0), signature, "");
        protocolConfig.confirmKmsContextCreation(newContextId, signature, "");
    }

    // -----------------------------------------------------------------------
    // EIP-712 epoch activation confirmations
    // -----------------------------------------------------------------------

    function test_revertConfirmEpochActivationResultsFromTwoSigners() public {
        _setupEpochLifecycle();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        IProtocolConfig.EpochKeyResult[] memory keys;
        IProtocolConfig.EpochCrsResult[] memory crsList;
        (keys, ) = _buildEpochResults(contextId, epochId, kmsPk0, KEY_COUNTER_BASE + 1, CRS_COUNTER_BASE + 1);
        (, crsList) = _buildEpochResults(contextId, epochId, kmsPk1, KEY_COUNTER_BASE + 1, CRS_COUNTER_BASE + 1);
        bytes memory signature = _signEpochActivation(contextId, epochId, kmsPk0, keys, crsList, "");

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.EpochResultSignerMismatch.selector, vm.addr(kmsPk0), vm.addr(kmsPk1))
        );
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, signature, "");
    }

    /// @dev Activation needs every signer on one digest: the same material with different extraData
    ///      splits the vote.
    function test_confirmEpochActivationDivergentExtraDataDoesNotActivate() public {
        _setupEpochLifecycle();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        _confirmEpochActivation(contextId, epochId, kmsPk0);
        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _buildEpochResults(contextId, epochId, kmsPk1, KEY_COUNTER_BASE + 1, CRS_COUNTER_BASE + 1);
        protocolConfig.confirmEpochActivation(
            epochId,
            keys,
            crsList,
            _signEpochActivation(contextId, epochId, kmsPk1, keys, crsList, hex"01"),
            hex"01"
        );

        (, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochId, EPOCH_COUNTER_BASE + 1);
    }

    /// @dev previousEpochId is the latest active epoch, not `epochId - 1`: a destroyed Pending epoch
    ///      leaves a gap in the ids.
    function test_confirmEpochActivationPreviousEpochIdSkipsDestroyedEpoch() public {
        _setupEpochLifecycle();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        vm.prank(owner);
        protocolConfig.destroyKmsEpoch(EPOCH_COUNTER_BASE + 2);
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        uint256 epochId = EPOCH_COUNTER_BASE + 3;

        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _buildEpochResults(contextId, epochId, kmsPk0, KEY_COUNTER_BASE + 1, CRS_COUNTER_BASE + 1);
        bytes32 epochMaterialHash = _epochMaterialHash(keys, crsList);

        // Over `epochId - 1`, the aggregate signature recovers an address other than the result signer.
        bytes memory signature = _computeSignature(
            kmsPk0,
            _hashEpochActivation(contextId, EPOCH_COUNTER_BASE + 2, epochId, epochMaterialHash, "")
        );
        vm.expectPartialRevert(IProtocolConfig.EpochResultSignerMismatch.selector);
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, signature, "");

        signature = _computeSignature(
            kmsPk0,
            _hashEpochActivation(contextId, EPOCH_COUNTER_BASE + 1, epochId, epochMaterialHash, "")
        );
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, signature, "");
        _confirmEpochActivation(contextId, epochId, kmsPk1);

        (, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochId, epochId);
    }

    function test_revertConfirmEpochActivationAfterActive() public {
        _setupEpochLifecycle();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        _confirmEpochActivation(contextId, epochId, kmsPk0);
        _confirmEpochActivation(contextId, epochId, kmsPk1);

        (
            IProtocolConfig.EpochKeyResult[] memory keys,
            IProtocolConfig.EpochCrsResult[] memory crsList
        ) = _buildEpochResults(contextId, epochId, kmsPk0, KEY_COUNTER_BASE + 1, CRS_COUNTER_BASE + 1);
        bytes memory signature = _signEpochActivation(contextId, epochId, kmsPk0, keys, crsList, "");
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, epochId));
        protocolConfig.confirmEpochActivation(epochId, keys, crsList, signature, "");
    }

    // -----------------------------------------------------------------------
    // EIP-712 destruction confirmations
    // -----------------------------------------------------------------------

    /// @dev Defines a switch to the disjoint committee {signer2, signer3}, confirms its creation, then
    ///      destroys it with its pending epoch. The genesis committee {signer0, signer1} stays active.
    function _destroyCreatedDisjointContext() internal returns (uint256 contextId, uint256 epochId) {
        contextId = KMS_CONTEXT_COUNTER_BASE + 2;
        epochId = EPOCH_COUNTER_BASE + 2;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[0].signerAddress = vm.addr(kmsPk2);
        nodes[1].signerAddress = vm.addr(kmsPk3);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());
        _confirmContextCreationWithTwoSigners(contextId);
        _confirmContextCreation(contextId, kmsPk2, "");
        _confirmContextCreation(contextId, kmsPk3, "");
        vm.prank(owner);
        protocolConfig.destroyKmsContext(contextId);
    }

    function test_revertConfirmKmsContextDestructionBeforeDestroy() public {
        _setupEpochLifecycle();
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 2;

        uint256[] memory destroyedEpochIds = new uint256[](0);
        bytes memory signature = _computeSignature(kmsPk0, _hashContextDestruction(contextId, destroyedEpochIds, ""));
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.KmsContextNotDestroyed.selector, contextId));
        protocolConfig.confirmKmsContextDestruction(contextId, destroyedEpochIds, signature, "");
    }

    /// @dev Only the active committee confirms, once per signer, from any sender. destroyedEpochIds is
    ///      digest material only, so both the cleared epoch and an empty list are accepted.
    function test_confirmKmsContextDestruction() public {
        _setupEpochLifecycle();
        (uint256 contextId, uint256 epochId) = _destroyCreatedDisjointContext();
        uint256[] memory destroyedEpochIds = new uint256[](1);
        destroyedEpochIds[0] = epochId;

        // signer2 belongs to the destroyed committee only.
        bytes memory signature = _computeSignature(kmsPk2, _hashContextDestruction(contextId, destroyedEpochIds, ""));
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsContextDestructionUnauthorized.selector,
                vm.addr(kmsPk2),
                contextId
            )
        );
        protocolConfig.confirmKmsContextDestruction(contextId, destroyedEpochIds, signature, "");

        bytes memory extraData = hex"01ab";
        signature = _computeSignature(kmsPk0, _hashContextDestruction(contextId, destroyedEpochIds, extraData));
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextDestructionConfirmed(
            contextId,
            destroyedEpochIds,
            vm.addr(kmsPk0),
            signature,
            extraData
        );
        vm.prank(address(0x999));
        protocolConfig.confirmKmsContextDestruction(contextId, destroyedEpochIds, signature, extraData);

        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsContextDestructionAlreadyConfirmed.selector,
                vm.addr(kmsPk0),
                contextId
            )
        );
        protocolConfig.confirmKmsContextDestruction(contextId, destroyedEpochIds, signature, extraData);

        uint256[] memory noEpochIds = new uint256[](0);
        signature = _computeSignature(kmsPk1, _hashContextDestruction(contextId, noEpochIds, ""));
        protocolConfig.confirmKmsContextDestruction(contextId, noEpochIds, signature, "");
    }

    /// @dev An epoch cleared by destroyKmsContext is confirmed through the context destruction.
    function test_revertConfirmKmsEpochDestructionForEpochClearedWithContext() public {
        _setupEpochLifecycle();
        (, uint256 epochId) = _destroyCreatedDisjointContext();

        bytes memory signature = _computeSignature(kmsPk0, _hashEpochDestruction(epochId, ""));
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.KmsEpochNotDestroyed.selector, epochId));
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, "");
    }

    function test_revertConfirmKmsEpochDestructionBeforeDestroy() public {
        _setupEpochLifecycle();
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;

        bytes memory signature = _computeSignature(kmsPk0, _hashEpochDestruction(epochId, ""));
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.KmsEpochNotDestroyed.selector, epochId));
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, "");
    }

    /// @dev After a switch to {signer2, signer3}, the superseded genesis epoch is destroyed. Its own
    ///      committee no longer confirms, the active one does, once per signer.
    function test_confirmKmsEpochDestruction() public {
        _setupEpochLifecycle();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[0].signerAddress = vm.addr(kmsPk2);
        nodes[1].signerAddress = vm.addr(kmsPk3);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _confirmContextCreationWithTwoSigners(newContextId);
        _confirmContextCreation(newContextId, kmsPk2, "");
        _confirmContextCreation(newContextId, kmsPk3, "");
        _confirmEpochActivation(newContextId, EPOCH_COUNTER_BASE + 2, kmsPk2);
        _confirmEpochActivation(newContextId, EPOCH_COUNTER_BASE + 2, kmsPk3);

        uint256 epochId = EPOCH_COUNTER_BASE + 1;
        vm.prank(owner);
        protocolConfig.destroyKmsEpoch(epochId);

        bytes memory signature = _computeSignature(kmsPk0, _hashEpochDestruction(epochId, ""));
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsEpochDestructionUnauthorized.selector, vm.addr(kmsPk0), epochId)
        );
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, "");

        bytes memory extraData = hex"01ab";
        signature = _computeSignature(kmsPk2, _hashEpochDestruction(epochId, extraData));
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsEpochDestructionConfirmed(epochId, vm.addr(kmsPk2), signature, extraData);
        vm.prank(address(0x999));
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, extraData);

        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsEpochDestructionAlreadyConfirmed.selector,
                vm.addr(kmsPk2),
                epochId
            )
        );
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, extraData);
    }

    // -----------------------------------------------------------------------
    // Coprocessor context tests
    // -----------------------------------------------------------------------

    function _makeChainUpgradeWindows(uint256 count) internal pure returns (ChainUpgradeWindow[] memory windows) {
        windows = new ChainUpgradeWindow[](count);
        for (uint256 i = 0; i < count; i++) {
            windows[i] = ChainUpgradeWindow({
                chainId: uint64(i + 1),
                startBlock: uint64(100 + i),
                endBlock: uint64(200 + i)
            });
        }
    }

    function _defaultSoftwareVersion() internal pure returns (string memory) {
        return "v0.14.0";
    }

    function _defaultGwStartBlock() internal pure returns (uint64) {
        return 8421337;
    }

    function _defaultProposalId() internal pure returns (uint256) {
        return 1;
    }

    function test_proposeCoprocessorUpgrade_singleChain() public {
        _setupDefault();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(1);
        uint64 gwStart = _defaultGwStartBlock();
        string memory version = _defaultSoftwareVersion();
        uint256 proposalId = _defaultProposalId();

        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.CoprocessorUpgradeProposed(proposalId, version, windows, gwStart);
        vm.prank(owner);
        protocolConfig.proposeCoprocessorUpgrade(proposalId, version, windows, gwStart);
    }

    function test_proposeCoprocessorUpgrade_multiChain() public {
        _setupDefault();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(3);
        uint64 gwStart = _defaultGwStartBlock();
        string memory version = _defaultSoftwareVersion();
        uint256 proposalId = _defaultProposalId();

        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.CoprocessorUpgradeProposed(proposalId, version, windows, gwStart);
        vm.prank(owner);
        protocolConfig.proposeCoprocessorUpgrade(proposalId, version, windows, gwStart);
    }

    function test_revertCoprocessor_InvalidProposalId() public {
        _setupDefault();
        vm.prank(owner);
        vm.expectRevert(IProtocolConfig.InvalidProposalId.selector);
        protocolConfig.proposeCoprocessorUpgrade(
            0,
            _defaultSoftwareVersion(),
            _makeChainUpgradeWindows(1),
            _defaultGwStartBlock()
        );
    }

    function test_revertCoprocessor_EmptySoftwareVersion() public {
        _setupDefault();
        vm.prank(owner);
        vm.expectRevert(IProtocolConfig.EmptySoftwareVersion.selector);
        protocolConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            "",
            _makeChainUpgradeWindows(1),
            _defaultGwStartBlock()
        );
    }

    function test_revertCoprocessor_EmptyChainUpgradeWindows() public {
        _setupDefault();
        ChainUpgradeWindow[] memory windows = new ChainUpgradeWindow[](0);
        vm.prank(owner);
        vm.expectRevert(IProtocolConfig.EmptyChainUpgradeWindows.selector);
        protocolConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );
    }

    function test_revertCoprocessor_ZeroChainId() public {
        _setupDefault();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(1);
        windows[0].chainId = 0;
        vm.prank(owner);
        vm.expectRevert(IProtocolConfig.ZeroChainId.selector);
        protocolConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );
    }

    function test_revertCoprocessor_DuplicateChainId() public {
        _setupDefault();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(2);
        windows[1].chainId = windows[0].chainId;
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.DuplicateChainId.selector, windows[0].chainId));
        protocolConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );
    }

    function test_revertCoprocessor_InvalidBlockWindow() public {
        _setupDefault();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(1);
        windows[0].startBlock = 500;
        windows[0].endBlock = 100; // start > end
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.InvalidBlockWindow.selector,
                windows[0].chainId,
                uint64(500),
                uint64(100)
            )
        );
        protocolConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );
    }

    function test_revertCoprocessor_ZeroGwStartBlock() public {
        _setupDefault();
        vm.prank(owner);
        vm.expectRevert(IProtocolConfig.ZeroGwStartBlock.selector);
        protocolConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            _defaultSoftwareVersion(),
            _makeChainUpgradeWindows(1),
            0
        );
    }

    function test_proposeCoprocessorUpgrade_sameIdTwice_bothEmit() public {
        _setupDefault();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(1);
        uint256 proposalId = _defaultProposalId();

        vm.prank(owner);
        protocolConfig.proposeCoprocessorUpgrade(
            proposalId,
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );

        // Uniqueness is the caller's responsibility — contract does not enforce.
        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.CoprocessorUpgradeProposed(
            proposalId,
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );
        vm.prank(owner);
        protocolConfig.proposeCoprocessorUpgrade(
            proposalId,
            _defaultSoftwareVersion(),
            windows,
            _defaultGwStartBlock()
        );
    }

    function test_proposeCoprocessorUpgrade_maxUint256() public {
        _setupDefault();
        uint256 proposalId = type(uint256).max;

        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.CoprocessorUpgradeProposed(
            proposalId,
            _defaultSoftwareVersion(),
            _makeChainUpgradeWindows(1),
            _defaultGwStartBlock()
        );
        vm.prank(owner);
        protocolConfig.proposeCoprocessorUpgrade(
            proposalId,
            _defaultSoftwareVersion(),
            _makeChainUpgradeWindows(1),
            _defaultGwStartBlock()
        );
    }

    function test_proposeCoprocessorUpgrade_twoDistinctPending_bothEmit() public {
        _setupDefault();
        ChainUpgradeWindow[] memory windows = _makeChainUpgradeWindows(1);

        vm.prank(owner);
        protocolConfig.proposeCoprocessorUpgrade(1, _defaultSoftwareVersion(), windows, _defaultGwStartBlock());

        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.CoprocessorUpgradeProposed(2, _defaultSoftwareVersion(), windows, _defaultGwStartBlock());
        vm.prank(owner);
        protocolConfig.proposeCoprocessorUpgrade(2, _defaultSoftwareVersion(), windows, _defaultGwStartBlock());
    }

    function test_revertProposeCoprocessorUpgradeNotOwner() public {
        _setupDefault();
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.proposeCoprocessorUpgrade(
            _defaultProposalId(),
            _defaultSoftwareVersion(),
            _makeChainUpgradeWindows(1),
            _defaultGwStartBlock()
        );
    }
}
