// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// dummy contract for dummy transactions just to advance blocks
contract Counter {
    uint32 value;

    function increment() public {
        value += 1;
    }

    function currentValue() public view returns (uint32) {
        return value;
    }
}
