// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ProtocolConfigReplicaFixture} from "./ProtocolConfigReplicaTestBase.sol";
import {IProtocolConfigReplica, KmsContextSubmission, KmsEpochSubmission} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigReplica.sol";
import {KmsNode, KmsThresholds} from "@fhevm-host-contracts/contracts/shared/Structs.sol";

contract ProtocolConfigReplicaSubmitTest is ProtocolConfigReplicaFixture {
    // -----------------------------------------------------------------------
    // submitKmsContextSwitch
    // -----------------------------------------------------------------------

    function test_submitKmsContextSwitchActivates() public {
        uint256 contextId = CTX1 + 1;
        uint256 epochId = EPOCH1 + 1;

        vm.expectEmit(true, true, false, true, address(protocolConfigReplica));
        emit IProtocolConfigReplica.KmsContextSwitchSubmitted(contextId, epochId);
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, contextId, SEED1, SEED0, QUORUM),
            _epochSubmission(contextId, EPOCH1, epochId, SEED1, N)
        );

        _assertActive(contextId, epochId);
        assertEq(protocolConfigReplica.getCurrentKmsContextIdCounter(), contextId);
        assertTrue(protocolConfigReplica.isValidEpochForContext(contextId, epochId));
        _assertMembers(contextId, SEED1);
        assertEq(protocolConfigReplica.getKmsGenThresholdForContext(contextId), _thresholds().kmsGen);
        // The previous context stays valid, like on the canonical.
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1));
    }

    function test_submitKmsContextSwitchAcceptsAllPreviousSigners() public {
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, N),
            _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED1, N)
        );
        assertEq(protocolConfigReplica.getCurrentKmsContextId(), CTX1 + 1);
    }

    function test_revertSwitchQuorumNotReached() public {
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, QUORUM - 1, QUORUM)
        );
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM - 1),
            _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED1, N)
        );
    }

    /// @dev A switch from 4 nodes with t = 1 to 7 nodes with t = 2, then back to 4 nodes. Each switch needs
    ///      `n - t` signatures of the previous committee and all `n` signatures of the new committee.
    function test_switchBetweenCommitteeSizes() public {
        uint256 bigN = 7;
        KmsNode[] memory bigNodes = _nodes(SEED2, bigN);
        KmsThresholds memory bigThresholds = _thresholds();
        bigThresholds.mpc = 2;

        // 4 nodes with t = 1 to 7 nodes: the quorums are 3 previous and 7 new signatures.
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, 2, 3));
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 1, bigNodes, bigThresholds, SEED0, 2),
            _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED2, bigN)
        );
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, 6, 7));
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 1, bigNodes, bigThresholds, SEED0, 3),
            _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED2, 6)
        );
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 1, bigNodes, bigThresholds, SEED0, 3),
            _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED2, bigN)
        );
        _assertActive(CTX1 + 1, EPOCH1 + 1);
        assertEq(
            keccak256(abi.encode(protocolConfigReplica.getKmsNodesForContext(CTX1 + 1))),
            keccak256(abi.encode(bigNodes))
        );

        // 7 nodes with t = 2 to 4 nodes: the quorums are 5 previous and 4 new signatures.
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, 4, 5));
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1 + 1, CTX1 + 2, SEED1, SEED2, 4),
            _epochSubmission(CTX1 + 2, EPOCH1 + 1, EPOCH1 + 2, SEED1, N)
        );
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, 3, 4));
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1 + 1, CTX1 + 2, SEED1, SEED2, 5),
            _epochSubmission(CTX1 + 2, EPOCH1 + 1, EPOCH1 + 2, SEED1, N - 1)
        );
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1 + 1, CTX1 + 2, SEED1, SEED2, 5),
            _epochSubmission(CTX1 + 2, EPOCH1 + 1, EPOCH1 + 2, SEED1, N)
        );
        _assertActive(CTX1 + 2, EPOCH1 + 2);
        _assertMembers(CTX1 + 2, SEED1);
    }

    function test_revertSwitchEpochNotUnanimous() public {
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, N - 1, N));
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM),
            _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED1, N - 1)
        );
    }

    function test_revertSwitchEpochSignedByPreviousCommittee() public {
        (KmsContextSubmission memory context, ) = _defaultSwitch();
        _assertEpochRejected(context, _epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED0, N));
    }

    function test_revertSwitchDuplicateSigner() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        context.signatures[2] = context.signatures[0];

        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.DuplicateKmsSigner.selector, _signers(SEED0)[0]));
        protocolConfigReplica.submitKmsContextSwitch(context, epoch);
    }

    function test_revertSwitchNonMemberSigner() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        // A signer of the new committee only is not in the previous committee.
        context.signatures[2] = _sign(_contextDigest(CTX1, context), SEED1, 1)[0];

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsSignerNotInCommittee.selector, _signers(SEED1)[0], CTX1)
        );
        protocolConfigReplica.submitKmsContextSwitch(context, epoch);
    }

    function test_revertSwitchTamperedNodes() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        context.nodes[0].ipAddress = "10.0.0.1";
        _assertContextRejected(context, epoch);
    }

    /// @dev An invalid `mpc` also pins that the previous committee check runs before the store.
    function test_revertSwitchTamperedThresholds() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        context.thresholds.mpc = N + 1;
        _assertContextRejected(context, epoch);
    }

    function test_revertSwitchTamperedExtraData() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        context.extraData = hex"bad0";
        _assertContextRejected(context, epoch);
    }

    function test_revertSwitchTamperedContextId() public {
        KmsContextSubmission memory context = _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM);
        context.contextId = CTX1 + 2;
        _assertContextRejected(context, _epochSubmission(CTX1 + 2, EPOCH1, EPOCH1 + 1, SEED1, N));
    }

    function test_revertSwitchTamperedEpochMaterial() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        epoch.epochMaterialHash = keccak256("other material");
        _assertEpochRejected(context, epoch);
    }

    function test_revertSwitchTamperedEpochExtraData() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        epoch.extraData = hex"bad0";
        _assertEpochRejected(context, epoch);
    }

    function test_revertSwitchTamperedEpochId() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        epoch.epochId = EPOCH1 + 2;
        _assertEpochRejected(context, epoch);
    }

    function test_revertSwitchSignedForOtherPreviousContext() public {
        _assertContextRejected(
            _ctxSubmission(CTX1 + 5, CTX1 + 6, SEED1, SEED0, QUORUM),
            _epochSubmission(CTX1 + 6, EPOCH1, EPOCH1 + 1, SEED1, N)
        );
    }

    function test_revertSwitchSignedForOtherPreviousEpoch() public {
        (KmsContextSubmission memory context, ) = _defaultSwitch();
        _assertEpochRejected(context, _epochSubmission(CTX1 + 1, EPOCH1 + 3, EPOCH1 + 4, SEED1, N));
    }

    function test_revertSwitchEpochContextMismatch() public {
        (KmsContextSubmission memory context, ) = _defaultSwitch();
        _assertEpochRejected(context, _epochSubmission(CTX1 + 7, EPOCH1, EPOCH1 + 1, SEED1, N));
    }

    function test_revertSwitchReplayAlreadyApplied() public {
        (KmsContextSubmission memory context, KmsEpochSubmission memory epoch) = _defaultSwitch();
        protocolConfigReplica.submitKmsContextSwitch(context, epoch);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsContextSwitchAlreadyApplied.selector, CTX1 + 1)
        );
        protocolConfigReplica.submitKmsContextSwitch(context, epoch);
    }

    function test_revertSwitchOlderIdAlreadyApplied() public {
        _switch(SEED0, CTX1 + 5, EPOCH1 + 5, SEED1);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsContextSwitchAlreadyApplied.selector, CTX1 + 3)
        );
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 3, SEED2, SEED0, QUORUM),
            _epochSubmission(CTX1 + 3, EPOCH1, EPOCH1 + 3, SEED2, N)
        );
    }

    function test_revertSwitchEpochIdNotAboveActive() public {
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N));

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.NonIncreasingEpochId.selector, EPOCH1, EPOCH1 + 1)
        );
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM),
            _epochSubmission(CTX1 + 1, EPOCH1 + 1, EPOCH1, SEED1, N)
        );
    }

    function test_switchesApplyInCanonicalOrder() public {
        (KmsContextSubmission memory contextB, KmsEpochSubmission memory epochB) = _defaultSwitch();
        KmsContextSubmission memory contextC = _ctxSubmission(CTX1 + 1, CTX1 + 2, SEED2, SEED1, QUORUM);
        KmsEpochSubmission memory epochC = _epochSubmission(CTX1 + 2, EPOCH1 + 1, EPOCH1 + 2, SEED2, N);

        _assertContextRejected(contextC, epochC);

        protocolConfigReplica.submitKmsContextSwitch(contextB, epochB);
        protocolConfigReplica.submitKmsContextSwitch(contextC, epochC);

        _assertActive(CTX1 + 2, EPOCH1 + 2);
        _assertMembers(CTX1 + 2, SEED2);
    }

    function test_revertSwitchToDestroyedId() public {
        uint256 destroyedId = CTX1 + 1;
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextDestruction(destroyedId);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.DestroyedKmsContextCannotBeActivated.selector, destroyedId)
        );
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, destroyedId, SEED1, SEED0, QUORUM),
            _epochSubmission(destroyedId, EPOCH1, EPOCH1 + 1, SEED1, N)
        );
    }

    function test_revertSwitchToDestroyedEpoch() public {
        uint256 destroyedId = EPOCH1 + 1;
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpochDestruction(destroyedId);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.DestroyedKmsEpochCannotBeActivated.selector, destroyedId)
        );
        protocolConfigReplica.submitKmsContextSwitch(
            _ctxSubmission(CTX1, CTX1 + 1, SEED1, SEED0, QUORUM),
            _epochSubmission(CTX1 + 1, EPOCH1, destroyedId, SEED1, N)
        );
    }

    // -----------------------------------------------------------------------
    // submitKmsKeyResharing
    // -----------------------------------------------------------------------

    function test_submitKmsKeyResharingActivates() public {
        vm.expectEmit(true, true, false, true, address(protocolConfigReplica));
        emit IProtocolConfigReplica.KmsKeyResharingSubmitted(CTX1, EPOCH1 + 1);
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N));

        _assertActive(CTX1, EPOCH1 + 1);
        assertTrue(protocolConfigReplica.isValidEpochForContext(CTX1, EPOCH1 + 1));
    }

    function test_revertResharingNotUnanimous() public {
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfigReplica.KmsSignatureQuorumNotReached.selector, N - 1, N));
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N - 1));
    }

    function test_revertResharingTamperedEpochMaterial() public {
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N);
        epoch.epochMaterialHash = keccak256("other material");
        _assertResharingRejected(epoch);
    }

    function test_revertResharingTamperedExtraData() public {
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N);
        epoch.extraData = hex"bad0";
        _assertResharingRejected(epoch);
    }

    function test_revertResharingTamperedEpochId() public {
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N);
        epoch.epochId = EPOCH1 + 2;
        _assertResharingRejected(epoch);
    }

    function test_revertResharingSignedForOtherPreviousEpoch() public {
        _assertResharingRejected(_epochSubmission(CTX1, EPOCH1 + 1, EPOCH1 + 2, SEED0, N));
    }

    function test_revertResharingWrongContext() public {
        _assertResharingRejected(_epochSubmission(CTX1 + 1, EPOCH1, EPOCH1 + 1, SEED0, N));
    }

    function test_revertResharingReplayAlreadyApplied() public {
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N);
        protocolConfigReplica.submitKmsKeyResharing(epoch);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.KmsKeyResharingAlreadyApplied.selector, EPOCH1 + 1)
        );
        protocolConfigReplica.submitKmsKeyResharing(epoch);
    }

    function test_revertResharingDestroyedEpoch() public {
        uint256 destroyedId = EPOCH1 + 1;
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpochDestruction(destroyedId);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.DestroyedKmsEpochCannotBeActivated.selector, destroyedId)
        );
        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, destroyedId, SEED0, N));
    }
}
