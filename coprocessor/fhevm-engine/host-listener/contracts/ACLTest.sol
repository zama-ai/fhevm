// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "contracts/ACLEvents.sol";

contract ACLTest is ACLEvents {

    function allow(bytes32 handle, address account) public {
        emit Allowed(msg.sender, account, handle);
    }


    function delegateForUserDecryption(
        address delegate,
        address contractAddress,
        uint64 delegationCounter,
        uint64 oldExpiryDate,
        uint64 newExpiryDate
    ) public virtual {
        emit DelegatedForUserDecryption(
            msg.sender,
            delegate,
            contractAddress,
            delegationCounter,
            oldExpiryDate,
            newExpiryDate
        );
    }
}