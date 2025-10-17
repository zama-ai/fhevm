// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import { IDecryption } from "./interfaces/IDecryption.sol";
import {
    ciphertextCommitsAddress,
    gatewayConfigAddress,
    multichainACLAddress
} from "../addresses/GatewayAddresses.sol";
import { ECDSA } from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import { EIP712Upgradeable } from "@openzeppelin/contracts-upgradeable/utils/cryptography/EIP712Upgradeable.sol";
import { MessageHashUtils } from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { IGatewayConfig } from "./interfaces/IGatewayConfig.sol";
import { IMultichainACL } from "./interfaces/IMultichainACL.sol";
import { ICiphertextCommits } from "./interfaces/ICiphertextCommits.sol";
import { UUPSUpgradeableEmptyProxy } from "./shared/UUPSUpgradeableEmptyProxy.sol";
import { GatewayConfigChecks } from "./shared/GatewayConfigChecks.sol";
import { MultichainACLChecks } from "./shared/MultichainACLChecks.sol";
import { FheType } from "./shared/FheType.sol";
import { Pausable } from "./shared/Pausable.sol";
import { FHETypeBitSizes } from "./libraries/FHETypeBitSizes.sol";
import { HandleOps } from "./libraries/HandleOps.sol";
import { GatewayOwnable } from "./shared/GatewayOwnable.sol";
import { SnsCiphertextMaterial, CtHandleContractPair } from "./shared/Structs.sol";
import { PUBLIC_DECRYPT_COUNTER_BASE, USER_DECRYPT_COUNTER_BASE } from "./shared/KMSRequestCounters.sol";

/**
 * @title Decryption contract
 * @notice See {IDecryption}.
 */
contract Decryption is
    IDecryption,
    EIP712Upgradeable,
    UUPSUpgradeableEmptyProxy,
    GatewayOwnable,
    GatewayConfigChecks,
    MultichainACLChecks,
    Pausable
{
    /**
     * @notice The typed data structure for the EIP712 signature to validate in public decryption responses.
     * @dev The name of this struct is not relevant for the signature validation, only the one defined
     * EIP712_PUBLIC_DECRYPT_TYPE is, but we keep it the same for clarity.
     */
    struct PublicDecryptVerification {
        /// @notice The handles of the ciphertexts that have been decrypted.
        bytes32[] ctHandles;
        /// @notice The decrypted result of the public decryption.
        bytes decryptedResult;
        /// @notice Generic bytes metadata for versioned payloads. First byte is for the version.
        bytes extraData;
    }

    /**
     * @notice The typed data structure for the EIP712 signature to validate in user decryption requests.
     * @dev The name of this struct is not relevant for the signature validation, only the one defined
     * EIP712_USER_DECRYPT_REQUEST_TYPE is, but we keep it the same for clarity.
     */
    struct UserDecryptRequestVerification {
        /// @notice The user's public key to be used for reencryption.
        bytes publicKey;
        /// @notice The contract addresses that verification is requested for.
        address[] contractAddresses;
        /// @notice The start timestamp of the user decryption request.
        uint256 startTimestamp;
        /// @notice The duration in days of the user decryption request after the start timestamp.
        uint256 durationDays;
        /// @notice Generic bytes metadata for versioned payloads. First byte is for the version.
        bytes extraData;
    }

    /**
     * @notice The typed data structure for the EIP712 signature to validate in delegated user decryption requests.
     * @dev The name of this struct is not relevant for the signature validation, only the one defined as
     * EIP712_DELEGATED_USER_DECRYPT_REQUEST_TYPE is.
     */
    struct DelegatedUserDecryptRequestVerification {
        /// @notice The user's public key to be used for reencryption.
        bytes publicKey;
        /// @notice The contract addresses that verification is requested for.
        address[] contractAddresses;
        /// @notice The address of the account that delegates access to its handles.
        address delegatorAddress;
        /// @notice The start timestamp of the user decryption request.
        uint256 startTimestamp;
        /// @notice The duration in days of the user decryption request after the start timestamp.
        uint256 durationDays;
        /// @notice Generic bytes metadata for versioned payloads. First byte is for the version.
        bytes extraData;
    }

    /**
     * @notice The typed data structure for the EIP712 signature to validate in user decryption responses.
     * @dev The name of this struct is not relevant for the signature validation, only the one defined
     * EIP712_USER_DECRYPT_RESPONSE_TYPE is, but we keep it the same for clarity.
     */
    struct UserDecryptResponseVerification {
        /// @notice The user's public key used for the reencryption.
        bytes publicKey;
        /// @notice The handles of the ciphertexts that have been decrypted.
        bytes32[] ctHandles;
        /// @notice The partial decryption share reencrypted with the user's public key.
        bytes userDecryptedShare;
        /// @notice Generic bytes metadata for versioned payloads. First byte is for the version.
        bytes extraData;
    }

    /**
     * @notice The publicKey and ctHandles from user decryption requests used for validations during responses.
     */
    struct UserDecryptionPayload {
        /// @notice The user's public key to be used for reencryption.
        bytes publicKey;
        /// @notice The handles of the ciphertexts requested for a user decryption
        bytes32[] ctHandles;
    }

    /**
     * @notice The address of the GatewayConfig contract for checking if a signer is valid.
     */
    IGatewayConfig private constant GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);

    /**
     * @notice The address of the MultichainACL contract for checking if a decryption requests are allowed.
     */
    IMultichainACL private constant MULTICHAIN_ACL = IMultichainACL(multichainACLAddress);

    /**
     * @notice The address of the CiphertextCommits contract for getting ciphertext materials.
     */
    ICiphertextCommits private constant CIPHERTEXT_COMMITS = ICiphertextCommits(ciphertextCommitsAddress);

    /**
     * @notice The maximum number of duration days that can be requested for a user decryption.
     */
    uint16 internal constant MAX_USER_DECRYPT_DURATION_DAYS = 365;

    /**
     * @notice The maximum number of contracts that can request for user decryption at once.
     */
    uint8 internal constant MAX_USER_DECRYPT_CONTRACT_ADDRESSES = 10;

    /**
     * @notice The maximum number of bits that can be decrypted in a single public/user decryption request.
     */
    uint256 internal constant MAX_DECRYPTION_REQUEST_BITS = 2048;

    /**
     * @notice The hash of the EIP712Domain structure typed data definition.
     */
    bytes32 private constant DOMAIN_TYPE_HASH =
        keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");

    /**
     * @notice The definition of the PublicDecryptVerification structure typed data.
     */
    string private constant EIP712_PUBLIC_DECRYPT_TYPE =
        "PublicDecryptVerification(bytes32[] ctHandles,bytes decryptedResult,bytes extraData)";

    /**
     * @notice The hash of the PublicDecryptVerification structure typed data definition used for
     * signature validation in public decryption requests.
     */
    bytes32 private constant EIP712_PUBLIC_DECRYPT_TYPE_HASH = keccak256(bytes(EIP712_PUBLIC_DECRYPT_TYPE));

    /**
     * @notice The definition of the UserDecryptRequestVerification structure typed data.
     */
    string private constant EIP712_USER_DECRYPT_REQUEST_TYPE =
        "UserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,uint256 startTimestamp,"
        "uint256 durationDays,bytes extraData)";

    /**
     * @notice The hash of the UserDecryptRequestVerification structure typed data definition
     * used for signature validation in user decryption requests.
     */
    bytes32 private constant EIP712_USER_DECRYPT_REQUEST_TYPE_HASH = keccak256(bytes(EIP712_USER_DECRYPT_REQUEST_TYPE));

    /**
     * @notice The definition of the DelegatedUserDecryptRequestVerification structure typed data.
     */
    string private constant EIP712_DELEGATED_USER_DECRYPT_REQUEST_TYPE =
        "DelegatedUserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,address delegatorAddress,"
        "uint256 startTimestamp,uint256 durationDays,bytes extraData)";

    /**
     * @notice The hash of the DelegatedUserDecryptRequestVerification structure typed data definition
     * used for signature validation in delegated user decryption requests.
     */
    bytes32 private constant EIP712_DELEGATED_USER_DECRYPT_REQUEST_TYPE_HASH =
        keccak256(bytes(EIP712_DELEGATED_USER_DECRYPT_REQUEST_TYPE));

    /**
     * @notice The definition of the UserDecryptResponseVerification structure typed data.
     */
    string private constant EIP712_USER_DECRYPT_RESPONSE_TYPE =
        "UserDecryptResponseVerification(bytes publicKey,bytes32[] ctHandles,bytes userDecryptedShare,bytes extraData)";

    /**
     * @notice The hash of the UserDecryptResponseVerification structure typed data definition
     * used for signature validation in user decryption responses.
     */
    bytes32 private constant EIP712_USER_DECRYPT_RESPONSE_TYPE_HASH =
        keccak256(bytes(EIP712_USER_DECRYPT_RESPONSE_TYPE));

    /**
     * @dev The following constants are used for versioning the contract. They are made private
     * in order to force derived contracts to consider a different version. Note that
     * they can still define their own private constants with the same name.
     */
    string private constant CONTRACT_NAME = "Decryption";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /**
     * @dev Constant used for making sure the version number using in the `reinitializer` modifier is
     * identical between `initializeFromEmptyProxy` and the reinitializeVX` method
     * This constant does not represent the number of time a specific contract have been upgraded,
     * as a contract deployed from version VX will have a REINITIALIZER_VERSION > 2.
     */
    uint64 private constant REINITIALIZER_VERSION = 2;

    /**
     * @notice The contract's variable storage struct (@dev see ERC-7201)
     */
    /// @custom:storage-location erc7201:fhevm_gateway.storage.Decryption
    struct DecryptionStorage {
        // ----------------------------------------------------------------------------------------------
        // Common decryption state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice Whether a (public, user, delegated user) decryption is done
        mapping(uint256 decryptionId => bool decryptionDone) decryptionDone;
        // prettier-ignore
        /// @notice Whether KMS signer has already responded to a decryption request.
        mapping(uint256 decryptionId =>
            mapping(address kmsSigner => bool alreadyResponded))
                kmsNodeAlreadySigned;
        // ----------------------------------------------------------------------------------------------
        // Common decryption consensus state variables:
        // ----------------------------------------------------------------------------------------------
        // prettier-ignore
        /// @notice The KMS transaction senders involved in a consensus for a decryption response.
        mapping(uint256 decryptionId =>
            mapping(bytes32 digest => address[] kmsTxSenderAddresses))
               consensusTxSenderAddresses;
        /// @notice The digest of the signed struct on which consensus was reached for a decryption request.
        mapping(uint256 decryptionId => bytes32 consensusDigest) decryptionConsensusDigest;
        // ----------------------------------------------------------------------------------------------
        // Public decryption state variables:
        // ----------------------------------------------------------------------------------------------
        // prettier-ignore
        /// @notice Verified signatures for a public decryption.
        mapping(uint256 decryptionId =>
            mapping(bytes32 digest => bytes[] verifiedSignatures))
                verifiedPublicDecryptSignatures;
        /// @notice Handles of the ciphertexts requested for a public decryption
        mapping(uint256 decryptionId => bytes32[] ctHandles) publicCtHandles;
        /// @notice The number of public decryption requests, used to generate request IDs (`decryptionId`).
        uint256 publicDecryptionCounter;
        // ----------------------------------------------------------------------------------------------
        // User decryption state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The decryption payloads stored during user decryption requests.
        mapping(uint256 decryptionId => UserDecryptionPayload payload) userDecryptionPayloads;
        /// @notice The number of user decryption requests, used to generate request IDs (`decryptionId`)
        /// @notice (including delegated user decryption requests).
        uint256 userDecryptionCounter;
    }

    /**
     * @notice Storage location has been computed using the following command:
     * keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.Decryption")) - 1)) &
     * ~bytes32(uint256(0xff))
     */
    bytes32 private constant DECRYPTION_STORAGE_LOCATION =
        0x68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee700;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initializes the contract.
     * @dev Contract name and version for EIP712 signature validation are defined here
     * This function needs to be public in order to be called by the UUPS proxy.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy() public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        __EIP712_init(CONTRACT_NAME, "1");
        __Pausable_init();

        DecryptionStorage storage $ = _getDecryptionStorage();

        // Initialize the counters in order to generate globally unique requestIds per request type
        $.publicDecryptionCounter = PUBLIC_DECRYPT_COUNTER_BASE;
        $.userDecryptionCounter = USER_DECRYPT_COUNTER_BASE;
    }

    /**
     * @notice Re-initializes the contract from V1.
     * @dev Define a `reinitializeVX` function once the contract needs to be upgraded.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    // function reinitializeV2() public virtual reinitializer(REINITIALIZER_VERSION) {}

    /**
     * @notice See {IDecryption-publicDecryptionRequest}.
     */
    function publicDecryptionRequest(
        bytes32[] calldata ctHandles,
        bytes calldata extraData
    ) external virtual whenNotPaused {
        // Check that the list of handles is not empty
        if (ctHandles.length == 0) {
            revert EmptyCtHandles();
        }

        // Check the handles' conformance
        _checkCtHandlesConformancePublic(ctHandles);

        // Fetch the SNS ciphertexts from the CiphertextCommits contract
        // This call is reverted if any of the ciphertexts are not found in the contract, but
        // this should not happen for now as a ciphertext cannot be allowed for decryption
        // without being added to the contract first (and we currently have no ways of deleting
        // a ciphertext from the contract).
        SnsCiphertextMaterial[] memory snsCtMaterials = CIPHERTEXT_COMMITS.getSnsCiphertextMaterials(ctHandles);

        // Check that received snsCtMaterials have the same keyId.
        // TODO: This should be removed once batched decryption requests with different keys is
        // supported by the KMS (see https://github.com/zama-ai/fhevm-internal/issues/376)
        _checkCtMaterialKeyIds(snsCtMaterials);

        DecryptionStorage storage $ = _getDecryptionStorage();

        // Generate a globally unique decryptionId for the public decryption request.
        // The counter is initialized at deployment such that decryptionId's first byte uniquely
        // represents a public decryption request, with format: [0000 0001 | counter_1..31]
        // This counter is used to ensure the IDs' uniqueness, as there is no proper way
        // of generating truly pseudo-random numbers on-chain on Arbitrum. This has some impact on
        // how IDs need to be handled off-chain in case of re-org.
        $.publicDecryptionCounter++;
        uint256 publicDecryptionId = $.publicDecryptionCounter;

        // The handles are used during response calls for the EIP712 signature validation.
        $.publicCtHandles[publicDecryptionId] = ctHandles;

        emit PublicDecryptionRequest(publicDecryptionId, snsCtMaterials, extraData);
    }

    /**
     * @notice See {IDecryption-publicDecryptionResponse}.
     * @dev We restrict this call to KMS transaction senders because, in case of reorgs, we need to
     * prevent anyone else from copying the signature and sending it to trigger a consensus.
     */
    function publicDecryptionResponse(
        uint256 decryptionId,
        bytes calldata decryptedResult,
        bytes calldata signature,
        bytes calldata extraData
    ) external virtual onlyKmsTxSender {
        DecryptionStorage storage $ = _getDecryptionStorage();

        // Make sure the decryptionId corresponds to a generated public decryption request:
        // - it must be greater than the base counter for public decryption requests
        // - it must be less than or equal to the current public decryption counter
        if (decryptionId <= PUBLIC_DECRYPT_COUNTER_BASE || decryptionId > $.publicDecryptionCounter) {
            revert DecryptionNotRequested(decryptionId);
        }

        // Initialize the PublicDecryptVerification structure for the signature validation.
        PublicDecryptVerification memory publicDecryptVerification = PublicDecryptVerification(
            $.publicCtHandles[decryptionId],
            decryptedResult,
            extraData
        );

        // Compute the digest of the PublicDecryptVerification structure.
        bytes32 digest = _hashPublicDecryptVerification(publicDecryptVerification);

        // Recover the signer address from the signature and validate that corresponds to a
        // KMS node that has not already signed.
        _validateDecryptionResponseEIP712Signature(decryptionId, digest, signature);

        // Store the signature for the public decryption response.
        // This list is then used to check the consensus. Important: the mapping considers
        // the digest (contrary to the user decryption case) as the decrypted result is expected
        // to be the same for all KMS nodes. This allows to filter out results from malicious
        // KMS nodes.
        bytes[] storage verifiedSignatures = $.verifiedPublicDecryptSignatures[decryptionId][digest];
        verifiedSignatures.push(signature);

        // Store the KMS transaction sender address for the public decryption response
        // It is important to consider the same mapping fields used for the consensus
        // A "late" valid KMS transaction sender address will still be added in the list.
        $.consensusTxSenderAddresses[decryptionId][digest].push(msg.sender);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (!$.decryptionDone[decryptionId] && _isConsensusReachedPublic(verifiedSignatures.length)) {
            $.decryptionDone[decryptionId] = true;

            // A "late" valid KMS could still see its transaction sender address be added to the list
            // after consensus. This storage variable is here to be able to retrieve this list later
            // by only knowing the decryption ID, since a consensus can only happen once per decryption
            // request, independently of the decryption response type (public or user).
            $.decryptionConsensusDigest[decryptionId] = digest;

            emit PublicDecryptionResponse(decryptionId, decryptedResult, verifiedSignatures, extraData);
        }
    }

    /**
     * @notice See {IDecryption-userDecryptionRequest}.
     */
    function userDecryptionRequest(
        CtHandleContractPair[] calldata ctHandleContractPairs,
        RequestValidity calldata requestValidity,
        ContractsInfo calldata contractsInfo,
        address userAddress,
        bytes calldata publicKey,
        bytes calldata signature,
        bytes calldata extraData
    ) external virtual whenNotPaused {
        if (contractsInfo.addresses.length == 0) {
            revert EmptyContractAddresses();
        }
        if (contractsInfo.addresses.length > MAX_USER_DECRYPT_CONTRACT_ADDRESSES) {
            revert ContractAddressesMaxLengthExceeded(
                MAX_USER_DECRYPT_CONTRACT_ADDRESSES,
                contractsInfo.addresses.length
            );
        }

        // Check the user decryption request is valid.
        _checkUserDecryptionRequestValidity(requestValidity);

        // Check the user address is not included in the contract addresses.
        if (_containsContractAddress(contractsInfo.addresses, userAddress)) {
            revert UserAddressInContractAddresses(userAddress, contractsInfo.addresses);
        }

        // - Extract the handles and check their conformance
        bytes32[] memory ctHandles = _extractCtHandlesCheckConformanceUser(
            ctHandleContractPairs,
            contractsInfo.addresses,
            userAddress
        );

        // Initialize the UserDecryptRequestVerification structure for the signature validation.
        UserDecryptRequestVerification memory userDecryptRequestVerification = UserDecryptRequestVerification(
            publicKey,
            contractsInfo.addresses,
            requestValidity.startTimestamp,
            requestValidity.durationDays,
            extraData
        );

        // Validate the received EIP712 signature on the user decryption request.
        _validateUserDecryptRequestEIP712Signature(
            userDecryptRequestVerification,
            userAddress,
            signature,
            contractsInfo.chainId
        );

        // Fetch the ciphertexts from the CiphertextCommits contract
        // This call is reverted if any of the ciphertexts are not found in the contract, but
        // this should not happen for now as a ciphertext cannot be allowed for decryption
        // without being added to the contract first (and we currently have no ways of deleting
        // a ciphertext from the contract).
        SnsCiphertextMaterial[] memory snsCtMaterials = CIPHERTEXT_COMMITS.getSnsCiphertextMaterials(ctHandles);

        // Check that received snsCtMaterials have the same keyId.
        // TODO: This should be removed once batched decryption requests with different keys is
        // supported by the KMS (see https://github.com/zama-ai/fhevm-internal/issues/376)
        _checkCtMaterialKeyIds(snsCtMaterials);

        DecryptionStorage storage $ = _getDecryptionStorage();

        // Generate a globally unique decryptionId for the user decryption request.
        // The counter is initialized at deployment such that decryptionId's first byte uniquely
        // represents a user decryption request (including delegated user decryption requests),
        // with format: [0000 0010 | counter_1..31]
        // This counter is used to ensure the IDs' uniqueness, as there is no proper way
        // of generating truly pseudo-random numbers on-chain on Arbitrum. This has some impact on
        // how IDs need to be handled off-chain in case of re-org.
        $.userDecryptionCounter++;
        uint256 userDecryptionId = $.userDecryptionCounter;

        // The publicKey and ctHandles are used during response calls for the EIP712 signature validation.
        $.userDecryptionPayloads[userDecryptionId] = UserDecryptionPayload(publicKey, ctHandles);

        emit UserDecryptionRequest(userDecryptionId, snsCtMaterials, userAddress, publicKey, extraData);
    }

    /**
     * @notice See {IDecryption-delegatedUserDecryptionRequest}.
     */
    function delegatedUserDecryptionRequest(
        CtHandleContractPair[] calldata ctHandleContractPairs,
        RequestValidity calldata requestValidity,
        DelegationAccounts calldata delegationAccounts,
        ContractsInfo calldata contractsInfo,
        bytes calldata publicKey,
        bytes calldata signature,
        bytes calldata extraData
    ) external virtual whenNotPaused {
        if (contractsInfo.addresses.length == 0) {
            revert EmptyContractAddresses();
        }
        if (contractsInfo.addresses.length > MAX_USER_DECRYPT_CONTRACT_ADDRESSES) {
            revert ContractAddressesMaxLengthExceeded(
                MAX_USER_DECRYPT_CONTRACT_ADDRESSES,
                contractsInfo.addresses.length
            );
        }

        // Check the user decryption request is valid.
        _checkUserDecryptionRequestValidity(requestValidity);

        // Check the delegator address is not included in the contract addresses.
        if (_containsContractAddress(contractsInfo.addresses, delegationAccounts.delegatorAddress)) {
            revert DelegatorAddressInContractAddresses(delegationAccounts.delegatorAddress, contractsInfo.addresses);
        }

        // Extract the handles and check their conformance.
        bytes32[] memory ctHandles = _extractCtHandlesCheckConformanceUser(
            ctHandleContractPairs,
            contractsInfo.addresses,
            delegationAccounts.delegatorAddress
        );

        // Check that the delegate address has been granted access to the contract addresses by the delegator.
        _checkIsUserDecryptionDelegated(
            contractsInfo.chainId,
            delegationAccounts.delegatorAddress,
            delegationAccounts.delegateAddress,
            contractsInfo.addresses
        );

        // Initialize the DelegatedUserDecryptRequestVerification structure for the signature validation.
        DelegatedUserDecryptRequestVerification
            memory delegatedUserDecryptRequestVerification = DelegatedUserDecryptRequestVerification(
                publicKey,
                contractsInfo.addresses,
                delegationAccounts.delegatorAddress,
                requestValidity.startTimestamp,
                requestValidity.durationDays,
                extraData
            );

        // Validate the received EIP712 signature on the delegated user decryption request.
        _validateDelegatedUserDecryptRequestEIP712Signature(
            delegatedUserDecryptRequestVerification,
            delegationAccounts.delegateAddress,
            signature,
            contractsInfo.chainId
        );

        // Fetch the ciphertexts from the CiphertextCommits contract.
        // This call is reverted if any of the ciphertexts are not found in the contract, but
        // this should not happen for now as a ciphertext cannot be allowed for decryption
        // without being added to the contract first (and we currently have no ways of deleting
        // a ciphertext from the contract).
        SnsCiphertextMaterial[] memory snsCtMaterials = CIPHERTEXT_COMMITS.getSnsCiphertextMaterials(ctHandles);

        // Check that received snsCtMaterials have the same keyId.
        // TODO: This should be removed once batched decryption requests with different keys is
        // supported by the KMS (see https://github.com/zama-ai/fhevm-internal/issues/376).
        _checkCtMaterialKeyIds(snsCtMaterials);

        DecryptionStorage storage $ = _getDecryptionStorage();

        // Generate a globally unique decryptionId for the delegated user decryption request.
        // The counter is initialized at deployment such that decryptionId's first byte uniquely
        // represents a user decryption request (including delegated user decryption requests),
        // with format: [0000 0010 | counter_1..31].
        // This counter is used to ensure the IDs' uniqueness, as there is no proper way
        // of generating truly pseudo-random numbers on-chain on Arbitrum. This has some impact on
        // how IDs need to be handled off-chain in case of re-org.
        $.userDecryptionCounter++;
        uint256 userDecryptionId = $.userDecryptionCounter;

        // The publicKey and ctHandles are used during response calls for the EIP712 signature validation.
        $.userDecryptionPayloads[userDecryptionId] = UserDecryptionPayload(publicKey, ctHandles);

        emit UserDecryptionRequest(
            userDecryptionId,
            snsCtMaterials,
            delegationAccounts.delegateAddress,
            publicKey,
            extraData
        );
    }

    /**
     * @notice See {IDecryption-userDecryptionResponse}.
     * @dev We restrict this call to KMS transaction senders because, in case of reorgs, we need to
     * prevent anyone else from copying the signature and sending it to trigger a consensus.
     */
    function userDecryptionResponse(
        uint256 decryptionId,
        bytes calldata userDecryptedShare,
        bytes calldata signature,
        bytes calldata extraData
    ) external virtual onlyKmsTxSender {
        DecryptionStorage storage $ = _getDecryptionStorage();

        // Make sure the decryptionId corresponds to a generated user decryption request:
        // - it must be greater than the base counter for user decryption requests
        // - it must be less than or equal to the current user decryption counter
        if (decryptionId <= USER_DECRYPT_COUNTER_BASE || decryptionId > $.userDecryptionCounter) {
            revert DecryptionNotRequested(decryptionId);
        }

        UserDecryptionPayload memory userDecryptionPayload = $.userDecryptionPayloads[decryptionId];
        // Initialize the UserDecryptResponseVerification structure for the signature validation.
        UserDecryptResponseVerification memory userDecryptResponseVerification = UserDecryptResponseVerification(
            userDecryptionPayload.publicKey,
            userDecryptionPayload.ctHandles,
            userDecryptedShare,
            extraData
        );

        // Compute the digest of the UserDecryptResponseVerification structure.
        bytes32 digest = _hashUserDecryptResponseVerification(userDecryptResponseVerification);

        // Recover the signer address from the signature and validate that it corresponds to a
        // KMS node that has not already signed.
        _validateDecryptionResponseEIP712Signature(decryptionId, digest, signature);

        // Store the KMS transaction sender address for the public decryption response
        // It is important to consider the same mapping fields used for the consensus
        // A "late" valid KMS transaction sender address will still be added in the list.
        // We thus use a zero digest (default value for `bytes32`) to still be able to retrieve the
        // list later independently of the decryption response type (public or user).
        address[] storage txSenderAddresses = $.consensusTxSenderAddresses[decryptionId][0];
        txSenderAddresses.push(msg.sender);

        // Store the user decrypted share for the user decryption response.
        // The index of the share is the length of the txSenderAddresses - 1 so that the first response
        // associated to this decryptionId has an index of 0.
        emit UserDecryptionResponse(
            decryptionId,
            txSenderAddresses.length - 1,
            userDecryptedShare,
            signature,
            extraData
        );

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (!$.decryptionDone[decryptionId] && _isThresholdReachedUser(txSenderAddresses.length)) {
            $.decryptionDone[decryptionId] = true;

            // Since we use the default value for `bytes32`, this means we do not need to store the
            // digest in `decryptionConsensusDigest` here like we do for the public decryption case.

            emit UserDecryptionResponseThresholdReached(decryptionId);
        }
    }

    /**
     * @dev See {IDecryption-isPublicDecryptionReady}.
     */
    function isPublicDecryptionReady(
        bytes32[] calldata ctHandles,
        bytes calldata /* extraData */
    ) external view virtual returns (bool) {
        // For each handle, check that it is allowed for public decryption and that the ciphertext
        // material represented by it has been added.
        for (uint256 i = 0; i < ctHandles.length; i++) {
            if (
                !MULTICHAIN_ACL.isPublicDecryptAllowed(ctHandles[i]) ||
                !CIPHERTEXT_COMMITS.isCiphertextMaterialAdded(ctHandles[i])
            ) {
                return false;
            }
        }
        return true;
    }

    /**
     * @dev See {IDecryption-isUserDecryptionReady}.
     */
    function isUserDecryptionReady(
        address userAddress,
        CtHandleContractPair[] calldata ctHandleContractPairs,
        bytes calldata /* extraData */
    ) external view virtual returns (bool) {
        // For each handle, check that the user and contracts accounts have access to it and that the
        // ciphertext material represented by it has been added.
        for (uint256 i = 0; i < ctHandleContractPairs.length; i++) {
            if (
                !MULTICHAIN_ACL.isAccountAllowed(ctHandleContractPairs[i].ctHandle, userAddress) ||
                !MULTICHAIN_ACL.isAccountAllowed(
                    ctHandleContractPairs[i].ctHandle,
                    ctHandleContractPairs[i].contractAddress
                ) ||
                !CIPHERTEXT_COMMITS.isCiphertextMaterialAdded(ctHandleContractPairs[i].ctHandle)
            ) {
                return false;
            }
        }
        return true;
    }

    /**
     * @notice See {IDecryption-isDecryptionDone}.
     */
    function isDecryptionDone(uint256 decryptionId) external view virtual returns (bool) {
        DecryptionStorage storage $ = _getDecryptionStorage();
        return $.decryptionDone[decryptionId];
    }

    /**
     * @notice See {IDecryption-getDecryptionConsensusTxSenders}.
     * For public decryption, the returned list remains empty until the consensus is reached.
     */
    function getDecryptionConsensusTxSenders(uint256 decryptionId) external view virtual returns (address[] memory) {
        DecryptionStorage storage $ = _getDecryptionStorage();

        // Get the unique digest associated to the decryption request in order to retrieve the list of
        // KMS transaction sender addresses that were involved in the associated consensus
        // For public decryption, this digest remains the default value (0x0) until the consensus is
        // reached, meaning the returned list will be empty until then.
        bytes32 consensusDigest = $.decryptionConsensusDigest[decryptionId];

        return $.consensusTxSenderAddresses[decryptionId][consensusDigest];
    }

    /**
     * @notice See {IDecryption-getVersion}.
     */
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

    /**
     * @notice Validates the EIP712 signature for a given decryption response.
     * @param decryptionId The decryption request ID.
     * @param digest The hashed EIP712 struct.
     * @param signature The signature to validate.
     */
    function _validateDecryptionResponseEIP712Signature(
        uint256 decryptionId,
        bytes32 digest,
        bytes calldata signature
    ) internal virtual {
        DecryptionStorage storage $ = _getDecryptionStorage();
        address signer = ECDSA.recover(digest, signature);

        // Check that the signer is a KMS signer.
        _checkIsKmsSigner(signer);

        // Check that the signer has not already responded to the user decryption request.
        if ($.kmsNodeAlreadySigned[decryptionId][signer]) {
            revert KmsNodeAlreadySigned(decryptionId, signer);
        }

        $.kmsNodeAlreadySigned[decryptionId][signer] = true;
    }

    /**
     * @notice Checks if the sender is authorized to upgrade the contract and reverts otherwise.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyGatewayOwner {}

    /**
     * @notice Validates the EIP712 signature for a given user decryption request
     * @dev This function checks that the signer address is the same as the user address.
     * @param userDecryptRequestVerification The signed UserDecryptRequestVerification structure
     * @param userAddress The address of the user.
     * @param signature The signature to be validated
     * @param contractsChainId The chain ID of the contracts.
     */
    function _validateUserDecryptRequestEIP712Signature(
        UserDecryptRequestVerification memory userDecryptRequestVerification,
        address userAddress,
        bytes calldata signature,
        uint256 contractsChainId
    ) internal view virtual {
        bytes32 digest = _hashUserDecryptRequestVerification(userDecryptRequestVerification, contractsChainId);
        address signer = ECDSA.recover(digest, signature);
        if (signer != userAddress) {
            revert InvalidUserSignature(signature);
        }
    }

    /**
     * @notice Validates the EIP712 signature for a given delegated user decryption request.
     * @dev This function checks that the signer address is the same as the delegate address.
     * @param delegatedUserDecryptRequestVerification The signed DelegatedUserDecryptRequestVerification structure.
     * @param delegateAddress The address of the account that has delegation granted.
     * @param signature The signature to be validated.
     * @param contractsChainId The chain ID of the contracts.
     */
    function _validateDelegatedUserDecryptRequestEIP712Signature(
        DelegatedUserDecryptRequestVerification memory delegatedUserDecryptRequestVerification,
        address delegateAddress,
        bytes calldata signature,
        uint256 contractsChainId
    ) internal view virtual {
        bytes32 digest = _hashDelegatedUserDecryptRequestVerification(
            delegatedUserDecryptRequestVerification,
            contractsChainId
        );
        address signer = ECDSA.recover(digest, signature);
        if (signer != delegateAddress) {
            revert InvalidUserSignature(signature);
        }
    }

    /**
     * @notice Computes the hash of a given PublicDecryptVerification structured data
     * @param publicDecryptVerification The PublicDecryptVerification structure
     * @return The hash of the PublicDecryptVerification structure
     */
    function _hashPublicDecryptVerification(
        PublicDecryptVerification memory publicDecryptVerification
    ) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        EIP712_PUBLIC_DECRYPT_TYPE_HASH,
                        keccak256(abi.encodePacked(publicDecryptVerification.ctHandles)),
                        keccak256(publicDecryptVerification.decryptedResult),
                        keccak256(abi.encodePacked(publicDecryptVerification.extraData))
                    )
                )
            );
    }

    /**
     * @notice Computes the hash of the hashed struct using a custom chain ID for the eip712 domain
     * @param chainId The chain ID
     * @param structHash The hash of the struct
     * @dev This could be improved along https://github.com/zama-ai/fhevm/issues/424
     */
    function _hashTypedDataV4CustomChainId(
        uint256 chainId,
        bytes32 structHash
    ) internal view virtual returns (bytes32) {
        bytes32 domainSeparatorV4 = keccak256(
            abi.encode(DOMAIN_TYPE_HASH, _EIP712NameHash(), _EIP712VersionHash(), chainId, address(this))
        );
        return MessageHashUtils.toTypedDataHash(domainSeparatorV4, structHash);
    }

    /**
     * @notice Computes the hash of a given UserDecryptRequestVerification structured data.
     * @param userDecryptRequestVerification The UserDecryptRequestVerification structure to hash.
     * @param contractsChainId The chain ID of the contracts.
     * @return The hash of the UserDecryptRequestVerification structure.
     */
    function _hashUserDecryptRequestVerification(
        UserDecryptRequestVerification memory userDecryptRequestVerification,
        uint256 contractsChainId
    ) internal view virtual returns (bytes32) {
        bytes32 structHash = keccak256(
            abi.encode(
                EIP712_USER_DECRYPT_REQUEST_TYPE_HASH,
                keccak256(userDecryptRequestVerification.publicKey),
                keccak256(abi.encodePacked(userDecryptRequestVerification.contractAddresses)),
                userDecryptRequestVerification.startTimestamp,
                userDecryptRequestVerification.durationDays,
                keccak256(abi.encodePacked(userDecryptRequestVerification.extraData))
            )
        );
        return _hashTypedDataV4CustomChainId(contractsChainId, structHash);
    }

    /**
     * @notice Computes the hash of a given DelegatedUserDecryptRequestVerification structured data.
     * @param delegatedUserDecryptRequestVerification The DelegatedUserDecryptRequestVerification structure to hash.
     * @param contractsChainId The chain ID of the contracts.
     * @return The hash of the DelegatedUserDecryptRequestVerification structure.
     */
    function _hashDelegatedUserDecryptRequestVerification(
        DelegatedUserDecryptRequestVerification memory delegatedUserDecryptRequestVerification,
        uint256 contractsChainId
    ) internal view virtual returns (bytes32) {
        bytes32 structHash = keccak256(
            abi.encode(
                EIP712_DELEGATED_USER_DECRYPT_REQUEST_TYPE_HASH,
                keccak256(delegatedUserDecryptRequestVerification.publicKey),
                keccak256(abi.encodePacked(delegatedUserDecryptRequestVerification.contractAddresses)),
                delegatedUserDecryptRequestVerification.delegatorAddress,
                delegatedUserDecryptRequestVerification.startTimestamp,
                delegatedUserDecryptRequestVerification.durationDays,
                keccak256(abi.encodePacked(delegatedUserDecryptRequestVerification.extraData))
            )
        );
        return _hashTypedDataV4CustomChainId(contractsChainId, structHash);
    }

    /**
     * @notice Computes the hash of a given UserDecryptResponseVerification structured data.
     * @param userDecryptResponseVerification The UserDecryptResponseVerification structure to hash.
     * @return The hash of the UserDecryptResponseVerification structure.
     */
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
                        keccak256(userDecryptResponseVerification.userDecryptedShare),
                        keccak256(abi.encodePacked(userDecryptResponseVerification.extraData))
                    )
                )
            );
    }

    /**
     * @notice Indicates if the consensus is reached for public decryption.
     * @param numVerifiedResponses The number of public decryption responses that have been verified.
     * @return Whether the consensus has been reached
     */
    function _isConsensusReachedPublic(uint256 numVerifiedResponses) internal view virtual returns (bool) {
        uint256 publicDecryptionThreshold = GATEWAY_CONFIG.getPublicDecryptionThreshold();
        return numVerifiedResponses >= publicDecryptionThreshold;
    }

    /**
     * @notice Indicates if the number of verified user decryption responses has reached the threshold.
     * @param numVerifiedResponses The number of user decryption responses that have been verified.
     * @return Whether the threshold has been reached.
     */
    function _isThresholdReachedUser(uint256 numVerifiedResponses) internal view virtual returns (bool) {
        uint256 userDecryptionThreshold = GATEWAY_CONFIG.getUserDecryptionThreshold();
        return numVerifiedResponses >= userDecryptionThreshold;
    }

    /**
     * @notice Check the handles' conformance for public decryption requests.
     * @dev Checks include:
     * @dev - Total bit size for each handle
     * @dev - FHE type validity for each handle
     * @dev - Handles are allowed for public decryption
     * @param ctHandles The list of ciphertext handles
     */
    function _checkCtHandlesConformancePublic(bytes32[] memory ctHandles) internal view virtual {
        uint256 totalBitSize = 0;
        for (uint256 i = 0; i < ctHandles.length; i++) {
            bytes32 ctHandle = ctHandles[i];

            // Extract the FHE type from the ciphertext handle
            FheType fheType = HandleOps.extractFheType(ctHandle);

            // Add the bit size of the FHE type to the total bit size
            // This reverts if the FHE type is invalid or not supported.
            totalBitSize += FHETypeBitSizes.getBitSize(fheType);

            // Check that the handles are allowed for public decryption.
            _checkIsPublicDecryptAllowed(ctHandle);
        }

        // Revert if the total bit size exceeds the maximum allowed.
        if (totalBitSize > MAX_DECRYPTION_REQUEST_BITS) {
            revert MaxDecryptionRequestBitSizeExceeded(MAX_DECRYPTION_REQUEST_BITS, totalBitSize);
        }
    }

    /**
     * @notice Extracts the handles and check their conformance for user decryption requests.
     * @dev Checks include:
     * @dev - Total bit size for each handle
     * @dev - FHE type validity for each handle
     * @dev - Contract addresses have access to the handles
     * @dev - Allowed address has access to the handles
     * @dev - Contract address inclusion in the list of allowed contract addresses
     * @param ctHandleContractPairs The list of ciphertext handles and contract addresses
     * @param contractAddresses The list of allowed contract addresses
     * @param allowedAddress The address that is allowed to access the handles
     * @return ctHandles The list of ciphertext handles
     */
    function _extractCtHandlesCheckConformanceUser(
        CtHandleContractPair[] calldata ctHandleContractPairs,
        address[] memory contractAddresses,
        address allowedAddress
    ) internal view virtual returns (bytes32[] memory ctHandles) {
        // Check that the list of ctHandleContractPair is not empty
        if (ctHandleContractPairs.length == 0) {
            revert EmptyCtHandleContractPairs();
        }

        ctHandles = new bytes32[](ctHandleContractPairs.length);

        uint256 totalBitSize = 0;
        for (uint256 i = 0; i < ctHandleContractPairs.length; i++) {
            bytes32 ctHandle = ctHandleContractPairs[i].ctHandle;
            address contractAddress = ctHandleContractPairs[i].contractAddress;

            // Extract the FHE type from the ciphertext handle
            FheType fheType = HandleOps.extractFheType(ctHandle);

            // Add the bit size of the FHE type to the total bit size
            // This reverts if the FHE type is invalid or not supported
            totalBitSize += FHETypeBitSizes.getBitSize(fheType);

            // Check that the allowed and contract accounts have access to the handles.
            _checkIsAccountAllowed(ctHandle, allowedAddress);
            _checkIsAccountAllowed(ctHandle, contractAddress);

            // Check the contract is included in the list of allowed contract addresses.
            if (!_containsContractAddress(contractAddresses, contractAddress)) {
                revert ContractNotInContractAddresses(contractAddress, contractAddresses);
            }

            ctHandles[i] = ctHandle;
        }

        // Revert if the total bit size exceeds the maximum allowed.
        if (totalBitSize > MAX_DECRYPTION_REQUEST_BITS) {
            revert MaxDecryptionRequestBitSizeExceeded(MAX_DECRYPTION_REQUEST_BITS, totalBitSize);
        }
    }

    /**
     * @notice Checks if a user decryption request's start timestamp and duration days are valid.
     * @param requestValidity The RequestValidity structure
     */
    function _checkUserDecryptionRequestValidity(RequestValidity memory requestValidity) internal view virtual {
        // Check the durationDays is not null.
        if (requestValidity.durationDays == 0) {
            revert InvalidNullDurationDays();
        }
        // Check the durationDays does not exceed the maximum allowed.
        if (requestValidity.durationDays > MAX_USER_DECRYPT_DURATION_DAYS) {
            revert MaxDurationDaysExceeded(MAX_USER_DECRYPT_DURATION_DAYS, requestValidity.durationDays);
        }

        // Check the start timestamp is not set in the future. This is to prevent a user
        // from bypassing the durationDays limit of 365 days by setting a start timestamp
        // far in the future.
        if (requestValidity.startTimestamp > block.timestamp) {
            revert StartTimestampInFuture(block.timestamp, requestValidity.startTimestamp);
        }

        // Check the user decryption request has not expired. A user decryption request is valid
        // from startTimestamp for a number of days equal to durationDays.
        if (requestValidity.startTimestamp + requestValidity.durationDays * 1 days < block.timestamp) {
            revert UserDecryptionRequestExpired(block.timestamp, requestValidity);
        }
    }

    /**
     * @notice Checks if a given contractAddress is included in the contractAddresses list.
     * @param contractAddresses The list of contract addresses
     * @param contractAddress The contract address to check
     * @return Whether the contract address is included in the list
     */
    function _containsContractAddress(
        address[] memory contractAddresses,
        address contractAddress
    ) internal pure virtual returns (bool) {
        for (uint256 i = 0; i < contractAddresses.length; i++) {
            if (contractAddresses[i] == contractAddress) {
                return true;
            }
        }
        return false;
    }

    /**
     * @notice Checks that all SNS ciphertext materials have the same keyId.
     * @param snsCtMaterials The list of SNS ciphertext materials to check
     * @dev TODO: This should be removed once batched decryption requests with different keys is
     * supported by the KMS (see https://github.com/zama-ai/fhevm-internal/issues/376)
     */
    function _checkCtMaterialKeyIds(SnsCiphertextMaterial[] memory snsCtMaterials) internal pure virtual {
        if (snsCtMaterials.length <= 1) return;

        uint256 firstKeyId = snsCtMaterials[0].keyId;
        for (uint256 i = 1; i < snsCtMaterials.length; i++) {
            if (snsCtMaterials[i].keyId != firstKeyId) {
                revert DifferentKeyIdsNotAllowed(snsCtMaterials[0], snsCtMaterials[i]);
            }
        }
    }

    /**
     * @notice Returns the Decryption storage location.
     * @dev Note that this function is internal but not virtual: derived contracts should be able to
     * access it, but if the underlying storage struct version changes, we force them to define a new
     * getter function and use that one instead in order to avoid overriding the storage location.
     */
    function _getDecryptionStorage() internal pure returns (DecryptionStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := DECRYPTION_STORAGE_LOCATION
        }
    }
}
