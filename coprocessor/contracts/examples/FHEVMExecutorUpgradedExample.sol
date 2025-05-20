// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {FHEVMExecutor} from "../contracts/FHEVMExecutor.sol";

/// @title FHEVMExecutorUpgradedExample
/// @dev Contract that extends FHEVMExecutor with version information
contract FHEVMExecutorUpgradedExample is FHEVMExecutor {
    /// @dev Name of the contract
    string private constant CONTRACT_NAME = "FHEVMExecutor";

    /// @dev Version numbers
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 2;
    uint256 private constant PATCH_VERSION = 0;

    /// @notice Returns the full version string of the contract
    /// @dev Concatenates the contract name and version numbers
    /// @return A string representing the full version of the contract
    function getVersion() external pure virtual override returns (string memory) {
        return
            string(
                abi.encodePacked(
                    CONTRACT_NAME,
                    " v",
                    Strings.toString(MAJOR_VERSION),
                    ".",
                    Strings.toString(MINOR_VERSION),
                    ".",
                    Strings.toString(PATCH_VERSION)
                )
            );
    }
}
