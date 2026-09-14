// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title   BytesOps library
 * @notice  Library that provides utility functions for bytes arrays.
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
        assembly {
            mcopy(add(result, 32), add(add(data, 32), offset), size)
        }
    }
}
