// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {EIP712UpgradeableCrossChain} from "./shared/EIP712UpgradeableCrossChain.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";

/**
 * @title   KMSVerifier.
 * @notice  KMSVerifier (Key Management System Verifier) is a contract that allows the management of signers and provides
 *          signature verification functions with epoch-based lifecycle management.
 * @dev     The contract uses EIP712UpgradeableCrossChain for cryptographic operations and is deployed using an UUPS proxy.
 */
contract KMSVerifier is UUPSUpgradeableEmptyProxy, EIP712UpgradeableCrossChain, ACLOwnable {
    // =========================================================================
    //  Errors
    // =========================================================================

    /// @notice Returned if the KMS signer to add is already a signer.
    error KMSAlreadySigner();

    /// @notice Returned if the recovered KMS signer is not a valid KMS signer.
    /// @param invalidSigner Address of the invalid signer.
    error KMSInvalidSigner(address invalidSigner);

    /// @notice Returned if the deserializing of the decryption proof fails.
    error DeserializingDecryptionProofFail();

    /// @notice Returned if the deserializing of the extra data fails.
    error DeserializingExtraDataFail();

    /// @notice Returned if the decryption proof is empty.
    error EmptyDecryptionProof();

    /// @notice Returned if the KMS signer to add is the null address.
    error KMSSignerNull();

    /// @notice                 Returned if the number of signatures is inferior to the threshold.
    /// @param numSignatures    Number of signatures.
    error KMSSignatureThresholdNotReached(uint256 numSignatures);

    /// @notice Returned if the number of signatures is equal to 0.
    error KMSZeroSignature();

    /// @notice Returned if the signers set is empty.
    error SignersSetIsEmpty();

    /// @notice Returned if the chosen threshold is null.
    error ThresholdIsNull();

    /// @notice Threshold is above number of signers.
    error ThresholdIsAboveNumberOfSigners();

    /// @notice Returned if the KMS context does not exist or has been destroyed.
    /// @param kmsContextId The non-existent context ID.
    error InvalidKMSContext(uint256 kmsContextId);

    /// @notice Returned if the extra data version is unsupported.
    /// @param version The unsupported version byte.
    error UnsupportedExtraDataVersion(uint8 version);

    // --- Epoch lifecycle errors ---

    error InvalidEpochState(uint256 epochId, uint8 currentState, uint8 expectedState);
    error InvalidContextState(uint256 contextId, uint8 currentState, uint8 expectedState);
    error AlreadyConfirmedContextCreation(uint256 contextId, address signer);
    error AlreadyConfirmedEpochResult(uint256 epochId, address signer);
    error NotContextSigner(uint256 contextId, address signer);
    error NoActiveContext();
    error ContextInFlight(uint256 contextId);
    error EpochInFlight(uint256 epochId);
    error InconsistentEpochResult(uint256 epochId, bytes32 expected, bytes32 actual);
    error InvalidExtraDataLength(uint256 actual, uint256 minimum);
    error InvalidExtraDataEpoch(uint256 expected, uint256 actual);
    error InvalidExtraDataContext(uint256 expected, uint256 actual);
    error NotPendingEpoch(uint256 epochId, uint256 pendingEpochId);
    error ActiveContextCannotBeDestroyed(uint256 kmsContextId);

    // =========================================================================
    //  Events
    // =========================================================================

    /// @notice         Emitted when a new context and epoch are defined.
    /// @param contextId         The context ID.
    /// @param threshold         The threshold for the context.
    /// @param softwareVersion   The software version string.
    /// @param pcrValues         The PCR values array.
    /// @param mpcNodes          The MPC node configuration array.
    event NewContextSet(
        uint256 indexed contextId,
        uint256 threshold,
        string softwareVersion,
        PcrValues[] pcrValues,
        MpcNode[] mpcNodes
    );

    /// @notice         Emitted when a KMS context is destroyed.
    /// @param kmsContextId      The destroyed context ID.
    event KMSContextDestroyed(uint256 indexed kmsContextId);

    /// @notice Emitted when all signers have confirmed context creation.
    event ContextCreated(
        uint256 indexed contextId,
        uint256 indexed epochId,
        uint256 previousContextId,
        uint256 previousEpochId
    );

    /// @notice Emitted when a new epoch is initiated for the current (active) context.
    /// @dev previousContextId is omitted — it always equals contextId for same-set transitions.
    event NewEpochForCurrentContext(uint256 indexed contextId, uint256 indexed epochId, uint256 previousEpochId);

    /// @notice Emitted when all signers have confirmed an epoch result, activating the epoch.
    event EpochActivated(uint256 indexed contextId, uint256 indexed epochId);

    // =========================================================================
    //  Structs
    // =========================================================================

    /// @notice The typed data structure for the EIP712 signature to validate in public decryption responses.
    /// @dev The name of this struct is not relevant for the signature validation, only the one defined
    /// @dev EIP712_PUBLIC_DECRYPT_TYPE is, but we keep it the same for clarity.
    struct PublicDecryptVerification {
        /// @notice The handles of the ciphertexts that have been decrypted.
        bytes32[] ctHandles;
        /// @notice The decrypted result of the public decryption.
        bytes decryptedResult;
        /// @notice Generic bytes metadata for versioned payloads.
        bytes extraData;
    }

    /// @notice The type of a generated key. Mirrors IKMSGeneration.KeyType from gateway-contracts.
    enum KeyType {
        Server, // 0
        Public // 1
    }

    /// @notice Key digest struct for epoch result verification. Mirrors IKMSGeneration.KeyDigest.
    /// @dev EIP-712 encodes keyType as uint8 in the type string regardless of Solidity enum vs uint8 declaration.
    struct KeyDigest {
        KeyType keyType;
        bytes digest;
    }

    /// @notice MPC node configuration, matching protobuf MpcNode message.
    struct MpcNode {
        string mpcIdentity;
        int32 partyId;
        bytes verificationKey;
        string externalUrl;
        bytes caCert;
        string publicStorageUrl;
        string publicStoragePrefix;
        bytes[] extraVerificationKeys;
    }

    /// @notice Platform Configuration Register values for attestation.
    struct PcrValues {
        bytes pcr0;
        bytes pcr1;
        bytes pcr2;
    }

    /// @notice Per-context lifecycle and metadata, stored in `KMSVerifierStorage.contexts`.
    struct ContextInfo {
        uint8 state;
        address[] signers;
        mapping(address => bool) isSigner;
        uint256 threshold;
        mapping(address => bool) creationConfirmed;
        uint256 creationConfirmCount;
        MpcNode[] mpcNodes;
        string softwareVersion;
        PcrValues[] pcrValues;
    }

    /// @notice Per-epoch lifecycle and consensus data, stored in `KMSVerifierStorage.epochs`.
    struct EpochInfo {
        uint8 state;
        uint256 contextId;
        bytes32 referenceDigest;
        mapping(address => bool) resultConfirmed;
        uint256 resultConfirmCount;
    }

    // =========================================================================
    //  EIP-712 Constants
    // =========================================================================

    /// @notice Decryption result type.
    string public constant EIP712_PUBLIC_DECRYPT_TYPE =
        "PublicDecryptVerification(bytes32[] ctHandles,bytes decryptedResult,bytes extraData)";

    /// @notice Decryption result typehash.
    bytes32 public constant DECRYPTION_RESULT_TYPEHASH = keccak256(bytes(EIP712_PUBLIC_DECRYPT_TYPE));

    /// @notice ContextCreationConfirmation EIP-712 type, verified under KMSVerifier native domain.
    string private constant EIP712_CONTEXT_CREATION_TYPE = "ContextCreationConfirmation(uint256 contextId)";
    bytes32 private constant CONTEXT_CREATION_TYPEHASH = keccak256(bytes(EIP712_CONTEXT_CREATION_TYPE));

    /// @notice KeyDigest EIP-712 type.
    string private constant EIP712_KEY_DIGEST_TYPE = "KeyDigest(uint8 keyType,bytes digest)";
    bytes32 private constant KEY_DIGEST_TYPEHASH = keccak256(bytes(EIP712_KEY_DIGEST_TYPE));

    /// @notice KeygenVerification EIP-712 type (post-#1125: includes extraData).
    string private constant EIP712_KEYGEN_TYPE =
        "KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)";
    bytes32 private constant KEYGEN_VERIFICATION_TYPEHASH = keccak256(bytes(EIP712_KEYGEN_TYPE));

    /// @notice Standard EIP-712 domain type hash.
    bytes32 private constant EIP712_DOMAIN_TYPEHASH =
        keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");

    /// @dev Pre-computed hash of the name string for the KMSVerifier self-domain.
    bytes32 private constant _HASHED_NAME_SELF = keccak256("KMSVerifier");

    /// @dev Pre-computed hash of the version string for the KMSVerifier self-domain.
    bytes32 private constant _HASHED_VERSION_SELF = keccak256("1");

    // =========================================================================
    //  Contract Info Constants
    // =========================================================================

    /// @notice Name of the contract.
    string private constant CONTRACT_NAME = "KMSVerifier";

    /// @notice Name of the source contract for which original EIP712 was destinated.
    string private constant CONTRACT_NAME_SOURCE = "Decryption";

    /// @notice Major version of the contract.
    uint256 private constant MAJOR_VERSION = 0;

    /// @notice Minor version of the contract.
    uint256 private constant MINOR_VERSION = 3;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    // =========================================================================
    //  Counter & State Constants
    // =========================================================================

    /// @notice Base value for KMS context IDs. Format: [0x07 type tag | 31 counter bytes].
    /// @dev See KMSRequestCounters on Gateway for the shared counter scheme.
    uint256 private constant KMS_CONTEXT_COUNTER_BASE = uint256(0x07) << 248;

    /// @notice Base value for epoch IDs. Format: [0x08 type tag | 31 counter bytes].
    /// @dev 0x08 follows KMSRequestCounters pattern (0x01–0x06 on Gateway, 0x07 for KMS context).
    ///      Local to KMSVerifier since host-contracts can't import from gateway-contracts.
    uint256 private constant EPOCH_COUNTER_BASE = uint256(0x08) << 248;

    /// @notice Context states.
    /// @dev 0 is intentionally unused (default for non-existent / pre-migration contexts).
    uint8 private constant CONTEXT_STATE_PENDING = 1;
    uint8 private constant CONTEXT_STATE_CREATED = 2;
    uint8 private constant CONTEXT_STATE_ACTIVE = 3;
    uint8 private constant CONTEXT_STATE_DESTROYED = 4;

    /// @notice Epoch states.
    uint8 private constant EPOCH_STATE_PENDING = 1;
    uint8 private constant EPOCH_STATE_ACTIVE = 2;

    // =========================================================================
    //  Storage
    // =========================================================================

    /// @custom:storage-location erc7201:fhevm.storage.KMSVerifier
    struct KMSVerifierStorage {
        /// @dev Deprecated (V1). Retained for storage layout compatibility.
        mapping(address => bool) _isSigner_deprecated;
        /// @dev Deprecated (V1). Retained for storage layout compatibility.
        address[] _signers_deprecated;
        /// @dev Deprecated (V1). Retained for storage layout compatibility.
        uint256 _threshold_deprecated;
        /// @notice Current KMS context ID counter.
        uint256 currentKmsContextId;
        /// @dev Deprecated (V2→V3). Migrated to contexts[id].signers in reinitializeV3.
        mapping(uint256 => address[]) _contextSigners_deprecated;
        /// @dev Deprecated (V2→V3). Migrated to contexts[id].isSigner in reinitializeV3.
        mapping(uint256 => mapping(address => bool)) _contextIsSigner_deprecated;
        /// @dev Deprecated (V2→V3). Migrated to contexts[id].threshold in reinitializeV3.
        mapping(uint256 => uint256) _contextThreshold_deprecated;
        /// @dev Deprecated. Destruction is now tracked via contexts[id].state == CONTEXT_STATE_DESTROYED.
        ///      Retained for storage layout compatibility with deployed V2 proxies.
        mapping(uint256 => bool) _destroyedContexts_deprecated;
        // --- Epoch lifecycle fields ---
        // activeContextId takes over the "active" semantic from currentKmsContextId,
        // which remains a pure counter/allocator. They diverge during PENDING phases.
        uint256 epochCounter;
        uint256 activeContextId;
        uint256 activeEpochId;
        uint256 pendingContextId;
        uint256 pendingEpochId;
        mapping(uint256 => ContextInfo) contexts;
        mapping(uint256 => EpochInfo) epochs;
    }

    /// @dev Reinitializer version shared between initializeFromEmptyProxy and reinitializeV3.
    uint64 private constant REINITIALIZER_VERSION = 4;

    /// keccak256(abi.encode(uint256(keccak256("fhevm.storage.KMSVerifier")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant KMSVerifierStorageLocation =
        0x7e81a744be86773af8644dd7304fa1dc9350ccabf16cfcaa614ddb78b4ce8900;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    // =========================================================================
    //  Initialization
    // =========================================================================

    /**
     * @notice  Re-initializes the contract.
     * @param verifyingContractSource The Decryption contract address from the Gateway chain.
     * @param chainIDSource The chain id of the Gateway chain.
     * @param initialSigners The list of initial KMS signers, should be non-empty and contain unique addresses.
     * @param initialThreshold Initial threshold, should be non-null and less or equal to the initialSigners length.
     * @param mpcNodes         MPC node descriptors for the genesis context.
     * @param softwareVersion  Software version string for the genesis context.
     * @param pcrValues        PCR attestation values for the genesis context.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        address verifyingContractSource,
        uint64 chainIDSource,
        address[] calldata initialSigners,
        uint256 initialThreshold,
        MpcNode[] memory mpcNodes,
        string memory softwareVersion,
        PcrValues[] memory pcrValues
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        __EIP712_init(CONTRACT_NAME_SOURCE, "1", verifyingContractSource, chainIDSource);
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();

        // Initialize counters
        $.currentKmsContextId = KMS_CONTEXT_COUNTER_BASE;
        $.epochCounter = EPOCH_COUNTER_BASE;

        // Create genesis context
        uint256 genesisContextId = _defineContext(
            initialSigners, initialThreshold, mpcNodes, softwareVersion, pcrValues
        );

        // Create genesis epoch
        $.epochCounter++;
        uint256 genesisEpochId = $.epochCounter;

        // Set genesis context and epoch as ACTIVE
        $.contexts[genesisContextId].state = CONTEXT_STATE_ACTIVE;
        $.epochs[genesisEpochId].state = EPOCH_STATE_ACTIVE;
        $.epochs[genesisEpochId].contextId = genesisContextId;
        $.activeContextId = genesisContextId;
        $.activeEpochId = genesisEpochId;
    }

    /**
     * @notice Re-initializes the contract from V1 to V2 with context-aware KMS support.
     * @dev Migrates existing signers into the first context. The legacy mapping retains its pre-migration
     *      values, which should be considered stale after this call.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV2() public virtual onlyACLOwner reinitializer(3) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        $.currentKmsContextId = KMS_CONTEXT_COUNTER_BASE;
        _defineContext($._signers_deprecated, $._threshold_deprecated, new MpcNode[](0), "", new PcrValues[](0));
    }

    /**
     * @notice Re-initializes the contract from V2 to V3 with epoch lifecycle support.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV3() public virtual onlyACLOwner reinitializer(REINITIALIZER_VERSION) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();

        // Ensure reinitializeV2 has been called and at least one context exists
        if ($.currentKmsContextId <= KMS_CONTEXT_COUNTER_BASE) {
            revert InvalidKMSContext($.currentKmsContextId);
        }

        // Migrate V2 flat-mapping signer data into ContextInfo structs.
        // Deployed V2 bytecode wrote to _contextSigners_deprecated / _contextThreshold_deprecated;
        // copy that data into contexts[id].signers / .isSigner / .threshold.
        for (uint256 ctxId = KMS_CONTEXT_COUNTER_BASE + 1; ctxId <= $.currentKmsContextId; ctxId++) {
            address[] storage oldSigners = $._contextSigners_deprecated[ctxId];
            if (oldSigners.length > 0) {
                ContextInfo storage ctx = $.contexts[ctxId];
                for (uint256 i = 0; i < oldSigners.length; i++) {
                    ctx.signers.push(oldSigners[i]);
                    ctx.isSigner[oldSigners[i]] = true;
                }
                ctx.threshold = $._contextThreshold_deprecated[ctxId];
            }
        }

        // Initialize epoch counter and create genesis epoch
        $.epochCounter = EPOCH_COUNTER_BASE;
        $.epochCounter++;
        uint256 genesisEpochId = $.epochCounter;

        // Set existing context and genesis epoch as ACTIVE
        $.contexts[$.currentKmsContextId].state = CONTEXT_STATE_ACTIVE;
        $.epochs[genesisEpochId].state = EPOCH_STATE_ACTIVE;
        $.epochs[genesisEpochId].contextId = $.currentKmsContextId;
        $.activeContextId = $.currentKmsContextId;
        $.activeEpochId = genesisEpochId;
    }

    // =========================================================================
    //  Context & Epoch Management
    // =========================================================================

    /**
     * @notice          Defines a new context and its initial epoch.
     * @dev             Only the owner can call. Does not immediately activate —
     *                  requires confirmContextCreation + confirmEpochResult from all signers.
     *                  Signer addresses are derived from each MpcNode.verificationKey
     *                  (expected to be a 64-byte uncompressed secp256k1 public key).
     * @param mpcNodes          The MPC node configurations for the new context.
     * @param newThreshold      The threshold to be set.
     * @param softwareVersion   The software version string.
     * @param pcrValues         The PCR values for attestation.
     */
    function defineNewContextAndEpoch(
        MpcNode[] memory mpcNodes,
        uint256 newThreshold,
        string memory softwareVersion,
        PcrValues[] memory pcrValues
    ) public virtual onlyACLOwner {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();

        // Only one pending context or epoch at a time
        if ($.pendingContextId != 0) revert ContextInFlight($.pendingContextId);
        if ($.pendingEpochId != 0) revert EpochInFlight($.pendingEpochId);

        // Derive signer addresses from MpcNode.verificationKey
        address[] memory signers = new address[](mpcNodes.length);
        for (uint256 i = 0; i < mpcNodes.length; i++) {
            signers[i] = address(uint160(uint256(keccak256(mpcNodes[i].verificationKey))));
        }

        uint256 newContextId = _defineContext(signers, newThreshold, mpcNodes, softwareVersion, pcrValues);

        // Create epoch for this context
        $.epochCounter++;
        uint256 newEpochId = $.epochCounter;

        // Set states
        $.contexts[newContextId].state = CONTEXT_STATE_PENDING;
        $.epochs[newEpochId].state = EPOCH_STATE_PENDING;
        $.epochs[newEpochId].contextId = newContextId;
        $.pendingContextId = newContextId;
        $.pendingEpochId = newEpochId;

        emit NewContextSet(newContextId, newThreshold, softwareVersion, pcrValues, mpcNodes);
    }

    /**
     * @notice Confirms context creation by a signer under the KMSVerifier native EIP-712 domain.
     * @param contextId The context ID to confirm.
     * @param signature The EIP-712 signature from the signer.
     */
    function confirmContextCreation(uint256 contextId, bytes calldata signature) public virtual {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        ContextInfo storage ctx = $.contexts[contextId];

        if (ctx.state != CONTEXT_STATE_PENDING) {
            revert InvalidContextState(contextId, ctx.state, CONTEXT_STATE_PENDING);
        }

        // Compute EIP-712 digest under KMSVerifier native domain
        bytes32 digest = _hashContextCreationConfirmation(contextId);

        // Recover signer
        address signer = _recoverSigner(digest, signature);
        if (!ctx.isSigner[signer]) {
            revert NotContextSigner(contextId, signer);
        }

        if (ctx.creationConfirmed[signer]) {
            revert AlreadyConfirmedContextCreation(contextId, signer);
        }

        ctx.creationConfirmed[signer] = true;
        ctx.creationConfirmCount++;

        // When all signers confirmed → transition to CREATED
        if (ctx.creationConfirmCount == ctx.signers.length) {
            ctx.state = CONTEXT_STATE_CREATED;
            // activeContextId/activeEpochId are frozen while a context is pending
            // (single-pending invariant), so they reflect the previous context/epoch.
            emit ContextCreated(contextId, $.pendingEpochId, $.activeContextId, $.activeEpochId);
        }
    }

    /**
     * @notice Confirms an epoch result by a signer under the KMSVerifier EIP-712 domain.
     * @param epochId The epoch ID to confirm.
     * @param prepKeygenId The preprocessing keygen ID.
     * @param keyId The key ID.
     * @param keyDigests The key digests array.
     * @param extraData The extra data bytes (must follow RFC 005 binary layout).
     * @param signature The EIP-712 signature from the signer.
     */
    function confirmEpochResult(
        uint256 epochId,
        uint256 prepKeygenId,
        uint256 keyId,
        KeyDigest[] calldata keyDigests,
        bytes calldata extraData,
        bytes calldata signature
    ) public virtual {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();

        uint256 contextId = _validateEpochForConfirmation($, epochId, extraData);

        // Compute keygen digest and enforce multi-signer consensus
        bytes32 keygenDigest = _computeKeygenStructHash(prepKeygenId, keyId, keyDigests, extraData);
        _ensureConsistentEpochDigest($, epochId, keygenDigest);

        // Compute full digest under KMSVerifier self-domain, recover signer, and record confirmation
        bytes32 digest = _hashTypedDataSelf(keygenDigest);
        address signer = _recoverSigner(digest, signature);
        _recordEpochConfirmation($, epochId, contextId, signer);
    }

    /**
     * @notice Defines a new epoch for the current active context (same-set epoch transition).
     * @dev Only the owner can call. Creates a new PENDING epoch for the active context.
     */
    function defineNewEpochForCurrentContext() public virtual onlyACLOwner {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();

        if ($.activeContextId == 0) revert NoActiveContext();
        // Only one pending context or epoch at a time
        if ($.pendingEpochId != 0) revert EpochInFlight($.pendingEpochId);
        if ($.pendingContextId != 0) revert ContextInFlight($.pendingContextId);

        uint256 previousEpochId = $.activeEpochId;

        $.epochCounter++;
        uint256 newEpochId = $.epochCounter;

        $.epochs[newEpochId].state = EPOCH_STATE_PENDING;
        $.epochs[newEpochId].contextId = $.activeContextId;
        $.pendingEpochId = newEpochId;

        emit NewEpochForCurrentContext($.activeContextId, newEpochId, previousEpochId);
    }

    /**
     * @notice              Destroys a KMS context, preventing it from being used for verification.
     * @dev                 Only the owner can destroy a context. The active context cannot be destroyed.
     * @param kmsContextId  The ID of the context to destroy.
     */
    function destroyKmsContext(uint256 kmsContextId) public virtual onlyACLOwner {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        if (!_contextIdInRange(kmsContextId)) {
            revert InvalidKMSContext(kmsContextId);
        }
        if ($.contexts[kmsContextId].state == CONTEXT_STATE_DESTROYED) {
            revert InvalidKMSContext(kmsContextId);
        }

        if (kmsContextId == $.activeContextId) {
            revert ActiveContextCannotBeDestroyed(kmsContextId);
        }

        $.contexts[kmsContextId].state = CONTEXT_STATE_DESTROYED;

        // If destroying the pending context, clear pending state
        if (kmsContextId == $.pendingContextId) {
            uint256 orphanedEpoch = $.pendingEpochId;
            $.pendingContextId = 0;
            $.pendingEpochId = 0;
            if (orphanedEpoch != 0) {
                $.epochs[orphanedEpoch].state = 0;
            }
        }

        // If the pending epoch belongs to this context (same-set resharing), clear it
        if ($.pendingEpochId != 0 && $.epochs[$.pendingEpochId].contextId == kmsContextId) {
            uint256 orphanedEpoch = $.pendingEpochId;
            $.pendingEpochId = 0;
            $.epochs[orphanedEpoch].state = 0;
        }

        emit KMSContextDestroyed(kmsContextId);
    }

    // =========================================================================
    //  Decryption Verification
    // =========================================================================

    /**
     * @notice                  Verifies multiple signatures for a given handlesList and a given decryptedResult.
     * @dev                     Calls verifySignaturesDigest internally.
     * @param handlesList       The list of handles, which where requested to be decrypted.
     * @param decryptedResult   A bytes array representing the abi-encoding of all requested decrypted values.
     * @param decryptionProof   Decryption proof containing KMS signatures and extra data.
     * @return isVerified       true if enough provided signatures are valid, false otherwise.
     */
    function verifyDecryptionEIP712KMSSignatures(
        bytes32[] memory handlesList,
        bytes memory decryptedResult,
        bytes memory decryptionProof
    ) public virtual returns (bool) {
        if (decryptionProof.length == 0) {
            revert EmptyDecryptionProof();
        }

        /// @dev The decryptionProof is the numSigners + kmsSignatures + extraData (1 + 65*numSigners + extraData bytes)
        uint256 numSigners = uint256(uint8(decryptionProof[0]));

        /// @dev The extraData is the rest of the decryptionProof bytes after the numSigners + signatures.
        uint256 extraDataOffset = 1 + 65 * numSigners;

        /// @dev Check that the decryptionProof is long enough to contain at least the numSigners + kmsSignatures.
        if (decryptionProof.length < extraDataOffset) {
            revert DeserializingDecryptionProofFail();
        }

        bytes[] memory signatures = new bytes[](numSigners);
        for (uint256 j = 0; j < numSigners; j++) {
            signatures[j] = new bytes(65);
            for (uint256 i = 0; i < 65; i++) {
                signatures[j][i] = decryptionProof[1 + 65 * j + i];
            }
        }

        /// @dev Extract the extraData from the decryptionProof.
        uint256 extraDataSize = decryptionProof.length - extraDataOffset;
        bytes memory extraData = new bytes(extraDataSize);
        for (uint i = 0; i < extraDataSize; i++) {
            extraData[i] = decryptionProof[extraDataOffset + i];
        }

        PublicDecryptVerification memory publicDecryptVerification = PublicDecryptVerification(
            handlesList,
            decryptedResult,
            extraData
        );
        bytes32 digest = _hashDecryptionResult(publicDecryptVerification);

        uint256 kmsContextId = _extractKmsContextId(extraData);
        return _verifySignaturesDigestForContext(digest, signatures, kmsContextId);
    }

    // =========================================================================
    //  View Functions
    // =========================================================================

    /**
     * @notice          Returns the list of KMS signers for the active context.
     * @dev             If there are too many signers, it could be out-of-gas.
     * @return signers  List of signers.
     */
    function getKmsSigners() public view virtual returns (address[] memory) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return $.contexts[$.activeContextId].signers;
    }

    /**
     * @notice              Get the threshold for signature.
     * @return threshold    Threshold for signature verification.
     */
    function getThreshold() public view virtual returns (uint256) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return $.contexts[$.activeContextId].threshold;
    }

    /**
     * @notice              Returns whether the account address is a valid KMS signer for the active context.
     * @param account       Account address.
     * @return isSigner_    Whether the account is a valid KMS signer.
     */
    function isSigner(address account) public view virtual returns (bool) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return $.contexts[$.activeContextId].isSigner[account];
    }

    /**
     * @notice              Returns the current active KMS context ID.
     * @dev                 Deprecated — prefer getCurrentKmsContext() which also returns the active epoch.
     * @return contextId    The current active KMS context ID.
     */
    function getCurrentKmsContextId() public view virtual returns (uint256) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return $.activeContextId;
    }

    /**
     * @notice Returns the current active context and epoch IDs.
     * @return contextId The active context ID.
     * @return epochId The active epoch ID.
     */
    function getCurrentKmsContext() public view virtual returns (uint256 contextId, uint256 epochId) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return ($.activeContextId, $.activeEpochId);
    }

    /**
     * @notice              Returns the list of signers for a given KMS context.
     * @dev                 Only returns signers for ACTIVE contexts. Returns empty for PENDING/CREATED contexts.
     * @param kmsContextId  The context ID.
     * @return signers      The list of signers for the context, or empty if context is not ACTIVE.
     */
    function getSignersForKmsContext(uint256 kmsContextId) public view virtual returns (address[] memory) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        if ($.contexts[kmsContextId].state != CONTEXT_STATE_ACTIVE) {
            return new address[](0);
        }
        return $.contexts[kmsContextId].signers;
    }

    /**
     * @notice              Resolves extraData into the context-specific signers and threshold.
     * @param extraData     The extra data bytes from the decryption proof.
     * @return signers      The list of signers for the resolved context.
     * @return threshold    The threshold for the resolved context.
     */
    function getContextSignersAndThresholdFromExtraData(
        bytes calldata extraData
    ) external view virtual returns (address[] memory signers, uint256 threshold) {
        uint256 kmsContextId = _extractKmsContextId(extraData);
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        ContextInfo storage ctx = $.contexts[kmsContextId];
        if (ctx.state != CONTEXT_STATE_ACTIVE) {
            revert InvalidKMSContext(kmsContextId);
        }
        return (ctx.signers, ctx.threshold);
    }

    /**
     * @notice        Getter for the name and version of the contract.
     * @return string Name and the version of the contract.
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

    // =========================================================================
    //  Internal Functions
    // =========================================================================

    /**
     * @notice          Cleans a hashmap in transient storage.
     * @dev             This is important to keep composability in the context of account abstraction.
     * @param keys      An array of keys to cleanup from transient storage.
     * @param maxIndex  The biggest index to take into account from the array - assumed to be less or equal to keys.length.
     */
    function _cleanTransientHashMap(address[] memory keys, uint256 maxIndex) internal virtual {
        for (uint256 j = 0; j < maxIndex; j++) {
            _tstore(keys[j], 0);
        }
    }

    /**
     * @notice          Writes to transient storage.
     * @dev             Uses inline assembly to access the Transient Storage's _tstore operation.
     * @param location  The address used as key where transient storage of the contract is written at.
     * @param value     An uint256 stored at location key in transient storage of the contract.
     */
    function _tstore(address location, uint256 value) internal virtual {
        assembly {
            tstore(location, value)
        }
    }

    /**
     * @notice              Validates and writes a threshold to context storage.
     * @param contextId     The context to update.
     * @param threshold_    The threshold to set. Must be non-zero and <= context signer count.
     */
    function _setContextThreshold(uint256 contextId, uint256 threshold_) internal virtual {
        if (threshold_ == 0) revert ThresholdIsNull();
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        if (threshold_ > $.contexts[contextId].signers.length) {
            revert ThresholdIsAboveNumberOfSigners();
        }
        $.contexts[contextId].threshold = threshold_;
    }

    /**
     * @notice              Checks whether a KMS context ID is within the allocated range.
     * @param kmsContextId  The context ID to check.
     * @return inRange      true if the context ID is within the valid range.
     */
    function _contextIdInRange(uint256 kmsContextId) internal view virtual returns (bool) {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        return kmsContextId >= KMS_CONTEXT_COUNTER_BASE + 1 && kmsContextId <= $.currentKmsContextId;
    }

    /**
     * @notice              Checks whether a KMS context ID is within the allocated range and not destroyed.
     * @dev                 Does NOT check ACTIVE state. The ACTIVE enforcement is in
     *                      _verifySignaturesDigestForContext.
     * @param kmsContextId  The context ID to check.
     * @return              true if the context ID is in range and not destroyed.
     */
    function _isValidKmsContext(uint256 kmsContextId) internal view virtual returns (bool) {
        uint8 state = _getKMSVerifierStorage().contexts[kmsContextId].state;
        // Reject: out of range, uninitialized (0 — non-existent or pre-V3 legacy), and destroyed.
        return _contextIdInRange(kmsContextId) && state != 0 && state != CONTEXT_STATE_DESTROYED;
    }

    /**
     * @notice              Validates and stores context signers for a given context ID.
     * @dev                 Reverts on null or duplicate addresses. Must only be called once
     *                      per contextId (appends without clearing).
     * @param contextId     The context ID.
     * @param signersList   The list of signers.
     */
    function _setContextSigners(uint256 contextId, address[] memory signersList) internal virtual {
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        ContextInfo storage ctx = $.contexts[contextId];
        for (uint256 i = 0; i < signersList.length; i++) {
            address signer = signersList[i];
            if (signer == address(0)) {
                revert KMSSignerNull();
            }
            if (ctx.isSigner[signer]) {
                revert KMSAlreadySigner();
            }
            ctx.signers.push(signer);
            ctx.isSigner[signer] = true;
        }
    }

    /**
     * @dev             Creates a new context with full metadata.
     * @param newSignersSet   The new set of signers. Must not be empty.
     * @param newThreshold    The threshold. Must be non-zero and <= signer count.
     * @param mpcNodes        MPC node descriptors (may be empty for genesis contexts).
     * @param softwareVersion Software version string (may be empty for genesis contexts).
     * @param pcrValues       PCR attestation values (may be empty for genesis contexts).
     * @return newContextId   The newly created context ID.
     */
    function _defineContext(
        address[] memory newSignersSet,
        uint256 newThreshold,
        MpcNode[] memory mpcNodes,
        string memory softwareVersion,
        PcrValues[] memory pcrValues
    ) internal virtual returns (uint256 newContextId) {
        if (newSignersSet.length == 0) {
            revert SignersSetIsEmpty();
        }
        KMSVerifierStorage storage $ = _getKMSVerifierStorage();
        newContextId = ++$.currentKmsContextId;
        _setContextSigners(newContextId, newSignersSet);
        _setContextThreshold(newContextId, newThreshold);
        _setContextMpcNodes(newContextId, mpcNodes);
        _setContextSoftwareVersion(newContextId, softwareVersion);
        _setContextPcrValues(newContextId, pcrValues);
    }

    /**
     * @notice              Stores MPC node descriptors for a given context ID.
     * @param contextId     The context ID.
     * @param mpcNodes      The MPC node descriptors.
     */
    function _setContextMpcNodes(uint256 contextId, MpcNode[] memory mpcNodes) internal virtual {
        ContextInfo storage ctx = _getKMSVerifierStorage().contexts[contextId];
        for (uint256 i = 0; i < mpcNodes.length; i++) {
            ctx.mpcNodes.push(mpcNodes[i]);
        }
    }

    /**
     * @notice              Stores the software version for a given context ID.
     * @param contextId     The context ID.
     * @param softwareVersion The software version string.
     */
    function _setContextSoftwareVersion(uint256 contextId, string memory softwareVersion) internal virtual {
        _getKMSVerifierStorage().contexts[contextId].softwareVersion = softwareVersion;
    }

    /**
     * @notice              Stores PCR attestation values for a given context ID.
     * @param contextId     The context ID.
     * @param pcrValues     The PCR values array.
     */
    function _setContextPcrValues(uint256 contextId, PcrValues[] memory pcrValues) internal virtual {
        ContextInfo storage ctx = _getKMSVerifierStorage().contexts[contextId];
        for (uint256 i = 0; i < pcrValues.length; i++) {
            ctx.pcrValues.push(pcrValues[i]);
        }
    }

    /**
     * @notice              Extracts the KMS context ID from extra data.
     * @param extraData     The extra data bytes from the decryption proof.
     * @return contextId    The extracted KMS context ID.
     */
    function _extractKmsContextId(bytes memory extraData) internal view virtual returns (uint256) {
        // v0 (0x00 prefix or empty): uses the active context. Trailing bytes are
        // ignored for forward-compatibility with potential v0 extensions.
        if (extraData.length == 0 || uint8(extraData[0]) == 0x00) {
            KMSVerifierStorage storage $ = _getKMSVerifierStorage();
            return $.activeContextId;
        }
        uint8 version = uint8(extraData[0]);
        if (version == 0x01) {
            // v1 (0x01 prefix): reads a 32-byte context ID starting at byte 1.
            // Trailing bytes after byte 33 are ignored for forward-compatibility
            // with potential v1 extensions (including the epoch ID at bytes [33:65]).
            if (extraData.length < 33) {
                revert DeserializingExtraDataFail();
            }
            uint256 contextId;
            // Memory layout: [32-byte length][version byte][32-byte contextId][...]
            // mload(add(extraData, 33)) reads 32 bytes starting at offset 1 (after version byte).
            assembly {
                contextId := mload(add(extraData, 33))
            }
            return contextId;
        }
        revert UnsupportedExtraDataVersion(version);
    }

    /**
     * @notice              Verifies multiple signatures for a given message using context-aware verification.
     * @param digest        The hash of the message that was signed by all signers.
     * @param signatures    An array of signatures to verify.
     * @param kmsContextId  The KMS context ID to verify against.
     * @return isVerified   true if enough provided signatures are valid, false otherwise.
     */
    function _verifySignaturesDigestForContext(
        bytes32 digest,
        bytes[] memory signatures,
        uint256 kmsContextId
    ) internal virtual returns (bool) {
        uint256 numSignatures = signatures.length;
        if (numSignatures == 0) {
            revert KMSZeroSignature();
        }

        KMSVerifierStorage storage $ = _getKMSVerifierStorage();

        if (!_isValidKmsContext(kmsContextId)) {
            revert InvalidKMSContext(kmsContextId);
        }

        // Only ACTIVE contexts can be used for decryption verification
        ContextInfo storage ctx = $.contexts[kmsContextId];
        if (ctx.state != CONTEXT_STATE_ACTIVE) {
            revert InvalidContextState(kmsContextId, ctx.state, CONTEXT_STATE_ACTIVE);
        }

        uint256 threshold = ctx.threshold;
        if (numSignatures < threshold) {
            revert KMSSignatureThresholdNotReached(numSignatures);
        }

        address[] memory recoveredSigners = new address[](numSignatures);
        uint256 uniqueValidCount;
        for (uint256 i = 0; i < numSignatures; i++) {
            address signerRecovered = _recoverSigner(digest, signatures[i]);
            if (!ctx.isSigner[signerRecovered]) {
                revert KMSInvalidSigner(signerRecovered);
            }
            if (!_tload(signerRecovered)) {
                recoveredSigners[uniqueValidCount] = signerRecovered;
                uniqueValidCount++;
                _tstore(signerRecovered, 1);
            }
            if (uniqueValidCount >= threshold) {
                _cleanTransientHashMap(recoveredSigners, uniqueValidCount);
                return true;
            }
        }
        _cleanTransientHashMap(recoveredSigners, uniqueValidCount);
        return false;
    }

    /**
     * @dev Should revert when msg.sender is not authorized to upgrade the contract.
     */
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}

    /**
     * @notice                  Hashes the decryption result.
     * @param decRes            Decryption result.
     * @return hashTypedData    Hash typed data.
     */
    function _hashDecryptionResult(PublicDecryptVerification memory decRes) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        DECRYPTION_RESULT_TYPEHASH,
                        keccak256(abi.encodePacked(decRes.ctHandles)),
                        keccak256(decRes.decryptedResult),
                        keccak256(abi.encodePacked(decRes.extraData))
                    )
                )
            );
    }

    /**
     * @notice           Reads transient storage.
     * @dev              Uses inline assembly to access the Transient Storage's tload operation.
     * @param location   The address used as key where transient storage of the contract is read at.
     * @return value     true if value stored at the given location is non-null, false otherwise.
     */
    function _tload(address location) internal view virtual returns (bool value) {
        assembly {
            value := tload(location)
        }
    }

    /**
     * @dev Returns the KMSVerifier storage location.
     */
    function _getKMSVerifierStorage() internal pure returns (KMSVerifierStorage storage $) {
        assembly {
            $.slot := KMSVerifierStorageLocation
        }
    }

    /**
     * @notice          Recovers the signer's address from a `signature` and a `message` digest.
     * @dev             It utilizes ECDSA for actual address recovery. It does not support contract signature (EIP-1271).
     * @param message   The hash of the message that was signed.
     * @param signature The signature to verify.
     * @return signer   The address that supposedly signed the message.
     */
    function _recoverSigner(bytes32 message, bytes memory signature) internal pure virtual returns (address) {
        address signerRecovered = ECDSA.recover(message, signature);
        return signerRecovered;
    }

    // =========================================================================
    //  EIP-712 Helpers
    // =========================================================================

    /// @dev Hashes typed data under the KMSVerifier native EIP-712 domain (self-referencing).
    ///      Computes the domain separator on-the-fly (~90 gas) which is cheaper than two
    ///      storage reads (~200 gas warm, ~4200 gas cold) for a cache-based approach.
    function _hashTypedDataSelf(bytes32 structHash) internal view returns (bytes32) {
        bytes32 domainSeparator = keccak256(
            abi.encode(EIP712_DOMAIN_TYPEHASH, _HASHED_NAME_SELF, _HASHED_VERSION_SELF, block.chainid, address(this))
        );
        return MessageHashUtils.toTypedDataHash(domainSeparator, structHash);
    }

    /// @dev Validates epoch state, context state, destroyed flag, and extraData layout for confirmEpochResult.
    function _validateEpochForConfirmation(
        KMSVerifierStorage storage $,
        uint256 epochId,
        bytes calldata extraData
    ) internal view returns (uint256 contextId) {
        // Must be the pending epoch
        if (epochId != $.pendingEpochId) {
            revert NotPendingEpoch(epochId, $.pendingEpochId);
        }
        EpochInfo storage epoch = $.epochs[epochId];
        if (epoch.state != EPOCH_STATE_PENDING) {
            revert InvalidEpochState(epochId, epoch.state, EPOCH_STATE_PENDING);
        }

        contextId = epoch.contextId;

        // Context must be CREATED (context switch) or ACTIVE (same-set resharing).
        // DESTROYED contexts are rejected here since DESTROYED != CREATED and DESTROYED != ACTIVE.
        uint8 ctxState = $.contexts[contextId].state;
        if (ctxState != CONTEXT_STATE_CREATED && ctxState != CONTEXT_STATE_ACTIVE) {
            revert InvalidContextState(contextId, ctxState, CONTEXT_STATE_CREATED);
        }

        // Validate extraData binary layout
        if (extraData.length < 65) {
            revert InvalidExtraDataLength(extraData.length, 65);
        }
        if (uint8(extraData[0]) != 0x01) {
            revert UnsupportedExtraDataVersion(uint8(extraData[0]));
        }
        uint256 edContextId = uint256(bytes32(extraData[1:33]));
        uint256 edEpochId = uint256(bytes32(extraData[33:65]));
        if (edContextId != contextId) {
            revert InvalidExtraDataContext(contextId, edContextId);
        }
        if (edEpochId != epochId) {
            revert InvalidExtraDataEpoch(epochId, edEpochId);
        }
    }

    /// @dev Enforces that all signers confirm the same keygen result for an epoch.
    ///      The first signer's digest becomes the reference; subsequent signers must match it.
    function _ensureConsistentEpochDigest(
        KMSVerifierStorage storage $,
        uint256 epochId,
        bytes32 keygenDigest
    ) internal {
        bytes32 referenceDigest = $.epochs[epochId].referenceDigest;
        if (referenceDigest == bytes32(0)) {
            $.epochs[epochId].referenceDigest = keygenDigest;
        } else if (keygenDigest != referenceDigest) {
            revert InconsistentEpochResult(epochId, referenceDigest, keygenDigest);
        }
    }

    /// @dev Records an epoch result confirmation, checking signer membership and duplicate.
    ///      Activates the epoch when all signers have confirmed.
    function _recordEpochConfirmation(
        KMSVerifierStorage storage $,
        uint256 epochId,
        uint256 contextId,
        address signer
    ) internal {
        ContextInfo storage ctx = $.contexts[contextId];
        if (!ctx.isSigner[signer]) {
            revert NotContextSigner(contextId, signer);
        }

        EpochInfo storage epoch = $.epochs[epochId];
        if (epoch.resultConfirmed[signer]) {
            revert AlreadyConfirmedEpochResult(epochId, signer);
        }

        epoch.resultConfirmed[signer] = true;
        epoch.resultConfirmCount++;

        // When all signers confirmed → activate
        if (epoch.resultConfirmCount == ctx.signers.length) {
            epoch.state = EPOCH_STATE_ACTIVE;
            // Intentional — simplicity over branching. Idempotent for same-set resharing.
            ctx.state = CONTEXT_STATE_ACTIVE;
            $.activeContextId = contextId;
            $.activeEpochId = epochId;
            $.pendingContextId = 0;
            $.pendingEpochId = 0;
            emit EpochActivated(contextId, epochId);
        }
    }

    /// @dev Computes the digest for ContextCreationConfirmation under the KMSVerifier native domain.
    function _hashContextCreationConfirmation(uint256 contextId) internal view virtual returns (bytes32) {
        return _hashTypedDataSelf(keccak256(abi.encode(CONTEXT_CREATION_TYPEHASH, contextId)));
    }

    /// @dev Computes the KeygenVerification struct hash (domain applied separately for digest pinning).
    function _computeKeygenStructHash(
        uint256 prepKeygenId,
        uint256 keyId,
        KeyDigest[] calldata keyDigests,
        bytes calldata extraData
    ) internal pure virtual returns (bytes32) {
        bytes32[] memory keyDigestHashes = new bytes32[](keyDigests.length);
        for (uint256 i = 0; i < keyDigests.length; i++) {
            keyDigestHashes[i] = keccak256(
                abi.encode(KEY_DIGEST_TYPEHASH, keyDigests[i].keyType, keccak256(keyDigests[i].digest))
            );
        }
        return
            keccak256(
                abi.encode(
                    KEYGEN_VERIFICATION_TYPEHASH,
                    prepKeygenId,
                    keyId,
                    keccak256(abi.encodePacked(keyDigestHashes)),
                    keccak256(extraData)
                )
            );
    }
}
