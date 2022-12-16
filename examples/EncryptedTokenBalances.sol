// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.7.0 <0.9.0;

import "./lib/Ciphertext.sol";

contract EncryptedTokenBalances {
    // A mapping from address to a ciphertext handle.
    mapping(address => uint256) balances;

    // Sets the balance of the caller to the given encrypted balance.
    function setBalance(bytes calldata encryptedBalance) public {
        uint256 handle = Ciphertext.verify(encryptedBalance);
        balances[msg.sender] = handle;
    }

    // Reencrypts the balance of the caller under their public FHE key.
    // The FHE public key is automatically determined based on the origin of the call.
    function viewBalance() public view returns(bytes memory) {
        return Ciphertext.reencrypt(balances[msg.sender]);
    }
}
