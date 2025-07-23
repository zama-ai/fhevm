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
abstract contract Pausable is Ownable2StepUpgradeable, PausableUpgradeable {
    /// @notice The address of the GatewayConfig contract
    IGatewayConfig private constant _GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);

    /**
     * @notice Error emitted when an address is not the pauser.
     * @param notPauser The address that is not the pauser.
     */
    error NotPauser(address notPauser);

    /**
     * @dev Triggers stopped state.
     *
     * Requirements:
     *
     * - Only pauser addresses can pause.
     * - The contract must not be paused.
     */
    function pause() external virtual {
        if (msg.sender != _GATEWAY_CONFIG.getPauser()) {
            revert NotPauser(msg.sender);
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
    function unpause() external virtual onlyOwner {
        _unpause();
    }
}
