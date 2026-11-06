// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {ProtocolConfigReplica} from "@fhevm-host-contracts/contracts/ProtocolConfigReplica.sol";
import {IProtocolConfigReplica} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigReplica.sol";
import {IProtocolConfigBase} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigBase.sol";
import {Coprocessor, KmsThresholds, KmsNodeParams, PcrValues} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {UUPSUpgradeableEmptyProxy} from "@fhevm-host-contracts/contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE, COPROCESSOR_CONTEXT_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {ProtocolConfigV014Example} from "@fhevm-host-contracts/examples/ProtocolConfigV014Example.sol";
import {protocolConfigAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";

contract ProtocolConfigReplicaTest is HostContractsDeployerTestUtils {
    ProtocolConfigReplica internal protocolConfigReplica;

    address internal constant owner = address(456);
    address internal constant notOwner = address(0x999);
    bytes32 internal constant INITIALIZABLE_STORAGE =
        0xf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00;
    uint256 internal constant FIRST_COPROCESSOR_CONTEXT_ID = COPROCESSOR_CONTEXT_COUNTER_BASE + 1;
    /// @dev The canonical coprocessor context a replica bootstrapped later starts from.
    uint256 internal constant CANONICAL_COPROCESSOR_CONTEXT_ID = COPROCESSOR_CONTEXT_COUNTER_BASE + 3;

    /// @dev Bootstraps a replica on the same active context and epoch as a fresh canonical deploy.
    function _setupReplica() internal {
        _deployACL(owner);
        (protocolConfigReplica, ) = _deployProtocolConfigReplica(
            owner,
            KMS_CONTEXT_COUNTER_BASE + 1,
            EPOCH_COUNTER_BASE + 1,
            _makeKmsNodeParams(2),
            _defaultKmsThresholds(),
            COPROCESSOR_CONTEXT_COUNTER_BASE + 1,
            _makeCoprocessors(1),
            1
        );
    }

    /// @dev A proxy as existing replicas run it: ProtocolConfig v0.14, with no coprocessor context.
    function _setupCanonical() internal returns (ProtocolConfig pc) {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();
        address impl = address(new ProtocolConfigV014Example());
        vm.prank(owner);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfig.initializeFromEmptyProxy,
                (_makeKmsNodeParams(2), _defaultKmsThresholds(), "", new PcrValues[](0), new Coprocessor[](0), 0)
            )
        );
        pc = ProtocolConfig(protocolConfigAdd);
    }

    function _upgradeToReplica(ProtocolConfig pc) internal returns (ProtocolConfigReplica) {
        // Existing replica proxies run the v0.14 ProtocolConfig, which is at initialized version 3.
        vm.store(protocolConfigAdd, INITIALIZABLE_STORAGE, bytes32(uint256(3)));
        address replicaImpl = address(new ProtocolConfigReplica());
        vm.prank(owner);
        pc.upgradeToAndCall(
            replicaImpl,
            abi.encodeCall(
                ProtocolConfigReplica.reinitializeV3,
                (COPROCESSOR_CONTEXT_COUNTER_BASE + 1, _makeCoprocessors(1), 1)
            )
        );
        return ProtocolConfigReplica(protocolConfigAdd);
    }

    // -----------------------------------------------------------------------
    // mirrorKmsEpoch
    // -----------------------------------------------------------------------

    function test_mirrorKmsEpochActivatesCanonicalEpoch() public {
        _setupReplica();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 epochId = EPOCH_COUNTER_BASE + 7;

        vm.expectEmit(true, true, false, true, address(protocolConfigReplica));
        emit IProtocolConfigReplica.MirrorKmsEpoch(contextId, epochId);
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpoch(contextId, epochId);

        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, contextId);
        assertEq(activeEpochId, epochId);
        assertTrue(protocolConfigReplica.isValidEpochForContext(contextId, epochId));
    }

    function test_revertMirrorKmsEpochNotOwner() public {
        _setupReplica();

        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfigReplica.mirrorKmsEpoch(KMS_CONTEXT_COUNTER_BASE + 1, EPOCH_COUNTER_BASE + 2);
    }

    function test_revertMirrorKmsEpochInvalidContext() public {
        _setupReplica();
        uint256 invalidContextId = KMS_CONTEXT_COUNTER_BASE + 2;

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, invalidContextId));
        protocolConfigReplica.mirrorKmsEpoch(invalidContextId, EPOCH_COUNTER_BASE + 2);
    }

    function test_revertMirrorKmsEpochSupersededContext() public {
        _setupReplica();
        uint256 supersededContextId = KMS_CONTEXT_COUNTER_BASE + 1;

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(
            KMS_CONTEXT_COUNTER_BASE + 2,
            EPOCH_COUNTER_BASE + 2,
            _makeKmsNodeParams(2),
            _defaultKmsThresholds(),
            "",
            new PcrValues[](0)
        );

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, supersededContextId));
        protocolConfigReplica.mirrorKmsEpoch(supersededContextId, EPOCH_COUNTER_BASE + 3);
    }

    function test_revertMirrorKmsEpochNonIncreasingEpoch() public {
        _setupReplica();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 currentEpochId = EPOCH_COUNTER_BASE + 1;
        uint256 staleEpochId = currentEpochId - 1;

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.NonIncreasingEpochId.selector, staleEpochId, currentEpochId)
        );
        protocolConfigReplica.mirrorKmsEpoch(contextId, staleEpochId);
    }

    function test_revertMirrorKmsEpochBeforePendingEpochCounter() public {
        // Open a pending epoch on canonical, then upgrade the proxy in place to the replica. This is the
        // only way a replica holds an epoch counter above its active epoch.
        ProtocolConfig pc = _setupCanonical();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 1;

        vm.prank(owner);
        pc.defineNewEpochForCurrentKmsContext();

        protocolConfigReplica = _upgradeToReplica(pc);

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfigReplica.NonIncreasingEpochId.selector,
                EPOCH_COUNTER_BASE + 2,
                EPOCH_COUNTER_BASE + 2
            )
        );
        protocolConfigReplica.mirrorKmsEpoch(contextId, EPOCH_COUNTER_BASE + 2);
    }

    // -----------------------------------------------------------------------
    // Canonical mirror initializer
    // -----------------------------------------------------------------------

    function test_revertInitializeFromCanonicalInvalidEpochId() public {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();

        address impl = address(new ProtocolConfigReplica());
        uint256 canonicalKmsContextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 invalidEpochId = EPOCH_COUNTER_BASE;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        KmsThresholds memory thresholds = _defaultKmsThresholds();

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, invalidEpochId));
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfigReplica.initializeFromCanonical,
                (
                    canonicalKmsContextId,
                    invalidEpochId,
                    nodes,
                    thresholds,
                    COPROCESSOR_CONTEXT_COUNTER_BASE + 1,
                    _makeCoprocessors(1),
                    1
                )
            )
        );
    }

    function test_revertInitializeFromCanonicalInvalidContextId() public {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();

        address impl = address(new ProtocolConfigReplica());
        uint256 invalidContextId = KMS_CONTEXT_COUNTER_BASE;
        uint256 canonicalKmsEpochId = EPOCH_COUNTER_BASE + 1;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        KmsThresholds memory thresholds = _defaultKmsThresholds();

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, invalidContextId));
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfigReplica.initializeFromCanonical,
                (
                    invalidContextId,
                    canonicalKmsEpochId,
                    nodes,
                    thresholds,
                    COPROCESSOR_CONTEXT_COUNTER_BASE + 1,
                    _makeCoprocessors(1),
                    1
                )
            )
        );
    }

    // -----------------------------------------------------------------------
    // Re-initialization protection
    // -----------------------------------------------------------------------

    function test_revertInitializeFromCanonicalAfterInit() public {
        _setupReplica();

        uint256 canonicalKmsContextId = KMS_CONTEXT_COUNTER_BASE + 5;
        uint256 canonicalKmsEpochId = EPOCH_COUNTER_BASE + 5;
        vm.prank(owner);
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        protocolConfigReplica.initializeFromCanonical(
            canonicalKmsContextId,
            canonicalKmsEpochId,
            _makeKmsNodeParams(1),
            _defaultKmsThresholds(),
            COPROCESSOR_CONTEXT_COUNTER_BASE + 5,
            _makeCoprocessors(1),
            1
        );
    }

    function test_revertInitializeFromCanonicalOnUpgradeFromProtocolConfig() public {
        ProtocolConfig pc = _setupCanonical();

        address replicaImpl = address(new ProtocolConfigReplica());
        vm.prank(owner);
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        pc.upgradeToAndCall(
            replicaImpl,
            abi.encodeCall(
                ProtocolConfigReplica.initializeFromCanonical,
                (
                    KMS_CONTEXT_COUNTER_BASE + 1,
                    EPOCH_COUNTER_BASE + 1,
                    _makeKmsNodeParams(2),
                    _defaultKmsThresholds(),
                    COPROCESSOR_CONTEXT_COUNTER_BASE + 1,
                    _makeCoprocessors(1),
                    1
                )
            )
        );
    }

    function test_reinitializeV3SucceedsOnUpgradeFromProtocolConfig() public {
        ProtocolConfig pc = _setupCanonical();

        (uint256 contextId, uint256 epochId) = pc.getCurrentKmsContextAndEpoch();
        address[] memory signers = pc.getKmsSignersForContext(contextId);
        uint256 publicDecryption = pc.getPublicDecryptionThresholdForContext(contextId);
        uint256 userDecryption = pc.getUserDecryptionThresholdForContext(contextId);
        uint256 kmsGen = pc.getKmsGenThresholdForContext(contextId);
        uint256 mpc = pc.getMpcThresholdForContext(contextId);

        protocolConfigReplica = _upgradeToReplica(pc);

        assertEq(uint256(vm.load(protocolConfigAdd, INITIALIZABLE_STORAGE)), 4);
        assertEq(protocolConfigReplica.getVersion(), "ProtocolConfigReplica v0.3.0");
        (uint256 newContextId, uint256 newEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(newContextId, contextId);
        assertEq(newEpochId, epochId);
        assertEq(protocolConfigReplica.getKmsSignersForContext(contextId), signers);
        assertEq(protocolConfigReplica.getPublicDecryptionThresholdForContext(contextId), publicDecryption);
        assertEq(protocolConfigReplica.getUserDecryptionThresholdForContext(contextId), userDecryption);
        assertEq(protocolConfigReplica.getKmsGenThresholdForContext(contextId), kmsGen);
        assertEq(protocolConfigReplica.getMpcThresholdForContext(contextId), mpc);

        vm.prank(owner);
        vm.expectRevert(Initializable.InvalidInitialization.selector);
        protocolConfigReplica.reinitializeV3(COPROCESSOR_CONTEXT_COUNTER_BASE + 2, _makeCoprocessors(1), 1);
    }

    function test_revertReinitializeV3OnFreshReplica() public {
        _setupReplica();

        vm.prank(owner);
        vm.expectRevert(Initializable.InvalidInitialization.selector);
        protocolConfigReplica.reinitializeV3(COPROCESSOR_CONTEXT_COUNTER_BASE + 2, _makeCoprocessors(1), 1);
    }

    function test_revertUpgradeNotOwner() public {
        _setupReplica();
        address newImpl = address(new ProtocolConfigReplica());

        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfigReplica.upgradeToAndCall(newImpl, "");
    }

    // -----------------------------------------------------------------------
    // mirrorKmsContextAndEpoch
    // -----------------------------------------------------------------------

    function test_mirrorKmsContextAndEpochActivatesAndEmits() public {
        _setupReplica();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 5;
        uint256 epochId = EPOCH_COUNTER_BASE + 5;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        KmsThresholds memory thresholds = _defaultKmsThresholds();
        PcrValues[] memory pcrValues = new PcrValues[](0);

        vm.expectEmit(true, true, false, true, address(protocolConfigReplica));
        emit IProtocolConfigReplica.MirrorKmsContextAndEpoch(
            contextId,
            epochId,
            nodes,
            thresholds,
            "kms-v9",
            pcrValues
        );
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(contextId, epochId, nodes, thresholds, "kms-v9", pcrValues);

        // Mirrored context and epoch are immediately active (no quorum replay).
        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, contextId);
        assertEq(activeEpochId, epochId);
        assertTrue(protocolConfigReplica.isValidKmsContext(contextId));
        assertTrue(protocolConfigReplica.isValidEpochForContext(contextId, epochId));
        assertEq(protocolConfigReplica.getKmsSignersForContext(contextId).length, 2);
    }

    function test_mirrorKmsContextAndEpochActivatesExactNextEpoch() public {
        _setupReplica();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        KmsThresholds memory thresholds = _defaultKmsThresholds();

        vm.expectEmit(true, true, false, true, address(protocolConfigReplica));
        emit IProtocolConfigReplica.MirrorKmsContextAndEpoch(
            contextId,
            epochId,
            nodes,
            thresholds,
            "",
            new PcrValues[](0)
        );
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(contextId, epochId, nodes, thresholds, "", new PcrValues[](0));

        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, contextId);
        assertEq(activeEpochId, epochId);
        assertTrue(protocolConfigReplica.isValidEpochForContext(contextId, epochId));
    }

    function test_mirrorKmsContextAndEpochAllowsGap() public {
        _setupReplica();
        // Active context is BASE + 1; mirror a non-contiguous (gapped) context ID.
        uint256 gappedContextId = KMS_CONTEXT_COUNTER_BASE + 10;
        uint256 gappedEpochId = EPOCH_COUNTER_BASE + 10;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        KmsThresholds memory thresholds = _defaultKmsThresholds();

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(
            gappedContextId,
            gappedEpochId,
            nodes,
            thresholds,
            "",
            new PcrValues[](0)
        );

        assertEq(protocolConfigReplica.getCurrentKmsContextId(), gappedContextId);
        assertTrue(protocolConfigReplica.isValidKmsContext(gappedContextId));
        // The skipped IDs in the gap remain invalid.
        assertFalse(protocolConfigReplica.isValidKmsContext(KMS_CONTEXT_COUNTER_BASE + 5));
    }

    function test_revertMirrorKmsContextAndEpochNonIncreasing() public {
        _setupReplica();
        uint256 activeContextId = protocolConfigReplica.getCurrentKmsContextId();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        KmsThresholds memory thresholds = _defaultKmsThresholds();

        // contextId == activeKmsContextId is rejected (not strictly increasing).
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfigBase.NonIncreasingKmsContextId.selector,
                activeContextId,
                activeContextId
            )
        );
        protocolConfigReplica.mirrorKmsContextAndEpoch(
            activeContextId,
            EPOCH_COUNTER_BASE + 2,
            nodes,
            thresholds,
            "",
            new PcrValues[](0)
        );
    }

    function test_revertMirrorKmsContextAndEpochNonIncreasingEpoch() public {
        _setupReplica();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 2;
        uint256 activeEpochId = EPOCH_COUNTER_BASE + 1;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        KmsThresholds memory thresholds = _defaultKmsThresholds();

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.NonIncreasingEpochId.selector, activeEpochId, activeEpochId)
        );
        protocolConfigReplica.mirrorKmsContextAndEpoch(
            contextId,
            activeEpochId,
            nodes,
            thresholds,
            "",
            new PcrValues[](0)
        );
    }

    function test_revertMirrorKmsContextAndEpochBeforePendingContextCounter() public {
        // Open a pending context on canonical, then upgrade the proxy in place to the replica. This is the
        // only way a replica holds a context counter above its active context.
        ProtocolConfig pc = _setupCanonical();
        uint256 pendingContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        KmsThresholds memory thresholds = _defaultKmsThresholds();

        vm.prank(owner);
        pc.defineNewKmsContextAndEpoch(nodes, thresholds, "", new PcrValues[](0));

        protocolConfigReplica = _upgradeToReplica(pc);

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfigBase.NonIncreasingKmsContextId.selector,
                pendingContextId,
                pendingContextId
            )
        );
        protocolConfigReplica.mirrorKmsContextAndEpoch(
            pendingContextId,
            EPOCH_COUNTER_BASE + 2,
            nodes,
            thresholds,
            "",
            new PcrValues[](0)
        );
    }

    function test_revertMirrorKmsContextAndEpochNotOwner() public {
        _setupReplica();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        KmsThresholds memory thresholds = _defaultKmsThresholds();

        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfigReplica.mirrorKmsContextAndEpoch(
            KMS_CONTEXT_COUNTER_BASE + 5,
            EPOCH_COUNTER_BASE + 5,
            nodes,
            thresholds,
            "",
            new PcrValues[](0)
        );
    }

    // -----------------------------------------------------------------------
    // Coprocessor contexts
    // -----------------------------------------------------------------------

    function _setupReplicaWithCoprocessors() internal {
        _deployACL(owner);
        (protocolConfigReplica, ) = _deployProtocolConfigReplica(
            owner,
            KMS_CONTEXT_COUNTER_BASE + 1,
            EPOCH_COUNTER_BASE + 1,
            _makeKmsNodeParams(2),
            _defaultKmsThresholds(),
            CANONICAL_COPROCESSOR_CONTEXT_ID,
            _makeCoprocessors(3),
            2
        );
    }

    function _mirrorCoprocessorContext(
        uint256 contextId,
        Coprocessor[] memory coprocessors,
        uint256 majorityThreshold
    ) internal {
        vm.prank(owner);
        protocolConfigReplica.mirrorCoprocessorContext(contextId, coprocessors, majorityThreshold);
    }

    function _assertCoprocessorContext(
        uint256 contextId,
        Coprocessor[] memory expected,
        uint256 expectedMajorityThreshold
    ) internal view {
        Coprocessor[] memory stored = protocolConfigReplica.getCoprocessorsForContext(contextId);
        assertEq(stored.length, expected.length);
        for (uint256 i = 0; i < expected.length; i++) {
            assertEq(stored[i].txSenderAddress, expected[i].txSenderAddress);
            assertEq(stored[i].signerAddress, expected[i].signerAddress);
            assertEq(stored[i].storageUrl, expected[i].storageUrl);
            assertEq(stored[i].prfEndpointUrl, expected[i].prfEndpointUrl);
            assertTrue(protocolConfigReplica.isCoprocessorSignerForContext(contextId, expected[i].signerAddress));
            assertTrue(protocolConfigReplica.isCoprocessorTxSenderForContext(contextId, expected[i].txSenderAddress));
        }
        assertEq(protocolConfigReplica.getCoprocessorMajorityThresholdForContext(contextId), expectedMajorityThreshold);
    }

    // -----------------------------------------------------------------------
    // Coprocessor contexts: Initialization
    // -----------------------------------------------------------------------

    function test_initializeFromCanonicalPreservesCoprocessorContextId() public {
        Coprocessor[] memory coprocessors = _makeCoprocessors(3);
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();
        address impl = address(new ProtocolConfigReplica());

        vm.expectEmit(true, false, false, true, protocolConfigAdd);
        emit IProtocolConfigReplica.MirrorCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID, coprocessors, 2);
        vm.prank(owner);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfigReplica.initializeFromCanonical,
                (
                    KMS_CONTEXT_COUNTER_BASE + 1,
                    EPOCH_COUNTER_BASE + 1,
                    _makeKmsNodeParams(2),
                    _defaultKmsThresholds(),
                    CANONICAL_COPROCESSOR_CONTEXT_ID,
                    coprocessors,
                    2
                )
            )
        );
        protocolConfigReplica = ProtocolConfigReplica(protocolConfigAdd);

        assertEq(protocolConfigReplica.getCurrentCoprocessorContextId(), CANONICAL_COPROCESSOR_CONTEXT_ID);
        assertTrue(protocolConfigReplica.isValidCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID));
        _assertCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID, coprocessors, 2);
        // Earlier canonical contexts were never mirrored.
        assertFalse(protocolConfigReplica.isValidCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID - 1));
    }

    function test_revertInitializeFromCanonicalInvalidCoprocessorContextId() public {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();
        address impl = address(new ProtocolConfigReplica());
        uint256 invalidId = COPROCESSOR_CONTEXT_COUNTER_BASE;

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidCoprocessorContext.selector, invalidId));
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfigReplica.initializeFromCanonical,
                (
                    KMS_CONTEXT_COUNTER_BASE + 1,
                    EPOCH_COUNTER_BASE + 1,
                    _makeKmsNodeParams(2),
                    _defaultKmsThresholds(),
                    invalidId,
                    _makeCoprocessors(1),
                    1
                )
            )
        );
    }

    function test_revertInitializeFromCanonicalInvalidCoprocessors() public {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();
        address impl = address(new ProtocolConfigReplica());

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidCoprocessorMajorityThreshold.selector, 1, 2));
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfigReplica.initializeFromCanonical,
                (
                    KMS_CONTEXT_COUNTER_BASE + 1,
                    EPOCH_COUNTER_BASE + 1,
                    _makeKmsNodeParams(2),
                    _defaultKmsThresholds(),
                    FIRST_COPROCESSOR_CONTEXT_ID,
                    _makeCoprocessors(2),
                    1
                )
            )
        );
    }

    /// @dev The v0.14 proxy of `_setupCanonical`, at the initialized version existing replicas have.
    function _deployV014Proxy() internal returns (ProtocolConfig pc) {
        pc = _setupCanonical();
        vm.store(protocolConfigAdd, INITIALIZABLE_STORAGE, bytes32(uint256(3)));
    }

    function test_reinitializeV3MirrorsCanonicalContextOnUpgrade() public {
        ProtocolConfig pc = _deployV014Proxy();
        Coprocessor[] memory coprocessors = _makeCoprocessors(5);
        address impl = address(new ProtocolConfigReplica());

        vm.expectEmit(true, false, false, true, protocolConfigAdd);
        emit IProtocolConfigReplica.MirrorCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID, coprocessors, 3);
        vm.prank(owner);
        pc.upgradeToAndCall(
            impl,
            abi.encodeCall(ProtocolConfigReplica.reinitializeV3, (FIRST_COPROCESSOR_CONTEXT_ID, coprocessors, 3))
        );
        protocolConfigReplica = ProtocolConfigReplica(protocolConfigAdd);

        assertEq(protocolConfigReplica.getCurrentCoprocessorContextId(), FIRST_COPROCESSOR_CONTEXT_ID);
        _assertCoprocessorContext(FIRST_COPROCESSOR_CONTEXT_ID, coprocessors, 3);
    }

    function test_revertReinitializeV3InvalidCoprocessorContextId() public {
        ProtocolConfig pc = _deployV014Proxy();
        address impl = address(new ProtocolConfigReplica());

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidCoprocessorContext.selector, 0));
        pc.upgradeToAndCall(impl, abi.encodeCall(ProtocolConfigReplica.reinitializeV3, (0, _makeCoprocessors(1), 1)));
    }

    function test_revertReinitializeV3NotOwner() public {
        ProtocolConfig pc = _deployV014Proxy();
        address impl = address(new ProtocolConfigReplica());
        vm.prank(owner);
        pc.upgradeToAndCall(impl, "");

        vm.prank(notOwner);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, notOwner));
        ProtocolConfigReplica(protocolConfigAdd).reinitializeV3(FIRST_COPROCESSOR_CONTEXT_ID, _makeCoprocessors(1), 1);
    }

    // -----------------------------------------------------------------------
    // Coprocessor contexts: mirrorCoprocessorContext
    // -----------------------------------------------------------------------

    function test_mirrorCoprocessorContextAllowsGaps() public {
        _setupReplicaWithCoprocessors();
        Coprocessor[] memory coprocessors = _makeCoprocessors(4);
        uint256 contextId = CANONICAL_COPROCESSOR_CONTEXT_ID + 2;

        vm.expectEmit(true, false, false, true, protocolConfigAdd);
        emit IProtocolConfigReplica.MirrorCoprocessorContext(contextId, coprocessors, 3);
        _mirrorCoprocessorContext(contextId, coprocessors, 3);

        assertEq(protocolConfigReplica.getCurrentCoprocessorContextId(), contextId);
        _assertCoprocessorContext(contextId, coprocessors, 3);
        assertTrue(protocolConfigReplica.isValidCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID));
        assertFalse(protocolConfigReplica.isValidCoprocessorContext(contextId - 1));
    }

    function test_revertMirrorCoprocessorContextNonIncreasing() public {
        _setupReplicaWithCoprocessors();
        uint256[2] memory staleIds = [CANONICAL_COPROCESSOR_CONTEXT_ID, CANONICAL_COPROCESSOR_CONTEXT_ID - 1];
        for (uint256 i = 0; i < staleIds.length; i++) {
            vm.prank(owner);
            vm.expectRevert(
                abi.encodeWithSelector(
                    IProtocolConfigReplica.NonIncreasingCoprocessorContextId.selector,
                    staleIds[i],
                    CANONICAL_COPROCESSOR_CONTEXT_ID
                )
            );
            protocolConfigReplica.mirrorCoprocessorContext(staleIds[i], _makeCoprocessors(1), 1);
        }
    }

    function test_revertMirrorCoprocessorContextInvalidSet() public {
        _setupReplicaWithCoprocessors();
        Coprocessor[] memory coprocessors = _makeCoprocessors(2);
        coprocessors[1].signerAddress = coprocessors[0].signerAddress;

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfigBase.CoprocessorSignerAlreadyRegistered.selector,
                coprocessors[0].signerAddress
            )
        );
        protocolConfigReplica.mirrorCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID + 1, coprocessors, 2);
    }

    function test_revertMirrorCoprocessorContextNotOwner() public {
        _setupReplicaWithCoprocessors();
        vm.prank(notOwner);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, notOwner));
        protocolConfigReplica.mirrorCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID + 1, _makeCoprocessors(1), 1);
    }

    // -----------------------------------------------------------------------
    // Coprocessor contexts: mirrorCoprocessorContextDestruction
    // -----------------------------------------------------------------------

    function test_mirrorCoprocessorContextDestruction() public {
        _setupReplicaWithCoprocessors();
        _mirrorCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID + 1, _makeCoprocessors(1), 1);

        vm.expectEmit(true, false, false, true, protocolConfigAdd);
        emit IProtocolConfigBase.CoprocessorContextDestroyed(CANONICAL_COPROCESSOR_CONTEXT_ID);
        vm.prank(owner);
        protocolConfigReplica.mirrorCoprocessorContextDestruction(CANONICAL_COPROCESSOR_CONTEXT_ID);

        assertFalse(protocolConfigReplica.isValidCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID));
        _assertCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID, _makeCoprocessors(3), 2);
    }

    function test_revertMirrorCoprocessorContextDestructionCurrent() public {
        _setupReplicaWithCoprocessors();
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfigBase.CurrentCoprocessorContextCannotBeDestroyed.selector,
                CANONICAL_COPROCESSOR_CONTEXT_ID
            )
        );
        protocolConfigReplica.mirrorCoprocessorContextDestruction(CANONICAL_COPROCESSOR_CONTEXT_ID);
    }

    function test_revertMirrorCoprocessorContextDestructionNeverMirrored() public {
        _setupReplicaWithCoprocessors();
        _mirrorCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID + 2, _makeCoprocessors(1), 1);
        uint256 skippedId = CANONICAL_COPROCESSOR_CONTEXT_ID + 1;

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidCoprocessorContext.selector, skippedId));
        protocolConfigReplica.mirrorCoprocessorContextDestruction(skippedId);
    }

    function test_revertMirrorCoprocessorContextDestructionTwice() public {
        _setupReplicaWithCoprocessors();
        _mirrorCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID + 1, _makeCoprocessors(1), 1);
        vm.prank(owner);
        protocolConfigReplica.mirrorCoprocessorContextDestruction(CANONICAL_COPROCESSOR_CONTEXT_ID);

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfigBase.InvalidCoprocessorContext.selector,
                CANONICAL_COPROCESSOR_CONTEXT_ID
            )
        );
        protocolConfigReplica.mirrorCoprocessorContextDestruction(CANONICAL_COPROCESSOR_CONTEXT_ID);
    }

    function test_revertMirrorCoprocessorContextDestructionNotOwner() public {
        _setupReplicaWithCoprocessors();
        _mirrorCoprocessorContext(CANONICAL_COPROCESSOR_CONTEXT_ID + 1, _makeCoprocessors(1), 1);
        vm.prank(notOwner);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, notOwner));
        protocolConfigReplica.mirrorCoprocessorContextDestruction(CANONICAL_COPROCESSOR_CONTEXT_ID);
    }

    // -----------------------------------------------------------------------
    // Coprocessor contexts: Canonical and replica agree
    // -----------------------------------------------------------------------

    /// @dev A replica fed the canonical reads returns the same reads as the canonical contract.
    function test_replicaMatchesCanonicalReads() public {
        // Canonical and replica share the proxy address, so read canonical first and roll back.
        uint256 beforeCanonical = vm.snapshotState();
        _deployACL(owner);
        (ProtocolConfig canonical, ) = _deployProtocolConfig(
            owner,
            _makeKmsNodeParams(2),
            _defaultKmsThresholds(),
            _makeCoprocessors(3),
            2
        );
        vm.prank(owner);
        canonical.defineNewCoprocessorContext(_makeCoprocessors(4), 3);
        uint256 contextId = canonical.getCurrentCoprocessorContextId();
        Coprocessor[] memory canonicalSet = canonical.getCoprocessorsForContext(contextId);
        uint256 canonicalThreshold = canonical.getCoprocessorMajorityThresholdForContext(contextId);
        vm.revertToState(beforeCanonical);

        _setupReplicaWithCoprocessors();
        _mirrorCoprocessorContext(contextId + 2, canonicalSet, canonicalThreshold);
        _assertCoprocessorContext(contextId + 2, _makeCoprocessors(4), 3);
    }
}
