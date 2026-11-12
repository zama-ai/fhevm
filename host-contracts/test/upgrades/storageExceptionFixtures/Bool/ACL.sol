// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @dev Fixture for test/upgrades/storageExceptions.ts: the ACL namespace with the bool permission of ACL v0.5.
contract BoolACL {
    /// @custom:storage-location erc7201:fhevm.storage.ACL
    struct ACLStorage {
        mapping(bytes32 handle => mapping(address account => bool isAllowed)) persistedAllowedPairs;
        mapping(bytes32 handle => bool isAllowedForDecryption) allowedForDecryption;
    }

    function fixture() external pure returns (uint256) {
        return 1;
    }
}
