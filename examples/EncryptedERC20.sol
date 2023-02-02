// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

import "../lib/Ciphertext.sol";
import "../lib/Common.sol";
import "../lib/FHEOps.sol";

contract EncryptedERC20 {

    FHEUInt public totalSupply;
    string public name = "Naraggara"; // City of Zama's battle
    string public symbol = "NARA";
    uint8 public decimals = 18;

    // A mapping from address to an encrypted balance.
    mapping(address => FHEUInt) public balances;

    // A mapping of the form mapping(owner => mapping(spender => allowance)).
    mapping(address => mapping(address => FHEUInt)) allowances;

    // The owner of the contract.
    address internal contractOwner;

    constructor() {
        contractOwner = msg.sender;
    }

    // Sets the balance of the owner to the given encrypted balance.
    function mint(bytes calldata encryptedAmount) public onlyContractOwner {
        FHEUInt amount = Ciphertext.verify(encryptedAmount);
        balances[contractOwner] = amount;
        totalSupply = FHEOps.add(totalSupply, amount);
    }

    // Transfers an encrypted amount from the message sender address to the `to` address.
    function transfer(address to, bytes calldata encryptedAmount) public {
        transfer(to, Ciphertext.verify(encryptedAmount));
    }

    // Transfers an amount from the message sender address to the `to` address.
    function transfer(address to, FHEUInt amount) public {
        _transfer(msg.sender, to, amount);
    }

    function getTotalSupply() public view returns (bytes memory) {
        return Ciphertext.reencrypt(totalSupply); // Should be decrypt later
    }

    // Returns the balance of the caller under their public FHE key.
    // The FHE public key is automatically determined based on the origin of the call.
    function balanceOf() public view returns (bytes memory) {
        return Ciphertext.reencrypt(balances[msg.sender]);
    }

    // Sets the `encryptedAmount` as the allowance of `spender` over the caller's tokens.
    function approve(address spender, bytes calldata encryptedAmount) public {
        address owner = msg.sender;
        _approve(owner, spender, Ciphertext.verify(encryptedAmount));
    }

    // Returns the remaining number of tokens that `spender` is allowed to spend
    // on behalf of the caller. The returned ciphertext is under the caller public FHE key.
    function allowance(address spender) public view returns (bytes memory) {
        address owner = msg.sender;
        return Ciphertext.reencrypt(_allowance(owner, spender));
    }

    // Transfers `encryptedAmount` tokens using the caller's allowance.
    function transferFrom(address from, address to, bytes calldata encryptedAmount) public {
        transferFrom(from, to, Ciphertext.verify(encryptedAmount));
    }

    // Transfers `amount` tokens using the caller's allowance.
    function transferFrom(address from, address to, FHEUInt amount) public {
        address spender = msg.sender;
        _updateAllowance(from, spender, amount);
        _transfer(from, to, amount);
    }

     function _approve(address owner, address spender, FHEUInt amount) internal {
        allowances[owner][spender] = amount;
     }

     function _allowance(address owner, address spender) internal view returns (FHEUInt) {
         return allowances[owner][spender];
     }

     function _updateAllowance(address owner, address spender, FHEUInt amount) internal {
         FHEUInt currentAllowance = _allowance(owner, spender);
         Common.requireCt(FHEOps.lte(amount, currentAllowance));
         _approve(owner, spender, FHEOps.sub(currentAllowance, amount));
     }

    // Transfers an encrypted amount.
    function _transfer(
        address from,
        address to,
        FHEUInt amount
    ) internal {
        // Make sure the sender has enough tokens.
        Common.requireCt(FHEOps.lte(amount, balances[from]));

        // Add to the balance of `to` and subract from the balance of `from`.
        balances[to] = FHEOps.add(balances[to], amount);
        balances[from] = FHEOps.sub(balances[from], amount);
    }

    modifier onlyContractOwner() {
        require(msg.sender == contractOwner);
        _;
    }
}
