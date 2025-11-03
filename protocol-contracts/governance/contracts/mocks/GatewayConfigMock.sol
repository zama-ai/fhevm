// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/access/Ownable2Step.sol";

contract GatewayConfigMock is Ownable2Step {
    uint256 public value;

    constructor(address _initialOwner) Ownable(_initialOwner) {}

    function setValue(uint256 _value) external onlyOwner {
        value = _value;
    }

    function expensiveUpdate(uint256 _value) external onlyOwner {
        uint256 expensiveVar;
        for (uint256 k = 0; k < 10000; k++) {
            expensiveVar += 1;
        }
        value = _value;
    }

    function payableUpdate(uint256 _value) external payable onlyOwner {
        require(msg.value == 1 ether, "Incorrect value sent");
        value = _value;
    }
}
