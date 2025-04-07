// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @title HandleOps library
/// @notice Library that provides utility functions for ciphertext handles
/// @dev Handles have the following format:
/// @dev [21 first random bytes from hashing] | index_21 | chainID_22...29 | type_30 | version_31
library HandleOps {
    /// @notice Extracts the chainId from a ciphertext handle
    /// @param handle The ciphertext handle
    /// @return The chainId
    function extractChainId(bytes32 handle) internal pure returns (uint256) {
        /// @dev The chainId is a 64-bit integer, represented by the handles' bytes (index 22 to 29,
        /// @dev which is 8 bytes, or 8*8=64 bits).
        /// @dev We thus cast the handle to a uint256, shift it 2 bytes (2*8=16 bits) to the right
        /// @dev and mask the result with a 64-bit mask to retrieve the expected value
        /// @dev We then cast the result to a uint256 for consistency with the usual chainId type
        return uint256((uint256(handle) >> 16) & 0xFFFFFFFFFFFFFFFF);
    }
}
