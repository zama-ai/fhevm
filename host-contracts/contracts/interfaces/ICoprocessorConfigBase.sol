// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Coprocessor} from "../shared/Structs.sol";

/**
 * @title Interface for the shared CoprocessorConfig reads, events and errors.
 * @notice Declares what CoprocessorConfig and CoprocessorConfigReplica share.
 */
interface ICoprocessorConfigBase {
    // -----------------------------------------------------------------------------------------
    // Events
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Emitted when a coprocessor context is destroyed. Its set and threshold stay readable.
     * @param contextId The destroyed coprocessor context ID.
     */
    event CoprocessorContextDestroyed(uint256 indexed contextId);

    // -----------------------------------------------------------------------------------------
    // Errors
    // -----------------------------------------------------------------------------------------

    /// @notice The coprocessors array is empty.
    error EmptyCoprocessors();

    /// @notice A coprocessor has a null tx sender address.
    error CoprocessorNullTxSender();

    /// @notice A coprocessor has a null signer address.
    error CoprocessorNullSigner();

    /// @notice A coprocessor has an empty storage URL.
    error CoprocessorEmptyStorageUrl();

    /// @notice A coprocessor tx sender address is already registered in this context.
    /// @param txSender The duplicate tx sender address.
    error CoprocessorTxSenderAlreadyRegistered(address txSender);

    /// @notice A coprocessor signer address is already registered in this context.
    /// @param signer The duplicate signer address.
    error CoprocessorSignerAlreadyRegistered(address signer);

    /// @notice The coprocessor set is larger than an input proof can carry (`uint8` signature count).
    /// @param coprocessorCount The number of coprocessors in the rejected set.
    /// @param maxAllowed The maximum size an input proof can carry.
    error CoprocessorSetExceedsProofFormatLimit(uint256 coprocessorCount, uint256 maxAllowed);

    /// @notice The majority threshold is not more than half the number of coprocessors, or exceeds it.
    /// @param majorityThreshold The rejected threshold.
    /// @param coprocessorCount The number of coprocessors.
    error InvalidCoprocessorMajorityThreshold(uint256 majorityThreshold, uint256 coprocessorCount);

    /// @notice The coprocessor context ID was never defined, or is destroyed when a valid context is required.
    /// @param coprocessorContextId The invalid coprocessor context ID.
    error InvalidCoprocessorContext(uint256 coprocessorContextId);

    /// @notice The current coprocessor context cannot be destroyed.
    /// @param coprocessorContextId The current coprocessor context ID.
    error CurrentCoprocessorContextCannotBeDestroyed(uint256 coprocessorContextId);

    // -----------------------------------------------------------------------------------------
    // Read functions
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Returns the current coprocessor context ID.
     * @dev The previous context is the current ID minus one.
     * @return The current coprocessor context ID.
     */
    function getCurrentCoprocessorContextId() external view returns (uint256);

    /**
     * @notice Checks whether a coprocessor context is defined and not destroyed.
     * @param coprocessorContextId The coprocessor context ID to check.
     * @return True if the context is valid.
     */
    function isValidCoprocessorContext(uint256 coprocessorContextId) external view returns (bool);

    /**
     * @notice Returns the coprocessors of a context, in definition order.
     * @dev Still returns the set of a destroyed context. Reverts with `InvalidCoprocessorContext`
     *      for an ID that was never defined.
     * @param coprocessorContextId The coprocessor context ID.
     * @return The coprocessors of the context.
     */
    function getCoprocessorsForContext(uint256 coprocessorContextId) external view returns (Coprocessor[] memory);

    /**
     * @notice Returns the majority threshold of a coprocessor context.
     * @dev Still returns the threshold of a destroyed context. Reverts with `InvalidCoprocessorContext`
     *      for an ID that was never defined.
     * @param coprocessorContextId The coprocessor context ID.
     * @return The number of matching coprocessor outputs required for consensus.
     */
    function getCoprocessorMajorityThresholdForContext(uint256 coprocessorContextId) external view returns (uint256);

    /**
     * @notice Checks whether an address is a coprocessor signer in the given context.
     * @dev Does not check that the context is valid: callers use `isValidCoprocessorContext` for that.
     * @param coprocessorContextId The coprocessor context ID.
     * @param signer The address to check.
     * @return True if the address is a signer of the context.
     */
    function isCoprocessorSignerForContext(uint256 coprocessorContextId, address signer) external view returns (bool);

    /**
     * @notice Checks whether an address is a coprocessor tx sender in the given context.
     * @dev Does not check that the context is valid: callers use `isValidCoprocessorContext` for that.
     * @param coprocessorContextId The coprocessor context ID.
     * @param txSender The address to check.
     * @return True if the address is a tx sender of the context.
     */
    function isCoprocessorTxSenderForContext(
        uint256 coprocessorContextId,
        address txSender
    ) external view returns (bool);

    /**
     * @notice Returns the contract version.
     * @return The version string.
     */
    function getVersion() external pure returns (string memory);
}
