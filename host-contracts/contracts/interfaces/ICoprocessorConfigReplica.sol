// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Coprocessor} from "../shared/Structs.sol";
import {ICoprocessorConfigBase} from "./ICoprocessorConfigBase.sol";

/**
 * @title Interface for the CoprocessorConfigReplica contract.
 * @notice Read-replica of the canonical CoprocessorConfig on non-canonical host chains.
 * @dev Contexts arrive through the owner mirror functions, with the IDs the canonical contract assigned.
 */
interface ICoprocessorConfigReplica is ICoprocessorConfigBase {
    // -----------------------------------------------------------------------------------------
    // Events
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Emitted when a canonical coprocessor context is mirrored. It becomes current immediately.
     * @param contextId The mirrored canonical coprocessor context ID.
     * @param coprocessors The coprocessors of the context.
     * @param majorityThreshold The majority threshold of the context.
     */
    event MirrorCoprocessorContext(uint256 indexed contextId, Coprocessor[] coprocessors, uint256 majorityThreshold);

    // -----------------------------------------------------------------------------------------
    // Errors
    // -----------------------------------------------------------------------------------------

    /// @notice The mirrored coprocessor context ID is not strictly greater than the current one.
    /// @param contextId The rejected coprocessor context ID.
    /// @param currentCoprocessorContextId The current coprocessor context ID on this replica.
    error NonIncreasingCoprocessorContextId(uint256 contextId, uint256 currentCoprocessorContextId);

    // -----------------------------------------------------------------------------------------
    // Mirror functions
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Mirror a canonical coprocessor context and make it current.
     * @dev Owner only. `contextId` must be strictly greater than the current coprocessor context ID.
     *      The set is checked as on the canonical contract.
     * @param contextId The canonical coprocessor context ID.
     * @param coprocessors The coprocessors of the canonical context.
     * @param majorityThreshold The majority threshold of the canonical context.
     */
    function mirrorCoprocessorContext(
        uint256 contextId,
        Coprocessor[] calldata coprocessors,
        uint256 majorityThreshold
    ) external;

    /**
     * @notice Mirror the destruction of a canonical coprocessor context.
     * @dev Owner only. Same checks as `destroyCoprocessorContext`, against this replica's state.
     * @param coprocessorContextId The coprocessor context ID to destroy.
     */
    function mirrorCoprocessorContextDestruction(uint256 coprocessorContextId) external;
}
