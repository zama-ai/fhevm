// SPDX-License-Identifier: BSD-3-Clause
pragma solidity ^0.8.24;

import {ICompoundTimelock} from "./ICompoundTimelock.sol";

/**
 * @title   CompoundTimelock.
 * @notice  This contract allows the admin to set a delay period before executing transactions.
 *          Transactions must be queued before execution. No transaction can be executed during this period,
 *          which offers time to verify the validity of pending transactions. It also has a grace period to allow
 *          for transactions not to be executed after a specific period following the queuing.
 */
contract CompoundTimelock is ICompoundTimelock {
    /**
     * @notice See {ICompoundTimelock-GRACE_PERIOD}.
     */
    uint256 public constant GRACE_PERIOD = 14 days;

    /// @notice Minimum delay that can be set in the `setDelay` function.
    uint256 public constant MINIMUM_DELAY = 2 days;

    /// @notice Maximum delay that can be set in the `setDelay` function.
    uint256 public constant MAXIMUM_DELAY = 30 days;

    /// @notice Admin address.
    address public admin;

    /// @notice Pending admin address.
    /// @dev    The transer of the admin is a two-step process.
    address public pendingAdmin;

    /**
     * @notice See {ICompoundTimelock-delay}.
     */
    uint256 public delay;

    /// @notice Return whether the transaction is queued based on its hash.
    mapping(bytes32 hashTransaction => bool isQueued) public queuedTransactions;

    /**
     * @param admin_ Admin address.
     * @param delay_ Delay (in timestamp).
     */
    constructor(address admin_, uint256 delay_) {
        if (delay_ < MINIMUM_DELAY) {
            revert DelayBelowMinimumDelay();
        }

        if (delay_ > MAXIMUM_DELAY) {
            revert DelayAboveMaximumDelay();
        }

        admin = admin_;
        delay = delay_;
    }

    receive() external payable {}

    /**
     * @notice       Set the delay.
     * @dev          This transaction must be queued.
     * @param delay_ Delay (in timestamp).
     */
    function setDelay(uint256 delay_) public {
        if (msg.sender != address(this)) {
            revert SenderIsNotTimelock();
        }

        if (delay_ < MINIMUM_DELAY) {
            revert DelayBelowMinimumDelay();
        }

        if (delay_ > MAXIMUM_DELAY) {
            revert DelayAboveMaximumDelay();
        }

        delay = delay_;

        emit NewDelay(delay);
    }

    /**
     * @notice See {ICompoundTimelock-acceptAdmin}.
     */
    function acceptAdmin() public {
        if (msg.sender != pendingAdmin) {
            revert SenderIsNotPendingAdmin();
        }

        admin = msg.sender;
        pendingAdmin = address(0);

        emit NewAdmin(admin);
    }

    /**
     * @notice              Set the pending admin.
     * @dev                 This transaction must be queued.
     * @param pendingAdmin_ Pending admin address.
     */
    function setPendingAdmin(address pendingAdmin_) public {
        if (msg.sender != address(this)) {
            revert SenderIsNotTimelock();
        }

        pendingAdmin = pendingAdmin_;

        emit NewPendingAdmin(pendingAdmin);
    }

    /**
     * @notice See {ICompoundTimelock-queueTransaction}.
     */
    function queueTransaction(address target, uint256 value, string memory signature, bytes memory data, uint256 eta)
        public
        returns (bytes32)
    {
        if (msg.sender != admin) {
            revert SenderIsNotTimelock();
        }

        if (eta < block.timestamp + delay) {
            revert TransactionTooEarlyForQueuing();
        }

        bytes32 txHash = keccak256(abi.encode(target, value, signature, data, eta));
        queuedTransactions[txHash] = true;

        emit QueueTransaction(txHash, target, value, signature, data, eta);
        return txHash;
    }

    /**
     * @notice See {ICompoundTimelock-cancelTransaction}.
     */
    function cancelTransaction(address target, uint256 value, string memory signature, bytes memory data, uint256 eta)
        public
    {
        if (msg.sender != admin) {
            revert SenderIsNotAdmin();
        }

        bytes32 txHash = keccak256(abi.encode(target, value, signature, data, eta));
        queuedTransactions[txHash] = false;

        emit CancelTransaction(txHash, target, value, signature, data, eta);
    }

    /**
     * @notice See {ICompoundTimelock-executeTransaction}.
     */
    function executeTransaction(address target, uint256 value, string memory signature, bytes memory data, uint256 eta)
        public
        payable
        returns (bytes memory)
    {
        if (msg.sender != admin) {
            revert SenderIsNotAdmin();
        }

        bytes32 txHash = keccak256(abi.encode(target, value, signature, data, eta));
        if (!queuedTransactions[txHash]) {
            revert TransactionNotQueued();
        }
        if (block.timestamp < eta) {
            revert TransactionTooEarlyForExecution();
        }

        if (block.timestamp > eta + GRACE_PERIOD) {
            revert TransactionTooLateForExecution();
        }

        queuedTransactions[txHash] = false;

        bytes memory callData;

        if (bytes(signature).length == 0) {
            callData = data;
        } else {
            callData = abi.encodePacked(bytes4(keccak256(bytes(signature))), data);
        }

        (bool success, bytes memory returnData) = target.call{value: value}(callData);

        if (!success) {
            revert ExecutionReverted();
        }

        emit ExecuteTransaction(txHash, target, value, signature, data, eta);

        return returnData;
    }
}
