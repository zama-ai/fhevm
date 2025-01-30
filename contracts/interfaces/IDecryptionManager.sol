// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

/// @title An interface for the decryption manager
/// @notice The decryption manager is responsible for decrypting ciphertext using a KMS
/// @notice Both user decryption and public decryption are handled
/// @dev Request functions are callable by any user or the relayer
/// @dev Response functions are only callable by the KMS Connectors
interface IDecryptionManager {
    /// @notice A struct that contains the ciphertext handle and its associated contract address
    /// @notice for which the ciphertext is allowed to be decrypted
    struct CiphertextContract {
        uint256 ciphertextHandle;
        address contractAddress;
    }

    /// @notice A struct that contains the ciphertext handle and the contract address
    /// @notice Emitted when an public decryption request is made
    /// @dev This event is meant to be listened by the KMS Connectors
    /// @param publicDecryptionId The public decryption request's unique ID
    /// @param ciphertextHandles The handles of the ciphertexts to decrypt
    event PublicDecryptionRequest(uint256 indexed publicDecryptionId, uint256[] ciphertextHandles);

    /// @notice Emitted when an public decryption response is made
    /// @dev This event is meant to be listened by a user or relayer
    /// @param publicDecryptionId The public decryption request's unique ID associated with the response
    /// @param decryptedResult The decrypted result
    /// @param signatures The signatures of all the KMS Connectors that responded
    event PublicDecryptionResponse(uint256 indexed publicDecryptionId, bytes decryptedResult, bytes[] signatures);

    /// @notice Emitted when a user decryption request is made
    /// @dev This event is meant to be listened by the KMS Connectors
    /// @param userDecryptionId The user decryption request's unique ID
    /// @param ciphertextContracts The ciphertexts and their associated contract addresses to decrypt
    /// @param userAddress The user's address
    event UserDecryptionRequest(
        uint256 indexed userDecryptionId,
        CiphertextContract[] ciphertextContracts,
        address userAddress
    );

    /// @notice Emitted when an public decryption response is made
    /// @dev This event is meant to be listened by a user or relayer
    /// @param userDecryptionId The user decryption request's unique ID associated with the response
    /// @param decryptedResult The decrypted result
    /// @param signatures The signatures of all the KMS Connectors that responded
    event UserDecryptionResponse(uint256 indexed userDecryptionId, bytes decryptedResult, bytes[] signatures);

    /// @notice Requests an public decryption
    /// @dev This function can be called by a user or relayer
    /// @param ciphertextHandles The handles of the ciphertexts to decrypt
    function publicDecryptionRequest(uint256[] calldata ciphertextHandles) external;

    /// @notice Responds to an public decryption request
    /// @dev This function can only be called by the KMS Connectors
    /// @param publicDecryptionId The public decryption request's unique ID associated with the response
    /// @param decryptedResult The decrypted result
    /// @param signature The signature of the KMS Connector that responded
    function publicDecryptionResponse(
        uint256 publicDecryptionId,
        bytes calldata decryptedResult,
        bytes calldata signature
    ) external;

    /// @notice Requests a user decryption
    /// @dev This function can be called by a user or relayer
    /// @param ciphertextContracts The ciphertexts and their associated contract addresses to decrypt
    /// @param userAddress The user's address
    /// @param publicKey The public key
    /// @param eip712ChainId The chain ID of the EIP712 signature
    /// @param eip712Contracts The EIP712 contracts found in the message
    /// @param eip712Signature The EIP712 signature to verify
    function userDecryptionRequest(
        CiphertextContract[] calldata ciphertextContracts,
        address userAddress,
        bytes calldata publicKey,
        uint256 eip712ChainId,
        address[] calldata eip712Contracts,
        bytes calldata eip712Signature
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
