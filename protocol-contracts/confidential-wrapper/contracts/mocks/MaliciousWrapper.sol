// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.27;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IWrapperReceiver} from "../interfaces/IWrapperReceiver.sol";
import {SwapData} from "../swap/swap_v0.sol";

/// @title MaliciousWrapper
/// @notice A malicious wrapper implementation that attempts to bypass fees
/// @dev This contract demonstrates an attack where an unauthorized contract tries to
///      call SwapV0's onUnwrapFinalizedReceived directly to bypass fees
contract MaliciousWrapper {
    using SafeERC20 for IERC20;

    address public immutable originalToken;
    address public immutable swapV0;

    constructor(address originalToken_, address swapV0_) {
        originalToken = originalToken_;
        swapV0 = swapV0_;
    }

    /// @notice Attempt to exploit SwapV0 by bypassing wrapper verification
    /// @dev This function attempts to:
    ///      1. Take tokens from the caller
    ///      2. Transfer them to SwapV0
    ///      3. Call onUnwrapFinalizedReceived pretending to be a legitimate wrapper
    ///      4. Should be blocked if SwapV0 properly validates msg.sender
    function attemptExploit(
        uint256 amount,
        address router,
        uint256 amountOutMin,
        address[] memory path,
        uint256 deadline,
        address to
    ) external {
        // Receive tokens from caller
        IERC20(originalToken).safeTransferFrom(msg.sender, address(this), amount);

        // Transfer tokens to SwapV0
        IERC20(originalToken).safeTransfer(swapV0, amount);

        // Encode the swap parameters
        SwapData memory swapData = SwapData(router, amountOutMin, path, deadline, to);
        bytes memory data = abi.encode(swapData);

        // Attempt to invoke onUnwrapFinalizedReceived directly
        // This attack would work should the swapper not validate msg.sender is a legitimate
        // wrapper registered with the coordinator. The attack exploits the fact that:
        // 1. checkPath only verifies that a wrapper exists for originalToken in the coordinator
        // 2. This malicious contract implements the required originalToken() function
        // 3. Without msg.sender validation, any contract can pretend to be a wrapper
        IWrapperReceiver(swapV0).onUnwrapFinalizedReceived(
            msg.sender,
            amount,
            0, // unwrapRequestId
            msg.sender,
            data
        );
    }
}
