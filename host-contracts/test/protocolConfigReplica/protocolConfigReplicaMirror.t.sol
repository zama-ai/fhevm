// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ProtocolConfigReplicaFixture} from "./ProtocolConfigReplicaTestBase.sol";
import {IProtocolConfigReplica, KmsContextSubmission, KmsEpochSubmission} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigReplica.sol";
import {IProtocolConfigBase} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigBase.sol";
import {KmsThresholds, KmsNode} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";

contract ProtocolConfigReplicaMirrorTest is ProtocolConfigReplicaFixture {
    // -----------------------------------------------------------------------
    // mirrorKmsEpoch
    // -----------------------------------------------------------------------

    function test_mirrorKmsEpochActivatesCanonicalEpoch() public {
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
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1 + 1);
    }

    function test_revertMirrorKmsEpochInvalidContext() public {
        uint256 invalidContextId = CTX1 + 1;

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, invalidContextId));
        protocolConfigReplica.mirrorKmsEpoch(invalidContextId, EPOCH1 + 1);
    }

    function test_revertMirrorKmsEpochSupersededContext() public {
        uint256 supersededContextId = CTX1;

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 1, EPOCH1 + 1, _nodes(SEED1), _thresholds());

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, supersededContextId));
        protocolConfigReplica.mirrorKmsEpoch(supersededContextId, EPOCH1 + 2);
    }

    function test_mirrorKmsEpochMovesEpochBack() public {
        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1 + 5);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1 + 2);
        vm.stopPrank();

        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, CTX1);
        assertEq(activeEpochId, EPOCH1 + 2);
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 2));
    }

    // -----------------------------------------------------------------------
    // mirrorKmsContextAndEpoch
    // -----------------------------------------------------------------------

    function test_mirrorKmsContextAndEpochActivatesAndEmits() public {
        uint256 contextId = CTX1 + 4;
        uint256 epochId = EPOCH1 + 4;
        KmsNode[] memory nodes = _nodes(SEED1);
        KmsThresholds memory thresholds = _thresholds();

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
        _assertMembers(contextId, SEED1);
    }

    function test_mirrorKmsContextAndEpochAllowsGap() public {
        // The active context is CTX1. Mirror a context ID with a gap.
        uint256 gappedContextId = CTX1 + 9;
        uint256 gappedEpochId = EPOCH1 + 9;

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(gappedContextId, gappedEpochId, _nodes(SEED1), _thresholds());

        assertEq(protocolConfigReplica.getCurrentKmsContextId(), gappedContextId);
        assertTrue(protocolConfigReplica.isValidKmsContext(gappedContextId));
        // The skipped IDs in the gap remain invalid.
        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1 + 4));
    }

    function test_mirrorKmsContextAndEpochMovesBack() public {
        _switch(SEED0, CTX1 + 2, EPOCH1 + 2, SEED1);

        // Lower IDs move the replica back.
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 1, EPOCH1 + 1, _nodes(SEED0), _thresholds());

        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, CTX1 + 1);
        assertEq(activeEpochId, EPOCH1 + 1);
        _assertMembers(CTX1 + 1, SEED0);
    }

    function test_recoveryFromLateActivation() public {
        // Canonical: C2 stays pending and is destroyed, then C1 switches to C3. A late signature set
        // activates C2 on the replica.
        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);

        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1, EPOCH1, _nodes(SEED0), _thresholds());
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1 + 1);
        vm.stopPrank();

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

    function test_mirrorKmsContextAndEpochOverridesCompromisedCommittee() public {
        uint256 forgedId = CTX1 + 1;
        // A compromised committee forges a switch to committee SEED1.
        _switch(SEED0, forgedId, EPOCH1 + 1, SEED1);

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(forgedId, EPOCH1 + 1, _nodes(SEED2), _thresholds());

        _assertNotMembers(forgedId, SEED1);
        _assertMembers(forgedId, SEED2);
        // The mirrored committee signs the next resharing.
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(forgedId, EPOCH1 + 1, EPOCH1 + 2, SEED2, N));
        (, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochId, EPOCH1 + 2);
    }

    function test_recoveryBlocksContextSwitchReplay() public {
        uint256 forgedId = CTX1 + 1;
        KmsContextSubmission memory context = _ctxSubmission(CTX1, forgedId, SEED1, SEED0, QUORUM);
        KmsEpochSubmission memory epoch = _epochSubmission(forgedId, EPOCH1, EPOCH1 + 1, SEED1, N);
        protocolConfigReplica.submitKmsContextSwitch(context, epoch);

        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1, EPOCH1, _nodes(SEED0), _thresholds());
        protocolConfigReplica.mirrorKmsContextDestruction(forgedId);
        vm.stopPrank();

        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, CTX1);
        assertEq(activeEpochId, EPOCH1);
        _assertMembers(CTX1, SEED0);
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
        assertFalse(protocolConfigReplica.isValidKmsContext(forgedId));

        // The replayed calldata still matches the digest built from the restored pair.
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.DestroyedKmsContextCannotBeActivated.selector, forgedId)
        );
        protocolConfigReplica.submitKmsContextSwitch(context, epoch);
    }

    function test_recoveryBlocksResharingReplay() public {
        uint256 forgedEpochId = EPOCH1 + 1;
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, forgedEpochId, SEED0, N);
        protocolConfigReplica.submitKmsKeyResharing(epoch);

        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1);
        protocolConfigReplica.mirrorKmsEpochDestruction(forgedEpochId);
        vm.stopPrank();

        (uint256 activeContextId, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, CTX1);
        assertEq(activeEpochId, EPOCH1);
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, forgedEpochId));

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.DestroyedKmsEpochCannotBeActivated.selector, forgedEpochId)
        );
        protocolConfigReplica.submitKmsKeyResharing(epoch);
    }

    function test_recoveryThenCanonicalUsesDestroyedId() public {
        uint256 forgedId = CTX1 + 1;
        uint256 forgedEpochId = EPOCH1 + 1;
        uint256 realEpochId = EPOCH1 + 2;
        _switch(SEED0, forgedId, forgedEpochId, SEED1);
        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1, EPOCH1, _nodes(SEED0), _thresholds());
        protocolConfigReplica.mirrorKmsContextDestruction(forgedId);
        protocolConfigReplica.mirrorKmsEpochDestruction(forgedEpochId);
        vm.stopPrank();

        // Canonical later allocates the same context ID to a real committee. The owner mirrors it, which
        // replaces the forged nodes and the Destroyed state.
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(forgedId, realEpochId, _nodes(SEED2), _thresholds());
        _assertNotMembers(forgedId, SEED1);
        _assertMembers(forgedId, SEED2);
        assertTrue(protocolConfigReplica.isValidEpochForContext(forgedId, realEpochId));
        // The forged epoch stays destroyed, so it is not valid for the reused context ID.
        assertFalse(protocolConfigReplica.isValidEpochForContext(forgedId, forgedEpochId));

        // The real committee signs the next resharing, and a later canonical destruction applies.
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(forgedId, realEpochId, realEpochId + 1, SEED2, N));
        _switch(SEED2, CTX1 + 2, EPOCH1 + 4, SEED1);
        _destroyContext(forgedId, SEED1, QUORUM);
        assertFalse(protocolConfigReplica.isValidKmsContext(forgedId));
    }

    function test_mirroredDestroyedEpochCanBeDestroyedAgain() public {
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
        uint256 forgedId = CTX1 + 1;
        _switch(SEED0, forgedId, EPOCH1 + 1, SEED1);

        // The owner moves the replica back and does not destroy the forged ID.
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1, EPOCH1, _nodes(SEED0), _thresholds());

        _switch(SEED0, forgedId, EPOCH1 + 1, SEED2);
        assertEq(protocolConfigReplica.getCurrentKmsContextId(), forgedId);
        _assertNotMembers(forgedId, SEED1);
        _assertMembers(forgedId, SEED2);
    }

    function test_revertMirrorKmsContextAndEpochNotOwner() public {
        KmsNode[] memory nodes = _nodes(SEED1);
        KmsThresholds memory thresholds = _thresholds();

        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 4, EPOCH1 + 4, nodes, thresholds);
    }

    // -----------------------------------------------------------------------
    // mirrorKmsContextDestruction / mirrorKmsEpochDestruction
    // -----------------------------------------------------------------------

    function test_mirrorKmsContextDestruction() public {
        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);

        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsContextDestroyed(CTX1);
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1);

        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1));
    }

    function test_mirrorKmsEpochDestruction() public {
        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1 + 1);

        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsEpochDestroyed(EPOCH1);
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH1);
        vm.stopPrank();

        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
    }

    function test_revertMirrorKmsContextDestructionTwice() public {
        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);

        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextAlreadyDestroyed.selector, CTX1));
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1);
        vm.stopPrank();
    }

    function test_revertMirrorKmsEpochDestructionTwice() public {
        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1 + 1);
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH1);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochAlreadyDestroyed.selector, EPOCH1));
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH1);
        vm.stopPrank();
    }

    function test_revertMirrorKmsContextDestructionNotOwner() public {
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1 + 9);
    }

    function test_revertMirrorKmsEpochDestructionNotOwner() public {
        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH1 + 9);
    }

    function test_revertMirrorKmsContextDestructionActive() public {
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.LatestActiveKmsContextCannotBeDestroyed.selector, CTX1)
        );
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1);
    }

    function test_revertMirrorKmsEpochDestructionActive() public {
        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.LatestActiveKmsEpochCannotBeDestroyed.selector, EPOCH1)
        );
        protocolConfigReplica.mirrorKmsEpochDestruction(EPOCH1);
    }
}
