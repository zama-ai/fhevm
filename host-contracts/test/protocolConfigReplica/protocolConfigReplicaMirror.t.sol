// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ProtocolConfigReplicaFixture} from "./ProtocolConfigReplicaTestBase.sol";
import {IProtocolConfigReplica} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigReplica.sol";
import {IProtocolConfigBase} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigBase.sol";
import {KmsThresholds, KmsNode} from "@fhevm-host-contracts/contracts/shared/Structs.sol";

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

        _assertActive(contextId, epochId);
        assertTrue(protocolConfigReplica.isValidEpochForContext(contextId, epochId));
    }

    function test_revertMirrorKmsEpochNotOwner() public {
        _expectNotOwner();
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

        _assertActive(CTX1, EPOCH1 + 2);
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

        // The mirrored context and epoch are active at once, with no quorum replay.
        _assertActive(contextId, epochId);
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

        _assertActive(CTX1 + 1, EPOCH1 + 1);
        _assertMembers(CTX1 + 1, SEED0);
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

    function test_revertMirrorKmsContextAndEpochNotOwner() public {
        KmsNode[] memory nodes = _nodes(SEED1);
        KmsThresholds memory thresholds = _thresholds();

        _expectNotOwner();
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
        _expectNotOwner();
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1 + 9);
    }

    function test_revertMirrorKmsEpochDestructionNotOwner() public {
        _expectNotOwner();
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
