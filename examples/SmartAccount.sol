// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// Import Ownable2Step from OpenZeppelin
import "@openzeppelin/contracts/access/Ownable2Step.sol";

// Smart account contract with batch transaction execution
contract SmartAccount is Ownable2Step {
    // Structure to represent a transaction
    struct Transaction {
        address target;
        uint256 value;
        bytes data;
    }

    // Constructor to set the initial owner
    constructor() Ownable(msg.sender) {}

    // Function to execute multiple transactions in a batch
    function executeBatch(Transaction[] memory transactions) public payable onlyOwner {
        for (uint i = 0; i < transactions.length; i++) {
            Transaction memory transaction = transactions[i];
            // Execute the transaction
            (bool success, ) = transaction.target.call{value: transaction.value}(transaction.data);
            // Ensure the transaction was successful
            require(success, "Transaction failed");
        }
    }
}
