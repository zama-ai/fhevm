// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

import "./IACLManager.sol";

/// @title An interface for the decryption manager
/// @notice The decryption manager is responsible for decrypting ciphertext using a KMS
/// @notice Both user decryption and public decryption are handled
/// @dev Request functions are callable by any user or the relayer
/// @dev Response functions are only callable by the KMS Connectors
interface IDecryptionManager {
    /// @notice A struct that contains a ciphertext handle and a contract address that is
    /// @notice expected to be allowed to decrypt this ciphertext
    struct CtHandleContractPair {
        /// @notice The handle of the ciphertext
        uint256 ciphertextHandle;
        /// @notice The address of the contract
        address contractAddress;
    }

    /// @notice Emitted when an public decryption request is made
    /// @dev This event is meant to be listened by the KMS Connectors
    /// @param publicDecryptionId The public decryption request's unique ID
    /// @param ctHandleCiphertext128Pairs The handles and 128-PBS ciphertexts of the ciphertexts to decrypt
    event PublicDecryptionRequest(
        uint256 indexed publicDecryptionId,
        IACLManager.CtHandleCiphertext128Pair[] ctHandleCiphertext128Pairs
    );

    /// @notice Emitted when an public decryption response is made
    /// @dev This event is meant to be listened by a user or relayer
    /// @param publicDecryptionId The public decryption request's unique ID associated with the response
    /// @param decryptedResult The decrypted result
    /// @param signatures The signatures of all the KMS Connectors that responded
    event PublicDecryptionResponse(uint256 indexed publicDecryptionId, bytes decryptedResult, bytes[] signatures);

    /// @notice Emitted when a user decryption request is made
    /// @dev This event is meant to be listened by the KMS Connectors
    /// @param userDecryptionId The user decryption request's unique ID
    /// @param ctHandleContractPairs The ciphertexts to decrypt for associated contracts
    /// @param userAddress The user's address
    event UserDecryptionRequest(
        uint256 indexed userDecryptionId,
        CtHandleContractPair[] ctHandleContractPairs,
        address userAddress
    );

    /// @notice Emitted when an public decryption response is made
    /// @dev This event is meant to be listened by a user or relayer
    /// @param userDecryptionId The user decryption request's unique ID associated with the response
    /// @param decryptedResult The decrypted result
    /// @param signatures The signatures of all the KMS Connectors that responded
    event UserDecryptionResponse(uint256 indexed userDecryptionId, bytes decryptedResult, bytes[] signatures);

    /// @notice Error indicating that the KMS Connector is not a valid signer
    /// @param invalidSigner The address of the invalid signer
    error InvalidKmsSigner(address invalidSigner);

    /// @notice Error indicating that the KMS Connector has already signed its public decryption response
    /// @param publicDecryptionId The public decryption request's unique ID associated with the response
    /// @param signer The address of the KMS Connector signer that has already signed
    error KmsSignerAlreadySigned(uint256 publicDecryptionId, address signer);

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
    /// @param ctHandleContractPairs The ciphertexts to decrypt for associated contracts
    /// @param userAddress The user's address
    /// @param publicKey The public key
    /// @param eip712ChainId The chain ID of the EIP712 signature
    /// @param eip712Contracts The EIP712 contracts found in the message
    /// @param eip712Signature The EIP712 signature to verify
    function userDecryptionRequest(
        CtHandleContractPair[] calldata ctHandleContractPairs,
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

    /// @notice Returns whether a public decryption is done
    /// @param publicDecryptionId The public decryption request's unique ID
    function isPublicDecryptionDone(uint256 publicDecryptionId) external view returns (bool);
}
