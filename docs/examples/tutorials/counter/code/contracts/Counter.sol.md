```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @title A simple counter contract
contract Counter {
    uint32 private _count;

    /// @notice Returns the current count
    function getCount() external view returns (uint32) {
        return _count;
    }

    /// @notice Increments the counter by 1
    function increment(uint32 value) external {
        _count += value;
    }

    /// @notice Decrements the counter by 1
    function decrement(uint32 value) external {
        require(_count > value, "Counter: cannot decrement below zero");
        _count -= value;
    }
}
```