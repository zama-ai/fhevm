// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {IProtocolConfigBase} from "./interfaces/IProtocolConfigBase.sol";
import {KmsContextAnchor, KmsNode, KmsNodeParams, KmsThresholds} from "./shared/Structs.sol";

/**
 * @title ProtocolConfigBase
 * @notice Shared storage, reads and write helpers for ProtocolConfig and ProtocolConfigReplica.
 */
abstract contract ProtocolConfigBase is IProtocolConfigBase {
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

    /// @notice Upper bound on the KMS committee size and on every per-context threshold.
    /// @dev Driven by the proof format consumed in
    ///      `KMSVerifier.verifyDecryptionEIP712KMSSignatures`, which encodes the signature count
    ///      in a single byte (`uint8(decryptionProof[0])`). A context registered above this
    ///      bound cannot ever satisfy verification, so the limit is enforced at registration time
    ///      to reject the misconfiguration loudly rather than silently bricking the context.
    uint256 private constant MAX_KMS_SIGNERS = type(uint8).max;

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
        mapping(uint256 contextId => mapping(address txSender => bool confirmed)) contextCreationConfirmedByTxSender;
        /// @notice Epoch activation confirmations per signer (one digest per signer per epoch).
        mapping(uint256 epochId => mapping(address signer => bool confirmed)) epochActivationConfirmedBySigner;
        /// @notice Number of epoch activation confirmations grouped by digest
        mapping(uint256 epochId => mapping(bytes32 dataHash => uint256 confirmations)) epochActivationConfirmationCountForDigest;
        /// @notice Required previous-context confirmation quorum, cached at pending-context creation time.
        mapping(uint256 contextId => uint256 threshold) contextCreationPreviousTxSenderThreshold;
        /// @notice New-context tx-sender confirmations for context creation.
        mapping(uint256 contextId => uint256 confirmations) contextCreationNewTxSenderConfirmationCount;
        /// @notice Previous-context tx-sender confirmations for context creation.
        mapping(uint256 contextId => uint256 confirmations) contextCreationPreviousTxSenderConfirmationCount;
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
    // View functions
    // -----------------------------------------------------------------------------------------

    /// @inheritdoc IProtocolConfigBase
    function getCurrentKmsContextId() external view virtual returns (uint256) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.latestActiveKmsContextId;
    }

    /// @inheritdoc IProtocolConfigBase
    function getCurrentKmsContextIdCounter() external view virtual returns (uint256) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.currentKmsContextId;
    }

    /// @inheritdoc IProtocolConfigBase
    function getCurrentKmsContextAndEpoch() external view virtual returns (uint256 contextId, uint256 epochId) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        contextId = $.latestActiveKmsContextId;
        epochId = $.latestActiveEpochId;
    }

    /// @inheritdoc IProtocolConfigBase
    function isValidKmsContext(uint256 kmsContextId) external view virtual returns (bool) {
        return _isValidKmsContext(kmsContextId);
    }

    /// @inheritdoc IProtocolConfigBase
    function isLiveKmsContext(uint256 kmsContextId) external view virtual returns (bool) {
        return _isLiveKmsContext(kmsContextId);
    }

    /// @inheritdoc IProtocolConfigBase
    function isValidEpochForContext(uint256 kmsContextId, uint256 epochId) external view virtual returns (bool) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return
            $.epochState[epochId] == EpochState.Active &&
            $.contextForEpoch[epochId] == kmsContextId &&
            _isValidKmsContext(kmsContextId);
    }

    /// @inheritdoc IProtocolConfigBase
    function getKmsSigners() external view virtual returns (address[] memory) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.kmsSignerAddressesForContext[$.latestActiveKmsContextId];
    }

    /// @inheritdoc IProtocolConfigBase
    function getKmsSignersForContext(uint256 kmsContextId) external view virtual returns (address[] memory) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().kmsSignerAddressesForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfigBase
    function isKmsSigner(address signer) external view virtual returns (bool) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.isKmsSignerForContext[$.latestActiveKmsContextId][signer];
    }

    /// @inheritdoc IProtocolConfigBase
    function isKmsSignerForContext(uint256 kmsContextId, address signer) external view virtual returns (bool) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().isKmsSignerForContext[kmsContextId][signer];
    }

    /// @inheritdoc IProtocolConfigBase
    function getKmsNodesForContext(uint256 kmsContextId) external view virtual returns (KmsNode[] memory) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().kmsNodesForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfigBase
    function isKmsTxSenderForContext(uint256 kmsContextId, address txSender) external view virtual returns (bool) {
        // `_isLiveKmsContext` is used so a `Created` (not yet `Active`) context's nodes are readable during resharing.
        if (!_isLiveKmsContext(kmsContextId)) {
            revert InvalidKmsContext(kmsContextId);
        }
        return _getProtocolConfigStorage().isKmsTxSenderForContext[kmsContextId][txSender];
    }

    /// @inheritdoc IProtocolConfigBase
    function getKmsNodeForContext(
        uint256 kmsContextId,
        address txSender
    ) external view virtual returns (KmsNode memory) {
        // Existence-based, not liveness: a context's nodes stay readable for its whole lifecycle —
        // Pending/Created during resharing, and even once it is destroyed — so key/CRS material
        // generated under it can still resolve its storage nodes. Only a never-created context reverts.
        if (!_kmsContextExists(kmsContextId)) {
            revert InvalidKmsContext(kmsContextId);
        }
        return _getProtocolConfigStorage().kmsNodeByTxSenderForContext[kmsContextId][txSender];
    }

    /// @inheritdoc IProtocolConfigBase
    function getPublicDecryptionThreshold() external view virtual returns (uint256) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.publicDecryptionThresholdForContext[$.latestActiveKmsContextId];
    }

    /// @inheritdoc IProtocolConfigBase
    function getPublicDecryptionThresholdForContext(uint256 kmsContextId) external view virtual returns (uint256) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().publicDecryptionThresholdForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfigBase
    function getUserDecryptionThreshold() external view virtual returns (uint256) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.userDecryptionThresholdForContext[$.latestActiveKmsContextId];
    }

    /// @inheritdoc IProtocolConfigBase
    function getUserDecryptionThresholdForContext(uint256 kmsContextId) external view virtual returns (uint256) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().userDecryptionThresholdForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfigBase
    function getKmsGenThreshold() external view virtual returns (uint256) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.kmsGenThresholdForContext[$.latestActiveKmsContextId];
    }

    /// @inheritdoc IProtocolConfigBase
    function getKmsGenThresholdForContext(uint256 kmsContextId) external view virtual returns (uint256) {
        if (!_isLiveKmsContext(kmsContextId)) {
            revert InvalidKmsContext(kmsContextId);
        }
        return _getProtocolConfigStorage().kmsGenThresholdForContext[kmsContextId];
    }

    /// @inheritdoc IProtocolConfigBase
    function getMpcThreshold() external view virtual returns (uint256) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        return $.mpcThresholdForContext[$.latestActiveKmsContextId];
    }

    /// @inheritdoc IProtocolConfigBase
    function getMpcThresholdForContext(uint256 kmsContextId) external view virtual returns (uint256) {
        _requireValidContext(kmsContextId);
        return _getProtocolConfigStorage().mpcThresholdForContext[kmsContextId];
    }

    /**
     * @dev Returns true if the context exists and has not been destroyed. Every stored context is
     * Pending, Created or Active, and only destruction resets it to None.
     */
    function _isLiveKmsContext(uint256 kmsContextId) internal view virtual returns (bool) {
        return _getProtocolConfigStorage().contextState[kmsContextId] != ContextState.None;
    }

    /**
     * @dev Returns true if the context was ever stored, even if it has since been destroyed.
     * Used for historical reads (e.g. context anchors) that must remain accessible post-destruction.
     */
    function _kmsContextExists(uint256 kmsContextId) internal view virtual returns (bool) {
        return _getProtocolConfigStorage().kmsNodesForContext[kmsContextId].length != 0;
    }

    /**
     * @dev Returns true if the context exists and is currently in the `Active` lifecycle state.
     */
    function _isValidKmsContext(uint256 kmsContextId) internal view virtual returns (bool) {
        return _getProtocolConfigStorage().contextState[kmsContextId] == ContextState.Active;
    }

    function _requireValidContext(uint256 kmsContextId) internal view virtual {
        if (!_isValidKmsContext(kmsContextId)) {
            revert InvalidKmsContext(kmsContextId);
        }
    }

    /**
     * @dev Stores a KMS context under the caller-provided `contextId`, validates nodes and
     *      thresholds, and activates it under `epochId`. Use this on the bootstrap/mirror paths
     *      where the context ID is externally determined. Callers are responsible for emitting
     *      context lifecycle events and recording the matching `KmsContextAnchor` when appropriate.
     */
    function _storeAndActivateKmsContextAndEpoch(
        uint256 contextId,
        uint256 epochId,
        KmsNodeParams[] calldata kmsNodeParams,
        KmsThresholds calldata thresholds
    ) internal virtual {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        _storeKmsContext(contextId, kmsNodeParams, thresholds);

        // Set the context to Active and update the active context and epoch IDs.
        $.contextState[contextId] = ContextState.Active;
        $.latestActiveKmsContextId = contextId;
        $.epochCounter = epochId;
        _activateEpoch(epochId, contextId);
    }

    function _storeKmsContext(
        uint256 contextId,
        KmsNodeParams[] calldata kmsNodeParams,
        KmsThresholds calldata thresholds
    ) internal virtual {
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
        // Single monotonic-uniqueness gate for every context-creation path.
        if (contextId <= $.currentKmsContextId) {
            revert NonIncreasingKmsContextId(contextId, $.currentKmsContextId);
        }
        $.currentKmsContextId = contextId;

        for (uint256 i = 0; i < kmsNodeParams.length; i++) {
            KmsNodeParams calldata params = kmsNodeParams[i];
            KmsNode memory node = KmsNode({
                txSenderAddress: params.txSenderAddress,
                signerAddress: params.signerAddress,
                ipAddress: params.ipAddress,
                storageUrl: params.storageUrl
            });
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

        // Store thresholds
        $.publicDecryptionThresholdForContext[contextId] = thresholds.publicDecryption;
        $.userDecryptionThresholdForContext[contextId] = thresholds.userDecryption;
        $.kmsGenThresholdForContext[contextId] = thresholds.kmsGen;
        $.mpcThresholdForContext[contextId] = thresholds.mpc;
    }

    /**
     * @dev Validates a single threshold: must be non-zero and at most nodeCount.
     */
    function _checkThreshold(string memory name, uint256 value, uint256 nodeCount) internal pure {
        if (value == 0) revert InvalidNullThreshold(name);
        if (value > nodeCount) revert InvalidHighThreshold(name, value, nodeCount);
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
}
