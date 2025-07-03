// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { kmsContextsAddress } from "../../addresses/KmsContextsAddress.sol";
import { IKmsContexts } from "../interfaces/IKmsContexts.sol";

/**
 * @title Context Checks
 * @dev Base contract that provides modifiers associated with context checks
 */
abstract contract ContextChecks {
    /// @notice The address of the KmsContexts contract
    IKmsContexts private constant KMS_CONTEXT = IKmsContexts(kmsContextsAddress);

    /// @notice Refresh the KMS context statuses.
    modifier refreshKmsContextStatuses() {
        KMS_CONTEXT.refreshKmsContextStatuses();
        _;
    }
}
