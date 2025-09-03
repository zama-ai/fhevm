// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { IKmsManagement } from "./interfaces/IKmsManagement.sol";
import { IGatewayConfig } from "./interfaces/IGatewayConfig.sol";
import { gatewayConfigAddress } from "../addresses/GatewayAddresses.sol";
import { ECDSA } from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import { EIP712Upgradeable } from "@openzeppelin/contracts-upgradeable/utils/cryptography/EIP712Upgradeable.sol";
import { MessageHashUtils } from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { UUPSUpgradeableEmptyProxy } from "./shared/UUPSUpgradeableEmptyProxy.sol";
import { GatewayConfigChecks } from "./shared/GatewayConfigChecks.sol";
import { Pausable } from "./shared/Pausable.sol";
import { PREP_KEYGEN_COUNTER_BASE, KEY_COUNTER_BASE, CRS_COUNTER_BASE } from "./shared/KmsRequestCounters.sol";

/**
 * @title KMS Management contract
 * @dev See {IKmsManagement}.
 */
contract KmsManagement is
    IKmsManagement,
    EIP712Upgradeable,
    Ownable2StepUpgradeable,
    UUPSUpgradeableEmptyProxy,
    GatewayConfigChecks,
    Pausable
{
    // ----------------------------------------------------------------------------------------------
    // EIP712 utility constants:
    // ----------------------------------------------------------------------------------------------

    /**
     * @notice The PrepKeygenVerification typed definition.
     * @dev prepKeygenId: The ID of the preprocessing keygen request.
     */
    string private constant EIP712_PREP_KEYGEN_TYPE = "PrepKeygenVerification(uint256 prepKeygenId)";

    /**
     * @notice The hash of the PrepKeygenVerification typed definition.
     */
    bytes32 private constant EIP712_PREP_KEYGEN_TYPE_HASH = keccak256(bytes(EIP712_PREP_KEYGEN_TYPE));

    /**
     * @notice The EIP-712 type definition for the KeyDigest struct.
     * @dev Required because EIP-712 mandates that each nested struct type
     *      used in a primary type (e.g. KeygenVerification) must be explicitly
     *      declared with its own type string and type hash.
     *      These constants are used when computing the struct hash of each
     *      KeyDigest element inside the keyDigests[] array.
     */
    string private constant EIP712_KEYDIGEST_TYPE = "KeyDigest(uint8 keyType,bytes digest)";
    bytes32 private constant EIP712_KEYDIGEST_TYPE_HASH = keccak256(bytes(EIP712_KEYDIGEST_TYPE));

    /**
     * @notice The KeygenVerification typed definition.
     * @dev prepKeygenId: The ID of the preprocessing keygen request.
     * @dev keyId: The ID of the generated key.
     * @dev keyDigests: The digests of the generated key.
     */
    string private constant EIP712_KEYGEN_TYPE =
        "KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests)KeyDigest(uint8 keyType,bytes digest)";

    /**
     * @notice The hash of the KeygenVerification typed definition.
     */
    bytes32 private constant EIP712_KEYGEN_TYPE_HASH = keccak256(bytes(EIP712_KEYGEN_TYPE));

    /**
     * @notice The CrsgenVerification typed definition.
     * @dev crsId: The ID of the generated CRS.
     * @dev maxBitLength: The max bit length of the generated CRS.
     * @dev crsDigest: The digest of the generated CRS.
     */
    string private constant EIP712_CRSGEN_TYPE =
        "CrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest)";

    /**
     * @notice The hash of the CrsgenVerification typed definition.
     */
    bytes32 private constant EIP712_CRSGEN_TYPE_HASH = keccak256(bytes(EIP712_CRSGEN_TYPE));

    // ----------------------------------------------------------------------------------------------
    // Other contract references:
    // ----------------------------------------------------------------------------------------------

    /**
     * @notice The address of the GatewayConfig contract for protocol state calls.
     */
    IGatewayConfig private constant GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);

    // ----------------------------------------------------------------------------------------------
    // Contract information:
    // ----------------------------------------------------------------------------------------------

    /**
     * @dev The following constants are used for versioning the contract. They are made private
     * in order to force derived contracts to consider a different version. Note that
     * they can still define their own private constants with the same name.
     */
    string private constant CONTRACT_NAME = "KmsManagement";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 2;
    uint256 private constant PATCH_VERSION = 0;

    /**
     * @dev Constant used for making sure the version number using in the `reinitializer` modifier
     * is identical between `initializeFromEmptyProxy` and the reinitializeVX` method
     */
    uint64 private constant REINITIALIZER_VERSION = 3;

    // ----------------------------------------------------------------------------------------------
    // Contract storage:
    // ----------------------------------------------------------------------------------------------

    /**
     * @notice The contract's variable storage struct (@dev see ERC-7201)
     */
    /// @custom:storage-location erc7201:fhevm_gateway.storage.KmsManagement
    struct KmsManagementStorage {
        /// @notice DEPRECATED
        mapping(string fheParamsName => bytes32 paramsDigest) fheParamsDigests; // DEPRECATED
        /// @notice DEPRECATED: use prepKeygenCounter instead
        uint256 _preKeygenRequestCounter; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 preKeygenRequestId => uint256 counter) _preKeygenResponseCounter; // DEPRECATED
        /// @notice DEPRECATED: use isPrepKeygenDone instead
        mapping(uint256 preKeyId => bool isDone) _isPreKeygenDone; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 preKeyId => mapping(address kmsConnector => bool hasResponded)) _preKeygenResponses; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 preKeyId => bool isOngoing) _isKeygenOngoing; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 keyId => mapping(address kmsConnector => bool hasResponded)) _keygenResponses; // DEPRECATED
        /// @notice DEPRECATED: use keygenResponseCounter instead
        mapping(uint256 keyId => uint256 responseCounter) _keygenResponseCounter; // DEPRECATED
        /// @notice DEPRECATED: use isRequestDone instead
        mapping(uint256 keyId => bool isGenerated) _isKeyGenerated; // DEPRECATED
        /// @notice DEPRECATED
        uint256 _preKskgenRequestCounter; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 preKskgenRequestId => uint256 counter) _preKskgenResponseCounter; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 preKskId => bool isDone) _isPreKskgenDone; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 preKskId => mapping(address kmsConnector => bool hasResponded)) _preKskgenResponses; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 preKskId => bool isOngoing) _isKskgenOngoing; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 preKskId => uint256 sourceKeyId) _kskgenSourceKeyIds; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 preKskId => uint256 destKeyId) _kskgenDestKeyIds; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 kskId => mapping(address kmsConnector => bool hasResponded)) _kskgenResponses; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 kskId => uint256 responseCounter) _kskgenResponseCounter; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 kskId => bool isGenerated) _isKskGenerated; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 sourceKeyId => mapping(uint256 destKeyId => uint256 kskId)) _kskgenIds; // DEPRECATED
        /// @notice DEPRECATED: use crsCounter instead
        uint256 _crsgenRequestCounter; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 crsgenRequestId => uint256 counter) _crsgenResponseCounter; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 crsId => mapping(address kmsConnector => bool hasResponded)) _crsgenResponses; // DEPRECATED
        /// @notice DEPRECATED: use isRequestDone instead
        mapping(uint256 crsId => bool isGenerated) _isCrsGenerated; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 keyId => bool isOngoing) _activateKeyOngoing; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 keyId => mapping(address coprocessorConnector => bool hasResponded)) _activateKeyResponses; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 keyId => uint256 responseCounter) _activateKeyResponseCounter; // DEPRECATED
        /// @notice DEPRECATED: use activeKeyId instead
        uint256 currentKeyId; // DEPRECATED
        /// @notice DEPRECATED
        uint256[] activatedKeyIds; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 keyId => bytes32 paramsDigest) keyFheParamsDigests; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 kskId => bytes32 paramsDigest) kskFheParamsDigests; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 crsId => bytes32 paramsDigest) crsFheParamsDigests; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 preKeyId => bytes32 paramsDigest) _preKeyFheParamsDigests; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 preKeygenRequestId => bytes32 paramsDigest) _preKeygenFheParamsDigests; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 preKskId => bytes32 paramsDigest) _preKskFheParamsDigests; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 preKskgenRequestId => bytes32 paramsDigest) _preKskgenFheParamsDigests; // DEPRECATED
        /// @notice DEPRECATED
        mapping(uint256 crsgenRequestId => bytes32 paramsDigest) _crsgenFheParamsDigests; // DEPRECATED
        /// @notice DEPRECATED
        mapping(string fheParamsName => bool isInitialized) _fheParamsInitialized; // DEPRECATED
        // ----------------------------------------------------------------------------------------------
        // Pre-processing keygen state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The number of preprocessing keygen, used to generate the prepKeygenIds.
        uint256 prepKeygenCounter;
        // ----------------------------------------------------------------------------------------------
        // Keygen state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The number of keygen, used to generate the keyIds.
        uint256 keyCounter;
        /// @notice Bidirectional mapping between preprocessing request IDs and key IDs
        mapping(uint256 id => uint256 pairedId) keygenIdPairs;
        /// @notice The digests of the generated keys
        mapping(uint256 keyId => KeyDigest[] keyDigests) keyDigests;
        /// @notice The ID of the currently active key
        uint256 activeKeyId;
        // ----------------------------------------------------------------------------------------------
        // Crsgen state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The number of crsgen, used to generate the crsIds.
        uint256 crsCounter;
        /// @notice The max bit length used for the CRS generation
        mapping(uint256 crsId => uint256 maxBitLength) crsMaxBitLength;
        /// @notice The digests of the generated CRS
        mapping(uint256 crsId => bytes crsDigest) crsDigests;
        /// @notice The ID of the currently active CRS
        uint256 activeCrsId;
        // ----------------------------------------------------------------------------------------------
        // Common state variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The parameters type used for the request
        mapping(uint256 requestId => ParamsType paramsType) requestParamsType;
        /// @notice Whether a KMS node has signed for a response
        mapping(uint256 requestId => mapping(address kmsSigner => bool hasSigned)) kmsHasSignedForResponse;
        /// @notice Whether a request has reached consensus
        mapping(uint256 requestId => bool hasConsensusAlreadyBeenReached) isRequestDone;
        /// @notice The KMS transaction sender addresses that propagated valid signatures for a request
        mapping(uint256 requestId => mapping(bytes32 digest => address[] kmsTxSenderAddresses)) consensusTxSenderAddresses;
        /// @notice The KMS nodes' s3 bucket URL that propagated valid signatures for a request
        mapping(uint256 requestId => mapping(bytes32 digest => string[] kmsNodeS3BucketUrls)) consensusS3BucketUrls;
        /// @notice The digest of the signed struct on which consensus was reached for a request
        mapping(uint256 requestId => bytes32 digest) consensusDigest;
    }

    /**
     * @dev Storage location has been computed using the following command:
     * keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.KmsManagement")) - 1))
     * & ~bytes32(uint256(0xff))
     */
    bytes32 private constant KMS_MANAGEMENT_STORAGE_LOCATION =
        0xa48b77331ab977c487fc73c0afbe86f1a3a130c068453f497d376ab9fac7e000;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initializes the contract.
     * @dev This function needs to be public in order to be called by the UUPS proxy.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy() public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        __EIP712_init(CONTRACT_NAME, "1");
        __Ownable_init(owner());
        __Pausable_init();

        KmsManagementStorage storage $ = _getKmsManagementStorage();

        // Initialize the counters in order to generate globally unique requestIds per request type
        $.prepKeygenCounter = PREP_KEYGEN_COUNTER_BASE;
        $.keyCounter = KEY_COUNTER_BASE;
        $.crsCounter = CRS_COUNTER_BASE;
    }

    /**
     * @notice Re-initializes the contract from V1.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV2() public virtual reinitializer(REINITIALIZER_VERSION) {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        // Initialize the counters in order to generate globally unique requestIds per request type
        $.prepKeygenCounter = PREP_KEYGEN_COUNTER_BASE;
        $.keyCounter = KEY_COUNTER_BASE;
        $.crsCounter = CRS_COUNTER_BASE;
    }

    /**
     * @dev See {IKmsManagement-keygen}.
     */
    function keygenRequest(ParamsType paramsType) external virtual onlyGatewayOwner {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        // Generate a globally unique prepKeygenId for the key generation preprocessing
        // The counter is initialized at deployment such that prepKeygenId's first byte uniquely
        // represents a preprocessing keygen request, with format: [0000 0011 | counter_1..31]
        $.prepKeygenCounter++;
        uint256 prepKeygenId = $.prepKeygenCounter;

        // Generate a globally unique keyId for the key generation
        // The counter is initialized at deployment such that keyId's first byte uniquely
        // represents a keygen request, with format: [0000 0100 | counter_1..31]
        // We generate the keyId in the preprocessing step in order to anticipate the introduction
        // of key lifecycle: the keyId will be set to `Generating` status here
        // See https://github.com/zama-ai/fhevm-internal/issues/185
        $.keyCounter++;
        uint256 keyId = $.keyCounter;

        // Associate both the prepKeygenId and the keyId to each other in order to retrieve them later
        // Since IDs are globally unique, the IDs can't overlap and the same mapping can be used
        $.keygenIdPairs[prepKeygenId] = keyId;
        $.keygenIdPairs[keyId] = prepKeygenId;

        // TODO: Get the epochId once resharing is implemented.
        // See https://github.com/zama-ai/fhevm-internal/issues/151
        uint256 epochId = 0;

        // Store the FHE params type, used for both the preprocessing and the key generation
        // This value can later be read through the `getKeyParamsType` function, once the key
        // has been generated
        $.requestParamsType[prepKeygenId] = paramsType;

        emit PrepKeygenRequest(prepKeygenId, epochId, paramsType);
    }

    /**
     * @dev See {IKmsManagement-prepKeygenResponse}.
     */
    function prepKeygenResponse(uint256 prepKeygenId, bytes calldata signature) external virtual onlyKmsTxSender {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        // Compute the digest of the PrepKeygenVerification struct.
        bytes32 digest = _hashPrepKeygenVerification(prepKeygenId);

        // Recover the signer address from the signature and check that it is a KMS node
        address kmsSigner = _validateEIP712Signature(digest, signature);

        // Check that the signer has not already signed for this preprocessing keygen response
        if ($.kmsHasSignedForResponse[prepKeygenId][kmsSigner]) {
            revert KmsAlreadySignedForPrepKeygen(prepKeygenId, kmsSigner);
        }

        $.kmsHasSignedForResponse[prepKeygenId][kmsSigner] = true;

        // Store the KMS transaction sender address for the preprocessing keygen response
        // It is important to consider the same mapping fields used for the consensus
        // A "late" valid KMS transaction sender address will still be added in the list
        address[] storage consensusTxSenders = $.consensusTxSenderAddresses[prepKeygenId][digest];
        consensusTxSenders.push(msg.sender);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (!$.isRequestDone[prepKeygenId] && _isKmsConsensusReached(consensusTxSenders.length)) {
            $.isRequestDone[prepKeygenId] = true;

            // Store the digest on which consensus was reached for the preprocessing keygen request
            $.consensusDigest[prepKeygenId] = digest;

            // Get the keyId associated to the prepKeygenId
            uint256 keyId = $.keygenIdPairs[prepKeygenId];

            emit KeygenRequest(prepKeygenId, keyId);
        }
    }

    /**
     * @dev See {IKmsManagement-keygenResponse}.
     */
    function keygenResponse(
        uint256 keyId,
        KeyDigest[] calldata keyDigests,
        bytes calldata signature
    ) external virtual onlyKmsTxSender {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        // Get the prepKeygenId associated to the keyId
        uint256 prepKeygenId = $.keygenIdPairs[keyId];

        // Compute the digest of the KeygenVerification struct.
        bytes32 digest = _hashKeygenVerification(prepKeygenId, keyId, keyDigests);

        // Recover the signer address from the signature and check that it is a KMS node
        address kmsSigner = _validateEIP712Signature(digest, signature);

        // Check that the signer has not already signed for this key generation response
        if ($.kmsHasSignedForResponse[keyId][kmsSigner]) {
            revert KmsAlreadySignedForKeygen(keyId, kmsSigner);
        }

        $.kmsHasSignedForResponse[keyId][kmsSigner] = true;

        // Store the KMS transaction sender address and the s3 bucket URL for the keygen response,
        // event if the consensus has already been reached
        string[] memory consensusUrls = _storeConsensusMaterials(keyId, digest);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (!$.isRequestDone[keyId] && _isKmsConsensusReached(consensusUrls.length)) {
            $.isRequestDone[keyId] = true;

            // Store the digests of the generated keys in order to retrieve them later
            // Copy each calldata struct to storage, as copying calldata array of structs to storage
            // is not yet supported
            // We do not need to clean `$.keyDigests[keyId]` first as this should only happen once
            // per keyId
            for (uint256 i = 0; i < keyDigests.length; i++) {
                $.keyDigests[keyId].push(keyDigests[i]);
            }

            // Store the digest on which consensus was reached for the keygen request
            $.consensusDigest[keyId] = digest;

            // Set the active keyId
            $.activeKeyId = keyId;

            emit ActivateKey(keyId, consensusUrls, keyDigests);
        }
    }

    /**
     * @dev See {IKmsManagement-crsgenRequest}.
     */
    function crsgenRequest(uint256 maxBitLength, ParamsType paramsType) external virtual onlyGatewayOwner {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        // Generate a globally unique crsId for the CRS generation
        // The counter is initialized at deployment such that crsId's first byte uniquely
        // represents a crsgen request, with format: [0000 0101 | counter_1..31]
        $.crsCounter++;
        uint256 crsId = $.crsCounter;

        // Store the max bit length used for signature verification
        $.crsMaxBitLength[crsId] = maxBitLength;

        // Store the CRS params type
        // This value can later be read through the `getCrsParamsType` function, once the CRS has
        // been generated
        $.requestParamsType[crsId] = paramsType;

        emit CrsgenRequest(crsId, maxBitLength, paramsType);
    }

    /**
     * @dev See {IKmsManagement-crsgenResponse}.
     */
    function crsgenResponse(
        uint256 crsId,
        bytes calldata crsDigest,
        bytes calldata signature
    ) external virtual onlyKmsTxSender {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        uint256 maxBitLength = $.crsMaxBitLength[crsId];

        // Compute the digest of the CrsgenVerification struct.
        bytes32 digest = _hashCrsgenVerification(crsId, maxBitLength, crsDigest);

        // Recover the signer address from the signature and check that it is a KMS node
        address kmsSigner = _validateEIP712Signature(digest, signature);

        // Check that the signer has not already signed for this CRS generation response
        if ($.kmsHasSignedForResponse[crsId][kmsSigner]) {
            revert KmsAlreadySignedForCrsgen(crsId, kmsSigner);
        }

        $.kmsHasSignedForResponse[crsId][kmsSigner] = true;

        // Store the KMS transaction sender address and the s3 bucket URL for the crsgen response,
        // event if the consensus has already been reached
        string[] memory consensusUrls = _storeConsensusMaterials(crsId, digest);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (!$.isRequestDone[crsId] && _isKmsConsensusReached(consensusUrls.length)) {
            $.isRequestDone[crsId] = true;

            // Store the digest of the generated CRS in order to retrieve it later
            $.crsDigests[crsId] = crsDigest;

            // Store the digest on which consensus was reached for the crsgen request
            $.consensusDigest[crsId] = digest;

            // Set the active CRS ID
            $.activeCrsId = crsId;

            emit ActivateCrs(crsId, consensusUrls, crsDigest);
        }
    }

    /**
     * @dev See {IKmsManagement-getKeyParamsType}.
     */
    function getKeyParamsType(uint256 keyId) external view virtual returns (ParamsType) {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        if (!$.isRequestDone[keyId]) {
            revert KeyNotGenerated(keyId);
        }

        // Get the prepKeygenId associated to the keyId
        uint256 prepKeygenId = $.keygenIdPairs[keyId];

        return $.requestParamsType[prepKeygenId];
    }

    /**
     * @dev See {IKmsManagement-getCrsParamsType}.
     */
    function getCrsParamsType(uint256 crsId) external view virtual returns (ParamsType) {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        if (!$.isRequestDone[crsId]) {
            revert CrsNotGenerated(crsId);
        }

        return $.requestParamsType[crsId];
    }

    /**
     * @dev See {IKmsManagement-getActiveKeyId}.
     */
    function getActiveKeyId() external view virtual returns (uint256) {
        KmsManagementStorage storage $ = _getKmsManagementStorage();
        return $.activeKeyId;
    }

    /**
     * @dev See {IKmsManagement-getActiveCrsId}.
     */
    function getActiveCrsId() external view virtual returns (uint256) {
        KmsManagementStorage storage $ = _getKmsManagementStorage();
        return $.activeCrsId;
    }

    /**
     * @dev See {IKmsManagement-getConsensusTxSenders}.
     * The returned list remains empty until the consensus is reached.
     */
    function getConsensusTxSenders(uint256 requestId) external view virtual returns (address[] memory) {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        // Get the unique digest associated to the request in order to retrieve the list of
        // KMS transaction sender addresses that were involved in the associated consensus
        // This digest remains the default value (0x0) until the consensus is reached, meaning
        // that the returned list remains empty until then.
        // Each requestId is unique across all request types.
        bytes32 digest = $.consensusDigest[requestId];

        return $.consensusTxSenderAddresses[requestId][digest];
    }

    /**
     * @dev See {IKmsManagement-getKeyMaterials}.
     */
    function getKeyMaterials(uint256 keyId) external view virtual returns (string[] memory, KeyDigest[] memory) {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        if (!$.isRequestDone[keyId]) {
            revert KeyNotGenerated(keyId);
        }

        // Get the unique digest associated to the keygen request
        bytes32 digest = $.consensusDigest[keyId];

        return ($.consensusS3BucketUrls[keyId][digest], $.keyDigests[keyId]);
    }

    /**
     * @dev See {IKmsManagement-getCrsMaterials}.
     */
    function getCrsMaterials(uint256 crsId) external view virtual returns (string[] memory, bytes memory) {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        if (!$.isRequestDone[crsId]) {
            revert CrsNotGenerated(crsId);
        }

        // Get the unique digest associated to the crsgen request
        bytes32 digest = $.consensusDigest[crsId];

        return ($.consensusS3BucketUrls[crsId][digest], $.crsDigests[crsId]);
    }

    /**
     * @dev See {IKmsManagement-getVersion}.
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
     * @notice Validates the EIP712 signature.
     * @param digest The hashed EIP712 struct.
     * @param signature The signature to validate.
     * @return The signer address.
     */
    function _validateEIP712Signature(bytes32 digest, bytes calldata signature) internal virtual returns (address) {
        // Recover the signer address from the signature
        address signer = ECDSA.recover(digest, signature);

        // Check that the signer is a KMS signer
        GATEWAY_CONFIG.checkIsKmsSigner(signer);

        return signer;
    }

    /**
     * @notice Stores the KMS transaction sender address and the s3 bucket URL for the keygen response
     * @param requestId The ID of the request.
     * @param digest The digest of the request.
     * @return The list of s3 bucket URLs.
     */
    function _storeConsensusMaterials(uint256 requestId, bytes32 digest) internal virtual returns (string[] memory) {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        // Get the KMS node's s3 bucket URL
        string memory kmsNodeS3BucketUrl = GATEWAY_CONFIG.getKmsNode(msg.sender).s3BucketUrl;

        // Store the KMS transaction sender address and the s3 bucket URL for the keygen response
        // It is important to consider the same mapping fields used for the consensus
        // A "late" valid KMS transaction sender address or s3 bucket URL will still be added in the list
        address[] storage consensusTxSenders = $.consensusTxSenderAddresses[requestId][digest];
        consensusTxSenders.push(msg.sender);

        string[] storage consensusUrls = $.consensusS3BucketUrls[requestId][digest];
        consensusUrls.push(kmsNodeS3BucketUrl);

        return consensusUrls;
    }

    /**
     * @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyGatewayOwner {}

    /**
     * @notice Checks if the consensus is reached among the KMS nodes.
     * @param kmsCounter The number of KMS nodes that agreed
     * @return Whether the consensus is reached
     */
    function _isKmsConsensusReached(uint256 kmsCounter) internal view virtual returns (bool) {
        uint256 consensusThreshold = GATEWAY_CONFIG.getKmsStrongMajorityThreshold();
        return kmsCounter >= consensusThreshold;
    }

    /**
     * @notice Computes the hash of a PrepKeygenVerification struct
     * @param prepKeygenId The ID of the preprocessing keygen request.
     * @return The hash of the PrepKeygenVerification struct
     */
    function _hashPrepKeygenVerification(uint256 prepKeygenId) internal view virtual returns (bytes32) {
        return _hashTypedDataV4(keccak256(abi.encode(EIP712_PREP_KEYGEN_TYPE_HASH, prepKeygenId)));
    }

    /**
     * @notice Computes the hash of a KeygenVerification struct
     * @param prepKeygenId The ID of the preprocessing keygen request.
     * @param keyId The ID of the generated key.
     * @param keyDigests The digests of the generated keys.
     * @return The hash of the KeygenVerification struct
     */
    function _hashKeygenVerification(
        uint256 prepKeygenId,
        uint256 keyId,
        KeyDigest[] calldata keyDigests
    ) internal view virtual returns (bytes32) {
        // Encode each KeyDigest struct and compute its struct hash.
        bytes32[] memory keyDigestHashes = new bytes32[](keyDigests.length);
        for (uint256 i = 0; i < keyDigests.length; i++) {
            keyDigestHashes[i] = keccak256(
                abi.encode(EIP712_KEYDIGEST_TYPE_HASH, keyDigests[i].keyType, keccak256(keyDigests[i].digest))
            );
        }

        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        EIP712_KEYGEN_TYPE_HASH,
                        prepKeygenId,
                        keyId,
                        keccak256(abi.encodePacked(keyDigestHashes))
                    )
                )
            );
    }

    /**
     * @notice Computes the hash of a CrsgenVerification struct
     * @param crsId The ID of the generated CRS.
     * @param maxBitLength The max bit length used for generating the CRS.
     * @param crsDigest The digest of the generated CRS.
     * @return The hash of the CrsgenVerification struct
     */
    function _hashCrsgenVerification(
        uint256 crsId,
        uint256 maxBitLength,
        bytes calldata crsDigest
    ) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(EIP712_CRSGEN_TYPE_HASH, crsId, maxBitLength, keccak256(abi.encodePacked(crsDigest)))
                )
            );
    }

    /**
     * @dev Returns the KmsManagement storage location.
     * Note that this function is internal but not virtual: derived contracts should be able to
     * access it, but if the underlying storage struct version changes, we force them to define a new
     * getter function and use that one instead in order to avoid overriding the storage location.
     */
    function _getKmsManagementStorage() internal pure returns (KmsManagementStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := KMS_MANAGEMENT_STORAGE_LOCATION
        }
    }
}
