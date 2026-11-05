// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {IKMSGeneration} from "../interfaces/IKMSGeneration.sol";

/**
 * @title   KeyDigestHash library
 * @notice  EIP-712 hash of a KeyDigest[] array, shared by KMSGeneration and ProtocolConfig.
 */
library KeyDigestHash {
    /// @dev Hash of the KeyDigest type. EIP-712 requires a type hash for each nested struct type.
    bytes32 private constant EIP712_KEY_DIGEST_TYPE_HASH = keccak256("KeyDigest(uint8 keyType,bytes digest)");

    /**
     * @notice Returns the EIP-712 hash of `keyDigests`: the keccak256 of the packed struct hashes.
     */
    function hash(IKMSGeneration.KeyDigest[] calldata keyDigests) internal pure returns (bytes32) {
        bytes32[] memory keyDigestHashes = new bytes32[](keyDigests.length);
        for (uint256 i = 0; i < keyDigests.length; i++) {
            keyDigestHashes[i] = keccak256(
                abi.encode(EIP712_KEY_DIGEST_TYPE_HASH, keyDigests[i].keyType, keccak256(keyDigests[i].digest))
            );
        }
        return keccak256(abi.encodePacked(keyDigestHashes));
    }
}
