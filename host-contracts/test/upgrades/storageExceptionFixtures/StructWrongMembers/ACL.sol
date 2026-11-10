// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @dev Fixture for test/upgrades/storageExceptions.ts: the ACL namespace with a Permission struct whose bool is not first.
contract ACL {
    struct Permission {
        uint48 depth;
        bool allowed;
        uint48 storedBlock;
    }

    /// @custom:storage-location erc7201:fhevm.storage.ACL
    struct ACLStorage {
        mapping(bytes32 handle => mapping(address account => Permission permission)) persistedAllowedPairs;
        mapping(bytes32 handle => bool isAllowedForDecryption) allowedForDecryption;
    }

    function fixture() external pure returns (uint256) {
        return 4;
    }
}
