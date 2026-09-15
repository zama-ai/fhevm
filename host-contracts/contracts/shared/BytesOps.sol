// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title   BytesOps library
 * @notice  Library that provides utility functions for bytes arrays.
 * @dev     Offsets are counted from the first byte of the array. In memory a bytes value starts
 *          with its 32-byte length, which is why the reads below shift every offset by 32.
 */
library BytesOps {
    /**
     * @notice          Returns a copy of the size bytes of data starting at offset.
     * @dev             The caller must have checked that the range lies within data.
     * @param data      The bytes array to copy from.
     * @param offset    The offset of the first byte to copy, counted from the first byte of data.
     * @param size      The number of bytes to copy.
     * @return result   The copied bytes.
     */
    function slice(bytes memory data, uint256 offset, uint256 size) internal pure returns (bytes memory result) {
        result = new bytes(size);
        assembly ("memory-safe") {
            mcopy(add(result, 32), add(add(data, 32), offset), size)
        }
    }

    /**
     * @notice          Returns the 32 bytes of data starting at offset.
     * @dev             The caller must have checked that the range lies within data.
     * @param data      The bytes array to read from.
     * @param offset    The offset of the first byte to read, counted from the first byte of data.
     * @return result   The 32 bytes read.
     */
    function readBytes32(bytes memory data, uint256 offset) internal pure returns (bytes32 result) {
        assembly ("memory-safe") {
            result := mload(add(add(data, 32), offset))
        }
    }
}
