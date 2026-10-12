// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/FHE.sol";
import {CoprocessorSetup} from "../lib/CoprocessorSetup.sol";

/// @title HCULimitTest
/// @notice A contract to demonstrate HCU limits in different scenarios
contract HCULimitTest {
    euint64 public lineage;

    function startLineage() external {
        lineage = FHE.asEuint64(0);
        FHE.allowThis(lineage);
    }

    function advanceLineage(euint64 input) external {
        lineage = FHE.add(input, uint64(1));
        FHE.allowThis(lineage);
    }

    function forwardLineage(HCULimitTest recipient) external {
        FHE.allowTransient(lineage, address(recipient));
        recipient.advanceLineage(lineage);
    }

    function allowLineage(address account) external {
        FHE.allow(lineage, account);
    }

    function combineWithFreshBranch(euint64 input) external {
        euint64 fresh = FHE.asEuint64(0);
        fresh = FHE.add(fresh, uint64(1));
        fresh = FHE.add(fresh, uint64(1));
        lineage = FHE.add(input, fresh);
        FHE.allowThis(lineage);
    }

    /// @notice Constructor that sets up FHE configuration and deposits initial value
    /// @dev Payable to allow initial deposit
    constructor() {
        FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
    }

    /// @notice Performs a small number of FHE operations
    /// @dev Should pass.
    function wayunderTransactionHCULimit() external {
        euint64 x = FHE.asEuint64(2);
        euint64 result;
        for (uint256 i; i < 3; i++) {
            result = FHE.mul(result, x);
        }
    }

    /// @notice Performs a moderate number of FHE operations
    /// @dev Should pass.
    function underTransactionHCULimit() external {
        euint64 x = FHE.asEuint64(2);
        euint64 result;
        for (uint256 i; i < 15; i++) {
            result = FHE.mul(result, x);
        }
    }

    /// @notice Performs a large number of FHE operations
    /// @dev Should revert due to exceeding the depth for HCU.
    function aboveTransactionHCULimitWithSequentialOperations() external {
        euint64 x = FHE.asEuint64(2);
        euint64 result;
        for (uint256 i; i < 16; i++) {
            result = FHE.mul(result, x);
        }
    }

    /// @notice Performs a large number of FHE operations with non-sequential FHE operations.
    /// @dev Should revert due to exceeding the HCU for the transaction.
    function aboveTransactionHCUWithNonSequentialOperations() external {
        euint64 result;
        for (uint256 i; i < 10000; i++) {
            result = FHE.randEuint64();
        }
    }
}
