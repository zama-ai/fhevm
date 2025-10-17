// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../shared/Structs.sol";

/**
 * @title Interface for the Decryption contract.
 * @notice The Decryption contract is responsible for decrypting ciphertext using a KMS.
 * Both user decryption and public decryption are handled.
 */
interface IDecryption {
    /**
     * @notice A struct that specifies information about the contracts to be used in the decryption.
     */
    struct ContractsInfo {
        /// @notice The chain ID of the contracts to be used in the decryption
        uint256 chainId;
        /// @notice The list of contract addresses to be used in the decryption
        address[] addresses;
    }

    /**
     * @notice A struct that specifies the validity period of a request, starting at "startTimestamp"
     * and remaining valid for "durationDays".
     */
    struct RequestValidity {
        /**
         * @notice The start timestamp of the user decryption request. This is a regular Unix timestamp
         * in seconds representing the time elapsed since midnight, January 1, 1970 Universal Coordinated Time (UTC).
         */
        uint256 startTimestamp;
        /// @notice The duration in days for the user decryption to be processed.
        uint256 durationDays;
    }

    /**
     * @notice A struct that contains the delegator and the delegate addresses for a delegated user decryption.
     */
    struct DelegationAccounts {
        /// @notice The address of the account that delegates access to its handles.
        address delegatorAddress;
        /// @notice The address of the account that receives the delegation.
        address delegateAddress;
    }

    /**
     * @notice Emitted when an public decryption request is made.
     * @param decryptionId The decryption request ID.
     * @param snsCtMaterials The handles, key IDs and SNS ciphertexts to decrypt.
     * @param extraData Generic bytes metadata for versioned payloads. First byte is for the version.
     */
    event PublicDecryptionRequest(
        uint256 indexed decryptionId,
        SnsCiphertextMaterial[] snsCtMaterials,
        bytes extraData
    );

    /**
     * @notice Emitted when an public decryption response is made.
     * @param decryptionId The decryption request ID associated with the response.
     * @param decryptedResult The decrypted result.
     * @param signatures The signatures of all the KMS connectors that responded.
     * @param extraData Generic bytes metadata for versioned payloads. First byte is for the version.
     */
    event PublicDecryptionResponse(
        uint256 indexed decryptionId,
        bytes decryptedResult,
        bytes[] signatures,
        bytes extraData
    );

    /**
     * @notice Emitted when a user decryption request is made.
     * @param decryptionId The decryption request ID.
     * @param snsCtMaterials The handles, key IDs and SNS ciphertexts to decrypt.
     * @param userAddress The user's address.
     * @param publicKey The user's public key for used reencryption.
     * @param extraData Generic bytes metadata for versioned payloads. First byte is for the version.
     */
    event UserDecryptionRequest(
        uint256 indexed decryptionId,
        SnsCiphertextMaterial[] snsCtMaterials,
        address userAddress,
        bytes publicKey,
        bytes extraData
    );

    /**
     * @notice Emitted when an public decryption response is made.
     * @param decryptionId The decryption request ID associated with the response.
     * @param indexShare The index of the share associated with the decryption.
     * @param userDecryptedShare The decryption share reencrypted with the user's public key.
     * @param signature The signature of the KMS connector that responded.
     * @param extraData Generic bytes metadata for versioned payloads. First byte is for the version.
     */
    event UserDecryptionResponse(
        uint256 indexed decryptionId,
        uint256 indexShare,
        bytes userDecryptedShare,
        bytes signature,
        bytes extraData
    );

    /**
     * @notice Emitted when the number of user decryption response received reaches the threshold.
     * @param decryptionId The decryption request ID.
     */
    event UserDecryptionResponseThresholdReached(uint256 indexed decryptionId);

    /**
     * @notice Error indicating that the input list of handles is empty.
     */
    error EmptyCtHandles();

    /**
     * @notice Error indicating that the input list of ctHandleContractPairs is empty.
     */
    error EmptyCtHandleContractPairs();

    /**
     * @notice Error indicating that the total bit size of the decryption request exceeds
     * the maximum allowed.
     * @param maxBitSize The maximum allowed bit size.
     * @param totalBitSize The total bit size of the decryption request.
     */
    error MaxDecryptionRequestBitSizeExceeded(uint256 maxBitSize, uint256 totalBitSize);

    /**
     * @notice Error indicating that a KMS node has already signed the decryption response.
     * @param decryptionId The decryption request ID.
     * @param signer The signer address of the KMS node that has already signed.
     */
    error KmsNodeAlreadySigned(uint256 decryptionId, address signer);

    /**
     * @notice Error indicating that the given signature for the user decryption request is invalid.
     * @param signature The invalid signature.
     */
    error InvalidUserSignature(bytes signature);

    /**
     * @notice Error indicating that the list of contract addresses is empty.
     */
    error EmptyContractAddresses();

    /**
     * @notice Error indicating that the number of contract addresses exceeds the maximum allowed.
     * @param maxLength The maximum number of contract addresses allowed.
     * @param actualLength The actual number of contract addresses provided.
     */
    error ContractAddressesMaxLengthExceeded(uint256 maxLength, uint256 actualLength);

    /**
     * @notice Error indicating that the durationDays of a user decryption request is 0.
     */
    error InvalidNullDurationDays();

    /**
     * @notice Error indicating that the durationDays of a user decryption request exceeds
     * the maximum allowed.
     * @param maxValue The maximum durationDays allowed.
     * @param actualValue The actual durationDays requested.
     */
    error MaxDurationDaysExceeded(uint256 maxValue, uint256 actualValue);

    /**
     * @notice Error indicating that the start timestamp of a user decryption request has been set in the future.
     * @param currentTimestamp The block timestamp at which the user decryption request was made.
     * @param startTimestamp The start timestamp of the user decryption request.
     */
    error StartTimestampInFuture(uint256 currentTimestamp, uint256 startTimestamp);

    /**
     * @notice Error indicating that the user decryption request has expired.
     * @param currentTimestamp The block timestamp at which the user decryption request was made.
     * @param requestValidity The validity period of the user decryption request.
     */
    error UserDecryptionRequestExpired(uint256 currentTimestamp, RequestValidity requestValidity);

    /**
     * @notice Error indicating that the user address is included in the contract addresses list.
     * @param userAddress The user address that is included in the list.
     * @param contractAddresses The list of expected contract addresses.
     */
    error UserAddressInContractAddresses(address userAddress, address[] contractAddresses);

    /**
     * @notice Error indicating that the delegator address is included in the contract addresses list.
     * @param delegatorAddress The delegator address that is included in the list.
     * @param contractAddresses The list of expected contract addresses.
     */
    error DelegatorAddressInContractAddresses(address delegatorAddress, address[] contractAddresses);

    /**
     * @notice Error indicating that the contract address is not included in the contract addresses list.
     * @param contractAddress The contract address that is not in the list.
     * @param contractAddresses The list of expected contract addresses.
     */
    error ContractNotInContractAddresses(address contractAddress, address[] contractAddresses);

    /**
     * @notice Error indicating that the key IDs in a given SNS ciphertext materials list are not the same.
     * @param firstSnsCtMaterial The first SNS ciphertext material in the list with the expected key ID.
     * @param invalidSnsCtMaterial The SNS ciphertext material found with a different key ID.
     * @dev This should be removed once batched decryption requests with different keys is support by the KMS
     * See https://github.com/zama-ai/fhevm-internal/issues/376
     */
    error DifferentKeyIdsNotAllowed(
        SnsCiphertextMaterial firstSnsCtMaterial,
        SnsCiphertextMaterial invalidSnsCtMaterial
    );

    /**
     * @notice Error indicating that the (public, user, delegated user) decryption is not requested yet.
     * @param decryptionId The decryption request ID.
     */
    error DecryptionNotRequested(uint256 decryptionId);

    /**
     * @notice Requests a public decryption.
     * @param ctHandles The handles of the ciphertexts to decrypt.
     * @param extraData Generic bytes metadata for versioned payloads. First byte is for the version.
     */
    function publicDecryptionRequest(bytes32[] calldata ctHandles, bytes calldata extraData) external;

    /**
     * @notice Responds to a public decryption request.
     * @param decryptionId The decryption request ID associated with the response.
     * @param decryptedResult The decrypted result.
     * @param signature The signature of the KMS connector that responded.
     * @param extraData Generic bytes metadata for versioned payloads. First byte is for the version.
     */
    function publicDecryptionResponse(
        uint256 decryptionId,
        bytes calldata decryptedResult,
        bytes calldata signature,
        bytes calldata extraData
    ) external;

    /**
     * @notice Requests a user decryption.
     * @param ctHandleContractPairs The ciphertexts to decrypt for associated contracts.
     * @param requestValidity The validity period of the user decryption request.
     * @param contractsInfo The contracts' information (chain ID, addresses).
     * @param userAddress The user's address.
     * @param publicKey The user's public key to reencrypt the decryption shares.
     * @param signature The EIP712 signature to verify.
     * @param extraData Generic bytes metadata for versioned payloads. First byte is for the version.
     */
    function userDecryptionRequest(
        CtHandleContractPair[] calldata ctHandleContractPairs,
        RequestValidity calldata requestValidity,
        ContractsInfo calldata contractsInfo,
        address userAddress,
        bytes calldata publicKey,
        bytes calldata signature,
        bytes calldata extraData
    ) external;

    /**
     * @notice Requests a delegated user decryption.
     * @param ctHandleContractPairs The ciphertexts to decrypt for associated contracts.
     * @param requestValidity The validity period of the user decryption request.
     * @param delegationAccounts The user's address and the delegated account address for the user decryption.
     * @param contractsInfo The contracts' information (chain ID, addresses).
     * @param publicKey The user's public key to reencrypt the decryption shares.
     * @param signature The EIP712 signature to verify.
     * @param extraData Generic bytes metadata for versioned payloads. First byte is for the version.
     */
    function delegatedUserDecryptionRequest(
        CtHandleContractPair[] calldata ctHandleContractPairs,
        RequestValidity calldata requestValidity,
        DelegationAccounts calldata delegationAccounts,
        ContractsInfo calldata contractsInfo,
        bytes calldata publicKey,
        bytes calldata signature,
        bytes calldata extraData
    ) external;

    /**
     * @notice Responds to a user decryption request.
     * @param decryptionId The decryption request ID associated with the response.
     * @param userDecryptedShare The partial decryption share reencrypted with the user's public key.
     * @param signature The signature of the KMS connector that responded.
     * @param extraData Generic bytes metadata for versioned payloads. First byte is for the version.
     */
    function userDecryptionResponse(
        uint256 decryptionId,
        bytes calldata userDecryptedShare,
        bytes calldata signature,
        bytes calldata extraData
    ) external;

    /**
     * @notice Indicates if handles are ready to be decrypted publicly.
     * @param ctHandles The ciphertext handles.
     * @param extraData Generic bytes metadata for versioned payloads. First byte is for the version.
     */
    function isPublicDecryptionReady(
        bytes32[] calldata ctHandles,
        bytes calldata extraData
    ) external view returns (bool);

    /**
     * @notice Indicates if handles are ready to be decrypted by a user.
     * @param userAddress The user's address.
     * @param ctHandleContractPairs The ciphertext handles with associated contract addresses.
     * @param extraData Generic bytes metadata for versioned payloads. First byte is for the version.
     */
    function isUserDecryptionReady(
        address userAddress,
        CtHandleContractPair[] calldata ctHandleContractPairs,
        bytes calldata extraData
    ) external view returns (bool);

    /**
     * @notice Indicates if a (public, user, delegated user) decryption is done.
     * @param decryptionId The decryption request ID.
     */
    function isDecryptionDone(uint256 decryptionId) external view returns (bool);

    /**
     * @notice Returns the KMS transaction sender addresses that were involved in the consensus for a decryption request.
     * @param decryptionId The decryption request ID.
     */
    function getDecryptionConsensusTxSenders(uint256 decryptionId) external view returns (address[] memory);

    /**
     * @notice Returns the versions of the Decryption contract in SemVer format.
     * @dev This is conventionally used for upgrade features.
     */
    function getVersion() external pure returns (string memory);
}
