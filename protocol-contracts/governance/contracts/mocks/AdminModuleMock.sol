// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "@safe-global/safe-contracts/contracts/common/Enum.sol";
import "@safe-global/safe-contracts/contracts/SafeL2.sol";

contract AdminModuleMock {
    address public immutable ADMIN_ACCOUNT;
    address payable public immutable SAFE_PROXY;

    error SenderIsNotAdmin();

    error CouldNotExecuteSafeTx(
        uint256 index,
        address target,
        uint256 value,
        bytes data,
        Enum.Operation operation,
        bytes errorData
    );

    error TargetsIsEmpty();
    error TargetsNotSameLengthAsDataPayloads();
    error TargetsNotSameLengthAsOperations();
    error TargetsNotSameLengthAsValues();

    constructor(address adminAccount, address payable safeProxy) {
        ADMIN_ACCOUNT = adminAccount;
        SAFE_PROXY = safeProxy;
    }

    function executeSafeTransactions(
        address[] calldata targets,
        uint256[] calldata values,
        bytes[] calldata dataPayloads,
        Enum.Operation[] calldata operations
    ) external {
        if (msg.sender != ADMIN_ACCOUNT) revert SenderIsNotAdmin();

        uint256 targetLen = targets.length;

        {
            uint256 valueLen = values.length;
            uint256 dataLen = dataPayloads.length;
            uint256 operationLen = operations.length;

            if (targetLen == 0) revert TargetsIsEmpty();
            if (targetLen != valueLen) revert TargetsNotSameLengthAsValues();
            if (targetLen != dataLen) revert TargetsNotSameLengthAsDataPayloads();
            if (targetLen != operationLen) revert TargetsNotSameLengthAsOperations();
        }

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
