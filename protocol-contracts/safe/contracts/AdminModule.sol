// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import "@safe-global/safe-contracts/contracts/common/Enum.sol";
import "@safe-global/safe-contracts/contracts/SafeL2.sol";

contract AdminModule {
    address public immutable ADMIN_ACCOUNT;
    address payable public immutable SAFE_PROXY;

    error CouldNotExecuteSafeTx(
        uint256 index,
        address target,
        uint256 value,
        bytes data,
        Enum.Operation operation,
        bytes errorData
    );
    error TargetsIsEmpty();
    error TargetsNotSameLengthAsDatas();
    error TargetsNotSameLengthAsOperations();
    error TargetsNotSameLengthAsValues();
    error SenderIsNotAdmin();

    constructor(address adminAccount, address payable safeProxy) {
        ADMIN_ACCOUNT = adminAccount;
        SAFE_PROXY = safeProxy;
    }

    function executeSafeTransactions(
        address[] calldata targets,
        uint256[] calldata values,
        bytes[] calldata datas,
        Enum.Operation[] calldata operations
    ) external {
        if (msg.sender != ADMIN_ACCOUNT) revert SenderIsNotAdmin();
        uint256 targetLen = targets.length;

        {
            // local scope to avoid stack too deep error
            uint256 valueLen = values.length;
            uint256 dataLen = datas.length;
            uint256 operationLen = operations.length;

            if (targetLen == 0) revert TargetsIsEmpty();
            if (targetLen != valueLen) revert TargetsNotSameLengthAsValues();
            if (targetLen != dataLen) revert TargetsNotSameLengthAsDatas();
            if (targetLen != operationLen) revert TargetsNotSameLengthAsOperations();
        }

        for (uint256 idx = 0; idx < targetLen; idx++) {
            (bool success, bytes memory returnData) = SafeL2(SAFE_PROXY).execTransactionFromModuleReturnData(
                targets[idx],
                values[idx],
                datas[idx],
                operations[idx]
            );
            if (!success)
                revert CouldNotExecuteSafeTx(idx, targets[idx], values[idx], datas[idx], operations[idx], returnData);
        }
    }
}
