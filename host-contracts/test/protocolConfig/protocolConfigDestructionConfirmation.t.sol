// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Vm} from "forge-std/Test.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {ProtocolConfigTestBase} from "./protocolConfig.t.sol";
import {IProtocolConfig} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";

/// @dev EIP-712 destruction confirmations.
contract ProtocolConfigDestructionConfirmationTest is ProtocolConfigTestBase {
    /// @dev Defines a switch to the disjoint committee {signer2, signer3}, confirms its creation, then
    ///      destroys it with its pending epoch. The genesis committee {signer0, signer1} stays active.
    function _destroyCreatedDisjointContext() internal returns (uint256 contextId) {
        contextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _defineDisjointSwitch();
        _confirmDisjointSwitchCreation(contextId);
        vm.prank(owner);
        protocolConfig.destroyKmsContext(contextId);
    }

    /// @dev Only the active committee confirms, once per signer, from any sender. extraData is
    ///      digest material only, so signers may confirm different extraData.
    function test_confirmKmsContextDestruction() public {
        _setupEpochLifecycle();
        uint256 contextId = _destroyCreatedDisjointContext();

        // signer2 belongs to the destroyed committee only.
        bytes memory signature = _signContextDestruction(contextId, kmsPk2, "");
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsDestructionUnauthorized.selector, vm.addr(kmsPk2), contextId)
        );
        protocolConfig.confirmKmsContextDestruction(contextId, signature, "");

        bytes memory extraData = hex"01ab";
        signature = _signContextDestruction(contextId, kmsPk0, extraData);
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextDestructionConfirmation(contextId, vm.addr(kmsPk0), signature, extraData);
        vm.prank(address(0x999));
        protocolConfig.confirmKmsContextDestruction(contextId, signature, extraData);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsDestructionAlreadyConfirmed.selector, vm.addr(kmsPk0), contextId)
        );
        protocolConfig.confirmKmsContextDestruction(contextId, signature, extraData);

        _confirmContextDestruction(contextId, kmsPk1, "");
    }

    /// @dev After a switch to {signer2, signer3}, the superseded genesis epoch is destroyed. Its own
    ///      committee no longer confirms, the active one does, once per signer.
    function test_confirmKmsEpochDestruction() public {
        _setupEpochLifecycle();
        _defineDisjointSwitch();
        uint256 newContextId = KMS_CONTEXT_COUNTER_BASE + 2;
        _confirmDisjointSwitchCreation(newContextId);
        _confirmEpochActivationWithDummyMaterial(newContextId, EPOCH_COUNTER_BASE + 2, kmsPk2);
        _confirmEpochActivationWithDummyMaterial(newContextId, EPOCH_COUNTER_BASE + 2, kmsPk3);

        uint256 epochId = EPOCH_COUNTER_BASE + 1;
        vm.prank(owner);
        protocolConfig.destroyKmsEpoch(epochId);

        bytes memory signature = _signEpochDestruction(epochId, kmsPk0, "");
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsDestructionUnauthorized.selector, vm.addr(kmsPk0), epochId)
        );
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, "");

        bytes memory extraData = hex"01ab";
        signature = _signEpochDestruction(epochId, kmsPk2, extraData);
        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsEpochDestructionConfirmation(epochId, vm.addr(kmsPk2), signature, extraData);
        vm.prank(address(0x999));
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, extraData);

        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfig.KmsDestructionAlreadyConfirmed.selector, vm.addr(kmsPk2), epochId)
        );
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, extraData);
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
        _confirmEpochDestruction(epochId, kmsPk0, extraData);
        _confirmEpochDestruction(epochId, kmsPk1, extraData);
        _confirmEpochDestruction(epochId, kmsPk2, "");
        assertEq(_countLogs(IProtocolConfig.KmsEpochDestructionQuorumReached.selector), 0);

        vm.expectEmit(true, false, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsEpochDestructionQuorumReached(epochId, extraData);
        _confirmEpochDestruction(epochId, kmsPk3, extraData);
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
        _confirmContextDestruction(contextId, kmsPk0, "");
        _confirmContextDestruction(contextId, kmsPk1, "");
        assertEq(_countLogs(IProtocolConfig.KmsContextDestructionQuorumReached.selector), 0);

        vm.expectEmit(true, true, false, true, address(protocolConfig));
        emit IProtocolConfig.KmsContextDestructionQuorumReached(contextId, "");
        _confirmContextDestruction(contextId, kmsPk2, "");
    }

    /// @dev Each confirmation type has its own type hash. An EpochDestructionConfirmation signature
    ///      over a destroyed context's ID does not count as its ContextDestructionConfirmation.
    function test_revertCrossTypeReplay() public {
        _setupEpochLifecycle();
        uint256 contextId = _defineSameSetSwitch();
        vm.prank(owner);
        protocolConfig.destroyKmsContext(contextId);

        bytes memory signature = _computeSignature(kmsPk0, _hashEpochDestruction(contextId, ""));
        vm.expectPartialRevert(IProtocolConfig.KmsDestructionUnauthorized.selector);
        protocolConfig.confirmKmsContextDestruction(contextId, signature, "");

        signature = _signContextCreation(contextId, kmsPk0, "");
        vm.expectPartialRevert(IProtocolConfig.KmsDestructionUnauthorized.selector);
        protocolConfig.confirmKmsContextDestruction(contextId, signature, "");
    }

    /// @dev OZ ECDSA rejects the high-s twin of a valid destruction signature.
    function test_revertConfirmDestructionMalleableSignature() public {
        _setupEpochLifecycle();
        uint256 contextId = KMS_CONTEXT_COUNTER_BASE + 2;
        (bytes memory signature, bytes32 highS) = _highSSignature(kmsPk0, _hashContextDestruction(contextId, ""));
        vm.expectRevert(abi.encodeWithSelector(ECDSA.ECDSAInvalidSignatureS.selector, highS));
        protocolConfig.confirmKmsContextDestruction(contextId, signature, "");

        uint256 epochId = EPOCH_COUNTER_BASE + 2;
        (signature, highS) = _highSSignature(kmsPk0, _hashEpochDestruction(epochId, ""));
        vm.expectRevert(abi.encodeWithSelector(ECDSA.ECDSAInvalidSignatureS.selector, highS));
        protocolConfig.confirmKmsEpochDestruction(epochId, signature, "");
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
