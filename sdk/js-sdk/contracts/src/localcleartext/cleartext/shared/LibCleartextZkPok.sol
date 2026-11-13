// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title LibCleartextZkPok
 * @notice Reads the plaintext of a ZKPoK input out of its input proof, cleartext mode only.
 * @dev Nothing is encrypted on a cleartext stack: the SDK's cleartext path (and forge's
 *      LibForgeFhevmEncrypt) appends one 32-byte plaintext per handle to the proof, after the signatures:
 *        numHandles 1 | numSigners 1 | handles 32×numHandles | signatures 65×numSigners |
 *        cleartexts 32×numHandles | extraData version
 *      The real InputVerifier treats that trailing region as opaque extraData folded into the signed
 *      EIP-712 digest, so it authenticates the plaintexts without reading them.
 *
 *      The ZKPoK counterpart of LibCleartextPad, which recovers an RFC-030 pad-based input instead.
 */
library LibCleartextZkPok {
    /**
     * @notice The plaintext `inputProof` carries for `inputHandle`, if it carries one.
     * @dev Not finding one is not an error: the proof has already passed InputVerifier, and a valid proof
     *      need not carry plaintexts (one signed by a real coprocessor carries the version byte alone).
     *      The caller then records nothing, leaving the handle unknown to the store rather than worth 0.
     * @return foundCleartext Whether `inputHandle` is in the proof and its plaintext slot is present.
     * @return cleartext The plaintext, unnormalized; 0 when not found.
     */
    function tryReadCleartext(bytes32 inputHandle, bytes memory inputProof)
        internal
        pure
        returns (bool foundCleartext, uint256 cleartext)
    {
        if (inputProof.length < 2) {
            return (false, 0);
        }

        uint8 numHandles = uint8(inputProof[0]);
        uint8 numSigners = uint8(inputProof[1]);
        uint256 cleartextStart = 2 + uint256(numHandles) * 32 + uint256(numSigners) * 65;

        if (inputProof.length < cleartextStart + 32) {
            return (false, 0);
        }

        for (uint8 i = 0; i < numHandles; i++) {
            uint256 handleOffset = 2 + uint256(i) * 32;
            bytes32 handleInProof;
            assembly {
                handleInProof := mload(add(add(inputProof, 32), handleOffset))
            }

            if (handleInProof != inputHandle) {
                continue;
            }

            uint256 cleartextOffset = cleartextStart + uint256(i) * 32;
            if (inputProof.length < cleartextOffset + 32) {
                return (false, 0);
            }

            assembly {
                cleartext := mload(add(add(inputProof, 32), cleartextOffset))
            }
            return (true, cleartext);
        }

        return (false, 0);
    }
}
