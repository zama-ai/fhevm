// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { IKMSGeneration } from "./interfaces/IKMSGeneration.sol";
import { IGatewayConfig } from "./interfaces/IGatewayConfig.sol";
import { gatewayConfigAddress } from "../addresses/GatewayAddresses.sol";
import { ECDSA } from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import { EIP712Upgradeable } from "@openzeppelin/contracts-upgradeable/utils/cryptography/EIP712Upgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { UUPSUpgradeableEmptyProxy } from "./shared/UUPSUpgradeableEmptyProxy.sol";
import { GatewayConfigChecks } from "./shared/GatewayConfigChecks.sol";
import { GatewayOwnable } from "./shared/GatewayOwnable.sol";
import {
    PREP_KEYGEN_COUNTER_BASE,
    KEY_COUNTER_BASE,
    CRS_COUNTER_BASE,
    KEY_RESHARE_COUNTER_BASE
} from "./shared/KMSRequestCounters.sol";

/**
 * @title KMSGeneration contract
 * @notice See {IKMSGeneration}.
 */
contract KMSGeneration is
    IKMSGeneration,
    EIP712Upgradeable,
    UUPSUpgradeableEmptyProxy,
    GatewayOwnable,
    GatewayConfigChecks
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
     * @dev The following fields are used for the KeyDigest struct:
     * - keyType: The type of the generated key.
     * - digest: The digest of the generated key.
     * Required because EIP-712 mandates that each nested struct type
     * used in a primary type (e.g. KeygenVerification) must be explicitly
     * declared with its own type string and type hash.
     * These constants are used when computing the struct hash of each
     * KeyDigest element inside the keyDigests[] array.
     */
    string private constant EIP712_KEY_DIGEST_TYPE = "KeyDigest(uint8 keyType,bytes digest)";
    bytes32 private constant EIP712_KEY_DIGEST_TYPE_HASH = keccak256(bytes(EIP712_KEY_DIGEST_TYPE));

    /**
     * @notice The KeygenVerification typed definition.
     * @dev The following fields are used for the KeygenVerification struct:
     * - prepKeygenId: The ID of the preprocessing keygen request.
     * - keyId: The ID of the generated key.
     * - keyDigests: The digests of the generated key.
     */
    string private constant EIP712_KEYGEN_TYPE =
        "KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests)KeyDigest(uint8 keyType,bytes digest)";

    /**
     * @notice The hash of the KeygenVerification typed definition.
     */
    bytes32 private constant EIP712_KEYGEN_TYPE_HASH = keccak256(bytes(EIP712_KEYGEN_TYPE));

    /**
     * @notice The CrsgenVerification typed definition.
     * @dev The following fields are used for the CrsgenVerification struct:
     * - crsId: The ID of the generated CRS.
     * - maxBitLength: The max bit length of the generated CRS.
     * - crsDigest: The digest of the generated CRS.
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
    string private constant CONTRACT_NAME = "KMSGeneration";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 3;
    uint256 private constant PATCH_VERSION = 0;

    /**
     * @dev Constant used for making sure the version number using in the `reinitializer` modifier
     * is identical between `initializeFromEmptyProxy` and the reinitializeVX` method
     */
    uint64 private constant REINITIALIZER_VERSION = 4;

    // ----------------------------------------------------------------------------------------------
    // Contract storage:
    // ----------------------------------------------------------------------------------------------

    /**
     * @notice The contract's variable storage struct (@dev see ERC-7201)
     */
    /// @custom:storage-location erc7201:fhevm_gateway.storage.KMSGeneration
    struct KMSGenerationStorage {
        // ----------------------------------------------------------------------------------------------
        // Common consensus variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice Whether a KMS node has signed for a response
        mapping(uint256 requestId => mapping(address kmsSigner => bool hasSigned)) kmsHasSignedForResponse;
        /// @notice Whether a request has reached consensus
        mapping(uint256 requestId => bool hasConsensusAlreadyBeenReached) isRequestDone;
        /// @notice The KMS transaction sender addresses that propagated valid signatures for a request
        mapping(uint256 requestId => mapping(bytes32 digest => address[] kmsTxSenderAddresses)) consensusTxSenderAddresses;
        /// @notice The digest of the signed struct on which consensus was reached for a request
        mapping(uint256 requestId => bytes32 digest) consensusDigest;
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
        // Parameters variables:
        // ----------------------------------------------------------------------------------------------
        /// @notice The parameters type used for the request
        mapping(uint256 requestId => ParamsType paramsType) requestParamsType;
        /// @notice The number of key resharing, used to generate the keyReshareIds.
        uint256 keyReshareCounter;
    }

    /**
     * @dev Storage location has been computed using the following command:
     * keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.KMSGeneration")) - 1))
     * & ~bytes32(uint256(0xff))
     */
    bytes32 private constant KMS_GENERATION_STORAGE_LOCATION =
        0x0b8fdb1f0a6356dd20a6cbc6f9668fac23b85f96575d10e333e603faa794ac00;

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

        KMSGenerationStorage storage $ = _getKMSGenerationStorage();

        // Initialize the counters in order to generate globally unique requestIds per request type
        $.prepKeygenCounter = PREP_KEYGEN_COUNTER_BASE;
        $.keyCounter = KEY_COUNTER_BASE;
        $.crsCounter = CRS_COUNTER_BASE;
        $.keyReshareCounter = KEY_RESHARE_COUNTER_BASE;
    }

    /**
     * @notice Re-initializes the contract from V2.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV3() public virtual reinitializer(REINITIALIZER_VERSION) {}

    /**
     * @notice See {IKMSGeneration-keygen}.
     */
    function keygen(ParamsType paramsType) external virtual onlyGatewayOwner {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();

        // Check that the previous keygen request has reached consensus
        // Exception for the first keygen request, which has no previous key (counter is KEY_COUNTER_BASE)
        uint256 previousKeyId = $.keyCounter;
        if (previousKeyId != KEY_COUNTER_BASE && !$.isRequestDone[previousKeyId]) {
            revert KeygenOngoing(previousKeyId);
        }

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
     * @notice See {IKMSGeneration-prepKeygenResponse}.
     */
    function prepKeygenResponse(uint256 prepKeygenId, bytes calldata signature) external virtual onlyKmsTxSender {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();

        // Make sure the prepKeygenId corresponds to a generated preprocessing keygen request.
        if (prepKeygenId > $.prepKeygenCounter || prepKeygenId == 0) {
            revert PrepKeygenNotRequested(prepKeygenId);
        }

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

        // Emit the event at each call for monitoring purposes.
        emit PrepKeygenResponse(prepKeygenId, signature, msg.sender);

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
     * @notice See {IKMSGeneration-keygenResponse}.
     */
    function keygenResponse(
        uint256 keyId,
        KeyDigest[] calldata keyDigests,
        bytes calldata signature
    ) external virtual onlyKmsTxSender {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();

        // Make sure the keyId corresponds to a generated keygen request.
        if (keyId > $.keyCounter || keyId == 0) {
            revert KeygenNotRequested(keyId);
        }

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

        // Store the KMS transaction sender address for the keygen response
        // A "late" valid KMS transaction sender address or storage URL will still be added in the list
        address[] storage consensusTxSenders = $.consensusTxSenderAddresses[keyId][digest];
        consensusTxSenders.push(msg.sender);

        uint256 consensusTxSendersLength = consensusTxSenders.length;

        // Emit the event at each call for monitoring purposes.
        emit KeygenResponse(keyId, keyDigests, signature, msg.sender);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (!$.isRequestDone[keyId] && _isKmsConsensusReached(consensusTxSendersLength)) {
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
            string[] memory consensusUrls = new string[](consensusTxSendersLength);
            for (uint256 i = 0; i < consensusTxSendersLength; i++) {
                consensusUrls[i] = GATEWAY_CONFIG.getKmsNode(consensusTxSenders[i]).storageUrl;
            }

            emit ActivateKey(keyId, consensusUrls, keyDigests);
        }
    }

    /**
     * @notice See {IKMSGeneration-crsgenRequest}.
     */
    function crsgenRequest(uint256 maxBitLength, ParamsType paramsType) external virtual onlyGatewayOwner {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();

        // Check that the previous CRS generation request has reached consensus
        // Exception for the first CRS generation request, which has no previous CRS (counter is CRS_COUNTER_BASE)
        uint256 previousCrsId = $.crsCounter;
        if (previousCrsId != CRS_COUNTER_BASE && !$.isRequestDone[previousCrsId]) {
            revert CrsgenOngoing(previousCrsId);
        }

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
     * @notice See {IKMSGeneration-crsgenResponse}.
     */
    function crsgenResponse(
        uint256 crsId,
        bytes calldata crsDigest,
        bytes calldata signature
    ) external virtual onlyKmsTxSender {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();

        // Make sure the crsId corresponds to a generated CRS generation request.
        if (crsId > $.crsCounter || crsId == 0) {
            revert CrsgenNotRequested(crsId);
        }

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

        // Store the KMS transaction sender address for the crsgen response
        // A "late" valid KMS transaction sender address or storage URL will still be added in the list
        address[] storage consensusTxSenders = $.consensusTxSenderAddresses[crsId][digest];
        consensusTxSenders.push(msg.sender);

        uint256 consensusTxSendersLength = consensusTxSenders.length;

        // Emit the event at each call for monitoring purposes.
        emit CrsgenResponse(crsId, crsDigest, signature, msg.sender);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (!$.isRequestDone[crsId] && _isKmsConsensusReached(consensusTxSendersLength)) {
            $.isRequestDone[crsId] = true;

            // Store the digest of the generated CRS in order to retrieve it later
            $.crsDigests[crsId] = crsDigest;

            // Store the digest on which consensus was reached for the crsgen request
            $.consensusDigest[crsId] = digest;

            // Set the active CRS ID
            $.activeCrsId = crsId;

            string[] memory consensusUrls = new string[](consensusTxSendersLength);
            for (uint256 i = 0; i < consensusTxSendersLength; i++) {
                consensusUrls[i] = GATEWAY_CONFIG.getKmsNode(consensusTxSenders[i]).storageUrl;
            }
            emit ActivateCrs(crsId, consensusUrls, crsDigest);
        }
    }

    /**
     * @notice See {IKMSGeneration-prssInit}.
     */
    function prssInit() external virtual onlyGatewayOwner {
        emit PRSSInit();
    }

    /**
     * @notice See {IKMSGeneration-keyReshareSameSet}.
     * @dev ⚠️ This function should only be called under exceptional circumstances.
     * It is intended for corrective flows when a previous resharing attempt failed.
     * Use with caution since incorrect usage may cause inconsistent key generation states.
     */
    function keyReshareSameSet(uint256 keyId) external virtual onlyGatewayOwner {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();

        if (!$.isRequestDone[keyId]) {
            revert KeyNotGenerated(keyId);
        }

        // Get the prepKeygenId associated to the keyId and its params type.
        uint256 prepKeygenId = $.keygenIdPairs[keyId];
        ParamsType paramsType = $.requestParamsType[prepKeygenId];

        // Generate a globally unique keyReshareId for the key resharing.
        // The counter is initialized at deployment such that keyReshareId's first byte uniquely
        // represents a key reshare request, with format: [0000 0110 | counter_1..31]
        $.keyReshareCounter++;
        uint256 keyReshareId = $.keyReshareCounter;

        emit KeyReshareSameSet(prepKeygenId, keyId, keyReshareId, paramsType);
    }

    /**
     * @notice See {IKMSGeneration-getKeyParamsType}.
     */
    function getKeyParamsType(uint256 keyId) external view virtual returns (ParamsType) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();

        if (!$.isRequestDone[keyId]) {
            revert KeyNotGenerated(keyId);
        }

        // Get the prepKeygenId associated to the keyId
        uint256 prepKeygenId = $.keygenIdPairs[keyId];

        return $.requestParamsType[prepKeygenId];
    }

    /**
     * @notice See {IKMSGeneration-getCrsParamsType}.
     */
    function getCrsParamsType(uint256 crsId) external view virtual returns (ParamsType) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();

        if (!$.isRequestDone[crsId]) {
            revert CrsNotGenerated(crsId);
        }

        return $.requestParamsType[crsId];
    }

    /**
     * @notice See {IKMSGeneration-getActiveKeyId}.
     */
    function getActiveKeyId() external view virtual returns (uint256) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();
        return $.activeKeyId;
    }

    /**
     * @notice See {IKMSGeneration-getActiveCrsId}.
     */
    function getActiveCrsId() external view virtual returns (uint256) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();
        return $.activeCrsId;
    }

    /**
     * @notice See {IKMSGeneration-getConsensusTxSenders}.
     * The returned list remains empty until the consensus is reached.
     */
    function getConsensusTxSenders(uint256 requestId) external view virtual returns (address[] memory) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();

        // Get the unique digest associated to the request in order to retrieve the list of
        // KMS transaction sender addresses that were involved in the associated consensus
        // This digest remains the default value (0x0) until the consensus is reached, meaning
        // that the returned list remains empty until then.
        // Each requestId is unique across all request types.
        bytes32 digest = $.consensusDigest[requestId];

        return $.consensusTxSenderAddresses[requestId][digest];
    }

    /**
     * @notice See {IKMSGeneration-getKeyMaterials}.
     */
    function getKeyMaterials(uint256 keyId) external view virtual returns (string[] memory, KeyDigest[] memory) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();
        if (!$.isRequestDone[keyId]) {
            revert KeyNotGenerated(keyId);
        }
        bytes32 digest = $.consensusDigest[keyId];
        address[] memory consensusTxSenders = $.consensusTxSenderAddresses[keyId][digest];
        uint256 consensusTxSendersLength = consensusTxSenders.length;

        string[] memory consensusUrls = new string[](consensusTxSendersLength);
        for (uint256 i = 0; i < consensusTxSendersLength; i++) {
            consensusUrls[i] = GATEWAY_CONFIG.getKmsNode(consensusTxSenders[i]).storageUrl;
        }

        return (consensusUrls, $.keyDigests[keyId]);
    }

    /**
     * @notice See {IKMSGeneration-getCrsMaterials}.
     */
    function getCrsMaterials(uint256 crsId) external view virtual returns (string[] memory, bytes memory) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();
        if (!$.isRequestDone[crsId]) {
            revert CrsNotGenerated(crsId);
        }
        bytes32 digest = $.consensusDigest[crsId];
        address[] memory consensusTxSenders = $.consensusTxSenderAddresses[crsId][digest];
        uint256 consensusTxSendersLength = consensusTxSenders.length;

        string[] memory consensusUrls = new string[](consensusTxSendersLength);
        for (uint256 i = 0; i < consensusTxSendersLength; i++) {
            consensusUrls[i] = GATEWAY_CONFIG.getKmsNode(consensusTxSenders[i]).storageUrl;
        }

        return (consensusUrls, $.crsDigests[crsId]);
    }

    /**
     * @notice See {IKMSGeneration-getVersion}.
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

        // Check that the signer is a KMS signer, and that it corresponds to the transaction sender of the same KMS node.
        _checkKmsSignerMatchesTxSender(signer, msg.sender);

        return signer;
    }

    /**
     * @notice Checks if the sender is authorized to upgrade the contract and reverts otherwise.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyGatewayOwner {}

    /**
     * @notice Checks if the consensus is reached among the KMS nodes.
     * @param kmsCounter The number of KMS nodes that agreed
     * @return Whether the consensus is reached
     */
    function _isKmsConsensusReached(uint256 kmsCounter) internal view virtual returns (bool) {
        uint256 consensusThreshold = GATEWAY_CONFIG.getKmsGenThreshold();
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
        // Encodes each KeyDigest struct and computes its struct hash.
        // The `keyDigests` array must be ordered consistently with the KMS nodes:
        // the first element corresponds to the Server type, and the second to the Public type.
        bytes32[] memory keyDigestHashes = new bytes32[](keyDigests.length);
        for (uint256 i = 0; i < keyDigests.length; i++) {
            keyDigestHashes[i] = keccak256(
                abi.encode(EIP712_KEY_DIGEST_TYPE_HASH, keyDigests[i].keyType, keccak256(keyDigests[i].digest))
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
     * @notice Returns the KMSGeneration storage location.
     * @dev Note that this function is internal but not virtual: derived contracts should be able to
     * access it, but if the underlying storage struct version changes, we force them to define a new
     * getter function and use that one instead in order to avoid overriding the storage location.
     */
    function _getKMSGenerationStorage() internal pure returns (KMSGenerationStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := KMS_GENERATION_STORAGE_LOCATION
        }
    }
}
