// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title ExtraData library
 * @notice Library that provides utility functions for handling extra data
 * @dev Extra data is expected to have the first byte as a version byte, followed by the payload.
 */
library ExtraData {
    /// @notice Error indicating that the extra data does not have a version byte
    error NoVersionByteInExtraData();

    /// @notice Parses the extra data and separates the version and payload.
    /// @param extraData The bytes containing the version and payload.
    /// @return version The first byte (version)
    /// @return payload The remaining bytes (payload)
    function parse(bytes memory extraData) internal pure returns (uint8 version, bytes memory payload) {
        // Ensure at least 1 byte for version
        if (extraData.length < 1) {
            revert NoVersionByteInExtraData();
        }

        // The first byte of extraData is the version
        version = uint8(extraData[0]);

        // Allocate a new bytes array with everything after the first byte (version byte)
        uint256 payloadLength = extraData.length - 1;
        payload = new bytes(payloadLength);

        for (uint256 i = 0; i < payloadLength; i++) {
            // Copy each byte from extraData skipping the version byte
            payload[i] = extraData[i + 1];
        }
    }
}
