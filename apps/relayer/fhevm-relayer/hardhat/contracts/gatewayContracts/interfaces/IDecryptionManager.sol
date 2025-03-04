// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "./IACLManager.sol";
import "../shared/Structs.sol";

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
        uint256 ctHandle;
        /// @notice The address of the contract
        address contractAddress;
    }

    /// @notice A struct that specifies the validity period of a request, starting at "startTimestamp"
    /// @notice and remaining valid for "durationDays".
    struct RequestValidity {
        /// @notice The start timestamp of the user decryption request
        uint256 startTimestamp;
        /// @notice The duration in days for the user decryption to be processed
        uint256 durationDays;
    }

    /// @notice Emitted when an public decryption request is made
    /// @dev This event is meant to be listened by the KMS Connectors
    /// @param publicDecryptionId The public decryption request's unique ID
    /// @param snsCtMaterials The handles, key IDs and SNS ciphertexts to decrypt
    event PublicDecryptionRequest(uint256 indexed publicDecryptionId, SnsCiphertextMaterial[] snsCtMaterials);

    /// @notice Emitted when an public decryption response is made
    /// @dev This event is meant to be listened by a user or relayer
    /// @param publicDecryptionId The public decryption request's unique ID associated with the response
    /// @param decryptedResult The decrypted result
    /// @param signatures The signatures of all the KMS Connectors that responded
    event PublicDecryptionResponse(uint256 indexed publicDecryptionId, bytes decryptedResult, bytes[] signatures);

    /// @notice Emitted when a user decryption request is made
    /// @dev This event is meant to be listened by the KMS Connectors
    /// @param userDecryptionId The user decryption request's unique ID
    /// @param snsCtMaterials The handles, key IDs and SNS ciphertexts to decrypt
    /// @param publicKey The user's public key for used reencryption
    event UserDecryptionRequest(
        uint256 indexed userDecryptionId,
        SnsCiphertextMaterial[] snsCtMaterials,
        bytes publicKey
    );

    /// @notice Emitted when an public decryption response is made
    /// @dev This event is meant to be listened by a user or relayer
    /// @param userDecryptionId The user decryption request's unique ID associated with the response
    /// @param reencryptedShares The list of decryption shares reencrypted with the user's public key
    /// @param signatures The signatures of all the KMS Connectors that responded
    event UserDecryptionResponse(uint256 indexed userDecryptionId, bytes[] reencryptedShares, bytes[] signatures);

    /// @notice Error indicating that the KMS Connector has already signed its public decryption response
    /// @param publicDecryptionId The public decryption request's unique ID associated with the response
    /// @param signer The address of the KMS Connector signer that has already signed
    error KmsSignerAlreadySigned(uint256 publicDecryptionId, address signer);

    /// @notice Error indicating that the given signature for the user decryption request is invalid
    /// @param signature The invalid signature
    error InvalidUserSignature(bytes signature);

    /// @notice Error indicating that the number of contract addresses exceeds the maximum allowed.
    error ContractAddressesMaxLengthExceeded(uint8 maxLength, uint256 actualLength);

    /// @notice Error indicating that the user decryption request has exceeded the maximum durationDays
    /// @param maxValue The maximum durationDays allowed
    /// @param actualValue The actual durationDays requested
    error MaxDurationDaysExceeded(uint256 maxValue, uint256 actualValue);

    /// @notice Error indicating that the contract address is not included in the contract addresses list
    /// @param contractAddress The contract address that is not in the list
    error ContractNotInContractAddresses(address contractAddress);

    /// @notice Error indicating that the key IDs in a given SNS ciphertext materials list are not the same
    /// @param keyId The key ID that is different
    /// @dev This will be removed in the future as multiple keyIds processing is implemented.
    /// @dev See https://github.com/zama-ai/gateway-l2/issues/104.
    error DifferentKeyIdsNotAllowed(uint256 keyId);

    /// @notice Requests an public decryption
    /// @dev This function can be called by a user or relayer
    /// @param ctHandles The handles of the ciphertexts to decrypt
    function publicDecryptionRequest(uint256[] calldata ctHandles) external;

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
    /// @param requestValidity The validity period of the user decryption request
    /// @param contractsChainId The chain ID of the given contract addresses
    /// @param contractAddresses The contract addresses found in the message
    /// @param userAddress The user's address
    /// @param publicKey The user's public key to reencrypt the decryption shares
    /// @param signature The EIP712 signature to verify
    function userDecryptionRequest(
        CtHandleContractPair[] calldata ctHandleContractPairs,
        RequestValidity calldata requestValidity,
        uint256 contractsChainId,
        address[] calldata contractAddresses,
        address userAddress,
        bytes calldata publicKey,
        bytes calldata signature
    ) external;

    /// @notice Responds to a user decryption request
    /// @dev This function can only be called by the KMS Connectors
    /// @param userDecryptionId The user decryption request's unique ID associated with the response
    /// @param reencryptedShare The partial decryption share reencrypted with the user's public key
    /// @param signature The signature of the KMS Connector that responded
    function userDecryptionResponse(
        uint256 userDecryptionId,
        bytes calldata reencryptedShare,
        bytes calldata signature
    ) external;

    /// @notice Returns whether a public decryption is done
    /// @param publicDecryptionId The public decryption request's unique ID
    function isPublicDecryptionDone(uint256 publicDecryptionId) external view returns (bool);

    /// @notice Returns whether a user decryption is done
    /// @param userDecryptionId The user decryption request's unique ID
    function isUserDecryptionDone(uint256 userDecryptionId) external view returns (bool);
}
