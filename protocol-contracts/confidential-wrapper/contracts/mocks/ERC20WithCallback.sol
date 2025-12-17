// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity 0.8.27;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {WrapperUpgradeable} from "../wrapper/WrapperUpgradeable.sol";

/// @notice Interface for tokens that call back to the sender
interface ITransferCallback {
    function onTokenTransfer(address from, address to, uint256 amount) external;
}

/// @notice Mock ERC20 token with transfer callback functionality for testing reentrancy protection
/// @dev Calls back to the sender during transferFrom to enable reentrancy testing
contract ERC20WithCallback is ERC20 {
    uint8 private _decimals;

    constructor(
        string memory name_,
        string memory symbol_,
        uint8 decimals_
    ) ERC20(name_, symbol_) {
        _decimals = decimals_;
    }

    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }

    function decimals() public view override returns (uint8) {
        return _decimals;
    }

    /// @dev Override transferFrom to add callback functionality
    function transferFrom(address from, address to, uint256 value) public virtual override returns (bool) {
        address spender = _msgSender();
        _spendAllowance(from, spender, value);

        // Callback to 'from' address BEFORE transfer (like ERC777 tokensToSend)
        // This enables reentrancy attacks
        if (from.code.length > 0) {
            try ITransferCallback(from).onTokenTransfer(from, to, value) {
                // Callback succeeded
            } catch {
                // Callback failed, continue with transfer
            }
        }

        _transfer(from, to, value);
        return true;
    }
}

/// @notice Malicious contract that attempts reentrancy during token transfer callbacks
/// @dev Implements callback to reenter wrap() during token transfers
contract ReentrancyAttacker is ITransferCallback {
    WrapperUpgradeable public wrapper;
    ERC20WithCallback public token;
    address public attacker;
    uint256 public attackCount;
    uint256 public reentrancyAttempts;
    uint256 public maxAttacks;
    bool public attacking;
    uint256 public wrapAmount;

    event ReentrancyAttempted(uint256 attemptNumber, bool success, string reason);
    event AttackStarted(uint256 maxAttacks);
    event AttackEnded(uint256 totalAttempts);

    constructor(WrapperUpgradeable wrapper_, ERC20WithCallback token_) {
        wrapper = wrapper_;
        token = token_;
        attacker = msg.sender;
    }

    /// @notice Start a reentrancy attack
    /// @param amount Amount to wrap
    /// @param maxAttacks_ Maximum number of reentrancy attempts (1 = no reentrancy, 2+ = reentrancy)
    function attack(uint256 amount, uint256 maxAttacks_) external {
        require(!attacking, "Attack already in progress");

        attacking = true;
        attackCount = 0;
        reentrancyAttempts = 0;
        maxAttacks = maxAttacks_;
        wrapAmount = amount;

        emit AttackStarted(maxAttacks_);

        // Approve wrapper to spend tokens
        token.approve(address(wrapper), type(uint256).max);

        // Start the wrap - this will trigger onTokenTransfer callback
        wrapper.wrap(address(this), amount);

        attacking = false;
        emit AttackEnded(attackCount);
    }

    /// @notice Callback function - called during token transfer
    /// @dev This is where the reentrancy attack happens
    function onTokenTransfer(
        address from,
        address /* to */,
        uint256 /* amount */
    ) external override {
        // Increment callback counter
        attackCount++;

        // Only reenter if we're in an attack, the callback is from our token,
        // and we haven't exceeded max reentrancy attempts
        // maxAttacks = 1 means no reentrancy (just count the call)
        // maxAttacks > 1 means attempt reentrancy up to (maxAttacks - 1) times
        if (attacking && msg.sender == address(token) && from == address(this) && reentrancyAttempts < (maxAttacks - 1)) {
            reentrancyAttempts++;

            // Attempt to reenter wrap() - this should fail with ReentrancyGuard
            try wrapper.wrap(address(this), wrapAmount) {
                emit ReentrancyAttempted(reentrancyAttempts, true, "Reentrancy succeeded");
            } catch Error(string memory reason) {
                emit ReentrancyAttempted(reentrancyAttempts, false, reason);
            } catch (bytes memory) {
                emit ReentrancyAttempted(reentrancyAttempts, false, "Reentrancy failed");
            }
        }
    }

    /// @notice Withdraw tokens from this contract
    function withdraw() external {
        uint256 balance = token.balanceOf(address(this));
        if (balance > 0) {
            token.transfer(msg.sender, balance);
        }
    }

    /// @notice Receive function to accept ETH
    receive() external payable {}
}
