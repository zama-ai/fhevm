// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { httpzAddress } from "../../addresses/HttpzAddress.sol";
import "../interfaces/IHTTPZ.sol";

/**
 * @title HTTPZ Checks
 * @dev Base contract that provides modifiers that checks proper registration in the HTTPZ contract
 */
abstract contract HttpzChecks {
    /// @notice The address of the HTTPZ contract
    IHTTPZ private constant _HTTPZ = IHTTPZ(httpzAddress);

    /// @notice Checks if the sender is a coprocessor transaction sender.
    modifier onlyCoprocessorTxSender() {
        _HTTPZ.checkIsCoprocessorTxSender(msg.sender);
        _;
    }

    /// @notice Checks if the sender is a KMS transaction sender.
    modifier onlyKmsTxSender() {
        _HTTPZ.checkIsKmsTxSender(msg.sender);
        _;
    }

    /// @notice Checks if the sender is the pauser.
    modifier onlyPauser() {
        _HTTPZ.checkIsPauser(msg.sender);
        _;
    }

    /// @dev Check that the network has been registered.
    modifier onlyRegisteredNetwork(uint256 chainId) {
        _HTTPZ.checkNetworkIsRegistered(chainId);
        _;
    }
}
