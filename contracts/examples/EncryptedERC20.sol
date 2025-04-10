// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/HTTPZ.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";

import "../lib/HTTPZConfig.sol";

/// @notice This contract implements an encrypted ERC20-like token with confidential balances using Zama's FHE (Fully Homomorphic Encryption) library.
/// @dev It supports typical ERC20 functionality such as transferring tokens, minting, and setting allowances, but uses encrypted data types.
contract EncryptedERC20 is Ownable2Step {
    /// @notice Emitted when tokens are transferred
    event Transfer(address indexed from, address indexed to);
    /// @notice Emitted when a spender is approved to spend tokens on behalf of an owner
    event Approval(address indexed owner, address indexed spender);
    /// @notice Emitted when new tokens are minted
    event Mint(address indexed to, uint64 amount);

    /// @dev Stores the total supply of the token
    uint64 private _totalSupply;
    /// @dev Name of the token (e.g., "Confidential Token")
    string private _name;
    /// @dev Symbol of the token (e.g., "CTK")
    string private _symbol;
    /// @notice Number of decimal places for the token
    uint8 public constant decimals = 6;

    /// @dev A mapping from address to an encrypted balance - tracks encrypted balances of each address
    mapping(address => euint64) internal balances;

    /// @dev Mapping to manage encrypted allowance - of the form mapping(owner => mapping(spender => allowance)).
    mapping(address => mapping(address => euint64)) internal allowances;

    /// @notice Constructor to initialize the token's name and symbol, and set up the owner
    /// @param name_ The name of the token
    /// @param symbol_ The symbol of the token
    constructor(string memory name_, string memory symbol_) Ownable(msg.sender) {
        HTTPZ.setCoprocessor(HTTPZConfig.defaultConfig()); // Set up the FHEVM configuration for this contract
        _name = name_;
        _symbol = symbol_;
    }

    /// @notice Returns the name of the token.
    function name() public view virtual returns (string memory) {
        return _name;
    }

    /// @notice Returns the symbol of the token, usually a shorter version of the name.
    function symbol() public view virtual returns (string memory) {
        return _symbol;
    }

    /// @notice Returns the total supply of the token
    function totalSupply() public view virtual returns (uint64) {
        return _totalSupply;
    }

    /// @notice Mints new tokens and assigns them to the owner, increasing the total supply.
    /// @dev Only the contract owner can call this function.
    /// @param mintedAmount The amount of tokens to mint
    function mint(uint64 mintedAmount) public virtual onlyOwner {
        balances[owner()] = HTTPZ.add(balances[owner()], mintedAmount); // overflow impossible because of next line
        HTTPZ.allowThis(balances[owner()]);
        HTTPZ.allow(balances[owner()], owner());
        _totalSupply = _totalSupply + mintedAmount;
        emit Mint(owner(), mintedAmount);
    }

    /// @notice Transfers an encrypted amount from the message sender address to the `to` address.
    /// @param to The recipient address
    /// @param encryptedAmount The encrypted amount to transfer
    /// @param inputProof The proof for the encrypted input
    /// @return bool indicating success of the transfer
    function transfer(address to, einput encryptedAmount, bytes calldata inputProof) public virtual returns (bool) {
        transfer(to, HTTPZ.asEuint64(encryptedAmount, inputProof));
        return true;
    }

    /// @notice Transfers an encrypted amount from the message sender address to the `to` address.
    /// @param to The recipient address
    /// @param amount The encrypted amount to transfer
    /// @return bool indicating success of the transfer
    function transfer(address to, euint64 amount) public virtual returns (bool) {
        require(HTTPZ.isSenderAllowed(amount));
        /// @dev Makes sure the owner has enough tokens
        ebool canTransfer = HTTPZ.le(amount, balances[msg.sender]);
        _transfer(msg.sender, to, amount, canTransfer);
        return true;
    }

    /// @notice Returns the balance handle (encrypted) of a specific address.
    /// @param wallet The address to check the balance of
    /// @return euint64 The encrypted balance of the address
    function balanceOf(address wallet) public view virtual returns (euint64) {
        return balances[wallet];
    }

    /// @notice Sets the allowance of `spender` to use a specific encrypted amount of the caller's tokens.
    /// @param spender The address authorized to spend
    /// @param encryptedAmount The encrypted amount to approve
    /// @param inputProof The proof for the encrypted input
    /// @return bool indicating success of the approval
    function approve(address spender, einput encryptedAmount, bytes calldata inputProof) public virtual returns (bool) {
        approve(spender, HTTPZ.asEuint64(encryptedAmount, inputProof));
        return true;
    }

    /// @notice Sets the allowance of `spender` to use a specific amount of the caller's tokens.
    /// @param spender The address authorized to spend
    /// @param amount The amount to approve
    /// @return bool indicating success of the approval
    function approve(address spender, euint64 amount) public virtual returns (bool) {
        require(HTTPZ.isSenderAllowed(amount));
        address owner = msg.sender;
        _approve(owner, spender, amount);
        emit Approval(owner, spender);
        return true;
    }

    /// @notice Returns the remaining number of tokens that `spender` is allowed to spend on behalf of the caller.
    /// @param owner The address that owns the tokens
    /// @param spender The address authorized to spend
    /// @return euint64 The remaining allowance
    function allowance(address owner, address spender) public view virtual returns (euint64) {
        return _allowance(owner, spender);
    }

    /// @notice Transfers `encryptedAmount` tokens using the caller's allowance.
    /// @param from The address to transfer from
    /// @param to The address to transfer to
    /// @param encryptedAmount The encrypted amount to transfer
    /// @param inputProof The proof for the encrypted input
    /// @return bool indicating success of the transfer
    function transferFrom(
        address from,
        address to,
        einput encryptedAmount,
        bytes calldata inputProof
    ) public virtual returns (bool) {
        transferFrom(from, to, HTTPZ.asEuint64(encryptedAmount, inputProof));
        return true;
    }

    /// @notice Transfers `amount` tokens using the caller's allowance.
    /// @param from The address to transfer from
    /// @param to The address to transfer to
    /// @param amount The amount to transfer
    /// @return bool indicating success of the transfer
    function transferFrom(address from, address to, euint64 amount) public virtual returns (bool) {
        require(HTTPZ.isSenderAllowed(amount));
        address spender = msg.sender;
        ebool isTransferable = _updateAllowance(from, spender, amount);
        _transfer(from, to, amount, isTransferable);
        return true;
    }

    /// @notice Internal function to approve a spender to use a specific amount.
    /// @dev Updates the allowance mapping and sets appropriate permissions
    /// @param owner The address that owns the tokens
    /// @param spender The address authorized to spend
    /// @param amount The amount to approve
    function _approve(address owner, address spender, euint64 amount) internal virtual {
        allowances[owner][spender] = amount;
        HTTPZ.allowThis(amount);
        HTTPZ.allow(amount, owner);
        HTTPZ.allow(amount, spender);
    }

    /// @notice Returns the internal allowance of a spender for a specific owner.
    /// @param owner The address that owns the tokens
    /// @param spender The address authorized to spend
    /// @return euint64 The current allowance
    function _allowance(address owner, address spender) internal view virtual returns (euint64) {
        return allowances[owner][spender];
    }

    /// @notice Updates the allowance after a transfer and returns whether it is valid.
    /// @dev Checks if the transfer is allowed based on current allowance and balance
    /// @param owner The address that owns the tokens
    /// @param spender The address authorized to spend
    /// @param amount The amount of the proposed transfer
    /// @return ebool indicating whether the transfer is allowed
    function _updateAllowance(address owner, address spender, euint64 amount) internal virtual returns (ebool) {
        euint64 currentAllowance = _allowance(owner, spender);
        /// @dev Makes sure the allowance suffices
        ebool allowedTransfer = HTTPZ.le(amount, currentAllowance);
        /// @dev Makes sure the owner has enough tokens
        ebool canTransfer = HTTPZ.le(amount, balances[owner]);
        ebool isTransferable = HTTPZ.and(canTransfer, allowedTransfer);
        _approve(owner, spender, HTTPZ.select(isTransferable, HTTPZ.sub(currentAllowance, amount), currentAllowance));
        return isTransferable;
    }

    /// @notice Internal function to handle the transfer of tokens between addresses.
    /// @dev Updates balances and sets appropriate permissions
    /// @param from The address to transfer from
    /// @param to The address to transfer to
    /// @param amount The amount to transfer
    /// @param isTransferable Boolean indicating if the transfer is allowed
    function _transfer(address from, address to, euint64 amount, ebool isTransferable) internal virtual {
        /// @dev Add to the balance of `to` and subract from the balance of `from`.
        euint64 transferValue = HTTPZ.select(isTransferable, amount, HTTPZ.asEuint64(0));
        euint64 newBalanceTo = HTTPZ.add(balances[to], transferValue);
        balances[to] = newBalanceTo;
        HTTPZ.allowThis(newBalanceTo);
        HTTPZ.allow(newBalanceTo, to);
        euint64 newBalanceFrom = HTTPZ.sub(balances[from], transferValue);
        balances[from] = newBalanceFrom;
        HTTPZ.allowThis(newBalanceFrom);
        HTTPZ.allow(newBalanceFrom, from);
        emit Transfer(from, to);
    }
}
