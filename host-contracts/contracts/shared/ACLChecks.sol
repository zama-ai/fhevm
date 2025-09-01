// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import {aclAdd} from "../../addresses/FHEVMHostAddresses.sol";

/**
 * @title ACL Checks
 * @dev Base contract that provides modifiers that checks proper registration in the ACL contract
 */
abstract contract ACLChecks {
    /**
     * @notice Error emitted when an address is not the owner of the ACL contract, i.e not Host contracts owner.
     * @param sender The address that is not the owner.
     */
    error NotHostOwner(address sender);

    /// @dev Check that the sender is the owner of the ACL contract.
    modifier onlyACLOwner() {
        /**
         * @dev We cast to Ownable2StepUpgradeable instead of importing ACL
         * to avoid a circular dependency. Solidity requires that base contracts be defined
         * before derived contracts, which GatewayConfig would violate in this context.
         */
        if (msg.sender != Ownable2StepUpgradeable(aclAdd).owner()) {
            revert NotHostOwner(msg.sender);
        }
        _;
    }
}
