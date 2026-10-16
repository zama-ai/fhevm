// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ProtocolConfigTestBase} from "./protocolConfig.t.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {KmsNodeParams} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";

/// @dev EIP-712 destruction confirmations.
contract ProtocolConfigDestructionTest is ProtocolConfigTestBase {
    /// @dev Defines a switch to the disjoint committee {signer2, signer3}, confirms its creation, then
    ///      destroys it with its pending epoch. The genesis committee {signer0, signer1} stays active.
    function _destroyCreatedDisjointContext() internal returns (uint256 contextId, uint256 epochId) {
        contextId = KMS_CONTEXT_COUNTER_BASE + 2;
        epochId = EPOCH_COUNTER_BASE + 2;
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[0].signerAddress = vm.addr(kmsPk2);
        nodes[1].signerAddress = vm.addr(kmsPk3);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());
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

        uint256[] memory destroyedEpochIds = new uint256[](0);
        bytes memory signature = _computeSignature(kmsPk0, _hashContextDestruction(contextId, destroyedEpochIds, ""));
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.KmsContextNotDestroyed.selector, contextId));
        protocolConfig.confirmKmsContextDestruction(contextId, destroyedEpochIds, signature, "");
    }

    /// @dev Only the active committee confirms, once per signer, from any sender. destroyedEpochIds is
    ///      digest material only, so both the cleared epoch and an empty list are accepted.
    function test_confirmKmsContextDestruction() public {
        _setupEpochLifecycle();
        (uint256 contextId, uint256 epochId) = _destroyCreatedDisjointContext();
        uint256[] memory destroyedEpochIds = new uint256[](1);
        destroyedEpochIds[0] = epochId;

        // signer2 belongs to the destroyed committee only.
        bytes memory signature = _computeSignature(kmsPk2, _hashContextDestruction(contextId, destroyedEpochIds, ""));
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsDestructionUnauthorized.selector, vm.addr(kmsPk2), contextId)
        );
        protocolConfig.confirmKmsContextDestruction(contextId, destroyedEpochIds, signature, "");

        bytes memory extraData = hex"01ab";
        signature = _computeSignature(kmsPk0, _hashContextDestruction(contextId, destroyedEpochIds, extraData));
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextDestructionConfirmed(
            contextId,
            destroyedEpochIds,
            vm.addr(kmsPk0),
            signature,
            extraData
        );
        vm.prank(address(0x999));
        protocolConfig.confirmKmsContextDestruction(contextId, destroyedEpochIds, signature, extraData);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsDestructionAlreadyConfirmed.selector, vm.addr(kmsPk0), contextId)
        );
        protocolConfig.confirmKmsContextDestruction(contextId, destroyedEpochIds, signature, extraData);

        uint256[] memory noEpochIds = new uint256[](0);
        signature = _computeSignature(kmsPk1, _hashContextDestruction(contextId, noEpochIds, ""));
        protocolConfig.confirmKmsContextDestruction(contextId, noEpochIds, signature, "");
    }

    /// @dev An epoch cleared by destroyKmsContext is confirmed through the context destruction.
    function test_revertConfirmKmsEpochDestructionForEpochClearedWithContext() public {
        _setupEpochLifecycle();
        (, uint256 epochId) = _destroyCreatedDisjointContext();

        bytes memory signature = _computeSignature(kmsPk0, _hashEpochDestruction(epochId, ""));
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.KmsEpochNotDestroyed.selector, epochId));
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, "");
    }

    function test_revertConfirmKmsEpochDestructionBeforeDestroy() public {
        _setupEpochLifecycle();
        vm.prank(owner);
        protocolConfig.defineNewEpochForCurrentKmsContext();
        uint256 epochId = EPOCH_COUNTER_BASE + 2;

        bytes memory signature = _computeSignature(kmsPk0, _hashEpochDestruction(epochId, ""));
        vm.expectRevert(abi.encodeWithSelector(IProtocolConfig.KmsEpochNotDestroyed.selector, epochId));
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, "");
    }

    /// @dev After a switch to {signer2, signer3}, the superseded genesis epoch is destroyed. Its own
    ///      committee no longer confirms, the active one does, once per signer.
    function test_confirmKmsEpochDestruction() public {
        _setupEpochLifecycle();
        KmsNodeParams[] memory nodes = _makeKmsNodeParams(2);
        nodes[0].signerAddress = vm.addr(kmsPk2);
        nodes[1].signerAddress = vm.addr(kmsPk3);
        vm.prank(owner);
        _defineNewKmsContextAndEpoch(nodes, _defaultThresholds());
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
        emit IProtocolConfig.KmsEpochDestructionConfirmed(epochId, vm.addr(kmsPk2), signature, extraData);
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
    ///      An A-only signer is rejected. A signer of both committees confirms once.
    function test_confirmKmsContextDestructionAcrossRotation() public {
        _setupEpochLifecycle();
        (uint256 contextId, uint256 epochId) = _destroyCreatedDisjointContext();
        uint256[] memory destroyedEpochIds = new uint256[](1);
        destroyedEpochIds[0] = epochId;
        bytes memory signature1 = _computeSignature(kmsPk1, _hashContextDestruction(contextId, destroyedEpochIds, ""));
        protocolConfig.confirmKmsContextDestruction(contextId, destroyedEpochIds, signature1, "");

        _rotateToSharedCommittee(EPOCH_COUNTER_BASE + 3);

        bytes memory signature = _computeSignature(kmsPk0, _hashContextDestruction(contextId, destroyedEpochIds, ""));
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsDestructionUnauthorized.selector, vm.addr(kmsPk0), contextId)
        );
        protocolConfig.confirmKmsContextDestruction(contextId, destroyedEpochIds, signature, "");

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsDestructionAlreadyConfirmed.selector, vm.addr(kmsPk1), contextId)
        );
        protocolConfig.confirmKmsContextDestruction(contextId, destroyedEpochIds, signature1, "");

        signature = _computeSignature(kmsPk2, _hashContextDestruction(contextId, destroyedEpochIds, ""));
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextDestructionConfirmed(
            contextId,
            destroyedEpochIds,
            vm.addr(kmsPk2),
            signature,
            ""
        );
        protocolConfig.confirmKmsContextDestruction(contextId, destroyedEpochIds, signature, "");
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
        emit IProtocolConfig.KmsEpochDestructionConfirmed(epochId, vm.addr(kmsPk2), signature, "");
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, "");
    }
}
