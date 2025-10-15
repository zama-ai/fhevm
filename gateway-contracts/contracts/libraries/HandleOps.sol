// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../shared/FheType.sol";

/**
 * @title HandleOps library
 * @notice Library that provides utility functions for ciphertext handles
 * @dev Handles have the following format:
 * [21 first random bytes from hashing] | index_21 | chainID_22...29 | fheType_30 | version_31
 */
library HandleOps {
    /**
     * @notice Error indicating that the FHE type is invalid
     * @param fheTypeUint8 The invalid FHE type as a uint8
     */
    error InvalidFHEType(uint8 fheTypeUint8);

    /**
     * @notice Extracts the chain ID from a ciphertext handle
     * @param handle The ciphertext handle
     * @return The chain ID
     */
    function extractChainId(bytes32 handle) internal pure returns (uint256) {
        // The chain ID is a 64-bit integer (8 bytes) represented by the handles' 23rd to 30th
        // bytes (index 22 to 29).
        // We thus cast the handle to a uint256, shift it 2 bytes (2*8=16 bits) to the right
        // and mask the result with a 64-bit mask to retrieve the expected value
        // We then cast the result to a uint256 for consistency with the usual chain ID type
        // Note that right shift + masking is slightly more gas efficient then left + right shift
        // when extracting multiple bytes
        return uint256((uint256(handle) >> 16) & 0xFFFFFFFFFFFFFFFF);
    }

    /**
     * @notice Extracts the FHE type from a ciphertext handle
     * @param handle The ciphertext handle
     * @return The FHE type
     */
    function extractFheType(bytes32 handle) internal pure returns (FheType) {
        // The FHE type is a 8-bit integer (1 byte) represented by the handles' 31st byte (index 30)
        // We thus shift the handle by 30 bytes (30*8=240 bits) to the left and then shift it
        // by 31 bytes (31*8=248 bits) to the right to retrieve the expected value
        // We then cast the result to a uint8 in order to better represent the expected enum.
        // Note that extracting a single byte left + right shift is slightly more gas efficient then :
        // - right shift + masking (`uint256(handle) >> 8) & 0xFF`)
        // - directly extract the byte at index 30 (`handle[30]`)
        uint8 fheTypeUint8 = uint8(uint256((handle << 240) >> 248));

        // Check that the FHE type is valid. Revert with an explicit error if it is not.
        if (fheTypeUint8 > uint8(type(FheType).max)) {
            revert InvalidFHEType(fheTypeUint8);
        }

        return FheType(fheTypeUint8);
    }
}
