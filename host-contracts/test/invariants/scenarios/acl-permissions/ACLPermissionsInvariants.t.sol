// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ACL} from "../../../../contracts/ACL.sol";
import {PauserSet} from "../../../../contracts/immutable/PauserSet.sol";
import {aclAdd, pauserSetAdd} from "../../../../addresses/FHEVMHostAddresses.sol";
import {BaseScenarioInvariant} from "../../base/BaseScenarioInvariant.t.sol";
import {ACLPermissionsHandler} from "./ACLPermissionsHandler.t.sol";

contract ACLPermissionsInvariants is BaseScenarioInvariant {
    address internal constant OWNER = address(0xA11);
    address internal constant PAUSER = address(0xA12);
    address internal constant ACTOR_1 = address(0x1201);
    address internal constant ACTOR_2 = address(0x1202);
    address internal constant ACTOR_3 = address(0x1203);

    ACL internal acl;
    ACLPermissionsHandler internal handler;

    function setUp() public {
        _deployACL(OWNER);
        _deployPauserSet();
        vm.prank(OWNER);
        PauserSet(pauserSetAdd).addPauser(PAUSER);

        acl = ACL(aclAdd);
        handler = new ACLPermissionsHandler(acl, OWNER, PAUSER, ACTOR_1, ACTOR_2, ACTOR_3);

        address[] memory senders = new address[](5);
        senders[0] = OWNER;
        senders[1] = PAUSER;
        senders[2] = ACTOR_1;
        senders[3] = ACTOR_2;
        senders[4] = ACTOR_3;

        bytes4[] memory selectors = new bytes4[](8);
        selectors[0] = ACLPermissionsHandler.allow.selector;
        selectors[1] = ACLPermissionsHandler.allowTransient.selector;
        selectors[2] = ACLPermissionsHandler.allowForDecryption.selector;
        selectors[3] = ACLPermissionsHandler.pause.selector;
        selectors[4] = ACLPermissionsHandler.unpause.selector;
        selectors[5] = ACLPermissionsHandler.blockAccount.selector;
        selectors[6] = ACLPermissionsHandler.unblockAccount.selector;
        selectors[7] = ACLPermissionsHandler.cleanTransientStorage.selector;
        _targetInvariant(address(handler), selectors, senders);
    }

    /// @custom:invariant ACL-PERM-001
    function invariant_SuccessfulPermissionWritesPersist() public view {
        uint256 count = handler.successfulAllowCount();
        for (uint256 i = 0; i < count; i++) {
            (bytes32 handle, address account) = handler.getSuccessfulAllowPair(i);
            assertTrue(acl.persistAllowed(handle, account));
        }

        uint256 decryptCount = handler.successfulDecryptHandleCount();
        for (uint256 i = 0; i < decryptCount; i++) {
            bytes32 handle = handler.getSuccessfulDecryptHandle(i);
            assertTrue(acl.isAllowedForDecryption(handle));
        }
    }

    /// @custom:invariant ACL-PERM-002
    function invariant_EnforcementGuardsHold() public view {
        assertFalse(handler.privilegeViolation());
        assertFalse(handler.deniedBypassViolation());
        assertFalse(handler.senderPermissionBypassViolation());
        assertFalse(handler.transientVisibilityViolation());
    }

    /// @custom:invariant ACL-PERM-003
    function invariant_PausedMutationsCannotSucceed() public view {
        assertFalse(handler.pausedBypassViolation());
    }
}
