// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

import "./lib/Ciphertext.sol";
import "./lib/Common.sol";
import "./lib/FHEOps.sol";

contract EncryptedERC20 {
    // A mapping from address to an encrypted balance.
    mapping(address => FheUInt) balances;

    // The owner of the contract.
    address internal owner;

    constructor() {
        owner = msg.sender;
    }

    // Sets the balance of the owner to the given encrypted balance.
    function mint(bytes calldata encryptedAmount) public onlyOwner {
        balances[owner] = Ciphertext.verify(encryptedAmount);
    }
    
    // Transfers an encrypted amount from the message sender address to the `to` address.
    function transfer(address to, bytes calldata encryptedAmount) public {
        _transfer(msg.sender, to, encryptedAmount);
    }

    // Reencrypts the balance of the caller under their public FHE key.
    // The FHE public key is automatically determined based on the origin of the call.
    function reencrypt() public view returns(bytes memory) {
        return Ciphertext.reencrypt(balances[msg.sender]);
    }

    // Transfers an encrypted amount.
    function _transfer(address from, address to, bytes calldata encryptedAmount) internal {
        FheUInt amount = Ciphertext.verify(encryptedAmount);

        // Make sure the sender has enough tokens.
        Common.requireCt(FHEOps.lte(amount, balances[from]));

        // Add to the balance of `to` and subract from the balance of `from`.
        balances[to] = FHEOps.add(balances[to], amount);
        balances[from] = FHEOps.sub(balances[from], amount);
    }

    modifier onlyOwner {
        require(msg.sender == owner);
        _;
    }
}
