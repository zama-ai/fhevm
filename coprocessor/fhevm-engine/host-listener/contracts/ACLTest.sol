// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "contracts/ACLEvents.sol";

contract ACLTest is ACLEvents {

    function allow(bytes32 handle, address account) public {
        emit Allowed(msg.sender, account, handle);
    }

}