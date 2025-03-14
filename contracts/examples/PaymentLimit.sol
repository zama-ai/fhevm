// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";
import "../lib/FHEVMConfig.sol";

/// @title PaymentLimit
/// @notice A contract to demonstrate FHE gas limits in different scenarios
contract PaymentLimit {
    /// @notice Constructor that sets up FHE configuration and deposits initial value
    /// @dev Payable to allow initial deposit
    constructor() {
        TFHE.setFHEVM(FHEVMConfig.defaultConfig());
    }

    /// @notice Performs a small number of FHE operations
    /// @dev Should pass if it's the only transaction in a block
    function wayunderBlockFHEGasLimit() external {
        euint64 x = TFHE.asEuint64(2);
        euint64 result;
        for (uint256 i; i < 3; i++) {
            result = TFHE.mul(result, x);
        }
    }

    /// @notice Performs a moderate number of FHE operations
    /// @dev Should pass if it's the only transaction in a block
    function underBlockFHEGasLimit() external {
        euint64 x = TFHE.asEuint64(2);
        euint64 result;
        for (uint256 i; i < 15; i++) {
            result = TFHE.mul(result, x);
        }
    }

    /// @notice Performs a large number of FHE operations
    /// @dev Should revert due to exceeding the block FHE gas limit
    function aboveBlockFHEGasLimit() external {
        euint64 x = TFHE.asEuint64(2);
        euint64 result;
        for (uint256 i; i < 16; i++) {
            result = TFHE.mul(result, x);
        }
    }
}
