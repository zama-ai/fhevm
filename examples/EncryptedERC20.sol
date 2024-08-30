// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";
import "../payment/Payment.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";

contract EncryptedERC20 is Ownable2Step {
    event Transfer(address indexed from, address indexed to);
    event Approval(address indexed owner, address indexed spender);
    event Mint(address indexed to, uint64 amount);

    uint64 private _totalSupply;
    string private _name;
    string private _symbol;
    uint8 public constant decimals = 6;

    // A mapping from address to an encrypted balance.
    mapping(address => euint64) internal balances;

    // A mapping of the form mapping(owner => mapping(spender => allowance)).
    mapping(address => mapping(address => euint64)) internal allowances;

    constructor(string memory name_, string memory symbol_) payable Ownable(msg.sender) {
        Payment.depositForThis(msg.value);
        _name = name_;
        _symbol = symbol_;
    }

    // Returns the name of the token.
    function name() public view virtual returns (string memory) {
        return _name;
    }

    // Returns the symbol of the token, usually a shorter version of the name.
    function symbol() public view virtual returns (string memory) {
        return _symbol;
    }

    // Returns the total supply of the token
    function totalSupply() public view virtual returns (uint64) {
        return _totalSupply;
    }

    // Sets the balance of the owner to the given encrypted balance.
    function mint(uint64 mintedAmount) public virtual onlyOwner {
        balances[owner()] = TFHE.add(balances[owner()], mintedAmount); // overflow impossible because of next line
        TFHE.allowThis(balances[owner()]);
        TFHE.allow(balances[owner()], owner());
        _totalSupply = _totalSupply + mintedAmount;
        emit Mint(owner(), mintedAmount);
    }

    // Transfers an encrypted amount from the message sender address to the `to` address.
    function transfer(address to, einput encryptedAmount, bytes calldata inputProof) public virtual returns (bool) {
        transfer(to, TFHE.asEuint64(encryptedAmount, inputProof));
        return true;
    }

    // Transfers an amount from the message sender address to the `to` address.
    function transfer(address to, euint64 amount) public virtual returns (bool) {
        require(TFHE.isSenderAllowed(amount));
        // makes sure the owner has enough tokens
        ebool canTransfer = TFHE.le(amount, balances[msg.sender]);
        _transfer(msg.sender, to, amount, canTransfer);
        return true;
    }

    // Returns the balance handle of the caller.
    function balanceOf(address wallet) public view virtual returns (euint64) {
        return balances[wallet];
    }

    // Sets the `encryptedAmount` as the allowance of `spender` over the caller's tokens.
    function approve(address spender, einput encryptedAmount, bytes calldata inputProof) public virtual returns (bool) {
        approve(spender, TFHE.asEuint64(encryptedAmount, inputProof));
        return true;
    }

    // Sets the `amount` as the allowance of `spender` over the caller's tokens.
    function approve(address spender, euint64 amount) public virtual returns (bool) {
        require(TFHE.isSenderAllowed(amount));
        address owner = msg.sender;
        _approve(owner, spender, amount);
        emit Approval(owner, spender);
        return true;
    }

    // Returns the remaining number of tokens that `spender` is allowed to spend
    // on behalf of the caller.
    function allowance(address owner, address spender) public view virtual returns (euint64) {
        return _allowance(owner, spender);
    }

    // Transfers `encryptedAmount` tokens using the caller's allowance.
    function transferFrom(
        address from,
        address to,
        einput encryptedAmount,
        bytes calldata inputProof
    ) public virtual returns (bool) {
        transferFrom(from, to, TFHE.asEuint64(encryptedAmount, inputProof));
        return true;
    }

    // Transfers `amount` tokens using the caller's allowance.
    function transferFrom(address from, address to, euint64 amount) public virtual returns (bool) {
        require(TFHE.isSenderAllowed(amount));
        address spender = msg.sender;
        ebool isTransferable = _updateAllowance(from, spender, amount);
        _transfer(from, to, amount, isTransferable);
        return true;
    }

    function _approve(address owner, address spender, euint64 amount) internal virtual {
        allowances[owner][spender] = amount;
        TFHE.allowThis(amount);
        TFHE.allow(amount, owner);
        TFHE.allow(amount, spender);
    }

    function _allowance(address owner, address spender) internal view virtual returns (euint64) {
        return allowances[owner][spender];
    }

    function _updateAllowance(address owner, address spender, euint64 amount) internal virtual returns (ebool) {
        euint64 currentAllowance = _allowance(owner, spender);
        // makes sure the allowance suffices
        ebool allowedTransfer = TFHE.le(amount, currentAllowance);
        // makes sure the owner has enough tokens
        ebool canTransfer = TFHE.le(amount, balances[owner]);
        ebool isTransferable = TFHE.and(canTransfer, allowedTransfer);
        _approve(owner, spender, TFHE.select(isTransferable, TFHE.sub(currentAllowance, amount), currentAllowance));
        return isTransferable;
    }

    // Transfers an encrypted amount.
    function _transfer(address from, address to, euint64 amount, ebool isTransferable) internal virtual {
        // Add to the balance of `to` and subract from the balance of `from`.
        euint64 transferValue = TFHE.select(isTransferable, amount, TFHE.asEuint64(0));
        euint64 newBalanceTo = TFHE.add(balances[to], transferValue);
        balances[to] = newBalanceTo;
        TFHE.allowThis(newBalanceTo);
        TFHE.allow(newBalanceTo, to);
        euint64 newBalanceFrom = TFHE.sub(balances[from], transferValue);
        balances[from] = newBalanceFrom;
        TFHE.allowThis(newBalanceFrom);
        TFHE.allow(newBalanceFrom, from);
        emit Transfer(from, to);
    }
}
