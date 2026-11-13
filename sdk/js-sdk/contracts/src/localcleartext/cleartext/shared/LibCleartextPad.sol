// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FheType, LibFheType} from "./LibFheType.sol";

/**
 * @title LibCleartextPad
 * @notice Cleartext stand-in for the RFC-030 homomorphic PRF `PRF_{FHE(k)}(label)`, cleartext mode only.
 * @dev Counter mode: block `i` is keccak256("FHE_cpad" ‖ prfId ‖ label ‖ i) (256 bits), `i` a little-endian 32-bit
 *      index as RFC-030 seeds the PRF with `label ‖ i` (tech-spec#553); the stream is
 *      the blocks' bytes concatenated, and stream bit `j` is bit `j % 8` (least significant first) of byte
 *      `j / 8`. Deterministic in `(prfId, label)` only, as RFC-030 requires. The pad decryption mock and the
 *      unmasking mock must both use it so they see the same pad.
 *
 *      `cleartextPrfBlock` and `cleartextPrfBlocks` are ported from fhevm's sdk/js-sdk/contracts/src/v0.16.0/cleartext/
 *      CleartextPad.sol, which still encodes the index big-endian: the SDK's cleartext pad decryption derives the
 *      same stream, so it must move to the little-endian index too. `unmaskPart` is added here: upstream keeps it in its CleartextFHEVMExecutor. `padLabel`,
 *      `sessionId` and `xorMask` are the pure parts of upstream's CleartextInputVerifier pad decryption mock,
 *      byte-compatible with it too: the SDK computes the same label, and reads the session id and the mask back.
 */
library LibCleartextPad {
    /// @notice Domain tag of the cleartext PRF.
    bytes8 internal constant CLEARTEXT_PAD_DOMAIN_SEPARATOR = "FHE_cpad";

    /// @notice Domain tag of the cleartext session id (stand-in for the MPC session, cleartext mode only).
    bytes8 internal constant CLEARTEXT_SESSION_DOMAIN_SEPARATOR = "FHE_csid";

    /**
     * @notice Version byte leading every cleartext `rawShares`: the RFC leaves the encoding of the ring elements to
     *         the KMS. Version 0 (cleartext mode only) is the pad itself, not a share of it: one 256-bit PRF block
     *         per ring element, `rawShares = 0x00 ‖ block_0 ‖ … ‖ block_{k-1}` for `k = ⌈padLength/256⌉`.
     */
    uint8 internal constant CLEARTEXT_RAW_SHARES_VERSION = 0x00;

    /// @notice The `extraData` of every cleartext share: version 0, no metadata.
    bytes internal constant CLEARTEXT_SHARE_EXTRA_DATA = hex"00";

    /// @notice Bits per block of the cleartext PRF (one keccak256 output).
    uint256 internal constant BLOCK_BITS = 256;

    /// @notice Domain tag of the RFC-030 label preimage.
    /// @dev Defined in https://github.com/zama-ai/tech-spec/blob/950587724a33f72066e864a8934ee0909144d147/rfcs/030-pad-based-input.md#L64
    bytes8 internal constant PAD_LABEL_DOMAIN_SEPARATOR = "FHE_padl";

    /**
     * @notice An RFC-030 label: keccak256("FHE_padl" ‖ witnessVersion ‖ prfId ‖ chainId ‖ submitter ‖ consumer ‖
     *         expirationTimestamp ‖ decryptorCount ‖ decryptors ‖ nonce).
     * @dev Fixed-width big-endian fields; each decryptor is 20 bytes (not ABI-padded).
     *      Production: the client (SDK) computes the label; Connector, PaddedInputter, coprocessor verify it.
     *      Defined in https://github.com/zama-ai/tech-spec/blob/950587724a33f72066e864a8934ee0909144d147/rfcs/030-pad-based-input.md#L61
     */
    function padLabel(
        uint8 witnessVersion,
        uint16 prfId,
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
            prfId,
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

    /// @notice RFC-030 `PadShareVerification` EIP-712 typehash: the share signature, one per pad share.
    /// @dev Defined in https://github.com/zama-ai/tech-spec/blob/950587724a33f72066e864a8934ee0909144d147/rfcs/030-pad-based-input.md#L411-L419
    bytes32 internal constant PAD_SHARE_VERIFICATION_TYPEHASH = keccak256(
        "PadShareVerification(bytes32 label,bytes rawShares,uint32 partyId,uint32 degree,bytes32 contextId,bytes16 sessionId,bytes extraData)"
    );

    /// @notice RFC-030 `PadDecryptResponseVerification` EIP-712 typehash: the transport signature, one per response.
    /// @dev Defined in https://github.com/zama-ai/tech-spec/blob/950587724a33f72066e864a8934ee0909144d147/rfcs/030-pad-based-input.md#L406-L409
    bytes32 internal constant PAD_DECRYPT_RESPONSE_VERIFICATION_TYPEHASH =
        keccak256("PadDecryptResponseVerification(bytes32 requestId,bytes encryptedPayload)");

    /// @notice EIP-712 struct hash of one share's `PadShareVerification`.
    /// @dev Defined in https://github.com/zama-ai/tech-spec/blob/950587724a33f72066e864a8934ee0909144d147/rfcs/030-pad-based-input.md#L411-L419
    function padShareVerificationStructHash(
        bytes32 label,
        bytes memory rawShares,
        uint32 partyId,
        uint32 degree,
        bytes32 contextId,
        bytes16 sessionId_,
        bytes memory extraData
    ) internal pure returns (bytes32) {
        return keccak256(
            abi.encode(
                PAD_SHARE_VERIFICATION_TYPEHASH,
                label,
                keccak256(rawShares),
                partyId,
                degree,
                contextId,
                sessionId_,
                keccak256(extraData)
            )
        );
    }

    /// @notice EIP-712 struct hash of a response's `PadDecryptResponseVerification`.
    /// @dev Defined in https://github.com/zama-ai/tech-spec/blob/950587724a33f72066e864a8934ee0909144d147/rfcs/030-pad-based-input.md#L406-L409
    function padDecryptResponseVerificationStructHash(bytes32 requestId, bytes memory encryptedPayload)
        internal
        pure
        returns (bytes32)
    {
        return keccak256(abi.encode(PAD_DECRYPT_RESPONSE_VERIFICATION_TYPEHASH, requestId, keccak256(encryptedPayload)));
    }

    /// @notice The cleartext `rawShares` of a pad: `CLEARTEXT_RAW_SHARES_VERSION` then its whole blocks.
    function cleartextRawShares(uint16 prfId, bytes32 label, uint256 padLength) internal pure returns (bytes memory) {
        return abi.encodePacked(CLEARTEXT_RAW_SHARES_VERSION, cleartextPrfBlocks(prfId, label, padLength));
    }

    /// @notice The cleartext mock's request id: keccak256(label_1 ‖ … ‖ label_n ‖ extraData ‖ signature).
    /// @dev The field it fills, `PadDecryptResponse.requestId`, is defined in https://github.com/zama-ai/tech-spec/blob/950587724a33f72066e864a8934ee0909144d147/rfcs/030-pad-based-input.md#L382
    ///      Its derivation is this mock's own: RFC-030 leaves it to the transport (RFC-002/004).
    function cleartextPadRequestId(bytes32[] memory labels, bytes memory extraData, bytes memory signature)
        internal
        pure
        returns (bytes32)
    {
        return keccak256(abi.encodePacked(labels, extraData, signature));
    }

    /// @notice The cleartext session id of `(prfId, label)`: one mock "session" per label, shared by every node.
    function sessionId(uint16 prfId, bytes32 label) internal pure returns (bytes16) {
        return bytes16(keccak256(abi.encodePacked(CLEARTEXT_SESSION_DOMAIN_SEPARATOR, prfId, label)));
    }

    /// @notice XORs `data` in place with `mask` repeated every 32 bytes, and returns it (NOT real encryption).
    function xorMask(bytes memory data, bytes32 mask) internal pure returns (bytes memory) {
        for (uint256 i = 0; i < data.length; ++i) {
            data[i] = data[i] ^ mask[i % 32];
        }
        return data;
    }

    /**
     * @notice Block `blockIndex` of the pad stream of `(prfId, label)`, from the CLEARTEXT MOCK of the RFC-030 PRF.
     *         NOT the production PRF, and never a secret: anyone can compute it, so every pad, and every input masked
     *         with it, is public. Cleartext mode only.
     * @dev Stands in for `PRF_{FHE(k)}(label ‖ i)`, which the coprocessor evaluates homomorphically under a key no one
     *      knows and only the KMS can decrypt: here `keccak256("FHE_cpad" ‖ prfId ‖ label ‖ i)`, in plaintext, with
     *      `prfId` in the role of the instance's key and a fixed 256-bit block. It keeps what the flow needs to be
     *      correct: determinism in `(prfId, label, i)` alone, counter mode, `i` a little-endian 32-bit index.
     *      Counter mode is defined in https://github.com/zama-ai/tech-spec/blob/950587724a33f72066e864a8934ee0909144d147/rfcs/030-pad-based-input.md#L188
     */
    function cleartextPrfBlock(uint16 prfId, bytes32 label, uint256 blockIndex) internal pure returns (bytes32) {
        // forge-lint: disable-next-line(unsafe-typecast)
        return
            keccak256(
                abi.encodePacked(CLEARTEXT_PAD_DOMAIN_SEPARATOR, prfId, label, littleEndian32(uint32(blockIndex)))
            );
    }

    /// @notice `value` as 4 bytes, least significant first: the RFC-030 encoding of a block index.
    function littleEndian32(uint32 value) internal pure returns (bytes4) {
        return bytes4((value >> 24) | ((value >> 8) & 0xff00) | ((value << 8) & 0xff0000) | (value << 24));
    }

    /// @notice The ⌈padLength/256⌉ whole blocks covering the first `padLength` bits of the pad stream, from the
    ///         CLEARTEXT MOCK of the RFC-030 PRF (`cleartextPrfBlock`): NOT the production PRF, and public.
    function cleartextPrfBlocks(uint16 prfId, bytes32 label, uint256 padLength)
        internal
        pure
        returns (bytes memory pad)
    {
        uint256 numBlocks = (padLength + BLOCK_BITS - 1) / BLOCK_BITS;
        pad = new bytes(numBlocks * 32);
        for (uint256 i = 0; i < numBlocks; ++i) {
            bytes32 blockBits = cleartextPrfBlock(prfId, label, i);
            assembly {
                mstore(add(add(pad, 32), mul(i, 32)), blockBits)
            }
        }
    }

    /**
     * @notice The host chain an RFC-030 input package names: package[4 .. 12), a big-endian uint64 (the witness's
     *         `chainId`, in the encoding of the chain id embedded in handles).
     * @dev The caller checks that the package is at least 12 bytes long; PaddedInputter already has.
     */
    function packageChainId(bytes memory inputPackage) internal pure returns (uint64 chainId) {
        for (uint256 i = 4; i < 12; ++i) {
            chainId = (chainId << 8) | uint8(inputPackage[i]);
        }
    }

    /**
     * @notice The label an RFC-030 input package carries, and the label its witness hashes to. They differ for a
     *         package whose label was not derived from its own witness.
     * @dev The witness is package[1 .. witnessEnd), with witnessEnd = 61 + 20 × decryptorCount + 16, encoded exactly
     *      as in the label preimage, so `expectedLabel = keccak256("FHE_padl" ‖ package[1 .. witnessEnd))`. The
     *      label follows it. The caller checks the package is long enough; PaddedInputter already has.
     *      Label defined in https://github.com/zama-ai/tech-spec/blob/950587724a33f72066e864a8934ee0909144d147/rfcs/030-pad-based-input.md#L61
     */
    function packageLabels(bytes memory inputPackage) internal pure returns (bytes32 label, bytes32 expectedLabel) {
        uint256 witnessEnd = 77 + 20 * uint256(uint8(inputPackage[60]));
        bytes memory witness = new bytes(witnessEnd - 1);
        assembly ("memory-safe") {
            mcopy(add(witness, 0x20), add(inputPackage, 0x21), mload(witness))
            label := mload(add(add(inputPackage, 0x20), witnessEnd))
        }
        expectedLabel = keccak256(abi.encodePacked(PAD_LABEL_DOMAIN_SEPARATOR, witness));
    }

    /**
     * @dev Cleartext unmasking of one RFC-030 input part: m = c ⊕ r over the part's bytes, least significant
     *      byte first, with the padding bits cleared (as the coprocessor does). The pad `r` is this library's
     *      stream; the caller normalizes the result to the type.
     *
     *      Reads only the fields it needs from the package, whose layout (big-endian integers) is
     *        packageType 1 | witnessVersion 1 | prfId 2 | chainId 8 | submitter 20 | consumer 20 |
     *        expirationTimestamp 8 | decryptorCount 1 | decryptors 20×decryptorCount | nonce 16 |
     *        label 32 | partCount 1 | parts 1×partCount | ciphertext N/8
     *      and trusts it: PaddedInputter has already validated the package and the external handle, so the
     *      offsets are in bounds and every part type is supported.
     *
     *      Differences with the production coprocessor (same resulting value):
     *      - the pad is `cleartextPrfBlock`'s public keccak stream, not `PRF_{FHE(k)}`;
     *      - plaintext XOR, not a homomorphic one (production: one lookup table per block);
     *      - blocks are 256 bits (`j / 32`, `j % 32`), not the instance's `b`;
     *      - one part per call, blocks recomputed each time: no unmasking of the whole package, no cache.
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
        // 1. The PRF instance identifier, which pins the PRF algorithm, key and parameters the pad comes from. It is
        //    package[2 .. 4), a big-endian uint16 (RFC-030 package layout): package[2] is the high byte and
        //    package[3] the low byte. Read as is, with no byte swap (only the block index is little-endian, in
        //    `cleartextPrfBlock`).
        uint16 prfId = (uint16(uint8(inputPackage[2])) << 8) | uint8(inputPackage[3]);

        bytes32 label;
        uint256 ciphertextOffset;
        uint256 byteOffset;

        // 2. Read three positions from the package: the label, the start of the ciphertext, and the byte offset,
        //    within the ciphertext, of the target part. A package carries a list of inputs (its parts), each with
        //    its own external handle; byte 29 of a handle is its input's index in that list (`inputIndex`). The
        //    target part is the one at this handle's index.
        //    Scoped so the parse-only locals are freed before the unmask loop (stack depth).
        {
            // 2.1 The label: package[witnessEnd .. witnessEnd + 32). The witness ends after the variable-length
            //     decryptor list: witnessEnd = 61 + 20 × decryptorCount + 16 (the nonce), decryptorCount = package[60].
            uint256 witnessEnd = 77 + 20 * uint256(uint8(inputPackage[60]));
            assembly {
                label := mload(add(add(inputPackage, 32), witnessEnd))
            }

            // 2.2 The ciphertext: after partCount (1 byte, at witnessEnd + 32) and the partCount type bytes.
            uint256 partsOffset = witnessEnd + 33;
            ciphertextOffset = partsOffset + uint8(inputPackage[witnessEnd + 32]);

            // 2.3 The target part's byte offset in the ciphertext: the sum of the sizes of the parts packed before
            //     it, a part of width w occupying ⌈w/8⌉ bytes.
            uint8 inputIndex = uint8(externalHandle[29]);
            for (uint256 i = 0; i < inputIndex; ++i) {
                byteOffset += (LibFheType.bitWidthForType(FheType(uint8(inputPackage[partsOffset + i]))) + 7) / 8;
            }
        }

        // 3. The target part's width, in bits and in whole bytes.
        uint256 bitWidth = LibFheType.bitWidthForType(inputType);
        uint256 byteLength = (bitWidth + 7) / 8;

        // 4. Unmask byte by byte, m = c ⊕ r, least significant byte first. Byte `j` of the ciphertext is masked by
        //    byte `j` of the pad stream, i.e. byte `j % 32` of block `j / 32`. A part is at most 32 bytes, so it
        //    spans at most two blocks: each is hashed once, when the loop first reaches it.
        uint256 blockIndex = type(uint256).max;
        bytes32 currentPadBlock;
        for (uint256 k = 0; k < byteLength; ++k) {
            // 4.1 The ciphertext byte `j`, the part's byte `k`, and the pad block that holds byte `j`.
            uint256 j = byteOffset + k;
            if (j / 32 != blockIndex) {
                blockIndex = j / 32;
                currentPadBlock = cleartextPrfBlock(prfId, label, blockIndex);
            }

            // 4.2 The masked byte.
            uint8 c = uint8(inputPackage[ciphertextOffset + j]);

            // 4.3 Unmasked, it becomes byte `k` of the value.
            value |= uint256(c ^ uint8(currentPadBlock[j % 32])) << (8 * k);
        }

        // 5. Clear the padding bits above the part's width (an ebool or a sub-byte width does not fill its byte),
        //    as the coprocessor does.
        if (bitWidth < 256) value &= (uint256(1) << bitWidth) - 1;
    }
}
