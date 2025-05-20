// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

contract ACL {
    event Allowed(address indexed caller, address indexed account, bytes32 handle);
    event AllowedForDecryption(address indexed caller, bytes32[] handlesList);
    event NewDelegation(address indexed caller, address indexed delegatee, address[] contractAddresses);
    event RevokedDelegation(address indexed caller, address indexed delegatee, address[] contractAddresses);

    function allow(bytes32 handle, address account) public {
        emit Allowed(msg.sender, account, handle);
    }

}