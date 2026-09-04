// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {euint64, externalEuint64} from "@fhevm/solidity/lib/FHE.sol";

/**
 * @title   IConfidentialERC20.
 * @notice  Interface that defines ERC20-like tokens with encrypted balances.
 */
interface IConfidentialERC20 {
    /**
     * @notice              Emitted when the allowance of a `spender` for an `owner` is set by
     *                      a call to {approve}.
     * @param owner         Owner address.
     * @param spender       Spender address.
     * @param placeholder   Placeholder.
     */
    event Approval(address indexed owner, address indexed spender, uint256 placeholder);

    /**
     * @notice              Emitted when tokens are moved from one account (`from`) to
     *                      another (`to`).
     * @param from          Sender address.
     * @param to            Receiver address.
     * @param transferId    If the implementation does not support error handling, it must be set to a default
     *                      placeholder (typically equal to max(uint256). However, it must be set to a transferId
     *                      if the implementation supports encrypted error handling.
     */
    event Transfer(address indexed from, address indexed to, uint256 transferId);

    /**
     * @notice                  Set the `encryptedAmount` as the allowance of `spender` over the caller's tokens.
     * @param spender           Spender address.
     * @param encryptedAmount   Encrypted amount.
     * @param inputProof        Input proof.
     * @return isSuccess        Whether it succeeds.
     */
    function approve(address spender, externalEuint64 encryptedAmount, bytes calldata inputProof)
        external
        returns (bool isSuccess);

    /**
     * @notice                  Set the `amount` as the allowance of `spender` over the caller's tokens.
     * @param spender           Spender address.
     * @param amount            Encrypted amount.
     * @return isSuccess        Whether it succeeds.
     */
    function approve(address spender, euint64 amount) external returns (bool isSuccess);

    /**
     * @notice                  Transfer an encrypted amount from the message sender address to the `to` address.
     * @param to                Receiver address.
     * @param encryptedAmount   Encrypted amount.
     * @param inputProof        Input proof.
     * @return isSuccess        Whether it succeeds.
     */
    function transfer(address to, externalEuint64 encryptedAmount, bytes calldata inputProof)
        external
        returns (bool isSuccess);

    /**
     * @notice              Transfer an amount from the message sender address to the `to` address.
     * @param to            Receiver address.
     * @param amount        Encrypted amount.
     * @return isSuccess    Whether it succeeds.
     */
    function transfer(address to, euint64 amount) external returns (bool isSuccess);

    /**
     * @notice              Transfer `amount` tokens using the caller's allowance.
     * @param from          Sender address.
     * @param to            Receiver address.
     * @param amount        Encrypted amount.
     * @return isSuccess    Whether it succeeds.
     */
    function transferFrom(address from, address to, euint64 amount) external returns (bool isSuccess);

    /**
     * @notice                  Transfer `encryptedAmount` tokens using the caller's allowance.
     * @param from              Sender address.
     * @param to                Receiver address.
     * @param encryptedAmount   Encrypted amount.
     * @param inputProof        Input proof.
     * @return isSuccess        Whether it succeeds.
     */
    function transferFrom(address from, address to, externalEuint64 encryptedAmount, bytes calldata inputProof)
        external
        returns (bool isSuccess);

    /**
     * @notice              Return the remaining number of tokens that `spender` is allowed to spend
     *                      on behalf of the `owner`.
     * @param owner         Owner address.
     * @param spender       Spender address.
     * @return allowance    Allowance handle of the spender on behalf of the owner.
     */
    function allowance(address owner, address spender) external view returns (euint64 allowance);

    /**
     * @notice          Return the balance handle of the `account`.
     * @param account   Account address.
     * @return balance  Balance handle of the `account`.
     */
    function balanceOf(address account) external view returns (euint64 balance);

    /**
     * @notice          Return the number of decimals.
     * @return decimals Number of decimals (e.g. 6).
     */
    function decimals() external view returns (uint8 decimals);

    /**
     * @notice          Return the name of the token.
     * @return name     Name of the token (e.g. "TestToken").
     */
    function name() external view returns (string memory name);

    /**
     * @notice          Return the symbol of the token.
     * @return symbol   Symbol of the token (e.g. "TEST").
     */
    function symbol() external view returns (string memory symbol);

    /**
     * @notice              Return the total supply of the token.
     * @return totalSupply  Total supply of the token.
     */
    function totalSupply() external view returns (uint64 totalSupply);
}
