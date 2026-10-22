// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {IProtocolConfigBase} from "./interfaces/IProtocolConfigBase.sol";
import {KmsContextAnchor, KmsNode, KmsThresholds} from "./shared/Structs.sol";

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

    /// @dev EIP-712 domain name of the canonical ProtocolConfig. The KMS signers sign under it, and
    ///      the replica verifies under it.
    string internal constant EIP712_NAME = "ProtocolConfig";

    // -----------------------------------------------------------------------------------------
    // EIP-712 type hashes
    //
    // The four KMS lifecycle confirmations. The canonical recovers one signer per call. A replica
    // verifies a quorum of them under the canonical's domain.
    // -----------------------------------------------------------------------------------------

    /// @dev Hash of the ContextCreationConfirmation type.
    bytes32 internal constant EIP712_CONTEXT_CREATION_TYPE_HASH =
        keccak256(
            "ContextCreationConfirmation(uint256 previousContextId,uint256 newContextId,bytes32 nodeConfigHash,bytes extraData)"
        );

    /// @dev Hash of the EpochActivationConfirmation type.
    bytes32 internal constant EIP712_EPOCH_ACTIVATION_TYPE_HASH =
        keccak256(
            "EpochActivationConfirmation(uint256 contextId,uint256 previousEpochId,uint256 epochId,bytes32 epochMaterialHash,bytes extraData)"
        );

    /// @dev Hash of the ContextDestructionConfirmation type.
    bytes32 internal constant EIP712_CONTEXT_DESTRUCTION_TYPE_HASH =
        keccak256("ContextDestructionConfirmation(uint256 destroyedContextId,bytes extraData)");

    /// @dev Hash of the EpochDestructionConfirmation type.
    bytes32 internal constant EIP712_EPOCH_DESTRUCTION_TYPE_HASH =
        keccak256("EpochDestructionConfirmation(uint256 destroyedEpochId,bytes extraData)");

    // -----------------------------------------------------------------------------------------
    // ERC-7201 namespaced storage
    // -----------------------------------------------------------------------------------------

    /// @dev A `KmsNode` without `connectorUrl`, as an element of the ordered committee.
    struct KmsCommitteeEntry {
        address txSenderAddress;
        address signerAddress;
        string ipAddress;
        string storageUrl;
    }

    /// @custom:storage-location erc7201:fhevm.storage.ProtocolConfig
    struct ProtocolConfigStorage {
        /// @notice On the canonical, the monotonic allocation counter for KMS context IDs: the latest
        ///         issued ID. Always `>= latestActiveKmsContextId`. Differs while a context is Pending/Created.
        ///         On a replica, every activation sets it to `latestActiveKmsContextId`, which can lower it.
        ///         A replica upgraded from v0.15 can hold a higher value until its first activation.
        uint256 currentKmsContextId;
        /// @notice KMS committee per context, in definition order.
        /// @dev Array elements occupy consecutive slots, so the element type cannot grow. A new field
        ///      would overlap the next element. The full `KmsNode` lives in `kmsNodeByTxSenderForContext`.
        mapping(uint256 contextId => KmsCommitteeEntry[]) kmsNodesForContext;
        /// @notice Tx sender lookup per context.
        mapping(uint256 contextId => mapping(address txSender => bool isRegistered)) isKmsTxSenderForContext;
        /// @notice Signer lookup per context.
        mapping(uint256 contextId => mapping(address signer => bool isRegistered)) isKmsSignerForContext;
        /// @notice KmsNode by tx sender per context.
        /// @dev Each mapping value has its own slot range, so `KmsNode` can grow in place. A value
        ///      stored before ProtocolConfig v0.4.0 reads an empty `connectorUrl`.
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
        ///         Updated only on activation, unlike the canonical `currentKmsContextId` allocation counter.
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
        /// @dev Unused since v0.4.0. Kept for storage layout.
        mapping(uint256 contextId => mapping(address txSender => bool confirmed)) contextCreationConfirmedByTxSender;
        /// @notice Epoch activation confirmations per signer (one digest per signer per epoch).
        mapping(uint256 epochId => mapping(address signer => bool confirmed)) epochActivationConfirmedBySigner;
        /// @notice Epoch activation confirmations grouped by EpochActivationConfirmation digest.
        mapping(uint256 epochId => mapping(bytes32 dataHash => uint256 confirmations)) epochActivationConfirmationCountForDigest;
        /// @notice Required previous-context confirmation quorum, cached at pending-context creation time.
        /// @dev Unused since v0.4.0. Kept for storage layout.
        mapping(uint256 contextId => uint256 threshold) contextCreationPreviousTxSenderThreshold;
        /// @notice New-context tx-sender confirmations for context creation.
        /// @dev Unused since v0.4.0. Kept for storage layout.
        mapping(uint256 contextId => uint256 confirmations) contextCreationNewTxSenderConfirmationCount;
        /// @notice Previous-context tx-sender confirmations for context creation.
        /// @dev Unused since v0.4.0. Kept for storage layout.
        mapping(uint256 contextId => uint256 confirmations) contextCreationPreviousTxSenderConfirmationCount;
        /// @notice Context anchor recorded when NewKmsContext was emitted.
        mapping(uint256 contextId => KmsContextAnchor) contextAnchors;
        /// @notice Whether an epoch has been destroyed.
        /// @dev Added in v0.4.0. An epoch destroyed before the upgrade is not flagged.
        mapping(uint256 epochId => bool) destroyedEpochs;
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
        return _getKmsNodes(kmsContextId);
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
     *      thresholds, and activates it under `epochId`. Use this on the two bootstrap paths, where
     *      the context ID is externally determined. Callers are responsible for emitting
     *      context lifecycle events and recording the matching `KmsContextAnchor` when appropriate.
     */
    function _storeAndActivateKmsContextAndEpoch(
        uint256 contextId,
        uint256 epochId,
        KmsNode[] memory nodes,
        KmsThresholds calldata thresholds
    ) internal virtual {
        _storeKmsCommittee(contextId, nodes, thresholds);
        _activateWithCounters(contextId, epochId);
    }

    /**
     * @dev Activates `contextId` and `epochId` as the latest active pair and sets both counters to
     *      them. The canonical calls this only on bootstrap, because its allocation counter can run
     *      ahead of the active pair. On a replica every activation is its newest state.
     */
    function _activateWithCounters(uint256 contextId, uint256 epochId) internal {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        $.currentKmsContextId = contextId;
        $.epochCounter = epochId;
        _activateKmsContextAndEpoch(contextId, epochId);
    }

    /**
     * @dev Validates `nodes` and `thresholds` and writes them under `contextId`.
     */
    function _storeKmsCommittee(
        uint256 contextId,
        KmsNode[] memory nodes,
        KmsThresholds calldata thresholds
    ) internal virtual {
        _checkKmsThresholds(nodes.length, thresholds);

        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        for (uint256 i = 0; i < nodes.length; i++) {
            _storeKmsNode(contextId, nodes[i]);
        }

        // Store thresholds
        $.publicDecryptionThresholdForContext[contextId] = thresholds.publicDecryption;
        $.userDecryptionThresholdForContext[contextId] = thresholds.userDecryption;
        $.kmsGenThresholdForContext[contextId] = thresholds.kmsGen;
        $.mpcThresholdForContext[contextId] = thresholds.mpc;
    }

    /**
     * @dev The committee of `contextId` in definition order, read from `kmsNodeByTxSenderForContext`.
     */
    function _getKmsNodes(uint256 contextId) internal view returns (KmsNode[] memory nodes) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        KmsCommitteeEntry[] storage entries = $.kmsNodesForContext[contextId];
        nodes = new KmsNode[](entries.length);
        for (uint256 i = 0; i < entries.length; i++) {
            nodes[i] = $.kmsNodeByTxSenderForContext[contextId][entries[i].txSenderAddress];
        }
    }

    /**
     * @dev Validates the node count, and `thresholds` against it.
     */
    function _checkKmsThresholds(uint256 nodeCount, KmsThresholds calldata thresholds) internal pure {
        if (nodeCount == 0) {
            revert EmptyKmsNodes();
        }
        if (nodeCount > MAX_KMS_SIGNERS) {
            revert KmsSignerSetExceedsProofFormatLimit(nodeCount, MAX_KMS_SIGNERS);
        }

        // Validate that thresholds are non-zero and not exceeding the node count.
        _checkThreshold("publicDecryption", thresholds.publicDecryption, nodeCount);
        _checkThreshold("userDecryption", thresholds.userDecryption, nodeCount);
        _checkThreshold("kmsGen", thresholds.kmsGen, nodeCount);
        _checkThreshold("mpc", thresholds.mpc, nodeCount);
    }

    /**
     * @dev Validates `node` and appends it to the committee of `contextId`.
     */
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

        $.kmsNodesForContext[contextId].push(
            KmsCommitteeEntry({
                txSenderAddress: node.txSenderAddress,
                signerAddress: node.signerAddress,
                ipAddress: node.ipAddress,
                storageUrl: node.storageUrl
            })
        );
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

    /**
     * @dev Sets `contextId` Active and activates `epochId` under it, as the latest active pair.
     *      Callers own the counter writes and any event emission.
     */
    function _activateKmsContextAndEpoch(uint256 contextId, uint256 epochId) internal virtual {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        $.contextState[contextId] = ContextState.Active;
        $.latestActiveKmsContextId = contextId;
        _activateEpoch(epochId, contextId);
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
     * @dev Marks `contextId` destroyed and resets its state to None. Callers own the preconditions.
     */
    function _markKmsContextDestroyed(uint256 contextId) internal {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        $.destroyedContexts[contextId] = true;
        $.contextState[contextId] = ContextState.None;
        emit KmsContextDestroyed(contextId);
    }

    /**
     * @dev Clears `epochId` and marks it destroyed. Callers own the preconditions.
     */
    function _markKmsEpochDestroyed(uint256 epochId) internal {
        _clearEpoch(epochId);
        _getProtocolConfigStorage().destroyedEpochs[epochId] = true;
        emit KmsEpochDestroyed(epochId);
    }

    /**
     * @dev Returns the `n - t` quorum of the committee of `contextId`, floored at 1.
     *      The committee has `n` nodes. At most `t` of them (its MPC threshold) can be faulty, which
     *      means crashed, offline, or malicious. The quorum must be:
     *        - more than `t`, so faulty nodes can never approve a step on their own.
     *        - at most `n - t`, because if `t` nodes stay silent only `n - t` confirmations ever
     *          arrive. Anything higher lets a dead node block the step forever.
     *      `n - t` satisfies `n - t >= t + 1` under the `n >= 3t + 1` topology the KMS core
     *      requires. The contract itself does not enforce the topology.
     *      The quorum is floored at 1, so the degenerate `t = n` config cannot make it zero.
     */
    function _faultTolerantQuorum(uint256 contextId) internal view returns (uint256) {
        ProtocolConfigStorage storage $ = _getProtocolConfigStorage();
        uint256 quorum = $.kmsNodesForContext[contextId].length - $.mpcThresholdForContext[contextId];
        return quorum > 0 ? quorum : 1;
    }

    /**
     * @dev Hash of a context's node set and thresholds, signed in ContextCreationConfirmation.
     *      It covers each node's `connectorUrl`, so a replica submitter cannot swap a URL without
     *      invalidating the signatures.
     */
    function _nodeConfigHash(KmsNode[] memory nodes, KmsThresholds memory thresholds) internal pure returns (bytes32) {
        return keccak256(abi.encode(nodes, thresholds));
    }

    /**
     * @dev Struct hash of a ContextCreationConfirmation.
     */
    function _contextCreationStructHash(
        uint256 previousContextId,
        uint256 newContextId,
        bytes32 nodeConfigHash,
        bytes calldata extraData
    ) internal pure returns (bytes32) {
        return
            keccak256(
                abi.encode(
                    EIP712_CONTEXT_CREATION_TYPE_HASH,
                    previousContextId,
                    newContextId,
                    nodeConfigHash,
                    keccak256(extraData)
                )
            );
    }

    /**
     * @dev Struct hash of an EpochActivationConfirmation.
     */
    function _epochActivationStructHash(
        uint256 contextId,
        uint256 previousEpochId,
        uint256 epochId,
        bytes32 epochMaterialHash,
        bytes calldata extraData
    ) internal pure returns (bytes32) {
        return
            keccak256(
                abi.encode(
                    EIP712_EPOCH_ACTIVATION_TYPE_HASH,
                    contextId,
                    previousEpochId,
                    epochId,
                    epochMaterialHash,
                    keccak256(extraData)
                )
            );
    }

    /**
     * @dev Struct hash of a ContextDestructionConfirmation.
     */
    function _contextDestructionStructHash(
        uint256 destroyedContextId,
        bytes calldata extraData
    ) internal pure returns (bytes32) {
        return keccak256(abi.encode(EIP712_CONTEXT_DESTRUCTION_TYPE_HASH, destroyedContextId, keccak256(extraData)));
    }

    /**
     * @dev Struct hash of an EpochDestructionConfirmation.
     */
    function _epochDestructionStructHash(
        uint256 destroyedEpochId,
        bytes calldata extraData
    ) internal pure returns (bytes32) {
        return keccak256(abi.encode(EIP712_EPOCH_DESTRUCTION_TYPE_HASH, destroyedEpochId, keccak256(extraData)));
    }
}
