// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ProtocolConfigReplicaFixture} from "./ProtocolConfigReplicaTestBase.sol";
import {IProtocolConfigReplica, KmsContextSubmission, KmsEpochSubmission} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigReplica.sol";

/// @dev The owner recovers the replica with the mirror functions, and the submit path keeps working.
contract ProtocolConfigReplicaRecoveryTest is ProtocolConfigReplicaFixture {
    /// @dev The owner moves the replica back to the bootstrap pair and destroys `CTX1 + 1`.
    function _recoverToGenesis() internal {
        vm.startPrank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1, EPOCH1, _nodes(SEED0), _thresholds());
        protocolConfigReplica.mirrorKmsContextDestruction(CTX1 + 1);
        vm.stopPrank();
    }

    function test_recoveryFromLateActivation() public {
        // Canonical: C2 stays pending and is destroyed, then C1 switches to C3. A late signature set
        // activates C2 on the replica.
        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);
        _recoverToGenesis();

        // The owner flag settles the destruction, so the signed path reports it as applied.
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsContextAlreadyDestroyed.selector, CTX1 + 1));
        _destroyContext(CTX1 + 1, SEED0, QUORUM);

        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 2, SEED2, SEED0, QUORUM),
            _epochSubmission(CTX1 + 2, EPOCH1, EPOCH1 + 2, SEED2, N)
        );

        _assertActive(CTX1 + 2, EPOCH1 + 2);
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
        _assertActive(forgedId, EPOCH1 + 2);
    }

    function test_recoveryBlocksContextSwitchReplay() public {
        uint256 forgedId = CTX1 + 1;
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        protocolConfigReplica.submitKmsContextSwitch(context, epoch);
        _recoverToGenesis();

        _assertActive(CTX1, EPOCH1);
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

        _assertActive(CTX1, EPOCH1);
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
        _recoverToGenesis();
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpochDestruction(forgedEpochId);

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
}
