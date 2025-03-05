// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import { IDecryptionManager } from "./interfaces/IDecryptionManager.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";
import { EIP712 } from "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { ECDSA } from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "./interfaces/IHTTPZ.sol";
import "./interfaces/IACLManager.sol";
import "./interfaces/ICiphertextStorage.sol";

/// @title DecryptionManager contract
/// @dev See {IDecryptionManager}.
contract DecryptionManager is Ownable2Step, EIP712, IDecryptionManager {
    /// @notice The typed data structure for the EIP712 signature to validate in public decryption responses.
    /// @dev The name of this struct is not relevant for the signature validation, only the one defined
    /// @dev EIP712_PUBLIC_DECRYPT_TYPE is, but we keep it the same for clarity.
    struct PublicDecryptVerification {
        /// @notice The handles of the ciphertexts that have been decrypted.
        uint256[] ctHandles;
        /// @notice The decrypted result of the public decryption.
        bytes decryptedResult;
    }

    /// @notice The typed data structure for the EIP712 signature to validate in user decryption requests.
    /// @dev The name of this struct is not relevant for the signature validation, only the one defined
    /// @dev EIP712_USER_DECRYPT_REQUEST_TYPE is, but we keep it the same for clarity.
    struct UserDecryptRequestVerification {
        /// @notice The user's public key to be used for reencryption.
        bytes publicKey;
        /// @notice The contract addresses that verification is requested for.
        address[] contractAddresses;
        /// @notice The chain ID of the contract addresses.
        uint256 contractsChainId;
        /// @notice The start timestamp of the user decryption request.
        uint256 startTimestamp;
        /// @notice The duration in days of the user decryption request after the start timestamp.
        uint256 durationDays;
    }

    /// @notice The typed data structure for the EIP712 signature to validate in user decryption responses.
    /// @dev The name of this struct is not relevant for the signature validation, only the one defined
    /// @dev EIP712_USER_DECRYPT_RESPONSE_TYPE is, but we keep it the same for clarity.
    struct UserDecryptResponseVerification {
        /// @notice The user's public key used for the share reencryption.
        bytes publicKey;
        /// @notice The handles of the ciphertexts that have been decrypted.
        uint256[] ctHandles;
        /// @notice The partial decryption share reencrypted with the user's public key.
        bytes reencryptedShare;
    }

    /// @notice The publicKey and ctHandles from user decryption requests used for validations during responses.
    struct UserDecryptionPayload {
        /// @notice The user's public key to be used for reencryption.
        bytes publicKey;
        /// @notice The handles of the ciphertexts requested for a user decryption
        uint256[] ctHandles;
    }

    /// @notice The address of the HTTPZ contract for checking if a signer is valid
    IHTTPZ internal immutable _HTTPZ;

    /// @notice The address of the ACLManager contract for checking if a decryption requests are allowed
    IACLManager internal immutable _ACL_MANAGER;

    /// @notice The address of the Ciphertext Storage contract for getting ciphertext materials.
    ICiphertextStorage internal immutable _CIPHERTEXT_STORAGE;

    // TODO: Use a reference to the PaymentManager contract
    /// @notice The address of the Payment Manager contract for service fees, burn and distribution
    address internal immutable _PAYMENT_MANAGER;

    /// @notice The maximum number of duration days that can be requested for a user decryption.
    uint16 internal constant _MAX_USER_DECRYPT_DURATION_DAYS = 365;

    /// @notice The maximum number of contracts that can request for user decryption at once.
    uint8 internal constant _MAX_USER_DECRYPT_CONTRACT_ADDRESSES = 10;

    /// @notice Whether a public/user decryption has been signed (and verified) by a signer.
    mapping(uint256 decryptionId => mapping(address kmsSigner => bool alreadySigned)) internal alreadySigned;

    /// @notice Pending verified signatures for a public/user decryption.
    mapping(uint256 decryptionId => mapping(bytes32 digest => bytes[] verifiedSignatures)) internal verifiedSignatures;

    /// @notice The number of public/user decryptions requested, used to generate the publicDecryptionIds/userDecryptionIds.
    // TODO: This will be replaced during gateway-l2/issues/92
    uint256 internal decryptionCounter;

    // ----------------------------------------------------------------------------------------------
    // Public decryption state variables:
    // ----------------------------------------------------------------------------------------------

    /// @notice Handles of the ciphertexts requested for a public decryption
    mapping(uint256 publicDecryptionId => uint256[] ctHandles) internal publicCtHandles;

    /// @notice Whether a public decryption has been done
    mapping(uint256 publicDecryptionId => bool publicDecryptionDone) internal publicDecryptionDone;

    /// @notice The definition of the PublicDecryptVerification structure typed data.
    string public constant EIP712_PUBLIC_DECRYPT_TYPE =
        "PublicDecryptVerification(uint256[] ctHandles,bytes decryptedResult)";

    /// @notice The hash of the PublicDecryptVerification structure typed data definition used for signature validation.
    bytes32 public constant EIP712_PUBLIC_DECRYPT_TYPE_HASH = keccak256(bytes(EIP712_PUBLIC_DECRYPT_TYPE));

    // ----------------------------------------------------------------------------------------------
    // User decryption state variables:
    // ----------------------------------------------------------------------------------------------

    /// @notice The decryption payloads stored during user decryption requests.
    mapping(uint256 userDecryptionId => UserDecryptionPayload payload) internal userDecryptionPayloads;

    /// @notice Whether a user decryption has been done
    mapping(uint256 userDecryptionId => bool userDecryptionDone) internal userDecryptionDone;

    /// @notice The reencrypted shares received from user decryption responses.
    mapping(uint256 userDecryptionId => bytes[] shares) internal reencryptedShares;

    /// @notice The definition of the UserDecryptRequestVerification structure typed data.
    string public constant EIP712_USER_DECRYPT_REQUEST_TYPE =
        "UserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,uint256 contractsChainId,"
        "uint256 startTimestamp,uint256 durationDays)";

    /// @notice The hash of the UserDecryptRequestVerification structure typed data definition
    /// @notice used for signature validation.
    bytes32 public constant EIP712_USER_DECRYPT_REQUEST_TYPE_HASH = keccak256(bytes(EIP712_USER_DECRYPT_REQUEST_TYPE));

    /// @notice The definition of the UserDecryptResponseVerification structure typed data.
    string public constant EIP712_USER_DECRYPT_RESPONSE_TYPE =
        "UserDecryptResponseVerification(bytes publicKey,uint256[] ctHandles,bytes reencryptedShare)";

    /// @notice The hash of the UserDecryptResponseVerification structure typed data definition
    /// @notice used for signature validation.
    bytes32 public constant EIP712_USER_DECRYPT_RESPONSE_TYPE_HASH =
        keccak256(bytes(EIP712_USER_DECRYPT_RESPONSE_TYPE));

    string private constant CONTRACT_NAME = "DecryptionManager";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// @dev Contract name and version for EIP712 signature validation are defined here
    constructor(
        IHTTPZ httpz,
        IACLManager aclManager,
        ICiphertextStorage ciphertextStorage,
        address paymentManager
    ) Ownable(msg.sender) EIP712(CONTRACT_NAME, "1") {
        _HTTPZ = httpz;
        _ACL_MANAGER = aclManager;
        _CIPHERTEXT_STORAGE = ciphertextStorage;
        _PAYMENT_MANAGER = paymentManager;
    }

    /// @dev See {IDecryptionManager-publicDecryptionRequest}.
    function publicDecryptionRequest(uint256[] calldata ctHandles) public virtual {
        /// @dev Check that the public decryption is allowed for the given ctHandles.
        _ACL_MANAGER.checkPublicDecryptAllowed(ctHandles);

        /// @dev Fetch the SNS ciphertexts from the ciphertext storage
        /// @dev This call is reverted if any of the ciphertexts are not found in the storage, but
        /// @dev this should not happen for now as a ciphertext cannot be allowed for decryption
        /// @dev without being added to the storage first (and we currently have no ways of deleting
        /// @dev a ciphertext from the storage).
        SnsCiphertextMaterial[] memory snsCtMaterials = _CIPHERTEXT_STORAGE.getSnsCiphertextMaterials(ctHandles);

        /// @dev Check that received snsCtMaterials have the same keyId.
        /// @dev This will be removed in the future as multiple keyIds processing is implemented.
        /// @dev See https://github.com/zama-ai/gateway-l2/issues/104.
        _checkCtMaterialKeyIds(snsCtMaterials);

        decryptionCounter++;
        uint256 publicDecryptionId = decryptionCounter;

        /// @dev The handles are used during response calls for the EIP712 signature validation.
        publicCtHandles[publicDecryptionId] = ctHandles;

        // TODO: Implement sending service fees to PaymentManager contract

        emit PublicDecryptionRequest(publicDecryptionId, snsCtMaterials);
    }

    /// @dev See {IDecryptionManager-publicDecryptionResponse}.
    function publicDecryptionResponse(
        uint256 publicDecryptionId,
        bytes calldata decryptedResult,
        bytes calldata signature
    ) public virtual {
        /// @dev Initialize the PublicDecryptVerification structure for the signature validation.
        PublicDecryptVerification memory publicDecryptVerification = PublicDecryptVerification(
            publicCtHandles[publicDecryptionId],
            decryptedResult
        );

        /// @dev Compute the digest of the PublicDecryptVerification structure.
        bytes32 digest = _hashPublicDecryptVerification(publicDecryptVerification);

        /// @dev Recover the signer address from the signature and validate that is a KMS node that
        /// @dev has not already signed.
        _validateEIP712Signature(publicDecryptionId, digest, signature);

        verifiedSignatures[publicDecryptionId][digest].push(signature);

        bytes[] memory verifiedSignaturesArray = verifiedSignatures[publicDecryptionId][digest];

        /// @dev Only send the event if consensus has not been reached in a previous response call
        /// @dev and the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (!publicDecryptionDone[publicDecryptionId] && _isConsensusReachedPublic(verifiedSignaturesArray.length)) {
            publicDecryptionDone[publicDecryptionId] = true;

            // TODO: Implement sending service fees to PaymentManager contract

            emit PublicDecryptionResponse(publicDecryptionId, decryptedResult, verifiedSignaturesArray);
        }
    }

    /// @dev See {IDecryptionManager-userDecryptionRequest}.
    function userDecryptionRequest(
        CtHandleContractPair[] calldata ctHandleContractPairs,
        RequestValidity calldata requestValidity,
        uint256 contractsChainId,
        address[] calldata contractAddresses,
        address userAddress,
        bytes calldata publicKey,
        bytes calldata signature
    ) external virtual {
        /// @dev Check the given number of contractAddresses does not exceed the maximum allowed.
        if (contractAddresses.length > _MAX_USER_DECRYPT_CONTRACT_ADDRESSES) {
            revert ContractAddressesMaxLengthExceeded(_MAX_USER_DECRYPT_CONTRACT_ADDRESSES, contractAddresses.length);
        }

        /// @dev Check the given durationDays does not exceed the maximum allowed.
        if (requestValidity.durationDays > _MAX_USER_DECRYPT_DURATION_DAYS) {
            revert MaxDurationDaysExceeded(_MAX_USER_DECRYPT_DURATION_DAYS, requestValidity.durationDays);
        }

        /// @dev Check that the user decryption is allowed for the given userAddress and ctHandleContractPairs.
        _ACL_MANAGER.checkUserDecryptAllowed(userAddress, ctHandleContractPairs);

        /// @dev Initialize the UserDecryptRequestVerification structure for the signature validation.
        UserDecryptRequestVerification memory userDecryptRequestVerification = UserDecryptRequestVerification(
            publicKey,
            contractAddresses,
            contractsChainId,
            requestValidity.startTimestamp,
            requestValidity.durationDays
        );

        /// @dev Validate the received EIP712 signature on the user decryption request.
        _validateUserDecryptRequestEIP712Signature(userDecryptRequestVerification, userAddress, signature);

        /// @dev Extract the ctHandles from the given ctHandleContractPairs.
        /// @dev We do not deduplicate handles if the same handle appears multiple times
        /// @dev for different contracts, it remains in the list as is. This ensures that
        /// @dev the ciphertext storage retrieval below returns all corresponding materials.
        uint256[] memory ctHandles = _extractCtHandles(ctHandleContractPairs, contractAddresses);

        /// @dev Fetch the ciphertexts from the ciphertext storage
        /// @dev This call is reverted if any of the ciphertexts are not found in the storage, but
        /// @dev this should not happen for now as a ciphertext cannot be allowed for decryption
        /// @dev without being added to the storage first (and we currently have no ways of deleting
        /// @dev a ciphertext from the storage).
        SnsCiphertextMaterial[] memory snsCtMaterials = _CIPHERTEXT_STORAGE.getSnsCiphertextMaterials(ctHandles);

        /// @dev Check that received snsCtMaterials have the same keyId.
        /// @dev This will be removed in the future as multiple keyIds processing is implemented.
        /// @dev See https://github.com/zama-ai/gateway-l2/issues/104.
        _checkCtMaterialKeyIds(snsCtMaterials);

        // TODO: This counter will be replaced during gateway-l2/issues/92
        decryptionCounter++;
        uint256 userDecryptionId = decryptionCounter;

        /// @dev The publicKey and ctHandles are used during response calls for the EIP712 signature validation.
        userDecryptionPayloads[userDecryptionId] = UserDecryptionPayload(publicKey, ctHandles);

        // TODO: Implement sending service fees to PaymentManager contract

        emit UserDecryptionRequest(userDecryptionId, snsCtMaterials, publicKey);
    }

    /// @dev See {IDecryptionManager-userDecryptionResponse}.
    function userDecryptionResponse(
        uint256 userDecryptionId,
        bytes calldata reencryptedShare,
        bytes calldata signature
    ) external virtual {
        UserDecryptionPayload memory userDecryptionPayload = userDecryptionPayloads[userDecryptionId];

        /// @dev Initialize the UserDecryptResponseVerification structure for the signature validation.
        UserDecryptResponseVerification memory userDecryptResponseVerification = UserDecryptResponseVerification(
            userDecryptionPayload.publicKey,
            userDecryptionPayload.ctHandles,
            reencryptedShare
        );

        /// @dev Compute the digest of the UserDecryptResponseVerification structure.
        bytes32 digest = _hashUserDecryptResponseVerification(userDecryptResponseVerification);

        /// @dev Recover the signer address from the signature and validate that is a KMS node that
        /// @dev has not already signed.
        _validateEIP712Signature(userDecryptionId, digest, signature);

        bytes[] storage userVerifiedSignatures = verifiedSignatures[userDecryptionId][digest];
        userVerifiedSignatures.push(signature);

        /// @dev Store the reencrypted share for the user decryption response.
        reencryptedShares[userDecryptionId].push(reencryptedShare);

        /// @dev Only send the event if consensus has not been reached in a previous response call
        /// @dev and the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (!userDecryptionDone[userDecryptionId] && _isConsensusReachedUser(userVerifiedSignatures.length)) {
            userDecryptionDone[userDecryptionId] = true;

            // TODO: Implement sending service fees to PaymentManager contract

            emit UserDecryptionResponse(userDecryptionId, reencryptedShares[userDecryptionId], userVerifiedSignatures);
        }
    }

    /// @dev See {IDecryptionManager-isPublicDecryptionDone}.
    function isPublicDecryptionDone(uint256 publicDecryptionId) public view virtual returns (bool) {
        return publicDecryptionDone[publicDecryptionId];
    }

    /// @dev See {IDecryptionManager-isUserDecryptionDone}.
    function isUserDecryptionDone(uint256 userDecryptionId) public view virtual returns (bool) {
        return userDecryptionDone[userDecryptionId];
    }

    /// @notice Returns the versions of the DecryptionManager contract in SemVer format.
    /// @dev This is conventionally used for upgrade features.
    function getVersion() external pure virtual returns (string memory) {
        return
            string(
                abi.encodePacked(
                    CONTRACT_NAME,
                    " v",
                    Strings.toString(MAJOR_VERSION),
                    ".",
                    Strings.toString(MINOR_VERSION),
                    ".",
                    Strings.toString(PATCH_VERSION)
                )
            );
    }

    /// @notice Validates the EIP712 signature for a given public/user decryption
    /// @dev This function checks that the signer address is a KMS Connector.
    /// @dev It also checks that the signer has not already signed the public/user decryption.
    /// @param decryptionId The ID of the public or user decryption
    /// @param digest The hash of the PublicDecryptVerification/UserDecryptResponseVerification structure
    /// @param signature The signature to be validated
    function _validateEIP712Signature(uint256 decryptionId, bytes32 digest, bytes calldata signature) internal virtual {
        address signer = ECDSA.recover(digest, signature);

        /// @dev Check that the signer is a KMS node
        _HTTPZ.checkIsKmsNode(signer);

        if (alreadySigned[decryptionId][signer]) {
            revert KmsSignerAlreadySigned(decryptionId, signer);
        }

        alreadySigned[decryptionId][signer] = true;
    }

    /// @notice Validates the EIP712 signature for a given user decryption request
    /// @dev This function checks that the signer address is the same as the user address.
    /// @param userDecryptRequestVerification The signed UserDecryptRequestVerification structure
    /// @param signature The signature to be validated
    function _validateUserDecryptRequestEIP712Signature(
        UserDecryptRequestVerification memory userDecryptRequestVerification,
        address userAddress,
        bytes calldata signature
    ) internal view virtual {
        bytes32 digest = _hashUserDecryptRequestVerification(userDecryptRequestVerification);
        address signer = ECDSA.recover(digest, signature);
        if (signer != userAddress) {
            revert InvalidUserSignature(signature);
        }
    }

    /// @notice Computes the hash of a given PublicDecryptVerification structured data
    /// @param publicDecryptVerification The PublicDecryptVerification structure
    /// @return The hash of the PublicDecryptVerification structure
    function _hashPublicDecryptVerification(
        PublicDecryptVerification memory publicDecryptVerification
    ) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        EIP712_PUBLIC_DECRYPT_TYPE_HASH,
                        keccak256(abi.encodePacked(publicDecryptVerification.ctHandles)),
                        keccak256(publicDecryptVerification.decryptedResult)
                    )
                )
            );
    }

    /// @notice Computes the hash of a given UserDecryptRequestVerification structured data.
    /// @param userDecryptRequestVerification The UserDecryptRequestVerification structure to hash.
    /// @return The hash of the UserDecryptRequestVerification structure.
    function _hashUserDecryptRequestVerification(
        UserDecryptRequestVerification memory userDecryptRequestVerification
    ) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        EIP712_USER_DECRYPT_REQUEST_TYPE_HASH,
                        keccak256(userDecryptRequestVerification.publicKey),
                        keccak256(abi.encodePacked(userDecryptRequestVerification.contractAddresses)),
                        userDecryptRequestVerification.contractsChainId,
                        userDecryptRequestVerification.startTimestamp,
                        userDecryptRequestVerification.durationDays
                    )
                )
            );
    }

    /// @notice Computes the hash of a given UserDecryptResponseVerification structured data.
    /// @param userDecryptResponseVerification The UserDecryptResponseVerification structure to hash.
    /// @return The hash of the UserDecryptResponseVerification structure.
    function _hashUserDecryptResponseVerification(
        UserDecryptResponseVerification memory userDecryptResponseVerification
    ) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        EIP712_USER_DECRYPT_RESPONSE_TYPE_HASH,
                        keccak256(userDecryptResponseVerification.publicKey),
                        keccak256(abi.encodePacked(userDecryptResponseVerification.ctHandles)),
                        keccak256(userDecryptResponseVerification.reencryptedShare)
                    )
                )
            );
    }

    /// @notice Checks if the consensus is reached among the KMS nodes.
    /// @dev This function calls the HTTPZ contract to retrieve the consensus threshold.
    /// @param kmsCounter The number of KMS nodes that agreed
    /// @return Whether the consensus is reached
    function _isConsensusReachedPublic(uint256 kmsCounter) internal view virtual returns (bool) {
        uint256 consensusThreshold = _HTTPZ.getKmsMajorityThreshold();
        return kmsCounter >= consensusThreshold;
    }

    /// @notice Checks if the consensus for user decryption is reached among the KMS signers.
    /// @param verifiedSignaturesCount The number of signatures that have been verified for a user decryption.
    /// @return Whether the consensus is reached.
    function _isConsensusReachedUser(uint256 verifiedSignaturesCount) internal view virtual returns (bool) {
        uint256 consensusThreshold = _HTTPZ.getKmsReconstructionThreshold();
        return verifiedSignaturesCount >= consensusThreshold;
    }

    /// @notice Extracts the ctHandles from the given ctHandleContractPairs.
    /// @dev This checks that the contracts from ctHandleContractPairs are included in the given contractAddresses.
    function _extractCtHandles(
        CtHandleContractPair[] calldata ctHandleContractPairs,
        address[] calldata contractAddresses
    ) internal pure returns (uint256[] memory) {
        uint256[] memory ctHandles = new uint256[](ctHandleContractPairs.length);

        for (uint256 i = 0; i < ctHandleContractPairs.length; i++) {
            /// @dev Check the contractAddress from ctHandleContractPairs is included in the given contractAddresses.
            if (!_containsContractAddress(contractAddresses, ctHandleContractPairs[i].contractAddress)) {
                revert ContractNotInContractAddresses(ctHandleContractPairs[i].contractAddress);
            }
            ctHandles[i] = ctHandleContractPairs[i].ctHandle;
        }
        return ctHandles;
    }

    /// @notice Checks if a given contractAddress is included in the contractAddresses list.
    function _containsContractAddress(
        address[] memory contractAddresses,
        address contractAddress
    ) internal pure returns (bool) {
        for (uint256 i = 0; i < contractAddresses.length; i++) {
            if (contractAddresses[i] == contractAddress) {
                return true;
            }
        }
        return false;
    }

    /// @notice Checks that all SNS ciphertext materials have the same keyId.
    function _checkCtMaterialKeyIds(SnsCiphertextMaterial[] memory snsCtMaterials) internal pure {
        if (snsCtMaterials.length <= 1) return;

        uint256 firstKeyId = snsCtMaterials[0].keyId;
        for (uint256 i = 1; i < snsCtMaterials.length; i++) {
            if (snsCtMaterials[i].keyId != firstKeyId) {
                revert DifferentKeyIdsNotAllowed(snsCtMaterials[i].keyId);
            }
        }
    }
}
