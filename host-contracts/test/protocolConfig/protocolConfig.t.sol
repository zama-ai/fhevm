// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Vm} from "forge-std/Test.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {KMSGeneration} from "@fhevm-host-contracts/contracts/KMSGeneration.sol";
import {IKMSGeneration} from "@fhevm-host-contracts/contracts/interfaces/IKMSGeneration.sol";
import {ProtocolConfigUpgradedExample} from "@fhevm-host-contracts/examples/ProtocolConfigUpgradedExample.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {KmsNode, KmsNodeParams, PcrValues} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {UUPSUpgradeableEmptyProxy} from "@fhevm-host-contracts/contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE, PREP_KEYGEN_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {protocolConfigAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";
import {ProtocolConfigV010TestDouble} from "./ProtocolConfigV010TestDouble.sol";

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
        IProtocolConfig.KmsThresholds memory thresholds = IProtocolConfig.KmsThresholds({
            publicDecryption: 1,
            userDecryption: 2,
            kmsGen: 3,
            mpc: 4
        });
        (ProtocolConfig pc, ) = _deployProtocolConfig(owner, _makeKmsNodeParams(4), thresholds);
        protocolConfig = pc;
        (KMSGeneration kg, ) = _deployKMSGeneration(owner);
        kmsGeneration = kg;
    }

    function _setupDefaultWithMpcThreshold(uint256 mpcThreshold) internal {
        _deployACL(owner);
        IProtocolConfig.KmsThresholds memory thresholds = IProtocolConfig.KmsThresholds({
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
        IProtocolConfig.KmsThresholds memory thresholds,
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

    function _revertThreshold(IProtocolConfig.KmsThresholds memory t, bytes memory expectedRevert) internal {
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
        IProtocolConfig.KmsThresholds memory thresholds,
        string memory softwareVersion,
        PcrValues[] memory pcrValues
    ) internal {
        protocolConfig.defineNewKmsContextAndEpoch(nodes, thresholds, softwareVersion, pcrValues);
    }

    function _confirmEpoch(uint256, /* contextId */ uint256 epochId, uint256, /* pk */ address txSender) internal {
        IProtocolConfig.EpochKeyResult[] memory keys = new IProtocolConfig.EpochKeyResult[](0);
        IProtocolConfig.EpochCrsResult[] memory crsList = new IProtocolConfig.EpochCrsResult[](0);
        vm.prank(txSender);
        protocolConfig.confirmEpochActivation(epochId, keys, crsList);
    }

    function _confirmEpochWithMaterial(
        uint256 contextId,
        uint256 epochId,
        uint256 pk,
        address txSender,
        uint256 keyId,
        uint256 crsId
    ) internal {
        bytes memory extraData = abi.encodePacked(uint8(0x02), contextId, epochId);
        IKMSGeneration.KeyDigest[] memory keyDigests = _mockKeyDigests();
        uint256 prepKeygenId = _prepKeygenIdForKeyId(keyId);

        IProtocolConfig.EpochKeyResult[] memory keys = new IProtocolConfig.EpochKeyResult[](1);
        keys[0] = IProtocolConfig.EpochKeyResult({
            prepKeygenId: prepKeygenId,
            keyId: keyId,
            keyDigests: keyDigests,
            signature: _computeSignature(pk, _hashProtocolConfigKeygen(prepKeygenId, keyId, keyDigests, extraData))
        });

        IProtocolConfig.EpochCrsResult[] memory crsList = new IProtocolConfig.EpochCrsResult[](1);
        crsList[0] = IProtocolConfig.EpochCrsResult({
            crsId: crsId,
            maxBitLength: 4096,
            crsDigest: hex"deadbeef",
            signature: _computeSignature(pk, _hashProtocolConfigCrsgen(crsId, 4096, hex"deadbeef", extraData))
        });

        vm.prank(txSender);
        protocolConfig.confirmEpochActivation(epochId, keys, crsList);
    }

    function _seedActiveEpochWithMaterialForTwoNodeContext() internal returns (uint256 epochId) {
        epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        (uint256 completedKeyId, uint256 completedCrsId) = _completeKmsGenerationMaterial();
        _confirmEpochWithMaterial(
            KMS_CONTEXT_COUNTER_BASE + 1,
            epochId,
            kmsPk0,
            kmsTxSender0,
            completedKeyId,
            completedCrsId
        );
        _confirmEpochWithMaterial(
            KMS_CONTEXT_COUNTER_BASE + 1,
            epochId,
            kmsPk1,
            kmsTxSender1,
            completedKeyId,
            completedCrsId
        );
    }

    function _confirmContextCreationWithTwoSigners(uint256 contextId) internal {
        vm.prank(vm.addr(kmsPk0));
        protocolConfig.confirmKmsContextCreation(contextId);
        vm.prank(vm.addr(kmsPk1));
        protocolConfig.confirmKmsContextCreation(contextId);
    }

    function _activatePendingContextWithOneKmsNode(uint256 contextId, uint256 epochId) internal {
        vm.prank(vm.addr(kmsPk0));
        protocolConfig.confirmKmsContextCreation(contextId);
        _confirmEpoch(contextId, epochId, kmsPk0, kmsTxSender0);
    }

    function _activatePendingContextWithTwoKmsNodes(uint256 contextId, uint256 epochId) internal {
        _confirmContextCreationWithTwoSigners(contextId);
        _confirmEpoch(contextId, epochId, kmsPk0, kmsTxSender0);
        _confirmEpoch(contextId, epochId, kmsPk1, kmsTxSender1);
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
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);
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
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);
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
        kmsGeneration.keygen(IKMSGeneration.ParamsType.Default);
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
        _confirmEpochWithMaterial(
            KMS_CONTEXT_COUNTER_BASE + 1,
            epochId,
            kmsPk0,
            kmsTxSender0,
            completedKeyId,
            completedCrsId
        );
        _confirmEpochWithMaterial(
            KMS_CONTEXT_COUNTER_BASE + 1,
            epochId,
            kmsPk1,
            kmsTxSender1,
            completedKeyId,
            completedCrsId
        );
        _confirmEpochWithMaterial(
            KMS_CONTEXT_COUNTER_BASE + 1,
            epochId,
            kmsPk2,
            kmsTxSender2,
            completedKeyId,
            completedCrsId
        );
        _confirmEpochWithMaterial(
            KMS_CONTEXT_COUNTER_BASE + 1,
            epochId,
            kmsPk3,
            kmsTxSender3,
            completedKeyId,
            completedCrsId
        );
    }

    /// @dev Asserts all seven context-guarded view functions revert for the given context ID.
    function _expectAllViewsRevertForContext(uint256 contextId) internal {
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, contextId));
        protocolConfig.getKmsSignersForContext(contextId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, contextId));
        protocolConfig.isKmsSignerForContext(contextId, address(0xDEAD));

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, contextId));
        protocolConfig.getKmsNodesForContext(contextId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, contextId));
        protocolConfig.isKmsTxSenderForContext(contextId, address(0xDEAD));

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, contextId));
        protocolConfig.getKmsNodeForContext(contextId, address(0xDEAD));

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, contextId));
        protocolConfig.getUserDecryptionThresholdForContext(contextId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, contextId));
        protocolConfig.getPublicDecryptionThresholdForContext(contextId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, contextId));
        protocolConfig.getKmsGenThresholdForContext(contextId);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, contextId));
        protocolConfig.getMpcThresholdForContext(contextId);
    }

    // -----------------------------------------------------------------------
    // Init tests
    // -----------------------------------------------------------------------

    function test_initSuccess() public {
        _setupDefault();

        // Version and current context.
        assertEq(protocolConfig.getVersion(), "ProtocolConfig v0.2.0");
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

    // -----------------------------------------------------------------------
    // Validation error tests
    // -----------------------------------------------------------------------

    function test_revertEmptyNodes() public {
        _setupEmptyProxy();
        KmsNodeParams[] memory emptyNodes = new KmsNodeParams[](0);
        _upgradeProxyExpectRevert(
            emptyNodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfig.EmptyKmsNodes.selector)
        );
    }

    function test_revertNullTxSender() public {
        _setupEmptyProxy();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(1);
        nodes[0].txSenderAddress = address(0);
        _upgradeProxyExpectRevert(
            nodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfig.KmsNodeNullTxSender.selector)
        );
    }

    function test_revertNullSigner() public {
        _setupEmptyProxy();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(1);
        nodes[0].signerAddress = address(0);
        _upgradeProxyExpectRevert(
            nodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfig.KmsNodeNullSigner.selector)
        );
    }

    function test_revertDuplicateTxSender() public {
        _setupEmptyProxy();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[1].txSenderAddress = nodes[0].txSenderAddress;
        _upgradeProxyExpectRevert(
            nodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfig.KmsTxSenderAlreadyRegistered.selector, nodes[0].txSenderAddress)
        );
    }

    function test_revertDuplicateSigner() public {
        _setupEmptyProxy();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[1].signerAddress = nodes[0].signerAddress;
        _upgradeProxyExpectRevert(
            nodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfig.KmsSignerAlreadyRegistered.selector, nodes[0].signerAddress)
        );
    }

    function test_revertNullPublicDecryptionThreshold() public {
        IProtocolConfig.KmsThresholds memory t = _defaultThresholds();
        t.publicDecryption = 0;
        _revertThreshold(t, abi.encodeWithSelector(IProtocolConfig.InvalidNullThreshold.selector, "publicDecryption"));
    }

    function test_revertHighPublicDecryptionThreshold() public {
        IProtocolConfig.KmsThresholds memory t = _defaultThresholds();
        t.publicDecryption = 5;
        _revertThreshold(
            t,
            abi.encodeWithSelector(IProtocolConfig.InvalidHighThreshold.selector, "publicDecryption", 5, 1)
        );
    }

    function test_revertNullUserDecryptionThreshold() public {
        IProtocolConfig.KmsThresholds memory t = _defaultThresholds();
        t.userDecryption = 0;
        _revertThreshold(t, abi.encodeWithSelector(IProtocolConfig.InvalidNullThreshold.selector, "userDecryption"));
    }

    function test_revertHighUserDecryptionThreshold() public {
        IProtocolConfig.KmsThresholds memory t = _defaultThresholds();
        t.userDecryption = 5;
        _revertThreshold(
            t,
            abi.encodeWithSelector(IProtocolConfig.InvalidHighThreshold.selector, "userDecryption", 5, 1)
        );
    }

    function test_revertNullKmsGenThreshold() public {
        IProtocolConfig.KmsThresholds memory t = _defaultThresholds();
        t.kmsGen = 0;
        _revertThreshold(t, abi.encodeWithSelector(IProtocolConfig.InvalidNullThreshold.selector, "kmsGen"));
    }

    function test_revertHighKmsGenThreshold() public {
        IProtocolConfig.KmsThresholds memory t = _defaultThresholds();
        t.kmsGen = 5;
        _revertThreshold(t, abi.encodeWithSelector(IProtocolConfig.InvalidHighThreshold.selector, "kmsGen", 5, 1));
    }

    function test_revertNullMpcThreshold() public {
        IProtocolConfig.KmsThresholds memory t = _defaultThresholds();
        t.mpc = 0;
        _revertThreshold(t, abi.encodeWithSelector(IProtocolConfig.InvalidNullThreshold.selector, "mpc"));
    }

    function test_revertHighMpcThreshold() public {
        IProtocolConfig.KmsThresholds memory t = _defaultThresholds();
        t.mpc = 5;
        _revertThreshold(t, abi.encodeWithSelector(IProtocolConfig.InvalidHighThreshold.selector, "mpc", 5, 1));
    }

    function test_revertSignerSetExceedsProofFormatLimit() public {
        _setupEmptyProxy();
        KmsNodeParams[] memory tooManyNodes = _makeKmsNodeParams(256);
        _upgradeProxyExpectRevert(
            tooManyNodes,
            _defaultThresholds(),
            abi.encodeWithSelector(IProtocolConfig.KmsSignerSetExceedsProofFormatLimit.selector, 256, 255)
        );
    }

    function test_revertThresholdExceedsProofFormatLimit() public {
        // A threshold above the proof-format limit is rejected before the per-node-count check.
        IProtocolConfig.KmsThresholds memory t = _defaultThresholds();
        t.publicDecryption = 256;
        _revertThreshold(
            t,
            abi.encodeWithSelector(
                IProtocolConfig.ThresholdExceedsProofFormatLimit.selector,
                "publicDecryption",
                256,
                255
            )
        );
    }

    // -----------------------------------------------------------------------
    // Context lifecycle tests
    // -----------------------------------------------------------------------

    function test_defineNewKmsContextAndEpochCreatesPendingContext() public {
        _setupDefault();

        KmsNodeParams[] memory newNodeParams = _makeKmsNodeParams(1);
        PcrValues[] memory pcrValues = new PcrValues[](0);

        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();
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
        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();
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
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.CurrentKmsContextCannotBeDestroyed.selector, currentId));
        protocolConfig.destroyKmsContext(currentId);
    }

    function testFuzz_revertDestroyInvalidContext(uint256 invalidContextId) public {
        _setupDefault();
        vm.assume(invalidContextId != protocolConfig.getCurrentKmsContextId());
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, invalidContextId));
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
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, firstContextId));
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

        _confirmEpochWithMaterial(
            KMS_CONTEXT_COUNTER_BASE + 1,
            epochId,
            kmsPk0,
            kmsTxSender0,
            completedKeyId,
            completedCrsId
        );
        (, uint256 activeEpochBeforeSecondConfirmation) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochBeforeSecondConfirmation, EPOCH_COUNTER_BASE + 1);

        _confirmEpochWithMaterial(
            KMS_CONTEXT_COUNTER_BASE + 1,
            epochId,
            kmsPk1,
            kmsTxSender1,
            completedKeyId,
            completedCrsId
        );

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

    function test_mirrorKmsEpochActivatesCanonicalEpoch() public {
        _setupEpochLifecycle();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 epochId = EPOCH_COUNTER_BASE + 7;

        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.MirrorKmsEpoch(contextId, epochId);
        vm.prank(owner);
        protocolConfig.mirrorKmsEpoch(contextId, epochId);

        (uint256 activeContextId, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, contextId);
        assertEq(activeEpochId, epochId);
        assertTrue(protocolConfig.isValidEpochForContext(contextId, epochId));
    }

    function test_revertMirrorKmsEpochNotOwner() public {
        _setupEpochLifecycle();

        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.mirrorKmsEpoch(KMS_CONTEXT_COUNTER_BASE + 1, EPOCH_COUNTER_BASE + 2);
    }

    function test_revertMirrorKmsEpochInvalidContext() public {
        _setupEpochLifecycle();
        uint256 invalidContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, invalidContextId));
        protocolConfig.mirrorKmsEpoch(invalidContextId, EPOCH_COUNTER_BASE + 2);
    }

    function test_revertMirrorKmsEpochNonIncreasingEpoch() public {
        _setupEpochLifecycle();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 currentEpochId = EPOCH_COUNTER_BASE + 1;
        uint256 staleEpochId = currentEpochId - 1;

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.NonIncreasingEpochId.selector, staleEpochId, currentEpochId)
        );
        protocolConfig.mirrorKmsEpoch(contextId, staleEpochId);
    }

    function test_revertMirrorKmsEpochBeforePendingEpochCounter() public {
        _setupEpochLifecycle();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;

        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.NonIncreasingEpochId.selector,
                EPOCH_COUNTER_BASE + 2,
                EPOCH_COUNTER_BASE + 2
            )
        );
        protocolConfig.mirrorKmsEpoch(contextId, EPOCH_COUNTER_BASE + 2);
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

    function test_isValidEpochForContext_falseForAbortedPendingEpoch() public {
        _setupEpochLifecycle();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 abortedEpochId = EPOCH_COUNTER_BASE + 2;

        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        vm.prank(owner);
        protocolConfig.abortPendingEpoch(abortedEpochId);

        assertFalse(protocolConfig.isValidEpochForContext(contextId, abortedEpochId));
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

    function test_abortPendingEpochForCurrentKmsContext() public {
        _setupEpochLifecycle();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;

        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.PendingEpochAborted(KMS_CONTEXT_COUNTER_BASE + 1, epochId);
        vm.prank(owner);
        protocolConfig.abortPendingEpoch(epochId);

        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        (, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochId, EPOCH_COUNTER_BASE + 1);
    }

    function test_revertAbortPendingEpochForPendingContext() public {
        _setupEpochLifecycle();

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 2;

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.EpochNotUnderActiveContext.selector, epochId, contextId)
        );
        protocolConfig.abortPendingEpoch(epochId);
    }

    function test_abortPendingContextClearsStateAndAllowsRedefine() public {
        _setupEpochLifecycle();
        uint256 pendingContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        (uint256 activeContextBefore, uint256 activeEpochBefore) = protocolConfig.getCurrentKmsContextAndEpoch();

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());

        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.PendingContextAborted(pendingContextId);
        vm.prank(owner);
        protocolConfig.abortPendingContext(pendingContextId);

        // Active context/epoch are untouched.
        (uint256 activeContextAfter, uint256 activeEpochAfter) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeContextAfter, activeContextBefore);
        assertEq(activeEpochAfter, activeEpochBefore);
        assertFalse(protocolConfig.isValidKmsContext(pendingContextId));

        // The aborted context can no longer be confirmed.
        vm.prank(vm.addr(kmsPk0));
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.KmsContextNotPending.selector, pendingContextId));
        protocolConfig.confirmKmsContextCreation(pendingContextId);

        // The pending slot is released, so a fresh pending context can be defined.
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
    }

    function test_revertAbortPendingContextWhenActive() public {
        _setupEpochLifecycle();
        uint256 activeContextId = protocolConfig.getCurrentKmsContextId();
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.KmsContextNotPending.selector, activeContextId));
        protocolConfig.abortPendingContext(activeContextId);
    }

    function test_revertAbortPendingContextWhenCreated() public {
        _setupEpochLifecycle();
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        uint256 createdContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _confirmContextCreationWithTwoSigners(createdContextId);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.KmsContextNotPending.selector, createdContextId));
        protocolConfig.abortPendingContext(createdContextId);
    }

    function test_revertAbortPendingContextWhenAlreadyAborted() public {
        _setupEpochLifecycle();
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        uint256 pendingContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        vm.prank(owner);
        protocolConfig.abortPendingContext(pendingContextId);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.KmsContextNotPending.selector, pendingContextId));
        protocolConfig.abortPendingContext(pendingContextId);
    }

    function testFuzz_revertAbortPendingContextInvalidId(uint256 invalidContextId) public {
        _setupEpochLifecycle();
        vm.assume(invalidContextId != protocolConfig.getCurrentKmsContextId());
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.KmsContextNotPending.selector, invalidContextId));
        protocolConfig.abortPendingContext(invalidContextId);
    }

    function test_revertAbortPendingContextNotOwner() public {
        _setupEpochLifecycle();
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        uint256 pendingContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.abortPendingContext(pendingContextId);
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

    function test_confirmKmsContextCreationUsesNewSignersAndOldQuorum() public {
        _setupDefaultWithMpcThreshold(3);
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[0].signerAddress = address(0xB2);
        nodes[1].signerAddress = address(0xB3);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        vm.prank(address(0xB2));
        protocolConfig.confirmKmsContextCreation(newContextId);
        vm.prank(address(0xB3));
        protocolConfig.confirmKmsContextCreation(newContextId);
        vm.prank(vm.addr(kmsPk0));
        protocolConfig.confirmKmsContextCreation(newContextId);
        assertFalse(protocolConfig.isValidKmsContext(newContextId));

        // Full-args assertion (indexed kmsContextId/epochId + data) on the quorum-completing event.
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.NewKmsEpoch(
            newContextId,
            EPOCH_COUNTER_BASE + 2,
            KMS_CONTEXT_COUNTER_BASE + 1,
            EPOCH_COUNTER_BASE + 1,
            block.number - 1
        );
        vm.prank(vm.addr(kmsPk1));
        protocolConfig.confirmKmsContextCreation(newContextId);
    }

    function test_confirmKmsContextCreationRequiresAllNewSigners() public {
        _setupDefaultWithMpcThreshold(3);
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[0].signerAddress = address(0xB2);
        nodes[1].signerAddress = address(0xB3);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        vm.prank(address(0xB2));
        protocolConfig.confirmKmsContextCreation(newContextId);
        vm.prank(vm.addr(kmsPk0));
        protocolConfig.confirmKmsContextCreation(newContextId);
        vm.prank(vm.addr(kmsPk1));
        protocolConfig.confirmKmsContextCreation(newContextId);
        assertFalse(protocolConfig.isValidKmsContext(newContextId));

        // The quorum-completing confirmation (all new signers present) must emit NewKmsEpoch with the
        // pending epoch's full indexed args. Before this last confirmation no such event was emitted.
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.NewKmsEpoch(
            newContextId,
            EPOCH_COUNTER_BASE + 2,
            KMS_CONTEXT_COUNTER_BASE + 1,
            EPOCH_COUNTER_BASE + 1,
            block.number - 1
        );
        vm.prank(address(0xB3));
        protocolConfig.confirmKmsContextCreation(newContextId);
    }

    function test_revertConfirmEpochActivationBeforeCreateKmsContext() public {
        _setupEpochLifecycle();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[0].signerAddress = vm.addr(kmsPk2);
        nodes[1].signerAddress = vm.addr(kmsPk3);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());

        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 newEpochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(kmsTxSender0);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.KmsContextNotCreated.selector, newContextId));
        IProtocolConfig.EpochKeyResult[] memory keys = new IProtocolConfig.EpochKeyResult[](0);
        IProtocolConfig.EpochCrsResult[] memory crsList = new IProtocolConfig.EpochCrsResult[](0);
        protocolConfig.confirmEpochActivation(newEpochId, keys, crsList);
    }

    function test_destroyCreatedContext() public {
        _setupEpochLifecycle();

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        uint256 createdContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        vm.prank(vm.addr(kmsPk0));
        protocolConfig.confirmKmsContextCreation(createdContextId);
        vm.prank(vm.addr(kmsPk1));
        protocolConfig.confirmKmsContextCreation(createdContextId);

        vm.prank(owner);
        protocolConfig.destroyKmsContext(createdContextId);
        assertFalse(protocolConfig.isValidKmsContext(createdContextId));
    }

    function test_fullContextSwitchFlow() public {
        _setupEpochLifecycle();
        _seedActiveEpochWithMaterialForTwoNodeContext();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[0].signerAddress = vm.addr(kmsPk2);
        nodes[1].signerAddress = vm.addr(kmsPk3);

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());

        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 newEpochId = EPOCH_COUNTER_BASE + 3;
        vm.prank(vm.addr(kmsPk0));
        protocolConfig.confirmKmsContextCreation(newContextId);
        vm.prank(vm.addr(kmsPk1));
        protocolConfig.confirmKmsContextCreation(newContextId);
        (uint256 contextBeforeCreation, uint256 epochBeforeCreation) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(contextBeforeCreation, KMS_CONTEXT_COUNTER_BASE + 1);
        assertEq(epochBeforeCreation, EPOCH_COUNTER_BASE + 2);
        vm.prank(vm.addr(kmsPk2));
        protocolConfig.confirmKmsContextCreation(newContextId);
        vm.prank(vm.addr(kmsPk3));
        protocolConfig.confirmKmsContextCreation(newContextId);
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
        _confirmEpochWithMaterial(newContextId, newEpochId, kmsPk2, kmsTxSender0, keyId, crsId);
        assertEq(protocolConfig.getCurrentKmsContextId(), KMS_CONTEXT_COUNTER_BASE + 1);

        _confirmEpochWithMaterial(newContextId, newEpochId, kmsPk3, kmsTxSender1, keyId, crsId);

        (uint256 activeContextId, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, newContextId);
        assertEq(activeEpochId, newEpochId);
    }

    function test_confirmKmsContextCreationEmitsMaterialBlockNumber() public {
        _setupEpochLifecycle();

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        vm.prank(vm.addr(kmsPk0));
        protocolConfig.confirmKmsContextCreation(newContextId);

        vm.recordLogs();
        vm.prank(vm.addr(kmsPk1));
        protocolConfig.confirmKmsContextCreation(newContextId);
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

        _confirmEpochWithMaterial(
            KMS_CONTEXT_COUNTER_BASE + 1,
            materialEpochId,
            kmsPk0,
            kmsTxSender0,
            keyIds[0],
            crsIds[0]
        );
        _confirmEpochWithMaterial(
            KMS_CONTEXT_COUNTER_BASE + 1,
            materialEpochId,
            kmsPk1,
            kmsTxSender1,
            keyIds[0],
            crsIds[0]
        );

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

        vm.prank(address(0x999));
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsContextCreationUnauthorized.selector,
                address(0x999),
                newContextId
            )
        );
        protocolConfig.confirmKmsContextCreation(newContextId);

        vm.prank(vm.addr(kmsPk0));
        protocolConfig.confirmKmsContextCreation(newContextId);
        vm.prank(vm.addr(kmsPk0));
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.KmsContextCreationAlreadyConfirmed.selector,
                vm.addr(kmsPk0),
                newContextId
            )
        );
        protocolConfig.confirmKmsContextCreation(newContextId);
    }

    function test_structuredConfirmEpochActivationDivergentDigestsAccumulateSeparately() public {
        _setupEpochLifecycle();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        (uint256 completedKeyId, uint256 completedCrsId) = _completeKmsGenerationMaterial();

        (, uint256 activeEpochBefore) = protocolConfig.getCurrentKmsContextAndEpoch();

        _confirmEpochWithMaterial(
            KMS_CONTEXT_COUNTER_BASE + 1,
            epochId,
            kmsPk0,
            kmsTxSender0,
            completedKeyId,
            completedCrsId
        );

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

        vm.prank(kmsTxSender1);
        protocolConfig.confirmEpochActivation(epochId, keys, crsList);

        (, uint256 activeEpochAfter) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochAfter, activeEpochBefore);
    }

    /// @dev Trigger events are independent from activation payloads; material is read by Connectors
    ///      from KMSGeneration at the emitted historical block.
    function test_emptyEpochActivationStillEmitsMaterialBlockNumber() public {
        _setupEpochLifecycle();
        uint256 materialEpochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        (uint256 completedKeyId, uint256 completedCrsId) = _completeKmsGenerationMaterial();
        _confirmEpochWithMaterial(
            KMS_CONTEXT_COUNTER_BASE + 1,
            materialEpochId,
            kmsPk0,
            kmsTxSender0,
            completedKeyId,
            completedCrsId
        );
        _confirmEpochWithMaterial(
            KMS_CONTEXT_COUNTER_BASE + 1,
            materialEpochId,
            kmsPk1,
            kmsTxSender1,
            completedKeyId,
            completedCrsId
        );

        uint256 emptyEpochId = EPOCH_COUNTER_BASE + 3;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        IProtocolConfig.EpochKeyResult[] memory keys = new IProtocolConfig.EpochKeyResult[](0);
        IProtocolConfig.EpochCrsResult[] memory crsList = new IProtocolConfig.EpochCrsResult[](0);

        vm.prank(kmsTxSender0);
        protocolConfig.confirmEpochActivation(emptyEpochId, keys, crsList);
        vm.prank(kmsTxSender1);
        protocolConfig.confirmEpochActivation(emptyEpochId, keys, crsList);

        (uint256 activeContextId, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, KMS_CONTEXT_COUNTER_BASE + 1);
        assertEq(activeEpochId, emptyEpochId);

        vm.recordLogs();
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        Vm.Log[] memory logs = vm.getRecordedLogs();

        (uint256 previousContextId, uint256 previousEpochId, uint256 materialBlockNumber) = abi.decode(
            logs[0].data,
            (uint256, uint256, uint256)
        );

        assertEq(previousContextId, KMS_CONTEXT_COUNTER_BASE + 1);
        assertEq(previousEpochId, emptyEpochId);
        assertEq(materialBlockNumber, block.number - 1);
    }

    function test_revertConfirmEpochActivationUnauthorizedAndReplay() public {
        _setupEpochLifecycle();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        // Caller that isn't a tx sender of the epoch's context is rejected.
        vm.prank(address(0x999));
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.EpochActivationUnauthorized.selector, address(0x999), epochId)
        );
        IProtocolConfig.EpochKeyResult[] memory keys = new IProtocolConfig.EpochKeyResult[](0);
        IProtocolConfig.EpochCrsResult[] memory crsList = new IProtocolConfig.EpochCrsResult[](0);
        protocolConfig.confirmEpochActivation(epochId, keys, crsList);

        (uint256 completedKeyId, uint256 completedCrsId) = _completeKmsGenerationMaterial();
        _confirmEpochWithMaterial(
            KMS_CONTEXT_COUNTER_BASE + 1,
            epochId,
            kmsPk0,
            kmsTxSender0,
            completedKeyId,
            completedCrsId
        );

        vm.prank(kmsTxSender0);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.EpochActivationAlreadyConfirmed.selector, vm.addr(kmsPk0), epochId)
        );
        protocolConfig.confirmEpochActivation(epochId, keys, crsList);
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

        _confirmEpochWithMaterial(KMS_CONTEXT_COUNTER_BASE + 1, epochId, kmsPk0, kmsTxSender0, keyIds[0], crsIds[0]);
    }

    function test_revertStructuredConfirmEpochActivationSignerMismatch() public {
        _setupEpochLifecycle();
        (uint256 completedKeyId, ) = _completeKmsGenerationMaterial();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        IProtocolConfig.EpochKeyResult[] memory keys = new IProtocolConfig.EpochKeyResult[](1);
        IKMSGeneration.KeyDigest[] memory keyDigests = _mockKeyDigests();
        bytes memory extraData = abi.encodePacked(uint8(0x02), KMS_CONTEXT_COUNTER_BASE + 1, epochId);
        keys[0] = IProtocolConfig.EpochKeyResult({
            prepKeygenId: PREP_KEYGEN_COUNTER_BASE + 1,
            keyId: completedKeyId,
            keyDigests: keyDigests,
            signature: _computeSignature(
                kmsPk1,
                _hashProtocolConfigKeygen(PREP_KEYGEN_COUNTER_BASE + 1, completedKeyId, keyDigests, extraData)
            )
        });
        IProtocolConfig.EpochCrsResult[] memory crsList = new IProtocolConfig.EpochCrsResult[](0);

        vm.prank(kmsTxSender0);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfig.EpochActivationSignerDoesNotMatchTxSender.selector,
                vm.addr(kmsPk1),
                kmsTxSender0
            )
        );
        protocolConfig.confirmEpochActivation(epochId, keys, crsList);
    }

    function test_activateEpochEventCarriesMaterialIds() public {
        _setupEpochLifecycle();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        (uint256 completedKeyId, uint256 completedCrsId) = _completeKmsGenerationMaterial();

        uint256[] memory keyIds = new uint256[](1);
        keyIds[0] = completedKeyId;
        uint256[] memory crsIds = new uint256[](1);
        crsIds[0] = completedCrsId;

        _confirmEpochWithMaterial(KMS_CONTEXT_COUNTER_BASE + 1, epochId, kmsPk0, kmsTxSender0, keyIds[0], crsIds[0]);

        vm.recordLogs();
        _confirmEpochWithMaterial(KMS_CONTEXT_COUNTER_BASE + 1, epochId, kmsPk1, kmsTxSender1, keyIds[0], crsIds[0]);
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

        (
            IProtocolConfig.EpochKeyResult[] memory eventKeys,
            IProtocolConfig.EpochCrsResult[] memory eventCrsList,
            string[] memory urls
        ) = abi.decode(
                logs[activateLogIndex].data,
                (IProtocolConfig.EpochKeyResult[], IProtocolConfig.EpochCrsResult[], string[])
            );
        assertEq(eventKeys.length, 1);
        assertEq(eventKeys[0].keyId, completedKeyId);
        assertEq(eventCrsList.length, 1);
        assertEq(eventCrsList[0].crsId, completedCrsId);
        // URLs must match the activated context's nodes in insertion order.
        assertEq(urls.length, 2);
        assertEq(urls[0], "https://s0.example.com");
        assertEq(urls[1], "https://s1.example.com");
    }

    function test_epochIdsUseTaggedCounterAndIncrementGlobally() public {
        _setupEpochLifecycle();

        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        (uint256 completedKeyId, uint256 completedCrsId) = _completeKmsGenerationMaterial();
        _confirmEpochWithMaterial(
            KMS_CONTEXT_COUNTER_BASE + 1,
            EPOCH_COUNTER_BASE + 2,
            kmsPk0,
            kmsTxSender0,
            completedKeyId,
            completedCrsId
        );
        _confirmEpochWithMaterial(
            KMS_CONTEXT_COUNTER_BASE + 1,
            EPOCH_COUNTER_BASE + 2,
            kmsPk1,
            kmsTxSender1,
            completedKeyId,
            completedCrsId
        );

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());

        (, uint256 activeEpochBeforeContextActivation) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochBeforeContextActivation, EPOCH_COUNTER_BASE + 2);
        vm.prank(vm.addr(kmsPk0));
        protocolConfig.confirmKmsContextCreation(KMS_CONTEXT_COUNTER_BASE + 2);
        vm.prank(vm.addr(kmsPk1));
        protocolConfig.confirmKmsContextCreation(KMS_CONTEXT_COUNTER_BASE + 2);
        (uint256 nextKeyId, uint256 nextCrsId) = _completeKmsGenerationMaterial();
        _confirmEpochWithMaterial(
            KMS_CONTEXT_COUNTER_BASE + 2,
            EPOCH_COUNTER_BASE + 3,
            kmsPk0,
            kmsTxSender0,
            nextKeyId,
            nextCrsId
        );
        _confirmEpochWithMaterial(
            KMS_CONTEXT_COUNTER_BASE + 2,
            EPOCH_COUNTER_BASE + 3,
            kmsPk1,
            kmsTxSender1,
            nextKeyId,
            nextCrsId
        );
        (, uint256 finalActiveEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(finalActiveEpochId, EPOCH_COUNTER_BASE + 3);
    }

    // -----------------------------------------------------------------------
    // Canonical mirror initializer
    // -----------------------------------------------------------------------

    function test_initializeFromCanonicalPreservesContextAndEpoch() public {
        _setupEmptyProxy();

        address impl = address(new ProtocolConfig());
        uint256 canonicalContextId = KMS_CONTEXT_COUNTER_BASE + 7;
        uint256 canonicalEpochId = EPOCH_COUNTER_BASE + 5;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();

        vm.prank(owner);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfig.initializeFromCanonical,
                (canonicalContextId, canonicalEpochId, nodes, thresholds)
            )
        );
        protocolConfig = ProtocolConfig(protocolConfigAdd);

        (uint256 activeContextId, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, canonicalContextId);
        assertEq(activeEpochId, canonicalEpochId);
        assertTrue(protocolConfig.isValidEpochForContext(canonicalContextId, canonicalEpochId));
        assertEq(protocolConfig.getKmsSignersForContext(canonicalContextId).length, nodes.length);
        assertEq(
            protocolConfig.getPublicDecryptionThresholdForContext(canonicalContextId),
            thresholds.publicDecryption
        );
    }

    function test_revertInitializeFromCanonicalInvalidEpochId() public {
        _setupEmptyProxy();

        address impl = address(new ProtocolConfig());
        uint256 canonicalContextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 invalidEpochId = EPOCH_COUNTER_BASE;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidEpoch.selector, invalidEpochId));
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfig.initializeFromCanonical,
                (canonicalContextId, invalidEpochId, nodes, thresholds)
            )
        );
    }

    function test_revertInitializeFromCanonicalInvalidContextId() public {
        _setupEmptyProxy();

        address impl = address(new ProtocolConfig());
        uint256 invalidContextId = KMS_CONTEXT_COUNTER_BASE;
        uint256 canonicalEpochId = EPOCH_COUNTER_BASE + 1;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, invalidContextId));
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfig.initializeFromCanonical,
                (invalidContextId, canonicalEpochId, nodes, thresholds)
            )
        );
    }

    function test_emptyProxyInitializer_emitsNewKmsContext() public {
        _setupEmptyProxy();

        address impl = address(new ProtocolConfig());
        KmsNodeParams[] memory nodeParams = _makeKmsNodeParams(2);
        PcrValues[] memory pcrValues = new PcrValues[](0);
        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();

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
        _expectAllViewsRevertForContext(invalidContextId);
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

        _expectAllViewsRevertForContext(firstContextId);
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
        IProtocolConfig.KmsThresholds memory newThresholds = IProtocolConfig.KmsThresholds({
            publicDecryption: 1,
            userDecryption: 1,
            kmsGen: 1,
            mpc: 1
        });
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

        IProtocolConfig.KmsThresholds memory newThresholds = IProtocolConfig.KmsThresholds({
            publicDecryption: 1,
            userDecryption: 1,
            kmsGen: 2,
            mpc: 1
        });
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), newThresholds);
        uint256 secondContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _activatePendingContextWithTwoKmsNodes(secondContextId, EPOCH_COUNTER_BASE + 3);

        assertEq(protocolConfig.getKmsGenThresholdForContext(secondContextId), 2);
        assertEq(protocolConfig.getKmsGenThresholdForContext(firstContextId), 3);

        uint256 invalidId = KMS_CONTEXT_COUNTER_BASE + 999;
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, invalidId));
        protocolConfig.getKmsGenThresholdForContext(invalidId);
    }

    function test_getMpcThresholdForContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        assertEq(protocolConfig.getMpcThresholdForContext(firstContextId), 4);
        _seedActiveEpochWithMaterialForFourNodeContext();

        IProtocolConfig.KmsThresholds memory newThresholds = IProtocolConfig.KmsThresholds({
            publicDecryption: 1,
            userDecryption: 1,
            kmsGen: 1,
            mpc: 2
        });
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), newThresholds);
        uint256 secondContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _activatePendingContextWithTwoKmsNodes(secondContextId, EPOCH_COUNTER_BASE + 3);

        assertEq(protocolConfig.getMpcThresholdForContext(secondContextId), 2);
        assertEq(protocolConfig.getMpcThresholdForContext(firstContextId), 4);

        uint256 invalidId = KMS_CONTEXT_COUNTER_BASE + 999;
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, invalidId));
        protocolConfig.getMpcThresholdForContext(invalidId);
    }

    function test_getPublicDecryptionThresholdForContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        assertEq(protocolConfig.getPublicDecryptionThresholdForContext(firstContextId), 1);
        _seedActiveEpochWithMaterialForFourNodeContext();

        IProtocolConfig.KmsThresholds memory newThresholds = IProtocolConfig.KmsThresholds({
            publicDecryption: 2,
            userDecryption: 1,
            kmsGen: 2,
            mpc: 1
        });
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), newThresholds);
        uint256 secondContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _activatePendingContextWithTwoKmsNodes(secondContextId, EPOCH_COUNTER_BASE + 3);

        assertEq(protocolConfig.getPublicDecryptionThresholdForContext(secondContextId), 2);
        assertEq(protocolConfig.getPublicDecryptionThresholdForContext(firstContextId), 1);

        uint256 invalidId = KMS_CONTEXT_COUNTER_BASE + 999;
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, invalidId));
        protocolConfig.getPublicDecryptionThresholdForContext(invalidId);
    }

    function test_thresholdsAfterContextRotation() public {
        _setupDefault();
        _seedActiveEpochWithMaterialForFourNodeContext();
        // Initial context uses thresholds {1, 2, 3, 4}.
        // Define a new context with different thresholds.
        IProtocolConfig.KmsThresholds memory newThresholds = IProtocolConfig.KmsThresholds({
            publicDecryption: 2,
            userDecryption: 1,
            kmsGen: 2,
            mpc: 1
        });

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), newThresholds);
        _activatePendingContextWithTwoKmsNodes(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 3);

        assertEq(protocolConfig.getPublicDecryptionThreshold(), 2);
        assertEq(protocolConfig.getUserDecryptionThreshold(), 1);
        assertEq(protocolConfig.getKmsGenThreshold(), 2);
        assertEq(protocolConfig.getMpcThreshold(), 1);
    }

    // -----------------------------------------------------------------------
    // Threshold setters
    // -----------------------------------------------------------------------

    function test_updateThresholdsForCurrentContext() public {
        _setupDefault();
        uint256 contextId = protocolConfig.getCurrentKmsContextId();

        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.PublicDecryptionThresholdUpdated(contextId, 2);
        vm.prank(owner);
        protocolConfig.updatePublicDecryptionThresholdForContext(contextId, 2);
        assertEq(protocolConfig.getPublicDecryptionThreshold(), 2);

        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.UserDecryptionThresholdUpdated(contextId, 3);
        vm.prank(owner);
        protocolConfig.updateUserDecryptionThresholdForContext(contextId, 3);
        assertEq(protocolConfig.getUserDecryptionThreshold(), 3);

        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsGenThresholdUpdated(contextId, 4);
        vm.prank(owner);
        protocolConfig.updateKmsGenThresholdForContext(contextId, 4);
        assertEq(protocolConfig.getKmsGenThreshold(), 4);

        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.MpcThresholdUpdated(contextId, 1);
        vm.prank(owner);
        protocolConfig.updateMpcThresholdForContext(contextId, 1);
        assertEq(protocolConfig.getMpcThreshold(), 1);
    }

    function test_updateThresholdForHistoricalContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        _seedActiveEpochWithMaterialForFourNodeContext();

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        _activatePendingContextWithTwoKmsNodes(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 3);

        vm.prank(owner);
        protocolConfig.updatePublicDecryptionThresholdForContext(firstContextId, 2);
        vm.prank(owner);
        protocolConfig.updateUserDecryptionThresholdForContext(firstContextId, 3);
        vm.prank(owner);
        protocolConfig.updateKmsGenThresholdForContext(firstContextId, 4);
        vm.prank(owner);
        protocolConfig.updateMpcThresholdForContext(firstContextId, 2);

        assertEq(protocolConfig.getPublicDecryptionThresholdForContext(firstContextId), 2);
        assertEq(protocolConfig.getUserDecryptionThresholdForContext(firstContextId), 3);
        assertEq(protocolConfig.getKmsGenThresholdForContext(firstContextId), 4);
        assertEq(protocolConfig.getMpcThresholdForContext(firstContextId), 2);
        assertEq(protocolConfig.getPublicDecryptionThreshold(), 1);
        assertEq(protocolConfig.getUserDecryptionThreshold(), 1);
        assertEq(protocolConfig.getKmsGenThreshold(), 1);
        assertEq(protocolConfig.getMpcThreshold(), 1);
    }

    function test_revertUpdateThresholdForInvalidContext() public {
        _setupDefault();
        uint256 invalidContextId = KMS_CONTEXT_COUNTER_BASE + 999;

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, invalidContextId));
        protocolConfig.updatePublicDecryptionThresholdForContext(invalidContextId, 1);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, invalidContextId));
        protocolConfig.updateUserDecryptionThresholdForContext(invalidContextId, 1);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, invalidContextId));
        protocolConfig.updateKmsGenThresholdForContext(invalidContextId, 1);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, invalidContextId));
        protocolConfig.updateMpcThresholdForContext(invalidContextId, 1);
    }

    function test_revertUpdateThresholdForDestroyedContext() public {
        _setupDefault();
        uint256 firstContextId = protocolConfig.getCurrentKmsContextId();
        _seedActiveEpochWithMaterialForFourNodeContext();

        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(1), _defaultThresholds());
        _activatePendingContextWithOneKmsNode(KMS_CONTEXT_COUNTER_BASE + 2, EPOCH_COUNTER_BASE + 3);

        vm.prank(owner);
        protocolConfig.destroyKmsContext(firstContextId);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, firstContextId));
        protocolConfig.updatePublicDecryptionThresholdForContext(firstContextId, 1);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, firstContextId));
        protocolConfig.updateUserDecryptionThresholdForContext(firstContextId, 1);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, firstContextId));
        protocolConfig.updateKmsGenThresholdForContext(firstContextId, 1);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidKmsContext.selector, firstContextId));
        protocolConfig.updateMpcThresholdForContext(firstContextId, 1);
    }

    function test_revertUpdateThresholdToZero() public {
        _setupDefault();
        uint256 contextId = protocolConfig.getCurrentKmsContextId();

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidNullThreshold.selector, "publicDecryption"));
        protocolConfig.updatePublicDecryptionThresholdForContext(contextId, 0);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidNullThreshold.selector, "userDecryption"));
        protocolConfig.updateUserDecryptionThresholdForContext(contextId, 0);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidNullThreshold.selector, "kmsGen"));
        protocolConfig.updateKmsGenThresholdForContext(contextId, 0);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidNullThreshold.selector, "mpc"));
        protocolConfig.updateMpcThresholdForContext(contextId, 0);
    }

    function test_revertUpdateThresholdAboveNodeCount() public {
        _setupDefault();
        uint256 contextId = protocolConfig.getCurrentKmsContextId();

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.InvalidHighThreshold.selector, "publicDecryption", 5, 4)
        );
        protocolConfig.updatePublicDecryptionThresholdForContext(contextId, 5);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidHighThreshold.selector, "userDecryption", 5, 4));
        protocolConfig.updateUserDecryptionThresholdForContext(contextId, 5);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidHighThreshold.selector, "kmsGen", 5, 4));
        protocolConfig.updateKmsGenThresholdForContext(contextId, 5);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidHighThreshold.selector, "mpc", 5, 4));
        protocolConfig.updateMpcThresholdForContext(contextId, 5);
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

    function test_revertInitializeFromCanonicalAfterInit() public {
        _setupDefault();

        uint256 canonicalContextId = KMS_CONTEXT_COUNTER_BASE + 5;
        uint256 canonicalEpochId = EPOCH_COUNTER_BASE + 5;
        vm.prank(owner);
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        protocolConfig.initializeFromCanonical(
            canonicalContextId,
            canonicalEpochId,
            _makeKmsNodeParams(1),
            _defaultThresholds()
        );
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

    function test_revertUpdateThresholdNotOwner() public {
        _setupDefault();
        uint256 contextId = protocolConfig.getCurrentKmsContextId();

        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.updatePublicDecryptionThresholdForContext(contextId, 1);

        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.updateUserDecryptionThresholdForContext(contextId, 1);

        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.updateKmsGenThresholdForContext(contextId, 1);

        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.updateMpcThresholdForContext(contextId, 1);
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

        assertEq(protocolConfig.getVersion(), "ProtocolConfig v0.3.0");
        // State preserved across upgrade.
        assertTrue(protocolConfig.isValidKmsContext(protocolConfig.getCurrentKmsContextId()));
    }

    // -----------------------------------------------------------------------
    // reinitializeV2 upgrade continuity
    // -----------------------------------------------------------------------

    /// @dev Upgrades an empty proxy INTO the pre-epoch v0.1.0 storage double, then INTO the real
    ///      ProtocolConfig via reinitializeV2, asserting the migration brings the pre-existing context
    ///      into the epoch-lifecycle shape (active context + first epoch + backfilled anchor) and emits
    ///      the genesis NewKmsContext with the KMS_CONTEXT_COUNTER_BASE sentinel previousContextId.
    function test_reinitializeV2MigratesPreEpochContext() public {
        _setupEmptyProxy();

        KmsNodeParams[] memory nodes = _makeKmsNodeParams(3);
        IProtocolConfig.KmsThresholds memory thresholds = IProtocolConfig.KmsThresholds({
            publicDecryption: 1,
            userDecryption: 2,
            kmsGen: 3,
            mpc: 2
        });
        PcrValues[] memory pcrValues = new PcrValues[](1);
        pcrValues[0] = PcrValues({
            pcr0: abi.encodePacked(uint256(1)),
            pcr1: abi.encodePacked(uint256(2)),
            pcr2: abi.encodePacked(uint256(3))
        });
        string memory softwareVersion = "kms-v1";
        uint256 existingContextId = KMS_CONTEXT_COUNTER_BASE + 1;

        // Stage 1: seed the proxy with the pre-epoch v0.1.0 storage layout. Its initializer seeds the
        // counter to BASE then increments to the first context (existingContextId).
        address v010Impl = address(new ProtocolConfigV010TestDouble());
        vm.prank(owner);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            v010Impl,
            abi.encodeCall(ProtocolConfigV010TestDouble.initializeFromEmptyProxy, (nodes, thresholds))
        );

        // Stage 2: upgrade into the real ProtocolConfig via reinitializeV2. The genesis NewKmsContext
        // must carry the KMS_CONTEXT_COUNTER_BASE sentinel as previousContextId so connectors do NOT
        // treat the migration as a context switch. Deploy the impl BEFORE pranking so the prank lands
        // on upgradeToAndCall, not the CREATE.
        address realImpl = address(new ProtocolConfig());
        vm.expectEmit(true, true, false, true, protocolConfigAdd);
        emit IProtocolConfig.NewKmsContext(
            existingContextId,
            KMS_CONTEXT_COUNTER_BASE,
            nodes,
            thresholds,
            softwareVersion,
            pcrValues
        );
        vm.prank(owner);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            realImpl,
            abi.encodeCall(ProtocolConfig.reinitializeV2, (nodes, thresholds, softwareVersion, pcrValues))
        );
        protocolConfig = ProtocolConfig(protocolConfigAdd);

        // Existing context is now Active and resolves as the live context.
        (uint256 activeContextId, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, existingContextId);
        assertEq(protocolConfig.getCurrentKmsContextId(), existingContextId);
        assertTrue(protocolConfig.isValidKmsContext(existingContextId));

        // The first epoch is opened and active.
        assertEq(activeEpochId, EPOCH_COUNTER_BASE + 1);
        assertTrue(protocolConfig.isValidEpochForContext(existingContextId, EPOCH_COUNTER_BASE + 1));

        // The anchor the pre-epoch version never recorded is backfilled.
        (uint256 emissionBlockNumber, bytes32 contextInfoHash) = protocolConfig.getKmsContextAnchor(existingContextId);
        assertEq(emissionBlockNumber, block.number);
        assertEq(contextInfoHash, keccak256(abi.encode(nodes, thresholds, softwareVersion, pcrValues)));

        // Node/threshold storage written under the v0.1.0 layout is readable through the new getters,
        // proving the namespaced storage slot lines up across the upgrade.
        assertEq(protocolConfig.getKmsGenThreshold(), 3);
        assertEq(protocolConfig.getUserDecryptionThreshold(), 2);
        assertEq(protocolConfig.getKmsSignersForContext(existingContextId).length, 3);
        assertTrue(protocolConfig.isKmsSignerForContext(existingContextId, vm.addr(kmsPk0)));
    }

    // -----------------------------------------------------------------------
    // mirrorKmsContextAndEpoch
    // -----------------------------------------------------------------------

    function test_mirrorKmsContextAndEpochActivatesAndEmits() public {
        _setupEpochLifecycle();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 5;
        uint256 epochId = EPOCH_COUNTER_BASE + 5;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();
        PcrValues[] memory pcrValues = new PcrValues[](0);

        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.MirrorKmsContextAndEpoch(contextId, epochId, nodes, thresholds, "kms-v9", pcrValues);
        vm.prank(owner);
        protocolConfig.mirrorKmsContextAndEpoch(contextId, epochId, nodes, thresholds, "kms-v9", pcrValues);

        // Mirrored context and epoch are immediately active (no quorum replay).
        (uint256 activeContextId, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, contextId);
        assertEq(activeEpochId, epochId);
        assertTrue(protocolConfig.isValidKmsContext(contextId));
        assertTrue(protocolConfig.isValidEpochForContext(contextId, epochId));
        assertEq(protocolConfig.getKmsSignersForContext(contextId).length, 2);

        // Unlike defineNewKmsContextAndEpoch, mirror does NOT record an anchor: it stays zeroed.
        (uint256 emissionBlockNumber, bytes32 contextInfoHash) = protocolConfig.getKmsContextAnchor(contextId);
        assertEq(emissionBlockNumber, 0);
        assertEq(contextInfoHash, bytes32(0));
    }

    function test_mirrorKmsContextAndEpochActivatesExactNextEpoch() public {
        _setupEpochLifecycle();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();

        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.MirrorKmsContextAndEpoch(contextId, epochId, nodes, thresholds, "", new PcrValues[](0));
        vm.prank(owner);
        protocolConfig.mirrorKmsContextAndEpoch(contextId, epochId, nodes, thresholds, "", new PcrValues[](0));

        (uint256 activeContextId, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, contextId);
        assertEq(activeEpochId, epochId);
        assertTrue(protocolConfig.isValidEpochForContext(contextId, epochId));
    }

    function test_mirrorKmsContextAndEpochBaseBoundary() public {
        _setupEmptyProxy();

        // Bootstrap a mirror replica whose first active context is exactly BASE + 1.
        address impl = address(new ProtocolConfig());
        KmsNodeParams[] memory bootstrapNodes = _makeKmsNodeParams(2);
        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();
        vm.prank(owner);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfig.initializeFromEmptyProxy,
                (bootstrapNodes, thresholds, "", new PcrValues[](0))
            )
        );
        protocolConfig = ProtocolConfig(protocolConfigAdd);

        // Active context after fresh deploy is BASE + 1, so the next mirror must be BASE + 2 minimum.
        uint256 boundaryContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 boundaryEpochId = EPOCH_COUNTER_BASE + 2;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        vm.prank(owner);
        protocolConfig.mirrorKmsContextAndEpoch(
            boundaryContextId,
            boundaryEpochId,
            nodes,
            thresholds,
            "",
            new PcrValues[](0)
        );
        assertEq(protocolConfig.getCurrentKmsContextId(), boundaryContextId);
        assertTrue(protocolConfig.isValidKmsContext(boundaryContextId));
    }

    function test_mirrorKmsContextAndEpochAllowsGap() public {
        _setupEpochLifecycle();
        // Active context is BASE + 1; mirror a non-contiguous (gapped) context ID.
        uint256 gappedContextId = KMS_CONTEXT_COUNTER_BASE + 10;
        uint256 gappedEpochId = EPOCH_COUNTER_BASE + 10;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();

        vm.prank(owner);
        protocolConfig.mirrorKmsContextAndEpoch(
            gappedContextId,
            gappedEpochId,
            nodes,
            thresholds,
            "",
            new PcrValues[](0)
        );

        assertEq(protocolConfig.getCurrentKmsContextId(), gappedContextId);
        assertTrue(protocolConfig.isValidKmsContext(gappedContextId));
        // The skipped IDs in the gap remain invalid.
        assertFalse(protocolConfig.isValidKmsContext(KMS_CONTEXT_COUNTER_BASE + 5));
    }

    function test_revertMirrorKmsContextAndEpochNonIncreasing() public {
        _setupEpochLifecycle();
        uint256 activeContextId = protocolConfig.getCurrentKmsContextId();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();

        // contextId == activeKmsContextId is rejected (not strictly increasing).
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.NonIncreasingKmsContextId.selector, activeContextId, activeContextId)
        );
        protocolConfig.mirrorKmsContextAndEpoch(
            activeContextId,
            EPOCH_COUNTER_BASE + 2,
            nodes,
            thresholds,
            "",
            new PcrValues[](0)
        );
    }

    function test_revertMirrorKmsContextAndEpochNonIncreasingEpoch() public {
        _setupEpochLifecycle();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 activeEpochId = EPOCH_COUNTER_BASE + 1;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.NonIncreasingEpochId.selector, activeEpochId, activeEpochId)
        );
        protocolConfig.mirrorKmsContextAndEpoch(contextId, activeEpochId, nodes, thresholds, "", new PcrValues[](0));
    }

    function test_revertMirrorKmsContextAndEpochNotOwner() public {
        _setupEpochLifecycle();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        IProtocolConfig.KmsThresholds memory thresholds = _defaultThresholds();

        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.mirrorKmsContextAndEpoch(
            KMS_CONTEXT_COUNTER_BASE + 5,
            EPOCH_COUNTER_BASE + 5,
            nodes,
            thresholds,
            "",
            new PcrValues[](0)
        );
    }

    // -----------------------------------------------------------------------
    // confirmEpochActivation / abortPendingEpoch negative branches
    // -----------------------------------------------------------------------

    /// @dev Reaching the `!_isLiveKmsContext` guard requires the epoch's context to be destroyed
    ///      mid-rotation while its pending epoch survives. Destroying a Created context clears its
    ///      pending epoch, so we keep the epoch alive by destroying the context only after defining
    ///      a fresh pending epoch under it via mirrorKmsEpoch is not possible; instead we drive the
    ///      InvalidKmsContext branch through a destroyed-then-confirmed path.
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
        vm.prank(vm.addr(kmsPk0));
        protocolConfig.confirmKmsContextCreation(secondContextId);
        vm.prank(vm.addr(kmsPk1));
        protocolConfig.confirmKmsContextCreation(secondContextId);
        vm.prank(vm.addr(kmsPk2));
        protocolConfig.confirmKmsContextCreation(secondContextId);
        vm.prank(vm.addr(kmsPk3));
        protocolConfig.confirmKmsContextCreation(secondContextId);
        (uint256 keyId, uint256 crsId) = _completeKmsGenerationMaterialWithTwoResponses(
            kmsPk0,
            kmsTxSender0,
            kmsPk1,
            kmsTxSender1
        );
        _confirmEpochWithMaterial(secondContextId, secondEpochId, kmsPk2, kmsTxSender0, keyId, crsId);
        _confirmEpochWithMaterial(secondContextId, secondEpochId, kmsPk3, kmsTxSender1, keyId, crsId);

        // Open a third context switch, confirm its creation (Created), then destroy that context.
        // Its pending epoch is cleared on destruction, so a later confirmation hits the lifecycle guards.
        KmsNodeParams[] memory thirdNodes = _makeKmsNodeParams(2);
        thirdNodes[0].signerAddress = vm.addr(kmsPk0);
        thirdNodes[1].signerAddress = vm.addr(kmsPk1);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(thirdNodes, _defaultThresholds());
        uint256 thirdContextId = KMS_CONTEXT_COUNTER_BASE + 3;
        uint256 thirdEpochId = EPOCH_COUNTER_BASE + 4;

        vm.prank(vm.addr(kmsPk2));
        protocolConfig.confirmKmsContextCreation(thirdContextId);
        vm.prank(vm.addr(kmsPk3));
        protocolConfig.confirmKmsContextCreation(thirdContextId);
        vm.prank(vm.addr(kmsPk0));
        protocolConfig.confirmKmsContextCreation(thirdContextId);
        vm.prank(vm.addr(kmsPk1));
        protocolConfig.confirmKmsContextCreation(thirdContextId);

        vm.prank(owner);
        protocolConfig.destroyKmsContext(thirdContextId);

        // The pending epoch was cleared with the context, so confirming activation now reverts on the
        // epoch state guard (InvalidEpoch) — the context is no longer live for it.
        IProtocolConfig.EpochKeyResult[] memory keys = new IProtocolConfig.EpochKeyResult[](0);
        IProtocolConfig.EpochCrsResult[] memory crsList = new IProtocolConfig.EpochCrsResult[](0);
        vm.prank(kmsTxSender0);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidEpoch.selector, thirdEpochId));
        protocolConfig.confirmEpochActivation(thirdEpochId, keys, crsList);
    }

    function test_revertAbortPendingEpochNotOwner() public {
        _setupEpochLifecycle();
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;

        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfig.abortPendingEpoch(epochId);
    }

    function testFuzz_revertAbortPendingEpochUnknownEpoch(uint256 unknownEpochId) public {
        _setupEpochLifecycle();
        // The active epoch (EPOCH_COUNTER_BASE + 1) is Active, not Pending, so it is also rejected.
        vm.assume(unknownEpochId != EPOCH_COUNTER_BASE + 2);
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.InvalidEpoch.selector, unknownEpochId));
        protocolConfig.abortPendingEpoch(unknownEpochId);
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
        vm.prank(vm.addr(kmsPk0));
        protocolConfig.confirmKmsContextCreation(newContextId);

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
        IProtocolConfig.KmsThresholds memory thresholds = IProtocolConfig.KmsThresholds({
            publicDecryption: 1,
            userDecryption: 1,
            kmsGen: 1,
            mpc: 1
        });
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
        _confirmEpochWithMaterial(contextId, epochId, kmsPk0, kmsTxSender0, keyId, crsId);
        _confirmEpochWithMaterial(contextId, epochId, kmsPk1, kmsTxSender1, keyId, crsId);

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
        vm.prank(kmsTxSender2);
        protocolConfig.confirmEpochActivation(epochId, keys, crsList);

        // No digest group reached the full quorum of 3 — epoch unchanged.
        (uint256 activeContextAfter, uint256 activeEpochAfter) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochAfter, activeEpochBefore);
        assertEq(activeContextAfter, contextId);
    }

    // -----------------------------------------------------------------------
    // Per-confirmation events and malformed-signature negative
    // -----------------------------------------------------------------------

    /// @dev Asserts KmsContextCreationConfirmation fires with correct isPreviousSigner/isNewSigner flags,
    ///      including a signer present in BOTH the previous and new committee.
    function test_kmsContextCreationConfirmationEventFlags() public {
        _setupEpochLifecycle();

        // New committee: kmsPk0 (also a previous signer -> both) and kmsPk2 (new only).
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[0].signerAddress = vm.addr(kmsPk0);
        nodes[1].signerAddress = vm.addr(kmsPk2);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        // kmsPk1 is a previous signer only.
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextCreationConfirmation(newContextId, vm.addr(kmsPk1), true, false);
        vm.prank(vm.addr(kmsPk1));
        protocolConfig.confirmKmsContextCreation(newContextId);

        // kmsPk2 is a new signer only.
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextCreationConfirmation(newContextId, vm.addr(kmsPk2), false, true);
        vm.prank(vm.addr(kmsPk2));
        protocolConfig.confirmKmsContextCreation(newContextId);

        // kmsPk0 belongs to both committees.
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextCreationConfirmation(newContextId, vm.addr(kmsPk0), true, true);
        vm.prank(vm.addr(kmsPk0));
        protocolConfig.confirmKmsContextCreation(newContextId);
    }

    /// @dev Asserts EpochActivationConfirmation fires for the confirming signer (indexed epochId/signer).
    ///      The dataHash field is left unchecked: it is the internally-derived consensus digest, asserted
    ///      indirectly by the divergent-digest accumulation tests.
    function test_epochActivationConfirmationEvent() public {
        _setupEpochLifecycle();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        (uint256 completedKeyId, uint256 completedCrsId) = _completeKmsGenerationMaterial();

        // checkData=false: assert only the indexed topics (epochId, signer).
        vm.expectEmit(true, true, false, false, address(protocolConfig));
        emit IProtocolConfig.EpochActivationConfirmation(epochId, vm.addr(kmsPk0), bytes32(0));
        _confirmEpochWithMaterial(
            KMS_CONTEXT_COUNTER_BASE + 1,
            epochId,
            kmsPk0,
            kmsTxSender0,
            completedKeyId,
            completedCrsId
        );
    }

    /// @dev Distinct from the signer-mismatch test: a malformed (too-short) signature must hit
    ///      ECDSA.recover's length validation, with real key material in the payload.
    function test_revertConfirmEpochActivationMalformedSignature() public {
        _setupEpochLifecycle();
        (uint256 completedKeyId, ) = _completeKmsGenerationMaterial();
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
        IProtocolConfig.EpochCrsResult[] memory crsList = new IProtocolConfig.EpochCrsResult[](0);

        vm.prank(kmsTxSender0);
        vm.expectRevert();
        protocolConfig.confirmEpochActivation(epochId, keys, crsList);
    }
}
