// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

library KMSDecryptionProofHelper {
    bytes32 internal constant EIP712_DOMAIN_TYPEHASH =
        keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");
    bytes32 internal constant DECRYPTION_RESULT_TYPEHASH =
        keccak256("PublicDecryptVerification(bytes32[] ctHandles,bytes decryptedResult,bytes extraData)");

    /// @notice Computes the EIP-712 domain separator used by the KMS verifier.
    /// @param name The EIP-712 domain name.
    /// @param version The EIP-712 domain version.
    /// @param chainId The chain id encoded in the domain.
    /// @param verifyingContract The verifier contract address encoded in the domain.
    /// @return The computed EIP-712 domain separator.
    function computeKMSDecryptionDomainSeparator(
        string memory name,
        string memory version,
        uint256 chainId,
        address verifyingContract
    ) internal pure returns (bytes32) {
        return keccak256(
            abi.encode(
                EIP712_DOMAIN_TYPEHASH, keccak256(bytes(name)), keccak256(bytes(version)), chainId, verifyingContract
            )
        );
    }

    /// @notice Computes the typed-data digest for decryption result verification.
    /// @param handlesList The ciphertext handles included in the decryption request.
    /// @param decryptedResult ABI-encoded cleartext values returned by decryption.
    /// @param extraData Extra trailing proof bytes included in the signed payload.
    /// @param domainSeparator The EIP-712 domain separator for the verifier context.
    /// @return The final EIP-712 digest signed by KMS signers.
    function computeDecryptionDigest(
        bytes32[] memory handlesList,
        bytes memory decryptedResult,
        bytes memory extraData,
        bytes32 domainSeparator
    ) internal pure returns (bytes32) {
        bytes32 structHash = keccak256(
            abi.encode(
                DECRYPTION_RESULT_TYPEHASH,
                keccak256(abi.encodePacked(handlesList)),
                keccak256(decryptedResult),
                keccak256(abi.encodePacked(extraData))
            )
        );
        return keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
    }

    /// @notice Builds the serialized KMS proof wire format.
    /// @param signatures The concatenated set of 65-byte ECDSA signatures.
    /// @param extraData The extra data bytes appended after signatures.
    /// @return proof The serialized proof bytes consumed by KMSVerifier.
    function assembleDecryptionProof(bytes[] memory signatures, bytes memory extraData)
        internal
        pure
        returns (bytes memory proof)
    {
        uint256 totalLength = 1 + extraData.length;
        for (uint256 i = 0; i < signatures.length; i++) {
            totalLength += signatures[i].length;
        }

        proof = new bytes(totalLength);
        proof[0] = bytes1(uint8(signatures.length));

        uint256 writeOffset = 1;
        for (uint256 i = 0; i < signatures.length; i++) {
            bytes memory signature = signatures[i];
            for (uint256 j = 0; j < signature.length; j++) {
                proof[writeOffset + j] = signature[j];
            }
            writeOffset += signature.length;
        }

        for (uint256 i = 0; i < extraData.length; i++) {
            proof[writeOffset + i] = extraData[i];
        }
    }
}
