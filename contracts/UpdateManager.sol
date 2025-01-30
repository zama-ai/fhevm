// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

import "./interfaces/IUpdateManager.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

/// @title Implementation of the update manager
/// @dev see {IUpdateManager} for more details
contract UpdateManager is IUpdateManager, AccessControl {
    /// @dev The FHE parameters's identifier currently in use
    uint256 private _fheParamsId;

    constructor(address governance, uint256 fheParamsId) {
        _grantRole(DEFAULT_ADMIN_ROLE, governance);

        /// @dev Set the new FHE parameters's identifier to use
        _fheParamsId = fheParamsId;
    }

    /// @dev See {IUpdateManager-updateFheParams}.
    function updateFheParams(uint256 newFheParamsId) external onlyRole(DEFAULT_ADMIN_ROLE) {
        _fheParamsId = newFheParamsId;

        emit UpdateFheParams(newFheParamsId);
    }

    /// @dev See {IUpdateManager-getFheParamsId}.
    function getFheParamsId() external view returns (uint256) {
        return _fheParamsId;
    }
}
