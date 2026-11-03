// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ProtocolConfigReplicaFixture} from "./ProtocolConfigReplicaTestBase.sol";
import {IProtocolConfigReplica, KmsContextSubmission, KmsEpochSubmission} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigReplica.sol";
import {IProtocolConfigBase} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigBase.sol";
import {KmsThresholds} from "@fhevm-host-contracts/contracts/shared/Structs.sol";

contract ProtocolConfigReplicaDestructionTest is ProtocolConfigReplicaFixture {
    // -----------------------------------------------------------------------
    // submitKmsContextDestruction / submitKmsEpochDestruction
    // -----------------------------------------------------------------------

    function test_submitKmsContextDestructionOfSupersededContext() public {
        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);

        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsContextDestroyed(CTX1);
        _destroyContext(CTX1, SEED1, QUORUM);

        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1));
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1 + 1, EPOCH1 + 1));
    }

    function test_revertContextDestructionReplay() public {
        bytes[] memory signatures = _sign(_hashContextDestruction(CTX1 + 9, EXTRA), SEED0, QUORUM);
        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsContextDestroyed(CTX1 + 9);
        protocolConfigReplica.submitKmsContextDestruction(CTX1 + 9, signatures, EXTRA);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextAlreadyDestroyed.selector, CTX1 + 9));
        protocolConfigReplica.submitKmsContextDestruction(CTX1 + 9, signatures, EXTRA);

        // After a rotation the old signatures no longer match the committee, but the replay still
        // reports the destruction as applied.
        _switch(SEED0, CTX1 + 11, EPOCH1 + 11, SEED1);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextAlreadyDestroyed.selector, CTX1 + 9));
        protocolConfigReplica.submitKmsContextDestruction(CTX1 + 9, signatures, EXTRA);
    }

    function test_revertContextDestructionActive() public {
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.LatestActiveKmsContextCannotBeDestroyed.selector, CTX1)
        );
        _destroyContext(CTX1, SEED0, QUORUM);
    }

    /// @dev Canonical destroys pending C2 before it reaches the replica. The replica records C2 as
    ///      destroyed, so a late signature set for C2 cannot activate it (RFC 037 section 3.2).
    function test_destroyedNeverStoredContextCannotBeActivated() public {
        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsContextDestroyed(CTX1 + 1);
        _destroyContext(CTX1 + 1, SEED0, QUORUM);

        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.DestroyedKmsContextCannotBeActivated.selector, CTX1 + 1)
        );
        protocolConfigReplica.submitKmsContextSwitch(context, epoch);
    }

    function test_revertContextDestructionQuorum() public {
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, QUORUM - 1, QUORUM)
        );
        _destroyContext(CTX1 + 9, SEED0, QUORUM - 1);
    }

    function test_revertDestructionZeroQuorumCommittee() public {
        // With `mpc == n` the `n - t` quorum is 0, and the floor raises it to 1.
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(
            CTX1 + 10,
            EPOCH1 + 10,
            _nodes(SEED0),
            KmsThresholds({publicDecryption: 1, userDecryption: 2, kmsGen: 3, mpc: 4})
        );

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, 0, 1));
        protocolConfigReplica.submitKmsContextDestruction(CTX1 + 9, new bytes[](0), EXTRA);
    }

    function test_revertContextDestructionSignedByOldCommittee() public {
        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);

        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfigReplica.KmsSignerNotInCommittee.selector,
                _signers(SEED0)[0],
                CTX1 + 1
            )
        );
        _destroyContext(CTX1, SEED0, QUORUM);
    }

    /// @dev Canonical C2 (SEED1) destroys C1, but the replica switches to C3 (SEED2) before the
    ///      destruction arrives. The C2 signatures no longer match the active committee, so the owner
    ///      applies the destruction (accepted RFC 037 risk).
    function test_contextDestructionAfterRotationRecoversThroughMirror() public {
        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);
        bytes[] memory signatures = _sign(_hashContextDestruction(CTX1, EXTRA), SEED1, QUORUM);
        _switch(SEED1, CTX1 + 2, EPOCH1 + 2, SEED2);

        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfigReplica.KmsSignerNotInCommittee.selector,
                _signers(SEED1)[0],
                CTX1 + 2
            )
        );
        protocolConfigReplica.submitKmsContextDestruction(CTX1, signatures, EXTRA);

        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsContextDestroyed(CTX1);
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1);
        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1));

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextAlreadyDestroyed.selector, CTX1));
        protocolConfigReplica.submitKmsContextDestruction(CTX1, signatures, EXTRA);
    }

    function test_submitKmsEpochDestruction() public {
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N));

        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsEpochDestroyed(EPOCH1);
        _destroyEpoch(EPOCH1, SEED0, QUORUM);

        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 1));
    }

    function test_revertEpochDestructionReplay() public {
        bytes[] memory signatures = _sign(_hashEpochDestruction(EPOCH1 + 9, EXTRA), SEED0, QUORUM);
        protocolConfigReplica.submitKmsEpochDestruction(EPOCH1 + 9, signatures, EXTRA);

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochAlreadyDestroyed.selector, EPOCH1 + 9));
        protocolConfigReplica.submitKmsEpochDestruction(EPOCH1 + 9, signatures, EXTRA);

        // After a rotation the old signatures no longer match the committee, but the replay still
        // reports the destruction as applied.
        _switch(SEED0, CTX1 + 11, EPOCH1 + 11, SEED1);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochAlreadyDestroyed.selector, EPOCH1 + 9));
        protocolConfigReplica.submitKmsEpochDestruction(EPOCH1 + 9, signatures, EXTRA);
    }

    function test_revertEpochDestructionActive() public {
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.LatestActiveKmsEpochCannotBeDestroyed.selector, EPOCH1)
        );
        _destroyEpoch(EPOCH1, SEED0, QUORUM);
    }

    function test_revertEpochDestructionQuorum() public {
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, QUORUM - 1, QUORUM)
        );
        _destroyEpoch(EPOCH1 + 9, SEED0, QUORUM - 1);
    }

    function test_supersededEpochDestructionAfterReplicaCatchesUp() public {
        // Canonical activates E2, then E3, then destroys the superseded E2. The replica catches up first.
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N));
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigBase.LatestActiveKmsEpochCannotBeDestroyed.selector, EPOCH1 + 1)
        );
        _destroyEpoch(EPOCH1 + 1, SEED0, QUORUM);
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1 + 1, EPOCH1 + 2, SEED0, N));
        _destroyEpoch(EPOCH1 + 1, SEED0, QUORUM);

        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 1));
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 2));
    }

    /// @dev Canonical activates E2, then E3, then destroys E2. The replica is still at E1 and receives
    ///      the E2 destruction first. Both resharings then revert, so the owner recovers the replica.
    function test_supersededEpochDestructionAheadOfReplicaStallsResharings() public {
        KmsEpochSubmission memory epoch2 = _epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N);
        KmsEpochSubmission memory epoch3 = _epochSubmission(CTX1, EPOCH1 + 1, EPOCH1 + 2, SEED0, N);
        _destroyEpoch(EPOCH1 + 1, SEED0, QUORUM);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.DestroyedKmsEpochCannotBeActivated.selector, EPOCH1 + 1)
        );
        protocolConfigReplica.submitKmsKeyResharing(epoch2);
        // The E3 signatures commit to previous epoch E2, but the replica builds the digest from E1.
        _assertResharingRejected(epoch3);

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpoch(CTX1, EPOCH1 + 2);

        _assertActive(CTX1, EPOCH1 + 2);
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 1));
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 2));
    }

    function test_supersededContextDestructionWaitsForReplica() public {
        // Canonical switches from C1 (SEED0) to C2 (SEED1), then to C3 (SEED2), then C3 destroys C2.
        // The replica is at C1, so SEED2 is not its active committee.
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignerNotInCommittee.selector, _signers(SEED2)[0], CTX1)
        );
        _destroyContext(CTX1 + 1, SEED2, QUORUM);

        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);
        _switch(SEED1, CTX1 + 2, EPOCH1 + 2, SEED2);
        _destroyContext(CTX1 + 1, SEED2, QUORUM);

        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1 + 1));
        assertTrue(protocolConfigReplica.isValidKmsContext(CTX1 + 2));
    }

    function test_destroyedPendingEpochAppliesAfterNextResharing() public {
        // Canonical: E2 stays pending and is destroyed, then E3 activates with previous epoch E1.
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 2, SEED0, N));
        _destroyEpoch(EPOCH1 + 1, SEED0, QUORUM);

        _assertActive(CTX1, EPOCH1 + 2);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochAlreadyDestroyed.selector, EPOCH1 + 1));
        _destroyEpoch(EPOCH1 + 1, SEED0, QUORUM);
    }
}
