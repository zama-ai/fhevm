// SPDX-License-Identifier: BSD-3-Clause
pragma solidity ^0.8.24;

/**
 * @title ICompoundTimelock.
 * @dev   The CompoundTimelock inherits this interface.
 */
interface ICompoundTimelock {
    /// @notice Returned if the delay is below the minimum delay.
    error DelayBelowMinimumDelay();

    /// @notice Returned if the delay exceeds the maximum delay.
    error DelayAboveMaximumDelay();

    /// @notice Returned if the transaction's execution reverted.
    error ExecutionReverted();

    /// @notice Returned if the `msg.sender` is not the admin.
    error SenderIsNotAdmin();

    /// @notice Returned if the `msg.sender` is not this contract (`CompoundTimelock`).
    error SenderIsNotTimelock();

    /// @notice Returned if the `msg.sender` is not `pendingAdmin`.
    error SenderIsNotPendingAdmin();

    /// @notice Returned if the transaction has not been queued.
    error TransactionNotQueued();

    /// @notice Returned if the transaction has not surpassed the time lock.
    error TransactionTooEarlyForExecution();

    /// @notice Returned if the estimated execution block does not satisfay the delay.
    error TransactionTooEarlyForQueuing();

    /// @notice Returned if the transaction is stale (too late for execution).
    error TransactionTooLateForExecution();

    /// @notice Emitted when there is a change of admin.
    event NewAdmin(address indexed newAdmin);

    /// @notice Emtited when there is a change of pending admin.
    event NewPendingAdmin(address indexed newPendingAdmin);

    /// @notice Emitted when there is a new delay set.
    event NewDelay(uint256 indexed newDelay);

    /// @notice Emitted when the queued transaction is canceled.
    event CancelTransaction(
        bytes32 indexed txHash, address indexed target, uint256 value, string signature, bytes data, uint256 eta
    );

    /// @notice Emitted when the queued transaction is executed.
    event ExecuteTransaction(
        bytes32 indexed txHash, address indexed target, uint256 value, string signature, bytes data, uint256 eta
    );

    /// @notice Emitted when a transaction is queued.
    event QueueTransaction(
        bytes32 indexed txHash, address indexed target, uint256 value, string signature, bytes data, uint256 eta
    );

    /**
     * @notice Returns the delay (in timestamp) for a queued transaction before it can be executed.
     */
    function delay() external view returns (uint256);

    /**
     * @notice Returns the grace period (in timestamp).
     *         The grace period indicates how long a transaction can remain queued before it cannot be
     *         executed again.
     */
    /* solhint-disable func-name-mixedcase*/
    function GRACE_PERIOD() external view returns (uint256);

    /**
     * @notice Accept admin role.
     */
    function acceptAdmin() external;

    /**
     * @notice Returns whether the transactions are queued.
     */
    function queuedTransactions(bytes32 hash) external view returns (bool);

    /**
     * @notice                  Queue a transaction.
     * @param target            Target address to execute the transaction.
     * @param signature         Function signature to execute.
     * @param data              The data to include in the transaction.
     * @param eta               The earliest eta to queue the transaction.
     * @return hashTransaction  The transaction's hash.
     */
    function queueTransaction(
        address target,
        uint256 value,
        string calldata signature,
        bytes calldata data,
        uint256 eta
    ) external returns (bytes32 hashTransaction);

    /**
     * @notice              Cancel a queued transaction.
     * @param target        Target address to execute the transaction.
     * @param signature     Function signature to execute.
     * @param data          The data to include in the transaction.
     * @param eta           The earliest eta to queue the transaction.
     */
    function cancelTransaction(
        address target,
        uint256 value,
        string calldata signature,
        bytes calldata data,
        uint256 eta
    ) external;

    /**
     * @notice              Cancel a queued transaction.
     * @param target        Target address to execute the transaction.
     * @param signature     Function signature to execute.
     * @param data          The data to include in the transaction.
     * @param eta           The earliest eta to queue the transaction.
     * @return response     The response from the transaction once executed.
     */
    function executeTransaction(
        address target,
        uint256 value,
        string calldata signature,
        bytes calldata data,
        uint256 eta
    ) external payable returns (bytes memory response);
}
