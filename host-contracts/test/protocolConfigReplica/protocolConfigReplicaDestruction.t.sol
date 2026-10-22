// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ProtocolConfigReplicaTestBase} from "./protocolConfigReplica.t.sol";
import {IProtocolConfigReplica} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigReplica.sol";
import {IProtocolConfigBase} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigBase.sol";
import {KmsThresholds} from "@fhevm-host-contracts/contracts/shared/Structs.sol";

contract ProtocolConfigReplicaDestructionTest is ProtocolConfigReplicaTestBase {
    // -----------------------------------------------------------------------
    // submitKmsContextDestruction / submitKmsEpochDestruction
    // -----------------------------------------------------------------------

    function test_submitKmsContextDestructionOfSupersededContext() public {
        _setupReplica();
        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);

        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsContextDestroyed(CTX1);
        _destroyContext(CTX1, SEED1, QUORUM);

        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1));
        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1 + 1, EPOCH1 + 1));
    }

    /// @dev Moves the replica to `CTX1 + 10` and `EPOCH1 + 10` under committee SEED0, so the IDs
    ///      below are never stored and not ahead of the replica.
    function _mirrorAhead() internal {
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 10, EPOCH1 + 10, _nodes(SEED0), _thresholds());
    }

    function test_revertContextDestructionReplay() public {
        _setupReplica();
        _mirrorAhead();
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
        _setupReplica();
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsContextDestructionAheadOfReplica.selector, CTX1, CTX1)
        );
        _destroyContext(CTX1, SEED0, QUORUM);
    }

    function test_revertContextDestructionQuorum() public {
        _setupReplica();
        _mirrorAhead();
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, QUORUM - 1, QUORUM)
        );
        _destroyContext(CTX1 + 9, SEED0, QUORUM - 1);
    }

    function test_revertDestructionZeroQuorumCommittee() public {
        _setupReplica();
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
        _setupReplica();
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

    function test_submitKmsEpochDestruction() public {
        _setupReplica();
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N));

        vm.expectEmit(true, false, false, false, address(protocolConfigReplica));
        emit IProtocolConfigBase.KmsEpochDestroyed(EPOCH1);
        _destroyEpoch(EPOCH1, SEED0, QUORUM);

        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 1));
    }

    function test_revertEpochDestructionReplay() public {
        _setupReplica();
        _mirrorAhead();
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
        _setupReplica();
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochDestructionAheadOfReplica.selector, EPOCH1, EPOCH1)
        );
        _destroyEpoch(EPOCH1, SEED0, QUORUM);
    }

    function test_revertEpochDestructionQuorum() public {
        _setupReplica();
        _mirrorAhead();
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, QUORUM - 1, QUORUM)
        );
        _destroyEpoch(EPOCH1 + 9, SEED0, QUORUM - 1);
    }

    function test_supersededEpochDestructionWaitsForReplica() public {
        _setupReplica();
        // Canonical activates E2, then E3, then destroys the superseded E2. The replica is still at E1.
        vm.expectRevert(
            abi.encodeWithSelector(
                IProtocolConfigReplica.KmsEpochDestructionAheadOfReplica.selector,
                EPOCH1 + 1,
                EPOCH1
            )
        );
        _destroyEpoch(EPOCH1 + 1, SEED2, QUORUM);

        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N));
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1 + 1, EPOCH1 + 2, SEED0, N));
        _destroyEpoch(EPOCH1 + 1, SEED0, QUORUM);

        assertFalse(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 1));
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 2));
    }

    function test_supersededContextDestructionWaitsForReplica() public {
        _setupReplica();
        // Canonical switches from C1 (SEED0) to C2 (SEED1), then to C3 (SEED2), then C3 destroys C2.
        // The replica is at C1.
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsContextDestructionAheadOfReplica.selector, CTX1 + 1, CTX1)
        );
        _destroyContext(CTX1 + 1, SEED2, QUORUM);

        _switch(SEED0, CTX1 + 1, EPOCH1 + 1, SEED1);
        _switch(SEED1, CTX1 + 2, EPOCH1 + 2, SEED2);
        _destroyContext(CTX1 + 1, SEED2, QUORUM);

        assertFalse(protocolConfigReplica.isValidKmsContext(CTX1 + 1));
        assertTrue(protocolConfigReplica.isValidKmsContext(CTX1 + 2));
    }

    function test_destroyedPendingEpochAppliesAfterNextResharing() public {
        _setupReplica();
        // Canonical: E2 stays pending and is destroyed, then E3 activates with previous epoch E1.
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 2, SEED0, N));
        _destroyEpoch(EPOCH1 + 1, SEED0, QUORUM);

        (, uint256 activeEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(activeEpochId, EPOCH1 + 2);
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsEpochAlreadyDestroyed.selector, EPOCH1 + 1));
        _destroyEpoch(EPOCH1 + 1, SEED0, QUORUM);
    }
}
