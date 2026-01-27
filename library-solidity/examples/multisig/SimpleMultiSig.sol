// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @notice Simple MultiSig contract, where all owners must approve a tx before executing it
contract SimpleMultiSig {
    struct Transaction {
        address target;
        bytes data;
    }

    event ProposedTx(uint256 indexed txId, address target, bytes data);

    uint256 public txCounter;
    address[] internal owners;
    mapping(address => bool) public isOwner;
    mapping(uint256 => Transaction) public transactions;
    mapping(uint256 => mapping(address => bool)) public isApprovedByOwner;
    mapping(uint256 => bool) public executed;

    constructor(address[] memory _owners) {
        uint256 length = _owners.length;
        require(length > 1, "Multisig should have several owners");
        for (uint256 i; i < length; i++) {
            require(!isOwner[_owners[i]], "Owner has already been added");
            owners.push(_owners[i]);
            isOwner[_owners[i]] = true;
        }
    }

    function proposeTx(address target, bytes calldata data) external {
        require(isOwner[msg.sender], "Sender is not an owner");
        txCounter++;
        uint256 txId = txCounter;
        transactions[txCounter] = Transaction({target: target, data: data});
        emit ProposedTx(txId, target, data);
        approveTx(txId); // proposer automatically approves
    }

    function approveTx(uint256 txId) public {
        require(isOwner[msg.sender], "Sender is not an owner");
        require(txId != 0 && txId <= txCounter, "Invalid txId");
        require(!isApprovedByOwner[txId][msg.sender], "txId has already been approved by sender");
        isApprovedByOwner[txId][msg.sender] = true;
    }

    function executeTx(uint256 txId) external {
        require(txId != 0 && txId <= txCounter, "Invalid txId");
        require(!executed[txId], "tx has already been executed");
        for (uint i = 0; i < owners.length; i++) {
            require(isApprovedByOwner[txId][owners[i]], "txId has not been approved by all owners");
        }
        Transaction memory transaction = transactions[txId];

        (bool success, ) = (transaction.target).call(transaction.data);
        require(success, "tx reverted");
        executed[txId] = true;
    }

    function getOwners() external view returns (address[] memory) {
        return owners;
    }
}
