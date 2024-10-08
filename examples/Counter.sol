// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @notice A simple contract that maintains a single state variable 'value'
/// @dev This contract provides functionality to increment the 'value' and read its current value
contract Counter {
    /// @notice State variable to keep track of the count
    /// @dev Stored as a uint32 to save gas
    uint32 value;

    /// @notice Increases the value by 1 each time this function is called
    function increment() public {
        value += 1;
    }

    /// @notice Returns the current value of the counter
    /// @return The current value as a uint32
    function currentValue() public view returns (uint32) {
        return value;
    }
}
