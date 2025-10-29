// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

/// @dev Minimal stub for LayerZero endpoint used in task tests.
contract EndpointStub {
    mapping(address => address) public delegates;

    event DelegateSet(address indexed caller, address indexed delegate);

    function setDelegate(address _delegate) external {
        delegates[msg.sender] = _delegate;
        emit DelegateSet(msg.sender, _delegate);
    }
}
