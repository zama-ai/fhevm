// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/access/Ownable2Step.sol";

contract GatewayConfigMock is Ownable2Step {
    uint256 public value;

    constructor(address _initialOwner) Ownable(_initialOwner) {}

    function setByOwner(uint256 _value) external onlyOwner {
        value = _value;
    }
}
