// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity 0.8.24;

/// @notice Fixture contract of the test engine's Chain module tests. It has no fhEVM dependency.
contract Counter {
    uint256 public count;

    event Incremented(address indexed by, uint256 newCount);

    /// @dev Reverts when `initialCount` is 1000 or more, to exercise a reverted deployment.
    constructor(uint256 initialCount) {
        require(initialCount < 1000, "Counter: initial count too large");
        count = initialCount;
    }

    function increment() external {
        count += 1;
        emit Incremented(msg.sender, count);
    }

    function incrementBy(uint256 amount) external {
        count += amount;
        emit Incremented(msg.sender, count);
    }

    /// @dev Always reverts, to exercise a reverted transaction. Not `view`, so that calling it sends a
    /// transaction instead of an `eth_call`.
    function fail() external {
        count += 1;
        revert("Counter: always fails");
    }
}
