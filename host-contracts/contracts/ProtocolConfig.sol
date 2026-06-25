// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {IProtocolConfig} from "./interfaces/IProtocolConfig.sol";
import {IKMSGeneration} from "./interfaces/IKMSGeneration.sol";
import {KmsContextAnchor, KmsNode, KmsNodeParams, PcrValues} from "./shared/Structs.sol";
import {EPOCH_COUNTER_BASE, EXTRA_DATA_V2, KMS_CONTEXT_COUNTER_BASE} from "./shared/Constants.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

/**
 * @title ProtocolConfig
 * @notice Manages KMS node sets, thresholds, and context lifecycle on the host chains.
 * @dev Ethereum is the canonical host and the single source of truth: the context/epoch lifecycle
 *      (`defineNewKmsContextAndEpoch` / `defineNewEpochForCurrentKmsContext`, then
 *      `confirmKmsContextCreation` / `confirmEpochActivation`) runs only there, alongside
 *      `KMSGeneration`. The same contract is deployed on every other host chain (e.g. Polygon) as a
 *      read-replica: those never run the quorum path and advance state only through the owner-only
 *      `mirrorKmsContextAndEpoch` / `mirrorKmsEpoch` methods (see the Mirror functions section).
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract ProtocolConfig is IProtocolConfig, UUPSUpgradeableEmptyProxy, ACLOwnable {
    // -----------------------------------------------------------------------------------------
    // Contract information
    // -----------------------------------------------------------------------------------------

    string private constant CONTRACT_NAME = "ProtocolConfig";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 2;
    uint256 private constant PATCH_VERSION = 0;

    /// @dev Shared between `initializeFromEmptyProxy`, `initializeFromCanonical`, and `reinitializeV2`.
    uint64 private constant REINITIALIZER_VERSION = 3;

    /// @notice Upper bound on the KMS committee size and on every per-context threshold.
    /// @dev Driven by the proof format consumed in
    ///      `KMSVerifier.verifyDecryptionEIP712KMSSignatures`, which encodes the signature count
    ///      in a single byte (`uint8(decryptionProof[0])`). A context registered above this
    ///      bound cannot ever satisfy verification, so the limit is enforced at registration time
    ///      to reject the misconfiguration loudly rather than silently bricking the context.
    uint256 private constant MAX_KMS_SIGNERS = type(uint8).max;

    // -----------------------------------------------------------------------------------------
    // EIP-712 type hashes
    //
    // Used to recover the KMS signer from the keygen/CRS attestations supplied to
    // `confirmEpochActivation`.
    // -----------------------------------------------------------------------------------------

    /// @dev Hash of the EIP-712 domain separator type.
    bytes32 private constant EIP712_DOMAIN_TYPE_HASH =
        keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");

    /// @dev Hash of the KeyDigest type, the nested struct referenced by KeygenVerification.
    bytes32 private constant EIP712_KEY_DIGEST_TYPE_HASH = keccak256("KeyDigest(uint8 keyType,bytes digest)");

    /// @dev Hash of the KeygenVerification type. The nested KeyDigest type is appended as
    ///      EIP-712 requires nested struct types to be declared inline with the primary type.
    bytes32 private constant EIP712_KEYGEN_TYPE_HASH =
        keccak256(
            "KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)"
        );

    /// @dev Hash of the CrsgenVerification type.
    bytes32 private constant EIP712_CRSGEN_TYPE_HASH =
        keccak256("CrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest,bytes extraData)");

    /// @notice Lifecycle state of a KMS context: created Pending, promoted to Created once the
    ///         creation quorum confirms, then Active when its first epoch activates.
    enum ContextState {
        None,
        Pending,
        Created,
        Active
    }

    /// @notice Lifecycle state of an epoch: opened Pending for resharing, then Active once the
    ///         activation quorum confirms.
    enum EpochState {
        None,
        Pending,
        Active
    }

    // -----------------------------------------------------------------------------------------
    // ERC-7201 namespaced storage
    // -----------------------------------------------------------------------------------------

    /// @custom:storage-location erc7201:fhevm.storage.ProtocolConfig
    struct ProtocolConfigStorage {
        /// @notice Monotonic allocation counter for KMS context IDs: the latest issued ID.
        ///         Always `>= latestActiveKmsContextId`. Differs while a context is Pending/Created.
        uint256 currentKmsContextId;
        /// @notice KMS nodes per context.
        mapping(uint256 contextId => KmsNode[]) kmsNodesForContext;
        /// @notice Tx sender lookup per context.
        mapping(uint256 contextId => mapping(address txSender => bool isRegistered)) isKmsTxSenderForContext;
        /// @notice Signer lookup per context.
        mapping(uint256 contextId => mapping(address signer => bool isRegistered)) isKmsSignerForContext;
        /// @notice KmsNode by tx sender per context.
        mapping(uint256 contextId => mapping(address txSender => KmsNode node)) kmsNodeByTxSenderForContext;
        /// @notice Signer addresses per context, in insertion order.
        mapping(uint256 contextId => address[]) kmsSignerAddressesForContext;
        /// @notice Public decryption threshold per context.
        mapping(uint256 contextId => uint256) publicDecryptionThresholdForContext;
        /// @notice User decryption threshold per context.
        mapping(uint256 contextId => uint256) userDecryptionThresholdForContext;
        /// @notice KmsGen threshold per context.
        mapping(uint256 contextId => uint256) kmsGenThresholdForContext;
        /// @notice MPC threshold per context.
        /// @dev The SDK derives the MPC threshold from the MPC nodes it knows about instead of reading this value.
        mapping(uint256 contextId => uint256) mpcThresholdForContext;
        /// @notice Whether a context has been destroyed.
        mapping(uint256 contextId => bool) destroyedContexts;
        /// @notice The most recently activated KMS context ID, the one reads resolve against.
        ///         Several contexts may remain in the `Active` state at once (prior contexts are not
        ///         demoted on rotation, so in-flight requests stay valid). This points at the newest.
        ///         Updated only on activation, unlike the `currentKmsContextId` allocation counter.
        uint256 latestActiveKmsContextId;
        /// @notice Epoch ID counter.
        uint256 epochCounter;
        /// @notice The most recently activated epoch ID. Multiple epochs may remain `Active`. This
        ///         points at the newest, which new reads resolve against.
        uint256 latestActiveEpochId;
        /// @notice Lifecycle state per context.
        mapping(uint256 contextId => ContextState) contextState;
        /// @notice Lifecycle state per epoch.
        mapping(uint256 epochId => EpochState) epochState;
        /// @notice Context owning each epoch.
        mapping(uint256 epochId => uint256 contextId) contextForEpoch;
        /// @notice Context creation confirmations.
        mapping(uint256 contextId => mapping(address signer => bool confirmed)) contextCreationConfirmedBySigner;
        /// @notice Epoch activation confirmations per signer (one digest per signer per epoch).
        mapping(uint256 epochId => mapping(address signer => bool confirmed)) epochActivationConfirmedBySigner;
        /// @notice Number of epoch activation confirmations grouped by digest
        mapping(uint256 epochId => mapping(bytes32 dataHash => uint256 confirmations)) epochActivationConfirmationCountForDigest;
        /// @notice Required previous-context signer quorum, cached at pending-context creation time.
        mapping(uint256 contextId => uint256 threshold) contextCreationPreviousSignerThreshold;
        /// @notice New-context signer confirmations for context creation.
        mapping(uint256 contextId => uint256 confirmations) contextCreationNewSignerConfirmationCount;
        /// @notice Previous-context signer confirmations for context creation.
        mapping(uint256 contextId => uint256 confirmations) contextCreationPreviousSignerConfirmationCount;
        /// @notice Context anchor recorded when NewKmsContext was emitted.
        mapping(uint256 contextId => KmsContextAnchor) contextAnchors;
    }

    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm.storage.ProtocolConfig")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant PROTOCOL_CONFIG_STORAGE_LOCATION =
        0x80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c200;

    function _getProtocolConfigStorage() internal pure returns (ProtocolConfigStorage storage $) {
        assembly {
            $.slot := PROTOCOL_CONFIG_STORAGE_LOCATION
        }
    }

    // -----------------------------------------------------------------------------------------
    // Constructor
    // -----------------------------------------------------------------------------------------

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    // -----------------------------------------------------------------------------------------
    // Initialization
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Fresh deploy initializer: creates the first KMS context.
     * @dev When deploying a fresh ProtocolConfig on Ethereum (canonical host), this function is called.
     * @param initialKmsNodeParams The initial KMS node set, including MPC metadata.
     * @param initialThresholds The initial thresholds.
     * @param softwareVersion The KMS software version expected for the context.
     * @param pcrValues Accepted enclave PCR values for the context.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        KmsNodeParams[] calldata initialKmsNodeParams,
        KmsThresholds calldata initialThresholds,
        string calldata softwareVersion,
        PcrValues[] calldata pcrValues
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();

        $.currentKmsContextId = KMS_CONTEXT_COUNTER_BASE;
        uint256 newContextId = _storeAndActivateKmsContext(
            initialKmsNodeParams,
            initialThresholds,
            EPOCH_COUNTER_BASE + 1
        );

        $.contextAnchors[newContextId] = KmsContextAnchor({
            emissionBlockNumber: block.number,
            contextInfoHash: keccak256(abi.encode(initialKmsNodeParams, initialThresholds, softwareVersion, pcrValues))
        });
        emit NewKmsContext(
            newContextId,
            KMS_CONTEXT_COUNTER_BASE,
            initialKmsNodeParams,
            initialThresholds,
            softwareVersion,
            pcrValues
        );
    }

    /**
     * @notice Canonical mirror initializer: seeds a non-canonical host from Ethereum's active state.
     * @dev Preserves both the canonical context ID and canonical epoch ID instead of allocating a
     *      fresh local epoch. This is the bootstrap path for read-replica ProtocolConfig deployments;
     *      later canonical updates use `mirrorKmsContextAndEpoch` and `mirrorKmsEpoch`.
     * @param canonicalContextId The active Ethereum KMS context ID to preserve.
     * @param canonicalEpochId The active Ethereum epoch ID to preserve.
     * @param canonicalKmsNodeParams The active Ethereum KMS node set, including MPC metadata.
     * @param canonicalThresholds The active Ethereum thresholds.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromCanonical(
        uint256 canonicalContextId,
        uint256 canonicalEpochId,
        KmsNodeParams[] calldata canonicalKmsNodeParams,
        KmsThresholds calldata canonicalThresholds
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        if (canonicalContextId < KMS_CONTEXT_COUNTER_BASE + 1) {
            revert InvalidKmsContext(canonicalContextId);
        }
        if (canonicalEpochId < EPOCH_COUNTER_BASE + 1) {
            revert InvalidEpoch(canonicalEpochId);
        }

        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();

        // Seed counter so _storeKmsContext's ++counter lands on the canonical context ID.
        $.currentKmsContextId = canonicalContextId - 1;
        _storeAndActivateKmsContext(canonicalKmsNodeParams, canonicalThresholds, canonicalEpochId);
    }

    /**
     * @notice Re-initializes the contract from V1.
     * @dev Define a `reinitializeVX` function once the contract needs to be upgraded.
     * @param kmsNodeParams The existing context's KMS node set, used to recompute the anchor hash.
     * @param thresholds The existing context's thresholds, used to recompute the anchor hash.
     * @param softwareVersion The KMS software version expected for the context.
     * @param pcrValues Accepted enclave PCR values for the context.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV2(
        KmsNodeParams[] calldata kmsNodeParams,
        KmsThresholds calldata thresholds,
        string calldata softwareVersion,
        PcrValues[] calldata pcrValues
    ) public virtual reinitializer(REINITIALIZER_VERSION) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();

        // Bring the existing context into the epoch-lifecycle shape: active context + first epoch.
        $.epochCounter = EPOCH_COUNTER_BASE;
        uint256 contextId = $.currentKmsContextId;
        $.latestActiveKmsContextId = contextId;
        $.contextState[contextId] = ContextState.Active;
        uint256 epochId = ++$.epochCounter;
        _activateEpoch(epochId, contextId);

        // Backfill the anchor the pre-epoch version never recorded.
        $.contextAnchors[contextId] = KmsContextAnchor({
            emissionBlockNumber: block.number,
            contextInfoHash: keccak256(abi.encode(kmsNodeParams, thresholds, softwareVersion, pcrValues))
        });
        // Emit the genesis NewKmsContext the pre-epoch version never recorded. KMS-connectors will consider KMS_CONTEXT_COUNTER_BASE as sentinel/zero value
        // and avoid triggering the context switch.
        emit NewKmsContext(contextId, KMS_CONTEXT_COUNTER_BASE, kmsNodeParams, thresholds, softwareVersion, pcrValues);
    }

    // -----------------------------------------------------------------------------------------
    // State-changing functions
    // -----------------------------------------------------------------------------------------

    /// @inheritdoc IProtocolConfig
    /// @dev Context-switch: governance opens a new signer set + epoch, both Pending.
    /// @dev The DAO must not open a switch while another is in flight: at most one context and one
    ///      epoch may be non-active (Pending/Created) at a time. Settle the in-flight one first, either
    ///      by completing it (confirmKmsContextCreation then confirmEpochActivation) or aborting it
    ///      (abortPendingContext / abortPendingEpoch).
    function defineNewKmsContextAndEpoch(
        KmsNodeParams[] calldata kmsNodeParams,
        KmsThresholds calldata thresholds,
        string calldata softwareVersion,
        PcrValues[] calldata pcrValues
    ) external virtual onlyACLOwner {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();

        // Store the new signer set and open its first epoch as Pending.
        uint256 previousContextId = $.latestActiveKmsContextId;
        uint256 contextId = _storeKmsContext(kmsNodeParams, thresholds);
        $.contextState[contextId] = ContextState.Pending;
        _createPendingEpoch(contextId);

        // Cache the previous-context signer quorum needed by confirmKmsContextCreation.
        $.contextCreationPreviousSignerThreshold[contextId] =
            $.kmsSignerAddressesForContext[previousContextId].length -
            $.mpcThresholdForContext[previousContextId] +
            1;

        // Store context anchor and emit NewKmsContext event.
        $.contextAnchors[contextId] = KmsContextAnchor({
            emissionBlockNumber: block.number,
            contextInfoHash: keccak256(abi.encode(kmsNodeParams, thresholds, softwareVersion, pcrValues))
        });
        emit NewKmsContext(contextId, previousContextId, kmsNodeParams, thresholds, softwareVersion, pcrValues);
    }

    /// @inheritdoc IProtocolConfig
    /// @dev Same-set resharing: governance opens a new Pending epoch under the active context, no signer change.
    /// @dev The DAO must not open a switch while another is in flight: at most one context and one
    ///      epoch may be non-active (Pending/Created) at a time. Settle the in-flight one first, either
    ///      by completing it (confirmKmsContextCreation then confirmEpochActivation) or aborting it
    ///      (abortPendingContext / abortPendingEpoch).
    function defineNewEpochForCurrentKmsContext() external virtual onlyACLOwner {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        uint256 latestActiveKmsContextId = $.latestActiveKmsContextId;
        uint256 epochId = _createPendingEpoch(latestActiveKmsContextId);

        // NewKmsEpoch: `previousContextId` equals `kmsContextId` because same-set resharing keeps the
        // context. `materialBlockNumber` is the last block before this request, where connectors read
        // the previous key/CRS material.
        emit NewKmsEpoch(
            latestActiveKmsContextId,
            epochId,
            latestActiveKmsContextId,
            $.latestActiveEpochId,
            block.number - 1
        );
    }

    /// @inheritdoc IProtocolConfig
    /// @dev Context-switch: previous+new signers confirm on split-threshold quorum.
    function confirmKmsContextCreation(uint256 kmsContextId) external virtual {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        if ($.contextState[kmsContextId] != ContextState.Pending) {
            revert KmsContextNotPending(kmsContextId);
        }

        // Caller must belong to the outgoing or incoming signer set and confirm only once.
        address signer = msg.sender;
        uint256 previousContextId = $.latestActiveKmsContextId;
        bool isPreviousSigner = $.isKmsSignerForContext[previousContextId][signer];
        bool isNewSigner = $.isKmsSignerForContext[kmsContextId][signer];
        if (!isPreviousSigner && !isNewSigner) {
            revert KmsContextCreationUnauthorized(signer, kmsContextId);
        }
        if ($.contextCreationConfirmedBySigner[kmsContextId][signer]) {
            revert KmsContextCreationAlreadyConfirmed(signer, kmsContextId);
        }

        // Record the confirmation and counts separately for the split quorum.
        $.contextCreationConfirmedBySigner[kmsContextId][signer] = true;
        if (isPreviousSigner) {
            ++$.contextCreationPreviousSignerConfirmationCount[kmsContextId];
        }
        if (isNewSigner) {
            ++$.contextCreationNewSignerConfirmationCount[kmsContextId];
        }

        emit KmsContextCreationConfirmation(kmsContextId, signer, isPreviousSigner, isNewSigner);

        // All new signers + (n - t + 1) previous signers confirm to tell Connectors the epoch transition may start.
        if (_hasContextCreationQuorum(kmsContextId)) {
            $.contextState[kmsContextId] = ContextState.Created;
            // The context-switch created this context and its epoch as a paired (Pending, Pending). The DAO
            // settles each switch before opening another, so the latest-issued epoch is this context's Pending epoch.
            uint256 epochId = $.epochCounter;
            // Connectors must read previous key/CRS material from the last block before this epoch request.
            emit NewKmsEpoch(kmsContextId, epochId, previousContextId, $.latestActiveEpochId, block.number - 1);
        }
    }

    /// @inheritdoc IProtocolConfig
    /// @dev Final step of both flows Context-switch and Same-set resharing:
    ///      new-context signers attest reshared keys/CRS with full quorum activates the epoch.
    function confirmEpochActivation(
        uint256 epochId,
        EpochKeyResult[] calldata keys,
        EpochCrsResult[] calldata crsList
    ) external virtual {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();

        // Validate epoch activation: Verify EIP-712 keygen/CRS attestations and derive the consensus hash all signers must agree on.
        if ($.epochState[epochId] != EpochState.Pending) {
            revert InvalidEpoch(epochId);
        }

        uint256 contextId = $.contextForEpoch[epochId];
        if (!$.isKmsTxSenderForContext[contextId][msg.sender]) {
            revert EpochActivationUnauthorized(msg.sender, epochId);
        }

        if ($.contextState[contextId] == ContextState.Pending) {
            revert KmsContextNotCreated(contextId);
        }

        if (!_isLiveKmsContext(contextId)) {
            revert InvalidKmsContext(contextId);
        }

        address signer = $.kmsNodeByTxSenderForContext[contextId][msg.sender].signerAddress;
        bytes32 dataHash;
        {
            bytes memory extraData = abi.encodePacked(EXTRA_DATA_V2, contextId, epochId);

            bytes32[] memory keyHashes = new bytes32[](keys.length);
            for (uint256 i = 0; i < keys.length; i++) {
                bytes32 keyDigestsHash = _hashKeyDigests(keys[i].keyDigests);
                bytes32 digest = _hashKeygenVerification(
                    keys[i].prepKeygenId,
                    keys[i].keyId,
                    keyDigestsHash,
                    extraData
                );
                _requireExpectedSigner(signer, digest, keys[i].signature);
                keyHashes[i] = keccak256(abi.encode(keys[i].prepKeygenId, keys[i].keyId, keyDigestsHash));
            }

            bytes32[] memory crsHashes = new bytes32[](crsList.length);
            for (uint256 i = 0; i < crsList.length; i++) {
                bytes32 digest = _hashCrsgenVerification(
                    crsList[i].crsId,
                    crsList[i].maxBitLength,
                    crsList[i].crsDigest,
                    extraData
                );
                _requireExpectedSigner(signer, digest, crsList[i].signature);
                crsHashes[i] = keccak256(abi.encode(crsList[i].crsId, crsList[i].maxBitLength, crsList[i].crsDigest));
            }

            dataHash = keccak256(abi.encode(keyHashes, crsHashes));
        }

        // Confirm epoch activation: add this signer's vote under that hash, activate the epoch once all signers agree.
        // Record one confirmation per signer, counted by data hash so quorum requires all signers on the same result.
        if ($.epochActivationConfirmedBySigner[epochId][signer]) {
            revert EpochActivationAlreadyConfirmed(signer, epochId);
        }
        $.epochActivationConfirmedBySigner[epochId][signer] = true;
        uint256 digestCount = ++$.epochActivationConfirmationCountForDigest[epochId][dataHash];

        emit EpochActivationConfirmation(epochId, signer, dataHash);

        // All signers agreed, promote context and epoch to Active.
        if (digestCount == $.kmsSignerAddressesForContext[contextId].length) {
            $.contextState[contextId] = ContextState.Active;
            $.latestActiveKmsContextId = contextId;
            _activateEpoch(epochId, contextId);

            KmsNode[] storage nodes = $.kmsNodesForContext[contextId];
            string[] memory urls = new string[](nodes.length);
            for (uint256 i = 0; i < nodes.length; i++) {
                urls[i] = nodes[i].storageUrl;
            }
            emit ActivateEpoch(contextId, epochId, keys, crsList, urls);
        }
    }

    /// @inheritdoc IProtocolConfig
    function destroyKmsContext(uint256 kmsContextId) external virtual onlyACLOwner {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();

        if (kmsContextId == $.latestActiveKmsContextId) {
            revert CurrentKmsContextCannotBeDestroyed(kmsContextId);
        }
        if (!_isLiveKmsContext(kmsContextId)) {
            revert InvalidKmsContext(kmsContextId);
        }

        _clearContext(kmsContextId);
        emit KmsContextDestroyed(kmsContextId);
    }

    /// @inheritdoc IProtocolConfig
    function abortPendingEpoch(uint256 epochId) external virtual onlyACLOwner {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        if ($.epochState[epochId] != EpochState.Pending) {
            revert InvalidEpoch(epochId);
        }

        uint256 contextId = $.contextForEpoch[epochId];
        if (contextId != $.latestActiveKmsContextId) {
            revert InvalidEpoch(epochId);
        }

        _clearEpoch(epochId);
        emit PendingEpochAborted(contextId, epochId);
    }

    /// @inheritdoc IProtocolConfig
    function abortPendingContext(uint256 kmsContextId) external virtual onlyACLOwner {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        if ($.contextState[kmsContextId] != ContextState.Pending) {
            revert KmsContextNotPending(kmsContextId);
        }

        _clearContext(kmsContextId);
        emit PendingContextAborted(kmsContextId);
    }

    /// @inheritdoc IProtocolConfig
    function updatePublicDecryptionThresholdForContext(
        uint256 kmsContextId,
        uint256 threshold
    ) external virtual onlyACLOwner {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        _requireValidContext(kmsContextId);
        _checkThreshold("publicDecryption", threshold, $.kmsNodesForContext[kmsContextId].length);
        $.publicDecryptionThresholdForContext[kmsContextId] = threshold;
        emit PublicDecryptionThresholdUpdated(kmsContextId, threshold);
    }

    /// @inheritdoc IProtocolConfig
    function updateUserDecryptionThresholdForContext(
        uint256 kmsContextId,
        uint256 threshold
    ) external virtual onlyACLOwner {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        _requireValidContext(kmsContextId);
        _checkThreshold("userDecryption", threshold, $.kmsNodesForContext[kmsContextId].length);
        $.userDecryptionThresholdForContext[kmsContextId] = threshold;
        emit UserDecryptionThresholdUpdated(kmsContextId, threshold);
    }

    /// @inheritdoc IProtocolConfig
    function updateKmsGenThresholdForContext(uint256 kmsContextId, uint256 threshold) external virtual onlyACLOwner {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        _requireValidContext(kmsContextId);
        _checkThreshold("kmsGen", threshold, $.kmsNodesForContext[kmsContextId].length);
        $.kmsGenThresholdForContext[kmsContextId] = threshold;
        emit KmsGenThresholdUpdated(kmsContextId, threshold);
    }

    /// @inheritdoc IProtocolConfig
    function updateMpcThresholdForContext(uint256 kmsContextId, uint256 threshold) external virtual onlyACLOwner {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        _requireValidContext(kmsContextId);
        _checkThreshold("mpc", threshold, $.kmsNodesForContext[kmsContextId].length);
        $.mpcThresholdForContext[kmsContextId] = threshold;
        emit MpcThresholdUpdated(kmsContextId, threshold);
    }

    // -----------------------------------------------------------------------------------------
    // Mirror functions
    //
    // The non-canonical replica's only write path. They bypass the context-creation / epoch-activation
    // quorum because a replica cannot re-run the MPC attestations — they import state Ethereum (the
    // source of truth) has already finalized, landing it as immediately Active.
    // -----------------------------------------------------------------------------------------

    /// @inheritdoc IProtocolConfig
    /// @dev Mirror path for non-canonical hosts: imports the canonical context as already active
    ///      without replaying context-creation confirmations.
    function mirrorKmsContextAndEpoch(
        uint256 contextId,
        uint256 epochId,
        KmsNodeParams[] calldata kmsNodeParams,
        KmsThresholds calldata thresholds,
        string calldata softwareVersion,
        PcrValues[] calldata pcrValues
    ) external virtual onlyACLOwner {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        uint256 latestActiveKmsContextId = $.latestActiveKmsContextId;
        if (contextId <= latestActiveKmsContextId) {
            revert NonIncreasingKmsContextId(contextId, latestActiveKmsContextId);
        }
        uint256 currentEpochId = $.epochCounter;
        if (epochId <= currentEpochId) {
            revert NonIncreasingEpochId(epochId, currentEpochId);
        }

        // Seed counter so _storeKmsContext's ++counter lands on the existing context ID.
        $.currentKmsContextId = contextId - 1;
        _storeAndActivateKmsContext(kmsNodeParams, thresholds, epochId);
        emit MirrorKmsContextAndEpoch(contextId, epochId, kmsNodeParams, thresholds, softwareVersion, pcrValues);
    }

    /// @inheritdoc IProtocolConfig
    /// @dev Mirror path for non-canonical hosts: advances the active epoch for the already
    ///      mirrored active context without replaying epoch-activation confirmations.
    function mirrorKmsEpoch(uint256 contextId, uint256 epochId) external virtual onlyACLOwner {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        if (contextId != $.latestActiveKmsContextId || !_isLiveKmsContext(contextId)) {
            revert InvalidKmsContext(contextId);
        }
        uint256 currentEpochId = $.epochCounter;
        if (epochId <= currentEpochId) {
            revert NonIncreasingEpochId(epochId, currentEpochId);
        }

        $.epochCounter = epochId;
        _activateEpoch(epochId, contextId);
        emit MirrorKmsEpoch(contextId, epochId);
    }

    // -----------------------------------------------------------------------------------------
    // View functions
    // -----------------------------------------------------------------------------------------

    /// @inheritdoc IProtocolConfig
    function getCurrentKmsContextId() external view virtual returns (uint256) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.latestActiveKmsContextId;
    }

    /// @inheritdoc IProtocolConfig
    function getCurrentKmsContextAndEpoch() external view virtual returns (uint256 contextId, uint256 epochId) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        contextId = $.latestActiveKmsContextId;
        epochId = $.latestActiveEpochId;
    }

    /// @inheritdoc IProtocolConfig
    function getKmsContextAnchor(
        uint256 contextId
    ) external view virtual returns (uint256 emissionBlockNumber, bytes32 contextInfoHash) {
        if (!_kmsContextExists(contextId)) {
            revert InvalidKmsContext(contextId);
        }
        KmsContextAnchor memory anchor = _getProtocolConfigStorage().contextAnchors[contextId];
        return (anchor.emissionBlockNumber, anchor.contextInfoHash);
    }

    /// @inheritdoc IProtocolConfig
    function isValidKmsContext(uint256 kmsContextId) external view virtual returns (bool) {
        return _isValidKmsContext(kmsContextId);
    }

    /// @inheritdoc IProtocolConfig
    function isValidEpochForContext(uint256 kmsContextId, uint256 epochId) external view virtual returns (bool) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.epochState[epochId] == EpochState.Active && $.contextForEpoch[epochId] == kmsContextId;
    }

    /// @inheritdoc IProtocolConfig
    function getKmsSigners() external view virtual returns (address[] memory) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.kmsSignerAddressesForContext[$.latestActiveKmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function getKmsSignersForContext(uint256 kmsContextId) external view virtual returns (address[] memory) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().kmsSignerAddressesForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function isKmsSigner(address signer) external view virtual returns (bool) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.isKmsSignerForContext[$.latestActiveKmsContextId][signer];
    }

    /// @inheritdoc IProtocolConfig
    function isKmsSignerForContext(uint256 kmsContextId, address signer) external view virtual returns (bool) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().isKmsSignerForContext[kmsContextId][signer];
    }

    /// @inheritdoc IProtocolConfig
    function getKmsNodesForContext(uint256 kmsContextId) external view virtual returns (KmsNode[] memory) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().kmsNodesForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function isKmsTxSenderForContext(uint256 kmsContextId, address txSender) external view virtual returns (bool) {
        // `_isLiveKmsContext` is used so a `Created` (not yet `Active`) context's nodes are readable during resharing.
        if (!_isLiveKmsContext(kmsContextId)) {
            revert InvalidKmsContext(kmsContextId);
        }
        return _getProtocolConfigStorage().isKmsTxSenderForContext[kmsContextId][txSender];
    }

    /// @inheritdoc IProtocolConfig
    function getKmsNodeForContext(
        uint256 kmsContextId,
        address txSender
    ) external view virtual returns (KmsNode memory) {
        // `_isLiveKmsContext` is used so a `Created` (not yet `Active`) context's nodes are readable during resharing.
        if (!_isLiveKmsContext(kmsContextId)) {
            revert InvalidKmsContext(kmsContextId);
        }
        return _getProtocolConfigStorage().kmsNodeByTxSenderForContext[kmsContextId][txSender];
    }

    /// @inheritdoc IProtocolConfig
    function getPublicDecryptionThreshold() external view virtual returns (uint256) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.publicDecryptionThresholdForContext[$.latestActiveKmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function getPublicDecryptionThresholdForContext(uint256 kmsContextId) external view virtual returns (uint256) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().publicDecryptionThresholdForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function getUserDecryptionThreshold() external view virtual returns (uint256) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.userDecryptionThresholdForContext[$.latestActiveKmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function getUserDecryptionThresholdForContext(uint256 kmsContextId) external view virtual returns (uint256) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().userDecryptionThresholdForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function getKmsGenThreshold() external view virtual returns (uint256) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.kmsGenThresholdForContext[$.latestActiveKmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function getKmsGenThresholdForContext(uint256 kmsContextId) external view virtual returns (uint256) {
        if (!_isLiveKmsContext(kmsContextId)) {
            revert InvalidKmsContext(kmsContextId);
        }
        return _getProtocolConfigStorage().kmsGenThresholdForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function getMpcThreshold() external view virtual returns (uint256) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.mpcThresholdForContext[$.latestActiveKmsContextId];
    }

    /// @inheritdoc IProtocolConfig
    function getMpcThresholdForContext(uint256 kmsContextId) external view virtual returns (uint256) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().mpcThresholdForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfig
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

    // -----------------------------------------------------------------------------------------
    // Internal
    // -----------------------------------------------------------------------------------------

    /**
     * @dev Creates a new KMS context, validates nodes and thresholds, and activates it under
     *      `epochId`. Returns the new context ID. Callers are responsible for emitting context
     *      lifecycle events and recording the matching `KmsContextAnchor` when appropriate.
     */
    function _storeAndActivateKmsContext(
        KmsNodeParams[] memory kmsNodeParams,
        KmsThresholds calldata thresholds,
        uint256 epochId
    ) internal virtual returns (uint256 newContextId) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        newContextId = _storeKmsContext(kmsNodeParams, thresholds);

        // Set the context to Active and update the active context and epoch IDs.
        $.contextState[newContextId] = ContextState.Active;
        $.latestActiveKmsContextId = newContextId;
        $.epochCounter = epochId;
        _activateEpoch(epochId, newContextId);
    }

    function _storeKmsContext(
        KmsNodeParams[] memory kmsNodeParams,
        KmsThresholds calldata thresholds
    ) internal virtual returns (uint256 newContextId) {
        if (kmsNodeParams.length == 0) {
            revert EmptyKmsNodes();
        }
        if (kmsNodeParams.length > MAX_KMS_SIGNERS) {
            revert KmsSignerSetExceedsProofFormatLimit(kmsNodeParams.length, MAX_KMS_SIGNERS);
        }

        // Validate that thresholds are non-zero and not exceeding the node count.
        _checkThreshold("publicDecryption", thresholds.publicDecryption, kmsNodeParams.length);
        _checkThreshold("userDecryption", thresholds.userDecryption, kmsNodeParams.length);
        _checkThreshold("kmsGen", thresholds.kmsGen, kmsNodeParams.length);
        _checkThreshold("mpc", thresholds.mpc, kmsNodeParams.length);

        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        newContextId = ++$.currentKmsContextId;

        for (uint256 i = 0; i < kmsNodeParams.length; i++) {
            KmsNodeParams memory params = kmsNodeParams[i];
            _storeKmsNode(
                newContextId,
                KmsNode({
                    txSenderAddress: params.txSenderAddress,
                    signerAddress: params.signerAddress,
                    ipAddress: params.ipAddress,
                    storageUrl: params.storageUrl
                })
            );
        }

        // Store thresholds
        $.publicDecryptionThresholdForContext[newContextId] = thresholds.publicDecryption;
        $.userDecryptionThresholdForContext[newContextId] = thresholds.userDecryption;
        $.kmsGenThresholdForContext[newContextId] = thresholds.kmsGen;
        $.mpcThresholdForContext[newContextId] = thresholds.mpc;
    }

    function _storeKmsNode(uint256 contextId, KmsNode memory node) internal virtual {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        if (node.txSenderAddress == address(0)) {
            revert KmsNodeNullTxSender();
        }
        if (node.signerAddress == address(0)) {
            revert KmsNodeNullSigner();
        }
        if ($.isKmsTxSenderForContext[contextId][node.txSenderAddress]) {
            revert KmsTxSenderAlreadyRegistered(node.txSenderAddress);
        }
        if ($.isKmsSignerForContext[contextId][node.signerAddress]) {
            revert KmsSignerAlreadyRegistered(node.signerAddress);
        }

        $.kmsNodesForContext[contextId].push(node);
        $.isKmsTxSenderForContext[contextId][node.txSenderAddress] = true;
        $.isKmsSignerForContext[contextId][node.signerAddress] = true;
        $.kmsNodeByTxSenderForContext[contextId][node.txSenderAddress] = node;
        $.kmsSignerAddressesForContext[contextId].push(node.signerAddress);
    }

    /**
     * @dev Validates a single threshold: must be non-zero and at most nodeCount.
     */
    function _checkThreshold(string memory name, uint256 value, uint256 nodeCount) internal pure {
        if (value == 0) revert InvalidNullThreshold(name);
        if (value > MAX_KMS_SIGNERS) revert ThresholdExceedsProofFormatLimit(name, value, MAX_KMS_SIGNERS);
        if (value > nodeCount) revert InvalidHighThreshold(name, value, nodeCount);
    }

    /**
     * @dev Returns true if the context exists and has not been destroyed. The stored-node
     * check also keeps skipped canonical IDs invalid when `initializeFromCanonical` preserves
     * a context ID above `BASE + 1`.
     */
    function _isLiveKmsContext(uint256 kmsContextId) internal view virtual returns (bool) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return _kmsContextExists(kmsContextId) && !$.destroyedContexts[kmsContextId];
    }

    /**
     * @dev Returns true if the context was ever stored, even if it has since been destroyed.
     * Used for historical reads (e.g. context anchors) that must remain accessible post-destruction.
     */
    function _kmsContextExists(uint256 kmsContextId) internal view virtual returns (bool) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return
            kmsContextId >= KMS_CONTEXT_COUNTER_BASE + 1 &&
            kmsContextId <= $.currentKmsContextId &&
            $.kmsNodesForContext[kmsContextId].length != 0;
    }

    /**
     * @dev Returns true if the context exists and is currently in the `Active` lifecycle state.
     */
    function _isValidKmsContext(uint256 kmsContextId) internal view virtual returns (bool) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return _isLiveKmsContext(kmsContextId) && $.contextState[kmsContextId] == ContextState.Active;
    }

    function _requireValidContext(uint256 kmsContextId) internal view virtual {
        if (!_isValidKmsContext(kmsContextId)) {
            revert InvalidKmsContext(kmsContextId);
        }
    }

    function _hasContextCreationQuorum(uint256 contextId) internal view virtual returns (bool) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return
            $.contextCreationNewSignerConfirmationCount[contextId] ==
            $.kmsSignerAddressesForContext[contextId].length &&
            $.contextCreationPreviousSignerConfirmationCount[contextId] >=
            $.contextCreationPreviousSignerThreshold[contextId];
    }

    function _requireExpectedSigner(
        address expectedSigner,
        bytes32 digest,
        bytes calldata signature
    ) internal view virtual {
        address recoveredSigner = ECDSA.recover(digest, signature);
        if (recoveredSigner != expectedSigner) {
            revert EpochActivationSignerDoesNotMatchTxSender(recoveredSigner, msg.sender);
        }
    }

    function _hashKeygenVerification(
        uint256 prepKeygenId,
        uint256 keyId,
        bytes32 keyDigestsHash,
        bytes memory extraData
    ) internal view virtual returns (bytes32) {
        return
            _hashTypedData(
                keccak256(
                    abi.encode(EIP712_KEYGEN_TYPE_HASH, prepKeygenId, keyId, keyDigestsHash, keccak256(extraData))
                )
            );
    }

    function _hashKeyDigests(IKMSGeneration.KeyDigest[] calldata keyDigests) internal pure virtual returns (bytes32) {
        bytes32[] memory keyDigestHashes = new bytes32[](keyDigests.length);
        for (uint256 i = 0; i < keyDigests.length; i++) {
            keyDigestHashes[i] = keccak256(
                abi.encode(EIP712_KEY_DIGEST_TYPE_HASH, keyDigests[i].keyType, keccak256(keyDigests[i].digest))
            );
        }
        return keccak256(abi.encodePacked(keyDigestHashes));
    }

    function _hashCrsgenVerification(
        uint256 crsId,
        uint256 maxBitLength,
        bytes calldata crsDigest,
        bytes memory extraData
    ) internal view virtual returns (bytes32) {
        return
            _hashTypedData(
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

    function _hashTypedData(bytes32 structHash) internal view virtual returns (bytes32) {
        bytes32 domainSeparator = keccak256(
            abi.encode(
                EIP712_DOMAIN_TYPE_HASH,
                keccak256(bytes(CONTRACT_NAME)),
                keccak256(bytes("1")),
                block.chainid,
                address(this)
            )
        );
        return keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
    }

    /**
     * @dev Marks a context destroyed, clears its pending epoch (if any), and wipes the
     *      context-creation bookkeeping. Shared by `destroyKmsContext` and `abortPendingContext`;
     *      callers own the preconditions and the event emitted.
     */
    function _clearContext(uint256 kmsContextId) internal virtual {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        $.destroyedContexts[kmsContextId] = true;
        $.contextState[kmsContextId] = ContextState.None;
        // At most one epoch is Pending at a time, so the latest-issued epoch is the only candidate.
        uint256 latestEpochId = $.epochCounter;
        if ($.epochState[latestEpochId] == EpochState.Pending && $.contextForEpoch[latestEpochId] == kmsContextId) {
            _clearEpoch(latestEpochId);
        }
        delete $.contextCreationPreviousSignerThreshold[kmsContextId];
        delete $.contextCreationNewSignerConfirmationCount[kmsContextId];
        delete $.contextCreationPreviousSignerConfirmationCount[kmsContextId];
    }

    /**
     * @dev Creates the next epoch in Pending state under `contextId` and returns its ID.
     *      Callers own any `NewKmsEpoch` event emission.
     */
    function _createPendingEpoch(uint256 contextId) internal virtual returns (uint256 epochId) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        epochId = ++$.epochCounter;
        $.epochState[epochId] = EpochState.Pending;
        $.contextForEpoch[epochId] = contextId;
    }

    /**
     * @dev Promotes `epochId` to Active under `contextId` and makes it the active epoch.
     *      Callers own the matching context-state writes and any event emission.
     */
    function _activateEpoch(uint256 epochId, uint256 contextId) internal virtual {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        $.epochState[epochId] = EpochState.Active;
        $.contextForEpoch[epochId] = contextId;
        $.latestActiveEpochId = epochId;
    }

    /**
     * @dev Clears `epochId` back to None and drops its context link. Callers own the preconditions
     *      and any event emission.
     */
    function _clearEpoch(uint256 epochId) internal virtual {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        $.epochState[epochId] = EpochState.None;
        delete $.contextForEpoch[epochId];
    }

    /**
     * @dev Authorization for UUPS upgrades.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
