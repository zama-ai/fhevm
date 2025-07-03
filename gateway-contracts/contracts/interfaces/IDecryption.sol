// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../shared/Structs.sol";
import { ContextStatus } from "../shared/Enums.sol";

/**
 * @title Interface for the Decryption contract.
 * @notice The Decryption contract is responsible for decrypting ciphertext using a KMS.
 * Both user decryption and public decryption are handled.
 */
interface IDecryption {
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
     * @notice Emitted when an public decryption request is made.
     * @param decryptionId The decryption request ID.
     * @param contextId The ID of the KMS context used for the decryption.
     * @param snsCtMaterials The handles, key IDs and SNS ciphertexts to decrypt.
     */
    event PublicDecryptionRequest(
        uint256 indexed decryptionId,
        uint256 indexed contextId,
        SnsCiphertextMaterial[] snsCtMaterials
    );

    /**
     * @notice Emitted when an public decryption response is made.
     * @param decryptionId The decryption request ID associated with the response.
     * @param decryptedResult The decrypted result.
     * @param signatures The signatures of all the KMS connectors that responded.
     */
    event PublicDecryptionResponse(uint256 indexed decryptionId, bytes decryptedResult, bytes[] signatures);

    /**
     * @notice Emitted when a user decryption request is made.
     * @param decryptionId The decryption request ID.
     * @param snsCtMaterials The handles, key IDs and SNS ciphertexts to decrypt.
     * @param userAddress The user's address.
     * @param publicKey The user's public key for used reencryption.
     * @param contextId The ID of the KMS context used for the decryption.
     */
    event UserDecryptionRequest(
        uint256 indexed decryptionId,
        uint256 indexed contextId,
        SnsCiphertextMaterial[] snsCtMaterials,
        address userAddress,
        bytes publicKey
    );

    /**
     * @notice Emitted when an public decryption response is made.
     * @param decryptionId The decryption request ID associated with the response.
     * @param userDecryptedShares The list of decryption shares reencrypted with the user's public key.
     * @param signatures The signatures of all the KMS connectors that responded.
     */
    event UserDecryptionResponse(uint256 indexed decryptionId, bytes[] userDecryptedShares, bytes[] signatures);

    /// @notice Error indicating that the input list of handles is empty.
    error EmptyCtHandles();

    /**
     * @notice Error indicating that the context is not allowed for decryption.
     * @param decryptionId The ID of the decryption request.
     * @param contextId The ID of the KMS context.
     * @param contextStatus The status of the KMS context.
     */
    error InvalidKmsContextDecryption(uint256 decryptionId, uint256 contextId, ContextStatus contextStatus);

    /// @notice Error indicating that the input list of ctHandleContractPairs is empty.
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
    error ContractAddressesMaxLengthExceeded(uint8 maxLength, uint256 actualLength);

    /// @notice Error indicating that the durationDays of a user decryption request is 0.
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
     * @dev This will be removed in the future as multiple keyIds processing is implemented.
     * See https://github.com/zama-ai/fhevm-gateway/issues/104.
     */
    error DifferentKeyIdsNotAllowed(
        SnsCiphertextMaterial firstSnsCtMaterial,
        SnsCiphertextMaterial invalidSnsCtMaterial
    );

    /**
     * @notice Error indicating that the (public, user, delegated user) decryption is not done.
     * @param decryptionId The decryption request ID.
     */
    error DecryptionNotDone(uint256 decryptionId);

    /**
     * @notice Requests a public decryption.
     * @param ctHandles The handles of the ciphertexts to decrypt.
     */
    function publicDecryptionRequest(bytes32[] calldata ctHandles) external;

    /**
     * @notice Responds to a public decryption request.
     * @param decryptionId The decryption request ID associated with the response.
     * @param decryptedResult The decrypted result.
     * @param signature The signature of the KMS connector that responded.
     */
    function publicDecryptionResponse(
        uint256 decryptionId,
        bytes calldata decryptedResult,
        bytes calldata signature
    ) external;

    /**
     * @notice Requests a user decryption.
     * @param ctHandleContractPairs The ciphertexts to decrypt for associated contracts.
     * @param requestValidity The validity period of the user decryption request.
     * @param contractsChainId The chain ID of the given contract addresses figuring in the signed EIP-712 message.
     * @param contractAddresses The contract addresses figuring in the signed EIP-712 message.
     * @param userAddress The user's address.
     * @param publicKey The user's public key to reencrypt the decryption shares.
     * @param signature The EIP712 signature to verify.
     */
    function userDecryptionRequest(
        CtHandleContractPair[] calldata ctHandleContractPairs,
        RequestValidity calldata requestValidity,
        uint256 contractsChainId,
        address[] calldata contractAddresses,
        address userAddress,
        bytes calldata publicKey,
        bytes calldata signature
    ) external;

    /**
     * @notice Requests a delegated user decryption.
     * @param ctHandleContractPairs The ciphertexts to decrypt for associated contracts.
     * @param requestValidity The validity period of the user decryption request.
     * @param delegationAccounts The user's address and the delegated account address for the user decryption.
     * @param contractsChainId The chain ID of the given contract addresses figuring in the signed EIP-712 message.
     * @param contractAddresses The contract addresses figuring in the signed EIP-712 message.
     * @param publicKey The user's public key to reencrypt the decryption shares.
     * @param signature The EIP712 signature to verify.
     */
    function delegatedUserDecryptionRequest(
        CtHandleContractPair[] calldata ctHandleContractPairs,
        RequestValidity calldata requestValidity,
        DelegationAccounts calldata delegationAccounts,
        uint256 contractsChainId,
        address[] calldata contractAddresses,
        bytes calldata publicKey,
        bytes calldata signature
    ) external;

    /**
     * @notice Responds to a user decryption request.
     * @param decryptionId The decryption request ID associated with the response.
     * @param userDecryptedShare The partial decryption share reencrypted with the user's public key.
     * @param signature The signature of the KMS connector that responded.
     */
    function userDecryptionResponse(
        uint256 decryptionId,
        bytes calldata userDecryptedShare,
        bytes calldata signature
    ) external;

    /**
     * @notice Checks if handles are ready to be decrypted publicly.
     * @param ctHandles The ciphertext handles.
     */
    function checkPublicDecryptionReady(bytes32[] calldata ctHandles) external view;

    /**
     * @notice Checks if handles are ready to be decrypted by a user.
     * @param userAddress The user's address.
     * @param ctHandleContractPairs The ciphertext handles with associated contract addresses.
     */
    function checkUserDecryptionReady(
        address userAddress,
        CtHandleContractPair[] calldata ctHandleContractPairs
    ) external view;

    /**
     * @notice Checks if handles are ready to be decrypted by a delegated address.
     * @param contractsChainId The contract's chain ID.
     * @param delegationAccounts The delegator and delegated address.
     * @param ctHandleContractPairs The ciphertext handles with associated contract addresses.
     * @param contractAddresses The contract addresses.
     */
    function checkDelegatedUserDecryptionReady(
        uint256 contractsChainId,
        DelegationAccounts calldata delegationAccounts,
        CtHandleContractPair[] calldata ctHandleContractPairs,
        address[] calldata contractAddresses
    ) external view;

    /**
     * @notice Checks if a (public, user, delegated user) decryption is done.
     * @param decryptionId The decryption request ID.
     */
    function checkDecryptionDone(uint256 decryptionId) external view;

    /**
     * @notice Returns the versions of the Decryption contract in SemVer format.
     * @dev This is conventionally used for upgrade features.
     */
    function getVersion() external pure returns (string memory);
}
