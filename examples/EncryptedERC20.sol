// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.7.0 <0.9.0;

import "./lib/Ciphertext.sol";
import "./lib/Common.sol";
import "./lib/FHEOps.sol";

contract EncryptedERC20 {
    // A mapping from address to a ciphertext handle.
    mapping(address => uint256) balances;

    // Sets the balance of the caller to the given encrypted balance.
    constructor(bytes memory initialEncryptedAmount) {
        uint256 handle = Ciphertext.verify(initialEncryptedAmount);
        balances[msg.sender] = handle;
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
        uint256 amount = Ciphertext.verify(encryptedAmount);

        // Make sure the sender has enough tokens.
        Common.requireCt(FHEOps.lte(amount, balances[from]));

        // Add to the balance of `to`.
        balances[to] = FHEOps.add(balances[to], amount);
    }
}
