// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ProtocolConfigReplicaTestBase} from "./protocolConfigReplica.t.sol";
import {IProtocolConfigReplica, KmsContextSubmission, KmsEpochSubmission} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigReplica.sol";
import {IProtocolConfigBase} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigBase.sol";
import {KmsThresholds, KmsNode} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";

contract ProtocolConfigReplicaMirrorTest is ProtocolConfigReplicaTestBase {
    // -----------------------------------------------------------------------
    // mirrorKmsEpoch
    // -----------------------------------------------------------------------

    function test_mirrorKmsEpochActivatesCanonicalEpoch() public {
        _setupReplica();
        uint256 contextId = CTX1;
        uint256 epochId = EPOCH1 + 6;

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
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1 + 1);
    }

    function test_revertMirrorKmsEpochInvalidContext() public {
        _setupReplica();
        uint256 invalidContextId = CTX1 + 1;

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, invalidContextId));
        protocolConfigReplica.mirrorKmsEpoch(invalidContextId, EPOCH1 + 1);
    }

    function test_revertMirrorKmsEpochSupersededContext() public {
        _setupReplica();
        uint256 supersededContextId = CTX1;

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(
            CTX1 + 1,
            EPOCH1 + 1,
            _toKmsNodes(_makeKmsNodeParams(2)),
            _defaultThresholds()
        );

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, supersededContextId));
        protocolConfigReplica.mirrorKmsEpoch(supersededContextId, EPOCH1 + 2);
    }

    function test_revertMirrorKmsEpochNonIncreasingId() public {
        _setupReplica();

        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1 + 5);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.NonIncreasingEpochId.selector, EPOCH1 + 2, EPOCH1 + 5)
        );
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1 + 2);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.NonIncreasingEpochId.selector, EPOCH1 + 5, EPOCH1 + 5)
        );
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1 + 5);
        vm.stopPrank();
    }

    // -----------------------------------------------------------------------
    // mirrorKmsContextAndEpoch
    // -----------------------------------------------------------------------

    function test_mirrorKmsContextAndEpochActivatesAndEmits() public {
        _setupReplica();
        uint256 contextId = CTX1 + 4;
        uint256 epochId = EPOCH1 + 4;
        KmsNode[] memory nodes = _toKmsNodes(_makeKmsNodeParams(2));
        KmsThresholds memory thresholds = _defaultThresholds();

        vm.expectEmit(true, true, false, true, address(protocolConfigReplica));
        emit IProtocolConfigReplica.MirrorKmsContextAndEpoch(contextId, epochId, nodes, thresholds);
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(contextId, epochId, nodes, thresholds);

        // Mirrored context and epoch are immediately active (no quorum replay).
        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, contextId);
        assertEq(activeEpochId, epochId);
        assertTrue(protocolConfigReplica.isValidKmsContext(contextId));
        assertTrue(protocolConfigReplica.isValidEpochForContext(contextId, epochId));
        assertEq(protocolConfigReplica.getKmsSignersForContext(contextId).length, 2);
    }

    function test_mirrorKmsContextAndEpochAllowsGap() public {
        _setupReplica();
        // The active context is CTX1. Mirror a context ID with a gap.
        uint256 gappedContextId = CTX1 + 9;
        uint256 gappedEpochId = EPOCH1 + 9;

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(
            gappedContextId,
            gappedEpochId,
            _toKmsNodes(_makeKmsNodeParams(2)),
            _defaultThresholds()
        );

        assertEq(protocolConfigReplica.getCurrentKmsContextId(), gappedContextId);
        assertTrue(protocolConfigReplica.isValidKmsContext(gappedContextId));
        // The skipped IDs in the gap remain invalid.
        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1 + 4));
    }

    function test_revertMirrorKmsContextAndEpochNonIncreasingId() public {
        _setupReplica();
        _switch(SEED0, CTX1 + 2, EPOCH1 + 2, SEED1);

        vm.startPrank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.NonIncreasingKmsContextId.selector, CTX1 + 2, CTX1 + 2)
        );
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 2, EPOCH1 + 3, _nodes(SEED2), _thresholds());
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.NonIncreasingKmsContextId.selector, CTX1 + 1, CTX1 + 2)
        );
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 1, EPOCH1 + 3, _nodes(SEED2), _thresholds());
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.NonIncreasingEpochId.selector, EPOCH1 + 2, EPOCH1 + 2)
        );
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 3, EPOCH1 + 2, _nodes(SEED2), _thresholds());
        vm.stopPrank();
    }

    function test_recoveryFromLateActivation() public {
        _setupReplica();
        // Canonical: C2 stays pending and is destroyed, then C1 switches to C3. A late signature set
        // activates C2 on the replica.
        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);

        vm.prank(owner);
        protocolConfigReplica.recoverKmsState(
            CTX1,
            EPOCH1,
            _nodes(SEED0),
            _thresholds(),
            _ids(CTX1 + 1),
            _ids(EPOCH1 + 1)
        );

        // The ID is still ahead of the replica, so the owner flag does not settle it.
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsContextDestructionAheadOfReplica.selector, CTX1 + 1, CTX1)
        );
        _destroyContext(CTX1 + 1, SEED0, QUORUM);

        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 2, SEED2, SEED0, QUORUM),
            _epochSubmission(CTX1 + 2, EPOCH1, EPOCH1 + 2, SEED2, N)
        );

        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, CTX1 + 2);
        assertEq(activeEpochId, EPOCH1 + 2);
        _assertMembers(CTX1 + 2, SEED2);
        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1 + 1));
    }

    function test_recoverKmsStateOverridesCompromisedCommittee() public {
        _setupReplica();
        uint256 forgedId = CTX1 + 1;
        // A compromised committee forges a switch to committee SEED1.
        _switch(SEED0, forgedId, EPOCH1 + 1, SEED1);

        vm.prank(owner);
        protocolConfigReplica.recoverKmsState(
            forgedId,
            EPOCH1 + 1,
            _nodes(SEED2),
            _thresholds(),
            new uint256[](0),
            new uint256[](0)
        );

        _assertNotMembers(forgedId, SEED1);
        _assertMembers(forgedId, SEED2);
        // The mirrored committee signs the next resharing.
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(forgedId, EPOCH1 + 1, EPOCH1 + 2, SEED2, N));
        (, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochId, EPOCH1 + 2);
    }

    function test_recoverKmsStateBlocksForgedReplay() public {
        _setupReplica();
        uint256 forgedId = CTX1 + 1;
        uint256 forgedEpochId = EPOCH1 + 7;
        KmsContextSubmission memory context = _ctxSubmission(CTX1, forgedId, SEED1, SEED0, QUORUM);
        KmsEpochSubmission memory epoch = _epochSubmission(forgedId, EPOCH1, forgedEpochId, SEED1, N);
        protocolConfigReplica.submitKmsContextSwitch(context, epoch);

        vm.expectEmit(true, true, false, true, address(protocolConfigReplica));
        emit IProtocolConfigReplica.MirrorKmsContextAndEpoch(CTX1, EPOCH1, _nodes(SEED0), _thresholds());
        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsContextDestroyed(forgedId);
        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsEpochDestroyed(forgedEpochId);
        vm.prank(owner);
        protocolConfigReplica.recoverKmsState(
            CTX1,
            EPOCH1,
            _nodes(SEED0),
            _thresholds(),
            _ids(forgedId),
            _ids(forgedEpochId)
        );

        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, CTX1);
        assertEq(activeEpochId, EPOCH1);
        assertEq(protocolConfigReplica.getCurrentKmsContextIdCounter(), CTX1);
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
        assertFalse(protocolConfigReplica.isValidKmsContext(forgedId));
        assertFalse(protocolConfigReplica.isValidEpochForContext(forgedId, forgedEpochId));

        // The forged calldata still matches the digest built from the restored pair.
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextIdDestroyed.selector, forgedId));
        protocolConfigReplica.submitKmsContextSwitch(context, epoch);
    }

    function test_recoverKmsStateThenCanonicalUsesDestroyedId() public {
        _setupReplica();
        uint256 forgedId = CTX1 + 1;
        uint256 forgedEpochId = EPOCH1 + 1;
        _switch(SEED0, forgedId, forgedEpochId, SEED1);
        vm.prank(owner);
        protocolConfigReplica.recoverKmsState(
            CTX1,
            EPOCH1,
            _nodes(SEED0),
            _thresholds(),
            _ids(forgedId),
            _ids(forgedEpochId)
        );

        // Canonical later allocates the same IDs to a real committee. The submit path rejects them.
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextIdDestroyed.selector, forgedId));
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, forgedId, SEED2, SEED0, QUORUM),
            _epochSubmission(forgedId, EPOCH1, forgedEpochId, SEED2, N)
        );

        // The owner mirrors them, which replaces the forged nodes and clears the destroyed flags.
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(forgedId, forgedEpochId, _nodes(SEED2), _thresholds());
        _assertNotMembers(forgedId, SEED1);
        _assertMembers(forgedId, SEED2);
        assertTrue(protocolConfigReplica.isValidEpochForContext(forgedId, forgedEpochId));

        // The real committee signs the next resharing, and a later canonical destruction applies.
        protocolConfigReplica.submitKmsKeyResharing(
            _epochSubmission(forgedId, forgedEpochId, forgedEpochId + 1, SEED2, N)
        );
        _switch(SEED2, CTX1 + 2, EPOCH1 + 3, SEED1);
        _destroyContext(forgedId, SEED1, QUORUM);
        assertFalse(protocolConfigReplica.isValidKmsContext(forgedId));
    }

    function test_recoverKmsStateClearsDestroyedRestoredPair() public {
        _setupReplica();
        uint256 forgedId = CTX1 + 1;
        uint256 forgedEpochId = EPOCH1 + 1;
        _switch(SEED0, forgedId, forgedEpochId, SEED1);
        // The forged committee is the active one, so it can destroy the canonical pair.
        _destroyContext(CTX1, SEED1, QUORUM);
        _destroyEpoch(EPOCH1, SEED1, QUORUM);

        vm.prank(owner);
        protocolConfigReplica.recoverKmsState(
            CTX1,
            EPOCH1,
            _nodes(SEED0),
            _thresholds(),
            _ids(forgedId),
            _ids(forgedEpochId)
        );
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));

        // A later canonical destruction of the restored pair applies.
        _switch(SEED0, CTX1 + 2, EPOCH1 + 2, SEED2);
        _destroyContext(CTX1, SEED2, QUORUM);
        _destroyEpoch(EPOCH1, SEED2, QUORUM);
        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1));
    }

    function test_revertRecoverKmsStateListsRestoredId() public {
        _setupReplica();
        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);

        vm.startPrank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.LatestActiveKmsContextCannotBeDestroyed.selector, CTX1)
        );
        protocolConfigReplica.recoverKmsState(CTX1, EPOCH1, _nodes(SEED0), _thresholds(), _ids(CTX1), _ids(EPOCH1 + 1));
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.LatestActiveKmsEpochCannotBeDestroyed.selector, EPOCH1)
        );
        protocolConfigReplica.recoverKmsState(CTX1, EPOCH1, _nodes(SEED0), _thresholds(), _ids(CTX1 + 1), _ids(EPOCH1));
        vm.stopPrank();
    }

    function test_revertRecoverKmsStateNotOwner() public {
        _setupReplica();
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfigReplica.recoverKmsState(
            CTX1,
            EPOCH1,
            _nodes(SEED0),
            _thresholds(),
            new uint256[](0),
            new uint256[](0)
        );
    }

    function test_mirroredDestroyedEpochCanBeDestroyedAgain() public {
        _setupReplica();
        uint256 epochId = EPOCH1 + 1;
        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsEpochDestruction(epochId);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, epochId);
        vm.stopPrank();
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, epochId, epochId + 1, SEED0, N));

        _destroyEpoch(epochId, SEED0, QUORUM);
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, epochId));
    }

    function test_mirroredContextClearsDestroyedEpoch() public {
        _setupReplica();
        uint256 epochId = EPOCH1 + 1;
        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsEpochDestruction(epochId);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 1, epochId, _nodes(SEED1), _thresholds());
        vm.stopPrank();
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1 + 1, epochId, epochId + 1, SEED1, N));

        _destroyEpoch(epochId, SEED1, QUORUM);
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1 + 1, epochId));
    }

    function test_revertMirrorKmsContextAndEpochInvalidThresholds() public {
        _setupReplica();
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidHighThreshold.selector, "mpc", 5, 4));
        protocolConfigReplica.mirrorKmsContextAndEpoch(
            CTX1 + 1,
            EPOCH1 + 1,
            _nodes(SEED1),
            KmsThresholds({publicDecryption: 1, userDecryption: 2, kmsGen: 3, mpc: 5})
        );
    }

    function test_submitSwitchReplacesForgedStoredId() public {
        _setupReplica();
        uint256 forgedId = CTX1 + 1;
        _switch(SEED0, forgedId, EPOCH1 + 1, SEED1);

        // The owner leaves the forged ID out of the lists.
        vm.prank(owner);
        protocolConfigReplica.recoverKmsState(
            CTX1,
            EPOCH1,
            _nodes(SEED0),
            _thresholds(),
            new uint256[](0),
            new uint256[](0)
        );

        _switch(SEED0, forgedId, EPOCH1 + 1, SEED2);
        assertEq(protocolConfigReplica.getCurrentKmsContextId(), forgedId);
        _assertNotMembers(forgedId, SEED1);
        _assertMembers(forgedId, SEED2);
    }

    function test_revertMirrorKmsContextAndEpochNotOwner() public {
        _setupReplica();
        KmsNode[] memory nodes = _toKmsNodes(_makeKmsNodeParams(2));
        KmsThresholds memory thresholds = _defaultThresholds();

        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 4, EPOCH1 + 4, nodes, thresholds);
    }

    // -----------------------------------------------------------------------
    // mirrorKmsContextDestruction / mirrorKmsEpochDestruction
    // -----------------------------------------------------------------------

    function test_mirrorKmsContextDestruction() public {
        _setupReplica();
        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);

        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsContextDestroyed(CTX1);
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1);

        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1));
    }

    function test_mirrorKmsEpochDestruction() public {
        _setupReplica();
        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1 + 1);

        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsEpochDestroyed(EPOCH1);
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH1);
        vm.stopPrank();

        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
    }

    function test_revertMirrorKmsContextDestructionNotOwner() public {
        _setupReplica();
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1 + 9);
    }

    function test_revertMirrorKmsEpochDestructionNotOwner() public {
        _setupReplica();
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH1 + 9);
    }

    function test_revertMirrorKmsContextDestructionActive() public {
        _setupReplica();
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.LatestActiveKmsContextCannotBeDestroyed.selector, CTX1)
        );
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1);
    }

    function test_revertMirrorKmsEpochDestructionActive() public {
        _setupReplica();
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.LatestActiveKmsEpochCannotBeDestroyed.selector, EPOCH1)
        );
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH1);
    }
}
