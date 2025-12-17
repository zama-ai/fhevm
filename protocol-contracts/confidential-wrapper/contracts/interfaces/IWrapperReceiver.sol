// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {ebool, euint64} from "@fhevm/solidity/lib/FHE.sol";

/// @dev Interface for contracts that can receive callbacks after a Wrapper unwrap is finalized.
/// @notice Implementing this interface allows contracts to be notified when an unwrap completes,
///         enabling composable operations like swaps or other token transformations.
interface IWrapperReceiver {
    /**
     * @dev Called by the Wrapper after an unwrap operation is finalized and underlying tokens are transferred.
     * @notice This callback is invoked AFTER the underlying tokens have been sent to the receiver.
     * @param operator The address that initiated the finalization
     * @param amount The amount of underlying tokens that were unwrapped
     * @param unwrapRequestId The unique identifier for this unwrap request
     * @param data Arbitrary callback data that was provided during the original unwrap request
     * @return A boolean indicating success of the callback operation
     */
    function onUnwrapFinalizedReceived(
        address operator,
        uint256 amount,
        uint256 unwrapRequestId,
        address refundTo,
        bytes calldata data
    ) external returns (bool);
}
