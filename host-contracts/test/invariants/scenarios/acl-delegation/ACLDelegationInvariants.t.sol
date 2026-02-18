// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ACL} from "../../../../contracts/ACL.sol";
import {PauserSet} from "../../../../contracts/immutable/PauserSet.sol";
import {aclAdd, pauserSetAdd} from "../../../../addresses/FHEVMHostAddresses.sol";
import {BaseScenarioInvariant} from "../../base/BaseScenarioInvariant.t.sol";
import {ACLDelegationHandler} from "./ACLDelegationHandler.t.sol";

contract ACLDelegationInvariants is BaseScenarioInvariant {
    uint8 internal constant STATUS_ACTIVE = 1;
    uint8 internal constant STATUS_REVOKED = 2;

    address internal constant OWNER = address(0xA11);
    address internal constant PAUSER = address(0xA12);
    address internal constant ACTOR_1 = address(0x1201);
    address internal constant ACTOR_2 = address(0x1202);
    address internal constant ACTOR_3 = address(0x1203);

    ACL internal acl;
    ACLDelegationHandler internal handler;

    function setUp() public {
        _deployACL(OWNER);
        _deployPauserSet();
        vm.prank(OWNER);
        PauserSet(pauserSetAdd).addPauser(PAUSER);

        acl = ACL(aclAdd);
        handler = new ACLDelegationHandler(acl, OWNER, PAUSER, ACTOR_1, ACTOR_2, ACTOR_3);

        address[] memory senders = new address[](5);
        senders[0] = OWNER;
        senders[1] = PAUSER;
        senders[2] = ACTOR_1;
        senders[3] = ACTOR_2;
        senders[4] = ACTOR_3;

        bytes4[] memory selectors = new bytes4[](5);
        selectors[0] = ACLDelegationHandler.delegateForUserDecryption.selector;
        selectors[1] = ACLDelegationHandler.revokeTrackedDelegationAsDelegator.selector;
        selectors[2] = ACLDelegationHandler.revokeTrackedDelegationAsCaller.selector;
        selectors[3] = ACLDelegationHandler.pause.selector;
        selectors[4] = ACLDelegationHandler.unpause.selector;
        _targetInvariant(address(handler), selectors, senders);
    }

    /// @custom:invariant ACL-DEL-001
    function invariant_RevokedDelegationHasZeroExpiration() public view {
        uint256 count = handler.trackedDelegationCount();
        for (uint256 i = 0; i < count; i++) {
            (address delegator, address delegate, address contractAddress, bytes32 handle, uint8 status) = handler
                .getTrackedDelegation(i);
            if (status != STATUS_REVOKED) {
                continue;
            }
            uint64 expiration = acl.getUserDecryptionDelegationExpirationDate(delegator, delegate, contractAddress);
            assertEq(expiration, 0);
            assertFalse(acl.isHandleDelegatedForUserDecryption(delegator, delegate, contractAddress, handle));
        }
    }

    /// @custom:invariant ACL-DEL-002
    function invariant_ActiveDelegationFromModelHasAccess() public view {
        uint256 count = handler.trackedDelegationCount();
        for (uint256 i = 0; i < count; i++) {
            (address delegator, address delegate, address contractAddress, bytes32 handle, uint8 status) = handler
                .getTrackedDelegation(i);
            if (status != STATUS_ACTIVE) {
                continue;
            }
            assertTrue(acl.isHandleDelegatedForUserDecryption(delegator, delegate, contractAddress, handle));
        }
    }

    /// @custom:invariant ACL-DEL-003
    function invariant_PausedDelegationMutationsCannotSucceed() public view {
        assertFalse(handler.pausedBypassViolation());
    }

    /// @custom:invariant ACL-DEL-004
    function invariant_OnlyDelegatorCanRevokeDelegation() public view {
        assertFalse(handler.privilegeViolation());
    }
}
