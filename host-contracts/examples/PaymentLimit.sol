// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/FHE.sol";
import "../lib/FHEVMConfig.sol";

/// @title HCULimit
/// @notice A contract to demonstrate FHE gas limits in different scenarios
contract HCULimit {
    /// @notice Constructor that sets up FHE configuration and deposits initial value
    /// @dev Payable to allow initial deposit
    constructor() {
        FHE.setCoprocessor(FHEVMConfig.defaultConfig());
    }

    /// @notice Performs a small number of FHE operations
    /// @dev Should pass if it's the only transaction in a block
    function wayunderBlockFHEGasLimit() external {
        euint64 x = FHE.asEuint64(2);
        euint64 result;
        for (uint256 i; i < 3; i++) {
            result = FHE.mul(result, x);
        }
    }

    /// @notice Performs a moderate number of FHE operations
    /// @dev Should pass if it's the only transaction in a block
    function underBlockFHEGasLimit() external {
        euint64 x = FHE.asEuint64(2);
        euint64 result;
        for (uint256 i; i < 15; i++) {
            result = FHE.mul(result, x);
        }
    }

    /// @notice Performs a large number of FHE operations
    /// @dev Should revert due to exceeding the depth for HCU.
    function aboveBlockFHEGasLimitWithSequentialOperations() external {
        euint64 x = FHE.asEuint64(2);
        euint64 result;
        for (uint256 i; i < 16; i++) {
            result = FHE.mul(result, x);
        }
    }

    /// @notice Performs a large number of FHE operations with non-sequential transactions.
    /// @dev Should revert due to exceeding the HCU for the transaction.
    function aboveBlockFHEGasLimitWithNonSequentialOperations() external {
        euint64 result;
        for (uint256 i; i < 10000; i++) {
            result = FHE.randEuint64();
        }
    }
}
