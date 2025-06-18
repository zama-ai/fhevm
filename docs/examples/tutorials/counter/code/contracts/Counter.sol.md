```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @title A simple counter contract
contract Counter {
    uint32 private count;

    /// @notice Returns the current count
    function getCount() external view returns (uint32) {
        return count;
    }

    /// @notice Increments the counter by 1
    function increment() external {
        count += 1;
    }

    /// @notice Decrements the counter by 1
    function decrement() external {
        require(count > 0, "Counter: cannot decrement below zero");
        count -= 1;
    }
}
```