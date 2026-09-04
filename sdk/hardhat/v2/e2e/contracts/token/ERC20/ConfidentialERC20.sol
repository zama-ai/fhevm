// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHE, ebool, euint64, externalEuint64} from "@fhevm/solidity/lib/FHE.sol";
import {IERC20Errors} from "@openzeppelin/contracts/interfaces/draft-IERC6093.sol";
import {IConfidentialERC20} from "./IConfidentialERC20.sol";
import {FHEErrors} from "../../utils/FHEErrors.sol";

/**
 * @title   ConfidentialERC20.
 * @notice  This contract implements an encrypted ERC20-like token with confidential balances using
 *          Zama's FHE (Fully Homomorphic Encryption) library.
 * @dev     It supports standard ERC20 functions such as transferring tokens, minting,
 *          and setting allowances, but uses encrypted data types.
 *          The total supply is not encrypted.
 */
abstract contract ConfidentialERC20 is IConfidentialERC20, IERC20Errors, FHEErrors {
    /// @notice Used as a placeholder in `Approval` & `Transfer` events to comply with the official EIP20.
    uint256 internal constant _PLACEHOLDER = type(uint256).max;
    /// @notice Total supply.
    uint64 internal _totalSupply;

    /// @notice Name.
    string internal _name;

    /// @notice Symbol.
    string internal _symbol;

    /// @notice A mapping from `account` address to an encrypted `balance`.
    mapping(address account => euint64 balance) internal _balances;

    /// @notice A mapping of the form mapping(account => mapping(spender => allowance)).
    mapping(address account => mapping(address spender => euint64 allowance)) internal _allowances;

    /**
     * @param name_     Name of the token.
     * @param symbol_   Symbol.
     */
    constructor(string memory name_, string memory symbol_) {
        _name = name_;
        _symbol = symbol_;
    }

    /**
     * @notice See {IConfidentialERC20-approve}.
     */
    function approve(address spender, externalEuint64 encryptedAmount, bytes calldata inputProof)
        public
        virtual
        returns (bool)
    {
        approve(spender, FHE.fromExternal(encryptedAmount, inputProof));
        return true;
    }

    /**
     * @notice See {IConfidentialERC20-approve}.
     */
    function approve(address spender, euint64 amount) public virtual returns (bool) {
        _isSenderAllowedForAmount(amount);
        address owner = msg.sender;
        _approve(owner, spender, amount);
        emit Approval(owner, spender, _PLACEHOLDER);
        return true;
    }

    /**
     * @notice See {IConfidentialERC20-transfer}.
     */
    function transfer(address to, externalEuint64 encryptedAmount, bytes calldata inputProof)
        public
        virtual
        returns (bool)
    {
        transfer(to, FHE.fromExternal(encryptedAmount, inputProof));
        return true;
    }

    /**
     * @notice See {IConfidentialERC20-transfer}.
     */
    function transfer(address to, euint64 amount) public virtual returns (bool) {
        _isSenderAllowedForAmount(amount);

        /// @dev Make sure the owner has enough tokens.
        ebool canTransfer = FHE.le(amount, _balances[msg.sender]);
        _transfer(msg.sender, to, amount, canTransfer);
        return true;
    }

    /**
     * @notice See {IConfidentialERC20-transferFrom}.
     */
    function transferFrom(address from, address to, externalEuint64 encryptedAmount, bytes calldata inputProof)
        public
        virtual
        returns (bool)
    {
        transferFrom(from, to, FHE.fromExternal(encryptedAmount, inputProof));
        return true;
    }

    /**
     * @notice See {IConfidentialERC20-transferFrom}.
     */
    function transferFrom(address from, address to, euint64 amount) public virtual returns (bool) {
        _isSenderAllowedForAmount(amount);
        address spender = msg.sender;
        ebool isTransferable = _updateAllowance(from, spender, amount);
        _transfer(from, to, amount, isTransferable);
        return true;
    }

    /**
     * @notice See {IConfidentialERC20-allowance}.
     */
    function allowance(address owner, address spender) public view virtual returns (euint64) {
        return _allowance(owner, spender);
    }

    /**
     * @notice See {IConfidentialERC20-balanceOf}.
     */
    function balanceOf(address account) public view virtual returns (euint64) {
        return _balances[account];
    }

    /**
     * @notice See {IConfidentialERC20-decimals}.
     */
    function decimals() public view virtual returns (uint8) {
        return 6;
    }

    /**
     * @notice See {IConfidentialERC20-name}.
     */
    function name() public view virtual returns (string memory) {
        return _name;
    }

    /**
     * @notice See {IConfidentialERC20-symbol}.
     */
    function symbol() public view virtual returns (string memory) {
        return _symbol;
    }

    /**
     * @notice See {IConfidentialERC20-totalSupply}.
     */
    function totalSupply() public view virtual returns (uint64) {
        return _totalSupply;
    }

    function _approve(address owner, address spender, euint64 amount) internal virtual {
        if (owner == address(0)) {
            revert ERC20InvalidApprover(owner);
        }

        if (spender == address(0)) {
            revert ERC20InvalidSpender(spender);
        }

        _allowances[owner][spender] = amount;
        FHE.allowThis(amount);
        FHE.allow(amount, owner);
        FHE.allow(amount, spender);
    }

    /**
     * @dev It does not incorporate any overflow check. It must be implemented
     *      by the function calling it.
     */
    function _unsafeMint(address account, uint64 amount) internal virtual {
        _unsafeMintNoEvent(account, amount);
        emit Transfer(address(0), account, _PLACEHOLDER);
    }

    /**
     * @dev It does not incorporate any overflow check. It must be implemented
     *      by the function calling it.
     */
    function _unsafeMintNoEvent(address account, uint64 amount) internal virtual {
        euint64 newBalanceAccount = FHE.add(_balances[account], amount);
        _balances[account] = newBalanceAccount;
        FHE.allowThis(newBalanceAccount);
        FHE.allow(newBalanceAccount, account);
    }

    function _transfer(address from, address to, euint64 amount, ebool isTransferable) internal virtual {
        _transferNoEvent(from, to, amount, isTransferable);
        emit Transfer(from, to, _PLACEHOLDER);
    }

    function _transferNoEvent(address from, address to, euint64 amount, ebool isTransferable) internal virtual {
        if (from == address(0)) {
            revert ERC20InvalidSender(from);
        }

        if (to == address(0)) {
            revert ERC20InvalidReceiver(to);
        }

        /// @dev Add to the balance of `to` and subtract from the balance of `from`.
        euint64 transferValue = FHE.select(isTransferable, amount, FHE.asEuint64(0));
        euint64 newBalanceTo = FHE.add(_balances[to], transferValue);
        _balances[to] = newBalanceTo;
        FHE.allowThis(newBalanceTo);
        FHE.allow(newBalanceTo, to);
        euint64 newBalanceFrom = FHE.sub(_balances[from], transferValue);
        _balances[from] = newBalanceFrom;
        FHE.allowThis(newBalanceFrom);
        FHE.allow(newBalanceFrom, from);
    }

    function _updateAllowance(address owner, address spender, euint64 amount) internal virtual returns (ebool) {
        euint64 currentAllowance = _allowance(owner, spender);
        /// @dev Make sure sure the allowance suffices.
        ebool allowedTransfer = FHE.le(amount, currentAllowance);
        /// @dev Make sure the owner has enough tokens.
        ebool canTransfer = FHE.le(amount, _balances[owner]);
        ebool isTransferable = FHE.and(canTransfer, allowedTransfer);
        _approve(owner, spender, FHE.select(isTransferable, FHE.sub(currentAllowance, amount), currentAllowance));
        return isTransferable;
    }

    function _allowance(address owner, address spender) internal view virtual returns (euint64) {
        return _allowances[owner][spender];
    }

    function _isSenderAllowedForAmount(euint64 amount) internal view virtual {
        if (!FHE.isSenderAllowed(amount)) {
            revert FHESenderNotAllowed();
        }
    }
}
