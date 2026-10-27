// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FheType, LibFheType} from "./LibFheType.sol";

/**
 * @title LibCleartextPad
 * @notice Cleartext stand-in for the RFC-030 homomorphic PRF `PRF_{FHE(k)}(label)`, cleartext mode only.
 * @dev Counter mode: block `i` is keccak256("FHE_cpad" ‖ prf ‖ label ‖ uint32(i)) (256 bits); the stream is
 *      the blocks' bytes concatenated, and stream bit `j` is bit `j % 8` (least significant first) of byte
 *      `j / 8`. Deterministic in `(prf, label)` only, as RFC-030 requires. The pad decryption mock and the
 *      unmasking mock must both use it so they see the same pad.
 *
 *      `padBlock` and `padBlocks` are ported from fhevm's sdk/js-sdk/contracts/src/v0.16.0/cleartext/
 *      CleartextPad.sol; keep them byte-compatible, since the SDK's cleartext pad decryption derives the same
 *      stream. `unmaskPart` is added here: upstream keeps it in its CleartextFHEVMExecutor. `padLabel`,
 *      `sessionId` and `xorMask` are the pure parts of upstream's CleartextInputVerifier pad decryption mock,
 *      byte-compatible with it too: the SDK computes the same label, and reads the session id and the mask back.
 */
library LibCleartextPad {
    /// @notice Domain tag of the cleartext PRF.
    bytes8 internal constant CLEARTEXT_PAD_DOMAIN_SEPARATOR = "FHE_cpad";

    /// @notice Bits per block of the cleartext PRF (one keccak256 output).
    uint256 internal constant BLOCK_BITS = 256;

    /// @notice Domain tag of the RFC-030 label preimage.
    bytes8 internal constant PAD_LABEL_DOMAIN_SEPARATOR = "FHE_padl";

    /// @notice Domain tag of the cleartext session id (stand-in for the MPC session, cleartext mode only).
    bytes8 internal constant CLEARTEXT_SESSION_DOMAIN_SEPARATOR = "FHE_csid";

    /**
     * @notice An RFC-030 label: keccak256("FHE_padl" ‖ witnessVersion ‖ prf ‖ chainId ‖ submitter ‖ consumer ‖
     *         expirationTimestamp ‖ decryptorCount ‖ decryptors ‖ nonce).
     * @dev Fixed-width big-endian fields; each decryptor is 20 bytes (not ABI-padded).
     *      Production: the client (SDK) computes the label; Connector, PaddedInputter, coprocessor verify it.
     */
    function padLabel(
        uint8 witnessVersion,
        uint16 prf,
        uint64 chainId,
        address submitter,
        address consumer,
        uint64 expirationTimestamp,
        address[] memory decryptors,
        bytes16 nonce
    ) internal pure returns (bytes32) {
        bytes memory preimage = abi.encodePacked(
            PAD_LABEL_DOMAIN_SEPARATOR,
            witnessVersion,
            prf,
            chainId,
            submitter,
            consumer,
            expirationTimestamp,
            // forge-lint: disable-next-line(unsafe-typecast)
            uint8(decryptors.length)
        );
        for (uint256 i = 0; i < decryptors.length; ++i) {
            preimage = abi.encodePacked(preimage, decryptors[i]);
        }
        return keccak256(abi.encodePacked(preimage, nonce));
    }

    /// @notice RFC-030 `PadShare` EIP-712 typehash.
    bytes32 internal constant PAD_SHARE_TYPEHASH = keccak256("PadShare(bytes32 label,bytes16 sessionId,bytes share)");

    /**
     * @notice RFC-030 `PadDecryptResponsePayload` EIP-712 typehash. The RFC names the struct but does not pin its
     *         encoding yet: this is the SDK's (`kmsPadDecryptResponseEip712Types`), field names included.
     */
    bytes32 internal constant PAD_DECRYPT_RESPONSE_TYPEHASH = keccak256(
        "PadDecryptResponsePayload(uint32 packingFactor,bytes extraData,uint32 degree,uint32 partyId,PadShare[] pad_shares)"
        "PadShare(bytes32 label,bytes16 sessionId,bytes share)"
    );

    /// @notice EIP-712 struct hash of one `PadShare`.
    function padShareStructHash(bytes32 label, bytes16 sessionId_, bytes memory share) internal pure returns (bytes32) {
        return keccak256(abi.encode(PAD_SHARE_TYPEHASH, label, sessionId_, keccak256(share)));
    }

    /**
     * @notice EIP-712 struct hash of a `PadDecryptResponsePayload`.
     * @param padSharesHash keccak256 of the concatenated `padShareStructHash`es, in order.
     */
    function padDecryptResponseStructHash(
        uint32 packingFactor,
        bytes memory extraData,
        uint32 degree,
        uint32 partyId,
        bytes32 padSharesHash
    ) internal pure returns (bytes32) {
        return keccak256(
            abi.encode(
                PAD_DECRYPT_RESPONSE_TYPEHASH, packingFactor, keccak256(extraData), degree, partyId, padSharesHash
            )
        );
    }

    /// @notice The cleartext mock's request id: keccak256(label_1 ‖ … ‖ label_n ‖ extraData ‖ signature).
    function padRequestId(bytes32[] memory labels, bytes memory extraData, bytes memory signature)
        internal
        pure
        returns (bytes32)
    {
        return keccak256(abi.encodePacked(labels, extraData, signature));
    }

    /// @notice The cleartext session id of `(prf, label)`: one mock "session" per label, shared by every node.
    function sessionId(uint16 prf, bytes32 label) internal pure returns (bytes16) {
        return bytes16(keccak256(abi.encodePacked(CLEARTEXT_SESSION_DOMAIN_SEPARATOR, prf, label)));
    }

    /// @notice XORs `data` in place with `mask` repeated every 32 bytes, and returns it (NOT real encryption).
    function xorMask(bytes memory data, bytes32 mask) internal pure returns (bytes memory) {
        for (uint256 i = 0; i < data.length; ++i) {
            data[i] = data[i] ^ mask[i % 32];
        }
        return data;
    }

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

    /**
     * @dev Cleartext unmasking of one RFC-030 input part: m = c ⊕ r over the part's bytes, least significant
     *      byte first, with the padding bits cleared (as the coprocessor does). The pad `r` is this library's
     *      stream; the caller normalizes the result to the type.
     *
     *      Reads only the fields it needs from the package, whose layout (big-endian integers) is
     *        packageType 1 | witnessVersion 1 | prf 2 | chainId 8 | submitter 20 | consumer 20 |
     *        expirationTimestamp 8 | decryptorCount 1 | decryptors 20×decryptorCount | nonce 16 |
     *        label 32 | partCount 1 | parts 1×partCount | ciphertext N/8
     *      and trusts it: PaddedInputter has already validated the package and the external handle, so the
     *      offsets are in bounds and every part type is supported.
     *
     * @param externalHandle The external handle: packageDigest[0..28] | inputIndex | type | version.
     * @param inputPackage The whole input package.
     * @param inputType The type of the part, as checked against the external handle.
     */
    function unmaskPart(bytes32 externalHandle, bytes memory inputPackage, FheType inputType)
        internal
        pure
        returns (uint256 value)
    {
        uint16 prf = (uint16(uint8(inputPackage[2])) << 8) | uint8(inputPackage[3]);
        bytes32 label;
        uint256 ciphertextOffset;
        uint256 byteOffset;
        // Scoped so the parse-only locals are freed before the unmask loop (stack depth).
        {
            // label = package[witnessEnd .. witnessEnd + 32), with witnessEnd = 61 + 20 × decryptorCount + 16.
            uint256 witnessEnd = 77 + 20 * uint256(uint8(inputPackage[60]));
            assembly {
                label := mload(add(add(inputPackage, 32), witnessEnd))
            }
            uint256 partsOffset = witnessEnd + 33;
            ciphertextOffset = partsOffset + uint8(inputPackage[witnessEnd + 32]);

            // A part occupies ⌈bitWidth/8⌉ bytes of the packed stream; the parts before it set its offset.
            uint8 inputIndex = uint8(externalHandle[29]);
            for (uint256 i = 0; i < inputIndex; ++i) {
                byteOffset += (LibFheType.bitWidthForType(FheType(uint8(inputPackage[partsOffset + i]))) + 7) / 8;
            }
        }
        uint256 bitWidth = LibFheType.bitWidthForType(inputType);
        uint256 byteLength = (bitWidth + 7) / 8;

        // A part is at most 32 bytes, so it spans at most two pad blocks: hash each one once.
        uint256 blockIndex = type(uint256).max;
        bytes32 currentBlock;
        for (uint256 k = 0; k < byteLength; ++k) {
            uint256 j = byteOffset + k;
            if (j / 32 != blockIndex) {
                blockIndex = j / 32;
                currentBlock = padBlock(prf, label, blockIndex);
            }
            uint8 c = uint8(inputPackage[ciphertextOffset + j]);
            value |= uint256(c ^ uint8(currentBlock[j % 32])) << (8 * k);
        }
        if (bitWidth < 256) value &= (uint256(1) << bitWidth) - 1;
    }
}
