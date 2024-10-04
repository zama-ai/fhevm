// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHEExecutor.sol";

// Contract that extends TFHEExecutor with version information
contract TFHEExecutorUpgradedExample is TFHEExecutor {
    // Name of the contract
    string private constant CONTRACT_NAME = "TFHEExecutor";

    // Version numbers
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 2;
    uint256 private constant PATCH_VERSION = 0;

    // Function to get the full version string of the contract
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
