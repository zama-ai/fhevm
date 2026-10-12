// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {ProtocolConfigReplica} from "@fhevm-host-contracts/contracts/ProtocolConfigReplica.sol";
import {IProtocolConfigReplica} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigReplica.sol";
import {IProtocolConfigBase} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigBase.sol";
import {KmsThresholds, KmsNodeParams, PcrValues} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {UUPSUpgradeableEmptyProxy} from "@fhevm-host-contracts/contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {protocolConfigAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";

contract ProtocolConfigReplicaTest is HostContractsDeployerTestUtils {
    ProtocolConfigReplica internal protocolConfigReplica;

    address internal constant owner = address(456);
    bytes32 internal constant INITIALIZABLE_STORAGE =
        0xf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00;

    function _setupEmptyProxy() internal {
        _deployACL(owner);
        address emptyProxyImpl = address(new EmptyUUPSProxy());
        deployCodeTo(
            "fhevm-foundry/HostContractsDeployerTestUtils.sol:DeployableERC1967Proxy",
            abi.encode(emptyProxyImpl, abi.encodeCall(EmptyUUPSProxy.initialize, ())),
            protocolConfigAdd
        );
    }

    /// @dev Bootstraps a replica on the same active context and epoch as a fresh canonical deploy.
    function _setupReplica() internal {
        _deployACL(owner);
        (protocolConfigReplica, ) = _deployProtocolConfigReplica(
            owner,
            KMS_CONTEXT_COUNTER_BASE + 1,
            EPOCH_COUNTER_BASE + 1,
            _makeKmsNodeParams(2),
            _defaultThresholds()
        );
    }

    function _setupCanonical() internal returns (ProtocolConfig pc) {
        _deployACL(owner);
        (pc, ) = _deployProtocolConfig(owner, _makeKmsNodeParams(2), _defaultThresholds());
    }

    function _upgradeToReplica(ProtocolConfig pc) internal returns (ProtocolConfigReplica) {
        address replicaImpl = address(new ProtocolConfigReplica());
        vm.prank(owner);
        pc.upgradeToAndCall(replicaImpl, abi.encodeCall(ProtocolConfigReplica.reinitializeV4, ()));
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
            _defaultThresholds(),
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
        // Open a pending epoch on canonical, then upgrade the proxy in place to the replica: this is the
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
        _setupEmptyProxy();

        address impl = address(new ProtocolConfigReplica());
        uint256 canonicalContextId = KMS_CONTEXT_COUNTER_BASE + 1;
        uint256 invalidEpochId = EPOCH_COUNTER_BASE;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        KmsThresholds memory thresholds = _defaultThresholds();

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, invalidEpochId));
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfigReplica.initializeFromCanonical,
                (canonicalContextId, invalidEpochId, nodes, thresholds)
            )
        );
    }

    function test_revertInitializeFromCanonicalInvalidContextId() public {
        _setupEmptyProxy();

        address impl = address(new ProtocolConfigReplica());
        uint256 invalidContextId = KMS_CONTEXT_COUNTER_BASE;
        uint256 canonicalEpochId = EPOCH_COUNTER_BASE + 1;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        KmsThresholds memory thresholds = _defaultThresholds();

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, invalidContextId));
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfigReplica.initializeFromCanonical,
                (invalidContextId, canonicalEpochId, nodes, thresholds)
            )
        );
    }

    // -----------------------------------------------------------------------
    // Re-initialization protection
    // -----------------------------------------------------------------------

    function test_revertInitializeFromCanonicalAfterInit() public {
        _setupReplica();

        uint256 canonicalContextId = KMS_CONTEXT_COUNTER_BASE + 5;
        uint256 canonicalEpochId = EPOCH_COUNTER_BASE + 5;
        vm.prank(owner);
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        protocolConfigReplica.initializeFromCanonical(
            canonicalContextId,
            canonicalEpochId,
            _makeKmsNodeParams(1),
            _defaultThresholds()
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
                (KMS_CONTEXT_COUNTER_BASE + 1, EPOCH_COUNTER_BASE + 1, _makeKmsNodeParams(2), _defaultThresholds())
            )
        );
    }

    function test_reinitializeV4SucceedsOnUpgradeFromProtocolConfig() public {
        ProtocolConfig pc = _setupCanonical();

        (uint256 contextId, uint256 epochId) = pc.getCurrentKmsContextAndEpoch();
        address[] memory signers = pc.getKmsSignersForContext(contextId);
        uint256 publicDecryption = pc.getPublicDecryptionThresholdForContext(contextId);
        uint256 userDecryption = pc.getUserDecryptionThresholdForContext(contextId);
        uint256 kmsGen = pc.getKmsGenThresholdForContext(contextId);
        uint256 mpc = pc.getMpcThresholdForContext(contextId);

        protocolConfigReplica = _upgradeToReplica(pc);

        assertEq(uint256(vm.load(protocolConfigAdd, INITIALIZABLE_STORAGE)), 5);
        assertEq(protocolConfigReplica.getVersion(), "ProtocolConfigReplica v0.1.0");
        (uint256 newContextId, uint256 newEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(newContextId, contextId);
        assertEq(newEpochId, epochId);
        assertEq(protocolConfigReplica.getKmsSignersForContext(contextId), signers);
        assertEq(protocolConfigReplica.getPublicDecryptionThresholdForContext(contextId), publicDecryption);
        assertEq(protocolConfigReplica.getUserDecryptionThresholdForContext(contextId), userDecryption);
        assertEq(protocolConfigReplica.getKmsGenThresholdForContext(contextId), kmsGen);
        assertEq(protocolConfigReplica.getMpcThresholdForContext(contextId), mpc);

        vm.expectRevert(Initializable.InvalidInitialization.selector);
        protocolConfigReplica.reinitializeV4();
    }

    function test_reinitializeV4SucceedsOnUpgradeFromInitializedVersion3() public {
        ProtocolConfig pc = _setupCanonical();
        (uint256 contextId, uint256 epochId) = pc.getCurrentKmsContextAndEpoch();
        // Simulate a proxy initialized by the v0.14.x release (initialized version 3).
        vm.store(protocolConfigAdd, INITIALIZABLE_STORAGE, bytes32(uint256(3)));

        protocolConfigReplica = _upgradeToReplica(pc);

        assertEq(uint256(vm.load(protocolConfigAdd, INITIALIZABLE_STORAGE)), 5);
        assertEq(protocolConfigReplica.getVersion(), "ProtocolConfigReplica v0.1.0");
        (uint256 newContextId, uint256 newEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(newContextId, contextId);
        assertEq(newEpochId, epochId);

        vm.expectRevert(Initializable.InvalidInitialization.selector);
        protocolConfigReplica.reinitializeV4();
    }

    function test_revertReinitializeV4OnFreshReplica() public {
        _setupReplica();

        vm.expectRevert(Initializable.InvalidInitialization.selector);
        protocolConfigReplica.reinitializeV4();
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
        KmsThresholds memory thresholds = _defaultThresholds();
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
        KmsThresholds memory thresholds = _defaultThresholds();

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
        KmsThresholds memory thresholds = _defaultThresholds();

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
        KmsThresholds memory thresholds = _defaultThresholds();

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
        KmsThresholds memory thresholds = _defaultThresholds();

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
        // Open a pending context on canonical, then upgrade the proxy in place to the replica: this is the
        // only way a replica holds a context counter above its active context.
        ProtocolConfig pc = _setupCanonical();
        uint256 pendingContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        KmsThresholds memory thresholds = _defaultThresholds();

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
        KmsThresholds memory thresholds = _defaultThresholds();

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
}
