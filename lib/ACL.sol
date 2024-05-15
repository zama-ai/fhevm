// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.25;

import "./TFHEExecutorAddress.sol";

contract ACL {
    mapping(uint256 => bool) public allowedForDecryption;

    // A set of (handle, address) pairs.
    // If address A is in the set for handle H, full access is granted to H for A.
    mapping(uint256 handle => mapping(address account => bool isAllowed)) public persistedAllowedPairs;

    mapping(address account => mapping(address delegatee => bool isDelegate)) public delegates;

    function allowTransient(uint256 handle, address account) public {
        if (msg.sender != TFHE_EXECUTOR_CONTRACT_ADDRESS) {
            require(isAllowed(handle, msg.sender), "sender doesn't own on allowedTransient");
        }
        bytes32 key = keccak256(abi.encodePacked(handle, account));
        assembly {
            tstore(key, 1)
            let length := tload(0)
            let lengthPlusOne := add(length, 1)
            tstore(lengthPlusOne, key)
            tstore(0, lengthPlusOne)
        }
    }

    function allowedTransient(uint256 handle, address account) public view returns (bool) {
        bool isAllowedTransient;
        bytes32 key = keccak256(abi.encodePacked(handle, account));
        assembly {
            isAllowedTransient := tload(key)
        }
        return isAllowedTransient;
    }

    function cleanAllTransientAllowed() external {
        // this function cleans the PPM, could be useful for integration with Account Abstraction when bundling several UserOps calling the FHEVMCoprocessor
        assembly {
            let length := tload(0)
            tstore(0, 0)
            let lengthPlusOne := add(length, 1)
            for {
                let i := 1
            } lt(i, lengthPlusOne) {
                i := add(i, 1)
            } {
                let handle := tload(i)
                tstore(i, 0)
                tstore(handle, 0)
            }
        }
    }

    // Allow use of `handle` for address `account`.
    // The caller must be allowed to use `c` for allow() to succeed. If not, allow() reverts.
    function allow(uint256 handle, address account) external {
        require(isAllowed(handle, msg.sender), "sender doesn't own on allowedTransient");
        persistedAllowedPairs[handle][account] = true;
    }

    // Returns true if address `a` is allowed to use `c` and false otherwise.
    function persistAllowed(uint256 handle, address account) public view returns (bool) {
        return persistedAllowedPairs[handle][account];
    }

    function isAllowed(uint256 handle, address account) public view returns (bool) {
        return allowedTransient(handle, account) || persistAllowed(handle, account);
    }

    function delegateAccount(address delegatee) external {
        delegates[msg.sender][delegatee] = true;
    }

    function removeDelegation(address delegatee) external {
        delegates[msg.sender][delegatee] = false;
    }

    function allowedOnBehalf(address delegatee, uint256 handle, address account) external view returns (bool) {
        return persistedAllowedPairs[handle][account] && delegates[account][delegatee];
    }

    function allowForDecryption(uint256[] memory ctsHandles) external {
        uint256 len = ctsHandles.length;
        for (uint256 k = 0; k < len; k++) {
            uint256 handle = ctsHandles[k];
            require(allowedTransient(handle, msg.sender), "sender doesn't own on isAllowedTransient");
            allowedForDecryption[handle] = true;
        }
    }
}
