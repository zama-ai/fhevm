// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { gatewayConfigAddress } from "../../addresses/GatewayConfigAddress.sol";
import { IGatewayConfig } from "../interfaces/IGatewayConfig.sol";

/**
 * @title GatewayConfig Contexts
 * @dev Base contract that provides modifiers that refresh context statuses
 */
abstract contract GatewayConfigContexts {
    /// @notice The address of the GatewayConfig contract
    IGatewayConfig private constant GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);

    /// @notice Refresh the KMS context statuses.
    modifier refreshKmsContextStatuses() {
        GATEWAY_CONFIG.refreshKmsContextStatuses();
        _;
    }
}
