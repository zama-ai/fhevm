// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity 0.8.27;

import {AdminProvider} from "../admin/AdminProvider.sol";

/// @title IDeploymentCoordinator
/// @notice Interface for accessing DeploymentCoordinator configuration
/// @dev Used by WrapperUpgradeable and RegulatedERC7984Upgradeable to dynamically retrieve AdminProvider
interface IDeploymentCoordinator {
    /// @notice Returns the current AdminProvider address
    /// @return The AdminProvider contract instance
    function adminProvider() external view returns (AdminProvider);
}
