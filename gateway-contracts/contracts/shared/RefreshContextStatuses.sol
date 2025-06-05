// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { kmsContextsAddress } from "../../addresses/KmsContextsAddress.sol";
import { IKmsContexts } from "../interfaces/IKmsContexts.sol";

/**
 * @title Refresh Statuses of Contexts
 * @dev Base contract that provides modifiers that refresh context statuses
 */
abstract contract RefreshContextStatuses {
    /// @notice The address of the KmsContexts contract
    IKmsContexts private constant KMS_CONTEXT = IKmsContexts(kmsContextsAddress);

    /// @notice Refresh the KMS context statuses.
    modifier refreshKmsContextStatuses() {
        KMS_CONTEXT.refreshKmsContextStatuses();
        _;
    }
}
