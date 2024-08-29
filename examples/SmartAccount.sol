// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/access/Ownable2Step.sol";

contract SmartAccount is Ownable2Step {
    struct Transaction {
        address target;
        uint256 value;
        bytes data;
    }

    constructor() Ownable(msg.sender) {}

    function executeBatch(Transaction[] memory transactions) public payable onlyOwner {
        for (uint i = 0; i < transactions.length; i++) {
            Transaction memory transaction = transactions[i];
            (bool success, ) = transaction.target.call{value: transaction.value}(transaction.data);
            require(success, "Transaction failed");
        }
    }
}
