// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {IKMSGeneration} from "./interfaces/IKMSGeneration.sol";
import {IProtocolConfig} from "./interfaces/IProtocolConfig.sol";
import {KmsNode} from "./shared/Structs.sol";
import {PREP_KEYGEN_COUNTER_BASE, KEY_COUNTER_BASE, CRS_COUNTER_BASE, EXTRA_DATA_V1, EXTRA_DATA_V2} from "./shared/Constants.sol";
import {protocolConfigAdd} from "../addresses/FHEVMHostAddresses.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {EIP712Upgradeable} from "@openzeppelin/contracts-upgradeable/utils/cryptography/EIP712Upgradeable.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";

/**
 * @title KMSGeneration contract
 * @notice See {IKMSGeneration}.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract KMSGeneration is IKMSGeneration, EIP712Upgradeable, UUPSUpgradeableEmptyProxy, ACLOwnable {
    // ----------------------------------------------------------------------------------------------
    // EIP712 utility constants:
    // ----------------------------------------------------------------------------------------------

    /**
     * @notice The PrepKeygenVerification typed definition.
     * @dev prepKeygenId: The ID of the preprocessing keygen request.
     *      extraData: Additional context data.
     */
    string private constant EIP712_PREP_KEYGEN_TYPE = "PrepKeygenVerification(uint256 prepKeygenId,bytes extraData)";

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
     * - extraData: Additional context data.
     */
    string private constant EIP712_KEYGEN_TYPE =
        "KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)";

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
     * - extraData: Additional context data.
     */
    string private constant EIP712_CRSGEN_TYPE =
        "CrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest,bytes extraData)";

    /**
     * @notice The hash of the CrsgenVerification typed definition.
     */
    bytes32 private constant EIP712_CRSGEN_TYPE_HASH = keccak256(bytes(EIP712_CRSGEN_TYPE));

    // ----------------------------------------------------------------------------------------------
    // Other contract references:
    // ----------------------------------------------------------------------------------------------

    /**
     * @notice The address of the ProtocolConfig contract for protocol state calls.
     */
    IProtocolConfig private constant PROTOCOL_CONFIG = IProtocolConfig(protocolConfigAdd);

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
    uint256 private constant MINOR_VERSION = 2;
    uint256 private constant PATCH_VERSION = 0;

    /// @dev RFC-029 is a one-time legacy -> migrated cutover, so the only valid migrated material
    /// version is 1. addKeyMaterials / scheduleKeyMaterialMigration reject anything else.
    uint256 private constant MIGRATED_MATERIAL_VERSION = 1;

    /**
     * @dev Constant used for making sure the version number used in the `reinitializer` modifier
     * is identical between `initializeFromEmptyProxy` and `reinitializeV2`.
     */
    uint64 private constant REINITIALIZER_VERSION = 3;

    // ----------------------------------------------------------------------------------------------
    // Contract storage:
    // ----------------------------------------------------------------------------------------------

    /**
     * @notice The contract's variable storage struct (@dev see ERC-7201)
     */
    /// @custom:storage-location erc7201:fhevm.storage.KMSGeneration
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
        /// @notice Request extra data: v1 for migrated/imported state, v2 for new requests.
        mapping(uint256 requestId => bytes extraData) requestExtraData;
        /// @notice Key IDs that reached consensus.
        uint256[] completedKeyIds;
        /// @notice CRS IDs that reached consensus.
        uint256[] completedCrsIds;
        // ----------------------------------------------------------------------------------------------
        // RFC-029 material-migration state (append-only):
        // ----------------------------------------------------------------------------------------------
        /// @notice Migration keygens (keygen-from-existing), keyed by the freshly generated keyId.
        /// Presence (non-zero value) IS the "is a migration" flag: prepKeygenResponse emits the
        /// typed MigrationKeygenRequest in place of KeygenRequest, and keygenResponse
        /// publishes-not-activates (the throwaway key is never activated). The migrated material is
        /// published under the existing key by governance (addKeyMaterials).
        mapping(uint256 keyId => uint256 existingKeyId) migrationKeygens;
        /// @notice For an existing key, the migration keygen its published migrated material came from
        /// (0 = none published). Presence marks that v1 material exists under the key.
        mapping(uint256 existingKeyId => uint256 migrationKeyId) publishedFrom;
        /// @notice Whether the one-time material-version cutover was already scheduled for a key.
        /// Enforces single-assignment of the schedule (a second scheduleKeyMaterialMigration reverts).
        mapping(uint256 keyId => bool isScheduled) migrationScheduled;
    }

    /**
     * @dev Storage location has been computed using the following command:
     * keccak256(abi.encode(uint256(keccak256("fhevm.storage.KMSGeneration")) - 1))
     * & ~bytes32(uint256(0xff))
     */
    bytes32 private constant KMS_GENERATION_STORAGE_LOCATION =
        0x26fdaf8a2cb20d20b55e36218986905e534ee7a970dd2fa827946e4b7496db00;

    /**
     * @notice Loads a request's pinned context and authorizes the response sender against it.
     * @dev Uses the request-time context, not the latest active context, so rotations do not invalidate
     * in-flight responses from the original KMS committee.
     */
    function _loadExtraDataAndAuthorizeResponse(
        uint256 requestId
    ) internal view virtual returns (bytes memory extraData, uint256 contextId) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();
        extraData = $.requestExtraData[requestId];
        contextId = _extractContextIdFromExtraData(extraData);
        if (!PROTOCOL_CONFIG.isKmsTxSenderForContext(contextId, msg.sender)) {
            revert NotKmsTxSender(msg.sender);
        }
    }

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
    }

    /**
     * @notice Re-initializes the contract from V1.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV2() public virtual reinitializer(REINITIALIZER_VERSION) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();

        // Backfill completed key/CRS IDs from pre-V2 state. These arrays were added in V2: existing
        // deployments reached consensus without populating them. "Completed" means consensus was
        // reached, which is recorded by a non-zero consensusDigest, unlike `isRequestDone`, which is
        // also set on abort (see abortKeygen/abortCrsgen).
        for (uint256 keyId = KEY_COUNTER_BASE + 1; keyId <= $.keyCounter; keyId++) {
            if ($.consensusDigest[keyId] != bytes32(0)) {
                $.completedKeyIds.push(keyId);
            }
        }
        for (uint256 crsId = CRS_COUNTER_BASE + 1; crsId <= $.crsCounter; crsId++) {
            if ($.consensusDigest[crsId] != bytes32(0)) {
                $.completedCrsIds.push(crsId);
            }
        }
    }

    /**
     * @notice See {IKMSGeneration-keygen}.
     */
    function keygen(ParamsType paramsType) external virtual onlyACLOwner {
        _keygen(paramsType);
    }

    /**
     * @notice See {IKMSGeneration-migrationKeygen}.
     * @dev RFC-029: a keygen-from-existing that re-derives `existingKeyId`'s material in the migrated
     * (CompressedXofKeySet) format. On-chain it is an ordinary keygen (normal v2 extraData, signed by
     * the KMS as usual); recording it in `migrationKeygens` flags it so prepKeygenResponse emits the
     * typed MigrationKeygenRequest (in place of KeygenRequest) and keygenResponse
     * publishes-not-activates. The connector branches on that event to drive the keygen-from-existing
     * (UseExisting + copy-to-original) on the KMS.
     */
    function migrationKeygen(ParamsType paramsType, uint256 existingKeyId) external virtual onlyACLOwner {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();
        // The key being migrated must already exist (reached consensus).
        if (!$.isRequestDone[existingKeyId]) {
            revert KeygenNotRequested(existingKeyId);
        }
        (, uint256 keyId) = _keygen(paramsType);
        $.migrationKeygens[keyId] = existingKeyId;
    }

    /// @dev Shared keygen-request body: allocates prepKeygenId/keyId, pins the v2 (context+epoch)
    /// extraData, and emits PrepKeygenRequest. Returns the ids so callers can annotate the request
    /// (e.g. migrationKeygen flagging it as a migration).
    function _keygen(ParamsType paramsType) internal virtual returns (uint256 prepKeygenId, uint256 keyId) {
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
        prepKeygenId = $.prepKeygenCounter;

        // Generate a globally unique keyId for the key generation
        // The counter is initialized at deployment such that keyId's first byte uniquely
        // represents a keygen request, with format: [0000 0100 | counter_1..31]
        // We generate the keyId in the preprocessing step in order to anticipate the introduction
        // of key lifecycle: the keyId will be set to `Generating` status here
        // See https://github.com/zama-ai/fhevm-internal/issues/185
        $.keyCounter++;
        keyId = $.keyCounter;

        // Associate both the prepKeygenId and the keyId to each other in order to retrieve them later
        // Since IDs are globally unique, the IDs can't overlap and the same mapping can be used
        $.keygenIdPairs[prepKeygenId] = keyId;
        $.keygenIdPairs[keyId] = prepKeygenId;

        // Store the FHE params type, used for both the preprocessing and the key generation
        // This value can later be read through the `getKeyParamsType` function, once the key
        // has been generated
        $.requestParamsType[prepKeygenId] = paramsType;

        (uint256 contextId, uint256 epochId) = PROTOCOL_CONFIG.getCurrentKmsContextAndEpoch();
        bytes memory extraData = _encodeRequestExtraDataV2(contextId, epochId);
        $.requestExtraData[prepKeygenId] = extraData;
        $.requestExtraData[keyId] = extraData;

        emit PrepKeygenRequest(prepKeygenId, paramsType, extraData);
    }

    /**
     * @notice See {IKMSGeneration-prepKeygenResponse}.
     */
    function prepKeygenResponse(uint256 prepKeygenId, bytes calldata signature) external virtual {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();

        // Make sure the prepKeygenId corresponds to a generated preprocessing keygen request.
        if (prepKeygenId > $.prepKeygenCounter || prepKeygenId <= PREP_KEYGEN_COUNTER_BASE) {
            revert PrepKeygenNotRequested(prepKeygenId);
        }

        (bytes memory extraData, uint256 contextId) = _loadExtraDataAndAuthorizeResponse(prepKeygenId);

        // Compute the digest of the PrepKeygenVerification struct.
        bytes32 digest = _hashPrepKeygenVerification(prepKeygenId, extraData);

        // Recover the signer address from the signature and check that it is a KMS node.
        address kmsSigner = _validateEIP712Signature(contextId, digest, signature);

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
        if (!$.isRequestDone[prepKeygenId] && _isKmsConsensusReachedForContext(contextId, consensusTxSenders.length)) {
            $.isRequestDone[prepKeygenId] = true;

            // Store the digest on which consensus was reached for the preprocessing keygen request
            $.consensusDigest[prepKeygenId] = digest;

            // Get the keyId associated to the prepKeygenId
            uint256 keyId = $.keygenIdPairs[prepKeygenId];

            // A migration keygen emits the typed MigrationKeygenRequest IN PLACE OF KeygenRequest, so
            // the connector branches on the event type with no side table and no path where a migration
            // silently runs as a normal keygen.
            uint256 existingKeyId = $.migrationKeygens[keyId];
            if (existingKeyId != 0) {
                emit MigrationKeygenRequest(prepKeygenId, keyId, existingKeyId, extraData);
            } else {
                emit KeygenRequest(prepKeygenId, keyId, extraData);
            }
        }
    }

    /**
     * @notice See {IKMSGeneration-keygenResponse}.
     */
    function keygenResponse(uint256 keyId, KeyDigest[] calldata keyDigests, bytes calldata signature) external virtual {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();

        // Make sure the keyId corresponds to a generated keygen request.
        if (keyId > $.keyCounter || keyId <= KEY_COUNTER_BASE) {
            revert KeygenNotRequested(keyId);
        }

        // Make sure the keygen response contains at least one key digest as keygen flow will always
        // generate at least one key
        if (keyDigests.length == 0) {
            revert EmptyKeyDigests(keyId);
        }

        (bytes memory extraData, uint256 contextId) = _loadExtraDataAndAuthorizeResponse(keyId);

        uint256 prepKeygenId = $.keygenIdPairs[keyId];
        if (!$.isRequestDone[prepKeygenId]) {
            revert KeyManagementRequestPending();
        }

        // Compute the digest of the KeygenVerification struct.
        bytes32 digest = _hashKeygenVerification(prepKeygenId, keyId, keyDigests, extraData);

        // Recover the signer address from the signature and check that it is a KMS node.
        address kmsSigner = _validateEIP712Signature(contextId, digest, signature);

        // Check that the signer has not already signed for this key generation response
        if ($.kmsHasSignedForResponse[keyId][kmsSigner]) {
            revert KmsAlreadySignedForKeygen(keyId, kmsSigner);
        }

        $.kmsHasSignedForResponse[keyId][kmsSigner] = true;

        // Store the KMS transaction sender address for the keygen response
        // A "late" valid KMS transaction sender address or storage URL will still be added in the list
        address[] storage consensusTxSenders = $.consensusTxSenderAddresses[keyId][digest];
        consensusTxSenders.push(msg.sender);

        // Emit the event at each call for monitoring purposes.
        emit KeygenResponse(keyId, keyDigests, signature, msg.sender);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (!$.isRequestDone[keyId] && _isKmsConsensusReachedForContext(contextId, consensusTxSenders.length)) {
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
            $.completedKeyIds.push(keyId);

            // RFC-029 publish-not-activate: a migration keygen (keygen-from-existing) is NOT
            // activated -- its freshly generated key is a throwaway used only to produce the
            // migrated material; that material is published under the EXISTING key by governance
            // (addKeyMaterials). So skip moving activeKeyId / emitting ActivateKey for migrations.
            if ($.migrationKeygens[keyId] == 0) {
                $.activeKeyId = keyId;
                string[] memory consensusUrls = _buildConsensusStorageUrls(contextId, consensusTxSenders);
                emit ActivateKey(keyId, consensusUrls, keyDigests);
            }
        }
    }

    /**
     * @notice See {IKMSGeneration-addKeyMaterials}.
     * @dev RFC-029 governance publish-not-activate: publishes migrated material under an EXISTING key
     * (NEVER moves activeKeyId) and emits KeyMaterialAdded for the coprocessor host-listener to
     * download. Bound to its source migration keygen via `migrationKeyId`: rejects anything that is
     * not a completed migration keygen for `existingKeyId`. No KMS signature is verified here --
     * governance is the ACL owner; the digests it supplies come from the (KMS-attested) migration
     * keygen result.
     */
    function addKeyMaterials(
        uint256 existingKeyId,
        uint256 migrationKeyId,
        KeyDigest[] calldata keyDigests,
        string[] calldata kmsNodeStorageUrls
    ) external virtual onlyACLOwner {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();
        // Bind the published material to a migration keygen that targeted exactly this existing key.
        if ($.migrationKeygens[migrationKeyId] != existingKeyId) {
            revert MigrationKeyNotForExistingKey(migrationKeyId, existingKeyId);
        }
        // ...and that the migration keygen has actually completed (its material exists).
        if (!$.isRequestDone[migrationKeyId]) {
            revert KeyManagementRequestPending();
        }
        // Single-assignment: the one-time cutover publishes exactly once per key.
        if ($.publishedFrom[existingKeyId] != 0) {
            revert KeyMaterialAlreadyPublished(existingKeyId);
        }
        if (keyDigests.length == 0) {
            revert EmptyKeyDigests(existingKeyId);
        }
        // Storage URLs are how the coprocessor downloads the migrated material; without them the key
        // would read as migrated while no node can fetch it (post-cutover halt-and-retry forever).
        if (kmsNodeStorageUrls.length == 0) {
            revert EmptyStorageUrls(existingKeyId);
        }

        $.publishedFrom[existingKeyId] = migrationKeyId;
        emit KeyMaterialAdded(existingKeyId, kmsNodeStorageUrls, keyDigests);
    }

    /**
     * @notice See {IKMSGeneration-scheduleKeyMaterialMigration}.
     */
    function scheduleKeyMaterialMigration(
        uint256 keyId,
        uint256[] calldata hostChainIds,
        uint256[] calldata hostMigrationBlocks,
        uint256 gatewayMigrationBlock
    ) external virtual onlyACLOwner {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();
        if (hostChainIds.length != hostMigrationBlocks.length) {
            revert MismatchedMigrationArrays();
        }
        // The migrated material must already be published under this key, else the cutover would
        // point coprocessors at material that does not exist (halt-and-retry forever).
        if ($.publishedFrom[keyId] == 0) {
            revert KeyMaterialNotPublished(keyId);
        }
        // Single-assignment: the one-time cutover is scheduled exactly once. Re-scheduling could
        // rewrite the cutover blocks under a fleet that already crossed them.
        if ($.migrationScheduled[keyId]) {
            revert MigrationAlreadyScheduled(keyId);
        }
        $.migrationScheduled[keyId] = true;

        emit KeyMaterialMigrationScheduled(
            keyId,
            hostChainIds,
            hostMigrationBlocks,
            gatewayMigrationBlock
        );
    }

    /**
     * @notice See {IKMSGeneration-crsgenRequest}.
     */
    function crsgenRequest(uint256 maxBitLength, ParamsType paramsType) external virtual onlyACLOwner {
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

        (uint256 contextId, uint256 epochId) = PROTOCOL_CONFIG.getCurrentKmsContextAndEpoch();
        bytes memory extraData = _encodeRequestExtraDataV2(contextId, epochId);
        $.requestExtraData[crsId] = extraData;

        emit CrsgenRequest(crsId, maxBitLength, paramsType, extraData);
    }

    /**
     * @notice See {IKMSGeneration-crsgenResponse}.
     */
    function crsgenResponse(uint256 crsId, bytes calldata crsDigest, bytes calldata signature) external virtual {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();

        // Make sure the crsId corresponds to a generated CRS generation request.
        if (crsId > $.crsCounter || crsId <= CRS_COUNTER_BASE) {
            revert CrsgenNotRequested(crsId);
        }

        (bytes memory extraData, uint256 contextId) = _loadExtraDataAndAuthorizeResponse(crsId);

        // Compute the digest of the CrsgenVerification struct.
        bytes32 digest = _hashCrsgenVerification(crsId, $.crsMaxBitLength[crsId], crsDigest, extraData);

        // Recover the signer address from the signature and check that it is a KMS node.
        address kmsSigner = _validateEIP712Signature(contextId, digest, signature);

        // Check that the signer has not already signed for this CRS generation response
        if ($.kmsHasSignedForResponse[crsId][kmsSigner]) {
            revert KmsAlreadySignedForCrsgen(crsId, kmsSigner);
        }

        $.kmsHasSignedForResponse[crsId][kmsSigner] = true;

        // Store the KMS transaction sender address for the crsgen response
        // A "late" valid KMS transaction sender address or storage URL will still be added in the list
        address[] storage consensusTxSenders = $.consensusTxSenderAddresses[crsId][digest];
        consensusTxSenders.push(msg.sender);

        // Emit the event at each call for monitoring purposes.
        emit CrsgenResponse(crsId, crsDigest, signature, msg.sender);

        // Send the event if and only if the consensus is reached in the current response call.
        // This means a "late" response will not be reverted, just ignored and no event will be emitted
        if (!$.isRequestDone[crsId] && _isKmsConsensusReachedForContext(contextId, consensusTxSenders.length)) {
            $.isRequestDone[crsId] = true;

            // Store the digest of the generated CRS in order to retrieve it later
            $.crsDigests[crsId] = crsDigest;

            // Store the digest on which consensus was reached for the crsgen request
            $.consensusDigest[crsId] = digest;

            // Set the active CRS ID
            $.activeCrsId = crsId;
            $.completedCrsIds.push(crsId);

            string[] memory consensusUrls = _buildConsensusStorageUrls(contextId, consensusTxSenders);
            emit ActivateCrs(crsId, consensusUrls, crsDigest);
        }
    }

    /**
     * @notice See {IKMSGeneration-abortKeygen}.
     */
    function abortKeygen(uint256 prepKeygenId) external virtual onlyACLOwner {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();

        if (prepKeygenId > $.prepKeygenCounter || prepKeygenId <= PREP_KEYGEN_COUNTER_BASE) {
            revert AbortKeygenInvalidId(prepKeygenId);
        }

        // The prep request reaches consensus before the paired key request does.
        // Keep abort available until the key lifecycle itself is done.
        uint256 keyId = $.keygenIdPairs[prepKeygenId];
        if ($.isRequestDone[keyId]) {
            revert AbortKeygenAlreadyDone(prepKeygenId);
        }

        // Mark both the prep-keygen and its associated key as done to unblock
        $.isRequestDone[prepKeygenId] = true;
        if (keyId != 0) {
            $.isRequestDone[keyId] = true;
        }

        emit AbortKeygen(prepKeygenId);
    }

    /**
     * @notice See {IKMSGeneration-abortCrsgen}.
     */
    function abortCrsgen(uint256 crsId) external virtual onlyACLOwner {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();

        if (crsId > $.crsCounter || crsId <= CRS_COUNTER_BASE) {
            revert AbortCrsgenInvalidId(crsId);
        }
        if ($.isRequestDone[crsId]) {
            revert AbortCrsgenAlreadyDone(crsId);
        }

        $.isRequestDone[crsId] = true;

        emit AbortCrsgen(crsId);
    }

    /**
     * @notice See {IKMSGeneration-getKeyParamsType}.
     */
    function getKeyParamsType(uint256 keyId) external view virtual returns (ParamsType) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();

        if (!$.isRequestDone[keyId]) {
            revert KeyNotGenerated(keyId);
        }
        if ($.consensusDigest[keyId] == bytes32(0)) {
            revert KeyAborted(keyId);
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
        if ($.consensusDigest[crsId] == bytes32(0)) {
            revert CrsAborted(crsId);
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
     * @notice See {IKMSGeneration-getKeyMaterialVersion}.
     */
    function getKeyMaterialVersion(uint256 keyId) external view virtual returns (uint256) {
        // Migrated material exists iff a migration keygen was published under this key.
        return _getKMSGenerationStorage().publishedFrom[keyId] != 0 ? MIGRATED_MATERIAL_VERSION : 0;
    }

    /**
     * @notice See {IKMSGeneration-isKeyMaterialMigrationScheduled}.
     */
    function isKeyMaterialMigrationScheduled(uint256 keyId) external view virtual returns (bool) {
        return _getKMSGenerationStorage().migrationScheduled[keyId];
    }

    /**
     * @notice See {IKMSGeneration-getActiveCrsId}.
     */
    function getActiveCrsId() external view virtual returns (uint256) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();
        return $.activeCrsId;
    }

    /**
     * @notice See {IKMSGeneration-getKeyCounter}.
     */
    function getKeyCounter() external view virtual returns (uint256) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();
        return $.keyCounter;
    }

    /**
     * @notice See {IKMSGeneration-getCrsCounter}.
     */
    function getCrsCounter() external view virtual returns (uint256) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();
        return $.crsCounter;
    }

    /**
     * @notice See {IKMSGeneration-isRequestDone}.
     */
    function isRequestDone(uint256 requestId) external view virtual returns (bool) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();
        return $.isRequestDone[requestId];
    }

    /**
     * @notice See {IKMSGeneration-getConsensusTxSenders}.
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
     * @notice See {IKMSGeneration-getCompletedKeyIds}.
     */
    function getCompletedKeyIds() external view virtual returns (uint256[] memory) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();
        return $.completedKeyIds;
    }

    /**
     * @notice See {IKMSGeneration-getCompletedCrsIds}.
     */
    function getCompletedCrsIds() external view virtual returns (uint256[] memory) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();
        return $.completedCrsIds;
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
        if (digest == bytes32(0)) {
            revert KeyAborted(keyId);
        }
        address[] memory consensusTxSenders = $.consensusTxSenderAddresses[keyId][digest];

        uint256 contextId = _extractContextIdFromExtraData($.requestExtraData[keyId]);
        string[] memory consensusUrls = _buildConsensusStorageUrls(contextId, consensusTxSenders);

        return (consensusUrls, $.keyDigests[keyId]);
    }

    /**
     * @notice See {IKMSGeneration-getKeyInfo}.
     */
    function getKeyInfo(uint256 keyId) external view virtual returns (KeyInfo memory) {
        KMSGenerationStorage storage $ = _getKMSGenerationStorage();
        if (!$.isRequestDone[keyId]) {
            revert KeyNotGenerated(keyId);
        }
        if ($.consensusDigest[keyId] == bytes32(0)) {
            revert KeyAborted(keyId);
        }
        uint256 prepKeygenId = $.keygenIdPairs[keyId];
        return
            KeyInfo({
                prepKeygenId: prepKeygenId,
                keyId: keyId,
                paramsType: $.requestParamsType[prepKeygenId],
                keyDigests: $.keyDigests[keyId]
            });
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
        if (digest == bytes32(0)) {
            revert CrsAborted(crsId);
        }
        address[] memory consensusTxSenders = $.consensusTxSenderAddresses[crsId][digest];

        uint256 contextId = _extractContextIdFromExtraData($.requestExtraData[crsId]);
        string[] memory consensusUrls = _buildConsensusStorageUrls(contextId, consensusTxSenders);

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
     * @notice Validates the EIP712 signature against the request's pinned KMS context.
     * @param contextId The KMS context ID pinned at request time.
     * @param digest The hashed EIP712 struct.
     * @param signature The signature to validate.
     * @return The signer address.
     */
    function _validateEIP712Signature(
        uint256 contextId,
        bytes32 digest,
        bytes calldata signature
    ) internal view virtual returns (address) {
        // Recover the signer address from the signature
        address signer = ECDSA.recover(digest, signature);

        // Verify signer/tx-sender membership within the request's pinned context so a mid-flight
        // rotation cannot invalidate in-flight responses or split consensus across committees.
        _checkKmsContextSignerMatchesTxSender(contextId, signer, msg.sender);

        return signer;
    }

    /**
     * @notice Checks if the sender is authorized to upgrade the contract and reverts otherwise.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}

    /**
     * @notice Checks if the consensus is reached among the KMS nodes for a given context.
     * @param contextId The KMS context ID pinned at request time.
     * @param kmsCounter The number of KMS nodes that agreed.
     * @return Whether the consensus is reached.
     */
    function _isKmsConsensusReachedForContext(
        uint256 contextId,
        uint256 kmsCounter
    ) internal view virtual returns (bool) {
        uint256 consensusThreshold = PROTOCOL_CONFIG.getKmsGenThresholdForContext(contextId);
        return kmsCounter >= consensusThreshold;
    }

    /**
     * @notice Computes the hash of a PrepKeygenVerification struct
     * @param prepKeygenId The ID of the preprocessing keygen request.
     * @param extraData The extra data for replay protection.
     * @return The hash of the PrepKeygenVerification struct
     */
    function _hashPrepKeygenVerification(
        uint256 prepKeygenId,
        bytes memory extraData
    ) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(keccak256(abi.encode(EIP712_PREP_KEYGEN_TYPE_HASH, prepKeygenId, keccak256(extraData))));
    }

    /**
     * @notice Computes the hash of a KeygenVerification struct
     * @param prepKeygenId The ID of the preprocessing keygen request.
     * @param keyId The ID of the generated key.
     * @param keyDigests The digests of the generated keys.
     * @param extraData The extra data for replay protection.
     * @return The hash of the KeygenVerification struct
     */
    function _hashKeygenVerification(
        uint256 prepKeygenId,
        uint256 keyId,
        KeyDigest[] calldata keyDigests,
        bytes memory extraData
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
                        keccak256(abi.encodePacked(keyDigestHashes)),
                        keccak256(extraData)
                    )
                )
            );
    }

    /**
     * @notice Computes the hash of a CrsgenVerification struct
     * @param crsId The ID of the generated CRS.
     * @param maxBitLength The max bit length used for generating the CRS.
     * @param crsDigest The digest of the generated CRS.
     * @param extraData The extra data for replay protection.
     * @return The hash of the CrsgenVerification struct
     */
    function _hashCrsgenVerification(
        uint256 crsId,
        uint256 maxBitLength,
        bytes calldata crsDigest,
        bytes memory extraData
    ) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        EIP712_CRSGEN_TYPE_HASH,
                        crsId,
                        maxBitLength,
                        keccak256(abi.encodePacked(crsDigest)),
                        keccak256(extraData)
                    )
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

    /**
     * @notice Checks that the signer corresponds to the tx sender within a given KMS context.
     * @param contextId The KMS context ID pinned at request time.
     * @param signerAddress The recovered EIP-712 signer.
     * @param txSenderAddress The transaction sender (msg.sender).
     */
    function _checkKmsContextSignerMatchesTxSender(
        uint256 contextId,
        address signerAddress,
        address txSenderAddress
    ) internal view virtual {
        if (!PROTOCOL_CONFIG.isKmsSignerForContext(contextId, signerAddress)) {
            revert KmsSignerDoesNotMatchTxSender(signerAddress, txSenderAddress);
        }
        KmsNode memory node = PROTOCOL_CONFIG.getKmsNodeForContext(contextId, txSenderAddress);
        if (node.signerAddress != signerAddress) {
            revert KmsSignerDoesNotMatchTxSender(signerAddress, txSenderAddress);
        }
    }

    /**
     * @notice Builds an array of storage URLs from the consensus tx senders.
     * @param contextId The KMS context under which the consensus was reached.
     * @param consensusTxSenders The tx sender addresses from the consensus.
     */
    function _buildConsensusStorageUrls(
        uint256 contextId,
        address[] memory consensusTxSenders
    ) internal view virtual returns (string[] memory) {
        uint256 len = consensusTxSenders.length;
        string[] memory urls = new string[](len);
        for (uint256 i = 0; i < len; i++) {
            urls[i] = PROTOCOL_CONFIG.getKmsNodeForContext(contextId, consensusTxSenders[i]).storageUrl;
        }
        return urls;
    }

    /**
     * @notice Extracts the context ID from the request extraData.
     * @param extraData The stored extra data.
     * @return contextId The extracted context ID, or the current context if extraData is empty.
     */
    function _extractContextIdFromExtraData(bytes memory extraData) internal view virtual returns (uint256 contextId) {
        // v0 (0x00 prefix or empty): uses the current context. Trailing bytes are
        // ignored for forward-compatibility with potential v0 extensions.
        if (extraData.length == 0 || uint8(extraData[0]) == 0x00) {
            return PROTOCOL_CONFIG.getCurrentKmsContextId();
        }

        uint8 version = uint8(extraData[0]);
        if (version != EXTRA_DATA_V1 && version != EXTRA_DATA_V2) {
            revert UnsupportedExtraDataVersion(version);
        }
        if (version == EXTRA_DATA_V1 && extraData.length != 33) {
            revert DeserializingExtraDataFail();
        }
        if (version == EXTRA_DATA_V2 && extraData.length != 65) {
            revert DeserializingExtraDataFail();
        }
        // v1 extraData layout: [version(1)] [contextId(32)]
        // v2 extraData layout: [version(1)] [contextId(32)] [epochId(32)]
        // mload at offset 33 reads 32 bytes starting after the 1-byte version prefix.
        assembly {
            contextId := mload(add(extraData, 33))
        }
    }

    function _encodeRequestExtraDataV2(
        uint256 contextId,
        uint256 epochId
    ) internal pure virtual returns (bytes memory) {
        return abi.encodePacked(EXTRA_DATA_V2, contextId, epochId);
    }
}
