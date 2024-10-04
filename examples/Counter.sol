// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// This is a simple contract named 'Counter' that maintains a single state variable 'value'.
// It provides functionality to increment the 'value' and read its current value.
contract Counter {
    // State variable to keep track of the count.
    uint32 value;

    // Increases the value by 1 each time this function is called.
    function increment() public {
        value += 1;
    }

    // Returns the current value of the counter.
    function currentValue() public view returns (uint32) {
        return value;
    }
}
