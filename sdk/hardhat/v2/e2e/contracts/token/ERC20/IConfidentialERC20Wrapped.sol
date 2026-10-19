// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title   IConfidentialERC20Wrapped.
 * @notice  Interface that defines events, errors, and structs for
 *          contracts that wrap native assets or ERC20 tokens.
 */
interface IConfidentialERC20Wrapped {
    /// @notice Returned if the amount is greater than 2**64.
    error AmountTooHigh();

    /// @notice Returned if user cannot transfer or mint.
    error CannotTransferOrUnwrap();

    /**
     * @notice         Emitted when token is unwrapped.
     * @param account  Address of the account that unwraps tokens.
     * @param amount   Amount to unwrap.
     */
    event Unwrap(address indexed account, uint64 amount);

    /**
     * @notice         Emitted if unwrap fails due to lack of funds.
     * @param account  Address of the account that tried to unwrap.
     * @param amount   Amount to unwrap.
     */
    event UnwrapFailNotEnoughBalance(address account, uint64 amount);

    /**
     * @notice         Emitted if unwrap fails due to fail transfer.
     * @param account  Address of the account that tried to unwrap.
     * @param amount   Amount to unwrap.
     */
    event UnwrapFailTransferFail(address account, uint64 amount);

    /**
     * @notice         Emitted when token is wrapped.
     * @param account  Address of the account that wraps tokens.
     * @param amount   Amount to wrap.
     */
    event Wrap(address indexed account, uint64 amount);

    /**
     * @notice          This struct keeps track of the unwrap request information.
     * @param account   Address of the account that has initiated the unwrap request.
     * @param amount    Amount to be unwrapped.
     */
    struct UnwrapRequest {
        address account;
        uint64 amount;
    }
}
