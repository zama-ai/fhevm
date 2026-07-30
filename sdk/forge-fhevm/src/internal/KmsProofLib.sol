// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

/**
 * Builds a KMS-signed decryption proof — the payload a gateway posts back to a contract that awaits a
 * decryption result, which the contract then checks with
 * `KMSVerifier.verifyDecryptionEIP712KMSSignatures(handles, decryptedResult, decryptionProof)`.
 *
 * This is the ASYNC decryption path, and it is a different thing from `FhevmTest.publicDecrypt` /
 * `userDecrypt`: those read a value out of the cleartext stack for the TEST to assert on, whereas this
 * forges the proof so the CONTRACT UNDER TEST can be driven through its own result-verification branch —
 * a branch that is otherwise unreachable in a test and typically holds the interesting state transitions.
 *
 * Wire format: [numSigners:1][signatures:65*S][extraData]
 */
library KmsProofLib {
    bytes32 private constant EIP712_DOMAIN_TYPEHASH =
        keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");

    /// @dev Must match `KMSVerifier.EIP712_PUBLIC_DECRYPT_TYPE` exactly.
    bytes32 private constant DECRYPTION_RESULT_TYPEHASH =
        keccak256("PublicDecryptVerification(bytes32[] ctHandles,bytes decryptedResult,bytes extraData)");

    function domainSeparator(string memory name, string memory version, uint256 chainId, address verifyingContract)
        internal
        pure
        returns (bytes32)
    {
        return keccak256(
            abi.encode(
                EIP712_DOMAIN_TYPEHASH, keccak256(bytes(name)), keccak256(bytes(version)), chainId, verifyingContract
            )
        );
    }

    /// @notice The digest the KMS signers sign, mirroring `KMSVerifier._hashPublicDecryptionResult`.
    function digest(
        bytes32[] memory handles,
        bytes memory decryptedResult,
        bytes memory extraData,
        bytes32 separator
    ) internal pure returns (bytes32) {
        bytes32 structHash = keccak256(
            abi.encode(
                DECRYPTION_RESULT_TYPEHASH,
                keccak256(abi.encodePacked(handles)),
                keccak256(decryptedResult),
                keccak256(abi.encodePacked(extraData))
            )
        );
        return keccak256(abi.encodePacked(hex"1901", separator, structHash));
    }

    /// @notice The leading count byte is how `KMSVerifier` locates `extraData`, so it must be exact.
    function assemble(bytes[] memory signatures, bytes memory extraData) internal pure returns (bytes memory proof) {
        proof = abi.encodePacked(uint8(signatures.length));
        for (uint256 i; i < signatures.length; ++i) {
            proof = abi.encodePacked(proof, signatures[i]);
        }
        proof = abi.encodePacked(proof, extraData);
    }
}
