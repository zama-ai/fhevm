// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

/// @title An interface for the update manager
/// @notice The update manager is responsible for managing updates to the protocol (ex: FHE params)
/// @notice Only the governance contract can call the update functions
interface IUpdateManager {
    /// @notice Emitted when the FHE params are updated
    /// @param newFheParamsId The new FHE params ID
    event UpdateFheParams(uint256 newFheParamsId);

    /// @notice Updates the FHE params
    /// @notice This function can only be called by the governance contract
    /// @param newFheParamsId The new FHE params ID
    function updateFheParams(uint256 newFheParamsId) external;

    /// @notice Gets the current FHE params ID
    /// @return The current FHE params ID
    function getFheParamsId() external view returns (uint256);
}
