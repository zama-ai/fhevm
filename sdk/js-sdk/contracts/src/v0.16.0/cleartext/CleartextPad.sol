// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

/**
 * @title CleartextPad
 * @notice Cleartext stand-in for the RFC-030 homomorphic PRF `PRF_{FHE(k)}(label)`, cleartext mode only.
 * @dev Counter mode: block `i` is keccak256("FHE_cpad" ‖ prf ‖ label ‖ uint32(i)) (256 bits); the stream is
 *      the blocks' bytes concatenated, and stream bit `j` is bit `j % 8` (least significant first) of byte
 *      `j / 8`. Deterministic in `(prf, label)` only, as RFC-030 requires. Shared by the pad decryption mock
 *      (`CleartextInputVerifier`) and the unmasking mock (`CleartextFHEVMExecutor`) so both see the same pad.
 */
library CleartextPad {
    /// @notice Domain tag of the cleartext PRF.
    bytes8 internal constant CLEARTEXT_PAD_DOMAIN_SEPARATOR = "FHE_cpad";

    /// @notice Bits per block of the cleartext PRF (one keccak256 output).
    uint256 internal constant BLOCK_BITS = 256;

    /// @notice PRF block `blockIndex` of the pad stream of `(prf, label)`.
    function padBlock(uint16 prf, bytes32 label, uint256 blockIndex) internal pure returns (bytes32) {
        // forge-lint: disable-next-line(unsafe-typecast)
        return keccak256(abi.encodePacked(CLEARTEXT_PAD_DOMAIN_SEPARATOR, prf, label, uint32(blockIndex)));
    }

    /// @notice The ⌈padLength/256⌉ whole blocks covering the first `padLength` bits of the pad stream.
    function padBlocks(uint16 prf, bytes32 label, uint256 padLength) internal pure returns (bytes memory pad) {
        uint256 numBlocks = (padLength + BLOCK_BITS - 1) / BLOCK_BITS;
        pad = new bytes(numBlocks * 32);
        for (uint256 i = 0; i < numBlocks; ++i) {
            bytes32 blockBits = padBlock(prf, label, i);
            assembly {
                mstore(add(add(pad, 32), mul(i, 32)), blockBits)
            }
        }
    }

    /// @notice Byte `byteIndex` of the pad stream of `(prf, label)`.
    function padByte(uint16 prf, bytes32 label, uint256 byteIndex) internal pure returns (uint8) {
        return uint8(padBlock(prf, label, byteIndex / 32)[byteIndex % 32]);
    }
}
