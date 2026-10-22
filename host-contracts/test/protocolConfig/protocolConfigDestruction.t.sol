// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Vm} from "forge-std/Test.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {ProtocolConfigTestBase} from "./protocolConfig.t.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {KmsNodeParams} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";

/// @dev EIP-712 destruction confirmations.
contract ProtocolConfigDestructionTest is ProtocolConfigTestBase {
    /// @dev Defines a switch to the disjoint committee {signer2, signer3}, confirms its creation, then
    ///      destroys it with its pending epoch. The genesis committee {signer0, signer1} stays active.
    function _destroyCreatedDisjointContext() internal returns (uint256 contextId) {
        contextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _defineDisjointSwitch();
        _confirmContextCreationWithTwoSigners(contextId);
        _confirmContextCreation(contextId, kmsPk2, "");
        _confirmContextCreation(contextId, kmsPk3, "");
        vm.prank(owner);
        protocolConfig.destroyKmsContext(contextId);
    }

    function test_revertConfirmKmsContextDestructionBeforeDestroy() public {
        _setupEpochLifecycle();
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds());
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 2;

        bytes memory signature = _computeSignature(kmsPk0, _hashContextDestruction(contextId, ""));
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.KmsContextNotDestroyed.selector, contextId));
        protocolConfig.confirmKmsContextDestruction(contextId, signature, "");
    }

    /// @dev Only the active committee confirms, once per signer, from any sender. extraData is
    ///      digest material only, so signers may confirm different extraData.
    function test_confirmKmsContextDestruction() public {
        _setupEpochLifecycle();
        uint256 contextId = _destroyCreatedDisjointContext();

        // signer2 belongs to the destroyed committee only.
        bytes memory signature = _computeSignature(kmsPk2, _hashContextDestruction(contextId, ""));
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsDestructionUnauthorized.selector, vm.addr(kmsPk2), contextId)
        );
        protocolConfig.confirmKmsContextDestruction(contextId, signature, "");

        bytes memory extraData = hex"01ab";
        signature = _computeSignature(kmsPk0, _hashContextDestruction(contextId, extraData));
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextDestructionConfirmation(contextId, vm.addr(kmsPk0), signature, extraData);
        vm.prank(address(0x999));
        protocolConfig.confirmKmsContextDestruction(contextId, signature, extraData);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsDestructionAlreadyConfirmed.selector, vm.addr(kmsPk0), contextId)
        );
        protocolConfig.confirmKmsContextDestruction(contextId, signature, extraData);

        signature = _computeSignature(kmsPk1, _hashContextDestruction(contextId, ""));
        protocolConfig.confirmKmsContextDestruction(contextId, signature, "");
    }

    /// @dev After a switch to {signer2, signer3}, the superseded genesis epoch is destroyed. Its own
    ///      committee no longer confirms, the active one does, once per signer.
    function test_confirmKmsEpochDestruction() public {
        _setupEpochLifecycle();
        _defineDisjointSwitch();
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _confirmContextCreationWithTwoSigners(newContextId);
        _confirmContextCreation(newContextId, kmsPk2, "");
        _confirmContextCreation(newContextId, kmsPk3, "");
        _confirmEpochActivation(newContextId, EPOCH_COUNTER_BASE + 2, kmsPk2);
        _confirmEpochActivation(newContextId, EPOCH_COUNTER_BASE + 2, kmsPk3);

        uint256 epochId = EPOCH_COUNTER_BASE + 1;
        vm.prank(owner);
        protocolConfig.destroyKmsEpoch(epochId);

        bytes memory signature = _computeSignature(kmsPk0, _hashEpochDestruction(epochId, ""));
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsDestructionUnauthorized.selector, vm.addr(kmsPk0), epochId)
        );
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, "");

        bytes memory extraData = hex"01ab";
        signature = _computeSignature(kmsPk2, _hashEpochDestruction(epochId, extraData));
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsEpochDestructionConfirmation(epochId, vm.addr(kmsPk2), signature, extraData);
        vm.prank(address(0x999));
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, extraData);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsDestructionAlreadyConfirmed.selector, vm.addr(kmsPk2), epochId)
        );
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, extraData);
    }

    /// @dev Switches from the genesis committee A = {signer0, signer1} to B = {signer1, signer2} and activates
    ///      its epoch `newEpochId`. signer1 belongs to both committees.
    function _rotateToSharedCommittee(uint256 newEpochId) internal {
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[0].signerAddress = vm.addr(kmsPk1);
        nodes[1].signerAddress = vm.addr(kmsPk2);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());
        uint256 contextId = protocolConfig.getCurrentKmsContextIdCounter();
        _confirmContextCreation(contextId, kmsPk0, "");
        _confirmContextCreation(contextId, kmsPk1, "");
        _confirmContextCreation(contextId, kmsPk2, "");
        _confirmEpochActivation(contextId, newEpochId, kmsPk1);
        _confirmEpochActivation(contextId, newEpochId, kmsPk2);
        (uint256 activeContextId, uint256 activeEpochId) = protocolConfig.getCurrentKmsContextAndEpoch();
        assertEq(activeContextId, contextId);
        assertEq(activeEpochId, newEpochId);
    }

    /// @dev A context destroyed under committee A is confirmed by committee B after the rotation.
    ///      An A-only signer is rejected. A signer of both committees confirms once, and its confirmation
    ///      under A does not count toward B's quorum.
    function test_confirmKmsContextDestructionAcrossRotation() public {
        _setupEpochLifecycle();
        uint256 contextId = _destroyCreatedDisjointContext();
        bytes memory signature1 = _computeSignature(kmsPk1, _hashContextDestruction(contextId, ""));
        protocolConfig.confirmKmsContextDestruction(contextId, signature1, "");

        _rotateToSharedCommittee(EPOCH_COUNTER_BASE + 3);

        bytes memory signature = _computeSignature(kmsPk0, _hashContextDestruction(contextId, ""));
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsDestructionUnauthorized.selector, vm.addr(kmsPk0), contextId)
        );
        protocolConfig.confirmKmsContextDestruction(contextId, signature, "");

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsDestructionAlreadyConfirmed.selector, vm.addr(kmsPk1), contextId)
        );
        protocolConfig.confirmKmsContextDestruction(contextId, signature1, "");

        signature = _computeSignature(kmsPk2, _hashContextDestruction(contextId, ""));
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextDestructionConfirmation(contextId, vm.addr(kmsPk2), signature, "");
        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextDestructionQuorumReached(contextId, "");
        protocolConfig.confirmKmsContextDestruction(contextId, signature, "");
    }

    /// @dev An epoch destroyed under committee A is confirmed by committee B after the rotation.
    ///      An A-only signer is rejected. A signer of both committees confirms once.
    function test_confirmKmsEpochDestructionAcrossRotation() public {
        _setupEpochLifecycle();
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.destroyKmsEpoch(epochId);
        bytes memory signature1 = _computeSignature(kmsPk1, _hashEpochDestruction(epochId, ""));
        protocolConfig.confirmKmsEpochDestruction(epochId, signature1, "");

        _rotateToSharedCommittee(EPOCH_COUNTER_BASE + 3);

        bytes memory signature = _computeSignature(kmsPk0, _hashEpochDestruction(epochId, ""));
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsDestructionUnauthorized.selector, vm.addr(kmsPk0), epochId)
        );
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, "");

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsDestructionAlreadyConfirmed.selector, vm.addr(kmsPk1), epochId)
        );
        protocolConfig.confirmKmsEpochDestruction(epochId, signature1, "");

        signature = _computeSignature(kmsPk2, _hashEpochDestruction(epochId, ""));
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsEpochDestructionConfirmation(epochId, vm.addr(kmsPk2), signature, "");
        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsEpochDestructionQuorumReached(epochId, "");
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, "");
    }

    /// @dev With n = 4 and t = 1 the quorum is 3. A confirmation of another extraData counts apart,
    ///      so the event fires on the third confirmation of one digest.
    function test_kmsEpochDestructionQuorum() public {
        _setupDefaultWithMpcThreshold(1);
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.destroyKmsEpoch(epochId);
        bytes memory extraData = hex"01";

        vm.recordLogs();
        protocolConfig.confirmKmsEpochDestruction(
            epochId,
            _signEpochDestruction(kmsPk0, epochId, extraData),
            extraData
        );
        protocolConfig.confirmKmsEpochDestruction(
            epochId,
            _signEpochDestruction(kmsPk1, epochId, extraData),
            extraData
        );
        protocolConfig.confirmKmsEpochDestruction(epochId, _signEpochDestruction(kmsPk2, epochId, ""), "");
        assertEq(_countLogs(IProtocolConfig.KmsEpochDestructionQuorumReached.selector), 0);

        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsEpochDestructionQuorumReached(epochId, extraData);
        protocolConfig.confirmKmsEpochDestruction(
            epochId,
            _signEpochDestruction(kmsPk3, epochId, extraData),
            extraData
        );
    }

    /// @dev Same quorum rule for a destroyed context. n = 4, t = 1, so the quorum is 3.
    function test_kmsContextDestructionQuorum() public {
        _setupDefaultWithMpcThreshold(1);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(_makeKmsNodeParams(1), _defaultThresholds());
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 2;
        vm.prank(owner);
        protocolConfig.destroyKmsContext(contextId);

        vm.recordLogs();
        protocolConfig.confirmKmsContextDestruction(contextId, _signContextDestruction(kmsPk0, contextId), "");
        protocolConfig.confirmKmsContextDestruction(contextId, _signContextDestruction(kmsPk1, contextId), "");
        assertEq(_countLogs(IProtocolConfig.KmsContextDestructionQuorumReached.selector), 0);

        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextDestructionQuorumReached(contextId, "");
        protocolConfig.confirmKmsContextDestruction(contextId, _signContextDestruction(kmsPk2, contextId), "");
    }

    /// @dev OZ ECDSA rejects the high-s twin of a valid destruction signature.
    function test_revertConfirmDestructionMalleableSignature() public {
        _setupEpochLifecycle();
        uint256 contextId = _destroyCreatedDisjointContext();
        (bytes memory signature, bytes32 highS) = _highSSignature(kmsPk0, _hashContextDestruction(contextId, ""));
        vm.expectRevert(abi.encodeWithSelector(ECDSA.ECDSAInvalidSignatureS.selector, highS));
        protocolConfig.confirmKmsContextDestruction(contextId, signature, "");

        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        uint256 epochId = EPOCH_COUNTER_BASE + 3;
        vm.prank(owner);
        protocolConfig.destroyKmsEpoch(epochId);
        (signature, highS) = _highSSignature(kmsPk0, _hashEpochDestruction(epochId, ""));
        vm.expectRevert(abi.encodeWithSelector(ECDSA.ECDSAInvalidSignatureS.selector, highS));
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, "");
    }

    function _signContextDestruction(uint256 pk, uint256 contextId) internal view returns (bytes memory) {
        return _computeSignature(pk, _hashContextDestruction(contextId, ""));
    }

    function _signEpochDestruction(
        uint256 pk,
        uint256 epochId,
        bytes memory extraData
    ) internal view returns (bytes memory) {
        return _computeSignature(pk, _hashEpochDestruction(epochId, extraData));
    }

    function _countLogs(bytes32 topic) internal view returns (uint256 count) {
        Vm.Log[] memory logs = vm.getRecordedLogs();
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].topics[0] == topic) {
                count++;
            }
        }
    }
}
