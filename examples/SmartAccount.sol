// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/access/Ownable2Step.sol";

/// @notice Smart account contract with batch transaction execution
contract SmartAccount is Ownable2Step {
    /// @dev Structure to represent a transaction
    struct Transaction {
        address target;
        uint256 value;
        bytes data;
    }

    /// @notice Constructor to set the initial owner
    constructor() Ownable(msg.sender) {}

    /// @notice Function to execute multiple transactions in a batch
    /// @param transactions Array of transactions to execute
    function executeBatch(Transaction[] memory transactions) public payable onlyOwner {
        for (uint i = 0; i < transactions.length; i++) {
            Transaction memory transaction = transactions[i];
            /// @dev Execute the transaction
            (bool success, ) = transaction.target.call{value: transaction.value}(transaction.data);
            /// @dev Ensure the transaction was successful
            require(success, "Transaction failed");
        }
    }
}
