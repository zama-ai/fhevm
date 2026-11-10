// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @dev Fixture for test/upgrades/storageExceptions.ts: the ACL namespace with the Permission struct and a second, incompatible change.
contract ACL {
    struct Permission {
        bool allowed;
        uint48 depth;
        uint48 storedBlock;
    }

    /// @custom:storage-location erc7201:fhevm.storage.ACL
    struct ACLStorage {
        mapping(bytes32 handle => mapping(address account => Permission permission)) persistedAllowedPairs;
        mapping(bytes32 handle => uint256 isAllowedForDecryption) allowedForDecryption;
    }

    function fixture() external pure returns (uint256) {
        return 3;
    }
}
