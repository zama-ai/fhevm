// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {protocolConfigAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";
import {KMSVerifier} from "@fhevm-host-contracts/contracts/KMSVerifier.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {ProtocolConfigMultichain} from "@fhevm-host-contracts/contracts/ProtocolConfigMultichain.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {IProtocolConfigCommon} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigCommon.sol";
import {IProtocolConfigMultichain} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigMultichain.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";
import {KmsNode, KmsNodeParams, PcrValues} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {EPOCH_COUNTER_BASE, KMS_CONTEXT_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {ProtocolConfigV010TestDouble} from "./ProtocolConfigV010TestDouble.sol";

contract ProtocolConfigMultichainTest is HostContractsDeployerTestUtils {
    ProtocolConfigMultichain internal protocolConfig;
    KMSVerifier internal kmsVerifier;

    address internal constant owner = address(456);
    address internal constant canonicalProtocolConfig = address(0xC0FFEE);
    address internal constant verifyingContractSource = address(10000);
    uint256 internal constant sourceChainId = 1;
    uint256 internal constant sourceBlockNumber = 12_345_678;

    uint256 internal constant privateKeySigner0 = 0x100;
    uint256 internal constant privateKeySigner1 = 0x200;
    uint256 internal constant privateKeySigner2 = 0x300;
    uint256 internal constant privateKeySigner3 = 0x400;

    function setUp() public {
        _deployACL(owner);
    }

    function _source(
        uint256 blockNumber,
        address protocolConfig_
    ) internal pure returns (IProtocolConfigMultichain.MirroredContextSource memory) {
        return
            IProtocolConfigMultichain.MirroredContextSource({
                sourceChainId: sourceChainId,
                sourceBlockNumber: blockNumber,
                sourceProtocolConfig: protocolConfig_
            });
    }

    function _deployEmptyProtocolConfigProxy() internal {
        address emptyProxyImpl = address(new EmptyUUPSProxy());
        deployCodeTo(
            "fhevm-foundry/HostContractsDeployerTestUtils.sol:DeployableERC1967Proxy",
            abi.encode(emptyProxyImpl, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            protocolConfigAdd
        );
    }

    function _deployProtocolConfigMultichain(
        uint256 initialContextId,
        KmsNodeParams[] memory nodes,
        IProtocolConfigCommon.KmsThresholds memory thresholds
    ) internal {
        _deployEmptyProtocolConfigProxy();
        address impl = address(new ProtocolConfigMultichain());
        vm.prank(owner);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfigMultichain.initializeFromEmptyProxy,
                (
                    initialContextId,
                    nodes,
                    thresholds,
                    "kms-v1",
                    new PcrValues[](0),
                    _source(sourceBlockNumber, canonicalProtocolConfig)
                )
            )
        );
        protocolConfig = ProtocolConfigMultichain(protocolConfigAdd);
    }

    function _deployVerifier() internal {
        (kmsVerifier, ) = _deployKMSVerifier(owner, verifyingContractSource, uint64(block.chainid));
    }

    function _deployLegacyProtocolConfig(
        KmsNodeParams[] memory nodes,
        IProtocolConfigCommon.KmsThresholds memory thresholds
    ) internal returns (ProtocolConfigV010TestDouble legacy) {
        _deployEmptyProtocolConfigProxy();
        address impl = address(new ProtocolConfigV010TestDouble());
        vm.prank(owner);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfigV010TestDouble.initializeFromEmptyProxy,
                (nodes, thresholds, "kms-v1", new PcrValues[](0))
            )
        );
        legacy = ProtocolConfigV010TestDouble(protocolConfigAdd);
    }

    function _thresholds(
        uint256 publicDecryption,
        uint256 userDecryption,
        uint256 kmsGen,
        uint256 mpc
    ) internal pure returns (IProtocolConfigCommon.KmsThresholds memory) {
        return
            IProtocolConfigCommon.KmsThresholds({
                publicDecryption: publicDecryption,
                userDecryption: userDecryption,
                kmsGen: kmsGen,
                mpc: mpc
            });
    }

    function _defaultCommonThresholds() internal pure returns (IProtocolConfigCommon.KmsThresholds memory) {
        return _thresholds(1, 1, 1, 1);
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

    function _handles() internal pure returns (bytes32[] memory handlesList) {
        handlesList = new bytes32[](1);
        handlesList[0] = bytes32(uint256(1));
    }

    function _decryptedResult() internal pure returns (bytes memory) {
        return abi.encodePacked(keccak256("multichain"));
    }

    function _proof(
        uint256 signerKey,
        bytes memory extraData
    ) internal view returns (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) {
        handlesList = _handles();
        decryptedResult = _decryptedResult();
        bytes32 digest = _computeDigest(handlesList, decryptedResult, extraData);
        proof = abi.encodePacked(uint8(1), _computeSignature(signerKey, digest), extraData);
    }

    function test_InitializeFromCanonicalSnapshotUsesExactContextId() public {
        uint256 canonicalContextId = KMS_CONTEXT_COUNTER_BASE + 7;
        _deployProtocolConfigMultichain(canonicalContextId, _makeKmsNodeParams(2), _thresholds(1, 2, 2, 1));

        assertEq(protocolConfig.getVersion(), "ProtocolConfigMultichain v0.2.0");
        assertEq(protocolConfig.getCurrentKmsContextId(), canonicalContextId);
        assertTrue(protocolConfig.isValidKmsContext(canonicalContextId));
        assertEq(protocolConfig.getKmsSignersForContext(canonicalContextId).length, 2);
        assertEq(protocolConfig.getUserDecryptionThreshold(), 2);
        assertFalse(protocolConfig.isValidKmsContext(KMS_CONTEXT_COUNTER_BASE + 1));

        IProtocolConfigMultichain.MirroredContextSource memory stored = protocolConfig.getMirroredContextSource(
            canonicalContextId
        );
        assertEq(stored.sourceChainId, sourceChainId);
        assertEq(stored.sourceBlockNumber, sourceBlockNumber);
        assertEq(stored.sourceProtocolConfig, canonicalProtocolConfig);
    }

    function test_MirrorAllowsCanonicalContextGapsAndKeepsOlderContextsReadable() public {
        uint256 contextN = KMS_CONTEXT_COUNTER_BASE + 10;
        uint256 contextNPlusTwo = contextN + 2;
        _deployProtocolConfigMultichain(contextN, _makeKmsNodeParams(1), _defaultCommonThresholds());

        KmsNodeParams[] memory nextNodes = _makeKmsNodeParamsFromSigners(_singleSigner(vm.addr(privateKeySigner3)));
        IProtocolConfigCommon.KmsThresholds memory nextThresholds = _defaultCommonThresholds();
        PcrValues[] memory nextPcrValues = new PcrValues[](0);
        vm.expectEmit(true, true, true, true);
        emit IProtocolConfigMultichain.MirrorKmsContext(
            contextNPlusTwo,
            nextNodes,
            nextThresholds,
            "kms-v2",
            nextPcrValues,
            sourceChainId,
            sourceBlockNumber + 1,
            canonicalProtocolConfig
        );
        vm.prank(owner);
        protocolConfig.mirrorKmsContext(
            contextNPlusTwo,
            nextNodes,
            nextThresholds,
            "kms-v2",
            nextPcrValues,
            _source(sourceBlockNumber + 1, canonicalProtocolConfig)
        );

        assertEq(protocolConfig.getCurrentKmsContextId(), contextNPlusTwo);
        assertEq(protocolConfig.getKmsSignersForContext(contextN)[0], vm.addr(privateKeySigner0));
        assertEq(protocolConfig.getKmsSignersForContext(contextNPlusTwo)[0], vm.addr(privateKeySigner3));
        assertFalse(protocolConfig.isValidKmsContext(contextN + 1));

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigCommon.InvalidKmsContext.selector, contextN + 1));
        protocolConfig.getKmsSignersForContext(contextN + 1);
    }

    function test_MirrorRejectsNonIncreasingCanonicalContextIds() public {
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _deployProtocolConfigMultichain(contextId, _makeKmsNodeParams(1), _defaultCommonThresholds());

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigMultichain.NonIncreasingKmsContextId.selector, contextId, contextId)
        );
        protocolConfig.mirrorKmsContext(
            contextId,
            _makeKmsNodeParams(1),
            _defaultCommonThresholds(),
            "kms-v1",
            new PcrValues[](0),
            _source(sourceBlockNumber, canonicalProtocolConfig)
        );
    }

    function test_MirrorThresholdsAppliesExplicitCanonicalContextOnly() public {
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 4;
        _deployProtocolConfigMultichain(contextId, _makeKmsNodeParams(3), _defaultCommonThresholds());

        vm.startPrank(owner);
        vm.expectEmit(true, false, false, true);
        emit IProtocolConfigMultichain.MirrorPublicDecryptionThreshold(contextId, 2);
        protocolConfig.mirrorPublicDecryptionThreshold(contextId, 2);

        vm.expectEmit(true, false, false, true);
        emit IProtocolConfigMultichain.MirrorUserDecryptionThreshold(contextId, 3);
        protocolConfig.mirrorUserDecryptionThreshold(contextId, 3);

        vm.expectEmit(true, false, false, true);
        emit IProtocolConfigMultichain.MirrorKmsGenThreshold(contextId, 2);
        protocolConfig.mirrorKmsGenThreshold(contextId, 2);

        vm.expectEmit(true, false, false, true);
        emit IProtocolConfigMultichain.MirrorMpcThreshold(contextId, 1);
        protocolConfig.mirrorMpcThreshold(contextId, 1);
        vm.stopPrank();

        assertEq(protocolConfig.getPublicDecryptionThresholdForContext(contextId), 2);
        assertEq(protocolConfig.getUserDecryptionThresholdForContext(contextId), 3);
        assertEq(protocolConfig.getKmsGenThresholdForContext(contextId), 2);
        assertEq(protocolConfig.getMpcThresholdForContext(contextId), 1);

        uint256 unknownContext = contextId + 1;
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigCommon.InvalidKmsContext.selector, unknownContext));
        protocolConfig.mirrorMpcThreshold(unknownContext, 1);
    }

    function test_MirrorThresholdsRejectInvalidInputs() public {
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 4;
        _deployProtocolConfigMultichain(contextId, _makeKmsNodeParams(3), _defaultCommonThresholds());

        uint256 unknownContext = contextId + 1;
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigCommon.InvalidKmsContext.selector, unknownContext));
        protocolConfig.mirrorPublicDecryptionThreshold(unknownContext, 1);

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigCommon.InvalidNullThreshold.selector, "publicDecryption")
        );
        protocolConfig.mirrorPublicDecryptionThreshold(contextId, 0);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigCommon.InvalidNullThreshold.selector, "userDecryption"));
        protocolConfig.mirrorUserDecryptionThreshold(contextId, 0);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigCommon.InvalidNullThreshold.selector, "kmsGen"));
        protocolConfig.mirrorKmsGenThreshold(contextId, 0);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigCommon.InvalidNullThreshold.selector, "mpc"));
        protocolConfig.mirrorMpcThreshold(contextId, 0);

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigCommon.InvalidHighThreshold.selector, "publicDecryption", 4, 3)
        );
        protocolConfig.mirrorPublicDecryptionThreshold(contextId, 4);

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigCommon.InvalidHighThreshold.selector, "userDecryption", 4, 3)
        );
        protocolConfig.mirrorUserDecryptionThreshold(contextId, 4);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigCommon.InvalidHighThreshold.selector, "kmsGen", 4, 3));
        protocolConfig.mirrorKmsGenThreshold(contextId, 4);

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigCommon.InvalidHighThreshold.selector, "mpc", 4, 3));
        protocolConfig.mirrorMpcThreshold(contextId, 4);
    }

    function test_MirrorMethodsAreOwnerOnly() public {
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 4;
        uint256 nextContextId = contextId + 1;
        address caller = address(0x999);
        _deployProtocolConfigMultichain(contextId, _makeKmsNodeParams(3), _defaultCommonThresholds());

        vm.startPrank(caller);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, caller));
        protocolConfig.mirrorMpcThreshold(contextId, 2);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, caller));
        protocolConfig.mirrorKmsContext(
            nextContextId,
            _makeKmsNodeParams(3),
            _defaultCommonThresholds(),
            "kms-v2",
            new PcrValues[](0),
            _source(sourceBlockNumber + 1, canonicalProtocolConfig)
        );
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, caller));
        protocolConfig.mirrorKmsContextDestruction(contextId, _source(sourceBlockNumber + 1, canonicalProtocolConfig));
        vm.stopPrank();
    }

    function test_MirrorContextRejectsZeroSourceProtocolConfig() public {
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 4;
        uint256 nextContextId = contextId + 1;
        _deployProtocolConfigMultichain(contextId, _makeKmsNodeParams(3), _defaultCommonThresholds());

        vm.prank(owner);
        vm.expectRevert(IProtocolConfigMultichain.InvalidSourceProtocolConfig.selector);
        protocolConfig.mirrorKmsContext(
            nextContextId,
            _makeKmsNodeParams(3),
            _defaultCommonThresholds(),
            "kms-v2",
            new PcrValues[](0),
            _source(sourceBlockNumber + 1, address(0))
        );
    }

    function test_MirrorContextDestructionInvalidatesContextAndEmitsProvenance() public {
        uint256 contextN = KMS_CONTEXT_COUNTER_BASE + 10;
        uint256 contextNPlusTwo = contextN + 2;
        _deployProtocolConfigMultichain(contextN, _makeKmsNodeParams(1), _defaultCommonThresholds());

        vm.prank(owner);
        protocolConfig.mirrorKmsContext(
            contextNPlusTwo,
            _makeKmsNodeParamsFromSigners(_singleSigner(vm.addr(privateKeySigner3))),
            _defaultCommonThresholds(),
            "kms-v2",
            new PcrValues[](0),
            _source(sourceBlockNumber + 1, canonicalProtocolConfig)
        );

        vm.expectEmit(true, true, true, true);
        emit IProtocolConfigMultichain.MirrorKmsContextDestroyed(
            contextN,
            sourceChainId,
            sourceBlockNumber + 2,
            canonicalProtocolConfig
        );
        vm.prank(owner);
        protocolConfig.mirrorKmsContextDestruction(contextN, _source(sourceBlockNumber + 2, canonicalProtocolConfig));

        assertFalse(protocolConfig.isValidKmsContext(contextN));
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigCommon.InvalidKmsContext.selector, contextN));
        protocolConfig.getKmsSignersForContext(contextN);

        assertTrue(protocolConfig.isValidKmsContext(contextNPlusTwo));
    }

    function test_MirrorContextDestructionRejectsZeroSourceProtocolConfig() public {
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 4;
        _deployProtocolConfigMultichain(contextId, _makeKmsNodeParams(3), _defaultCommonThresholds());

        vm.prank(owner);
        vm.expectRevert(IProtocolConfigMultichain.InvalidSourceProtocolConfig.selector);
        protocolConfig.mirrorKmsContextDestruction(contextId, _source(sourceBlockNumber + 1, address(0)));
    }

    function test_MirrorContextDestructionRejectsCurrentContext() public {
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 4;
        _deployProtocolConfigMultichain(contextId, _makeKmsNodeParams(3), _defaultCommonThresholds());

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigCommon.CurrentKmsContextCannotBeDestroyed.selector, contextId)
        );
        protocolConfig.mirrorKmsContextDestruction(contextId, _source(sourceBlockNumber + 1, canonicalProtocolConfig));
    }

    function test_KMSVerifierUsesMirroredContextsForV0V1AndV2ExtraData() public {
        uint256 contextN = KMS_CONTEXT_COUNTER_BASE + 10;
        uint256 contextNPlusTwo = contextN + 2;
        _deployProtocolConfigMultichain(contextN, _makeKmsNodeParams(1), _defaultCommonThresholds());

        vm.prank(owner);
        protocolConfig.mirrorKmsContext(
            contextNPlusTwo,
            _makeKmsNodeParamsFromSigners(_singleSigner(vm.addr(privateKeySigner3))),
            _defaultCommonThresholds(),
            "kms-v2",
            new PcrValues[](0),
            _source(sourceBlockNumber + 1, canonicalProtocolConfig)
        );
        _deployVerifier();

        (bytes32[] memory handlesList, bytes memory decryptedResult, bytes memory proof) = _proof(
            privateKeySigner3,
            hex"00"
        );
        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof));

        (handlesList, decryptedResult, proof) = _proof(privateKeySigner0, abi.encodePacked(uint8(0x01), contextN));
        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof));

        (handlesList, decryptedResult, proof) = _proof(
            privateKeySigner3,
            abi.encodePacked(uint8(0x02), contextNPlusTwo, uint256(77))
        );
        assertTrue(kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof));

        (handlesList, decryptedResult, proof) = _proof(privateKeySigner0, abi.encodePacked(uint8(0x01), contextN + 1));
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigCommon.InvalidKmsContext.selector, contextN + 1));
        kmsVerifier.verifyDecryptionEIP712KMSSignatures(handlesList, decryptedResult, proof);
    }

    function test_UpgradeLegacyV010ToEthereumProtocolConfigSeedsLifecycleState() public {
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        IProtocolConfigCommon.KmsThresholds memory thresholds = IProtocolConfigCommon.KmsThresholds({
            publicDecryption: 1,
            userDecryption: 2,
            kmsGen: 2,
            mpc: 1
        });
        ProtocolConfigV010TestDouble legacy = _deployLegacyProtocolConfig(nodes, thresholds);
        uint256 legacyContextId = legacy.getCurrentKmsContextId();
        address[] memory legacySigners = legacy.getKmsSignersForContext(legacyContextId);
        KmsNode[] memory legacyNodes = legacy.getKmsNodesForContext(legacyContextId);
        uint256 legacyPublicDecryptionThreshold = legacy.getPublicDecryptionThreshold();
        uint256 legacyUserDecryptionThreshold = legacy.getUserDecryptionThreshold();
        uint256 legacyKmsGenThreshold = legacy.getKmsGenThreshold();
        uint256 legacyMpcThreshold = legacy.getMpcThreshold();

        address impl = address(new ProtocolConfig());
        vm.prank(owner);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(ProtocolConfig.reinitializeV2, (nodes, thresholds, "kms-v1", new PcrValues[](0)))
        );

        ProtocolConfig canonical = ProtocolConfig(protocolConfigAdd);
        (uint256 activeContextId, uint256 activeEpochId) = canonical.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, legacyContextId);
        assertEq(activeEpochId, EPOCH_COUNTER_BASE + 1);
        assertTrue(canonical.isValidKmsContext(legacyContextId));
        assertTrue(canonical.isValidEpochForContext(legacyContextId, activeEpochId));

        address[] memory upgradedSigners = canonical.getKmsSignersForContext(legacyContextId);
        assertEq(upgradedSigners.length, legacySigners.length);
        for (uint256 i = 0; i < legacySigners.length; i++) {
            assertEq(upgradedSigners[i], legacySigners[i]);
        }

        KmsNode[] memory upgradedNodes = canonical.getKmsNodesForContext(legacyContextId);
        assertEq(upgradedNodes.length, legacyNodes.length);
        for (uint256 i = 0; i < legacyNodes.length; i++) {
            assertEq(upgradedNodes[i].txSenderAddress, legacyNodes[i].txSenderAddress);
            assertEq(upgradedNodes[i].signerAddress, legacyNodes[i].signerAddress);
            assertEq(upgradedNodes[i].ipAddress, legacyNodes[i].ipAddress);
            assertEq(upgradedNodes[i].storageUrl, legacyNodes[i].storageUrl);
        }

        assertEq(canonical.getPublicDecryptionThreshold(), legacyPublicDecryptionThreshold);
        assertEq(canonical.getUserDecryptionThreshold(), legacyUserDecryptionThreshold);
        assertEq(canonical.getKmsGenThreshold(), legacyKmsGenThreshold);
        assertEq(canonical.getMpcThreshold(), legacyMpcThreshold);
    }

    function test_UpgradeLegacyV010ToProtocolConfigMultichainRejectsZeroSourceProtocolConfig() public {
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        IProtocolConfigCommon.KmsThresholds memory legacyThresholds = IProtocolConfigCommon.KmsThresholds({
            publicDecryption: 1,
            userDecryption: 2,
            kmsGen: 2,
            mpc: 1
        });
        _deployLegacyProtocolConfig(nodes, legacyThresholds);

        address impl = address(new ProtocolConfigMultichain());
        vm.prank(owner);
        vm.expectRevert(IProtocolConfigMultichain.InvalidSourceProtocolConfig.selector);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(ProtocolConfigMultichain.reinitializeV2, (_source(sourceBlockNumber, address(0))))
        );
    }

    function test_UpgradeLegacyV010ToProtocolConfigMultichainKeepsContextStorage() public {
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        IProtocolConfigCommon.KmsThresholds memory legacyThresholds = IProtocolConfigCommon.KmsThresholds({
            publicDecryption: 1,
            userDecryption: 2,
            kmsGen: 2,
            mpc: 1
        });
        ProtocolConfigV010TestDouble legacy = _deployLegacyProtocolConfig(nodes, legacyThresholds);
        uint256 legacyContextId = legacy.getCurrentKmsContextId();
        address[] memory legacySigners = legacy.getKmsSignersForContext(legacyContextId);
        KmsNode[] memory legacyNodes = legacy.getKmsNodesForContext(legacyContextId);
        uint256 legacyPublicDecryptionThreshold = legacy.getPublicDecryptionThreshold();
        uint256 legacyUserDecryptionThreshold = legacy.getUserDecryptionThreshold();
        uint256 legacyKmsGenThreshold = legacy.getKmsGenThreshold();
        uint256 legacyMpcThreshold = legacy.getMpcThreshold();

        address impl = address(new ProtocolConfigMultichain());
        vm.prank(owner);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfigMultichain.reinitializeV2,
                (_source(sourceBlockNumber, canonicalProtocolConfig))
            )
        );

        ProtocolConfigMultichain multichain = ProtocolConfigMultichain(protocolConfigAdd);
        assertEq(multichain.getVersion(), "ProtocolConfigMultichain v0.2.0");
        assertEq(multichain.getCurrentKmsContextId(), legacyContextId);
        assertTrue(multichain.isValidKmsContext(legacyContextId));
        address[] memory upgradedSigners = multichain.getKmsSignersForContext(legacyContextId);
        assertEq(upgradedSigners.length, legacySigners.length);
        for (uint256 i = 0; i < legacySigners.length; i++) {
            assertEq(upgradedSigners[i], legacySigners[i]);
        }

        KmsNode[] memory upgradedNodes = multichain.getKmsNodesForContext(legacyContextId);
        assertEq(upgradedNodes.length, legacyNodes.length);
        for (uint256 i = 0; i < legacyNodes.length; i++) {
            assertEq(upgradedNodes[i].txSenderAddress, legacyNodes[i].txSenderAddress);
            assertEq(upgradedNodes[i].signerAddress, legacyNodes[i].signerAddress);
            assertEq(upgradedNodes[i].ipAddress, legacyNodes[i].ipAddress);
            assertEq(upgradedNodes[i].storageUrl, legacyNodes[i].storageUrl);
        }

        assertEq(multichain.getPublicDecryptionThreshold(), legacyPublicDecryptionThreshold);
        assertEq(multichain.getUserDecryptionThreshold(), legacyUserDecryptionThreshold);
        assertEq(multichain.getKmsGenThreshold(), legacyKmsGenThreshold);
        assertEq(multichain.getMpcThreshold(), legacyMpcThreshold);
        IProtocolConfigMultichain.MirroredContextSource memory stored = multichain.getMirroredContextSource(
            legacyContextId
        );
        assertEq(stored.sourceChainId, sourceChainId);
        assertEq(stored.sourceBlockNumber, sourceBlockNumber);
        assertEq(stored.sourceProtocolConfig, canonicalProtocolConfig);
    }

    function _singleSigner(address signer) internal pure returns (address[] memory signers) {
        signers = new address[](1);
        signers[0] = signer;
    }
}
