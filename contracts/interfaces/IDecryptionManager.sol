// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

/// @title An interface for the decryption manager
/// @notice The decryption manager is responsible for decrypting ciphertext using a KMS
/// @notice Both user decryption and oracle decryption are handled
/// @dev Request functions are callable by any user or the relayer
/// @dev Response functions are only callable by the KMS Connectors
interface IDecryptionManager {
    /// @notice Emitted when an oracle decryption request is made
    /// @dev This event is meant to be listened by a user or relayer
    /// @param oracleDecryptionId The oracle decryption request's unique ID
    event OracleDecryptionId(uint256 indexed oracleDecryptionId);

    /// @notice Emitted when an oracle decryption request is made
    /// @dev This event is meant to be listened by the KMS Connectors
    /// @param keychainId The keychain's unique ID
    /// @param oracleDecryptionId The oracle decryption request's unique ID
    /// @param chainId The network's chain ID
    /// @param kmsVerifier The network's KMS Verifier address to consider
    /// @param acl The network's ACL address to consider
    /// @param ciphertextHandles The handles of the ciphertexts to decrypt
    event OracleDecryptionRequest(
        uint256 indexed keychainId,
        uint256 indexed oracleDecryptionId,
        uint256 indexed chainId,
        address kmsVerifier,
        address acl,
        uint256[] ciphertextHandles
    );

    /// @notice Emitted when an oracle decryption response is made
    /// @dev This event is meant to be listened by a user or relayer
    /// @param oracleDecryptionId The oracle decryption request's unique ID associated with the response
    /// @param decryptedResult The decrypted result
    /// @param signatures The signatures of all the KMS Connectors that responded
    event OracleDecryptionResponse(uint256 indexed oracleDecryptionId, bytes decryptedResult, bytes[] signatures);

    /// @notice Emitted when a user decryption request is made
    /// @dev This event is meant to be listened by a user or relayer
    /// @param userDecryptionId The user decryption request's unique ID
    event UserDecryptionId(uint256 indexed userDecryptionId);

    /// @notice Emitted when a user decryption request is made
    /// @dev This event is meant to be listened by the KMS Connectors
    /// @param keychainId The keychain's unique ID
    /// @param userDecryptionId The user decryption request's unique ID
    /// @param chainId The network's chain ID
    /// @param kmsVerifier The network's KMS Verifier address to consider
    /// @param acl The network's ACL address to consider
    /// @param userAddress The user's address
    /// @param encryptionKey The encryption key
    /// @param proof The proof
    /// @param ciphertextHandle The handles of the ciphertexts to decrypt
    event UserDecryptionRequest(
        uint256 indexed keychainId,
        uint256 indexed userDecryptionId,
        uint256 indexed chainId,
        address kmsVerifier,
        address acl,
        address userAddress,
        bytes encryptionKey,
        string proof,
        uint256 ciphertextHandle
    );

    /// @notice Emitted when an oracle decryption response is made
    /// @dev This event is meant to be listened by a user or relayer
    /// @param userDecryptionId The user decryption request's unique ID associated with the response
    /// @param decryptedResult The decrypted result
    /// @param signatures The signatures of all the KMS Connectors that responded
    event UserDecryptionResponse(uint256 indexed userDecryptionId, bytes decryptedResult, bytes[] signatures);

    /// @notice Requests an oracle decryption
    /// @dev This function can be called by a user or relayer
    /// @param keychainId The keychain's unique ID
    /// @param chainId The network's chain ID
    /// @param kmsVerifier The network's KMS Verifier address to consider
    /// @param acl The network's ACL address to consider
    /// @param ciphertextHandles The handles of the ciphertexts to decrypt
    function oracleDecryptionRequest(
        uint256 keychainId,
        uint256 chainId,
        address kmsVerifier,
        address acl,
        uint256[] calldata ciphertextHandles
    ) external;

    /// @notice Responds to an oracle decryption request
    /// @dev This function can only be called by the KMS Connectors
    /// @param oracleDecryptionId The oracle decryption request's unique ID associated with the response
    /// @param decryptedResult The decrypted result
    /// @param signature The signature of the KMS Connector that responded
    function oracleDecryptionResponse(
        uint256 oracleDecryptionId,
        bytes calldata decryptedResult,
        bytes calldata signature
    ) external;

    /// @notice Requests a user decryption
    /// @dev This function can be called by a user or relayer
    /// @param keychainId The keychain's unique ID
    /// @param chainId The network's chain ID
    /// @param kmsVerifier The network's KMS Verifier address to consider
    /// @param acl The network's ACL address to consider
    /// @param userAddress The user's address
    /// @param encryptionKey The encryption key
    /// @param proof The proof
    /// @param ciphertextHandle The handles of the ciphertexts to decrypt
    function userDecryptionRequest(
        uint256 keychainId,
        uint256 chainId,
        address kmsVerifier,
        address acl,
        address userAddress,
        bytes calldata encryptionKey,
        string calldata proof,
        uint256 ciphertextHandle
    ) external;

    /// @notice Responds to a user decryption request
    /// @dev This function can only be called by the KMS Connectors
    /// @param userDecryptionId The user decryption request's unique ID associated with the response
    /// @param decryptedResult The decrypted result
    /// @param signature The signature of the KMS Connector that responded
    function userDecryptionResponse(
        uint256 userDecryptionId,
        bytes calldata decryptedResult,
        bytes calldata signature
    ) external;
}
