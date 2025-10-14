// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../CoprocessorContexts.sol";

contract CoprocessorContextsV2Example is CoprocessorContexts {
    /// @notice Name of the contract
    string private constant CONTRACT_NAME = "CoprocessorContexts";

    /// @notice Version of the contract
    uint256 private constant MAJOR_VERSION = 1000;
    uint256 private constant MINOR_VERSION = 0;
    uint256 private constant PATCH_VERSION = 0;

    /// @notice Getter for the name and version of the contract
    /// @return string representing the name and the version of the contract
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
