// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { gatewayConfigAddress } from "../../addresses/GatewayAddresses.sol";
import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { PausableUpgradeable } from "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import "../interfaces/IGatewayConfig.sol";

/**
 * @title Pausable.
 * @dev This contract provides an abstract implementation for the pausing features
 * based on the OpenZeppelin PausableUpgradeable contract.
 */
abstract contract Pausable is PausableUpgradeable {
    /// @notice The address of the GatewayConfig contract
    IGatewayConfig private constant _GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);

    /**
     * @notice Error emitted when an address is not the pauser.
     * @param notPauser The address that is not the pauser.
     */
    error NotPauser(address notPauser);

    /**
     * @notice Error emitted when an address is not the pauser or the gateway config.
     * @param notPauserOrGatewayConfig The address that is not the pauser or the gateway config.
     */
    error NotPauserOrGatewayConfig(address notPauserOrGatewayConfig);

    /**
     * @notice Error emitted when an address is not the owner or the gateway config.
     * @param notOwnerOrGatewayConfig The address that is not the owner or the gateway config.
     */
    error NotOwnerOrGatewayConfig(address notOwnerOrGatewayConfig);

    modifier onlyPauser() {
        if (msg.sender != _GATEWAY_CONFIG.getPauser()) {
            revert NotPauser(msg.sender);
        }
        _;
    }

    /**
     * @dev Triggers stopped state.
     *
     * Requirements:
     *
     * - Only pauser addresses can pause.
     * - The contract must not be paused.
     */
    function pause() external virtual {
        if (msg.sender != _GATEWAY_CONFIG.getPauser() && msg.sender != gatewayConfigAddress) {
            revert NotPauserOrGatewayConfig(msg.sender);
        }
        _pause();
    }

    /**
     * @dev Returns to normal state.
     *
     * Requirements:
     *
     * - Only owner can unpause.
     * - The contract must be paused.
     */
    function unpause() external virtual {
        /**
         * @dev We cast to Ownable2StepUpgradeable instead of importing GatewayConfig
         * to avoid a circular dependency. Solidity requires that base contracts be defined
         * before derived contracts, which GatewayConfig would violate in this context.
         */
        if (msg.sender != Ownable2StepUpgradeable(gatewayConfigAddress).owner() && msg.sender != gatewayConfigAddress) {
            revert NotOwnerOrGatewayConfig(msg.sender);
        }
        _unpause();
    }
}
