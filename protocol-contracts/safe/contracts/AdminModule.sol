// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "@safe-global/safe-contracts/contracts/common/Enum.sol";
import "@safe-global/safe-contracts/contracts/SafeL2.sol";

/**
 * @title AdminModule
 * @notice This contract is used to execute transactions on behalf of the Safe Smart Account.
 * The Safe's owner will need to enable this module in it.
 */
contract AdminModule {
    /**
     * @notice The address of the account that will be allowed to execute transactions.
     */
    address public immutable ADMIN_ACCOUNT;
    /**
     * @notice The address of the Safe Smart Account proxy that will be used to execute transactions.
     */
    address payable public immutable SAFE_PROXY;

    /**
     * @notice Error emitted when the sender is not the admin account.
     */
    error SenderIsNotAdmin();

    /**
     * @notice Error emitted when the transaction execution fails in the Safe Smart Account.
     * @param index The index of the transaction that failed.
     * @param target The target address of the transaction that failed.
     * @param value The value of the transaction that failed.
     * @param data The data payload of the transaction that failed.
     * @param operation The operation type of the transaction that failed.
     * @param errorData The error data returned by the transaction that failed.
     */
    error CouldNotExecuteSafeTx(
        uint256 index,
        address target,
        uint256 value,
        bytes data,
        Enum.Operation operation,
        bytes errorData
    );
    /**
     * @notice Error emitted when the targets array is empty.
     */
    error TargetsIsEmpty();
    /**
     * @notice Error emitted when the targets array is not the same length as the data payloads array.
     */
    error TargetsNotSameLengthAsDataPayloads();
    /**
     * @notice Error emitted when the targets array is not the same length as the operations array.
     */
    error TargetsNotSameLengthAsOperations();
    /**
     * @notice Error emitted when the targets array is not the same length as the values array.
     */
    error TargetsNotSameLengthAsValues();

    constructor(address adminAccount, address payable safeProxy) {
        ADMIN_ACCOUNT = adminAccount;
        SAFE_PROXY = safeProxy;
    }

    /**
     * @notice Executes multiple transactions on behalf of the Safe Smart Account (restricted to the
     * admin account).
     * @param targets Destination addresses of the transactions.
     * @param values Ether values of the transactions.
     * @param dataPayloads Data payloads of the transactions.
     * @param operations Operation types of the transactions (0 for call, 1 for delegate call).
     */
    function executeSafeTransactions(
        address[] calldata targets,
        uint256[] calldata values,
        bytes[] calldata dataPayloads,
        Enum.Operation[] calldata operations
    ) external {
        // Only the admin account can execute transactions.
        if (msg.sender != ADMIN_ACCOUNT) revert SenderIsNotAdmin();

        uint256 targetLen = targets.length;

        // local scope to avoid stack too deep error
        {
            uint256 valueLen = values.length;
            uint256 dataLen = dataPayloads.length;
            uint256 operationLen = operations.length;

            // Make sure the arrays are not empty and of the same length.
            if (targetLen == 0) revert TargetsIsEmpty();
            if (targetLen != valueLen) revert TargetsNotSameLengthAsValues();
            if (targetLen != dataLen) revert TargetsNotSameLengthAsDataPayloads();
            if (targetLen != operationLen) revert TargetsNotSameLengthAsOperations();
        }

        // For each target, execute the transaction in the Safe Smart Account. Make sure each one
        // of them is successful, else revert the transaction.
        for (uint256 idx = 0; idx < targetLen; idx++) {
            (bool success, bytes memory returnData) = SafeL2(SAFE_PROXY).execTransactionFromModuleReturnData(
                targets[idx],
                values[idx],
                dataPayloads[idx],
                operations[idx]
            );
            if (!success)
                revert CouldNotExecuteSafeTx(
                    idx,
                    targets[idx],
                    values[idx],
                    dataPayloads[idx],
                    operations[idx],
                    returnData
                );
        }
    }
}
