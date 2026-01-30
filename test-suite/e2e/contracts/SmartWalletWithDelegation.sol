// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "./E2ECoprocessorConfigLocal.sol";

/// @notice SmartWallet contract that supports delegated user decryption.
contract SmartWalletWithDelegation is E2ECoprocessorConfig {
    struct Transaction {
        address target;
        bytes data;
    }

    event ProposedTx(uint256 indexed txId, address target, bytes data);

    uint256 public txCounter;
    address public owner;
    mapping(uint256 => Transaction) public transactions;
    mapping(uint256 => bool) public executed;

    constructor(address _owner) {
        require(_owner != address(0), "Owner cannot be zero address");
        owner = _owner;
    }

    modifier onlyOwner() {
        require(msg.sender == owner, "Sender is not the owner");
        _;
    }

    /// @notice Propose a transaction and assume as approved (since there's only one owner).
    /// @param target The target contract address
    /// @param data The calldata to execute
    function proposeTx(address target, bytes calldata data) external onlyOwner returns (uint256) {
        txCounter++;
        uint256 txId = txCounter;
        transactions[txCounter] = Transaction({target: target, data: data});
        emit ProposedTx(txId, target, data);
        return txId;
    }

    /// @notice Execute a previously proposed transaction.
    /// @param txId The transaction ID to execute.
    function executeTx(uint256 txId) external onlyOwner {
        require(txId != 0 && txId <= txCounter, "Invalid txId");
        require(!executed[txId], "tx has already been executed");
        Transaction memory transaction = transactions[txId];

        (bool success, ) = (transaction.target).call(transaction.data);
        require(success, "tx reverted");
        executed[txId] = true;
    }

    /// @notice Delegate user decryption for a specific contract.
    /// @dev This allows an EOA to decrypt confidential data owned by this smart wallet.
    /// @param delegate The address that will be able to user decrypt.
    /// @param delegateContractAddress The contract address for which delegation applies.
    /// @param expirationTimestamp When the delegation expires.
    function delegateUserDecryption(
        address delegate,
        address delegateContractAddress,
        uint64 expirationTimestamp
    ) external onlyOwner {
        FHE.delegateUserDecryption(delegate, delegateContractAddress, expirationTimestamp);
    }

    /// @notice Revoke a previously granted delegation.
    /// @param delegate The address to revoke delegation from.
    /// @param delegateContractAddress The contract address for which to revoke delegation.
    function revokeUserDecryptionDelegation(address delegate, address delegateContractAddress) external onlyOwner {
        FHE.revokeUserDecryptionDelegation(delegate, delegateContractAddress);
    }
}
