// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { coprocessorContextsAddress } from "../../addresses/CoprocessorContextsAddress.sol";
import { ICoprocessorContexts } from "../interfaces/ICoprocessorContexts.sol";

/**
 * @title Context Checks
 * @dev Base contract that provides modifiers associated with context checks
 */
abstract contract ContextChecks {
    /// @notice The address of the CoprocessorContexts contract
    ICoprocessorContexts private constant COPROCESSOR_CONTEXTS = ICoprocessorContexts(coprocessorContextsAddress);

    /// @notice Refresh the coprocessor context statuses.
    modifier refreshCoprocessorContextStatuses() {
        COPROCESSOR_CONTEXTS.refreshCoprocessorContextStatuses();
        _;
    }
}
