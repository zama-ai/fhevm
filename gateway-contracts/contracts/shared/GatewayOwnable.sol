// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { gatewayConfigAddress } from "../../addresses/GatewayAddresses.sol";
import { IGatewayConfig } from "../interfaces/IGatewayConfig.sol";
import { HandleOps } from "../libraries/HandleOps.sol";

/**
 * @title GatewayOwnable
 * @dev Ensures that a contract is owned by the Gateway owner, defined as the owner of the
 * GatewayConfig contract
 */
abstract contract GatewayOwnable {
    /**
     * @notice Error emitted when an address is not the owner of the GatewayConfig contract.
     * @param sender The address that is not the owner.
     */
    error NotGatewayOwner(address sender);

    /**
     * @notice Checks if the sender is the owner of the GatewayConfig contract.
     */
    modifier onlyGatewayOwner() {
        /**
         * @dev We cast to Ownable2StepUpgradeable instead of importing GatewayConfig
         * to avoid a circular dependency. Solidity requires that base contracts are defined
         * before derived contracts, which GatewayConfig would violate in this context.
         */
        if (msg.sender != Ownable2StepUpgradeable(gatewayConfigAddress).owner()) {
            revert NotGatewayOwner(msg.sender);
        }
        _;
    }
}
