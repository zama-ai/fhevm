// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {IWrapperReceiver} from "../interfaces/IWrapperReceiver.sol";

/// @notice Mock contract for testing IWrapperReceiver callback functionality
/// @dev Allows configuration of return value for testing both success and failure cases
contract WrapperReceiverMock is IWrapperReceiver {
    bool public returnValue = true;

    // Track callback invocations for testing
    uint256 public callbackCount;
    address public lastOperator;
    uint256 public lastAmount;
    uint256 public lastUnwrapRequestId;
    address public lastRefundTo;
    bytes public lastData;

    /// @notice Set the return value for subsequent callback calls
    function setReturnValue(bool _returnValue) external {
        returnValue = _returnValue;
    }

    /// @notice Implementation of IWrapperReceiver callback
    function onUnwrapFinalizedReceived(
        address operator,
        uint256 amount,
        uint256 unwrapRequestId,
        address refundTo,
        bytes calldata data
    ) external override returns (bool) {
        callbackCount++;
        lastOperator = operator;
        lastAmount = amount;
        lastUnwrapRequestId = unwrapRequestId;
        lastRefundTo = refundTo;
        lastData = data;

        return returnValue;
    }

    /// @notice Reset tracking state for fresh test
    function reset() external {
        callbackCount = 0;
        lastOperator = address(0);
        lastAmount = 0;
        lastUnwrapRequestId = 0;
        lastRefundTo = address(0);
        lastData = "";
        returnValue = true;
    }
}
